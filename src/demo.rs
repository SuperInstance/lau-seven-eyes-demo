//! Run the demo: `cargo test demo_full_story -- --nocapture`

#![allow(dead_code)]

use crate::traditions::{Agent, Concept, Tradition};
use crate::rhythm::PolyrhythmEngine;
use crate::bridges::BridgeNetwork;
use crate::training::{Curriculum, TrainingRoom};
use crate::consensus::Palaver;
use crate::recovery::Kintsugi;

/// The full demo output — a story with chapters.
#[derive(Debug)]
pub struct DemoStory {
    pub chapters: Vec<Chapter>,
}

/// One chapter of the story.
#[derive(Debug)]
pub struct Chapter {
    pub title: String,
    pub lines: Vec<String>,
}

impl DemoStory {
    pub fn new() -> Self {
        Self { chapters: Vec::new() }
    }

    pub fn chapter(&mut self, title: impl Into<String>) -> &mut Chapter {
        self.chapters.push(Chapter { title: title.into(), lines: Vec::new() });
        self.chapters.last_mut().unwrap()
    }

    pub fn add(&mut self, line: impl Into<String>) {
        if let Some(ch) = self.chapters.last_mut() {
            ch.lines.push(line.into());
        }
    }

    /// Render the full story as a string.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for (i, ch) in self.chapters.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            out.push_str("═══════════════════════════════════════\n");
            out.push_str(&format!("  {}\n", ch.title.to_uppercase()));
            out.push_str("═══════════════════════════════════════\n\n");
            for line in &ch.lines {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}

/// Run the full demo and return the story.
pub fn run_demo() -> DemoStory {
    let mut story = DemoStory::new();

    // ── Prologue ──
    {
        story.chapter("Prologue: Three Paths, One Problem");
        story.add("In a world where rivers run dry and forests shrink, three scholars");
        story.add("from three traditions gather. They have never met. They speak");
        story.add("different languages of wisdom. But they share one concern:");
        story.add("");
        story.add("The river delta is dying.");
    }

    // ── Chapter 1: Enter the Agents ──
    let agents = vec![
        Agent::new("Arjun", Tradition::Vedic),
        Agent::new("Fatima", Tradition::Islamic),
        Agent::new("Kofi", Tradition::African),
    ];

    {
        story.chapter("Chapter 1: Three Scholars Arrive");
        for agent in &agents {
            let p = agent.perspective_on(Concept::Conservation);
            story.add(format!("{} enters the room.", agent));
            story.add(format!("  Their way: {} — {}", p.keyword, p.description));
            story.add("");
        }
    }

    // ── Chapter 2: Training ──
    let mut training_room = TrainingRoom::new(Curriculum::seven_eyes_basics());
    for agent in &agents {
        training_room.enroll(agent);
    }
    training_room.train(&agents);
    let graduates = training_room.graduate();

    {
        story.chapter("Chapter 2: The Training Room");
        story.add("Before they can work together, each scholar must deepen");
        story.add("their understanding of the core concepts.\n");
        for event in &training_room.events {
            story.add(event.clone());
        }
        story.add("");
        story.add(format!("All {} scholars graduate. They are ready.", graduates.len()));
    }

    // ── Chapter 3: Bridge Network ──
    let mut network = BridgeNetwork::new();
    for agent in &agents {
        network.join(agent.clone());
    }

    // Share perspectives
    for agent in &agents {
        let p = agent.perspective_on(Concept::Conservation);
        network.share_perspective(&agent.id, "Conservation", &p.keyword, &p.description);
    }

    // Form bridges
    network.form_bridge("Arjun", "Fatima");
    network.form_bridge("Fatima", "Kofi");
    network.form_bridge("Arjun", "Kofi");
    network.record_alignment("All three traditions align on conservation as a core value.".to_string());

    {
        story.chapter("Chapter 3: Bridges Form");
        story.add("The scholars enter the bridge network — a space where");
        story.add("different ways of knowing can connect.\n");
        for event in &network.events {
            story.add(event.description.clone());
        }
        story.add("");
        story.add(format!("Bridges: {} | Agents: {} | Connected: {}",
            network.bridge_count(),
            network.agent_count(),
            if network.is_fully_connected() { "YES" } else { "NO" }
        ));
    }

    // ── Chapter 4: The Polyrhythm of Conversation ──
    let mut engine = PolyrhythmEngine::three_way();
    let beats = engine.run(24);

    {
        story.chapter("Chapter 4: The Rhythm of Three Voices");
        story.add("As the scholars speak, each brings their own rhythm —");
        story.add("their own cadence of thought and expression.\n");
        // Show a few key moments
        let key_ticks: Vec<u64> = vec![0, 7, 11, 23];
        for tick in key_ticks {
            if (tick as usize) < beats.len() {
                story.add(format!("Tick {:2}:", tick));
                for beat in &beats[tick as usize] {
                    if beat.intensity > 0.3 {
                        story.add(format!("  {} {:.0}% — {}",
                            beat.tradition.emoji(), beat.intensity * 100.0, beat.label));
                    }
                }
            }
        }
        story.add("");
        story.add(format!("Total beats played: {}", engine.total_beats()));
        story.add("The rhythms interweave — never identical, always complementary.");
    }

    // ── Chapter 5: The Palaver ──
    let mut palaver = Palaver::new("How to save the river delta");
    palaver.propose(&agents[0], "Protect the delta as sacred — Prakṛti Rakṣaṇa, nature as divine mother");
    palaver.propose(&agents[1], "Steward the delta as Khalifah — a trust from God to humanity");
    palaver.propose(&agents[2], "The delta is us — Ubuntu Earth, conserving it is conserving ourselves");
    palaver.deliberate(&agents);

    {
        story.chapter("Chapter 5: The Palaver");
        story.add("The scholars sit in council. Each proposes their approach.\n");
        for event in &palaver.events {
            story.add(event.clone());
        }
        story.add("");
        story.add(format!("Synthesis: {}", palaver.synthesize()));
    }

    // ── Chapter 6: Break and Repair (Kintsugi) ──
    let mut kintsugi = Kintsugi::new();
    kintsugi.record_break(
        "Arjun and Fatima disagree: sacred vs. stewardship",
        0.6,
        vec!["Arjun".to_string(), "Fatima".to_string()],
    );
    kintsugi.repair(
        0,
        "Sacred stewardship — nature is both divine AND entrusted to us",
        0.95,
        vec!["Arjun".to_string(), "Fatima".to_string(), "Kofi".to_string()],
    );
    kintsugi.record_break(
        "Kofi's oral tradition clashes with the others' written approaches",
        0.4,
        vec!["Kofi".to_string()],
    );
    kintsugi.repair(
        1,
        "Griot circles become the living memory of the conservation plan",
        0.88,
        vec!["Kofi".to_string(), "Arjun".to_string(), "Fatima".to_string()],
    );

    {
        story.chapter("Chapter 6: Breaks and Golden Repairs");
        story.add("The path to consensus was not smooth. Things broke.");
        story.add("But in the kintsugi way, every break was repaired with gold.\n");
        for event in &kintsugi.events {
            story.add(event.clone());
        }
        story.add("");
        story.add(format!("Total wisdom gained (gold): {:.0}%", kintsugi.total_gold() * 100.0));
        story.add(format!("All repaired: {}", if kintsugi.all_repaired() { "YES ✅" } else { "NO ❌" }));
        story.add(format!("More beautiful than before: {}", if kintsugi.is_more_beautiful_than_before() { "YES 💛" } else { "NO" }));
    }

    // ── Epilogue ──
    {
        story.chapter("Epilogue: Seven Eyes, One Vision");
        story.add("The three scholars stand at the river delta, watching the sun set.");
        story.add("");
        story.add("They did not agree on everything. That was never the point.");
        story.add("They each brought their tradition's eyes — and together,");
        story.add("they saw more than any one of them could alone.");
        story.add("");
        story.add("Arjun saw the divine in the water.");
        story.add("Fatima saw the trust in the land.");
        story.add("Kofi saw the community in the soil.");
        story.add("");
        story.add("Seven traditions. Seven eyes.");
        story.add("This was just three of them.");
        story.add("");
        story.add("Imagine what all seven could see together.");
        story.add("");
        story.add("— End of Demo —");
    }

    // Record insight in the bridge network
    network.record_insight(format!(
        "Three traditions reached consensus through {} proposals, {} breaks, and {} golden repairs.",
        palaver.proposals.len(),
        kintsugi.breaks.len(),
        kintsugi.repairs.len(),
    ));

    story
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_produces_chapters() {
        let story = run_demo();
        assert!(story.chapters.len() >= 7, "expected at least 7 chapters, got {}", story.chapters.len());
    }

    #[test]
    fn demo_story_renders() {
        let story = run_demo();
        let rendered = story.render();
        assert!(rendered.contains("PROLOGUE"));
        assert!(rendered.contains("EPILOGUE"));
        assert!(rendered.contains("Arjun"));
        assert!(rendered.contains("Fatima"));
        assert!(rendered.contains("Kofi"));
    }

    #[test]
    fn demo_agents_have_three_traditions() {
        let story = run_demo();
        let rendered = story.render();
        assert!(rendered.contains("Vedic"));
        assert!(rendered.contains("Islamic"));
        assert!(rendered.contains("African"));
    }

    #[test]
    fn demo_training_graduates_all() {
        let mut training_room = TrainingRoom::new(Curriculum::seven_eyes_basics());
        let agents = vec![
            Agent::new("Arjun", Tradition::Vedic),
            Agent::new("Fatima", Tradition::Islamic),
            Agent::new("Kofi", Tradition::African),
        ];
        for a in &agents { training_room.enroll(a); }
        training_room.train(&agents);
        let grads = training_room.graduate();
        assert_eq!(grads.len(), 3);
    }

    #[test]
    fn demo_network_becomes_fully_connected() {
        let story = run_demo();
        let rendered = story.render();
        assert!(rendered.contains("Connected: YES"));
    }

    #[test]
    fn demo_polyrhythm_plays() {
        let story = run_demo();
        let rendered = story.render();
        assert!(rendered.contains("Total beats played"));
    }

    #[test]
    fn demo_palaver_resolves() {
        let story = run_demo();
        let rendered = story.render();
        assert!(rendered.contains("Consensus reached"));
        assert!(rendered.contains("Synthesis:"));
    }

    #[test]
    fn demo_kintsugi_repairs_all() {
        let story = run_demo();
        let rendered = story.render();
        assert!(rendered.contains("All repaired: YES"));
        assert!(rendered.contains("More beautiful than before: YES"));
    }

    #[test]
    fn demo_story_is_readable() {
        let story = run_demo();
        let rendered = story.render();
        // Should be a substantial story
        assert!(rendered.len() > 1000, "story should be >1000 chars, got {}", rendered.len());
        // Should have line breaks
        assert!(rendered.lines().count() > 30);
    }

    #[test]
    fn demo_includes_conservation_concept() {
        let story = run_demo();
        let rendered = story.render();
        assert!(rendered.contains("river delta"));
        assert!(rendered.contains("Conservation"));
    }

    /// The big one — run the full demo with output.
    #[test]
    fn demo_full_story() {
        let story = run_demo();
        let rendered = story.render();
        println!("{}", rendered);
        assert!(!rendered.is_empty());
    }

    #[test]
    fn seven_traditions_all_have_emojis() {
        for &t in Tradition::all() {
            assert!(!t.emoji().is_empty());
        }
    }

    #[test]
    fn chapter_rendering_format() {
        let mut story = DemoStory::new();
        story.chapter("Test Chapter");
        story.add("A line");
        let rendered = story.render();
        assert!(rendered.contains("TEST CHAPTER"));
        assert!(rendered.contains("A line"));
    }

    #[test]
    fn demo_agents_perspectives_are_unique() {
        let agents = vec![
            Agent::new("Arjun", Tradition::Vedic),
            Agent::new("Fatima", Tradition::Islamic),
            Agent::new("Kofi", Tradition::African),
        ];
        let keywords: Vec<&str> = agents.iter()
            .map(|a| a.perspective_on(Concept::Conservation).keyword.as_str())
            .collect();
        // All different
        assert_eq!(keywords.len(), 3);
        assert_ne!(keywords[0], keywords[1]);
        assert_ne!(keywords[1], keywords[2]);
    }
}
