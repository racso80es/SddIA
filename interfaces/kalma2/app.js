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
  if (chat) chat.disabled = busy;
  if (forge) forge.disabled = busy;
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

document.addEventListener("DOMContentLoaded", () => {
  $("chat").addEventListener("click", enviarChat);
  $("forge").addEventListener("click", forjarProceso);
  $("prompt").addEventListener("keydown", (e) => {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      enviarChat();
    }
  });
});
