use deadpool_postgres::{ Manager, Pool};
use tokio_postgres::{Config, NoTls,Row};
use std::env;
use std::str::FromStr;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Todo {
    id: i32,
    name: String,
}

fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    Todo { id, name }
}

async fn fetch_to_dos(db_pool: &Pool) -> Result<Vec<Todo>> {
    let client = db_pool.get().await.unwrap();
    let rows = client
        .query("SELECT id, name from todo", &[])
        .await
        .unwrap();
    Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let mut cfg = Config::new();
    cfg.host(&env::var("HOST").unwrap());
    cfg.user(&env::var("USER").unwrap());
    cfg.password(&env::var("PASSWORD").unwrap());
    cfg.dbname(&env::var("DBNAME").unwrap());
    let port = &env::var("DBPORT").unwrap();
    cfg.port(port.parse::<u16>().unwrap());
    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    let pool = Pool::new(mgr, 16);
    //let pool = pool.clone();
    let r: Vec<Todo> = fetch_to_dos(&pool).await.unwrap();
    println!("length is {}", r.len());
    for i in r.iter() {
        println!("first name is {:?}", i);
    }
}
