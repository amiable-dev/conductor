// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Velocity mapping and calculation engine (v2.2)
//!
//! Provides functions to map trigger velocity to output velocity
//! based on VelocityMapping configuration.
//!
//! # Velocity Mapping Modes
//!
//! ## Fixed
//! Always returns the same velocity value, regardless of trigger velocity.
//! This is the v2.1 behavior and provides backward compatibility.
//!
//! ## PassThrough
//! Output velocity = trigger velocity (1:1 mapping).
//! Preserves the dynamics of the trigger exactly.
//!
//! ## Linear
//! Maps the full input range (0-127) to a configurable output range (min-max).
//! Formula: `output = min + (trigger / 127) * (max - min)`
//!
//! ## Curve
//! Applies non-linear transformation curves:
//! - **Exponential**: Soft hits become louder, hard hits stay near max
//! - **Logarithmic**: Soft hits become quieter, compresses dynamic range
//! - **S-Curve**: Smooth acceleration in the middle range
//!
//! # Examples
//!
//! ```rust
//! use conductor_core::velocity::calculate_velocity;
//! use conductor_core::actions::VelocityMapping;
//!
//! // Pass-through mode
//! let output = calculate_velocity(80, &VelocityMapping::PassThrough);
//! assert_eq!(output, 80);
//!
//! // Linear scaling (0-127 → 50-100)
//! let output = calculate_velocity(127, &VelocityMapping::Linear { min: 50, max: 100 });
//! assert_eq!(output, 100);
//!
//! // Fixed velocity
//! let output = calculate_velocity(80, &VelocityMapping::Fixed { velocity: 64 });
//! assert_eq!(output, 64);
//! ```

use crate::actions::{VelocityCurve, VelocityMapping};

/// Calculate output velocity based on trigger velocity and mapping config
///
/// # Arguments
/// * `trigger_velocity` - Input velocity from trigger event (0-127)
/// * `mapping` - Velocity mapping configuration
///
/// # Returns
/// * Output MIDI velocity (0-127)
///
/// # Panics
/// This function does not panic. All outputs are clamped to the valid MIDI
/// velocity range (0-127).
pub fn calculate_velocity(trigger_velocity: u8, mapping: &VelocityMapping) -> u8 {
    let result = match mapping {
        VelocityMapping::Fixed { velocity } => *velocity,

        VelocityMapping::PassThrough => trigger_velocity,

        VelocityMapping::Linear { min, max } => calculate_linear(trigger_velocity, *min, *max),

        VelocityMapping::Curve {
            curve_type,
            intensity,
        } => apply_curve(trigger_velocity, *curve_type, *intensity),
    };

    // Ensure all outputs are clamped to valid MIDI range
    result.min(127)
}

/// Calculate linear scaling from input range to output range
///
/// Maps the full input range (0-127) to a configurable output range (min-max).
fn calculate_linear(input: u8, min: u8, max: u8) -> u8 {
    // Normalize input to 0.0-1.0
    let normalized = input as f32 / 127.0;

    // Calculate output range
    let range = (max as f32) - (min as f32);

    // Linear interpolation: output = min + normalized * range
    let output = (min as f32) + (normalized * range);

    // Round and clamp to valid MIDI range
    output.round().clamp(0.0, 127.0) as u8
}

