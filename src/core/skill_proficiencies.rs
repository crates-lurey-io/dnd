use crate::core::Skill;
use enumflags2::BitFlags;

/// Represents the proficiency level a creature has in a skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum SkillLevel {
    /// Proficient in a skill.
    Proficient,

    /// Has expertise in a skill; double the proficiency bonus when using this skill.
    Expertise,
}

/// What skill proficiencies a creature has.
///
/// This type acts as a set of skills, where each skill can either be:
/// - [`SkillLevel::Proficient`]; the creature is proficient in the skill.
/// - [`SkillLevel::Expertise`]; the creature has expertise in the skill.
///
/// # Examples
///
/// ```rust
/// use dnd::core::{Skill, SkillProficiencies, SkillLevel};
///
/// let mut profs = SkillProficiencies::new();
/// profs.set_proficient(Skill::Acrobatics);
/// profs.set_expertise(Skill::Stealth);
///
/// assert!(profs.is_proficient(Skill::Acrobatics));
/// assert!(profs.has_expertise(Skill::Stealth));
/// assert_eq!(profs.get_proficiency(Skill::Acrobatics), Some(SkillLevel::Proficient));
/// assert_eq!(profs.get_proficiency(Skill::Stealth), Some(SkillLevel::Expertise));
///
/// let skills: Vec<_> = profs.iter().collect();
/// assert_eq!(
///    &skills,
///    &[(Skill::Acrobatics, SkillLevel::Proficient), (Skill::Stealth, SkillLevel::Expertise)]
/// );
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillProficiencies {
    proficient: BitFlags<Skill>,
    expertise: BitFlags<Skill>,
}

