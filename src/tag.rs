use super::query::Describe;

#[derive(Debug, Default)]
pub struct Tag {
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Default)]
pub struct ResourceConfig {
    pub filter: String,
    pub sort: String,
    pub uri: String,
}

impl ResourceConfig {
    fn new() -> Self {
        Self {
            filter: String::from("filter-tag"),
            sort: String::from("popular"),
            uri: String::from("https://my-endpoint.example.com"),
        }
    }
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
