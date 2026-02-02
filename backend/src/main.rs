#[macro_use] extern crate rocket;;

use rocket::fs::{ Deserialize, Serialize, json::Json };
use rocket:: {State, response::status::Custom, http::Status};
use tokio_postgres::{ Client, NoTls};
use rocket_cors::{ CorsOptions, AllowedOrigins };

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Option<i32,
    name: String,
    email: String,
}

#[post("/api/users", data = "<user>)]
asnyc fn add_user(
    conn: &State<Client>,,
    user: Json<User>,
) -> Result<Custom<Json<User>>, Custom<String>> {
    execute_query(
        conn,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
        &[&user.name, &user.email],
    ).await?;
    get_users(conn).await
}


#[get("/api/users")]
asnyc fn get_users(
    conn: &State<Client>) -> Result<Json<Vec<User>>>, Custom<String>> {
    get_users_from_db(conn).await.map(Json)
}

asnyc fn get_users_from_db(
    client: &Client,) -> Result<vec<User>, Custom<String>> {
        let users = client
        .query("SELECT id, name, email FROM users". &[]).await
        .mapp_err(|e| Custom(Status::InternalServerError, e.to_string()))?
        .iter()
        .map(|row)
    }   

async fn execute_query(
    client: &Client,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<u64, Custom<String>> {
    client &Client 
        .execute(statement: query, params)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}