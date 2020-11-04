// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    // Execute `rustlings hint if1` for hints
    return if a > b { a } else { b };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq()  {
        assert_eq!(42, bigger(32, 42));
    }
}
