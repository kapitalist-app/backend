use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};

use db::{schema::wallets, DatabaseExecutor};
use request::{WalletCreationRequest};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Wallet {
    pub id: i32,
    pub name: String,
    pub initial_balance: i32,
    pub current_balance: i32,
    pub color: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "wallets"]
pub struct NewWallet {
    pub name: String,
    pub initial_balance: i32,
    pub current_balance: i32,
    pub color: String,
}

impl NewWallet {
    pub fn from_request(req: WalletCreationRequest) -> NewWallet {
        NewWallet {
            name: req.name,
            initial_balance: req.balance,
            current_balance: req.balance,
            color: req.color,
        }
    }
}

impl Message for NewWallet {
    type Result = Result<Wallet, Error>;
}

impl Handler<NewWallet> for DatabaseExecutor {
    type Result = Result<Wallet, Error>;

    fn handle(&mut self, msg: NewWallet, _: &mut Self::Context) -> Self::Result {
        use db::schema::wallets::dsl::*;

        // XXX: Figure out error type to be used here and add conversion functions for convenience
        /*let exists: bool = diesel::select(diesel::dsl::exists(wallets.filter(email.eq(&msg.email))))
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error getting User from Db"))?;

        if exists {
            // TODO: should we really return this message?
            return Err(error::ErrorUnauthorized("User already exists"));
        }*/

        let wallet: Wallet = diesel::insert_into(wallets)
            .values(&msg)
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error inserting wallet"))?;
        Ok(wallet)
    }
}
