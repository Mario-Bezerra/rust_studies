use std::{
    hash::{Hash, Hasher},
    collections::{HashMap, hash_map::DefaultHasher},
    sync::{Mutex, Arc}, };
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame , Command:: {self, Get, Set}};
use bytes::Bytes;

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = new_shared_db(4);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: ShardedDb) {

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap(){

        let response = match Command::from_frame(frame).unwrap() {
            
            Set(cmd) => {
                let key = cmd.key();
                let mut shard_guard = get_shard(key, &db).lock().unwrap();
                shard_guard.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            
            Get(cmd) => {
                let db = get_shard(cmd.key(), &db).lock().unwrap();
                if let Some(value) = db.get(cmd.key()){
                    Frame::Bulk(Bytes::copy_from_slice(value))
                } else {
                    Frame::Null
                }
            }
            
            cmd =>  panic!("Uninplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    } 
}

fn new_shared_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()))
    }
    Arc::new(db)
}

fn hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

fn get_shard<'a>(key : &str , db : &'a ShardedDb) -> &'a Mutex<HashMap<String, Vec<u8>>>{
    let shard_index = hash(&key) % db.len() as u64;
    let shard_mutex = &db[shard_index as usize];
    shard_mutex
}