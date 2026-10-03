use sddia_io::outbound_lab::{lab_mock_linear_url, lab_mock_outbound_enabled};
use sddia_io::read_stdin_json;
use serde_json::{json, Map, Value};
use std::env;
use std::process;
use std::time::Duration;

const ENTITY_ID: &str = "linear-tracker-adapter";
const DEFAULT_GRAPHQL_URL: &str = "https://api.linear.app/graphql";

#[derive(Debug, Clone)]
struct LinearFail {
    code: &'static str,
    message: String,
    exit: i32,
}

fn emit_v2(success: bool, exit_code: i32, message: &str, result: Option<Value>, error_code: Option<&str>) -> ! {
    let mut body = json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": ENTITY_ID,
        },
        "success": success,
        "exitCode": exit_code,
        "message": message,
    });
    if let Some(r) = result {
        body["result"] = r;
    }
    if let Some(code) = error_code {
        body["error"] = json!({ "code": code, "message": message });
    }
    println!("{body}");
    process::exit(exit_code);
}

fn fail(err: LinearFail) -> ! {
    emit_v2(false, err.exit, &err.message, None, Some(err.code));
}

fn ok(result: Value, message: &str) -> ! {
    emit_v2(true, 0, message, Some(result), None);
}

fn request_inner(doc: &Value) -> &Value {
    doc.get("request").unwrap_or(doc)
}

fn optional_str(req: &Value, key: &str) -> Option<String> {
    req.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn required_str(req: &Value, key: &str) -> Result<String, LinearFail> {
    optional_str(req, key).ok_or_else(|| LinearFail {
        code: "LINEAR_GRAPHQL_ERROR",
        message: format!("request.{key} obligatorio"),
        exit: 1,
    })
}

fn graphql_url() -> String {
    env::var("SDDIA_LINEAR_API_URL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_GRAPHQL_URL.to_string())
}

fn resolve_token() -> Result<String, LinearFail> {
    if lab_mock_outbound_enabled() {
        return Ok("lab-mock-token".into());
    }
    env::var("LINEAR_API_TOKEN")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| LinearFail {
            code: "LINEAR_AUTH_MISSING",
            message: "LINEAR_API_TOKEN ausente".into(),
            exit: 1,
        })
}

fn issue_reduced(node: &Value) -> Value {
    let labels: Vec<Value> = node
        .pointer("/labels/nodes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|n| n.get("name").and_then(|x| x.as_str()))
                .map(|name| json!(name))
                .collect()
        })
        .unwrap_or_default();
    json!({
        "id": node.get("id"),
        "identifier": node.get("identifier"),
        "title": node.get("title"),
        "state": node.pointer("/state/name"),
        "priority": node.get("priority"),
        "labels": labels,
        "parent": node.pointer("/parent/identifier"),
        "children": node.pointer("/children/nodes").and_then(|c| c.as_array()).map(|a| {
            a.iter()
                .filter_map(|n| n.get("identifier").cloned())
                .collect::<Vec<_>>()
        }),
        "url": node.get("url"),
        "updated_at": node.get("updatedAt"),
    })
}

fn graphql_post(url: &str, token: &str, query: &str, variables: Value) -> Result<Value, LinearFail> {
    let agent = ureq::AgentBuilder::new().timeout(Duration::from_secs(30)).build();
    let payload = json!({ "query": query, "variables": variables });
    let resp = agent
        .post(url)
        .set("Content-Type", "application/json")
        .set("Authorization", token)
        .send_json(payload)
        .map_err(|e| LinearFail {
            code: "LINEAR_TRANSPORT",
            message: format!("http-post-failed: {e}"),
            exit: 1,
        })?;
    let status = resp.status();
    let body: Value = resp.into_json().map_err(|e| LinearFail {
        code: "LINEAR_TRANSPORT",
        message: format!("body-not-json: {e}"),
        exit: 1,
    })?;
    if status == 429 {
        return Err(LinearFail {
            code: "LINEAR_RATE_LIMITED",
            message: "rate limited".into(),
            exit: 1,
        });
    }
    if status == 401 || status == 403 {
        return Err(LinearFail {
            code: "LINEAR_AUTH_REJECTED",
            message: format!("http-status-{status}"),
            exit: 1,
        });
    }
    if status >= 400 {
        return Err(LinearFail {
            code: "LINEAR_TRANSPORT",
            message: format!("http-status-{status}: {body}"),
            exit: 1,
        });
    }
    if body.get("errors").is_some() {
        return Err(LinearFail {
            code: "LINEAR_GRAPHQL_ERROR",
            message: body.to_string(),
            exit: 1,
        });
    }
    Ok(body)
}

