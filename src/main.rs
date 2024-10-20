use actix_web::{delete, post, get, web, HttpServer, App, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, Arc};

#[derive(Deserialize, Serialize, Clone)]
struct Actividad{
    id: u8,
    encargado: String,
    credito: String,
    nombre: String,
    descripcion: String
}

type ActividadesExtraescolares = Arc<Mutex<Vec<Actividad>>>;

#[post("/insertar-actividad")]
async fn insertar_actividad(actividad: web::Form<Actividad>,data: web::Data<ActividadesExtraescolares>,) -> HttpResponse {
    let mut actividades = data.lock().unwrap();
    actividades.push(actividad.into_inner());
    HttpResponse::Ok().json("Actividad insertada correctamente")
}

#[get("/actividad")]
async fn actividades_extra(data: web::Data<ActividadesExtraescolares>) -> HttpResponse {
    let actividades = data.lock().unwrap();
    HttpResponse::Ok().json(&*actividades) // Se regresa el vector de actividades en formato JSON
}

#[get("/actividad-id/{id}")]
async fn actividad_id(parametros: web::Path<u8>, data: web::Data<ActividadesExtraescolares>) -> HttpResponse {
    let actividades = data.lock().unwrap();
    let id = parametros.into_inner();
    if let Some(actividad) = actividades.iter().find(|&a| a.id == id) {
        HttpResponse::Ok().json(actividad)
    } else {
        HttpResponse::NotFound().json("Actividad no encontrada")
    }
}

#[delete("/borrar-actividad/{id}")]
async fn borrar_actividad(parametros: web::Path<u8>, data: web::Data<ActividadesExtraescolares>) -> HttpResponse {
    let mut actividades = data.lock().unwrap();
    let id = parametros.into_inner();
    if let Some(pos) = actividades.iter().position(|a| a.id == id) {
        actividades.remove(pos);
        HttpResponse::Ok().json("Actividad borrada correctamente")
    } else {
        HttpResponse::NotFound().json("Actividad no encontrada")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let actividades_data = web::Data::new(Arc::new(Mutex::new(Vec::<Actividad>::new())));

    HttpServer::new(move || App::new()
        .app_data(actividades_data.clone())
        .service(insertar_actividad)
        .service(actividades_extra)
        .service(actividad_id)
        .service(borrar_actividad)
    )
        .bind(("0.0.0.0",8080))?
        .run()
        .await 
}
