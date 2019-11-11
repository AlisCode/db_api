use rocket::response::{Responder, Stream};
use rocket::Request;
use rocket_contrib::json::Json;
use serde::ser::Serialize;

use crate::responders::{JsonResponse, StreamResponse};

use std::io::Read;

// Chunk size for Rocket's Stream implementation
const CHUNK_SIZE: u64 = 10;

impl<'r, T: 'r> Responder<'r> for StreamResponse<T>
where
    T: Read,
{
    fn respond_to(self, request: &Request) -> rocket::response::Result<'r> {
        let streamed = Stream::<T>::chunked(self.inner, CHUNK_SIZE);
        streamed.respond_to(request)
    }
}

impl<'r, T> Responder<'r> for JsonResponse<T>
where
    T: Serialize,
{
    fn respond_to(self, request: &Request) -> rocket::response::Result<'r> {
        let json_repr: Json<T> = Json(self.inner);
        json_repr.respond_to(request)
    }
}
