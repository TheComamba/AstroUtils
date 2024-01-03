use crate::{
    coordinates::cartesian::CartesianCoordinates,
    units::{illuminance::Illuminance, length::Length, luminosity::Luminosity},
    Float,
};

pub fn planet_brightness(
    star_luminosity: Luminosity,
    star_position: &CartesianCoordinates,
    planet_position: &CartesianCoordinates,
    observer_position: &CartesianCoordinates,
    planet_radius: Length,
    planet_albedo: Float,
) -> Illuminance {
    let distance_star_to_planet = (planet_position - star_position).length();
    let distance_planet_to_observer = (planet_position - observer_position).length();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        coordinates::cartesian::CartesianCoordinates, data::planets::*, units::length::Length,
    };

    const REAL_DATA_TEST_ACCURACY: f32 = 0.05;

    #[test]
    fn apparent_magnitude_of_jupiter_at_opposition() {
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(JUPITER_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let expected = -2.94;
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            JUPITER_RADIUS,
            JUPITER_BOND_ALBEDO,
        )
        .get_apparent_magnitude();
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY);
    }

    #[test]
    fn apparent_magnitude_of_venus_at_greatest_elongation() {
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(Length::ZERO, VENUS_SEMI_MAJOR_AXIS, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let expected = -4.92;
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            VENUS_RADIUS,
            VENUS_BOND_ALBEDO,
        )
        .get_apparent_magnitude();
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY);
    }
}
