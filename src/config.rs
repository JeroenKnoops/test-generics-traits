#[derive(Debug, Default, Clone)]
pub struct Config {
    pub resource: String,
    pub filter: String,
    pub sort: String,
    pub uri: String,
}

impl Config {
    pub fn new(resource: impl Into<String>, filter: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
            filter: filter.into(),
            sort: String::from("created_by"),
            uri: String::from("https://my-endpoint.example.com"),
        }
    }
}
