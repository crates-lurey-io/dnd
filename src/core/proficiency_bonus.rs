use crate::core::Level;

/// Represents a proficiency bonus.
///
/// A proficiency bonus is a [`u8`] value in the range of `2..=9`.
///
/// # Examples
///
/// ```rust
/// use dnd::core::{ProficiencyBonus, Level};
///
/// let bonus = ProficiencyBonus::new(3);
/// assert_eq!(bonus.value(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct ProficiencyBonus(u8);

impl ProficiencyBonus {
    /// The minimum possible proficiency bonus.
    pub const MIN: Self = Self(2);

    /// The maximum possible proficiency bonus.
    pub const MAX: Self = Self(9);

    /// Creates a new `ProficiencyBonus` with the given value.
    ///
    /// In debug mode, this will panic if the value is outside the valid range of 2 to 9.
    ///
    /// In release mode, it will clamp the value to the range of [`Self::MIN`] to [`Self::MAX`].
    #[must_use]
    pub const fn new(value: u8) -> Self {
        debug_assert!(
            !(value < Self::MIN.value() || value > Self::MAX.value()),
            "Proficiency bonus must be between 2 and 9"
        );
        Self::new_clamped(value)
    }

    /// Creates a new `ProficiencyBonus` with the given value.
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

    /// Creates a new `ProficiencyBonus` with the given value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the valid range of 2 to 9.
    pub fn try_new(value: u8) -> Result<Self, &'static str> {
        if value < Self::MIN.value() {
            Err("Proficiency bonus cannot be less than 2")
        } else if value > Self::MAX.value() {
            Err("Proficiency bonus cannot be greater than 9")
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the value of the proficiency bonus.
    #[must_use]
    pub const fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for ProficiencyBonus {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<ProficiencyBonus> for u8 {
    fn from(bonus: ProficiencyBonus) -> Self {
        bonus.value()
    }
}

impl From<Level> for ProficiencyBonus {
    fn from(level: Level) -> Self {
        level.proficiency_bonus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clamped_min() {
        let bonus = ProficiencyBonus::new_clamped(1);
        assert_eq!(bonus, ProficiencyBonus::MIN);
    }

    #[test]
    fn new_clamped_max() {
        let bonus = ProficiencyBonus::new_clamped(9);
        assert_eq!(bonus, ProficiencyBonus::MAX);
    }

    #[test]
    fn new_clamped_valid() {
        let bonus = ProficiencyBonus::new_clamped(5);
        assert_eq!(bonus.value(), 5);
    }

    #[test]
    fn try_new_min() {
        let bonus = ProficiencyBonus::try_new(1);
        assert_eq!(bonus, Err("Proficiency bonus cannot be less than 2"));
    }

    #[test]
    fn try_new_max() {
        let bonus = ProficiencyBonus::try_new(19);
        assert_eq!(bonus, Err("Proficiency bonus cannot be greater than 9"));
    }

    #[test]
    fn try_new_valid() {
        let bonus = ProficiencyBonus::try_new(5);
        assert_eq!(bonus, Ok(ProficiencyBonus(5)));
    }

    #[test]
    #[should_panic(expected = "Proficiency bonus must be between 2 and 9")]
    fn new_panic() {
        let _bonus = ProficiencyBonus::new(1);
    }

    #[test]
    fn new() {
        let bonus = ProficiencyBonus::new(5);
        assert_eq!(bonus, ProficiencyBonus(5));
    }

    #[test]
    fn value() {
        let bonus = ProficiencyBonus(5);
        assert_eq!(bonus.value(), 5);
    }

    #[test]
    fn try_from_u8() {
        let bonus: ProficiencyBonus = 6u8.try_into().unwrap();
        assert_eq!(bonus, ProficiencyBonus(6));
    }

    #[test]
    fn from_u8() {
        let bonus: u8 = ProficiencyBonus(7).into();
        assert_eq!(bonus, 7);
    }

    #[test]
    fn into_u8() {
        let bonus = ProficiencyBonus(8);
        let value: u8 = bonus.into();
        assert_eq!(value, 8);
    }

    #[test]
    fn from_level() {
        let level = Level::new_clamped(5);
        let bonus: ProficiencyBonus = level.into();
        assert_eq!(bonus, ProficiencyBonus(3));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serialize() {
        let bonus = ProficiencyBonus(5);
        let serialized = serde_json::to_string(&bonus).unwrap();
        assert_eq!(serialized, "5");

        let deserialized: ProficiencyBonus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, bonus);
    }
}
