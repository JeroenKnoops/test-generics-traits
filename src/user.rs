use super::query::Describe;

pub struct User {
    pub name: String,
    pub age: usize,
}

impl Describe for User {
    fn desc(&self) -> String {
        format!("Human: {}", self.name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desc_user() {
        let user = User {
            name: String::from("jeroenknoops"),
            age: 47,
        };

        assert_eq!(user.desc(), "Human: jeroenknoops");
    }
}
