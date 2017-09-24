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

use std::fmt::Debug;

use stats::Stats;
use time::Time;


/// Necessary for some quirks I don't quite understand really.
///
/// See https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-trait-object
/// You don't have to worry about this trait if you create a new ability, it will be
/// implemented automatically.
pub trait AbilityClone {
    fn clone_box(&self) -> Box<Ability>;
}

impl<T> AbilityClone for T
    where T: 'static + Ability + Clone {
    fn clone_box(&self) -> Box<Ability> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Ability> {
    fn clone(&self) -> Box<Ability> {
        self.clone_box()
    }
}

/// A character ability.
///
/// To create a new ability, simply create a new struct and implement the
/// required functions.
///
/// You don't have to worry about `AbilityClone`, it is implemented automatically.
pub trait Ability: Debug+AbilityClone {
    /// Returns the name of the ability
    ///
    ///. Used to determine the character's level
    /// for this ability (or zero if they don't have the ability at all).
    ///
    /// The emplementation should return a unique, lowercase string. 
    fn name(&self) -> &str;
    
    /// Adjust the character stats by some factor. Default: no modification.
    fn stats_modifier(&self, _stats: &mut Stats) { }

    /// Cooldown time for this ability. Default: no cooldown
    fn cooldown(&self) -> Time { Time::new() }

    /// Time the ability takes to perform. Default: no time at all
    fn time(&self) -> Time { Time::new() }
}
