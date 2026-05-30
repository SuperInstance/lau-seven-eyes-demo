//! Rhythm — polyrhythm engine for tracking conversation cadence.
//!
//! Simulates `lau-tensor-midi` + `lau-rhythm-nation`: multiple traditions
//! running simultaneous rhythmic patterns that interweave.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::traditions::Tradition;

/// A beat within a rhythmic cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beat {
    pub tick: u64,
    pub intensity: f64,       // 0.0–1.0
    pub tradition: Tradition,
    pub label: String,
}

/// A rhythmic cycle for one tradition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmicCycle {
    pub tradition: Tradition,
    pub beats_per_cycle: usize,
    pub pattern: Vec<f64>,  // intensity per beat position
    pub current_position: usize,
}

impl RhythmicCycle {
    pub fn new(tradition: Tradition, pattern: Vec<f64>) -> Self {
        let beats_per_cycle = pattern.len();
        Self {
            tradition,
            beats_per_cycle,
            pattern,
            current_position: 0,
        }
    }

    /// Advance one tick and return the beat (if any — always returns, intensity may be low).
    pub fn step(&mut self, tick: u64) -> Beat {
        let intensity = self.pattern[self.current_position];
        let beat = Beat {
            tick,
            intensity,
            tradition: self.tradition,
            label: format!("{} beat {}/{}", self.tradition.name(), self.current_position + 1, self.beats_per_cycle),
        };
        self.current_position = (self.current_position + 1) % self.beats_per_cycle;
        beat
    }

    /// Total energy of this cycle's pattern.
    pub fn energy(&self) -> f64 {
        self.pattern.iter().sum()
    }
}

/// The polyrhythm engine — multiple cycles running simultaneously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolyrhythmEngine {
    pub cycles: Vec<RhythmicCycle>,
    pub tick: u64,
    pub history: Vec<Beat>,
}

impl PolyrhythmEngine {
    pub fn new(cycles: Vec<RhythmicCycle>) -> Self {
        Self {
            cycles,
            tick: 0,
            history: Vec::new(),
        }
    }

    /// Create a 3-tradition engine with characteristic patterns.
    pub fn three_way() -> Self {
        let vedic = RhythmicCycle::new(
            Tradition::Vedic,
            vec![1.0, 0.0, 0.5, 0.0, 0.8, 0.0, 0.3, 0.0], // 4-beat pattern in 8
        );
        let islamic = RhythmicCycle::new(
            Tradition::Islamic,
            vec![1.0, 0.0, 0.0, 0.7, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5], // 10-beet usul
        );
        let african = RhythmicCycle::new(
            Tradition::African,
            vec![1.0, 0.3, 0.7, 0.0, 1.0, 0.5, 0.3, 0.8, 0.0, 0.6, 0.4, 0.9], // 12-beat bell pattern
        );
        Self::new(vec![vedic, islamic, african])
    }

    /// Advance all cycles by one tick. Returns beats from all traditions.
    pub fn step(&mut self) -> Vec<Beat> {
        let beats: Vec<Beat> = self.cycles.iter_mut().map(|c| c.step(self.tick)).collect();
        self.history.extend(beats.iter().cloned());
        self.tick += 1;
        beats
    }

    /// Run N ticks and return all beats.
    pub fn run(&mut self, ticks: usize) -> Vec<Vec<Beat>> {
        (0..ticks).map(|_| self.step()).collect()
    }

    /// Combined intensity at the current moment.
    pub fn combined_intensity(&self) -> f64 {
        self.cycles.iter().map(|c| c.pattern[c.current_position]).sum::<f64>() / self.cycles.len() as f64
    }

    /// How many beats have been played.
    pub fn total_beats(&self) -> usize {
        self.history.len()
    }

    /// Check if cycles are in alignment (all at position 0).
    pub fn is_aligned(&self) -> bool {
        self.cycles.iter().all(|c| c.current_position == 0)
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_steps_through_pattern() {
        let mut cycle = RhythmicCycle::new(Tradition::Vedic, vec![1.0, 0.5, 0.0]);
        let b0 = cycle.step(0);
        assert_eq!(b0.intensity, 1.0);
        let b1 = cycle.step(1);
        assert_eq!(b1.intensity, 0.5);
        let b2 = cycle.step(2);
        assert_eq!(b2.intensity, 0.0);
        // wraps
        let b3 = cycle.step(3);
        assert_eq!(b3.intensity, 1.0);
    }

    #[test]
    fn cycle_energy_sums_pattern() {
        let cycle = RhythmicCycle::new(Tradition::African, vec![1.0, 0.5, 0.3]);
        assert!((cycle.energy() - 1.8).abs() < 1e-10);
    }

    #[test]
    fn three_way_engine_has_three_cycles() {
        let engine = PolyrhythmEngine::three_way();
        assert_eq!(engine.cycles.len(), 3);
    }

    #[test]
    fn engine_step_produces_one_beat_per_cycle() {
        let mut engine = PolyrhythmEngine::three_way();
        let beats = engine.step();
        assert_eq!(beats.len(), 3);
    }

    #[test]
    fn engine_run_accumulates_history() {
        let mut engine = PolyrhythmEngine::three_way();
        engine.run(10);
        assert_eq!(engine.total_beats(), 30); // 3 cycles × 10 ticks
    }

    #[test]
    fn combined_intensity_in_range() {
        let mut engine = PolyrhythmEngine::three_way();
        for _ in 0..100 {
            let intensity = engine.combined_intensity();
            assert!(intensity >= 0.0 && intensity <= 1.0, "intensity: {}", intensity);
            engine.step();
        }
    }

    #[test]
    fn alignment_detected_at_tick_zero() {
        let engine = PolyrhythmEngine::three_way();
        assert!(engine.is_aligned());
    }

    #[test]
    fn serde_roundtrip_engine() {
        let engine = PolyrhythmEngine::three_way();
        let json = serde_json::to_string(&engine).unwrap();
        let engine2: PolyrhythmEngine = serde_json::from_str(&json).unwrap();
        assert_eq!(engine2.cycles.len(), 3);
        assert_eq!(engine2.tick, 0);
    }
}
