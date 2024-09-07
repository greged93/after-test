#[after_test::cleanup(panic_with_message)]
#[cfg(test)]
mod panics {
    fn panic_with_message() {
        panic!("This is a panic message");
    }

    #[test]
    #[should_panic(expected = "This is a panic message")]
    fn test_should_panic() {}
}

#[after_test::cleanup(clean_resources)]
#[cfg(test)]
mod resources {
    use std::sync::{Arc, LazyLock, Mutex};

    static ZERO: LazyLock<Arc<Mutex<u32>>> = LazyLock::new(|| Arc::new(Mutex::new(0)));

    fn clean_resources() {
        *ZERO.lock().unwrap() = 0;
    }

    #[test]
    fn test_increment() {
        let mut zero = ZERO.lock().unwrap();
        *zero += 1;
        assert_eq!(*zero, 1);
    }

    #[test]
    fn test_increment_double() {
        let mut zero = ZERO.lock().unwrap();
        *zero += 2;
        assert_eq!(*zero, 2);
    }
}
