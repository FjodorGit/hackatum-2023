use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
// use crate::model::quality_factor_score::*;
use crate::schema::quality_factor_score;
//use crate::schema::quality_factor_score::dsl::quality_factor_score;
use crate::buisiness_logic;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Debug, Deserialize)]
pub struct PatchRequest {
    // At least one of the attributes should be defined
    maxDrivingDistance: i32,
    profilePictureScore: f64,
    profileDescriptionScore: f64,
}
