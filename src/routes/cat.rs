use axum::{
  extract::Extension,
  routing::{get, post, MethodRouter},
  Json, Router,
};
use serde::Deserialize;
use bson::doc;
use tracing::debug;

use crate::context::Context;
use crate::errors::Error;
use crate::lib::token::TokenUser;
use crate::models::cat::{Cat, PublicCat};
use crate::models::ModelExt;

pub fn create_route() -> Router {
  Router::new()
    .merge(route("/cats", post(create_cat)))
    .merge(route("/cats", get(query_cats)))
}

fn route(path: &str, method_router: MethodRouter) -> Router {
  Router::new().route(path, method_router)
}

#[derive(Deserialize)]
struct CreateCat {
  name: String,
}

async fn create_cat(
  user: TokenUser,
  Extension(context): Extension<Context>,
  Json(payload): Json<CreateCat>,
) -> Result<Json<PublicCat>, Error> {
  let cat = Cat::new(user.id, payload.name);
  let cat = context.models.cat.create(cat).await?;
  let res = PublicCat::from(cat);

  Ok(Json(res))
}

async fn query_cats(user: TokenUser, Extension(context): Extension<Context>) -> Result<Json<Vec<PublicCat>>, Error> {
    let cats = context
    .models
    .cat
    .find(doc! { "user": &user.id }, None)
    .await?
    .into_iter()
    .map(Into::into)
    .collect::<Vec<PublicCat>>();

    debug!("Returning cats");
    Ok(Json(cats))
}