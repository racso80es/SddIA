const $ = (id) => document.getElementById(id);

const POLL_MS = 1500;
const POLL_TIMEOUT_MS = 120000;

function setStatus(text, kind) {
  const el = $("status");
  if (!el) return;
  el.textContent = text || "";
  el.dataset.kind = kind || "";
}

function payloadOf(data) {
  return data.data && typeof data.data === "object" ? data.data : data;
}

async function pollStatus(eventId, out, ackText) {
  const started = Date.now();
  setStatus(`pending · ${eventId.slice(0, 8)}…`, "pending");

  while (Date.now() - started < POLL_TIMEOUT_MS) {
    await new Promise((r) => setTimeout(r, POLL_MS));
    let st;
    try {
      const r = await fetch(`/api/status?event_id=${encodeURIComponent(eventId)}`);
      if (r.status === 404) {
        // Dominio puede haberse purgado post-route; seguir hasta PEC o timeout.
        setStatus("pending · esperando rastro…", "pending");
        out.value = `${ackText}\n\n[estado: pending — evento en tránsito]`;
        continue;
      }
      st = await r.json();
      if (!r.ok || !st.success) {
        setStatus(`error · ${st.message || r.status}`, "failed");
        out.value = `${ackText}\n\n[error status] ${st.message || r.status}`;
        return;
      }
    } catch (e) {
      setStatus(`red · ${e}`, "failed");
      out.value = `${ackText}\n\n[fallo red status] ${e}`;
      return;
    }

    const status = st.status || "pending";
    setStatus(`${status} · ${eventId.slice(0, 8)}…`, status);
    out.value = `${ackText}\n\n[estado: ${status}] ${st.message || ""}`;

    // Terminales: cierre de negocio, fallo, o arranque honesto (slice A kalma2-full-cycle).
    if (
      status === "completed" ||
      status === "failed" ||
      status === "initialized" ||
      status === "awaiting_agents"
    ) {
      const orch = st.orchestration || {};
      if (orch.found) {
        out.value += `\nproceso=${orch.process_name || "?"} status=${orch.process_status || "?"}`;
        if (orch.cycle_phase) {
          out.value += ` cycle_phase=${orch.cycle_phase}`;
        }
      }
      return;
    }
  }

  setStatus("timeout", "failed");
  out.value = `${ackText}\n\n[timeout] sin completed/failed/initialized en ${POLL_TIMEOUT_MS / 1000}s`;
}

async function enviar() {
  const btn = $("send");
  const out = $("output");
  const prompt = $("prompt").value.trim();
  if (!prompt) return;

  btn.disabled = true;
  out.value = "…procesando";
  setStatus("enviando…", "pending");

  try {
    const r = await fetch("/api/interact", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ prompt }),
    });
    const data = await r.json();
    const payload = payloadOf(data);
    const text = data.response ?? payload.response;
    const ok = data.success && text;

    if (!ok) {
      setStatus("error", "failed");
      out.value = `[error] ${data.message ?? data.error ?? "motor falló"}`;
      return;
    }

    if (payload.emitted && payload.event_id) {
      const eventId = payload.event_id;
      out.value = text;
      await pollStatus(eventId, out, text);
      return;
    }

    if (payload.degraded === true) {
      setStatus("degradado", "degraded");
      out.value = `[degradado]\n${text}`;
    } else {
      setStatus("ok", "ok");
      out.value = text;
    }
  } catch (e) {
    setStatus("red", "failed");
    out.value = `[fallo red] ${e}`;
  } finally {
    btn.disabled = false;
  }
}

document.addEventListener("DOMContentLoaded", () => {
  $("send").addEventListener("click", enviar);
  $("prompt").addEventListener("keydown", (ev) => {
    if (ev.key === "Enter" && (ev.ctrlKey || ev.metaKey)) {
      ev.preventDefault();
      enviar();
    }
  });
});
