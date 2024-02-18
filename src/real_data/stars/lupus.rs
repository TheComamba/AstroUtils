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

const ALPHA_LUPI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lupi",
    constellation: "Lupus",
    radius: None,
    mass: Some(Mass {
        kg: 10.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.83,
    apparent_magnitude: 2.30,
    temperature: Some(Temperature { K: 21_820. }),
    age: Some(Time {
        s: 0.018 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(14, 41, 56),
    declination: Declination::new(Sgn::Neg, 47, 23, 18),
    distance: Distance {
        m: 548. * LIGHT_YEAR.m,
    },
};

const BETA_LUPI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Lupi",
    constellation: "Lupus",
    right_ascension: RightAscension::new(14, 58, 32),
    declination: Declination::new(Sgn::Neg, 43, 8, 2),
    apparent_magnitude: 2.68,
    distance: Distance {
        m: 383. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.3,
    mass: Some(Mass {
        kg: 8.8 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 6.6 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 24_090. }),
    age: Some(Time {
        s: 0.0246 * BILLION_YEARS.s,
    }),
};

const GAMMA_LUPI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Lupi",
    constellation: "Lupus",
    right_ascension: RightAscension::new(15, 35, 8),
    declination: Declination::new(Sgn::Neg, 41, 10, 0),
    apparent_magnitude: 2.77,
    distance: Distance {
        m: 420. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.4,
    mass: Some(Mass {
        kg: 9.5 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 20_900. }),
    age: Some(Time {
        s: 0.0186 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_LUPI, BETA_LUPI, GAMMA_LUPI];