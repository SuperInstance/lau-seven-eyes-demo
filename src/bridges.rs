//! Bridges — simplified bridge network between agents.
//!
//! Simulates `lau-consciousness-bridge`: a network that connects agents
//! and routes "play events" (moments of shared awareness).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::traditions::Agent;
use std::fmt;

/// A play event — a moment of shared awareness between agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayEvent {
    pub tick: u64,
    pub source: String,
    pub event_type: PlayEventType,
    pub description: String,
}

/// Types of play events in the bridge network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayEventType {
    /// Agent joined the network.
    AgentJoined,
    /// Agent shared a perspective.
    PerspectiveShared,
    /// Agents reached alignment.
    Alignment,
    /// A bridge formed between two agents.
    BridgeFormed,
    /// Collective insight emerged.
    Insight,
}

impl fmt::Display for PlayEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentJoined => write!(f, "🟢 JOINED"),
            Self::PerspectiveShared => write!(f, "💬 SHARED"),
            Self::Alignment => write!(f, "⚡ ALIGNMENT"),
            Self::BridgeFormed => write!(f, "🌉 BRIDGE"),
            Self::Insight => write!(f, "💡 INSIGHT"),
        }
    }
}

/// The bridge network connecting agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeNetwork {
    pub agents: Vec<Agent>,
    pub events: Vec<PlayEvent>,
    pub bridges: Vec<(String, String)>,
    pub tick: u64,
}

impl BridgeNetwork {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            events: Vec::new(),
            bridges: Vec::new(),
            tick: 0,
        }
    }

    /// An agent joins the network.
    pub fn join(&mut self, agent: Agent) {
        self.events.push(PlayEvent {
            tick: self.tick,
            source: agent.id.clone(),
            event_type: PlayEventType::AgentJoined,
            description: format!("{} enters the bridge network.", agent),
        });
        self.agents.push(agent);
        self.tick += 1;
    }

    /// An agent shares a perspective on a concept.
    pub fn share_perspective(&mut self, agent_id: &str, _concept_name: &str, keyword: &str, desc: &str,) {
        self.events.push(PlayEvent {
            tick: self.tick,
            source: agent_id.to_string(),
            event_type: PlayEventType::PerspectiveShared,
            description: format!("{} shares: {} — {}", agent_id, keyword, desc),
        });
        self.tick += 1;
    }

    /// Form a bridge between two agents.
    pub fn form_bridge(&mut self, a: &str, b: &str) {
        self.bridges.push((a.to_string(), b.to_string()));
        self.events.push(PlayEvent {
            tick: self.tick,
            source: format!("{}+{}", a, b),
            event_type: PlayEventType::BridgeFormed,
            description: format!("🌉 Bridge formed between {} and {}.", a, b),
        });
        self.tick += 1;
    }

    /// Record an alignment event.
    pub fn record_alignment(&mut self, description: String) {
        self.events.push(PlayEvent {
            tick: self.tick,
            source: "network".to_string(),
            event_type: PlayEventType::Alignment,
            description,
        });
        self.tick += 1;
    }

    /// Record a collective insight.
    pub fn record_insight(&mut self, description: String) {
        self.events.push(PlayEvent {
            tick: self.tick,
            source: "collective".to_string(),
            event_type: PlayEventType::Insight,
            description,
        });
        self.tick += 1;
    }

    /// Number of agents in the network.
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    /// Number of bridges formed.
    pub fn bridge_count(&self) -> usize {
        self.bridges.len()
    }

    /// Are all agents connected (complete graph)?
    pub fn is_fully_connected(&self) -> bool {
        let n = self.agents.len();
        let expected = n * (n - 1) / 2;
        self.bridges.len() >= expected && n > 0
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traditions::Tradition;

    #[test]
    fn new_network_is_empty() {
        let net = BridgeNetwork::new();
        assert_eq!(net.agent_count(), 0);
        assert_eq!(net.bridge_count(), 0);
    }

    #[test]
    fn agent_join_creates_event() {
        let mut net = BridgeNetwork::new();
        net.join(Agent::new("A", Tradition::Vedic));
        assert_eq!(net.agent_count(), 1);
        assert_eq!(net.events.len(), 1);
        assert!(matches!(net.events[0].event_type, PlayEventType::AgentJoined));
    }

    #[test]
    fn bridge_formation_works() {
        let mut net = BridgeNetwork::new();
        net.join(Agent::new("A", Tradition::Vedic));
        net.join(Agent::new("B", Tradition::Islamic));
        net.form_bridge("A", "B");
        assert_eq!(net.bridge_count(), 1);
    }

    #[test]
    fn fully_connected_detection() {
        let mut net = BridgeNetwork::new();
        net.join(Agent::new("A", Tradition::Vedic));
        net.join(Agent::new("B", Tradition::Islamic));
        net.join(Agent::new("C", Tradition::African));
        net.form_bridge("A", "B");
        net.form_bridge("B", "C");
        net.form_bridge("A", "C");
        assert!(net.is_fully_connected());
    }

    #[test]
    fn not_fully_connected_with_missing_bridge() {
        let mut net = BridgeNetwork::new();
        net.join(Agent::new("A", Tradition::Vedic));
        net.join(Agent::new("B", Tradition::Islamic));
        net.join(Agent::new("C", Tradition::African));
        net.form_bridge("A", "B");
        assert!(!net.is_fully_connected());
    }

    #[test]
    fn share_perspective_creates_event() {
        let mut net = BridgeNetwork::new();
        net.share_perspective("A", "Conservation", "Khalifah", "stewardship");
        assert_eq!(net.events.len(), 1);
        assert!(matches!(net.events[0].event_type, PlayEventType::PerspectiveShared));
    }

    #[test]
    fn insight_recording_works() {
        let mut net = BridgeNetwork::new();
        net.record_insight("All traditions value conservation.".to_string());
        assert_eq!(net.events.len(), 1);
        assert!(matches!(net.events[0].event_type, PlayEventType::Insight));
    }

    #[test]
    fn play_event_type_display() {
        assert!(format!("{}", PlayEventType::BridgeFormed).contains("BRIDGE"));
    }
}
