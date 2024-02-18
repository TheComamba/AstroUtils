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

const SADALSUUD: RealData = RealData {
    common_name: "Sadalsuud",
    astronomical_name: "Beta Aquarii",
    constellation: "Aquarius",
    radius: Some(Distance {
        m: 47.88 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 4.97 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.04,
    apparent_magnitude: 2.87,
    temperature: Some(Temperature { K: 5608. }),
    age: Some(Time {
        s: 0.110 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 31, 34),
    declination: Declination::new(Sgn::Neg, 5, 34, 16),
    distance: Distance {
        m: 550. * LIGHT_YEAR.m,
    },
};

const SADALMELIK: RealData = RealData {
    common_name: "Sadalmelik",
    astronomical_name: "Alpha Aquarii",
    constellation: "Aquarius",
    radius: Some(Distance {
        m: 53.89 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.13 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.882,
    apparent_magnitude: 2.942,
    temperature: Some(Temperature { K: 5383. }),
    age: Some(Time {
        s: 0.053 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 5, 47),
    declination: Declination::new(Sgn::Neg, 0, 19, 11),
    distance: Distance {
        m: 520. * LIGHT_YEAR.m,
    },
};

const SKAT: RealData = RealData {
    common_name: "Skat",
    astronomical_name: "Delta Aquarii",
    constellation: "Aquarius",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.51 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.17,
    apparent_magnitude: 3.28,
    temperature: Some(Temperature { K: 8650. }),
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 54, 39),
    declination: Declination::new(Sgn::Neg, 15, 49, 15),
    distance: Distance {
        m: 113. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [SADALSUUD, SADALMELIK, SKAT];