use crate::{character::Character, character::Skill, CombatStyle, PotionVariant, Prayer};
use crate::{AttackStyle, Equipment, PrayerStrength::*};

pub fn calculate_max_hit(
    character: &Character,
    combat_style: &CombatStyle,
    potion: &PotionVariant,
    prayer: &Prayer,
    attack_style: &AttackStyle,
) -> u8 {
    let effective_strength = effective_level(
        character,
        combat_style,
        potion,
        prayer,
        attack_style,
        &Skill::Strength,
    ) as f32;

    let strength_bonus = gather_strength_bonus(character.equipment, combat_style);

    match combat_style {
        CombatStyle::Melee => {
            let maximum_hit =
                ((0.5 + effective_strength * (strength_bonus as f32 + 64.0)) / 640.0) as u8;

            maximum_hit
        }
        _ => 1,
    }
}

pub fn effective_level(
    character: &Character,
    combat_style: &CombatStyle,
    potion: &PotionVariant,
    prayer: &Prayer,
    attack_style: &AttackStyle,
    skill: &Skill,
) -> u8 {
    let potion_boost = potion.get_boost(&combat_style);
    let level = character.get_skill(skill);
    let level_and_pot_boosted = (level as f32 * potion_boost.multiplier) as u8 + potion_boost.level;

    match combat_style {
        CombatStyle::Melee => {
            let prayer_boosted = match (prayer.get_prayer(combat_style), skill) {
                (Weak(multi), _) => (level_and_pot_boosted as f32 * multi) as u8,
                (Strong(_, multi, _), Skill::Strength) => {
                    (level_and_pot_boosted as f32 * multi) as u8
                }
                (Strong(multi, _, _), Skill::Attack) => {
                    (level_and_pot_boosted as f32 * multi) as u8
                }
                (_, _) => 1,
            };

            println!("{}", prayer_boosted);
            prayer_boosted + 8 + attack_style.bonus(skill)
        }
        CombatStyle::Ranged => 1,
        _ => 1,
    }
}

pub fn gather_strength_bonus(equipment: Equipment, combat_style: &CombatStyle) -> u8 {
    let strength_bonus = equipment
        .as_vec()
        .iter()
        .fold(0, |acc, equip| acc + equip.get_strength(combat_style));

    strength_bonus
}
