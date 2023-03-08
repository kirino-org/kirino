extern crate axum;
// extern crate axum_extra;

use alloc::{boxed::Box, string::String};
use std::error::Error;

use axum::{
  body::{Body, Bytes, Full, StreamBody},
  extract::{self, Extension, Path},
  http::{header, header::*, HeaderValue, StatusCode},
  middleware::from_fn,
  response::{self, Html, IntoResponse, Redirect, Response},
  routing::{self, any, get, get_service, post, Route},
  Router, Server,
};
// use axum_extra::routing::SpaRouter;
use kirino_core::Handle;
use kirino_proto::{self as proto, Message};

pub async fn main(handle: Handle) -> Result<(), Box<dyn Error>> {
  let r = Router::new()
    .nest("/meta", Router::new().route("/archive.pb", get(archive)))
    .nest(
      "/media",
      Router::new()
        .route("/:id", get(media))
        .route("/thumbnails/:id", get(thumbnail)),
    )
    .layer(Extension(handle));
  Server::bind(&"127.0.0.1:8080".parse()?)
    .serve(r.into_make_service())
    .await?;
  Ok(())
}

async fn archive(Extension(mut handle): Extension<Handle>) -> impl IntoResponse {
  let mut res = Response::builder();

  let mut archive = proto::Archive::default();
  for i in handle.items() {
    let item = handle.item(i.parse().unwrap());

    archive.items.push(proto::Item {
      r#type: proto::item::Type::Video.into(),
      id: i,
      name: item.name,
        img: None,
      //path: item.path,
    });
  }
  let buf = archive.encode_to_vec();

  res = res.status(200);
  res = res.header("Content-Type", "application/octet-stream");
  res.body(Body::try_from(buf).unwrap()).unwrap()
}

async fn media(
  Path(id): Path<u64>,
  Extension(mut handle): Extension<Handle>,
) -> impl IntoResponse {
  let mut res = Response::builder();
  if let Ok(f) = tokio::fs::File::open(handle.item(id).path).await {
    res = res.status(200);
    let st = tokio_util::io::ReaderStream::new(f);
    res.body(StreamBody::new(st)).unwrap()
  } else {
    panic!()
  }
}

async fn thumbnail(
  Path(id): Path<u64>,
  Extension(mut handle): Extension<Handle>,
) -> impl IntoResponse {
  let mut res = Response::builder();
  let f = tokio::fs::read(handle.item(id).img).await.unwrap();
  res = res.status(200);
  res = res.header("Content-Type", "image/png");
  res.body(Body::try_from(f).unwrap()).unwrap()
}
