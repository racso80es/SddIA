const $ = (id) => document.getElementById(id);

const POLL_MS = 1500;
/** Timeout corto mientras no hay rastro de ciclo. */
const POLL_TIMEOUT_MS = 120000;
/** Timeout largo tras initialized/awaiting_agents (agentes IDE pueden tardar). */
const POLL_TIMEOUT_LIFECYCLE_MS = 30 * 60 * 1000;

let progressSource = null;

function setStatus(text, kind) {
  const el = $("status");
  if (!el) return;
  el.textContent = text || "";
  el.dataset.kind = kind || "";
}

function payloadOf(data) {
  return data.data && typeof data.data === "object" ? data.data : data;
}

function setBusy(busy) {
  const chat = $("chat");
  const forge = $("forge");
  const sync = $("sync-genome");
  if (chat) chat.disabled = busy;
  if (forge) forge.disabled = busy;
  if (sync) sync.disabled = busy;
}

function closeProgressStream() {
  if (progressSource) {
    progressSource.close();
    progressSource = null;
  }
}

function appendProgressTrace(trace) {
  const consoleEl = $("progress-console");
  if (!consoleEl || !trace || typeof trace !== "object") return;

  const severity = trace.severity || "info";
  const agent = trace.source_agent || "?";
  const phase = trace.phase || "?";
  const msg = trace.message || "";

  const line = document.createElement("div");
  line.className = `progress-line severity-${severity}`;
  line.innerHTML = `<span class="badge">[${agent}]</span><span class="phase">${phase}</span> · ${msg}`;
  consoleEl.appendChild(line);
  consoleEl.scrollTop = consoleEl.scrollHeight;
}

function openProgressStream(correlationId) {
  closeProgressStream();
  const consoleEl = $("progress-console");
  if (consoleEl) consoleEl.innerHTML = "";

  const url = `/api/progress/stream?correlation_id=${encodeURIComponent(correlationId)}`;
  progressSource = new EventSource(url);
  progressSource.addEventListener("progress", (ev) => {
    try {
      appendProgressTrace(JSON.parse(ev.data));
    } catch (_) {
      /* ignore malformed frame */
    }
  });
  progressSource.onerror = () => {
    /* poll status sigue; reconexión nativa EventSource */
  };
}

async function pollStatus(eventId, out, ackText) {
  const started = Date.now();
  let deadline = started + POLL_TIMEOUT_MS;
  let lifecycleArmed = false;
  setStatus(`pending · ${eventId.slice(0, 8)}…`, "pending");

  while (Date.now() < deadline) {
    await new Promise((r) => setTimeout(r, POLL_MS));
    let st;
    try {
      const r = await fetch(`/api/status?event_id=${encodeURIComponent(eventId)}`);
      if (r.status === 404) {
        setStatus("pending · esperando rastro…", "pending");
        out.value = `${ackText}\n\n[estado: pending — evento en tránsito]`;
        continue;
      }
      st = await r.json();
      if (!r.ok || !st.success) {
        setStatus(`error · ${st.message || r.status}`, "failed");
        out.value = `${ackText}\n\n[error status] ${st.message || r.status}`;
        closeProgressStream();
        return;
      }
    } catch (e) {
      setStatus(`red · ${e}`, "failed");
      out.value = `${ackText}\n\n[fallo red status] ${e}`;
      closeProgressStream();
      return;
    }

    const status = st.status || "pending";
    setStatus(`${status} · ${eventId.slice(0, 8)}…`, status);
    out.value = `${ackText}\n\n[estado: ${status}] ${st.message || ""}`;

    const orch = st.orchestration || {};
    if (orch.found) {
      out.value += `\nproceso=${orch.process_name || "?"} status=${orch.process_status || "?"}`;
      if (orch.cycle_phase) {
        out.value += ` cycle_phase=${orch.cycle_phase}`;
      }
    }

    if (status === "completed" || status === "failed") {
      closeProgressStream();
      return;
    }
    if (
      !lifecycleArmed &&
      (status === "initialized" || status === "awaiting_agents")
    ) {
      lifecycleArmed = true;
      deadline = Math.max(deadline, Date.now() + POLL_TIMEOUT_LIFECYCLE_MS);
    }
  }

  setStatus("timeout", "failed");
  out.value = `${ackText}\n\n[timeout] sin completed/failed en ${(deadline - started) / 1000}s`;
  closeProgressStream();
}

