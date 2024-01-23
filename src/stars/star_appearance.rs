use crate::{
    color::sRGBColor, coordinates::direction::Direction, units::illuminance::Illuminance, Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAppearance {
    pub(crate) name: String,
    pub(crate) illuminance: Illuminance,
    pub(crate) color: sRGBColor,
    pub(crate) direction_in_ecliptic: Direction,
}

impl StarAppearance {
    pub fn new(
        name: String,
        illuminance: Illuminance,
        color: sRGBColor,
        direction_in_ecliptic: Direction,
    ) -> Self {
        Self {
            name,
            illuminance,
            color,
            direction_in_ecliptic,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_illuminance(&self) -> &Illuminance {
        &self.illuminance
    }

    pub const fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub const fn get_direction_in_ecliptic(&self) -> &Direction {
        &self.direction_in_ecliptic
    }

    pub fn set_direction_in_ecliptic(&mut self, direction: Direction) {
        self.direction_in_ecliptic = direction;
    }

    pub(super) fn apparently_the_same(&self, other: &Self) -> bool {
        const DIRECTION_ACCURACY: Float = 1. / 24. / 60. / 60.;

        if !self
            .direction_in_ecliptic
            .eq_within(&other.direction_in_ecliptic, DIRECTION_ACCURACY)
        {
            return false;
        }
        let illuminance_ratio = self.illuminance.as_lux() / other.illuminance.as_lux();
        if illuminance_ratio < 0.5 || illuminance_ratio > 5.0 {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn star_is_apparently_the_same_with_itself() {
        use super::*;
        use crate::{
            color::sRGBColor, coordinates::direction::Direction, units::illuminance::Illuminance,
        };

        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::from_lux(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            Direction::X,
        );

        assert!(star.apparently_the_same(&star));
    }

    #[test]
    fn star_is_not_the_same_if_direction_is_too_different() {
        use super::*;
        use crate::{
            color::sRGBColor, coordinates::direction::Direction, units::illuminance::Illuminance,
        };

        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::from_lux(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            Direction::X,
        );
        let mut other = star.clone();
        other.direction_in_ecliptic = Direction::Y;

        assert!(!star.apparently_the_same(&other));
    }

    #[test]
    fn star_is_not_the_same_if_brightness_is_too_different() {
        use super::*;
        use crate::{
            color::sRGBColor, coordinates::direction::Direction, units::illuminance::Illuminance,
        };

        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::from_lux(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            Direction::X,
        );
        let mut other = star.clone();
        other.illuminance = Illuminance::from_lux(100.0);

        assert!(!star.apparently_the_same(&other));
    }
}
