use actix_web::{HttpResponse, Responder, post, web};
use aws_sdk_s3::Client;

use crate::store;

#[post("/problems/{id}/ac")]
pub async fn post_ac(
    client: web::Data<Client>,
    path: web::Path<String>
) -> impl Responder {
    let id = path.into_inner();

    let Some(mut problems) = store::read_json(client.clone()).await else {
        return HttpResponse::NotFound().finish();
    };

    let Some(problem) = problems.iter_mut().find(|p| p.id.to_string() == id) else {
        return HttpResponse::NotFound().finish();
    };

    problem.ac_count += 1;
    problem.last_solved_at = Some(chrono::Local::now().format("%Y%m%d").to_string());

    let updated = problem.clone();

    if store::write_json(client, &problems).await.is_none() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(updated)
}