fn build_list_issues_query(include_label_filter: bool) -> &'static str {
    if include_label_filter {
        r#"query ListIssues($teamKey: String!, $first: Int!, $after: String, $labels: [String!]) {
  team(key: $teamKey) {
    issues(first: $first, after: $after, filter: { labels: { name: { in: $labels } } }) {
      nodes { id identifier title priority url updatedAt state { name } labels { nodes { name } } parent { identifier } children { nodes { identifier } } }
      pageInfo { hasNextPage endCursor }
    }
  }
}"#
    } else {
        r#"query ListIssues($teamKey: String!, $first: Int!, $after: String) {
  team(key: $teamKey) {
    issues(first: $first, after: $after) {
      nodes { id identifier title priority url updatedAt state { name } labels { nodes { name } } parent { identifier } children { nodes { identifier } } }
      pageInfo { hasNextPage endCursor }
    }
  }
}"#
    }
}

fn lab_fetch_issue_fixture(issue_ref: &str) -> Value {
    let base = |state: &str, labels: &[&str], parent: Option<&str>, children: Vec<&str>| {
        json!({
            "id": format!("issue-uuid-{issue_ref}"),
            "identifier": issue_ref,
            "title": format!("lab mock {issue_ref}"),
            "state": state,
            "priority": 0,
            "labels": labels,
            "parent": parent,
            "children": children,
            "url": format!("https://linear.app/issue/{issue_ref}"),
            "updated_at": "2026-10-02T00:00:00Z",
        })
    };
    match issue_ref {
        "LAB-PBI-OK" | "LAB-PBI-CYCLE" => {
            base("Backlog", &["pbi"], Some("LAB-HU-1"), vec![])
        }
        "LAB-PBI-BADLABEL" => base("Backlog", &["hu"], Some("LAB-HU-1"), vec![]),
        "LAB-PBI-BADPARENT" => base("Backlog", &["pbi"], Some("LAB-HU-OTHER"), vec![]),
        "LAB-HU-1" => base("Backlog", &["hu"], None, vec!["LAB-PBI-OK", "LAB-PBI-CYCLE"]),
        "LAB-HU-OTHER" => base("Backlog", &["hu"], None, vec![]),
        "LAB-HU-AC9-READY" => base(
            "In Progress",
            &["hu"],
            None,
            vec!["LAB-PBI-CHILD-A", "LAB-PBI-CHILD-B", "LAB-PBI-MERGE-LAST"],
        ),
        "LAB-PBI-CHILD-A" | "LAB-PBI-CHILD-B" => {
            base("Done", &["pbi"], Some("LAB-HU-AC9-READY"), vec![])
        }
        "LAB-PBI-MERGE-LAST" => base("In Review", &["pbi"], Some("LAB-HU-AC9-READY"), vec![]),
        "LAB-PBI-CHILD-OPEN" => base("In Progress", &["pbi"], Some("LAB-HU-AC9-PENDING"), vec![]),
        "LAB-HU-AC9-PENDING" => base(
            "In Progress",
            &["hu"],
            None,
            vec!["LAB-PBI-CHILD-OPEN", "LAB-PBI-CHILD-B"],
        ),
        _ => base("Backlog", &["hu"], None, vec![]),
    }
}

