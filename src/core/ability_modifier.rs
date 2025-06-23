use crate::core::AbilityScore;

/// Represents a modifier to a D20 test, often originating from an [`AbilityScore`][].
///
/// [`AbilityScore`]: [`dnd::core::AbilityScore`]
///
/// An modifier score is a [`i8`] value in the range of `-5..==10`.
///
/// # Examples
///
/// ```rust
/// use dnd::core::{AbilityModifier, AbilityScore};
///
/// let modifier = AbilityModifier::new(3);
/// assert_eq!(modifier.value(), 3);
/// ```
///
/// # Conversion from [`AbilityScore`][]
///
/// An `AbilityModifier` can be created directly from an `AbilityScore` using either:
///
/// ```rust
/// use dnd::core::{AbilityModifier, AbilityScore};
///
/// let score = AbilityScore::new(16);
/// let modifier = AbilityModifier::from(score);
/// assert_eq!(modifier.value(), 3);
/// ```
///
/// Or, using the `modifier()` method on `AbilityScore`:
///
/// ```rust
/// use dnd::core::{AbilityModifier, AbilityScore};
///
/// let score = AbilityScore::new(16);
/// let modifier = score.modifier();
/// assert_eq!(modifier.value(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct AbilityModifier(i8);

impl AbilityModifier {
    /// The minimum possible value for an ability modifier.
    pub const MIN: Self = Self(-5);

    /// The maximum possible value for an ability modifier.
    pub const MAX: Self = Self(10);

    /// Creates a new `AbilityModifier` with the given value.
    ///
    /// In debug mode, this will panic if the value is outside the valid range of 1 to 30.
    ///
    /// In release mode, it will clamp the value to the range of [`Self::MIN`] to [`Self::MAX`].
    #[must_use]
    pub const fn new(value: i8) -> Self {
        debug_assert!(
            !(value < Self::MIN.value() || value > Self::MAX.value()),
            "Ability modifier must be between -5 and 10"
        );
        Self::new_clamped(value)
    }

    /// Creates a new `AbilityModifier` with the given value.
    ///
    /// The value is automatically clamped to the range of [`Self::MIN`] to [`Self::MAX`].
    #[must_use]
    pub const fn new_clamped(value: i8) -> Self {
        if value < Self::MIN.value() {
            Self::MIN
        } else if value > Self::MAX.value() {
            Self::MAX
        } else {
            Self(value)
        }
    }

    /// Creates a new `AbilityModifier` with the given value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the valid range of -5 to 10.
    pub fn try_new(value: i8) -> Result<Self, &'static str> {
        if value < Self::MIN.value() {
            Err("Ability modifier cannot be less than -5")
        } else if value > Self::MAX.value() {
            Err("Ability modifier cannot be greater than 10")
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the value of the ability modifier.
    #[must_use]
    pub const fn value(&self) -> i8 {
        self.0
    }
}

impl TryFrom<i8> for AbilityModifier {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<AbilityModifier> for i8 {
    fn from(modifier: AbilityModifier) -> Self {
        modifier.value()
    }
}

impl From<AbilityScore> for AbilityModifier {
    fn from(value: AbilityScore) -> Self {
        value.modifier()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clamped_min() {
        let modifier = AbilityModifier::new_clamped(-10);
        assert_eq!(modifier, AbilityModifier::MIN);
    }

    #[test]
    fn new_clamped_max() {
        let modifier = AbilityModifier::new_clamped(20);
        assert_eq!(modifier, AbilityModifier::MAX);
    }

    #[test]
    fn try_new_min() {
        let modifier = AbilityModifier::try_new(-10);
        assert_eq!(modifier, Err("Ability modifier cannot be less than -5"));
    }

    #[test]
    fn try_new_max() {
        let modifier = AbilityModifier::try_new(20);
        assert_eq!(modifier, Err("Ability modifier cannot be greater than 10"));
    }

    #[test]
    fn try_new_valid() {
        let modifier = AbilityModifier::try_new(3);
        assert_eq!(modifier, Ok(AbilityModifier(3)));
    }

    #[test]
    #[should_panic(expected = "Ability modifier must be between -5 and 10")]
    fn new_panic() {
        let _modifier = AbilityModifier::new(20);
    }

    #[test]
    fn new() {
        let modifier = AbilityModifier::new(5);
        assert_eq!(modifier, AbilityModifier(5));
    }

    #[test]
    fn value() {
        let modifier = AbilityModifier::new_clamped(5);
        assert_eq!(modifier.value(), 5);
    }

    #[test]
    fn from_i8() {
        let modifier: AbilityModifier = 4i8.try_into().unwrap();
        assert_eq!(modifier, AbilityModifier(4));
    }

    #[test]
    fn into_i8() {
        let modifier = AbilityModifier::new_clamped(7);
        let value: i8 = modifier.into();
        assert_eq!(value, 7);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_serialize() {
        let modifier = AbilityModifier::new_clamped(2);
        let serialized = serde_json::to_string(&modifier).unwrap();
        assert_eq!(serialized, "2");

        let deserialized: AbilityModifier = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, modifier);
    }
}
