use crate::eco::entities::creature::Creature;
use colored::*;

pub fn safe_action(player: &mut Creature) {
    while !player.hp.is_max() && player.gold.get() >= 10 {
        player.gold.sub(10);
        player.hp.add(10);

        println!(
            "Вы восстановили 10 баллов здоровья. Ваши ХП: {} из {}",
            player.hp.get().to_string().green(),
            player.hp.max().to_string().green(),
        );
    }

    if player.hp.is_max() {
        println!("Вы полностью вылечелись");
    }
}
