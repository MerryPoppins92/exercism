// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // unimplemented!("Revive this player")
        if self.health > 0 {
            return None;
        }
        if self.level < 10 {
            let pl:Player = Player {
                health: 100,
                mana: None,
                level: self.level,
            };
            return Some(pl);
        }
        let pl:Player = Player {
            health: 100,
            mana: Some(100),
            level: self.level,
        };
        Some(pl)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {}", mana_cost)
        match self.mana {
            Some(mana) => if mana > mana_cost {
                self.mana = Some(mana - mana_cost)
            } else { 
                return 0
            },
            None => if self.health > mana_cost {
            self.health = self.health - mana_cost;
            return 0;
        } else {
            self.health = 0;
            return 0;
        }, 
        };
        // Ok(self.mana) - mana_cost;
        2 * mana_cost
    }
}
