//! Shared Kernel SddIA — frontera de consumo del Core.
//!
//! Jurisdicción ciega: sin dominio de cliente. Reexporta la tubería I/O
//! canónica (`sddia-io`) y markers documentales de jurisdicción.

pub use sddia_io::*;

/// Markers de jurisdicción Core (documentales / compilador, sin I/O de red).
pub mod jurisdiction {
    /// Identidad lógica del Shared Kernel (producto F1).
    pub const SHARED_KERNEL_ID: &str = "sddia-core";

    /// Semver de producto F1 alineada con npm `@sddia/core`.
    pub const SHARED_KERNEL_VERSION: &str = "0.1.0";

    /// El Core declara frontera; no conoce dominio de instancia.
    pub const DOMAIN_BLIND: bool = true;
}

#[cfg(test)]
mod tests {
    use super::jurisdiction::{DOMAIN_BLIND, SHARED_KERNEL_ID, SHARED_KERNEL_VERSION};

    #[test]
    fn shared_kernel_markers_are_stable() {
        assert_eq!(SHARED_KERNEL_ID, "sddia-core");
        assert_eq!(SHARED_KERNEL_VERSION, "0.1.0");
        assert!(DOMAIN_BLIND);
    }

    #[test]
    fn reexports_sddia_io_envelope_surface() {
        // Compila si `pub use sddia_io::*` permanece como frontera de consumo.
        let _ = std::any::type_name::<super::SddiaResponse<()>>();
    }
}
