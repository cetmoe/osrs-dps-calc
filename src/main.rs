use osrs_calc::{
    character::Character, math::effective_strength, EquipmentPiece, Potion, PotionVariant, Prayer,
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

    let pot = PotionVariant::Overload;
    let pray = Prayer::Piety;

    println!(
        "{}",
        effective_strength(character, osrs_calc::CombatStyle::Melee, pot, pray)
    );
}
