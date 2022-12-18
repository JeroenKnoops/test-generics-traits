mod config;
mod query;
mod tag;
mod user;

use config::Config;
use query::{Query, QueryBuilder};
use tag::Tag;
use user::User;

fn main() {
    println!("hello world!");

    let config = Config::new("user", "filtertje");
    let query = QueryBuilder::<User>::new().config(config).build();
    println!("Query: {query:#?}");

    let config = Config::new("tag", "tags");
    let query = QueryBuilder::<Tag>::new().config(config).build();
    println!("Query: {query:#?}");
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
