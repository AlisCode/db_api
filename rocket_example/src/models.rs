#[derive(Queryable)]
pub struct Hero {
    id: i32,
    year: i32,
    name: String,
    power: String,
}
