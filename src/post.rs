struct Post {
    pub title: String,
}

impl Describe for Post {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn desc_post() {
        let post = Post {
            title: String::from("rust is awesome"),
        };

        assert_eq!(post.desc(), "...");
    }
}
