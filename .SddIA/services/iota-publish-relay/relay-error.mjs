/**
 * Formateador de fallos de publish IOTA (instancia).
 * Puro: sin SDK, sin HTTP. El catch de server.mjs serializa err.cause
 * para que iota-immutable-publisher (lee error/feedback) no trunque a "fetch failed".
 */
export function serializeCause(err, depth = 0) {
  if (err == null || depth > 1) return null;
  const c = err.cause;
  if (c == null) return null;
  if (c instanceof Error) {
    const out = {
      name: c.name,
      message: c.message,
    };
    if ("code" in c && c.code != null) out.code = String(c.code);
    if ("syscall" in c && c.syscall != null) out.syscall = String(c.syscall);
    if ("errno" in c && c.errno != null) out.errno = String(c.errno);
    const nested = serializeCause(c, depth + 1);
    if (nested) out.cause = nested;
    return out;
  }
  if (typeof c === "object") {
    const out = {
      name: c.name != null ? String(c.name) : "Cause",
      message: c.message != null ? String(c.message) : String(c),
    };
    if (c.code != null) out.code = String(c.code);
    if (c.syscall != null) out.syscall = String(c.syscall);
    if (c.errno != null) out.errno = String(c.errno);
    return out;
  }
  return { name: typeof c, message: String(c) };
}

function causeSuffix(cause) {
  if (!cause) return "";
  const bits = [cause.code, cause.syscall, cause.message].filter(
    (x) => typeof x === "string" && x.trim().length > 0,
  );
  const uniq = [...new Set(bits)];
  if (uniq.length === 0) return "";
  return ` | cause: ${uniq.join(" ")}`;
}

export function formatPublishFailure(err) {
  const message = err instanceof Error ? err.message : String(err);
  const cause = serializeCause(err);
  const error = `${message}${causeSuffix(cause)}`;
  const body = { error, feedback: error };
  if (cause) body.cause = cause;
  return body;
}
