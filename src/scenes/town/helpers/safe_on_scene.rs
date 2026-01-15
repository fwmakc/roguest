use colored::*;
use console::Key;

use crate::eco::entities::creature::Creature;
use crate::engine::inputs::InputHandler;
use crate::scenes::battle::helpers;

pub fn safe_on_scene(input: &mut InputHandler, player: &mut Creature) -> bool {
    if player.hp.is_max() || player.hp.is_min() || player.gold.get() < 10 {
        return true;
    }

    println!("У вас мало здоровья");
    println!("{}", "Хотите подлечиться?".yellow());
    println!("{}", "Восстановить 10 баллов за 10 монет.".blue());
    println!(
        "Ваши ХП: {}. Ваши монеты: {}",
        player.hp.get().to_string().red(),
        player.gold.get().to_string().green(),
    );
    println!(
        "{} лечиться | {} пропустить",
        "[S]".blue(),
        "[любая клавиша]".blue()
    );

    if let Some(key) = input.capture() {
        if key == Key::Char('s') || key == Key::Char('S') {
            helpers::safe_action(player);
            input.clear_history();
        } else {
            println!("{}", "Вы пропустили лечение".yellow());
        }

        if key == Key::Escape {
            return false;
        }

        let combo = [Key::ArrowUp, Key::ArrowDown];
        if input.check_sequence(&combo) {
            println!("СУПЕР-ПРИЕМ!");
            input.clear_history();
        }
    }

    true
}
