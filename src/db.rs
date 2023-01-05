
use std::error::Error;

use tokio_postgres_migration::Migration;
// use tokio_postgres::{Config,NoTls, Error};
use tokio_postgres::NoTls;
use deadpool_postgres::{Config, Pool,Runtime,Manager, ManagerConfig, RecyclingMethod};



const SCRIPTS_UP: [(&str, &str); 2] = [
    ("01_create_users",include_str!("../migrations/01_create_users.sql")),
    ("02_create_cycles",include_str!("../migrations/02_create_cycles.sql")),
 ];
    
// pub async fn get_client() -> tokio_postgres::Client  {
//     let mut c = Config::new();
//     c.host("localhost");
//     c.password("operatorpass123");
//     c.user("lilian");
//     c.dbname("postgres");
//     c.port(5432 as u16);

    
//     let (client, connection) = c.connect(NoTls).await.unwrap();
//     tokio::spawn(connection);
//     client
// }

pub fn to_pool() -> Pool {
    let mut c = tokio_postgres::Config::new();
    c.host("localhost");
    c.password("operatorpass123");
    c.user("lilian");
    c.dbname("postgres");
    c.port(5432 as u16);    
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(c, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();
    pool
}

// pub async fn get_pool() -> Pool<Manager> {
//     let mut cfg = Config::new();
//     cfg.dbname = Some("postgres".to_string());
//     cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
//     let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
//     pool
// }
// pub async fn migrate_up() -> Result<(), Error> {
//     let mut client = get_pool().await.get().await.unwrap();
//     let migration = Migration::new("migrations".to_string());
//     migration
//         .up(&mut client, &SCRIPTS_UP)
//         .await
//         .unwrap();
//     // Now we can execute a simple statement that just returns its parameter.
//     let rows = client
//         .query("SELECT $1::TEXT", &[&"hello world"])
//         .await?;

//     // And then check that we got back the same string we sent over.
//     let value: &str = rows[0].get(0);
//     assert_eq!(value, "hello world");
//     Ok(())
// }