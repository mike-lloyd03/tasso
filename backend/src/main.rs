// use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    // cookie::{Key, SameSite},
    middleware::Logger,
    middleware::NormalizePath,
    web::{scope, Data},
    App,
    HttpServer,
};
// use actix_web_lab::web::spa;

mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // let secret_string = env::var("TASSO_SECRET_KEY");
    // let secret_key = match secret_string {
    //     Ok(s) => {
    //         info!("Generating secret key from environment variable");
    //         Key::from(s.as_bytes())
    //     }
    //     Err(_) => {
    //         info!("Generating random secret key");
    //         Key::generate()
    //     }
    // };

    let pool = match models::db().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // models::initialize_admin(&pool).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            // .wrap(
            //     SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
            //         .cookie_secure(false) // TODO: set env specific
            //         .cookie_http_only(false)
            //         .cookie_same_site(SameSite::Strict)
            //         .build(),
            // )
            .app_data(Data::new(pool.clone()))
        // .service(scope("/api").service(routes::keys::get))
        // .service(
        //     spa()
        //         .index_file("./dist/index.html")
        //         .static_resources_mount("/")
        //         .static_resources_location("./dist")
        //         .finish(),
        // )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
