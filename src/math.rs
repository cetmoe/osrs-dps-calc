use crate::PrayerStrength::*;
use crate::{character::Character, CombatStyle, PotionVariant, Prayer};
// pub fn calculate_max_hit(
//     character: Character,
//     combat_style: CombatStyle,
//     potion: PotionVariant,
//     prayer: PrayerStrength,
//     attack_style: u8,
// ) {
// }

pub fn effective_strength(
    character: Character,
    combat_style: CombatStyle,
    potion: PotionVariant,
    prayer: Prayer,
) -> u32 {
    match combat_style {
        CombatStyle::Melee => {
            let str_and_pot_boosted = character.strength + potion.get_boost(&combat_style).level;
            let prayer_boosted = match prayer.get_boost(&combat_style) {
                // cast both to u32 to floor them.
                Weak(multi) => (str_and_pot_boosted as f32 * multi) as u32,
                Strong(_, multi, _) => (str_and_pot_boosted as f32 * multi) as u32,
            };
            let attack_style_boosted = prayer_boosted;
            attack_style_boosted
        }
        _ => 1,
    }
}
