#[derive(sqlx::FromRow, Debug)]
struct User {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "description")]
    pub about_me: String
}

