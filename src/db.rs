use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct DbExecutor(PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(database_url: &str) -> Self {
        DbExecutor(
            PgConnection::establish(database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        )
    }
}

pub mod callback {
    use super::super::db::DbExecutor;
    use super::super::models;
    use super::super::schema::*;
    use actix::prelude::*;
    use chrono::prelude::*;
    use diesel::prelude::*;

    use actix_web::Error;

    pub struct CreateCallback {
        pub url: String,
        pub scheduled_date: DateTime<Utc>,
    }

    impl Message for CreateCallback {
        type Result = Result<models::Callback, Error>;
    }

    impl Handler<CreateCallback> for DbExecutor {
        type Result = Result<models::Callback, Error>;

        fn handle(&mut self, msg: CreateCallback, _: &mut Self::Context) -> Self::Result {
            let new_callback = models::NewCallback {
                url: &msg.url,
                scheduled_date: &msg.scheduled_date,
            };

            Ok(diesel::insert_into(callbacks::table)
                .values(&new_callback)
                .get_result(&self.0)
                .expect("Error inserting callback"))
        }
    }

    pub struct GetCallback {
        pub id: i32,
    }

    impl Message for GetCallback {
        type Result = Result<models::Callback, Error>;
    }

    impl Handler<GetCallback> for DbExecutor {
        type Result = Result<models::Callback, Error>;

        fn handle(&mut self, msg: GetCallback, _: &mut Self::Context) -> Self::Result {
            Ok(callbacks::table
                .filter(callbacks::id.eq(msg.id))
                .limit(1)
                .load::<models::Callback>(&self.0)
                .expect("Error loading callbacks")
                .pop()
                .unwrap())
        }
    }
}
