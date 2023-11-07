use crate::Equipment;

#[derive(Copy, Clone)]
pub struct Character {
    pub attack: u8,
    pub strength: u8,
    pub defence: u8,
    pub prayer: u8,
    pub ranged: u8,
    pub magic: u8,
    pub hitpoints: u8,
    pub equipment: Equipment,
}

impl Character {
    pub fn get_skill(&self, skill: &Skill) -> u8 {
        match skill {
            Skill::Attack => self.attack,
            Skill::Strength => self.strength,
            Skill::Defence => self.defence,
            Skill::Prayer => self.prayer,
            Skill::Ranged => self.ranged,
            Skill::Magic => self.magic,
        }
    }
}

pub enum Skill {
    Attack,
    Strength,
    Defence,
    Prayer,
    Ranged,
    Magic,
}
