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
        if self.health > 0 {
            return None; // Not dead
        }

        let mana = if self.level >= 10 { Some(100) } else { None };

        Some(Player {
            health: 100,
            mana: mana,
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => match mana.checked_sub(mana_cost) {
                None => 0,
                remaining_mana => {
                    self.mana = remaining_mana;
                    mana_cost.saturating_mul(2)
                }
            },
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}
