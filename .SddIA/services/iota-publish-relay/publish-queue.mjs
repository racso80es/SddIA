/**
 * Cola FIFO async: un trabajo a la vez.
 * El siguiente no arranca hasta que el precedente se resuelve (éxito o error).
 */
export function createSerialQueue() {
  let tail = Promise.resolve();
  return function enqueue(fn) {
    const run = tail.then(
      () => fn(),
      () => fn(),
    );
    tail = run.then(
      () => undefined,
      () => undefined,
    );
    return run;
  };
}
