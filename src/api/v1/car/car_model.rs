use serde::{ Deserialize, Serialize};

$[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub id: String,
    pub name: String,
    pub price: String,
    pub user_id: String,
}

// Post request body for new car
#[derive(Debug, Serialize, Deserialize)]
pub struct NewCar {
    pub name: String,
    pub price: String,
    pub user_id: String,
}