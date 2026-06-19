const $ = (id) => document.getElementById(id);

async function enviar() {
  const btn = $("send");
  const out = $("output");
  const prompt = $("prompt").value.trim();
  if (!prompt) return;

  btn.disabled = true;
  out.value = "…procesando";

  try {
    const r = await fetch("/api/interact", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ prompt }),
    });
    const data = await r.json();
    const text = data.response ?? data.data?.response;
    out.value = data.success && text ? text : `[error] ${data.message ?? data.error ?? "motor falló"}`;
  } catch (e) {
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
