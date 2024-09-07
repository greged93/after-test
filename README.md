# Introduction

This crate is a provides users with a simple macro that allows to define a cleanup function which can be
applied to all tests in a module. This can be useful if you require to remove temporary files or directories
at the end of each test, or if you need to reset some global state.

# Examples

Simply provide the cleanup function you wish to call for each of your test a the end of the execution. The below
shows an example to reset the state of a global variable used across the tests.

```rust
use after_test::cleanup;

#[cfg(test)]
#[cleanup(reset_global_state)]
mod tests {
    use std::sync::{Arc, LazyLock, Mutex};

    static ZERO: LazyLock<Arc<Mutex<u32>>> = LazyLock::new(|| Arc::new(Mutex::new(0)));


    fn reset_global_state() {
        // Reset the global state
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
```

The `reset_global_state` function will be called at the end of each function marked with the `#[test]` attribute in the
module.

# Roadmap

- [ ] Add support for async functions
- [ ] Add support for parametrized cleanup functions
- [ ] Add support for closures