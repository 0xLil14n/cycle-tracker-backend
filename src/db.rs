
use tokio_postgres_migration::Migration;
use tokio_postgres::{Config,NoTls};



const SCRIPTS_UP: [(&str, &str); 2] = [
    ("01_create_users",include_str!("../migrations/01_create_users.sql")),
    ("02_create_cycles",include_str!("../migrations/02_create_cycles.sql")),
 ];
    
pub async fn get_client() -> tokio_postgres::Client  {
    let mut c = Config::new();
    c.host("localhost");
    c.password("operatorpass123");
    c.user("lilian");
    c.dbname("postgres");
    c.port(5432 as u16);
    
    let (client, connection) = c.connect(NoTls).await.unwrap();
    tokio::spawn(connection);
    client
}

pub async fn migrate_up() {
    let mut client = get_client().await;
    let migration = Migration::new("migrations".to_string());
    migration
        .up(&mut client, &SCRIPTS_UP)
        .await
        .unwrap();
}