fn lab_inline_mock(req: &Value) -> Result<Value, LinearFail> {
    let inner = request_inner(req);
    let op = required_str(inner, "operation")?;
    match op.as_str() {
        "fetch_issue" => {
            let issue_ref = required_str(inner, "issue_ref")?;
            if issue_ref == "MISSING-1" {
                return Err(LinearFail {
                    code: "LINEAR_NOT_FOUND",
                    message: "issue not found".into(),
                    exit: 1,
                });
            }
            Ok(lab_fetch_issue_fixture(&issue_ref))
        }
        "list_issues" => {
            let team_key = required_str(inner, "team_key")?;
            let labels = inner.get("labels").and_then(|v| v.as_array());
            let items = if labels.map(|a| !a.is_empty()).unwrap_or(false) {
                vec![json!({
                    "id": "i1",
                    "identifier": format!("{team_key}-1"),
                    "title": "HU mock",
                    "state": "Backlog",
                    "priority": 2,
                    "labels": ["hu"],
                    "parent": null,
                    "children": [],
                    "url": format!("https://linear.app/{team_key}-1"),
                    "updated_at": "2026-10-02T00:00:00Z",
                })]
            } else {
                vec![
                    json!({
                        "id": "i1",
                        "identifier": format!("{team_key}-1"),
                        "title": "HU mock",
                        "state": "Backlog",
                        "priority": 2,
                        "labels": ["hu"],
                        "parent": null,
                        "children": [],
                        "url": format!("https://linear.app/{team_key}-1"),
                        "updated_at": "2026-10-02T00:00:00Z",
                    }),
                    json!({
                        "id": "i2",
                        "identifier": format!("{team_key}-2"),
                        "title": "other label",
                        "state": "Backlog",
                        "priority": 1,
                        "labels": ["other"],
                        "parent": null,
                        "children": [],
                        "url": format!("https://linear.app/{team_key}-2"),
                        "updated_at": "2026-10-02T00:00:00Z",
                    }),
                ]
            };
            Ok(json!({
                "items": items,
                "page_info": { "has_next_page": false, "end_cursor": null }
            }))
        }
        "update_issue_state" => {
            if optional_str(inner, "state_name").as_deref() == Some("Ambiguous") {
                return Err(LinearFail {
                    code: "LINEAR_STATE_AMBIGUOUS",
                    message: "multiple workflow states match".into(),
                    exit: 1,
                });
            }
            if optional_str(inner, "state_name").as_deref() == Some("Unknown") {
                return Err(LinearFail {
                    code: "LINEAR_STATE_UNKNOWN",
                    message: "workflow state not found".into(),
                    exit: 1,
                });
            }
            let issue_ref = required_str(inner, "issue_ref")?;
            Ok(json!({
                "issue_ref": issue_ref,
                "previous_state": "Backlog",
                "state": inner.get("state_name").or_else(|| inner.get("state_id")).cloned().unwrap_or(json!("In Progress")),
            }))
        }
        "create_comment" => {
            let issue_ref = required_str(inner, "issue_ref")?;
            Ok(json!({
                "comment_id": "comment-lab-1",
                "url": format!("https://linear.app/issue/{issue_ref}#comment-lab-1"),
            }))
        }
        "update_issue_description" => {
            let issue_ref = required_str(inner, "issue_ref")?;
            let description = required_str(inner, "description")?;
            Ok(json!({
                "issue_ref": issue_ref,
                "description_bytes": description.len(),
            }))
        }
        other => Err(LinearFail {
            code: "LINEAR_GRAPHQL_ERROR",
            message: format!("operation no soportada: {other}"),
            exit: 1,
        }),
    }
}

fn run_fetch_issue(req: &Value, url: &str, token: &str) -> Result<Value, LinearFail> {
    let inner = request_inner(req);
    let issue_ref = required_str(inner, "issue_ref")?;
    let query = r#"query FetchIssue($ref: String!) {
  issue(id: $ref) { id identifier title priority url updatedAt state { name } labels { nodes { name } } parent { identifier } children { nodes { identifier } } }
}"#;
    let mut variables = Map::new();
    variables.insert("ref".into(), json!(issue_ref));
    let body = graphql_post(url, token, query, Value::Object(variables))?;
    let node = body
        .pointer("/data/issue")
        .ok_or_else(|| LinearFail {
            code: "LINEAR_NOT_FOUND",
            message: "issue not found".into(),
            exit: 1,
        })?;
    if node.is_null() {
        return Err(LinearFail {
            code: "LINEAR_NOT_FOUND",
            message: "issue not found".into(),
            exit: 1,
        });
    }
    Ok(issue_reduced(node))
}

