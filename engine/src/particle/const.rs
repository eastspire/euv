use super::*;

/// The default maximum number of live particles per emitter.
pub(crate) const PARTICLE_DEFAULT_MAX_COUNT: usize = 512;

/// The default emission rate in particles per second.
pub(crate) const PARTICLE_DEFAULT_EMISSION_RATE: f64 = 32.0;

/// The default minimum particle lifetime in seconds.
pub(crate) const PARTICLE_DEFAULT_LIFETIME_MIN: f64 = 0.5;

/// The default maximum particle lifetime in seconds.
pub(crate) const PARTICLE_DEFAULT_LIFETIME_MAX: f64 = 1.5;

/// The default minimum initial particle speed in world units per second.
pub(crate) const PARTICLE_DEFAULT_SPEED_MIN: f64 = 32.0;

/// The default maximum initial particle speed in world units per second.
pub(crate) const PARTICLE_DEFAULT_SPEED_MAX: f64 = 96.0;

/// The default emission direction in radians (straight up, screen space).
pub(crate) const PARTICLE_DEFAULT_ANGLE: f64 = -HALF_PI;

/// The default emission spread in radians (a 45-degree cone).
pub(crate) const PARTICLE_DEFAULT_SPREAD: f64 = PI / 4.0;

/// The default particle radius at birth in world units.
pub(crate) const PARTICLE_DEFAULT_SIZE_START: f64 = 4.0;

/// The default particle radius at death in world units.
pub(crate) const PARTICLE_DEFAULT_SIZE_END: f64 = 0.0;

/// The default seed for the emitter's deterministic random generator.
pub(crate) const PARTICLE_DEFAULT_RNG_SEED: u64 = 0x9E37_79B9_7F4A_7C15;

/// The size of the color quantization palette used to batch particle draws.
///
/// Quantizing each particle's interpolated color to one of `PALETTE_SIZE`
/// preset colors dramatically increases the number of particles that share
/// the same `DrawCommand::FillCircle::color` field, which is the key the
/// DrawList replay loop uses to coalesce `set_fill_style_str` calls. With
/// 32 buckets a typical fade-to-white gradient particles span at most ~5
/// buckets, so a 1000-particle frame emits at most ~5 fill_style changes
/// instead of 1000.
pub(crate) const PARTICLE_PALETTE_SIZE: usize = 32;
