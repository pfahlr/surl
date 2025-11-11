use axum::Router;

mod public;
mod account;
mod session;
mod admin;

use crate::app::AppState;

pub fn router(state: AppState) -> Router {
  public::routes(state.clone())
    .merge(account::routes(state.clone()))
    .merge(session::routes(state.clone()))
    .merge(admin::routes(state))
}
