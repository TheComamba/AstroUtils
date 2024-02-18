use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature};

const ALPHA_SCUTI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Scuti",
    constellation: "Scutum",
    right_ascension: RightAscension::new(18, 35, 12),
    declination: Declination::new(Sgn::Neg, 8, 14, 39),
    apparent_magnitude: 3.83,
    distance: Distance {
        m: 199. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.08,
    mass: Some(Mass {
        kg: 1.33 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 20. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4315. }),
    age: None,
};

const BETA_SCUTI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Scuti",
    constellation: "Scutum",
    right_ascension: RightAscension::new(18, 47, 10),
    declination: Declination::new(Sgn::Neg, 4, 44, 52),
    apparent_magnitude: 4.22,
    distance: Distance {
        m: 900. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.99,
    mass: None,
    radius: None,
    temperature: Some(Temperature { K: 4622. }),
    age: None,
};

const ZETA_SCUTI: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeata Scuti",
    constellation: "Scutum",
    right_ascension: RightAscension::new(18, 23, 40),
    declination: Declination::new(Sgn::Neg, 8, 56, 4),
    apparent_magnitude: 4.66,
    distance: Distance {
        m: 210. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.66,
    mass: Some(Mass {
        kg: 1.29 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9.3 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4750. }),
    age: None,
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_SCUTI, BETA_SCUTI, ZETA_SCUTI];