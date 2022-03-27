#[cfg(test)]
mod test {
    #[test]
    pub fn tests() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn failed() {
        assert_ne!(2 + 1, 4);
    }
}
