//! Training — simplified training room with curricula.
//!
//! Simulates `lau-training-room`: agents learn concepts from their tradition.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::traditions::{Agent, Concept, Tradition};

/// A training curriculum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curriculum {
    pub name: String,
    pub concepts: Vec<Concept>,
    pub required_mastery: f64,
}

impl Curriculum {
    pub fn seven_eyes_basics() -> Self {
        Self {
            name: "Seven Eyes Basics".to_string(),
            concepts: vec![Concept::Conservation, Concept::Harmony, Concept::Community],
            required_mastery: 0.8,
        }
    }
}

/// An agent's training progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingProgress {
    pub agent_id: String,
    pub tradition: Tradition,
    pub curriculum: String,
    pub concept_scores: Vec<(Concept, f64)>,
    pub graduated: bool,
}

impl TrainingProgress {
    pub fn average_mastery(&self) -> f64 {
        if self.concept_scores.is_empty() {
            return 0.0;
        }
        self.concept_scores.iter().map(|(_, s)| s).sum::<f64>() / self.concept_scores.len() as f64
    }
}

/// The training room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRoom {
    pub curriculum: Curriculum,
    pub students: Vec<TrainingProgress>,
    pub events: Vec<String>,
}

impl TrainingRoom {
    pub fn new(curriculum: Curriculum) -> Self {
        Self {
            curriculum,
            students: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Enroll an agent.
    pub fn enroll(&mut self, agent: &Agent) {
        self.events.push(format!("📚 {} enrolls in '{}'.", agent, self.curriculum.name));
        self.students.push(TrainingProgress {
            agent_id: agent.id.clone(),
            tradition: agent.tradition,
            curriculum: self.curriculum.name.clone(),
            concept_scores: Vec::new(),
            graduated: false,
        });
    }

    /// Run training — each agent learns each concept from their tradition.
    pub fn train(&mut self, agents: &[Agent]) {
        for agent in agents {
            let progress = self.students
                .iter_mut()
                .find(|s| s.agent_id == agent.id)
                .expect("agent must be enrolled");

            for &concept in &self.curriculum.concepts {
                let perspective = agent.perspective_on(concept);
                // Mastery is deterministic based on tradition + concept
                let mastery = match (agent.tradition, concept) {
                    (Tradition::Vedic, Concept::Conservation) => 0.92,
                    (Tradition::Vedic, Concept::Harmony) => 0.95,
                    (Tradition::Vedic, Concept::Community) => 0.88,
                    (Tradition::Islamic, Concept::Conservation) => 0.90,
                    (Tradition::Islamic, Concept::Harmony) => 0.87,
                    (Tradition::Islamic, Concept::Community) => 0.93,
                    (Tradition::African, Concept::Conservation) => 0.94,
                    (Tradition::African, Concept::Harmony) => 0.89,
                    (Tradition::African, Concept::Community) => 0.96,
                    _ => 0.85,
                };
                progress.concept_scores.push((concept, mastery));
                self.events.push(format!(
                    "  {} studies {} → {} (mastery: {:.0}%)",
                    agent.id, concept, perspective.keyword, mastery * 100.0
                ));
            }
        }
    }

    /// Graduate agents who meet the mastery threshold.
    pub fn graduate(&mut self) -> Vec<TrainingProgress> {
        let mut graduates = Vec::new();
        for student in &mut self.students {
            if student.average_mastery() >= self.curriculum.required_mastery {
                student.graduated = true;
                graduates.push(student.clone());
                self.events.push(format!(
                    "🎓 {} graduates from '{}' (avg mastery: {:.0}%)",
                    student.agent_id, student.curriculum, student.average_mastery() * 100.0
                ));
            } else {
                self.events.push(format!(
                    "❌ {} needs more training (avg mastery: {:.0}%)",
                    student.agent_id, student.average_mastery() * 100.0
                ));
            }
        }
        graduates
    }

    /// Number of enrolled students.
    pub fn enrollment(&self) -> usize {
        self.students.len()
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curriculum_has_concepts() {
        let c = Curriculum::seven_eyes_basics();
        assert_eq!(c.concepts.len(), 3);
        assert_eq!(c.name, "Seven Eyes Basics");
    }

    #[test]
    fn enroll_adds_student() {
        let mut room = TrainingRoom::new(Curriculum::seven_eyes_basics());
        let agent = Agent::new("Test", Tradition::Vedic);
        room.enroll(&agent);
        assert_eq!(room.enrollment(), 1);
    }

    #[test]
    fn training_produces_scores() {
        let mut room = TrainingRoom::new(Curriculum::seven_eyes_basics());
        let agent = Agent::new("Arjun", Tradition::Vedic);
        room.enroll(&agent);
        room.train(&[agent.clone()]);
        let progress = &room.students[0];
        assert_eq!(progress.concept_scores.len(), 3);
    }

    #[test]
    fn vedic_agent_graduates() {
        let mut room = TrainingRoom::new(Curriculum::seven_eyes_basics());
        let agent = Agent::new("Arjun", Tradition::Vedic);
        room.enroll(&agent);
        room.train(&[agent.clone()]);
        let grads = room.graduate();
        assert_eq!(grads.len(), 1);
        assert!(grads[0].graduated);
    }

    #[test]
    fn all_three_traditions_graduate() {
        let mut room = TrainingRoom::new(Curriculum::seven_eyes_basics());
        let agents = vec![
            Agent::new("Arjun", Tradition::Vedic),
            Agent::new("Fatima", Tradition::Islamic),
            Agent::new("Kofi", Tradition::African),
        ];
        for a in &agents {
            room.enroll(a);
        }
        room.train(&agents);
        let grads = room.graduate();
        assert_eq!(grads.len(), 3);
    }

    #[test]
    fn progress_average_mastery() {
        let progress = TrainingProgress {
            agent_id: "Test".to_string(),
            tradition: Tradition::Vedic,
            curriculum: "Test".to_string(),
            concept_scores: vec![(Concept::Conservation, 0.9), (Concept::Harmony, 0.7)],
            graduated: false,
        };
        assert!((progress.average_mastery() - 0.8).abs() < 1e-10);
    }

    #[test]
    fn empty_progress_has_zero_mastery() {
        let progress = TrainingProgress {
            agent_id: "Test".to_string(),
            tradition: Tradition::Vedic,
            curriculum: "Test".to_string(),
            concept_scores: vec![],
            graduated: false,
        };
        assert_eq!(progress.average_mastery(), 0.0);
    }
}
