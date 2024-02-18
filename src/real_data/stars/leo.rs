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

const REGULUS: RealData = RealData {
    common_name: "Regulus",
    astronomical_name: "Alpha Leonis",
    constellation: "Leo",
    radius: Some(Distance {
        m: 4.35 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.52,
    apparent_magnitude: 1.36,
    temperature: Some(Temperature { K: 11_668. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(10, 8, 22),
    declination: Declination::new(Sgn::Pos, 11, 58, 2),
    distance: Distance {
        m: 77. * LIGHT_YEAR.m,
    },
};

const ALGIEBA: RealData = RealData {
    common_name: "Algieba",
    astronomical_name: "Gamma Leonis",
    constellation: "Leo",
    radius: Some(Distance {
        m: 31.88 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.23 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.92,
    apparent_magnitude: 2.01,
    temperature: Some(Temperature { K: 4470. }),
    age: None,
    right_ascension: RightAscension::new(10, 19, 58),
    declination: Declination::new(Sgn::Pos, 19, 50, 29),
    distance: Distance {
        m: 126. * LIGHT_YEAR.m,
    },
};

const DENEBOLA: RealData = RealData {
    common_name: "Denebola",
    astronomical_name: "Beta Leonis",
    constellation: "Leo",
    radius: Some(Distance {
        m: 1.728 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.78 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.92,
    apparent_magnitude: 2.14,
    temperature: Some(Temperature { K: 8500. }),
    age: Some(Time {
        s: 0.25 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 49, 3),
    declination: Declination::new(Sgn::Pos, 14, 34, 19),
    distance: Distance {
        m: 36. * LIGHT_YEAR.m,
    },
};

const ZOSMA: RealData = RealData {
    common_name: "Zosma",
    astronomical_name: "Delta Leonis",
    constellation: "Leo",
    radius: Some(Distance {
        m: 2.14 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.32,
    apparent_magnitude: 2.56,
    temperature: Some(Temperature { K: 8_296. }),
    age: Some(Time {
        s: 0.65 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 14, 7),
    declination: Declination::new(Sgn::Pos, 20, 31, 25),
    distance: Distance {
        m: 58. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 4] = [REGULUS, ALGIEBA, DENEBOLA, ZOSMA];