use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: i32,
}

impl User {
    async fn new(name: String, age: i32) -> User {
        let user = User {
            name: name.to_string(),
            age,
        };
        user
    }

    pub async fn get_all() -> Vec<User> {
        return vec![
            User::new("Jonathan Rwabahizi".to_string(), 25).await,
            User::new("John Smith".to_string(), 42).await,
        ];
    }
}
