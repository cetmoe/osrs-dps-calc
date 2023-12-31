pub mod character;
pub mod math;

#[derive(Copy, Clone)]
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

impl Equipment {
    pub fn as_vec(&self) -> Vec<EquipmentPiece> {
        vec![
            self.ammo,
            self.helmet,
            self.neck,
            self.cloak,
            self.body,
            self.hands,
            self.ring,
            self.legs,
            self.feet,
            self.weapon,
            self.shield,
        ]
    }
}

#[derive(Copy, Clone)]
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
    pub melee_strength: u8,
    pub ranged_strength: u8,
    pub magic_strength: u8,
    pub prayer: u32,
    pub weight: f32,
}

impl EquipmentPiece {
    pub fn get_strength(self, combat_style: &CombatStyle) -> u8 {
        match combat_style {
            Melee => self.melee_strength,
            Ranged => self.ranged_strength,
            Magic => self.magic_strength,
        }
    }
}

pub enum AttackStyle {
    Accurate,
    Aggressive,
    Defensive,
    Controlled,
    Rapid,
    Longrange,
    Autocast,
}

impl AttackStyle {
    pub fn bonus(&self, skill: &Skill) -> u8 {
        match (self, skill) {
            (Self::Aggressive, Skill::Strength) => 3,
            (Self::Controlled, Skill::Strength) => 1,
            (Self::Aggressive, Skill::Attack) => 1,
            (Self::Controlled, Skill::Attack) => 3,
            (_, _) => 0,
        }
    }
}

pub enum SkillType {
    Attack,
    Strength,
    Defence,
}

pub enum CombatStyle {
    Melee,
    Magic,
    Ranged,
}
use CombatStyle::*;

pub enum PrayerStrength {
    Weak(f32),
    Strong(f32, f32, f32),
}

impl PrayerStrength {
    pub fn get_multiplier(self, skill_type: SkillType) -> f32 {
        match self {
            Self::Weak(multi) => multi,
            Self::Strong(atk, str, def) => match skill_type {
                SkillType::Attack => atk,
                SkillType::Strength => str,
                SkillType::Defence => def,
            },
        }
    }
}

use character::Skill;
use PrayerStrength::*;

pub enum Prayer {
    EagleEye,
    MysticMight,
    UltimateStrength,
    Chivalry,
    Piety,
    Rigour,
    Augury,
}

impl Prayer {
    pub fn get_prayer(&self, combat_style: &CombatStyle) -> PrayerStrength {
        match (self, combat_style) {
            (Self::EagleEye, Ranged) => Weak(1.15),
            (Self::MysticMight, Magic) => Weak(1.15),
            (Self::UltimateStrength, Melee) => Weak(1.15),
            (Self::Chivalry, Melee) => Strong(1.15, 1.18, 1.2),
            (Self::Piety, Melee) => Strong(1.2, 1.23, 1.25),
            (Self::Rigour, Ranged) => Strong(1.2, 1.23, 1.25),
            (Self::Augury, Magic) => Strong(1.25, 1.0, 1.25),
            (_, _) => Weak(1.0),
        }
    }
}

pub struct Potion {
    level: u8,
    multiplier: f32,
}

impl Potion {
    pub fn new(level: u8, multiplier: f32) -> Self {
        Self { level, multiplier }
    }
}

pub enum PotionVariant {
    Normal,
    Super,
    Overload,
    SmellingSalt,
    ImbuedHeart,
    SaturatedHeart,
}

impl PotionVariant {
    pub fn get_boost(&self, style: &CombatStyle) -> Potion {
        match (self, style) {
            (Self::Normal, Magic) => Potion::new(4, 1.0),
            (Self::Normal, Ranged) => Potion::new(4, 1.1),
            (Self::Normal, Melee) => Potion::new(3, 1.1),
            (Self::Super, Melee) => Potion::new(5, 1.15),
            (Self::Overload, _) => Potion::new(6, 1.16),
            (Self::SmellingSalt, _) => Potion::new(11, 1.16),
            (Self::ImbuedHeart, Magic) => Potion::new(1, 1.1),
            (Self::SaturatedHeart, Magic) => Potion::new(4, 1.1),
            (_, _) => Potion::new(0, 1.0),
        }
    }
}
