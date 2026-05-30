//! Traditions — the seven knowledge traditions, concept translation, and identity.
//!
//! Simulates `lau-polyglot-tradition`: cross-cultural concept translation.
//! Seven traditions, each with perspectives on five universal concepts.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// The seven knowledge traditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tradition {
    Vedic,
    Islamic,
    African,
    Indigenous,
    EastAsian,
    Western,
    Pacific,
}

impl Tradition {
    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Vedic => "Vedic",
            Self::Islamic => "Islamic",
            Self::African => "African",
            Self::Indigenous => "Indigenous",
            Self::EastAsian => "East Asian",
            Self::Western => "Western",
            Self::Pacific => "Pacific",
        }
    }

    /// An emoji symbol for this tradition.
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Vedic => "🕉️",
            Self::Islamic => "☪️",
            Self::African => "🌍",
            Self::Indigenous => "🌿",
            Self::EastAsian => "☯️",
            Self::Western => "🔭",
            Self::Pacific => "🌊",
        }
    }

    /// All seven traditions.
    pub fn all() -> &'static [Tradition] {
        &[
            Self::Vedic,
            Self::Islamic,
            Self::African,
            Self::Indigenous,
            Self::EastAsian,
            Self::Western,
            Self::Pacific,
        ]
    }

    /// Number of traditions.
    pub fn count() -> usize {
        7
    }
}

impl fmt::Display for Tradition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.name())
    }
}

/// Five universal concepts that every tradition has a perspective on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Concept {
    Conservation,
    Harmony,
    Community,
    Knowledge,
    Renewal,
}

impl Concept {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Conservation => "Conservation",
            Self::Harmony => "Harmony",
            Self::Community => "Community",
            Self::Knowledge => "Knowledge",
            Self::Renewal => "Renewal",
        }
    }

    pub fn all() -> &'static [Concept] {
        &[Self::Conservation, Self::Harmony, Self::Community, Self::Knowledge, Self::Renewal]
    }
}

impl fmt::Display for Concept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// A tradition's perspective on a concept — how it translates that idea.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perspective {
    pub tradition: Tradition,
    pub concept: Concept,
    /// Short keyword for this perspective.
    pub keyword: String,
    /// One-sentence description.
    pub description: String,
}

/// Translate a concept through a tradition's lens.
pub fn translate(tradition: Tradition, concept: Concept) -> Perspective {
    let (keyword, description) = match (tradition, concept) {
        // Conservation
        (Tradition::Vedic, Concept::Conservation) =>
            ("Prakṛti Rakṣaṇa", "Protecting nature as the body of the divine mother."),
        (Tradition::Islamic, Concept::Conservation) =>
            ("Khalifah", "Stewardship of the Earth as a trust from Allah."),
        (Tradition::African, Concept::Conservation) =>
            ("Ubuntu Earth", "We are the land; conserving it is conserving ourselves."),
        (Tradition::Indigenous, Concept::Conservation) =>
            ("Seventh Generation", "Every decision must serve those seven generations ahead."),
        (Tradition::EastAsian, Concept::Conservation) =>
            ("Wú Wéi", "Conserving by not interfering — letting nature find its balance."),
        (Tradition::Western, Concept::Conservation) =>
            ("Ecosystem Services", "Nature's systems provide measurable value we must maintain."),
        (Tradition::Pacific, Concept::Conservation) =>
            ("Moana Kaitiaki", "The ocean guards us; we must guard the ocean."),

        // Harmony
        (Tradition::Vedic, Concept::Harmony) =>
            ("Ṛta", "The cosmic order that holds all things in balance."),
        (Tradition::Islamic, Concept::Harmony) =>
            ("Mizan", "The balance God placed in all creation."),
        (Tradition::African, Concept::Harmony) =>
            ("Ngoma", "The drum that keeps the community in rhythm together."),
        (Tradition::Indigenous, Concept::Harmony) =>
            ("Walking in Beauty", "Living in right relationship with all beings."),
        (Tradition::EastAsian, Concept::Harmony) =>
            ("Hé", "Harmony without uniformity — difference enriching the whole."),
        (Tradition::Western, Concept::Harmony) =>
            ("Homeostasis", "Systems self-regulate to maintain stable equilibrium."),
        (Tradition::Pacific, Concept::Harmony) =>
            ("Faka-Tokatanga", "Navigating together through the currents of life."),

        // Community
        (Tradition::Vedic, Concept::Community) =>
            ("Sangha", "The community of seekers walking the path together."),
        (Tradition::Islamic, Concept::Community) =>
            ("Ummah", "The global community bound by mutual responsibility."),
        (Tradition::African, Concept::Community) =>
            ("Ubuntu", "I am because we are; personhood through relationship."),
        (Tradition::Indigenous, Concept::Community) =>
            ("Kinship Web", "All beings are relatives; the web holds us together."),
        (Tradition::EastAsian, Concept::Community) =>
            ("Guanxi", "The network of relationships that sustains society."),
        (Tradition::Western, Concept::Community) =>
            ("Social Contract", "Rights and duties that bind citizens together."),
        (Tradition::Pacific, Concept::Community) =>
            ("Fa'a Samoa", "The way of the family — extended kinship as foundation."),

        // Knowledge
        (Tradition::Vedic, Concept::Knowledge) =>
            ("Jñāna", "Direct knowing — wisdom that arises from contemplation."),
        (Tradition::Islamic, Concept::Knowledge) =>
            ("ʿIlm", "The pursuit of knowledge as a sacred obligation."),
        (Tradition::African, Concept::Knowledge) =>
            ("Griot Memory", "Knowledge lives in the storyteller's voice, not on shelves."),
        (Tradition::Indigenous, Concept::Knowledge) =>
            ("Story Law", "Knowledge encoded in stories passed down through country."),
        (Tradition::EastAsian, Concept::Knowledge) =>
            ("Zhī", "Practical wisdom — knowing when and how to act."),
        (Tradition::Western, Concept::Knowledge) =>
            ("Episteme", "Structured, justified knowledge built through method."),
        (Tradition::Pacific, Concept::Knowledge) =>
            ("Wayfinding", "Knowing by reading stars, waves, winds, and birds."),

        // Renewal
        (Tradition::Vedic, Concept::Renewal) =>
            ("Saṃsāra", "The wheel of rebirth — every ending is a beginning."),
        (Tradition::Islamic, Concept::Renewal) =>
            ("Tajdīd", "Renewal of faith and practice in every generation."),
        (Tradition::African, Concept::Renewal) =>
            ("Sankofa", "Go back and fetch it — learn from the past to renew the future."),
        (Tradition::Indigenous, Concept::Renewal) =>
            ("Fire Stick Farming", "Burning the old to make way for new growth."),
        (Tradition::EastAsian, Concept::Renewal) =>
            ("I Ching", "The only constant is change; each change brings renewal."),
        (Tradition::Western, Concept::Renewal) =>
            ("Creative Destruction", "Old structures must break for new ones to emerge."),
        (Tradition::Pacific, Concept::Renewal) =>
            ("Tangaroa's Tide", "The sea recedes and returns — renewal is nature's rhythm."),
    };

    Perspective {
        tradition,
        concept,
        keyword: keyword.to_string(),
        description: description.to_string(),
    }
}

