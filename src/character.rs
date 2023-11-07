 
use PrayerVariants::*;

use crate::CombatStyle;
use crate::Skill;

pub struct Character {
    pub attack: Skill,
    pub strength: Skill,
    pub defence: Skill,
    pub prayer: Skill,
    pub ranged: Skill,
    pub magic: Skill,
    pub hitpoints: Skill,
    pub equipment: Equipment
}

pub enum PrayerVariants {
    Strong(CombatStyle, f32, f32, f32),
    Weak(CombatStyle, f32)
}


pub struct Prayer;

impl Prayer {
    const EAGLE_EYE = Weak(CombatStyle::Ranged, 0.15);
    const MTSTIC_MIGHT = Weak(CombatStyle::Magic, 0.15);
    const ULTIMATE_STRENGTH = Weak(CombatStyle::Melee, 0.15);
    const CHIVALRY = Strong(CombatStyle::Melee, 0.15, 0.18, 0.2);
    const PIETY = Strong(CombatStyle::Melee, 0.2, 0.23, 0.25);
    const RIGOUR = Strong(CombatStyle::Ranged, 0.2,0.23, 0.25);
    const AUGURY = Strong(CombatStyle::Magic, 0.25, 0.0, 0.25);
}

