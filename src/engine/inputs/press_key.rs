use console::{Key, Term};

#[allow(unused)]
pub fn press_key<Resolve, Reject>(
    target_keys: &[Key],
    mut callback_resolve: Resolve,
    mut callback_reject: Reject,
) where
    Resolve: FnMut(),
    Reject: FnMut(),
{
    let term = Term::stdout();
    let key = term.read_key().expect("Ошибка чтения клавиши");

    if target_keys.contains(&key) {
        callback_resolve();
    } else {
        callback_reject();
    }
}