impl SkillProficiencies {
    /// Creates a new `SkillProficiencies` with no proficiencies.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            proficient: BitFlags::EMPTY,
            expertise: BitFlags::EMPTY,
        }
    }

    /// Creates a new `SkillProficiencies` with the given proficiencies.
    #[must_use]
    pub fn with_proficiencies(proficient: impl Iterator<Item = (Skill, SkillLevel)>) -> Self {
        let mut profs = Self::new();
        profs.set_proficiencies(proficient);
        profs
    }

    /// Returns whether the creature is proficient in the given skill.
    ///
    /// If the creature has expertise in the skill, this will return `false`.
    #[must_use]
    pub fn is_proficient(&self, skill: Skill) -> bool {
        self.proficient.contains(skill)
    }

    /// Returns whether the creature has expertise in the given skill.
    #[must_use]
    pub fn has_expertise(&self, skill: Skill) -> bool {
        self.expertise.contains(skill)
    }

    /// Returns the proficiency level for the given skill.
    ///
    /// If the creature does not have proficiency or expertise in the skill, it returns `None`.
    #[must_use]
    pub fn get_proficiency(&self, skill: Skill) -> Option<SkillLevel> {
        if self.has_expertise(skill) {
            Some(SkillLevel::Expertise)
        } else if self.is_proficient(skill) {
            Some(SkillLevel::Proficient)
        } else {
            None
        }
    }

    /// Sets the proficiency level for the given skill.
    pub fn set_proficiency(&mut self, skill: Skill, proficiency: SkillLevel) -> &mut Self {
        match proficiency {
            SkillLevel::Proficient => {
                self.proficient.insert(skill);
                self.expertise.remove(skill);
            }
            SkillLevel::Expertise => {
                self.expertise.insert(skill);
                self.proficient.remove(skill);
            }
        }
        self
    }

    /// Sets the proficiency level for each of the given skills.
    pub fn set_proficiencies(
        &mut self,
        skills: impl Iterator<Item = (Skill, SkillLevel)>,
    ) -> &mut Self {
        for (skill, proficiency) in skills {
            self.set_proficiency(skill, proficiency);
        }
        self
    }

    /// Sets the proficiency level for the given skill to `Proficient`.
    ///
    /// This will remove any existing expertise for the skill.
    pub fn set_proficient(&mut self, skill: Skill) -> &mut Self {
        self.set_proficiency(skill, SkillLevel::Proficient)
    }

    /// Sets the proficiency level for the given skill to `Expertise`.
    ///
    /// This will remove any existing proficiency for the skill.
    pub fn set_expertise(&mut self, skill: Skill) -> &mut Self {
        self.set_proficiency(skill, SkillLevel::Expertise)
    }

    /// Clears the proficiency for the given skill.
    pub fn clear_proficiency(&mut self, skill: Skill) -> &mut Self {
        self.proficient.remove(skill);
        self.expertise.remove(skill);
        self
    }

    /// Clears all proficiencies.
    pub fn clear_all(&mut self) -> &mut Self {
        self.proficient = BitFlags::EMPTY;
        self.expertise = BitFlags::EMPTY;
        self
    }

    /// Returns an iterator over all proficient skills, including those with expertise.
    pub fn iter(&self) -> impl Iterator<Item = (Skill, SkillLevel)> + '_ {
        Skill::all().iter().filter_map(move |&skill| {
            if self.has_expertise(skill) {
                Some((skill, SkillLevel::Expertise))
            } else if self.is_proficient(skill) {
                Some((skill, SkillLevel::Proficient))
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn new() {
        let profs = SkillProficiencies::new();
        assert!(profs.iter().next().is_none());
    }

    #[test]
    fn set_proficient() {
        let mut profs = SkillProficiencies::new();
        profs.set_proficient(Skill::Acrobatics);

        assert!(profs.is_proficient(Skill::Acrobatics));
        assert!(!profs.has_expertise(Skill::Acrobatics));
        assert_eq!(
            profs.get_proficiency(Skill::Acrobatics),
            Some(SkillLevel::Proficient)
        );

        let skills: Vec<_> = profs.iter().collect();
        assert_eq!(&skills, &[(Skill::Acrobatics, SkillLevel::Proficient)]);
    }

    #[test]
    fn set_expertise() {
        let mut profs = SkillProficiencies::new();
        profs.set_expertise(Skill::Acrobatics);

        assert!(!profs.is_proficient(Skill::Acrobatics));
        assert!(profs.has_expertise(Skill::Acrobatics));
        assert_eq!(
            profs.get_proficiency(Skill::Acrobatics),
            Some(SkillLevel::Expertise)
        );
        let skills: Vec<_> = profs.iter().collect();
        assert_eq!(&skills, &[(Skill::Acrobatics, SkillLevel::Expertise)]);
    }

    #[test]
    fn clear_proficiency() {
        let mut profs = SkillProficiencies::new();
        profs.set_proficient(Skill::Acrobatics);
        profs.clear_proficiency(Skill::Acrobatics);

        assert!(!profs.is_proficient(Skill::Acrobatics));
        assert!(!profs.has_expertise(Skill::Acrobatics));
        assert_eq!(profs.get_proficiency(Skill::Acrobatics), None);
        let skills: Vec<_> = profs.iter().collect();
        assert!(skills.is_empty());
    }

    #[test]
    fn clear_all() {
        let mut profs = SkillProficiencies::new();
        profs.set_proficient(Skill::Acrobatics);
        profs.set_expertise(Skill::Stealth);
        profs.clear_all();

        assert!(!profs.is_proficient(Skill::Acrobatics));
        assert!(!profs.has_expertise(Skill::Stealth));
        assert_eq!(profs.get_proficiency(Skill::Acrobatics), None);
        assert_eq!(profs.get_proficiency(Skill::Stealth), None);
        let skills: Vec<_> = profs.iter().collect();
        assert!(skills.is_empty());
    }

    #[test]
    fn iter() {
        let mut profs = SkillProficiencies::new();
        profs.set_proficient(Skill::Acrobatics);
        profs.set_expertise(Skill::Stealth);

        let skills: Vec<_> = profs.iter().collect();
        assert_eq!(
            &skills,
            &[
                (Skill::Acrobatics, SkillLevel::Proficient),
                (Skill::Stealth, SkillLevel::Expertise)
            ]
        );
    }

    #[test]
    fn with_proficiencies() {
        let profs = SkillProficiencies::with_proficiencies(
            [
                (Skill::Acrobatics, SkillLevel::Proficient),
                (Skill::Stealth, SkillLevel::Expertise),
            ]
            .into_iter(),
        );
        assert!(profs.is_proficient(Skill::Acrobatics));
        assert!(profs.has_expertise(Skill::Stealth));
    }
}
