use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
}


#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
}



#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub name: String,
    pub role: String,
}



