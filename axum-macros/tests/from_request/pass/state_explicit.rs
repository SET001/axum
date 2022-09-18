use axum_macros::FromRequest;
use axum::extract::{FromRef, State};

#[derive(FromRequest)]
#[from_request(state(AppState))]
struct Extractor {
    inner_state: State<InnerState>,
}

struct AppState {
    inner: InnerState,
}

#[derive(Clone)]
struct InnerState {}

impl FromRef<AppState> for InnerState {
    fn from_ref(input: &AppState) -> Self {
        input.inner.clone()
    }
}

fn assert_from_request()
where
    Extractor: axum::extract::FromRequest<AppState, axum::body::Body, Rejection = axum::response::Response>,
{
}

fn main() {}
