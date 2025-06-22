use crate::core::ProficiencyBonus;

/// Level of a player character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Level(u8);

impl Level {
    /// The minimum possible level.
    pub const MIN: Self = Self(1);

    /// The maximum possible level.
    pub const MAX: Self = Self(20);

    /// Creates a new `Level` with the given value.
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

    /// Creates a new `Level` with the given value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the valid range of 1 to 20.
    pub fn try_new(value: u8) -> Result<Self, &'static str> {
        if value < Self::MIN.value() {
            Err("Level cannot be less than 1")
        } else if value > Self::MAX.value() {
            Err("Level cannot be greater than 20")
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the value of the level.
    #[must_use]
    pub const fn value(&self) -> u8 {
        self.0
    }

    /// Returns the proficiency bonus for this level.
    #[must_use]
    pub const fn proficiency_bonus(&self) -> ProficiencyBonus {
        ProficiencyBonus::new_clamped(match self.value() {
            1..=4 => 2,
            5..=8 => 3,
            9..=12 => 4,
            13..=16 => 5,
            17..=20 => 6,
            21..=24 => 7,
            25..=28 => 8,
            29..=30 => 9,
            _ => unreachable!(),
        })
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::MIN
    }
}

impl TryFrom<u8> for Level {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<Level> for u8 {
    fn from(level: Level) -> Self {
        level.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clamped_min() {
        let level = Level::new_clamped(0);
        assert_eq!(level, Level::MIN);
    }

    #[test]
    fn new_clamped_max() {
        let level = Level::new_clamped(21);
        assert_eq!(level, Level::MAX);
    }

    #[test]
    fn new_clamped_valid() {
        let level = Level::new_clamped(10);
        assert_eq!(level, Level(10));
    }

    #[test]
    fn try_new_min() {
        let level = Level::try_new(0);
        assert_eq!(level, Err("Level cannot be less than 1"));
    }

    #[test]
    fn try_new_max() {
        let level = Level::try_new(21);
        assert_eq!(level, Err("Level cannot be greater than 20"));
    }

    #[test]
    fn try_new_valid() {
        let level = Level::try_new(10);
        assert_eq!(level, Ok(Level(10)));
    }

    #[test]
    fn value() {
        let level = Level(15);
        assert_eq!(level.value(), 15);
    }

    #[test]
    fn proficiency_bonus() {
        let expected = [
            (1..=4, 2),
            (5..=8, 3),
            (9..=12, 4),
            (13..=16, 5),
            (17..=20, 6),
            (21..=24, 7),
            (25..=28, 8),
            (29..=30, 9),
        ];

        for (range, bonus) in expected {
            for level in range {
                assert_eq!(
                    Level(level).proficiency_bonus(),
                    ProficiencyBonus::new_clamped(bonus)
                );
            }
        }
    }

    #[test]
    fn default() {
        assert_eq!(Level::default(), Level::MIN);
    }

    #[test]
    fn try_from_u8() {
        let level: Level = 5u8.try_into().unwrap();
        assert_eq!(level, Level(5));
    }

    #[test]
    fn from_u8() {
        let level: u8 = Level(7).into();
        assert_eq!(level, 7);
    }

    #[test]
    fn into_u8() {
        let level = Level(12);
        let value: u8 = level.into();
        assert_eq!(value, 12);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_serialize() {
        let level = Level(18);
        let serialized = serde_json::to_string(&level).unwrap();
        assert_eq!(serialized, "18");

        let deserialized: Level = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, level);
    }
}
