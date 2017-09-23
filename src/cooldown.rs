// (C) 2017, Élisabeth Henry
//
// Licensed under either of
// 
// Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
// MIT license: http://opensource.org/licenses/MIT
// at your option.
//
// Unless you explicitly state otherwise, any contribution intentionally submitted
// for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
// dual licensed as above, without any additional terms or conditions.

use stats::Stats;

/// The cooldown of an ability. Can be a mix of a fixed cooldown, and cooldown that can be
/// reduced from character's stats or its cooldown reduction.
///
/// # Example
///
/// ```
/// use anarpg::Cooldown;
/// use anarpg::Stats;
///
/// // Create a new Cooldown 
/// let cooldown = Cooldown::new()
///     .with_fixed(0.5)
///     .with_variable(1.0)
///     .with_badassness(1.0);
///
/// // Compute the cooldown for default stats
/// println!("cooldown: {}", cooldown.adjusted_by(&Stats::new()));
/// ```
#[derive(Debug, Clone)]
pub struct Cooldown {
    /// The fixed part. Won't be reduced by anything.
    /// E.g. the time a gun takes to eject a bullet and be ready to fire again.
    pub fixed: f32,
    /// Variable cooldown, reduced by a character's cooldown reduction stat.
    pub variable: f32,
    /// Cooldown reduced by having more badassness.
    /// E.g. the time it takes to be ready to punch again.
    pub badassness: f32,
    /// Cooldown reduced by having more skill.
    /// E.g. the time it takes to aim again.
    pub skill: f32,
    /// Cooldown reduced by having more swag.
    /// E.g. the time it takes to be able to cast another spell.
    pub swag: f32,
}

impl Cooldown {
    /// Creates a new, zero cooldown
    pub fn new() -> Cooldown {
        Cooldown {
            fixed: 0.0,
            variable: 0.0,
            badassness: 0.0,
            skill: 0.0,
            swag: 0.0,
        }
    }

    /// Sets the fixed part of a cooldown
    pub fn with_fixed(mut self, fixed: f32) -> Cooldown {
        self.fixed = fixed;
        self
    }

    /// Sets the variable part of a cooldown
    pub fn with_variable(mut self, variable: f32) -> Cooldown {
        self.variable = variable;
        self
    }

    /// Sets the badassness part of a cooldown
    pub fn with_badassness(mut self, badassness: f32) -> Cooldown {
        self.badassness = badassness;
        self
    }

    /// Sets the skill part of a cooldown
    pub fn with_skill(mut self, skill: f32) -> Cooldown {
        self.skill = skill;
        self
    }

    /// Sets the swag part of a cooldown
    pub fn with_swag(mut self, swag: f32) -> Cooldown {
        self.swag = swag;
        self
    }

    /// Compute the cooldown after character's stats reduction
    pub fn adjusted_by(&self, stats: &Stats) -> f32 {
        self.fixed
            + self.variable * stats.cooldown_reduction
            + self.badassness * f(stats.badassness)
            + self.skill * f(stats.skill)
            + self.swag * f(stats.swag)
    }
}

/// Convert a stat's value to a cooldown reduction value
fn f(x: i16) -> f32 {
    f32::from(0.95).powi(x as i32)
}
