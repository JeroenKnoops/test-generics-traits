use super::query::Describe;

pub struct User {
    pub account_id: Option<i32>, // removed users do not have an account_id
    pub display_name: String,
    pub user_id: i32, // removed users do have a negative user_id
    pub is_employee: bool,
    pub creation_date: usize,
    pub user_type: String,
    pub last_access_date: usize,
    pub reputation_change_year: i32,
    pub reputation_change_quarter: i32,
    pub reputation_change_month: i32,
    pub reputation_change_week: i32,
    pub reputation_change_day: i32,
    pub reputation: i32,
    pub link: String,
    pub profile_image: String,
}

impl Describe for User {
    fn desc(&self) -> String {
        format!("Human: {}", self.display_name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desc_user() {
        let user = User {
            account_id: Some(10),
            display_name: String::from("jeroenknoops"),
            user_id: 102,
            is_employee: true,
            creation_date: 120,
            user_type: String::from("moderator"),
            last_access_date: 1232,
            reputation_change_year: 2022,
            reputation_change_quarter: 2,
            reputation_change_month: 6,
            reputation_change_week: 46,
            reputation_change_day: 6,
            reputation: 6,
            link: String::from("https://my-link.example.com"),
            profile_image: String::from("https://my-profile-image.example.com"),
        };

        assert_eq!(user.desc(), "Human: jeroenknoops");
    }
}
