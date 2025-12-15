use serde::{Deserialize, Serialize};
use crate::Db;
#[derive(Serialize,Deserialize)]
pub struct UserRequest {
    username:String,
    password:String
}
#[derive(Serialize,Deserialize)]
pub struct UserResponse{
    user_id:String
}

#[derive(Serialize,Deserialize)]
pub struct User{
    id:String,
    username:String,
    password:String
}
#[derive(Serialize,Deserialize)]
pub struct GetUserRequest{
    username:String,
}
#[derive(Serialize,Deserialize)]
pub struct GetUserResponse{
    user:User
}
impl Db {
     pub async fn create_user(&self,request:UserRequest)->Result<UserResponse>{
        let user = sqlx::query_as!(User, 
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username, password",
            request.username,
            request.password
        ).fetch_one(&self.pool).await?;
        Ok(UserResponse{
            user_id:user.id,
        });

      pub async fn get_user(&self,request:GetUserRequest)->Result<GetUserResponse>{
        let user = sqlx::query_as!(User, "SELECT id, username, password FROM users WHERE username = $1", request.username)
        .fetch_one(&self.pool)
        .await?;

    Ok(GetUserResponse {
        user: user,
    })
      }
    }
}