mod query;
mod tag;
mod user;

use query::{Query, QueryBuilder};
use user::User;

fn main() {
    println!("hello world!");
    let config = String::from("configuration");
    let query = QueryBuilder::<User>::new().config(config).build();
    println!("Query: {:?}", query);
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
