mod query;
mod user;

use crate::query::Describe;

fn main() {
    println!("{}, world!", query::hello());
    println!(
        "test: {}",
        user::User {
            name: String::from("Jeroen Knoops"),
            age: 47
        }
        .desc()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desc_user() {
        let user = user::User {
            name: String::from("jeroenknoops"),
            age: 47,
        };

        assert_eq!(user.desc(), "Human: jeroenknoops");
    }
}
