use actix_web::web;

pub mod auth;
pub mod loans;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(auth::register))
            .route("/login", web::post().to(auth::login))
    )
    .service(
        web::scope("/loans")
            .route("", web::post().to(loans::create_loan))
            .route("", web::get().to(loans::get_loans))
    );
}
