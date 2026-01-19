use colored::Colorize;

use crate::objects::types::Creature;

pub fn fight_action(player: &mut Creature, enemy: &mut Creature) -> bool {
    while !player.hp.is_min() {
        let player_attack = player.calculate_attack();
        enemy.hp.sub(player_attack);

        println!(
            "Вы ударили {} и нанесли {} урона",
            enemy.name.get().to_string().red(),
            player_attack.to_string().green(),
        );

        if enemy.hp.is_min() {
            break;
        }

        let enemy_attack = enemy.calculate_attack();
        player.hp.sub(enemy_attack);

        println!("Его ХП: {}", enemy.hp.get().to_string().red(),);

        println!(
            "Вас ударил {} и нанес {} урона",
            enemy.name.get().to_string().red(),
            enemy_attack.to_string().red(),
        );

        println!("Ваши ХП: {}", player.hp.get().to_string().green(),);
    }

    if player.hp.is_min() {
        println!("Вас убил {}", enemy.name.get().to_string().red());
        return false;
    }

    println!("{}", "Вы его убили".yellow());

    let gold = enemy.gold.get();
    player.gold.add(gold);

    println!(
        "Вы получили {} монет. У вас {} монет",
        gold.to_string().green(),
        player.gold.get().to_string().green(),
    );

    true
}
