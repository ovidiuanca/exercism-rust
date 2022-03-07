// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
use std::cmp::Ordering;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        let health: u32;
        let mana: Option<u32>;

        match self.level.cmp(&10) {
            Ordering::Less => {
                health = 100;
                mana = None;
            },
            Ordering::Equal | Ordering::Greater => {
                health = 100;
                mana = Some(100);
            }
        }

        Some(
            Player { health: health, mana: mana, level: self.level }
        )
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
            Some(mana) if mana_cost > mana => 0,
            Some(mana) => {
                self.mana = Some(mana.saturating_sub(mana_cost));
                mana_cost.saturating_mul(2)
            }
        }
    }
}
