pub trait Describe {
    fn desc(&self) -> String {
        String::from("...")
    }
}

pub fn desc<T: Describe>(model: &T) -> String {
    model.desc()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desc_generic_on_default() {
        struct TestStruct {}
        impl Describe for TestStruct {}

        assert_eq!(desc(&TestStruct {}), "...");
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
