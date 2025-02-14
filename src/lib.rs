pub mod utils {
    /// Add two numbers
    /// # Examples
    /// ```rust
    /// use publish_test::utils::add;
    /// assert_eq!(add(1,1),2);
    /// ```
    /// # Panic
    /// parameters or result large then 255 will panic
    pub fn add(left: u8, right: u8) -> u8 {
        left + right
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn it_should_panic() {
        add(255, 1);
    }
}
