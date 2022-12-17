use super::query::Describe;

pub struct Tag {
    pub name: String,
    pub count: usize,
}

impl Describe for Tag {
    fn desc(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desc_tag() {
        let tag = Tag {
            name: String::from("nerd"),
            count: 303,
        };

        assert_eq!(tag.desc(), "nerd");
    }
}
