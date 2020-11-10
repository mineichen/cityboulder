use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_files::Files;
use env_logger::Env;
use futures::prelude::*;

async fn get() -> Result<HttpResponse, actix_web::Error> {
    let stream = web::block::<_,_,std::io::Error>(|| {
        Ok(cityboulder_data::VisitorRepository::new("sqlite.db").load())
    }).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    
    HttpResponse::Ok().streaming::<_, actix_web::Error>(stream.map(|visit| {
        let mut buf = actix_web::web::BytesMut::new();
        use std::fmt::Write;
        writeln!(
            &mut buf,
            "{},{}", 
            visit.recorded_at.format("%Y-%m-%d %H:%M:%S UTC"), 
            visit.guest_count
        ).map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
        
        Ok(buf.into())
    })).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/data/{name}", web::get().to(get))
            .service(Files::new("/", "./cityboulder_web/static/").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
