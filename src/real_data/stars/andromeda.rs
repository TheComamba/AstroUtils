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

const ALPHERATZ: RealData = RealData {
    common_name: "Alpheratz",
    astronomical_name: "Alpha Andromedae",
    constellation: "Andromeda",
    radius: Some(Distance {
        m: 2.7 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.30,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 13_800. }),
    age: Some(Time {
        s: 0.06 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 8, 23),
    declination: Declination::new(Sgn::Pos, 29, 5, 26),
    distance: Distance {
        m: 97.0 * LIGHT_YEAR.m,
    },
};

const MIRACH: RealData = RealData {
    common_name: "Mirach",
    astronomical_name: "Beta Andromedae",
    constellation: "Andromeda",
    radius: Some(Distance {
        m: 100. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.49 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.86,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 3842. }),
    age: None,
    right_ascension: RightAscension::new(1, 9, 44),
    declination: Declination::new(Sgn::Pos, 35, 37, 14),
    distance: Distance {
        m: 199. * LIGHT_YEAR.m,
    },
};

const ALMACH: RealData = RealData {
    common_name: "Almach",
    astronomical_name: "Gamma Andromedae",
    constellation: "Andromeda",
    radius: Some(Distance {
        m: 80. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 23.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.08,
    apparent_magnitude: 2.1,
    temperature: Some(Temperature { K: 4250. }),
    age: Some(Time {
        s: 0.0065 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(2, 3, 54),
    declination: Declination::new(Sgn::Pos, 42, 19, 47),
    distance: Distance {
        m: 355. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHERATZ, MIRACH, ALMACH];
