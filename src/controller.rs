pub mod callback {
    use super::super::db::callback::{CreateCallback, GetCallback};
    use super::super::AppState;
    use actix_web::{
        error, AsyncResponder, Error, FutureResponse, HttpResponse, Json, Path, State,
    };
    use chrono::prelude::*;
    use futures::Future;

    #[derive(Deserialize)]
    pub struct CallbackCall {
        pub url: String,
    }

    pub fn create(
        (params, callback, state): (Path<(String,)>, Json<CallbackCall>, State<AppState>),
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let date_param = params
            .0
            .parse::<DateTime<Utc>>()
            .map_err(|_e| error::ErrorBadRequest("Bad date"))
            .unwrap();

        state
            .db
            .send(CreateCallback {
                url: callback.url.to_owned(),
                scheduled_date: date_param,
            })
            .from_err()
            .and_then(|res| match res {
                Ok(callback) => Ok(HttpResponse::Ok().json(callback)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            })
    }

    pub fn get((id, state): (Path<i32>, State<AppState>)) -> FutureResponse<HttpResponse> {
        state
            .db
            .send(GetCallback {
                id: id.into_inner(),
            })
            .from_err()
            .and_then(|res| match res {
                Ok(callback) => Ok(HttpResponse::Ok().json(callback)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            })
            .responder()
    }
}
