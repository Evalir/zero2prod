use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(
    _form: web::Form<FormData>,
    _connection: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        _subscriber_email = %_form.email,
        _subscriber_name = %_form.name
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
        "
        Saving new subscriber details into the database
        "
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_connection.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            tracing::error!("Failed to execute query: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
