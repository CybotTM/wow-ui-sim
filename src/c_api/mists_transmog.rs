//! Explicit Mists transmog availability; no native default is inferred.

/// Host-supplied availability for one environment.
#[derive(Debug, Default)]
pub struct MistsTransmogAvailability {
    pub enabled: Option<bool>,
}
