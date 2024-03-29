use std::f64::consts::PI;

use crate::astro_display::AstroDisplay;
use simple_si_units::{
    base::Distance,
    geometry::{Area, SolidAngle},
};

pub const SOLID_ANGLE_ZERO: SolidAngle<f64> = SolidAngle { sr: 0.0 };

impl AstroDisplay for SolidAngle<f64> {
    fn astro_display(&self) -> String {
        format!("{:.2} sr", self.sr)
    }
}

pub fn area_and_distance_to_solid_angle(
    surface_area: Area<f64>,
    distance: Distance<f64>,
) -> SolidAngle<f64> {
    SolidAngle {
        sr: surface_area / (distance * distance),
    }
}

pub fn radius_and_distance_to_solid_angle(
    radius: Distance<f64>,
    distance: Distance<f64>,
) -> SolidAngle<f64> {
    area_and_distance_to_solid_angle(PI * radius * radius, distance)
}

pub fn solid_angle_to_area_at_distance(
    solid_angle: SolidAngle<f64>,
    distance: Distance<f64>,
) -> Area<f64> {
    solid_angle.sr * (distance * distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        real_data::planets::{EARTH, MOON},
        tests::eq,
        units::distance::SOLAR_RADIUS,
    };

    #[test]
    fn test_area_and_distance_to_solid_angle() {
        let surface_area = Area { m2: 1.0 };
        let distance = Distance { m: 1.0 };
        let solid_angle = area_and_distance_to_solid_angle(surface_area, distance);
        assert!(eq(solid_angle.sr, 1.0));
    }

    #[test]
    fn test_solid_angle_to_area_at_distance() {
        let solid_angle = SolidAngle { sr: 1.0 };
        let distance = Distance { m: 1.0 };
        let surface_area = solid_angle_to_area_at_distance(solid_angle, distance);
        assert!(eq(surface_area.m2, 1.0));
    }

    #[test]
    fn solid_angle_of_sun() {
        let expected = SolidAngle { sr: 7e-5 };
        let actual =
            radius_and_distance_to_solid_angle(SOLAR_RADIUS, EARTH.orbit.get_semi_major_axis());
        assert!(eq(actual.sr, expected.sr));
    }

    #[test]
    fn solid_angle_of_full_moon() {
        let expected = SolidAngle { sr: 6.4e-5 };
        let actual =
            radius_and_distance_to_solid_angle(MOON.radius, MOON.orbit.get_semi_major_axis());
        assert!(eq(actual.sr, expected.sr));
    }
}
