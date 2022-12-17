mod query;
mod tag;
mod user;

fn main() {
    println!("hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::Describe;

    #[test]
    fn desc_generic() {
        struct TestStruct {}
        impl Describe for TestStruct {}

        assert_eq!(query::desc(&TestStruct {}), "...");
    }
}
