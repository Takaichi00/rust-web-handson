use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct TodoCreateRequestJson {
    title: String,
    description: String
}

impl TodoCreateRequestJson {
    pub fn new (title: String, description: String) -> Self {

        Self {
            title,
            description
        }
    }

    pub fn getTitle(&self) -> &String {
        &self.title
    }

    pub fn getDescription(&self) -> &String {
        &self.description
    }
}
