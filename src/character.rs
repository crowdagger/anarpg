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
use stats::Stats;

use std::convert::Into;

/// Representation of a character (player or NPC)
#[derive(Debug, Clone)]
pub struct Character {
    /// Name of the character (yeah this is mandatory even for NPCs)
    name: String,

    /// Class of the character. Cna be multiple strings, e.g. "Young human ranger":
    /// to determine if a character is part of a class, it uses a lowcases version and
    /// look at the string it contains.
    class: String,
    
    /// Character stats
    stats: Stats,

    abilities: Vec<Box<Ability>>,
}


impl Character {
    /// Creates a new level 1 character
    pub fn new<S1, S2>(name: S1, class: S2) -> Character
        where S1: Into<String>,
              S2: Into<String> {
        let name = name.into();
        let class = class.into();
        debug!("Creating a {class} character named {name}",
               name = name,
               class = class);
        Character {
            name: name,
            class: class,
            stats: Stats::new(),
            abilities: vec![],
        }
    }

    /// Returns true if the character is of a given class
    pub fn is_a(&self, class: &str) -> bool {
        for s in self.class
            .to_lowercase()
            .split(|c: char| match c {
                ',' | ';' => true,
                c if c.is_whitespace() => true,
                _ => false
            }) {
                if s == &class.to_lowercase() {
                    return true;
                }
            }
        false
    }
}


#[test]
fn class() {
    let foo = Character::new("Foo", "Demon ranger");
    assert_eq!(foo.is_a("demon"), true);
    assert_eq!(foo.is_a("Demon"), true);
    assert_eq!(foo.is_a("ranger"), true);
    assert_eq!(foo.is_a("Ranger"), true);
    assert_eq!(foo.is_a("human"), false);

    let bar = Character::new("Foo", "Human, cop, man");
    assert_eq!(bar.is_a("human"), true);
    assert_eq!(bar.is_a("COP"), true);
    assert_eq!(bar.is_a("man"), true);
    assert_eq!(bar.is_a("woman"), false);

    let woman = Character::new("Foo", "Human, cop, woman");
    assert_eq!(woman.is_a("human"), true);
    assert_eq!(woman.is_a("COP"), true);
    assert_eq!(woman.is_a("man"), false);
    assert_eq!(woman.is_a("woman"), true);
}
