use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

const ANKAA: RealData = RealData {
    common_name: "Ankaa",
    astronomical_name: "Alpha Phoenicis",
    constellation: "Phoenix",
    radius: Some(Distance {
        m: 15. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.57 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.52,
    apparent_magnitude: 2.4,
    temperature: Some(Temperature { K: 4436. }),
    age: None,
    right_ascension: RightAscension::new(0, 26, 17),
    declination: Declination::new(Sgn::Neg, 42, 18, 21),
    distance: Distance {
        m: 77. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
