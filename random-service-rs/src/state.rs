use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;
use actix_web::web;

pub struct AuthState{
    pub mongo: Pool<MongodbConnectionManager>,
}

pub struct SvrState {
    pub auth: AuthState,
}


pub type State = web::Data<SvrState>;