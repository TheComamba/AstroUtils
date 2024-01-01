use crate::Float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Luminosity {
    pub(super) magnitude: Float,
}

impl Luminosity {
    pub const fn from_magnitude(magnitude: Float) -> Luminosity {
        Luminosity { magnitude }
    }

    pub const fn get_magnitude(&self) -> Float {
        self.magnitude
    }
}