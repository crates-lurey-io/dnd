use core::ops::{Index, IndexMut};

use crate::core::{Ability, AbilityScore};

/// The six ability scores of a creature.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Abilities {
    /// Physical might.
    pub strength: AbilityScore,

    /// Agility, reflexes, and balance.
    pub dexterity: AbilityScore,

    /// Health and stamina.
    pub constitution: AbilityScore,

    /// Reasoning and memory.
    pub intelligence: AbilityScore,

    /// Perceptiveness and mental fortitude.
    pub wisdom: AbilityScore,

    /// Confidence, poise, and charm.
    pub charisma: AbilityScore,
}

impl Abilities {
    /// Creates a new `Abilities` instance with default ability scores (value of 10 for each).
    #[must_use]
    pub const fn new() -> Self {
        Self::with_uniform(AbilityScore::DEFAULT)
    }

    /// Creates a new `Abilities` instance using the same uniform value for all abilities.
    #[must_use]
    pub const fn with_uniform(value: AbilityScore) -> Self {
        Self {
            strength: value,
            dexterity: value,
            constitution: value,
            intelligence: value,
            wisdom: value,
            charisma: value,
        }
    }

    /// Returns an iterator of each ability and it's cooresponding score.
    pub fn iter(&self) -> impl Iterator<Item = (Ability, AbilityScore)> {
        Ability::all()
            .iter()
            .map(move |&ability| (ability, self[ability]))
    }
}

impl Default for Abilities {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<Ability> for Abilities {
    type Output = AbilityScore;

    /// Returns the ability score for the given ability.
    fn index(&self, ability: Ability) -> &Self::Output {
        match ability {
            Ability::Strength => &self.strength,
            Ability::Dexterity => &self.dexterity,
            Ability::Constitution => &self.constitution,
            Ability::Intelligence => &self.intelligence,
            Ability::Wisdom => &self.wisdom,
            Ability::Charisma => &self.charisma,
        }
    }
}

impl IndexMut<Ability> for Abilities {
    /// Returns a mutable reference to the ability score for the given ability.
    fn index_mut(&mut self, ability: Ability) -> &mut Self::Output {
        match ability {
            Ability::Strength => &mut self.strength,
            Ability::Dexterity => &mut self.dexterity,
            Ability::Constitution => &mut self.constitution,
            Ability::Intelligence => &mut self.intelligence,
            Ability::Wisdom => &mut self.wisdom,
            Ability::Charisma => &mut self.charisma,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_default() {
        assert_eq!(
            Abilities::new(),
            Abilities::with_uniform(AbilityScore::DEFAULT)
        );
    }

    #[test]
    fn with_uniform() {
        let score = AbilityScore::try_new(12).unwrap();
        let abilities = Abilities::with_uniform(score);
        assert_eq!(abilities.strength, score);
        assert_eq!(abilities.dexterity, score);
        assert_eq!(abilities.constitution, score);
        assert_eq!(abilities.intelligence, score);
        assert_eq!(abilities.wisdom, score);
        assert_eq!(abilities.charisma, score);
    }

    #[test]
    fn iter() {
        let abilities = Abilities {
            strength: AbilityScore::new_clamped(12),
            dexterity: AbilityScore::new_clamped(14),
            constitution: AbilityScore::new_clamped(13),
            intelligence: AbilityScore::new_clamped(15),
            wisdom: AbilityScore::new_clamped(10),
            charisma: AbilityScore::new_clamped(8),
        };

        let expected = [
            (Ability::Strength, AbilityScore::new_clamped(12)),
            (Ability::Dexterity, AbilityScore::new_clamped(14)),
            (Ability::Constitution, AbilityScore::new_clamped(13)),
            (Ability::Intelligence, AbilityScore::new_clamped(15)),
            (Ability::Wisdom, AbilityScore::new_clamped(10)),
            (Ability::Charisma, AbilityScore::new_clamped(8)),
        ];

        let mut iter = abilities.iter();
        for (ability, score) in expected {
            assert_eq!(iter.next(), Some((ability, score)));
        }
    }

    #[test]
    fn index() {
        let abilities = Abilities {
            strength: AbilityScore::new_clamped(12),
            dexterity: AbilityScore::new_clamped(14),
            constitution: AbilityScore::new_clamped(13),
            intelligence: AbilityScore::new_clamped(15),
            wisdom: AbilityScore::new_clamped(10),
            charisma: AbilityScore::new_clamped(8),
        };

        assert_eq!(abilities[Ability::Strength], AbilityScore::new_clamped(12));
        assert_eq!(abilities[Ability::Dexterity], AbilityScore::new_clamped(14));
        assert_eq!(
            abilities[Ability::Constitution],
            AbilityScore::new_clamped(13)
        );
        assert_eq!(
            abilities[Ability::Intelligence],
            AbilityScore::new_clamped(15)
        );
        assert_eq!(abilities[Ability::Wisdom], AbilityScore::new_clamped(10));
        assert_eq!(abilities[Ability::Charisma], AbilityScore::new_clamped(8));
    }

    #[test]
    fn index_mut() {
        let mut abilities = Abilities::new();
        abilities[Ability::Strength] = AbilityScore::new_clamped(18);
        assert_eq!(abilities.strength, AbilityScore::new_clamped(18));
        abilities[Ability::Dexterity] = AbilityScore::new_clamped(16);
        assert_eq!(abilities.dexterity, AbilityScore::new_clamped(16));
        abilities[Ability::Constitution] = AbilityScore::new_clamped(14);
        assert_eq!(abilities.constitution, AbilityScore::new_clamped(14));
        abilities[Ability::Intelligence] = AbilityScore::new_clamped(12);
        assert_eq!(abilities.intelligence, AbilityScore::new_clamped(12));
        abilities[Ability::Wisdom] = AbilityScore::new_clamped(10);
        assert_eq!(abilities.wisdom, AbilityScore::new_clamped(10));
        abilities[Ability::Charisma] = AbilityScore::new_clamped(8);
        assert_eq!(abilities.charisma, AbilityScore::new_clamped(8));
    }
}