async function enviarChat() {
  const out = $("output");
  const prompt = $("prompt").value.trim();
  if (!prompt) return;

  closeProgressStream();
  setBusy(true);
  out.value = "";
  setStatus("chat SSE…", "pending");

  try {
    const r = await fetch("/api/chat", {
      method: "POST",
      headers: { "Content-Type": "application/json", Accept: "text/event-stream" },
      body: JSON.stringify({ prompt }),
    });
    if (!r.ok) {
      let msg = r.statusText;
      try {
        const j = await r.json();
        msg = j.message || j.error || msg;
      } catch (_) {
        /* ignore */
      }
      setStatus("error", "failed");
      out.value = `[error] ${msg}`;
      return;
    }

    const reader = r.body.getReader();
    const dec = new TextDecoder();
    let buf = "";
    let acc = "";
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      buf += dec.decode(value, { stream: true });
      const parts = buf.split("\n\n");
      buf = parts.pop() || "";
      for (const block of parts) {
        for (const line of block.split("\n")) {
          if (line.startsWith("data: ")) {
            const token = line.slice(6);
            if (token.startsWith("[kalma2-meta]")) {
              setStatus(token.replace("[kalma2-meta] ", "meta · "), "pending");
              continue;
            }
            acc += (acc ? " " : "") + token;
            out.value = acc;
          }
        }
      }
    }
    setStatus("ok", "ok");
    if (!acc) out.value = "(stream vacío)";
  } catch (e) {
    setStatus("red", "failed");
    out.value = `[fallo red] ${e}`;
  } finally {
    setBusy(false);
  }
}

async function forjarProceso() {
  const out = $("output");
  const prompt = $("prompt").value.trim();
  if (!prompt) return;

  closeProgressStream();
  setBusy(true);
  out.value = "…encolando proceso";
  setStatus("execute…", "pending");

  try {
    const r = await fetch("/api/execute", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ prompt }),
    });
    const data = await r.json();
    const payload = payloadOf(data);
    const text = data.response ?? payload.response;

    const acceptedId =
      data.correlation_id || data.event_id || payload.correlation_id || payload.event_id;
    if (data.success && (r.status === 202 || data.status === "accepted") && acceptedId) {
      const ack =
        data.message ||
        text ||
        `intención aceptada (${acceptedId}); consultando estado…`;
      out.value = ack;
      openProgressStream(acceptedId);
      await pollStatus(acceptedId, out, ack);
      return;
    }

    const ok = data.success && text;

    if (!ok) {
      setStatus("error", "failed");
      out.value = `[error] ${data.message ?? data.error ?? "motor falló"}`;
      return;
    }

    if (payload.emitted && payload.event_id) {
      const eventId = payload.event_id;
      out.value = text;
      openProgressStream(eventId);
      await pollStatus(eventId, out, text);
      return;
    }

    setStatus("ok", "ok");
    out.value = text;
  } catch (e) {
    setStatus("red", "failed");
    out.value = `[fallo red] ${e}`;
  } finally {
    closeProgressStream();
    setBusy(false);
  }
}

