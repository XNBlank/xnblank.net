#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub image: String,
    pub url: String,
    pub category: String,
}