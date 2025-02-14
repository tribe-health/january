use actix_web::{
    web::{self, Query},
    Responder,
};
use serde::Deserialize;

use crate::structs::embed::Embed;
use crate::structs::metadata::Metadata;
use crate::util::request::fetch;
use crate::{
    structs::media::{Image, ImageSize},
    util::{request::consume_size, result::Error},
};

#[derive(Deserialize)]
pub struct Parameters {
    url: String,
}

pub async fn get(info: Query<Parameters>) -> Result<impl Responder, Error> {
    let url = info.into_inner().url;
    let (resp, mime) = fetch(&url).await?;

    if let mime::HTML = mime.subtype() {
        let mut metadata = Metadata::from(resp, url).await?;
        metadata.resolve_external().await;

        if metadata.is_none() {
            return Ok(web::Json(Embed::None));
        }

        Ok(web::Json(Embed::Website(metadata)))
    } else if let mime::IMAGE = mime.type_() {
        if let Ok((width, height)) = consume_size(resp).await {
            Ok(web::Json(Embed::Image(Image {
                url,
                width,
                height,
                size: ImageSize::Large,
            })))
        } else {
            Ok(web::Json(Embed::None))
        }
    } else {
        Ok(web::Json(Embed::None))
    }
}
