#[derive(Debug)]
pub struct Query<T> {
    pub config: String,
    pub elements: Vec<T>,
}

#[derive(Default)]
pub struct QueryBuilder<T>
where
    T: Default,
{
    pub config: Option<String>,
    pub elements: Vec<T>,
}

impl<T: std::default::Default> QueryBuilder<T> {
    pub fn new() -> Self {
        QueryBuilder::<T>::default()
    }

    pub fn config(&mut self, config: impl Into<String>) -> &mut Self {
        self.config = Some(config.into());
        self
    }

    pub fn build(&self) -> Result<Query<T>, &'static str> {
        let Some(config) = self.config.as_ref() else {
            return Err("No config");
        };
        Ok(Query {
            config: config.to_string(),
            elements: vec![T::default()],
        })
    }
}

pub trait Describe {
    fn desc(&self) -> String {
        String::from("...")
    }
}

pub fn desc<T: Describe>(model: &T) -> String {
    model.desc()
}

pub trait Resource {
    fn query_uri(&self) -> String {
        String::from("https://{}.stackenterprise.co/api/2.3/{}?page={}&pagesize={}&order=desc&sort={}&filter={}&key={}")
    }

    // Default no filter
    fn query_filter(&self) -> String {
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_defaults() {
        struct TestStruct {}
        impl Resource for TestStruct {}

        assert_eq!(&TestStruct {}.query_filter(), "");
        assert_eq!(&TestStruct {}.query_uri(), "https://{}.stackenterprise.co/api/2.3/{}?page={}&pagesize={}&order=desc&sort={}&filter={}&key={}");
    }

    #[test]
    fn desc_generic_on_struct() {
        struct TestStruct {
            name: String,
        }

        impl Describe for TestStruct {
            fn desc(&self) -> String {
                format!("Hi, my name is {}", self.name)
            }
        }

        assert_eq!(
            desc(&TestStruct {
                name: String::from("Slim")
            }),
            "Hi, my name is Slim"
        );
    }
}
