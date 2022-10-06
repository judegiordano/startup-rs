use async_once::AsyncOnce;
use mongodb::{bson::doc, Client, Database};

use crate::util::config::Config;

lazy_static! {
    pub static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(async {
        let conn = Config::get_required("MONGO_URI");
        let client = Client::with_uri_str(conn).await.unwrap_or_else(|err| {
            eprintln!("error connecting to db: {}", err);
            std::process::exit(1)
        });
        client.default_database().map_or_else(
            || {
                eprintln!("no default database found");
                std::process::exit(1)
            },
            |db| db,
        )
    });
}
