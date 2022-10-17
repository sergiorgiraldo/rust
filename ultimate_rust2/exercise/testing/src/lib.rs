pub fn sploosh(x: i32, y: i32, z: i32) -> i32 {
    // I ran once with `match` and others with `if` to see the benchmarks

    match (x, y, z) {
        (x, _, _) if x < 0 => 99,
        (1, 2, 3) => 4,
        (5, 6, 7) => 3,
        (x, y, z) => x + y - z,
    }

    // if x < 0 {
    //     99
    // }
    // else if x == 1 && y == 2 && z == 3{
    //     4
    // }
    // else if x == 5 && y == 6 && z == 7{
    //     3
    // }
    // else{
    //     x + y - z
    // }

    // if x < 0 {
    //     99
    // } else {
    //     match (x, y, z) {
    //         (1, 2, 3) => 4,
    //         (5, 6, 7) => 3,
    //         (x, y, z) => x + y - z,
    //     }
    // }
}

///# examples
/// ```
/// # use testing::splish;
/// let res = -70;
/// assert_eq!(splish(100, 10), res);
/// ```
//running `cargo doc --no-deps --open` will show this example
pub fn splish(a: i32, b: i32) -> i32 {
    -a + 3 * b
}

pub fn guess(a: i32) -> Result<bool, String> {
    if a == 42 {
        Ok(true)
    } else {
        Err(String::from("Not answer for everything"))
    }
}

// 1. Use the `cfg` attribute to mark the `test` module below as a test module

#[cfg(test)]
mod test {
    // 2. Bring all the library items into scope with a `use` statement
    // Hint: It's okay to use `*` here.
    use super::*;

    // 3. Write a test function that verifies the following condition using the `assert_eq!` or
    // `assert_ne!` macros
    // - sploosh(1, 2, 3) returns 4
    // - sploosh(5, 6, 7) does not return 4
    // - If you pass sploosh a negative number for the first argument, 99 is returned
    //
    // `cargo test` should run your tests and pass
    // Hint: Don't forget the `#[test]` attribute for your test function!

    #[test]
    fn test_sploosh() {
        assert_eq!(sploosh(1, 2, 3), 4);
        assert_ne!(sploosh(5, 6, 7), 4);
        for i in -500000..-1 {
            assert_eq!(sploosh(i, 6, 7), 99);
        }
    }

    // 4. Write a test function that verifies the following conditions using the `assert!` macro
    // - splish(100, 10) is negative
    // - splish(40, 20) is positive
    // - splish(9, 3) is 0
    #[test]
    fn test_splish() {
        assert!(splish(100, 10) < 0);
        assert!(splish(40, 20) > 0);
        assert!(splish(9, 3) == 0);
    }

    #[test]
    fn test_guess() {
        assert_eq!(guess(42), Ok(true));
    }

    #[should_panic]
    #[test]
    fn test_guess_fail() {
        assert_eq!(guess(0), Ok(true));
    }

    #[test]
    fn test_guess_fail_message() {
        let actual_err = guess(0).unwrap_err();
        assert_eq!(actual_err, "Not answer for everything");
    }
}

// 5. Create a `tests/` directory and an integration test file `tests/more_tests.rs`
// Inside that file, create a test function that verifies:
// - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
//
// `cargo test` should run your `more_tests.rs` file and pass
//DONE

// Challenge: Create a benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started
//DONE
//the report is at ultimate_rust2/target/criterion