async function syncGenome() {
  const out = $("output");
  if (!out) return;

  closeProgressStream();
  setBusy(true);
  out.value = "…solicitando sincronización de genoma";
  setStatus("sync…", "pending");

  try {
    const r = await fetch("/api/sync-assets", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        asset_id: "c43544f3-c557-4cc3-8a03-7175282f2c88",
        asset_family: "library_codexes",
      }),
    });
    const data = await r.json();

    if (!data.success || r.status !== 202) {
      setStatus("error", "failed");
      out.value = `[error] ${data.message ?? data.error ?? "sync falló"}`;
      return;
    }

    const correlationId = data.correlation_id || data.event_id;
    const ack = data.message || `sync-client-assets aceptado (${correlationId}); observando progreso…`;
    out.value = ack;
    openProgressStream(correlationId);
    await pollStatus(correlationId, out, ack);
  } catch (e) {
    setStatus("red", "failed");
    out.value = `[fallo red] ${e}`;
  } finally {
    setBusy(false);
  }
}

let cognitiveSource = null;

function closeCognitiveStream() {
  if (cognitiveSource) {
    cognitiveSource.close();
    cognitiveSource = null;
  }
}

function renderCognitiveSnapshot(cog) {
  if (!cog || typeof cog !== "object") return;
  const prompt = Number(cog.tokens_prompt_total || 0);
  const completion = Number(cog.tokens_completion_total || 0);
  const tokensEl = $("cognitive-tokens");
  const modelEl = $("cognitive-model");
  const latEl = $("cognitive-latency");
  const quotaEl = $("cognitive-quota");
  if (tokensEl) tokensEl.textContent = `${prompt + completion} tokens (${prompt}↑ ${completion}↓)`;
  if (modelEl) modelEl.textContent = `modelo: ${cog.last_model || "—"}`;
  if (latEl) {
    const avg = Number(cog.latency_ms_avg || 0);
    latEl.textContent = `latencia media: ${avg ? `${Math.round(avg)} ms` : "—"}`;
  }
  if (quotaEl) {
    quotaEl.classList.toggle("hidden", !cog.quota_alert);
  }
}

async function loadCognitiveSnapshot() {
  try {
    const r = await fetch("/api/telemetry/cognitive");
    const data = await r.json();
    if (data.success && data.cognitive) renderCognitiveSnapshot(data.cognitive);
  } catch (_) {
    /* snapshot opcional */
  }
}

function openCognitiveStream() {
  closeCognitiveStream();
  cognitiveSource = new EventSource("/api/telemetry/stream");
  cognitiveSource.addEventListener("cognitive", (ev) => {
    try {
      const body = JSON.parse(ev.data);
      const receipt = body?.payload?.telemetry_receipt;
      if (receipt) {
        renderCognitiveSnapshot({
          tokens_prompt_total: receipt.prompt_tokens,
          tokens_completion_total: receipt.completion_tokens,
          last_model: receipt.llm_model,
          latency_ms_avg: receipt.provider_latency_ms,
          quota_alert: false,
        });
      }
    } catch (_) {
      /* ignore malformed frame */
    }
  });
  cognitiveSource.onerror = () => {
    /* EventSource reconecta */
  };
}

function renderSystemHealth(data) {
  const matrix = $("health-matrix");
  const statusEl = $("health-map-status");
  if (!matrix) return;
  matrix.innerHTML = "";
  if (statusEl) {
    const ms = data?.map_status || "—";
    const warn = data?.warning ? ` — ${data.warning}` : "";
    statusEl.textContent = `mapa: ${ms}${warn}`;
  }
  const rows = Array.isArray(data?.rows) ? data.rows : [];
  if (!rows.length) {
    matrix.textContent = "sin filas de salud";
    return;
  }
  for (const row of rows) {
    const color = row.color || "gray";
    const el = document.createElement("div");
    el.className = `health-row health-${color}`;
    el.title = row.reason || "";
    const dot = document.createElement("span");
    dot.className = "health-dot";
    dot.setAttribute("aria-hidden", "true");
    const label = document.createElement("span");
    label.className = "health-label";
    label.textContent = `${row.family || "?"}:${row.id || "?"}`;
    const reason = document.createElement("span");
    reason.className = "health-reason";
    reason.textContent = row.reason || "";
    el.append(dot, label, reason);
    matrix.appendChild(el);
  }
}

