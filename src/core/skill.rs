use crate::core::Ability;
use core::{fmt::Display, str::FromStr};
use enumflags2::bitflags;

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
/// let acrobatics = Skill::Acrobatics;
/// assert_eq!(acrobatics.name(), "Acrobatics");
/// assert_eq!(acrobatics.ability(), Ability::Dexterity);
/// ```
#[bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum Skill {
    /// Stay on your feet in a tricky situation, or perform an acrobatic stunt.
    Acrobatics,

    /// Calm or train an animal, or get an animal to behave in a certain way.
    AnimalHandling,

    /// Recall lore about spells, magic items, and the planes of existence.
    Arcana,

    /// Jump farther than normal, stay afloat in rough water, or break something.
    Athletics,

    /// Tell a convincing lie, or wear a disguise convincingly.
    Deception,

    /// Recall lore about historical events, people, nations, and cultures.
    History,

    /// Discern a person’s mood and intentions.
    Insight,

    /// Awe or threaten someone into doing what you want.
    Intimidation,

    /// Find obscure information in books, or deduce how something works.
    Investigation,

    /// Diagnose an illness, or determine what killed the recently slain.
    Medicine,

    /// Recall lore about terrain, plants, animals, and weather.
    Nature,

    /// Using a combination of senses, notice something that’s easy to miss.
    Perception,

    /// Act, tell a story, perform music, or dance.
    Performance,

    /// Honestly and graciously convince someone of something.
    Persuasion,

    /// Recall lore about gods, religious rituals, and holy symbols.
    Religion,

    /// Pick a pocket, conceal a handheld object, or perform legerdemain.
    SleightOfHand,

    /// Escape notice by moving quietly and hiding behind things.
    Stealth,

    /// Follow tracks, forage, find a trail, or avoid natural hazards.
    Survival,
}

impl Skill {
    /// All skills available in the game.
    #[must_use]
    pub const fn all() -> &'static [Skill] {
        &[
            Skill::Acrobatics,
            Skill::AnimalHandling,
            Skill::Arcana,
            Skill::Athletics,
            Skill::Deception,
            Skill::History,
            Skill::Insight,
            Skill::Intimidation,
            Skill::Investigation,
            Skill::Medicine,
            Skill::Nature,
            Skill::Perception,
            Skill::Performance,
            Skill::Persuasion,
            Skill::Religion,
            Skill::SleightOfHand,
            Skill::Stealth,
            Skill::Survival,
        ]
    }

    /// Returns the name of the skill.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Skill::Acrobatics => "Acrobatics",
            Skill::AnimalHandling => "Animal Handling",
            Skill::Arcana => "Arcana",
            Skill::Athletics => "Athletics",
            Skill::Deception => "Deception",
            Skill::History => "History",
            Skill::Insight => "Insight",
            Skill::Intimidation => "Intimidation",
            Skill::Investigation => "Investigation",
            Skill::Medicine => "Medicine",
            Skill::Nature => "Nature",
            Skill::Perception => "Perception",
            Skill::Performance => "Performance",
            Skill::Persuasion => "Persuasion",
            Skill::Religion => "Religion",
            Skill::SleightOfHand => "Sleight of Hand",
            Skill::Stealth => "Stealth",
            Skill::Survival => "Survival",
        }
    }

    /// Returns the ability associated with this skill.
    #[must_use]
    pub const fn ability(&self) -> Ability {
        match self {
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => Ability::Dexterity,
            Skill::Athletics => Ability::Strength,
            Skill::AnimalHandling
            | Skill::Insight
            | Skill::Medicine
            | Skill::Perception
            | Skill::Survival => Ability::Wisdom,
            Skill::Arcana
            | Skill::History
            | Skill::Investigation
            | Skill::Nature
            | Skill::Religion => Ability::Intelligence,
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => {
                Ability::Charisma
            }
        }
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
        match s {
            "Acrobatics" => Ok(Skill::Acrobatics),
            "Animal Handling" => Ok(Skill::AnimalHandling),
            "Arcana" => Ok(Skill::Arcana),
            "Athletics" => Ok(Skill::Athletics),
            "Deception" => Ok(Skill::Deception),
            "History" => Ok(Skill::History),
            "Insight" => Ok(Skill::Insight),
            "Intimidation" => Ok(Skill::Intimidation),
            "Investigation" => Ok(Skill::Investigation),
            "Medicine" => Ok(Skill::Medicine),
            "Nature" => Ok(Skill::Nature),
            "Perception" => Ok(Skill::Perception),
            "Performance" => Ok(Skill::Performance),
            "Persuasion" => Ok(Skill::Persuasion),
            "Religion" => Ok(Skill::Religion),
            "Sleight of Hand" => Ok(Skill::SleightOfHand),
            "Stealth" => Ok(Skill::Stealth),
            "Survival" => Ok(Skill::Survival),
            _ => Err("Unknown skill"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use core::fmt::Write;

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

        let skill = Skill::Acrobatics;
        let serialized = serde_json::to_string(&skill).unwrap();
        assert_eq!(serialized, "\"Acrobatics\"");

        let deserialized: Skill = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, skill);
    }
}
