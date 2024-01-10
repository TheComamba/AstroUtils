use crate::{
    color::sRGBColor,
    coordinates::{
        declination::Declination, earth_equatorial::EarthEquatorialCoordinates,
        right_ascension::RightAscension,
    },
    units::{
        length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature, time::Time,
    },
};

use super::star::StarData;

pub struct RealData {
    pub name: &'static str,
    pub mass: Option<Mass>,
    pub radius: Option<Length>,
    pub luminosity: Luminosity,
    pub temperature: Option<Temperature>,
    pub age: Option<Time>,
    pub right_ascension: RightAscension,
    pub declination: Declination,
    pub distance: Length,
}

impl RealData {
    pub fn to_star(&self) -> StarData {
        let ra = self.right_ascension.to_angle();
        let dec = self.declination.to_angle();
        let dir = EarthEquatorialCoordinates::new(ra, dec).to_direction();
        let color = match self.temperature {
            Some(temperature) => sRGBColor::from_temperature(temperature),
            None => sRGBColor::DEFAULT,
        };
        StarData {
            name: self.name.to_string(),
            mass: self.mass,
            radius: self.radius,
            luminosity: self.luminosity,
            temperature: self.temperature,
            color,
            age: self.age,
            distance: self.distance,
            direction_in_ecliptic: dir,
        }
    }
}
