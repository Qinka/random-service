use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;

pub struct AuthState{
    pub mongo: Pool<MongodbConnectionManager>,
}

pub struct SvrState {
    pub auth: AuthState,
}