fn run_list_issues(req: &Value, url: &str, token: &str) -> Result<Value, LinearFail> {
    let inner = request_inner(req);
    let team_key = required_str(inner, "team_key")?;
    let first = inner.get("first").and_then(|v| v.as_u64()).unwrap_or(50).min(100) as i64;
    let after = optional_str(inner, "after");
    let label_filter: Option<Vec<String>> = inner
        .get("labels")
        .and_then(|v| {
            v.as_array().map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str().map(str::to_string))
                    .collect::<Vec<_>>()
            })
        })
        .filter(|v| !v.is_empty());

    let include_labels = label_filter.is_some();
    let query = build_list_issues_query(include_labels);
    let mut variables = Map::new();
    variables.insert("teamKey".into(), json!(team_key));
    variables.insert("first".into(), json!(first));
    if let Some(a) = after {
        variables.insert("after".into(), json!(a));
    }
    if let Some(labels) = label_filter {
        variables.insert("labels".into(), json!(labels));
    }

    let body = graphql_post(url, token, query, Value::Object(variables))?;
    let issues = body
        .pointer("/data/team/issues/nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let page = body.pointer("/data/team/issues/pageInfo").cloned().unwrap_or(json!({}));
    let items: Vec<Value> = issues.iter().map(issue_reduced).collect();
    Ok(json!({
        "items": items,
        "page_info": {
            "has_next_page": page.get("hasNextPage"),
            "end_cursor": page.get("endCursor"),
        }
    }))
}

fn run_update_issue_state(req: &Value, url: &str, token: &str) -> Result<Value, LinearFail> {
    let inner = request_inner(req);
    let issue_ref = required_str(inner, "issue_ref")?;
    if let Some(state_name) = optional_str(inner, "state_name") {
        let team_key = required_str(inner, "team_key")?;
        let states_query = r#"query States($teamKey: String!) {
  team(key: $teamKey) { states { nodes { id name } } }
}"#;
        let body = graphql_post(
            url,
            token,
            states_query,
            json!({ "teamKey": team_key }),
        )?;
        let nodes = body
            .pointer("/data/team/states/nodes")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let matches: Vec<&Value> = nodes
            .iter()
            .filter(|n| n.get("name").and_then(|s| s.as_str()) == Some(state_name.as_str()))
            .collect();
        match matches.len() {
            0 => {
                return Err(LinearFail {
                    code: "LINEAR_STATE_UNKNOWN",
                    message: "workflow state not found".into(),
                    exit: 1,
                })
            }
            1 => {
                let state_id = matches[0]
                    .get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| LinearFail {
                        code: "LINEAR_GRAPHQL_ERROR",
                        message: "state id missing".into(),
                        exit: 1,
                    })?;
                return apply_state_id(url, token, &issue_ref, state_id, &state_name);
            }
            _ => {
                return Err(LinearFail {
                    code: "LINEAR_STATE_AMBIGUOUS",
                    message: "multiple workflow states match".into(),
                    exit: 1,
                })
            }
        }
    }
    let state_id = required_str(inner, "state_id")?;
    apply_state_id(url, token, &issue_ref, &state_id, &state_id)
}

fn apply_state_id(
    url: &str,
    token: &str,
    issue_ref: &str,
    state_id: &str,
    state_label: &str,
) -> Result<Value, LinearFail> {
    let mutation = r#"mutation UpdateState($id: String!, $stateId: String!) {
  issueUpdate(id: $id, input: { stateId: $stateId }) {
    success
    issue { identifier state { name } }
  }
}"#;
    let body = graphql_post(
        url,
        token,
        mutation,
        json!({ "id": issue_ref, "stateId": state_id }),
    )?;
    let success = body
        .pointer("/data/issueUpdate/success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if !success {
        return Err(LinearFail {
            code: "LINEAR_GRAPHQL_ERROR",
            message: "issueUpdate failed".into(),
            exit: 1,
        });
    }
    let prev = body
        .pointer("/data/issueUpdate/issue/state/name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    Ok(json!({
        "issue_ref": issue_ref,
        "previous_state": prev,
        "state": state_label,
    }))
}

