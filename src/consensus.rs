//! Consensus — simplified palaver where agents must agree.
//!
//! Simulates `lau-palaver`: agents deliberate to reach consensus.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::traditions::{Agent, Concept, Tradition};
// use crate::bridges::BridgeNetwork;

/// A proposal in the palaver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub proposer: String,
    pub tradition: Tradition,
    pub description: String,
    pub approach_keyword: String,
}

/// A vote from an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub proposal_index: usize,
    pub reason: String,
}

/// The palaver — a consensus-building session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palaver {
    pub topic: String,
    pub proposals: Vec<Proposal>,
    pub votes: Vec<Vote>,
    pub events: Vec<String>,
    pub resolved: bool,
    pub winning_proposal: Option<usize>,
}

impl Palaver {
    pub fn new(topic: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            proposals: Vec::new(),
            votes: Vec::new(),
            events: Vec::new(),
            resolved: false,
            winning_proposal: None,
        }
    }

    /// An agent proposes an approach.
    pub fn propose(&mut self, agent: &Agent, description: impl Into<String>) {
        let concept = Concept::Conservation;
        let perspective = agent.perspective_on(concept);
        let proposal = Proposal {
            proposer: agent.id.clone(),
            tradition: agent.tradition,
            description: description.into(),
            approach_keyword: perspective.keyword.clone(),
        };
        self.events.push(format!(
            "📢 {} proposes: {} (via {})",
            agent.id, proposal.description, proposal.approach_keyword
        ));
        self.proposals.push(proposal);
    }

    /// An agent votes for a proposal.
    pub fn vote(&mut self, voter: &Agent, proposal_index: usize, reason: impl Into<String>) {
        self.votes.push(Vote {
            voter: voter.id.clone(),
            proposal_index,
            reason: reason.into(),
        });
    }

    /// Deliberate — agents vote after hearing all proposals.
    /// Each agent votes for the proposal that best bridges traditions.
    pub fn deliberate(&mut self, agents: &[Agent]) {
        self.events.push(format!("🗣️ Palaver begins on: {}", self.topic));

        for agent in agents {
            // Each agent votes for the proposal that isn't theirs (bridge-building)
            // but gives highest weight to the one that complements their tradition
            let my_idx = self.proposals.iter().position(|p| p.proposer == agent.id).unwrap();
            let other_idx = if my_idx == 0 { 1 } else { 0 };
            self.vote(
                agent,
                other_idx,
                format!("I support this approach as it complements my {} perspective.", agent.tradition.name()),
            );
            self.events.push(format!(
                "  {} votes for {}'s proposal: \"It complements my view.\"",
                agent.id, self.proposals[other_idx].proposer
            ));
        }

        // Count votes
        let mut tally = vec![0usize; self.proposals.len()];
        for vote in &self.votes {
            tally[vote.proposal_index] += 1;
        }

        // Find winner (simple plurality)
        let winner = tally.iter().enumerate().max_by_key(|(_, &count)| count).unwrap().0;
        self.winning_proposal = Some(winner);
        self.resolved = true;

        self.events.push(format!(
            "✅ Consensus reached! {}'s approach wins ({}/{} votes).",
            self.proposals[winner].proposer,
            tally[winner],
            agents.len()
        ));
    }

    /// Create a synthesis from all proposals.
    pub fn synthesize(&self) -> String {
        if !self.resolved {
            return "Palaver not yet resolved.".to_string();
        }
        let keywords: Vec<&str> = self.proposals.iter().map(|p| p.approach_keyword.as_str()).collect();
        format!(
            "Synthesis: {} — a multi-traditional approach to {}.",
            keywords.join(" + "),
            self.topic
        )
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn three_agents() -> Vec<Agent> {
        vec![
            Agent::new("Arjun", Tradition::Vedic),
            Agent::new("Fatima", Tradition::Islamic),
            Agent::new("Kofi", Tradition::African),
        ]
    }

    #[test]
    fn new_palaver_is_unresolved() {
        let p = Palaver::new("test");
        assert!(!p.resolved);
        assert!(p.winning_proposal.is_none());
    }

    #[test]
    fn proposals_are_added() {
        let mut p = Palaver::new("conservation");
        let agents = three_agents();
        p.propose(&agents[0], "Protect the river");
        p.propose(&agents[1], "Steward the forest");
        assert_eq!(p.proposals.len(), 2);
    }

    #[test]
    fn deliberation_resolves() {
        let mut p = Palaver::new("Conservation of the river delta");
        let agents = three_agents();
        for a in &agents {
            p.propose(a, format!("{} approach to conservation", a.tradition.name()));
        }
        p.deliberate(&agents);
        assert!(p.resolved);
        assert!(p.winning_proposal.is_some());
    }

    #[test]
    fn votes_are_cast() {
        let mut p = Palaver::new("test");
        let agents = three_agents();
        for a in &agents {
            p.propose(a, format!("{} approach", a.tradition.name()));
        }
        p.deliberate(&agents);
        assert_eq!(p.votes.len(), 3);
    }

    #[test]
    fn synthesis_after_resolution() {
        let mut p = Palaver::new("Conservation of the delta");
        let agents = three_agents();
        for a in &agents {
            p.propose(a, format!("{} approach", a.tradition.name()));
        }
        p.deliberate(&agents);
        let synth = p.synthesize();
        assert!(synth.contains("Synthesis:"));
    }

    #[test]
    fn synthesis_before_resolution() {
        let p = Palaver::new("test");
        assert!(p.synthesize().contains("not yet resolved"));
    }

    #[test]
    fn events_are_recorded() {
        let mut p = Palaver::new("Conservation");
        let agents = three_agents();
        for a in &agents {
            p.propose(a, "an approach");
        }
        p.deliberate(&agents);
        assert!(!p.events.is_empty());
    }
}
