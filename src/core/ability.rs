use crate::core::Skill;
use core::{fmt::Display, str::FromStr};
use enumflags2::bitflags;

/// Six abilities that measure physical and mental characteristics of creatures.
///
/// Each ability has a name, an abbreviation, and is associated with certain skills.
///
/// # Examples
///
/// ```rust
/// use dnd::core::{Ability, Skill};
///
/// let strength = Ability::Strength;
/// assert_eq!(strength.name(), "Strength");
/// assert_eq!(strength.abbr(), "STR");
/// assert_eq!(strength.skills(), &[Skill::Athletics]);
/// ```
#[bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Ability {
    /// Physical might.
    Strength,

    /// Agility, reflexes, and balance.
    Dexterity,

    /// Health and stamina.
    Constitution,

    /// Reasoning and memory
    Intelligence,

    /// Perceptiveness and mental fortitude
    Wisdom,

    /// Confidence, poise, and charm.
    Charisma,
}

impl Ability {
    /// Returns an array containing all possible [`Ability`] variants.
    ///
    /// The elements are ordered in the same way as the enum definition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dnd::core::Ability;
    ///
    /// let abilities = Ability::all();
    /// assert_eq!(abilities.len(), 6);
    /// assert_eq!(abilities.first(), Some(&Ability::Strength));
    /// assert_eq!(abilities.last(), Some(&Ability::Charisma));
    /// ```
    #[must_use]
    pub const fn all() -> &'static [Ability] {
        &[
            Ability::Strength,
            Ability::Dexterity,
            Ability::Constitution,
            Ability::Intelligence,
            Ability::Wisdom,
            Ability::Charisma,
        ]
    }

    /// Returns the full name of the ability as a string slice, in title case.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dnd::core::Ability;
    ///
    /// assert_eq!(Ability::Strength.name(), "Strength");
    /// assert_eq!(Ability::Dexterity.name(), "Dexterity");
    /// ```
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Ability::Strength => "Strength",
            Ability::Dexterity => "Dexterity",
            Ability::Constitution => "Constitution",
            Ability::Intelligence => "Intelligence",
            Ability::Wisdom => "Wisdom",
            Ability::Charisma => "Charisma",
        }
    }

    /// Returns the abbreviation of the ability as a string slice, in all uppercase letters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dnd::core::Ability;
    ///
    /// assert_eq!(Ability::Strength.abbr(), "STR");
    /// assert_eq!(Ability::Dexterity.abbr(), "DEX");
    /// ```
    #[must_use]
    pub const fn abbr(&self) -> &'static str {
        match self {
            Ability::Strength => "STR",
            Ability::Dexterity => "DEX",
            Ability::Constitution => "CON",
            Ability::Intelligence => "INT",
            Ability::Wisdom => "WIS",
            Ability::Charisma => "CHA",
        }
    }

    /// Skills that are associated with this ability.
    #[must_use]
    pub const fn skills(&self) -> &'static [Skill] {
        match self {
            Ability::Strength => &[Skill::Athletics],
            Ability::Dexterity => &[Skill::Acrobatics, Skill::SleightOfHand, Skill::Stealth],
            Ability::Constitution => &[],
            Ability::Intelligence => &[
                Skill::Arcana,
                Skill::History,
                Skill::Investigation,
                Skill::Nature,
                Skill::Religion,
            ],
            Ability::Wisdom => &[
                Skill::AnimalHandling,
                Skill::Insight,
                Skill::Medicine,
                Skill::Perception,
                Skill::Survival,
            ],
            Ability::Charisma => &[
                Skill::Deception,
                Skill::Intimidation,
                Skill::Performance,
                Skill::Persuasion,
            ],
        }
    }
}

impl Display for Ability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsRef<str> for Ability {
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl FromStr for Ability {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Strength" | "STR" => Ok(Ability::Strength),
            "Dexterity" | "DEX" => Ok(Ability::Dexterity),
            "Constitution" | "CON" => Ok(Ability::Constitution),
            "Intelligence" | "INT" => Ok(Ability::Intelligence),
            "Wisdom" | "WIS" => Ok(Ability::Wisdom),
            "Charisma" | "CHA" => Ok(Ability::Charisma),
            _ => Err("Unknown ability"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::string::String;
    use core::fmt::Write;

    #[test]
    fn all() {
        assert_eq!(
            Ability::all(),
            &[
                Ability::Strength,
                Ability::Dexterity,
                Ability::Constitution,
                Ability::Intelligence,
                Ability::Wisdom,
                Ability::Charisma,
            ]
        );
    }

    #[test]
    fn name() {
        let cases = [
            (Ability::Strength, "Strength"),
            (Ability::Dexterity, "Dexterity"),
            (Ability::Constitution, "Constitution"),
            (Ability::Intelligence, "Intelligence"),
            (Ability::Wisdom, "Wisdom"),
            (Ability::Charisma, "Charisma"),
        ];
        for (ability, expected_name) in cases {
            assert_eq!(ability.name(), expected_name);
        }
    }

    #[test]
    fn abbr() {
        let cases = [
            (Ability::Strength, "STR"),
            (Ability::Dexterity, "DEX"),
            (Ability::Constitution, "CON"),
            (Ability::Intelligence, "INT"),
            (Ability::Wisdom, "WIS"),
            (Ability::Charisma, "CHA"),
        ];
        for (ability, expected_abbr) in cases {
            assert_eq!(ability.abbr(), expected_abbr);
        }
    }

    #[test]
    fn display() {
        // Assume `Display` is implemented correctly other than Strength
        let mut output = String::new();
        write!(&mut output, "{}", Ability::Strength).unwrap();
        assert_eq!(output, "Strength");
    }

    #[test]
    fn as_ref() {
        let ability = Ability::Dexterity;
        assert_eq!(ability.as_ref(), "Dexterity");
    }

    #[test]
    fn from_str() {
        let cases = [
            ("Strength", Ability::Strength),
            ("STR", Ability::Strength),
            ("Dexterity", Ability::Dexterity),
            ("DEX", Ability::Dexterity),
            ("Constitution", Ability::Constitution),
            ("CON", Ability::Constitution),
            ("Intelligence", Ability::Intelligence),
            ("INT", Ability::Intelligence),
            ("Wisdom", Ability::Wisdom),
            ("WIS", Ability::Wisdom),
            ("Charisma", Ability::Charisma),
            ("CHA", Ability::Charisma),
        ];

        for (input, expected) in cases {
            assert_eq!(Ability::from_str(input).unwrap(), expected);
        }

        assert!(Ability::from_str("invalid").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_json() {
        use serde_json;

        let ability = Ability::Strength;
        let serialized = serde_json::to_string(&ability).unwrap();
        assert_eq!(serialized, "\"Strength\"");

        let deserialized: Ability = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ability);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_postcard() {
        use postcard;

        let ability = Ability::Dexterity;
        let serialized = postcard::to_vec::<_, 8>(&ability).unwrap();
        assert_eq!(serialized.as_slice(), &[1]);

        let deserialized: Ability = postcard::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized, ability);
    }

    #[test]
    fn skills() {
        // Check against Skill::all()
        for ability in Ability::all() {
            for skill in ability.skills() {
                let expected = Skill::all().iter().find(|s| s.ability() == *ability);
                assert!(
                    expected.is_some(),
                    "Skill {skill:?} should be associated with {ability:?}"
                );
            }
        }
    }
}
