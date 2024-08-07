use bevy::prelude::*;

/// Bevy [`Component`] for linear and rotational velocity
#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub linear:  Vec3,
    pub angular: Vec3,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            linear:  Vec3::ZERO,
            angular: Vec3::ZERO,
        }
    }
}

impl Velocity {
    /// Make a zero velocity
    pub const ZERO: Self = Self {
        linear:  Vec3::ZERO,
        angular: Vec3::ZERO,
    };

    /// Normalize the linear velocity
    pub fn normalize(&mut self) -> &mut Self {
        self.linear = self.linear.normalize_or_zero();
        self
    }
}

/// Bevy [`Component`] for linear and rotational acceleration
#[derive(Component, Debug, Clone)]
pub struct Acceleration {
    pub linear:  Vec3,
    pub angular: Vec3,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self {
            linear:  Vec3::ZERO,
            angular: Vec3::ZERO,
        }
    }
}

impl Acceleration {
    /// Make a zero acceleration
    pub const ZERO: Self = Self {
        linear:  Vec3::ZERO,
        angular: Vec3::ZERO,
    };

    /// Normalize the linear acceleration
    pub fn normalize(&mut self) -> &mut Self {
        self.linear = self.linear.normalize_or_zero();
        self
    }
}

/// Bevy Movement [`Component`] Bundle including [`Velocity`] and
/// [`Acceleration`]
#[derive(Bundle)]
pub struct MovementBundle {
    pub velocity:     Velocity,
    pub acceleration: Acceleration,
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self {
            velocity:     Velocity::default(),
            acceleration: Acceleration::default(),
        }
    }
}
