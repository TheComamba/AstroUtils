use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

use crate::units::{
    distance::DISTANCE_ZERO, luminous_intensity::LUMINOSITY_ZERO, mass::MASS_ZERO,
    temperature::TEMPERATURE_ZERO, time::TIME_ZERO,
};

use super::{data::StarData, fate::StarFate};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarDataEvolution {
    lifestage_evolution: Option<StarDataLifestageEvolution>,
    pub(super) age: Option<Time<f64>>,
    pub(super) lifetime: Time<f64>,
    pub(super) fate: StarFate,
}

impl StarDataEvolution {
    pub const NONE: StarDataEvolution = StarDataEvolution {
        lifestage_evolution: None,
        age: None,
        lifetime: TIME_ZERO,
        fate: StarFate::WhiteDwarf,
    };

    pub(crate) fn new(
        lifestage_evolution: Option<StarDataLifestageEvolution>,
        age: Option<Time<f64>>,
        lifetime: Time<f64>,
        fate: StarFate,
    ) -> Self {
        Self {
            lifestage_evolution,
            age,
            lifetime,
            fate,
        }
    }

    pub(super) fn has_changed(&self, then: Time<f64>, now: Time<f64>) -> bool {
        const EVOLUTION_TIMESCALE: Time<f64> = Time {
            s: 1_000. * 365.25 * 24. * 60. * 60.,
        }; // 1_000 years
        const DEATH_TIMESCALE: Time<f64> = Time {
            s: 365.25 * 24. * 60. * 60.,
        }; // 1 year

        if let Some(time_until_death) = self.time_until_death(now) {
            if time_until_death < TIME_ZERO && time_until_death.s.abs() < DEATH_TIMESCALE.s {
                return true;
            }
        }

        let diff = Time {
            s: (then.s - now.s).abs(),
        };
        if self.lifestage_evolution.is_some() && diff > EVOLUTION_TIMESCALE {
            return true;
        }
        false
    }

    pub(super) fn time_until_death(&self, time_since_epoch: Time<f64>) -> Option<Time<f64>> {
        self.age.map(|age| self.lifetime - age - time_since_epoch)
    }

    pub(crate) fn apply_to_mass(&self, mass: Mass<f64>, time_since_epoch: Time<f64>) -> Mass<f64> {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death < TIME_ZERO {
                return self.fate.apply_to_mass(mass, -time_until_death);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return mass + lifestage_evolution.mass_per_year * time_since_epoch.to_yr();
        }
        mass
    }

    pub(crate) fn apply_to_radius(
        &self,
        radius: Distance<f64>,
        time_since_epoch: Time<f64>,
    ) -> Distance<f64> {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death < TIME_ZERO {
                return self.fate.apply_to_radius(radius, -time_until_death);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return radius + lifestage_evolution.radius_per_year * time_since_epoch.to_yr();
        }
        radius
    }

    pub(crate) fn apply_to_luminous_intensity(
        &self,
        luminous_intensity: Luminosity<f64>,
        time_since_epoch: Time<f64>,
    ) -> Luminosity<f64> {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death < TIME_ZERO {
                return self
                    .fate
                    .apply_to_luminous_intensity(luminous_intensity, -time_until_death);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return luminous_intensity
                + lifestage_evolution.luminous_intensity_per_year * time_since_epoch.to_yr();
        }
        luminous_intensity
    }

    pub(crate) fn apply_to_temperature(
        &self,
        temperature: Temperature<f64>,
        time_since_epoch: Time<f64>,
    ) -> Temperature<f64> {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death < TIME_ZERO {
                return self
                    .fate
                    .apply_to_temperature(temperature, -time_until_death);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return temperature
                + lifestage_evolution.temperature_per_year * time_since_epoch.to_yr();
        }
        temperature
    }

    pub fn get_lifestage_mass_per_year(&self) -> Mass<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.mass_per_year)
            .unwrap_or(MASS_ZERO)
    }

    pub fn get_lifestage_radius_per_year(&self) -> Distance<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.radius_per_year)
            .unwrap_or(DISTANCE_ZERO)
    }

    pub fn get_lifestage_luminous_intensity_per_year(&self) -> Luminosity<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.luminous_intensity_per_year)
            .unwrap_or(LUMINOSITY_ZERO)
    }

    pub fn get_lifestage_temperature_per_year(&self) -> Temperature<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.temperature_per_year)
            .unwrap_or(TEMPERATURE_ZERO)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct StarDataLifestageEvolution {
    mass_per_year: Mass<f64>,
    radius_per_year: Distance<f64>,
    luminous_intensity_per_year: Luminosity<f64>,
    temperature_per_year: Temperature<f64>,
}

impl StarDataLifestageEvolution {
    pub(crate) fn new(now: &StarData, then: &StarData, years: f64) -> Self {
        let mass_per_year = match (now.mass, then.mass) {
            (Some(now_mass), Some(then_mass)) => (now_mass - then_mass) / years,
            _ => MASS_ZERO,
        };
        let radius_per_year = match (now.radius, then.radius) {
            (Some(now_radius), Some(then_radius)) => (now_radius - then_radius) / years,
            _ => DISTANCE_ZERO,
        };
        let luminous_intensity_per_year = match (now.luminous_intensity, then.luminous_intensity) {
            (Some(now_luminous_intensity), Some(then_luminous_intensity)) => {
                (now_luminous_intensity - then_luminous_intensity) / years
            }
            _ => LUMINOSITY_ZERO,
        };
        let temperature_per_year = (now.temperature - then.temperature) / years;
        Self {
            mass_per_year,
            radius_per_year,
            luminous_intensity_per_year,
            temperature_per_year,
        }
    }
}
