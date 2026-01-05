use actix_web::web;

pub mod auth;
pub mod loans;
pub mod savings;

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
            .route("/repay", web::post().to(loans::repay_loan))
    )
    .service(
        web::scope("/savings")
            .route("", web::get().to(savings::get_savings))
            .route("", web::post().to(savings::create_savings))
            .route("/{id}/deposit", web::post().to(savings::deposit))
    );
}
