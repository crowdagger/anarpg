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

use ability::Ability;

/// Representation of a character (player or NPC)
#[derive(Debug, Clone)]
pub struct Character {
    /// Strength and constitution
    fight: u16,
    /// Dexterity/agility
    skill: u16,
    /// Intelligence/perception/charisma
    swag: u16,

    abilities: Vec<Box<Ability>>,
}
