//! Recovery — simplified kintsugi, breaks and golden repairs.
//!
//! Simulates `lau-kintsugi`: when things break, we repair them with gold.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Something that broke during the process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Break {
    pub description: String,
    pub severity: f64, // 0.0–1.0
    pub agents_involved: Vec<String>,
}

/// A golden repair — how the break was healed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenRepair {
    pub break_description: String,
    pub repair_description: String,
    pub gold_value: f64,     // how much wisdom was gained
    pub repaired_by: Vec<String>,
}

impl fmt::Display for GoldenRepair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "💛 Repair: {} → {} (gold: {:.0}%)",
            self.break_description, self.repair_description, self.gold_value * 100.0)
    }
}

/// The kintsugi tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kintsugi {
    pub breaks: Vec<Break>,
    pub repairs: Vec<GoldenRepair>,
    pub events: Vec<String>,
}

impl Kintsugi {
    pub fn new() -> Self {
        Self {
            breaks: Vec::new(),
            repairs: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Record a break.
    pub fn record_break(&mut self, description: impl Into<String>, severity: f64, agents: Vec<String>) {
        let desc = description.into();
        self.events.push(format!("💔 BREAK: {} (severity: {:.0}%)", desc, severity * 100.0));
        self.breaks.push(Break {
            description: desc,
            severity,
            agents_involved: agents,
        });
    }

    /// Repair a break with gold.
    pub fn repair(&mut self, break_index: usize, repair_description: impl Into<String>, gold_value: f64, repaired_by: Vec<String>) {
        let break_desc = self.breaks[break_index].description.clone();
        let repair_desc = repair_description.into();
        self.events.push(format!(
            "💛 REPAIR: {} → {} (gold value: {:.0}%)",
            break_desc, repair_desc, gold_value * 100.0
        ));
        self.repairs.push(GoldenRepair {
            break_description: break_desc,
            repair_description: repair_desc,
            gold_value,
            repaired_by,
        });
    }

    /// All breaks repaired?
    pub fn all_repaired(&self) -> bool {
        self.breaks.len() == self.repairs.len()
    }

    /// Total gold value (wisdom gained).
    pub fn total_gold(&self) -> f64 {
        self.repairs.iter().map(|r| r.gold_value).sum()
    }

    /// The break is now beautiful — kintsugi philosophy.
    pub fn is_more_beautiful_than_before(&self) -> bool {
        self.total_gold() > 0.0 && self.all_repaired()
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_kintsugi_is_empty() {
        let k = Kintsugi::new();
        assert!(k.breaks.is_empty());
        assert!(k.repairs.is_empty());
    }

    #[test]
    fn record_break_works() {
        let mut k = Kintsugi::new();
        k.record_break("Miscommunication", 0.5, vec!["A".to_string()]);
        assert_eq!(k.breaks.len(), 1);
        assert!(!k.all_repaired());
    }

    #[test]
    fn repair_resolves_break() {
        let mut k = Kintsugi::new();
        k.record_break("Disagreement on approach", 0.7, vec!["Arjun".to_string(), "Fatima".to_string()]);
        k.repair(0, "Found common ground through shared values", 0.9, vec!["Arjun".to_string(), "Fatima".to_string()]);
        assert!(k.all_repaired());
        assert_eq!(k.repairs.len(), 1);
    }

    #[test]
    fn total_gold_accumulates() {
        let mut k = Kintsugi::new();
        k.record_break("A", 0.3, vec![]);
        k.record_break("B", 0.5, vec![]);
        k.repair(0, "fix A", 0.8, vec![]);
        k.repair(1, "fix B", 0.6, vec![]);
        assert!((k.total_gold() - 1.4).abs() < 1e-10);
    }

    #[test]
    fn kintsugi_beauty() {
        let mut k = Kintsugi::new();
        k.record_break("x", 0.1, vec![]);
        k.repair(0, "golden fix", 1.0, vec![]);
        assert!(k.is_more_beautiful_than_before());
    }

    #[test]
    fn unrepaired_is_not_beautiful() {
        let mut k = Kintsugi::new();
        k.record_break("x", 0.5, vec![]);
        assert!(!k.is_more_beautiful_than_before());
    }

    #[test]
    fn golden_repair_display() {
        let repair = GoldenRepair {
            break_description: "crack".to_string(),
            repair_description: "gold".to_string(),
            gold_value: 0.75,
            repaired_by: vec![],
        };
        let s = format!("{}", repair);
        assert!(s.contains("gold: 75%"));
    }

    #[test]
    fn events_recorded() {
        let mut k = Kintsugi::new();
        k.record_break("test", 0.5, vec![]);
        k.repair(0, "fix", 0.8, vec![]);
        assert_eq!(k.events.len(), 2);
    }
}
