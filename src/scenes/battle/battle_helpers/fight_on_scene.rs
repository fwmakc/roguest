use colored::Colorize;
use console::Key;

use crate::{
    etv::{entities::goblin, types::Creature},
    interface::inputs::InputHandler,
    scenes::battle::battle_helpers,
};

pub fn fight_on_scene(input: &mut InputHandler, player: &mut Creature) -> bool {
    let mut goblin = goblin();

    println!(
        "Вы увидели противника: {} уровень {}",
        goblin.name.get().to_string().red(),
        goblin.level.get().to_string().red(),
    );

    println!("{}", "Хотите с ним сразиться?".yellow());

    println!(
        "Его ХП: {} | Ваши ХП: {}",
        goblin.hp.get().to_string().red(),
        player.hp.get().to_string().green(),
    );
    println!(
        "{} aтака | {} пройти мимо",
        "[A]".blue(),
        "[любая клавиша]".blue()
    );

    if let Some(key) = input.capture() {
        if key == Key::Char('a') || key == Key::Char('A') {
            let next = battle_helpers::fight_action(player, &mut goblin);

            input.clear_history();

            if !next {
                return false;
            }
        } else {
            println!("{}", "Вы прошли мимо".yellow());
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
