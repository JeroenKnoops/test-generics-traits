pub trait Describe {
    fn desc(&self) -> String {
        String::from("...")
    }
}

struct Tag {
    pub name: String,
    pub count: usize,
}

impl Describe for Tag {
    fn desc(&self) -> String {
        self.name.clone()
    }
}

struct Post {
    pub title: String,
}

impl Describe for Post {}

pub fn hello() -> String {
    String::from("hello")
}

pub fn desc<T: Describe>(model: &T) -> String {
    model.desc()
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

    #[test]
    fn desc_post() {
        let post = Post {
            title: String::from("rust is awesome"),
        };

        assert_eq!(post.desc(), "...");
    }

    #[test]
    fn desc_generic() {
        let post = Post {
            title: String::from("rust is awesome"),
        };

        let tag = Tag {
            name: String::from("nerd"),
            count: 303,
        };

        assert_eq!(desc(&post), "...");
        assert_eq!(desc(&tag), "nerd");
    }
}
