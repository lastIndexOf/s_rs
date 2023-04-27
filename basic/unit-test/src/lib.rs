pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_error() {
        panic!("has error");
    }
}
