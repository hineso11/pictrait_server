use async_graphql::{Object, ID};

pub struct Profile {
    user_id: u64,
    first_name: String,
    second_names: String,
}

impl Profile {
    pub fn new(user_id: u64, first_name: String, second_names: String) -> Profile {
        Profile {
            user_id,
            first_name,
            second_names,
        }
    }
}

#[Object]
impl Profile {
    async fn user_id(&self) -> ID {
        ID(self.user_id.to_string())
    }

    async fn first_name(&self) -> &String {
        &self.first_name
    }

    async fn second_names(&self) -> &String {
        &self.second_names
    }
}