fn run_create_comment(req: &Value, url: &str, token: &str) -> Result<Value, LinearFail> {
    let inner = request_inner(req);
    let issue_ref = required_str(inner, "issue_ref")?;
    let body_md = required_str(inner, "body")?;
    let mutation = r#"mutation Comment($issueId: String!, $body: String!) {
  commentCreate(input: { issueId: $issueId, body: $body }) {
    success
    comment { id url }
  }
}"#;
    let resp = graphql_post(
        url,
        token,
        mutation,
        json!({ "issueId": issue_ref, "body": body_md }),
    )?;
    let comment = resp.pointer("/data/commentCreate/comment").ok_or_else(|| LinearFail {
        code: "LINEAR_GRAPHQL_ERROR",
        message: "commentCreate failed".into(),
        exit: 1,
    })?;
    Ok(json!({
        "comment_id": comment.get("id"),
        "url": comment.get("url"),
    }))
}

fn run_update_issue_description(req: &Value, url: &str, token: &str) -> Result<Value, LinearFail> {
    let inner = request_inner(req);
    let issue_ref = required_str(inner, "issue_ref")?;
    let description = required_str(inner, "description")?;
    let mutation = r#"mutation UpdateDesc($id: String!, $description: String!) {
  issueUpdate(id: $id, input: { description: $description }) {
    success
    issue { identifier }
  }
}"#;
    let resp = graphql_post(
        url,
        token,
        mutation,
        json!({ "id": issue_ref, "description": description }),
    )?;
    let success = resp
        .pointer("/data/issueUpdate/success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if !success {
        return Err(LinearFail {
            code: "LINEAR_GRAPHQL_ERROR",
            message: "issueUpdate description failed".into(),
            exit: 1,
        });
    }
    let ident = resp
        .pointer("/data/issueUpdate/issue/identifier")
        .and_then(|v| v.as_str())
        .unwrap_or(&issue_ref);
    Ok(json!({
        "issue_ref": ident,
        "description_bytes": description.len(),
    }))
}

fn dispatch(req: &Value) -> Result<Value, LinearFail> {
    if lab_mock_outbound_enabled() && lab_mock_linear_url().is_none() {
        return lab_inline_mock(req);
    }
    let token = resolve_token()?;
    let url = if let Some(mock) = lab_mock_linear_url() {
        mock
    } else {
        graphql_url()
    };
    let inner = request_inner(req);
    let op = required_str(inner, "operation")?;
    match op.as_str() {
        "fetch_issue" => run_fetch_issue(req, &url, &token),
        "list_issues" => run_list_issues(req, &url, &token),
        "update_issue_state" => run_update_issue_state(req, &url, &token),
        "create_comment" => run_create_comment(req, &url, &token),
        "update_issue_description" => run_update_issue_description(req, &url, &token),
        other => Err(LinearFail {
            code: "LINEAR_GRAPHQL_ERROR",
            message: format!("operation no soportada: {other}"),
            exit: 1,
        }),
    }
}

