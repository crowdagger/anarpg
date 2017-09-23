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

/// Represents the various statistics of a character that are used for performing actions.
#[derive(Debug, Clone)]
pub struct Stats {
    /// More or less corresponds to strength and constitution. Used when hitting, taking
    /// hits and this kind of stuff.
    pub badassness: i16,
    /// More or less corresponds to dexterity/agility. Used for range attacks, dodging, and
    /// stuff involving precision.
    pub skill: i16,
    /// More or less corresponds to intelligence/charisma. Used for psychic attacks, resisng
    /// magic attacks, and the parts where you must look intelligent to other people.
    pub swag: i16,

    /// Hit points. If you have none, you die.
    pub hp: i16,
    /// Maximum hit points.
    pub max_hp: i16,

    /// Magic points, mana, energy, or whatever you want to call it.
    pub mp: i16,
    /// Maximum magic points
    pub max_mp: i16,

    /// A multiplier to the time it takes to perform actions.
    /// Default is 1.0; 0.5 means character's action will take half time.
    pub speed: f32,
    /// Cooldown reduction. Similar to speed, but reducing abilities cooldown instead of action's time.
    pub cooldown_reduction: f32,
}


impl Stats {
    /// Create a new `Stats` representation with inital level ("level 1")
    pub fn new() -> Stats {
        Stats {
            badassness: 2,
            skill: 2,
            swag: 2,

            hp: 6,
            max_hp: 6,

            mp: 6,
            max_mp: 6,

            speed: 1.0,
            cooldown_reduction: 1.0,
        }
    }
}

/// The number of points needed to go from 0.5 probability to 0.75
const HALF: f32 = 5.0;
lazy_static! {
    static ref LAMBDA: f32 = 0.3333333333333333333f32.powf(1.0/HALF);
}

/// Returns the probability of an action to succed if the attribute corresponds to value.
///
/// The difficulty is the point required to have 50% chance of succeding. 
pub fn probability(value: i16, difficulty: i16) -> f32 {
    let adjusted = value as i32 - difficulty as i32;
    1.0 / (1.0 + LAMBDA.powi(adjusted))
} 


#[test]
fn test_proba() {
    let epsilon = 0.001;
    let p = probability(2, 2);
    assert_eq!(p, 0.5);

    let p = probability(20, 20);
    assert_eq!(p, 0.5);

    let p = probability(5 + HALF as i16, 5);
    assert!(p > 0.75 - epsilon && p < 0.75 + epsilon);

    let p = probability(10 - HALF as i16, 10);
    assert!(p > 0.25 - epsilon && p < 0.25 + epsilon);
}
