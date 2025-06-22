extern crate alloc;
use crate::core::Ability;
use core::{fmt::Display, str::FromStr};

/// A category of things creatures try to do with an ability check.
///
/// Skills are tied to abilities and represent specific areas of expertise or talent.
///
/// [`Skill`] is enum-like, but is implemented as a private struct to allow more flexibility.
///
/// # Examples
///
/// ```rust
/// use dnd::core::{Skill, Ability};
///
/// let acrobatics = Skill::ACROBATICS;
/// assert_eq!(acrobatics.name(), "Acrobatics");
/// assert_eq!(acrobatics.ability(), Ability::Dexterity);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct Skill {
    name: &'static str,

    #[cfg_attr(feature = "serde", serde(skip))]
    ability: Ability,
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Skill {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let name: &'a str = serde::Deserialize::deserialize(deserializer)?;
        Skill::from_str(name).map_err(serde::de::Error::custom)
    }
}

impl Skill {
    /// All skills available in the game.
    #[must_use]
    pub const fn all() -> &'static [Skill] {
        &[
            Self::ACROBATICS,
            Self::ANIMAL_HANDLING,
            Self::ARCANA,
            Self::ATHLETICS,
            Self::DECEPTION,
            Self::HISTORY,
            Self::INSIGHT,
            Self::INTIMIDATION,
            Self::INVESTIGATION,
            Self::MEDICINE,
            Self::NATURE,
            Self::PERCEPTION,
            Self::PERFORMANCE,
            Self::PERSUASION,
            Self::RELIGION,
            Self::SLEIGHT_OF_HAND,
            Self::STEALTH,
            Self::SURVIVAL,
        ]
    }

    /// Stay on your feet in a tricky situation, or perform an acrobatic stunt.
    pub const ACROBATICS: Skill = Skill::new("Acrobatics", Ability::Dexterity);

    /// Calm or train an animal, or get an animal to behave in a certain way.
    pub const ANIMAL_HANDLING: Skill = Skill::new("Animal Handling", Ability::Wisdom);

    /// Recall lore about spells, magic items, and the planes of existence.
    pub const ARCANA: Skill = Skill::new("Arcana", Ability::Intelligence);

    /// Jump farther than normal, stay afloat in rough water, or break something.
    pub const ATHLETICS: Skill = Skill::new("Athletics", Ability::Strength);

    /// Tell a convincing lie, or wear a disguise convincingly.
    pub const DECEPTION: Skill = Skill::new("Deception", Ability::Charisma);

    /// Recall lore about historical events, people, nations, and cultures.
    pub const HISTORY: Skill = Skill::new("History", Ability::Intelligence);

    /// Discern a person’s mood and intentions.
    pub const INSIGHT: Skill = Skill::new("Insight", Ability::Wisdom);

    /// Awe or threaten someone into doing what you want.
    pub const INTIMIDATION: Skill = Skill::new("Intimidation", Ability::Charisma);

    /// Find obscure information in books, or deduce how something works.
    pub const INVESTIGATION: Skill = Skill::new("Investigation", Ability::Intelligence);

    /// Diagnose an illness, or determine what killed the recently slain.
    pub const MEDICINE: Skill = Skill::new("Medicine", Ability::Wisdom);

    /// Recall lore about terrain, plants, animals, and weather.
    pub const NATURE: Skill = Skill::new("Nature", Ability::Intelligence);

    /// Using a combination of senses, notice something that’s easy to miss.
    pub const PERCEPTION: Skill = Skill::new("Perception", Ability::Wisdom);

    /// Act, tell a story, perform music, or dance.
    pub const PERFORMANCE: Skill = Skill::new("Performance", Ability::Charisma);

    /// Honestly and graciously convince someone of something.
    pub const PERSUASION: Skill = Skill::new("Persuasion", Ability::Charisma);

    /// Recall lore about gods, religious rituals, and holy symbols.
    pub const RELIGION: Skill = Skill::new("Religion", Ability::Intelligence);

    /// Pick a pocket, conceal a handheld object, or perform legerdemain.
    pub const SLEIGHT_OF_HAND: Skill = Skill::new("Sleight of Hand", Ability::Dexterity);

    /// Escape notice by moving quietly and hiding behind things.
    pub const STEALTH: Skill = Skill::new("Stealth", Ability::Dexterity);

    /// Follow tracks, forage, find a trail, or avoid natural hazards.
    pub const SURVIVAL: Skill = Skill::new("Survival", Ability::Wisdom);

    #[must_use]
    const fn new(name: &'static str, ability: Ability) -> Self {
        Self { name, ability }
    }

    /// Returns the name of the skill.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the ability associated with this skill.
    #[must_use]
    pub const fn ability(&self) -> Ability {
        self.ability
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsRef<str> for Skill {
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl FromStr for Skill {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::all()
            .iter()
            .find(|&skill| skill.name() == s)
            .copied()
            .ok_or("Unknown skill")
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;

    use super::*;

    #[test]
    fn names() {
        for skill in Skill::all() {
            assert!(!skill.name().is_empty());
        }
    }

    #[test]
    fn abilities() {
        for skill in Skill::all() {
            assert!(skill.ability().skills().contains(skill));
        }
    }

    #[test]
    fn parse() {
        for skill in Skill::all() {
            assert_eq!(Skill::from_str(skill.name()).unwrap(), *skill);
        }
    }

    #[test]
    fn display() {
        for skill in Skill::all() {
            let mut output = alloc::string::String::new();
            write!(&mut output, "{skill}").unwrap();
            assert_eq!(output, skill.name());
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        use serde_json;

        let skill = Skill::ACROBATICS;
        let serialized = serde_json::to_string(&skill).unwrap();
        assert_eq!(serialized, "\"Acrobatics\"");

        let deserialized: Skill = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, skill);
    }
}
