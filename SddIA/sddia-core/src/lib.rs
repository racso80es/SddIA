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
