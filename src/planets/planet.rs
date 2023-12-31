use super::orbit_parameters::OrbitParameters;
use crate::{
    color::sRGBColor,
    coordinates::direction::Direction,
    units::{length::Length, mass::Mass, time::Time},
    Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub(super) name: String,
    pub(super) mass: Mass,
    pub(super) radius: Length,
    pub(super) geometric_albedo: Float,
    pub(super) color: sRGBColor,
    pub(super) orbital_parameters: OrbitParameters,
    pub(super) sideral_rotation_period: Time,
    pub(super) rotation_axis: Direction,
}

impl PartialEq for Planet {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Planet {
    pub fn new(
        name: String,
        mass: Mass,
        orbital_parameters: OrbitParameters,
        radius: Length,
        geometric_albedo: Float,
        color: sRGBColor,
        sideral_rotation_period: Time,
        rotation_axis: Direction,
    ) -> Self {
        Planet {
            name,
            mass,
            orbital_parameters,
            radius,
            geometric_albedo,
            color,
            sideral_rotation_period,
            rotation_axis,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_mass(&self) -> Mass {
        self.mass
    }

    pub fn get_radius(&self) -> Length {
        self.radius
    }

    pub fn get_geometric_albedo(&self) -> Float {
        self.geometric_albedo
    }

    pub fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub fn get_orbital_parameters(&self) -> &OrbitParameters {
        &self.orbital_parameters
    }

    pub fn get_sideral_rotation_period(&self) -> Time {
        self.sideral_rotation_period
    }

    pub fn get_rotation_axis(&self) -> &Direction {
        &self.rotation_axis
    }
}
