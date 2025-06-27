//! Audio spectrum visualization data structures and simulation.

use std::time::Instant;

/// Represents audio frequency spectrum data for visualization
#[derive(Debug, Clone)]
pub struct AudioSpectrum {
    /// Frequency bands (typically 8-16 bands for visualization)
    pub bands: Vec<f32>,
    /// Last update timestamp
    pub last_update: Instant,
    /// Spectrum animation state
    animation_state: SpectrumAnimationState,
}

#[derive(Debug, Clone)]
struct SpectrumAnimationState {
    /// Target values for smooth animation
    targets: Vec<f32>,
    /// Decay rate for falling bars
    decay_rate: f32,
    /// Update frequency in Hz
    update_hz: f32,
    /// Random number generator seed state
    rng_state: u64,
}

impl Default for AudioSpectrum {
    fn default() -> Self {
        Self::new(12) // 12 frequency bands by default
    }
}

impl AudioSpectrum {
    /// Create a new audio spectrum with the specified number of bands
    pub fn new(num_bands: usize) -> Self {
        let mut spectrum = Self {
            bands: vec![0.0; num_bands],
            last_update: Instant::now(),
            animation_state: SpectrumAnimationState {
                targets: vec![0.0; num_bands],
                decay_rate: 0.95, // How fast bars fall
                update_hz: 30.0,  // 30 FPS updates
                rng_state: 42,    // Seed for deterministic randomness
            },
        };
        
        // Initialize with some random values to make it immediately visible
        spectrum.animation_state.rng_state = 12345;
        for i in 0..num_bands {
            spectrum.animation_state.rng_state = spectrum.animation_state.rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            let initial_value = ((spectrum.animation_state.rng_state % 500) as f32 / 1000.0) + 0.1;
            spectrum.bands[i] = initial_value;
            spectrum.animation_state.targets[i] = initial_value;
        }
        
        spectrum
    }

    /// Update the spectrum with simulated audio data
    /// This creates a realistic-looking spectrum that reacts to "music"
    pub fn update(&mut self, is_playing: bool, is_paused: bool) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        
        // Only update at target framerate
        if delta_time < 1.0 / self.animation_state.update_hz {
            return;
        }
        
        self.last_update = now;

        if is_paused {
            // When paused, just decay existing values
            for (band, target) in self.bands.iter_mut().zip(self.animation_state.targets.iter_mut()) {
                *target *= 0.8; // Faster decay when paused
                *band = (*band * 0.9).max(*target);
            }
            return;
        }

        if !is_playing {
            // When not playing, gradually reduce to zero
            for (band, target) in self.bands.iter_mut().zip(self.animation_state.targets.iter_mut()) {
                *target = 0.0;
                *band *= 0.95;
            }
            return;
        }

