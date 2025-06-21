extern crate alloc;
use core::{fmt::Display, str::FromStr};

/// Six abilities that measure physical and mental characteristics of creatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Returns an array containing all possible [`AbilityType`] variants.
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
        match s.to_lowercase().as_str() {
            "strength" | "str" => Ok(Ability::Strength),
            "dexterity" | "dex" => Ok(Ability::Dexterity),
            "constitution" | "con" => Ok(Ability::Constitution),
            "intelligence" | "int" => Ok(Ability::Intelligence),
            "wisdom" | "wis" => Ok(Ability::Wisdom),
            "charisma" | "cha" => Ok(Ability::Charisma),
            _ => Err("Invalid ability name"),
        }
    }
}

#[cfg(test)]
mod tests {
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
            ("strength", Ability::Strength),
            ("str", Ability::Strength),
            ("dexterity", Ability::Dexterity),
            ("dex", Ability::Dexterity),
            ("constitution", Ability::Constitution),
            ("con", Ability::Constitution),
            ("intelligence", Ability::Intelligence),
            ("int", Ability::Intelligence),
            ("wisdom", Ability::Wisdom),
            ("wis", Ability::Wisdom),
            ("charisma", Ability::Charisma),
            ("cha", Ability::Charisma),
        ];

        for (input, expected) in cases {
            assert_eq!(Ability::from_str(input).unwrap(), expected);
        }

        assert!(Ability::from_str("invalid").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde() {
        use serde_json;

        let ability = Ability::Strength;
        let serialized = serde_json::to_string(&ability).unwrap();
        assert_eq!(serialized, "\"Strength\"");

        let deserialized: Ability = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ability);
    }
}
