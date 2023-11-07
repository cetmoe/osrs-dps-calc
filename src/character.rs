use crate::Equipment;

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
