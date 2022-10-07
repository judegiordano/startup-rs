#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tools;

    #[test]
    fn nanoid_should_unique() {
        assert_ne!(tools::nanoid(), tools::nanoid());
        assert_ne!(tools::nanoid(), tools::nanoid());
        assert_ne!(tools::nanoid(), tools::nanoid());
        assert_ne!(tools::nanoid(), tools::nanoid());
    }
}
