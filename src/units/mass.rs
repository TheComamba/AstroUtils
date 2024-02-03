use crate::Float;
use serde::{Deserialize, Serialize};

pub(crate) const KILOGRAMS_PER_MOON_MASS: Float = 7.342e22;
const MOON_MASSES_PER_KILOGRAM: Float = 1. / KILOGRAMS_PER_MOON_MASS;
pub(crate) const KILOGRAMS_PER_EARTH_MASS: Float = 5.972e24;
const EARTH_MASSES_PER_KILOGRAM: Float = 1. / KILOGRAMS_PER_EARTH_MASS;
pub(crate) const KILOGRAMS_PER_JUPITER_MASS: Float = 1.898e27;
const JUPITER_MASSES_PER_KILOGRAM: Float = 1. / KILOGRAMS_PER_JUPITER_MASS;
pub(crate) const KILOGRAMS_PER_SOLAR_MASS: Float = 1.989e30;
const SOLAR_MASSES_PER_KILOGRAM: Float = 1. / KILOGRAMS_PER_SOLAR_MASS;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Mass {
    pub(crate) kilograms: Float,
}

impl Mass {
    pub const ZERO: Mass = Mass { kilograms: 0. };

    pub const fn from_kilograms(kilograms: Float) -> Mass {
        Mass { kilograms }
    }

    pub fn from_moon_masses(moon_masses: Float) -> Mass {
        Mass {
            kilograms: moon_masses * KILOGRAMS_PER_MOON_MASS,
        }
    }

    pub fn from_earth_masses(earth_masses: Float) -> Mass {
        Mass {
            kilograms: earth_masses * KILOGRAMS_PER_EARTH_MASS,
        }
    }

    pub fn from_jupiter_masses(jupiter_masses: Float) -> Mass {
        Mass {
            kilograms: jupiter_masses * KILOGRAMS_PER_JUPITER_MASS,
        }
    }

    pub fn from_solar_masses(solar_masses: Float) -> Mass {
        Mass {
            kilograms: solar_masses * KILOGRAMS_PER_SOLAR_MASS,
        }
    }

    pub const fn as_kilograms(&self) -> Float {
        self.kilograms
    }

    pub fn as_moon_masses(&self) -> Float {
        self.kilograms * MOON_MASSES_PER_KILOGRAM
    }

    pub fn as_earth_masses(&self) -> Float {
        self.kilograms * EARTH_MASSES_PER_KILOGRAM
    }

    pub fn as_jupiter_masses(&self) -> Float {
        self.kilograms * JUPITER_MASSES_PER_KILOGRAM
    }

    pub fn as_solar_masses(&self) -> Float {
        self.kilograms * SOLAR_MASSES_PER_KILOGRAM
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: Mass, accuracy: Mass) -> bool {
        let diff = self.kilograms - other.kilograms;
        diff.abs() <= accuracy.kilograms
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{TEST_ACCURACY, TEST_MASS_ACCURACY};

    use super::*;

    #[test]
    fn test_kilogram() {
        let mass = Mass::from_kilograms(1.);
        assert!((mass.as_kilograms() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_moon_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_MOON_MASS);
        let mass = Mass::from_moon_masses(1.);
        assert!(mass.eq_within(expected, TEST_MASS_ACCURACY));
        assert!((mass.as_moon_masses() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_earth_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_EARTH_MASS);
        let mass = Mass::from_earth_masses(1.);
        assert!(mass.eq_within(expected, TEST_MASS_ACCURACY));
        assert!((mass.as_earth_masses() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_jupiter_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_JUPITER_MASS);
        let mass = Mass::from_jupiter_masses(1.);
        assert!(mass.eq_within(expected, TEST_MASS_ACCURACY));
        assert!((mass.as_jupiter_masses() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_solar_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);
        let mass = Mass::from_solar_masses(1.);
        assert!(mass.eq_within(expected, TEST_MASS_ACCURACY));
        assert!((mass.as_solar_masses() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_addition() {
        let mass1 = Mass::from_kilograms(1.);
        let mass2 = Mass::from_kilograms(2.);
        let expected = Mass::from_kilograms(3.);
        assert!((mass1 + mass2).eq_within(expected, TEST_MASS_ACCURACY));
    }

    #[test]
    fn test_subtraction() {
        let mass1 = Mass::from_kilograms(1.);
        let mass2 = Mass::from_kilograms(2.);
        let expected = Mass::from_kilograms(-1.);
        assert!((mass1 - mass2).eq_within(expected, TEST_MASS_ACCURACY));
    }
}
