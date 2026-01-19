use roguest::engine::math::for_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_panic_when_a_is_too_large() -> Result<(), String> {
        let res = for_tests::result(90);

        if res.is_err() {
            Err("Got an error!".into())
        } else {
            Ok(())
        }
    }

    #[test]
    #[should_panic(expected = "boom!")]
    fn it_always_panics() {
        for_tests::boom();
    }

    #[test]
    #[ignore]
    fn it_returns_str() {
        assert_eq!(
            for_tests::str_data(),
            "Hello, world!",
            "Invalid string has been returned!"
        )
    }

    #[test]
    fn it_returns_true() {
        // assert!(false);
        assert_eq!(true, true);
        assert_ne!(true, false);
    }
}