        // Simulate realistic frequency spectrum
        self.simulate_music_spectrum(delta_time);
    }

    /// Simulate a realistic music frequency spectrum
    fn simulate_music_spectrum(&mut self, delta_time: f32) {
        // Use a simple LCG for consistent randomness
        self.animation_state.rng_state = self.animation_state.rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        let mut rng_value = self.animation_state.rng_state;
        
        // Store previous target for influence calculation
        let mut prev_target = 0.0;

        for (i, (band, target)) in self.bands.iter_mut().zip(self.animation_state.targets.iter_mut()).enumerate() {
            // Different frequency bands have different characteristics
            let band_multiplier = match i {
                0..=1 => 0.8,   // Bass - more consistent
                2..=4 => 1.2,   // Low-mid - varies more
                5..=8 => 1.0,   // Mid - moderate variation
                _ => 0.6,       // High - more sporadic
            };

            // Generate pseudo-random value for this band
            rng_value = rng_value.wrapping_mul(1664525).wrapping_add(1013904223);
            let random_factor = (rng_value % 1000) as f32 / 1000.0;

            // Create musical patterns (some correlation between adjacent bands)
            let prev_influence = if i > 0 { prev_target * 0.3 } else { 0.0 };
            
            // Simulate musical "beats" and variation
            let beat_pattern = ((self.last_update.elapsed().as_secs_f32() * 2.0).sin() + 1.0) / 2.0;
            let variation = random_factor * band_multiplier + prev_influence + beat_pattern * 0.2;
            
            // Set new target with some smoothing
            *target = (*target * 0.7 + variation.clamp(0.0, 1.0) * 0.3).min(1.0);
            
            // Store current target for next iteration
            prev_target = *target;
            
            // Animate towards target with decay
            if *band < *target {
                // Rise quickly
                *band = (*band + (*target - *band) * 8.0 * delta_time).min(*target);
            } else {
                // Fall with decay
                *band *= self.animation_state.decay_rate.powf(delta_time * 60.0);
            }
            
            *band = band.clamp(0.0, 1.0);
        }
    }

    /// Get the current spectrum bands for rendering
    pub fn get_bands(&self) -> &[f32] {
        &self.bands
    }

    /// Get the number of frequency bands
    pub fn band_count(&self) -> usize {
        self.bands.len()
    }

    /// Set the decay rate (how fast bars fall)
    pub fn set_decay_rate(&mut self, decay_rate: f32) {
        self.animation_state.decay_rate = decay_rate.clamp(0.1, 0.99);
    }

    /// Set the update frequency
    pub fn set_update_hz(&mut self, hz: f32) {
        self.animation_state.update_hz = hz.clamp(10.0, 60.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_creation() {
        let spectrum = AudioSpectrum::new(8);
        assert_eq!(spectrum.band_count(), 8);
        assert_eq!(spectrum.get_bands().len(), 8);
    }

    #[test]
    fn test_spectrum_update_when_not_playing() {
        let mut spectrum = AudioSpectrum::new(4);
        
        // Set some initial values
        let initial_values = vec![0.5, 0.7, 0.3, 0.9];
        spectrum.bands = initial_values.clone();
        
        // Update when not playing
        spectrum.update(false, false);
        
        // Values should decay - each should be less than or equal to initial value
        for (i, &band) in spectrum.get_bands().iter().enumerate() {
            assert!(band <= initial_values[i], "Band {} should decay from {} but was {}", i, initial_values[i], band);
        }
    }

    #[test]
    fn test_spectrum_update_when_paused() {
        let mut spectrum = AudioSpectrum::new(4);
        
        // Set some initial values
        spectrum.bands = vec![0.8, 0.6, 0.4, 0.2];
        
        // Update when paused
        spectrum.update(true, true);
        
        // Values should still be positive but decaying
        for &band in spectrum.get_bands() {
            assert!(band >= 0.0);
            assert!(band <= 0.8);
        }
    }

    #[test]
    fn test_spectrum_bounds() {
        let mut spectrum = AudioSpectrum::new(6);
        
        // Force update multiple times
        for _ in 0..100 {
            spectrum.update(true, false);
            
            // All values should be within bounds
            for &band in spectrum.get_bands() {
                assert!(band >= 0.0 && band <= 1.0, "Band value {} out of bounds", band);
            }
        }
    }

    #[test]
    fn test_configuration_methods() {
        let mut spectrum = AudioSpectrum::new(4);
        
        spectrum.set_decay_rate(0.8);
        spectrum.set_update_hz(25.0);
        
        // Should accept valid values
        assert!(spectrum.animation_state.decay_rate >= 0.1);
        assert!(spectrum.animation_state.update_hz >= 10.0);
        
        // Test boundary conditions
        spectrum.set_decay_rate(2.0); // Should be clamped
        spectrum.set_update_hz(5.0);  // Should be clamped
        
        assert!(spectrum.animation_state.decay_rate <= 0.99);
        assert!(spectrum.animation_state.update_hz >= 10.0);
    }
}
