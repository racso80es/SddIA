//! API compartida SIGTERM/SIGINT → flag de shutdown (D-RT / AUT-111).

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
static HANDLERS_INSTALLED: AtomicBool = AtomicBool::new(false);

/// Marca solicitud de terminación limpia (también usable desde tests).
pub fn request_shutdown() {
    SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
}

/// ¿El bucle continuo debe salir?
pub fn shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::SeqCst)
}

/// Resetea la flag (tests / reinicio controlado del harness).
pub fn clear_shutdown_request() {
    SHUTDOWN_REQUESTED.store(false, Ordering::SeqCst);
}

/// Instala handlers SIGTERM/SIGINT vía `ctrlc` (idempotente).
/// El handler solo activa la flag; sin panics ni I/O pesado.
pub fn install_termination_handlers() -> Result<(), String> {
    if HANDLERS_INSTALLED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    ctrlc::set_handler(|| {
        request_shutdown();
    })
    .map_err(|e| format!("install_termination_handlers: {e}"))
}

/// Duerme en rodajas de hasta 200ms hasta agotar `duration` o recibir señal de shutdown.
/// Devuelve `true` si se solicitó shutdown durante la espera.
pub fn sleep_interruptible(duration: Duration) -> bool {
    if shutdown_requested() {
        return true;
    }
    let deadline = Instant::now() + duration;
    let slice = Duration::from_millis(200);
    while !shutdown_requested() {
        let now = Instant::now();
        if now >= deadline {
            return false;
        }
        thread::sleep((deadline - now).min(slice));
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_sets_shutdown_flag() {
        clear_shutdown_request();
        assert!(!shutdown_requested());
        request_shutdown();
        assert!(shutdown_requested());
        clear_shutdown_request();
        assert!(!shutdown_requested());
    }

    #[test]
    fn sleep_interruptible_returns_true_when_requested() {
        clear_shutdown_request();
        request_shutdown();
        assert!(sleep_interruptible(Duration::from_secs(5)));
        clear_shutdown_request();
    }
}
