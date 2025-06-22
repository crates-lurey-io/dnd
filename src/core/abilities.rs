use core::ops::{Index, IndexMut};

use crate::core::{Ability, AbilityScore};

/// The six ability scores of a creature.
///
/// # Examples
///
/// ```rust
/// use dnd::core::{Abilities, AbilityScore};
///
/// let abilities = Abilities::new();
/// assert_eq!(abilities.strength, AbilityScore::new(10));
/// assert_eq!(abilities.dexterity, AbilityScore::new(10));
/// ```
///
/// Or, to create a set with specific values for each ability:
///
/// ```rust
/// use dnd::core::{Abilities, Ability, AbilityScore};
///
/// let abilities = Abilities {
///   strength: AbilityScore::new(16),
///   dexterity: AbilityScore::new(14),
///   constitution: AbilityScore::new(15),
///   intelligence: AbilityScore::new(13),
///   wisdom: AbilityScore::new(12),
///   charisma: AbilityScore::new(10),
/// };
/// ```
///
/// # Indexing
///
/// You can also dynamically access each ability score using the [`Ability`] enum:
///
/// ```rust
/// use dnd::core::{Abilities, Ability, AbilityScore};
///
/// let mut abilities = Abilities::new();
/// abilities[Ability::Strength] = AbilityScore::new(18);
/// assert_eq!(abilities[Ability::Strength], AbilityScore::new(18));
/// ```
///
/// # Iteration
///
/// You can iterate over all abilities and their scores using the `iter` method:
///
/// ```rust
/// use dnd::core::{Abilities, Ability, AbilityScore};
///
/// let abilities = Abilities::new();
/// for (ability, score) in abilities.iter() {
///    println!("{:?}: {}", ability, score.value());
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Abilities {
    /// Physical might.
    ///
    /// To read or write to this field with indexing, use [`Ability::Strength`].
    pub strength: AbilityScore,

    /// Agility, reflexes, and balance.
    ///
    /// To read or write to this field with indexing, use [`Ability::Dexterity`].
    pub dexterity: AbilityScore,

    /// Health and stamina.
    ///
    /// To read or write to this field with indexing, use [`Ability::Constitution`].
    pub constitution: AbilityScore,

    /// Reasoning and memory.
    ///
    /// To read or write to this field with indexing, use [`Ability::Intelligence`].
    pub intelligence: AbilityScore,

    /// Perceptiveness and mental fortitude.
    ///
    /// To read or write to this field with indexing, use [`Ability::Wisdom`].
    pub wisdom: AbilityScore,

    /// Confidence, poise, and charm.
    ///
    /// To read or write to this field with indexing, use [`Ability::Charisma`].
    pub charisma: AbilityScore,
}

impl Abilities {
    /// Creates a new `Abilities` instance with default ability scores (value of 10 for each).
    ///
    /// This is equivalent to calling `Abilities::with_uniform(AbilityScore::DEFAULT)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dnd::core::{Abilities, AbilityScore};
    ///
    /// let abilities = Abilities::new();
    /// assert_eq!(abilities.strength, AbilityScore::new(10));
    /// assert_eq!(abilities.dexterity, AbilityScore::new(10));
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self::with_uniform(AbilityScore::DEFAULT)
    }

    /// Creates a new `Abilities` instance using the same uniform value for all abilities.
    ///
    /// This is useful for quickly initializing abilities to the same score.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dnd::core::{Abilities, AbilityScore};
    ///
    /// let abilities = Abilities::with_uniform(AbilityScore::new(12));
    /// assert_eq!(abilities.strength, AbilityScore::new(12));
    /// assert_eq!(abilities.dexterity, AbilityScore::new(12));
    /// ```
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
    ///
    /// The abilities are returned in the order defined by the [`Ability`] enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dnd::core::{Abilities, Ability, AbilityScore};
    ///
    /// let abilities = Abilities::new();
    /// for (ability, score) in abilities.iter() {
    ///   println!("{:?}: {}", ability, score.value());
    /// }
    /// ```
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