fn main() {
    let req = read_stdin_json();
    match dispatch(&req) {
        Ok(result) => ok(result, "linear-tracker-adapter ok"),
        Err(e) => fail(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use tiny_http::{Response, Server};

    static TEST_ENV: Mutex<()> = Mutex::new(());

    fn with_lab<F: FnOnce()>(f: F) {
        let _guard = TEST_ENV.lock().unwrap();
        env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        env::remove_var("LINEAR_API_TOKEN");
        env::remove_var("SDDIA_LAB_MOCK_LINEAR_URL");
        f();
        env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
    }

    #[test]
    fn auth_missing_without_lab() {
        let _guard = TEST_ENV.lock().unwrap();
        env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        env::remove_var("LINEAR_API_TOKEN");
        let err = dispatch(&json!({"request": {"operation": "fetch_issue", "issue_ref": "BX-1"}})).unwrap_err();
        assert_eq!(err.code, "LINEAR_AUTH_MISSING");
    }

    #[test]
    fn four_operations_lab_inline() {
        with_lab(|| {
            let fetch = dispatch(&json!({"request": {"operation": "fetch_issue", "issue_ref": "BX-9"}})).unwrap();
            assert_eq!(fetch.get("identifier"), Some(&json!("BX-9")));

            let list = dispatch(&json!({"request": {"operation": "list_issues", "team_key": "BX"}})).unwrap();
            assert_eq!(list["items"].as_array().map(|a| a.len()), Some(2));

            let list_hu = dispatch(&json!({"request": {"operation": "list_issues", "team_key": "BX", "labels": ["hu"]}})).unwrap();
            assert_eq!(list_hu["items"].as_array().map(|a| a.len()), Some(1));

            let st = dispatch(&json!({"request": {"operation": "update_issue_state", "issue_ref": "BX-1", "state_name": "In Progress", "team_key": "BX"}})).unwrap();
            assert_eq!(st.get("issue_ref"), Some(&json!("BX-1")));

            let c = dispatch(&json!({"request": {"operation": "create_comment", "issue_ref": "BX-1", "body": "hi"}})).unwrap();
            assert!(c.get("comment_id").is_some());
        });
    }

    #[test]
    fn state_errors_do_not_mutate_in_lab() {
        with_lab(|| {
            let e = dispatch(&json!({"request": {"operation": "update_issue_state", "issue_ref": "BX-1", "state_name": "Ambiguous", "team_key": "BX"}})).unwrap_err();
            assert_eq!(e.code, "LINEAR_STATE_AMBIGUOUS");
            let e2 = dispatch(&json!({"request": {"operation": "update_issue_state", "issue_ref": "BX-1", "state_name": "Unknown", "team_key": "BX"}})).unwrap_err();
            assert_eq!(e2.code, "LINEAR_STATE_UNKNOWN");
        });
    }

    #[test]
    fn list_issues_query_includes_label_filter() {
        let q = build_list_issues_query(true);
        assert!(q.contains("labels: { name: { in: $labels } }"));
        let q2 = build_list_issues_query(false);
        assert!(!q2.contains("$labels"));
    }

    #[test]
    fn mock_http_server_list_issues() {
        let _guard = TEST_ENV.lock().unwrap();
        let captured = Arc::new(Mutex::new(String::new()));
        let cap2 = captured.clone();
        let server = Server::http("127.0.0.1:0").unwrap();
        let listen = match server.server_addr() {
            tiny_http::ListenAddr::IP(addr) => addr,
            _ => panic!("expected IP listen addr"),
        };
        let port = listen.port();
        thread::spawn(move || {
            if let Some(mut req) = server.recv().ok() {
                let mut body = String::new();
                let _ = req.as_reader().read_to_string(&mut body);
                *cap2.lock().unwrap() = body;
                let resp = json!({"data": {"team": {"issues": {"nodes": [], "pageInfo": {"hasNextPage": false, "endCursor": null}}}}});
                let _ = req.respond(Response::from_string(resp.to_string()).with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
                ));
            }
        });
        env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        env::set_var("SDDIA_LAB_MOCK_LINEAR_URL", format!("http://127.0.0.1:{port}/graphql"));
        env::set_var("LINEAR_API_TOKEN", "test-token");
        let _ = dispatch(&json!({"request": {"operation": "list_issues", "team_key": "BX", "labels": ["hu"]}}));
        env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        env::remove_var("SDDIA_LAB_MOCK_LINEAR_URL");
        env::remove_var("LINEAR_API_TOKEN");
        let body = captured.lock().unwrap().clone();
        assert!(body.contains("hu"), "{body}");
    }

    #[test]
    fn envelope_never_contains_token() {
        with_lab(|| {
            env::set_var("LINEAR_API_TOKEN", "super-secret-token");
            let r = dispatch(&json!({"request": {"operation": "fetch_issue", "issue_ref": "BX-1"}})).unwrap();
            let blob = r.to_string();
            assert!(!blob.contains("super-secret-token"));
            env::remove_var("LINEAR_API_TOKEN");
        });
    }
}
