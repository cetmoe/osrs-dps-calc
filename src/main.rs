use osrs_calc::{
    character::Character,
    math::{calculate_max_hit, effective_level},
    AttackStyle, EquipmentPiece, PotionVariant, Prayer,
};

fn main() {
    let e = EquipmentPiece {
        stab_attack: 0,
        slash_attack: 0,
        crush_attack: 0,
        magic_attack: 0,
        ranged_attack: 0,
        stab_defence: 0,
        slash_defence: 0,
        crush_defence: 0,
        magic_defence: 0,
        ranged_defence: 0,
        melee_strength: 0,
        ranged_strength: 0,
        magic_strength: 0,
        prayer: 0,
        weight: 0.0,
    };

    let character = Character {
        attack: 99,
        strength: 99,
        defence: 99,
        prayer: 99,
        magic: 99,
        ranged: 99,
        hitpoints: 99,
        equipment: osrs_calc::Equipment {
            ammo: e,
            helmet: e,
            neck: e,
            cloak: e,
            body: e,
            hands: e,
            ring: e,
            legs: e,
            feet: e,
            weapon: e,
            shield: e,
        },
    };

    let pot = PotionVariant::SmellingSalt;
    let pray = Prayer::Piety;

    let attack_style = AttackStyle::Aggressive;

    println!(
        "{}",
        effective_level(
            &character,
            &osrs_calc::CombatStyle::Ranged,
            &pot,
            &pray,
            &attack_style,
            &osrs_calc::character::Skill::Ranged
        )
    );

    println!(
        "{}",
        calculate_max_hit(
            &character,
            &osrs_calc::CombatStyle::Melee,
            &pot,
            &pray,
            &attack_style,
        )
    )
}
