pub mod character;
pub mod math;

pub struct Equipment {
    pub ammo: EquipmentPiece,
    pub helmet: EquipmentPiece,
    pub neck: EquipmentPiece,
    pub cloak: EquipmentPiece,
    pub body: EquipmentPiece,
    pub hands: EquipmentPiece,
    pub ring: EquipmentPiece,
    pub legs: EquipmentPiece,
    pub feet: EquipmentPiece,
    pub weapon: EquipmentPiece,
    pub shield: EquipmentPiece,
}

pub struct EquipmentPiece {
    pub stab_attack: u32,
    pub slash_attack: u32,
    pub crush_attack: u32,
    pub magic_attack: u32,
    pub ranged_attack: u32,
    pub stab_defence: u32,
    pub slash_defence: u32,
    pub crush_defence: u32,
    pub magic_defence: u32,
    pub ranged_defence: u32,
    pub melee_strength: u32,
    pub ranged_strength: u32,
    pub magic_strength: u32,
    pub prayer: u32,
    pub weight: f32,
    pub bonus
}

pub enum CombatStyle {
    Melee,
    Magic,
    Ranged,
}

pub enum Skill {
    Attack(u64),
    Strength(u64),
    Defence(u64),
    Prayer(u64),
    Ranged(u64),
    Magic(u64),
    Hitpoints(u64),
}

pub enum Boost {
    Nothing,
    Normal,
    Super,
    Overload,
    SmellingSalt,
}