/// Apply non-linear curve transformation to velocity
///
/// Applies exponential, logarithmic, or S-curve transformations
/// to the input velocity based on the curve type and intensity.
///
/// # Curve Types
///
/// ## Exponential
/// Makes soft hits louder while preserving hard hits near maximum.
/// Formula: `output = input ^ (1 / (1 + intensity))`
/// - `intensity = 0.0`: Linear (no change)
/// - `intensity = 1.0`: Square root function (soft hits much louder)
///
/// ## Logarithmic
/// Compresses dynamic range, making soft hits quieter.
/// Formula: `output = input ^ (1 + k)` where `k = intensity * 10`
/// - `intensity = 0.0`: Linear (no change)
/// - `intensity = 1.0`: Strong compression (power of 11)
///
/// ## S-Curve
/// Creates smooth transitions with acceleration in the middle range.
/// Formula: `output = 1 / (1 + exp(-k * (input - 0.5)))` where `k = intensity * 10`
/// - `intensity = 0.0`: Linear (no change)
/// - `intensity = 1.0`: Strong S-curve
fn apply_curve(input: u8, curve_type: VelocityCurve, intensity: f32) -> u8 {
    // Normalize input to 0.0-1.0
    let normalized = input as f32 / 127.0;

    // Apply curve transformation
    let output = match curve_type {
        VelocityCurve::Exponential => {
            // Exponential: y = x^(1 - intensity)
            // Makes soft hits louder by reducing the exponent
            // intensity 0.0 → x^1 (linear), 1.0 → x^0 (constant max)
            // Using 1/(1+intensity) for better control: 0.0 → x^1, 1.0 → x^0.5
            if intensity < 0.01 {
                normalized
            } else {
                normalized.powf(1.0 / (1.0 + intensity))
            }
        }

        VelocityCurve::Logarithmic => {
            // Logarithmic: Compresses dynamic range, making soft hits quieter
            // Use exponential of normalized log scale for compression
            // where k controls compression strength (higher k = more compression)
            let k = intensity * 10.0;
            if k < 0.01 {
                // Avoid issues for very small intensity
                normalized
            } else {
                // For compression: raise to power > 1
                // This makes small values smaller (compression)
                normalized.powf(1.0 + k)
            }
        }

        VelocityCurve::SCurve => {
            // Sigmoid S-curve: y = 1 / (1 + exp(-k * (x - 0.5)))
            // Normalized to ensure 0→0 and 1→1
            // intensity controls steepness (scaled for usable range)
            let k = intensity * 10.0;
            if k < 0.01 {
                normalized
            } else {
                let sigmoid = |x: f32| 1.0 / (1.0 + (-k * (x - 0.5)).exp());
                let s0 = sigmoid(0.0);
                let s1 = sigmoid(1.0);
                // Normalize: (sigmoid(x) - sigmoid(0)) / (sigmoid(1) - sigmoid(0))
                (sigmoid(normalized) - s0) / (s1 - s0)
            }
        }
    };

    // Scale back to 0-127 and clamp
    (output * 127.0).round().clamp(0.0, 127.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_velocity() {
        let mapping = VelocityMapping::Fixed { velocity: 100 };
        assert_eq!(calculate_velocity(0, &mapping), 100);
        assert_eq!(calculate_velocity(63, &mapping), 100);
        assert_eq!(calculate_velocity(127, &mapping), 100);
    }

    #[test]
    fn test_passthrough() {
        let mapping = VelocityMapping::PassThrough;
        assert_eq!(calculate_velocity(0, &mapping), 0);
        assert_eq!(calculate_velocity(63, &mapping), 63);
        assert_eq!(calculate_velocity(127, &mapping), 127);
    }

    #[test]
    fn test_linear_scaling() {
        let mapping = VelocityMapping::Linear { min: 50, max: 100 };

        // Edge cases
        assert_eq!(calculate_velocity(0, &mapping), 50);
        assert_eq!(calculate_velocity(127, &mapping), 100);

        // Middle value: 63.5 → (50 + 100) / 2 = 75
        let mid = calculate_velocity(63, &mapping);
        assert!(mid >= 74 && mid <= 76, "Expected ~75, got {}", mid);
    }

    #[test]
    fn test_linear_full_range() {
        let mapping = VelocityMapping::Linear { min: 0, max: 127 };
        assert_eq!(calculate_velocity(0, &mapping), 0);
        assert_eq!(calculate_velocity(127, &mapping), 127);
        assert_eq!(calculate_velocity(63, &mapping), 63);
    }

    #[test]
    fn test_linear_narrow_range() {
        let mapping = VelocityMapping::Linear { min: 60, max: 70 };
        assert_eq!(calculate_velocity(0, &mapping), 60);
        assert_eq!(calculate_velocity(127, &mapping), 70);
    }

    #[test]
    fn test_exponential_curve() {
        let mapping = VelocityMapping::Curve {
            curve_type: VelocityCurve::Exponential,
            intensity: 0.5,
        };

        // Exponential curve makes soft hits louder
        let soft = calculate_velocity(30, &mapping);
        let linear_soft = (30.0 / 127.0 * 127.0) as u8; // Would be 30 with passthrough
        assert!(
            soft > linear_soft,
            "Exponential should make soft hits louder: {} vs {}",
            soft,
            linear_soft
        );

        // Hard hits stay close to max
        assert_eq!(calculate_velocity(127, &mapping), 127);
    }

    #[test]
    fn test_exponential_zero_intensity() {
        let mapping = VelocityMapping::Curve {
            curve_type: VelocityCurve::Exponential,
            intensity: 0.0,
        };

        // Zero intensity should be nearly linear
        assert_eq!(calculate_velocity(0, &mapping), 0);
        assert_eq!(calculate_velocity(127, &mapping), 127);
    }

    #[test]
    fn test_logarithmic_curve() {
        let mapping = VelocityMapping::Curve {
            curve_type: VelocityCurve::Logarithmic,
            intensity: 0.5,
        };

        // Logarithmic curve compresses dynamic range
        let soft = calculate_velocity(30, &mapping);
        let linear_soft = (30.0 / 127.0 * 127.0) as u8;
        // Soft hits should be quieter than linear
        assert!(
            soft < linear_soft,
            "Logarithmic should compress soft hits: {} vs {}",
            soft,
            linear_soft
        );

        // Max should still be max
        assert_eq!(calculate_velocity(127, &mapping), 127);
    }

    #[test]
    fn test_s_curve() {
        let mapping = VelocityMapping::Curve {
            curve_type: VelocityCurve::SCurve,
            intensity: 0.5,
        };

        // S-curve should have smooth transitions
        let low = calculate_velocity(30, &mapping);
        let mid = calculate_velocity(63, &mapping);
        let high = calculate_velocity(100, &mapping);

        // Middle should accelerate
        assert!(mid > 50 && mid < 80, "S-curve middle should accelerate");

        // Edges preserved
        assert_eq!(calculate_velocity(0, &mapping), 0);
        assert_eq!(calculate_velocity(127, &mapping), 127);
    }

    #[test]
    fn test_clamping() {
        // All curves should clamp output to 0-127
        let curves = vec![
            VelocityMapping::Fixed { velocity: 200 }, // Invalid, should clamp
            VelocityMapping::PassThrough,
            VelocityMapping::Linear { min: 0, max: 200 }, // Invalid max, should clamp
        ];

        for mapping in curves {
            let output = calculate_velocity(127, &mapping);
            assert!(
                output <= 127,
                "Output should be clamped to 127, got {}",
                output
            );
        }
    }

    #[test]
    fn test_edge_cases() {
        let mappings = vec![
            VelocityMapping::Fixed { velocity: 0 },
            VelocityMapping::PassThrough,
            VelocityMapping::Linear { min: 0, max: 127 },
            VelocityMapping::Curve {
                curve_type: VelocityCurve::Exponential,
                intensity: 0.0,
            },
        ];

        for mapping in mappings {
            // Zero input
            let output_zero = calculate_velocity(0, &mapping);
            assert!(output_zero <= 127, "Zero input should produce valid output");

            // Max input
            let output_max = calculate_velocity(127, &mapping);
            assert!(output_max <= 127, "Max input should produce valid output");
        }
    }
}
