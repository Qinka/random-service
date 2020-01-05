use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;



pub fn create_mongo_client(uri: &str, max_size: Option<u32>) -> Pool<MongodbConnectionManager>  {
    let manager = MongodbConnectionManager::new_with_uri(uri).unwrap();
    Pool::builder()
        .max_size(max_size.unwrap_or(16))
        .build(manager)
        .unwrap()
}