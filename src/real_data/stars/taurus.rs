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

const ALDEBARAN: RealData = RealData {
    common_name: "Aldebaran",
    astronomical_name: "Alpha Tauri",
    constellation: "Taurus",
    radius: Some(Distance {
        m: 45.1 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.16 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.63,
    apparent_magnitude: 0.87,
    temperature: Some(Temperature { K: 3900. }),
    age: Some(Time {
        s: 6.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 35, 55),
    declination: Declination::new(Sgn::Pos, 16, 30, 33),
    distance: Distance {
        m: 65. * LIGHT_YEAR.m,
    },
};

const ALNATH: RealData = RealData {
    common_name: "Alnath",
    astronomical_name: "Beta Tauri",
    constellation: "Taurus",
    radius: Some(Distance {
        m: 4.2 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.0 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.37,
    apparent_magnitude: 1.65,
    temperature: Some(Temperature { K: 13_824. }),
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 26, 18),
    declination: Declination::new(Sgn::Pos, 28, 36, 27),
    distance: Distance {
        m: 131. * LIGHT_YEAR.m,
    },
};

const GAMMA_TAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Tauri",
    constellation: "Taurus",
    right_ascension: RightAscension::new(4, 19, 48),
    declination: Declination::new(Sgn::Pos, 15, 37, 40),
    apparent_magnitude: 3.654,
    distance: Distance {
        m: 154. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.22,
    mass: Some(Mass {
        kg: 2.7 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 13.4 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4844. }),
    age: Some(Time {
        s: 0.5 * BILLION_YEARS.s,
    }),
};

const EPSILON_TAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Epislon Tauri",
    constellation: "Taurus",
    right_ascension: RightAscension::new(4, 28, 37),
    declination: Declination::new(Sgn::Pos, 19, 10, 50),
    apparent_magnitude: 3.53,
    distance: Distance {
        m: 146. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.145,
    mass: Some(Mass {
        kg: 2.57 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 12.35 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4950. }),
    age: Some(Time {
        s: 0.625 * BILLION_YEARS.s,
    }),
};

const LAMBDA_TAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Lambda Tauri",
    constellation: "Taurus",
    right_ascension: RightAscension::new(4, 0, 41),
    declination: Declination::new(Sgn::Pos, 12, 29, 25),
    apparent_magnitude: 3.37,
    distance: Distance {
        m: 480. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.45,
    mass: Some(Mass {
        kg: 7.18 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 6.4 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 18_700. }),
    age: Some(Time {
        s: 0.0332 * BILLION_YEARS.s,
    }),
};

const ZETA_TAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Tauri",
    constellation: "Taurus",
    right_ascension: RightAscension::new(5, 37, 39),
    declination: Declination::new(Sgn::Pos, 21, 8, 33),
    apparent_magnitude: 3.010,
    distance: Distance {
        m: 440. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.67,
    mass: Some(Mass {
        kg: 11.2 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 5.5 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 15_500. }),
    age: Some(Time {
        s: 22.5 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 6] = [
    ALDEBARAN,
    ALNATH,
    GAMMA_TAURI,
    EPSILON_TAURI,
    LAMBDA_TAURI,
    ZETA_TAURI,
];