async function loadSystemHealth() {
  try {
    const r = await fetch("/api/system-health");
    const data = await r.json();
    if (data.success !== false) renderSystemHealth(data);
  } catch (_) {
    /* snapshot opcional */
  }
}

document.addEventListener("DOMContentLoaded", () => {
  $("chat").addEventListener("click", enviarChat);
  const forgeBtn = $("forge");
  if (forgeBtn) forgeBtn.addEventListener("click", forjarProceso);
  const syncBtn = $("sync-genome");
  if (syncBtn) syncBtn.addEventListener("click", syncGenome);
  $("prompt").addEventListener("keydown", (e) => {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      enviarChat();
    }
  });
  applyConsumerProfileUi();
  loadEmailInbox();
  loadCognitiveSnapshot();
  openCognitiveStream();
  loadSystemHealth();
});

/** Filtro C: oculta Forjar Proceso en perfil consumidor. */
async function applyConsumerProfileUi() {
  const forge = $("forge");
  if (!forge) return;
  try {
    const r = await fetch("/api/runtime-profile");
    const data = await r.json();
    if (data && data.forge_allowed === false) {
      forge.hidden = true;
      forge.disabled = true;
      forge.removeAttribute("aria-hidden");
      forge.setAttribute("aria-hidden", "true");
    }
  } catch (_) {
    /* lab sin bridge actualizado: botón visible (fail-open engineering) */
  }
}

async function loadEmailInbox() {
  const root = $("email-inbox");
  if (!root) return;
  root.textContent = "cargando…";
  try {
    const r = await fetch("/api/email-inbox");
    const data = await r.json();
    // #region agent log
    fetch('http://127.0.0.1:7400/ingest/b4cd7cca-3613-4ae7-9d27-95676f671320',{method:'POST',headers:{'Content-Type':'application/json','X-Debug-Session-Id':'478d0f'},body:JSON.stringify({sessionId:'478d0f',runId:'post-fix',hypothesisId:'W',location:'app.js:loadEmailInbox',message:'inbox fetch',data:{http:r.status,success:!!data.success,n:Array.isArray(data.items)?data.items.length:-1},timestamp:Date.now()})}).catch(()=>{});
    // #endregion
    const items = Array.isArray(data.items) ? data.items : [];
    if (!items.length) {
      root.textContent = "Sin fricción accionable.";
      return;
    }
    root.innerHTML = "";
    for (const it of items) {
      const card = document.createElement("article");
      card.className = "inbox-card";
      const from = it.from || "?";
      const subject = it.subject || "(sin asunto)";
      const uid = it.message_uid || "";
      const eid = it.event_id || "";
      card.innerHTML = `<div class="meta">${from} · uid ${uid}</div>
        <div class="subject"></div>
        <div class="quick">
          <button type="button" data-action="archive">Archivar</button>
          <button type="button" data-action="draft" class="secondary">Generar borrador</button>
          <button type="button" data-action="delegate" class="secondary">Delegar</button>
        </div>`;
      card.querySelector(".subject").textContent = subject;
      card.querySelectorAll("button[data-action]").forEach((btn) => {
        btn.addEventListener("click", () =>
          sendQuickAction(uid, btn.getAttribute("data-action"), eid, btn)
        );
      });
      root.appendChild(card);
    }
  } catch (e) {
    root.textContent = `error inbox: ${e}`;
  }
}

async function sendQuickAction(messageUid, action, sourceEventId, btn) {
  if (btn) btn.disabled = true;
  try {
    const r = await fetch("/api/email-quick-action", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        message_uid: String(messageUid),
        action,
        source_event_id: sourceEventId || undefined,
      }),
    });
    const data = await r.json();
    if (!data.success) {
      setStatus(data.message || "acción fallida", "failed");
      return;
    }
    setStatus(`acción ${action} · ${String(data.event_id || "").slice(0, 8)}…`, "ok");
  } catch (e) {
    setStatus(`red · ${e}`, "failed");
  } finally {
    if (btn) btn.disabled = false;
  }
}
