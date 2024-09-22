use rocket::{
  response::{self, Responder},
  Request, Response, http::Status,
};

pub struct EmptyResponse;

impl<'r> Responder<'r, 'static> for EmptyResponse {
  fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
    Response::build()
      .status(Status::NoContent)
      .ok()
  }
}

#[cfg(feature = "schema")]
use autodocu::openapi3::RefOr;

#[cfg(feature = "schema")]
use rocket_autodocu::autodocu::openapi3;

#[cfg(feature = "schema")]
impl rocket_autodocu::response::OpenApiResponderInner for EmptyResponse {
  fn responses(
    _gen: &mut rocket_autodocu::gen::OpenApiGenerator,
  ) -> std::result::Result<openapi3::Responses, rocket_autodocu::OpenApiError> {
    let mut responses = autodocu::Map::new();

    responses.insert(
      "204".to_string(),
      RefOr::Object(openapi3::Response {
        description: "Success".to_string(),
        ..Default::default()
      }),
    );

    Ok(openapi3::Responses {
      responses,
      ..Default::default()
    })
  }
}
