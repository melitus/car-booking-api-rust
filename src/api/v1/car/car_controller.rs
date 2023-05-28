use super::{
    service,
};
use uuid::Uuid;

type CarId = Uuid;

pub async fn get_cars(state: web::Data<AppState>, req: HttpRequest) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let list = service::find_all_cars(conn)?;
    Ok(HttpResponse::Ok().json(list))
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<CarId>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let (car_id) = path.into_inner();
    let car_id = uuid::parse(&car_id)?;

    service::delete(
        conn,
        &CarId
    )?;
    Ok(HttpResponse::Ok().json(()))
}