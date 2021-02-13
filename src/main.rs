use restaurant_api::ignite_rocket;
use restaurant_api::initialize_database;

fn main() {
    ignite_rocket(|rocket| {
        initialize_database(rocket, &"sql/schema.sql");
        initialize_database(rocket, &"sql/data.sql");
    })
    .launch();
}