/// An agent that carries a tradition's identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub tradition: Tradition,
    pub perspectives: Vec<Perspective>,
}

impl Agent {
    pub fn new(id: impl Into<String>, tradition: Tradition) -> Self {
        let perspectives = Concept::all()
            .iter()
            .map(|&c| translate(tradition, c))
            .collect();
        Self {
            id: id.into(),
            tradition,
            perspectives,
        }
    }

    /// This agent's perspective on a specific concept.
    pub fn perspective_on(&self, concept: Concept) -> &Perspective {
        self.perspectives
            .iter()
            .find(|p| p.concept == concept)
            .expect("every agent has a perspective on every concept")
    }

    pub fn name(&self) -> &str {
        &self.id
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.id, self.tradition)
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seven_traditions_exist() {
        assert_eq!(Tradition::all().len(), 7);
    }

    #[test]
    fn five_concepts_exist() {
        assert_eq!(Concept::all().len(), 5);
    }

    #[test]
    fn translation_yields_correct_tradition_and_concept() {
        let p = translate(Tradition::African, Concept::Community);
        assert_eq!(p.tradition, Tradition::African);
        assert_eq!(p.concept, Concept::Community);
        assert_eq!(p.keyword, "Ubuntu");
    }

    #[test]
    fn all_traditions_translate_all_concepts() {
        for &tradition in Tradition::all() {
            for &concept in Concept::all() {
                let p = translate(tradition, concept);
                assert_eq!(p.tradition, tradition);
                assert_eq!(p.concept, concept);
                assert!(!p.keyword.is_empty());
                assert!(!p.description.is_empty());
            }
        }
    }

    #[test]
    fn agent_has_five_perspectives() {
        let agent = Agent::new("TestAgent", Tradition::Vedic);
        assert_eq!(agent.perspectives.len(), 5);
    }

    #[test]
    fn agent_perspective_lookup_works() {
        let agent = Agent::new("Kofi", Tradition::African);
        let p = agent.perspective_on(Concept::Conservation);
        assert_eq!(p.keyword, "Ubuntu Earth");
    }

    #[test]
    fn tradition_display_includes_emoji() {
        let s = format!("{}", Tradition::Islamic);
        assert!(s.contains("Islamic"));
        assert!(s.contains("☪️"));
    }

    #[test]
    fn traditions_are_distinct() {
        let all: std::collections::HashSet<_> = Tradition::all().iter().copied().collect();
        assert_eq!(all.len(), 7);
    }

    #[test]
    fn serde_roundtrip_tradition() {
        let t = Tradition::EastAsian;
        let json = serde_json::to_string(&t).unwrap();
        let t2: Tradition = serde_json::from_str(&json).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn serde_roundtrip_agent() {
        let agent = Agent::new("Amara", Tradition::African);
        let json = serde_json::to_string(&agent).unwrap();
        let agent2: Agent = serde_json::from_str(&json).unwrap();
        assert_eq!(agent2.id, "Amara");
        assert_eq!(agent2.tradition, Tradition::African);
    }
}
