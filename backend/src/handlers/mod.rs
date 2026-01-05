use actix_web::web;

pub mod auth;
pub mod loans;
pub mod savings;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(auth::register))
            .route("/login", web::post().to(auth::login))
            .route("/profile", web::get().to(auth::get_profile))
    )
    .service(
        web::scope("/loans")
            .route("", web::post().to(loans::create_loan))
            .route("", web::get().to(loans::get_loans))
            .route("/marketplace", web::get().to(loans::get_marketplace))
            .route("/{id}/fund", web::post().to(loans::fund_loan))
            .route("/repay", web::post().to(loans::repay_loan))
    )
    .service(
        web::scope("/savings")
            .route("", web::get().to(savings::get_savings))
            .route("", web::post().to(savings::create_savings))
            .route("/{id}/deposit", web::post().to(savings::deposit))
    );
}
