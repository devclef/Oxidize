use actix_web::Responder;
use actix_files::NamedFile;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}
