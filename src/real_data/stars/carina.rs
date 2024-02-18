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

const CANOPUS: RealData = RealData {
    common_name: "Canopus",
    astronomical_name: "Alpha Carinae",
    constellation: "Carina",
    radius: Some(Distance {
        m: 72. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 9. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.53,
    apparent_magnitude: -0.62,
    temperature: Some(Temperature { K: 7400. }),
    age: Some(Time {
        s: 0.0251 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 23, 57),
    declination: Declination::new(Sgn::Neg, 52, 41, 44),
    distance: Distance {
        m: 313. * LIGHT_YEAR.m,
    },
};

const MIAPLACIDUS: RealData = RealData {
    common_name: "Miaplacidus",
    astronomical_name: "Beta Carinae",
    constellation: "Carina",
    radius: Some(Distance {
        m: 6.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.99,
    apparent_magnitude: 1.67,
    temperature: Some(Temperature { K: 8866. }),
    age: Some(Time {
        s: 0.260 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 13, 12),
    declination: Declination::new(Sgn::Neg, 69, 43, 2),
    distance: Distance {
        m: 111. * LIGHT_YEAR.m,
    },
};

const AVIOR: RealData = RealData {
    common_name: "Avior",
    astronomical_name: "Epsilon Carinae",
    constellation: "Carina",
    radius: None,
    mass: Some(Mass {
        kg: 10.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.58,
    apparent_magnitude: 1.86,
    temperature: Some(Temperature { K: 3523. }),
    age: Some(Time {
        s: 0.020 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 22, 31),
    declination: Declination::new(Sgn::Neg, 59, 30, 34),
    distance: Distance {
        m: 632. * LIGHT_YEAR.m,
    },
};

const ASPIDISKE: RealData = RealData {
    common_name: "Aspidiske",
    astronomical_name: "Iota Carinae",
    constellation: "Carina",
    radius: Some(Distance {
        m: 43. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.42,
    apparent_magnitude: 2.21,
    temperature: Some(Temperature { K: 7500. }),
    age: Some(Time {
        s: 0.0374 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 17, 5),
    declination: Declination::new(Sgn::Neg, 59, 16, 30),
    distance: Distance {
        m: 694. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 4] = [CANOPUS, MIAPLACIDUS, AVIOR, ASPIDISKE];