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

const NAOS: RealData = RealData {
    common_name: "Naos",
    astronomical_name: "Zeta Puppis",
    constellation: "Puppis",
    radius: Some(Distance {
        m: 20. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 56.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.95,
    apparent_magnitude: 2.21,
    temperature: Some(Temperature { K: 40_000. }),
    age: Some(Time {
        s: 0.0032 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 3, 35),
    declination: Declination::new(Sgn::Neg, 40, 0, 12),
    distance: Distance {
        m: 1399. * LIGHT_YEAR.m,
    },
};

const AHADI: RealData = RealData {
    common_name: "Ahadi",
    astronomical_name: "Pi Puppis",
    constellation: "Puppis",
    radius: Some(Distance {
        m: 235. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 11.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.92,
    apparent_magnitude: 2.71,
    temperature: Some(Temperature { K: 4000. }),
    age: Some(Time {
        s: 0.02 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 17, 9),
    declination: Declination::new(Sgn::Neg, 37, 5, 51),
    distance: Distance {
        m: 1094. * LIGHT_YEAR.m,
    },
};

const RHO_PUPPIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Rho Puppis",
    constellation: "Puppis",
    right_ascension: RightAscension::new(8, 7, 33),
    declination: Declination::new(Sgn::Neg, 24, 18, 16),
    apparent_magnitude: 2.78,
    distance: Distance {
        m: 63.5 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.41,
    mass: Some(Mass {
        kg: 1.85 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.41 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6920. }),
    age: Some(Time {
        s: 2. * BILLION_YEARS.s,
    }),
};

const TAU_PUPPIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Tau Puppis",
    constellation: "Puppis",
    right_ascension: RightAscension::new(6, 49, 56),
    declination: Declination::new(Sgn::Neg, 50, 36, 52),
    apparent_magnitude: 2.95,
    distance: Distance {
        m: 174. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.80,
    mass: Some(Mass {
        kg: 3.19 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 27. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4489. }),
    age: Some(Time {
        s: 0.540 * BILLION_YEARS.s,
    }),
};

const NU_PUPPIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Nu Puppis",
    constellation: "Puppis",
    right_ascension: RightAscension::new(6, 37, 46),
    declination: Declination::new(Sgn::Neg, 43, 11, 45),
    apparent_magnitude: 3.173,
    distance: Distance {
        m: 370. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.11,
    mass: None,
    radius: Some(Distance {
        m: 4.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 12_120. }),
    age: None,
};

pub(crate) const STARS: [RealData; 5] = [NAOS, AHADI, RHO_PUPPIS, TAU_PUPPIS, NU_PUPPIS];