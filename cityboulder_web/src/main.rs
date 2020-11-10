use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_files::Files;
use env_logger::Env;
use std::fmt::Write;

async fn get() -> Result<HttpResponse, actix_web::Error> {
    let res = web::block::<_,_,std::fmt::Error>(|| {
        let visits = cityboulder_data::VisitorRepository::new("sqlite.db").load();
        
        let mut buff = String::new();
        for visit in visits.iter() {
            writeln!(
                &mut buff, 
                "{},{}", 
                visit.recorded_at.format("%Y-%m-%d %H:%M:%S UTC"), 
                visit.guest_count
            )?;
        }

        Ok(buff)
    }).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    
    HttpResponse::Ok().body(res).await
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
