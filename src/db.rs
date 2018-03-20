use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type ManagedPgConn = ConnectionManager<PgConnection>;
type Pool = ::diesel::r2d2::Pool<ManagedPgConn>;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

/// Db Connection request guard type: wrapper around diesel::r2d2 pooled connection
pub struct DbConn(pub PooledConnection<ManagedPgConn>);

/// Create a new database connection pool from a given database url
pub fn new(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    ::diesel::r2d2::Pool::new(manager).expect("Failed to create connection pool.")
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = req.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &PgConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
