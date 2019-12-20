use super::run;
use actix_web;
use actix_files;

use actix_web::{HttpServer, App as ActixApp, HttpRequest, HttpResponse, web};
use actix_files as fs;

pub(super) fn web() {
    println!("Building for Web…");

    let mut ip_port = "0.0.0.0:8080";
    println!("Running on {}…", ip_port);

    let mut server = HttpServer::new(move ||
        ActixApp::new()
            .service(web::resource("/{page}").route(
                web::get().to(|_req: HttpRequest, path: web::Path<(String,)>| {
                    fs::NamedFile::open(format!("./res/web/{}", path.0))
                }
            )))
            .default_service(
                web::get().to(|_req: HttpRequest| {
                    fs::NamedFile::open("./res/web/404.html")
                })
            )
    );

    server.bind(ip_port).unwrap().run().unwrap();
}
