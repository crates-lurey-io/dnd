use core::fmt::Display;

use crate::core::AbilityModifier;

/// Represents the magnitude of an [`Ability`][].
///
/// [`Ability`]: [`dnd::core::Ability`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AbilityScore(u8);

impl AbilityScore {
    /// The minimum possible value for an ability score.
    pub const MIN: AbilityScore = AbilityScore(1);

    /// The maximum possible value for an ability score.
    pub const MAX: AbilityScore = AbilityScore(30);

    /// A sensible default value for an ability score, which is 10.
    pub const DEFAULT: AbilityScore = AbilityScore(10);

    /// Creates a new `AbilityScore` with the given value.
    ///
    /// The value is automatically clamped to the range of [`Self::MIN`] to [`Self::MAX`].
    #[must_use]
    pub const fn new_clamped(value: u8) -> Self {
        if value < Self::MIN.value() {
            Self::MIN
        } else if value > Self::MAX.value() {
            Self::MAX
        } else {
            Self(value)
        }
    }

    /// Creates a new `AbilityScore` with the given value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the valid range of 1 to 30.
    pub fn try_new(value: u8) -> Result<Self, &'static str> {
        if value < Self::MIN.value() {
            Err("Ability score cannot be less than 1")
        } else if value > Self::MAX.value() {
            Err("Ability score cannot be greater than 30")
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the value of the ability score.
    #[must_use]
    pub const fn value(&self) -> u8 {
        self.0
    }

    /// Returns the [`AbilityModifier`] for this ability score.
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub const fn modifier(&self) -> AbilityModifier {
        AbilityModifier::new_clamped((self.value() as i8 - 10) >> 1)
    }
}

impl Default for AbilityScore {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl TryFrom<u8> for AbilityScore {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<AbilityScore> for u8 {
    fn from(score: AbilityScore) -> Self {
        score.value()
    }
}

impl Display for AbilityScore {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use alloc::string::String;
    use core::fmt::Write;

    #[test]
    fn new_clamped_min() {
        let score = AbilityScore::new_clamped(0);
        assert_eq!(score, AbilityScore::MIN);
    }

    #[test]
    fn new_clamped_max() {
        let score = AbilityScore::new_clamped(31);
        assert_eq!(score, AbilityScore::MAX);
    }

    #[test]
    fn new_clamped_valid() {
        let score = AbilityScore::new_clamped(15);
        assert_eq!(score, AbilityScore(15));
    }

    #[test]
    fn try_new_min() {
        let score = AbilityScore::try_new(0);
        assert_eq!(score, Err("Ability score cannot be less than 1"));
    }

    #[test]
    fn try_new_max() {
        let score = AbilityScore::try_new(31);
        assert_eq!(score, Err("Ability score cannot be greater than 30"));
    }

    #[test]
    fn try_new_valid() {
        let score = AbilityScore::try_new(15);
        assert_eq!(score, Ok(AbilityScore(15)));
    }

    #[test]
    fn value() {
        let score = AbilityScore(15);
        assert_eq!(score.value(), 15);
    }

    #[test]
    fn modifier() {
        let expected: [(u8, i8); 30] = [
            (1, -5),
            (2, -4),
            (3, -4),
            (4, -3),
            (5, -3),
            (6, -2),
            (7, -2),
            (8, -1),
            (9, -1),
            (10, 0),
            (11, 0),
            (12, 1),
            (13, 1),
            (14, 2),
            (15, 2),
            (16, 3),
            (17, 3),
            (18, 4),
            (19, 4),
            (20, 5),
            (21, 5),
            (22, 6),
            (23, 6),
            (24, 7),
            (25, 7),
            (26, 8),
            (27, 8),
            (28, 9),
            (29, 9),
            (30, 10),
        ];
        for (score, modifier) in expected {
            let ability_score = AbilityScore(score);
            assert_eq!(
                ability_score.modifier().value(),
                modifier,
                "Modifier mismatch for score {score}"
            );
        }
    }

    #[test]
    fn from_u8() {
        let score: AbilityScore = 20u8.try_into().unwrap();
        assert_eq!(score, AbilityScore(20));
    }

    #[test]
    fn into_u8() {
        let score = AbilityScore(25);
        let value: u8 = score.into();
        assert_eq!(value, 25);
    }

    #[test]
    fn display() {
        let mut output = String::new();
        write!(&mut output, "{}", AbilityScore(18)).unwrap();
        assert_eq!(output, "18");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_serialize() {
        let score = AbilityScore(18);
        let serialized = serde_json::to_string(&score).unwrap();
        assert_eq!(serialized, "18");

        let deserialized: AbilityScore = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, AbilityScore(18));
    }
}
