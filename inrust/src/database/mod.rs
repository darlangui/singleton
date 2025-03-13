use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct DatabaseConnection {
    pub(crate) url: String,
    pool_size: u32,
    is_connected: bool,
}

impl DatabaseConnection {
    fn new(url: &str, pool_size: u32) -> Self {
        println!("Criando nova conexão de banco de dados para: {}", url);
        DatabaseConnection {
            url: url.to_string(),
            pool_size,
            is_connected: false,
        }
    }

    pub fn connect(&mut self) {
        println!("Conectando ao banco de dados: {}", self.url);
        self.is_connected = true;
    }

    pub fn execute_query(&self, query: &str) -> Result<(), String> {
        if !self.is_connected {
            return Err("Conexão não estabelecida".to_string());
        }
        println!("Executando query '{}' no banco {}", query, self.url);
        Ok(())
    }
}

lazy_static! {
    pub static ref DB_CONNECTION: Mutex<DatabaseConnection> = Mutex::new(
        DatabaseConnection::new("postgres://localhost:5432/meudb", 10)
    );
}

pub struct Database;

impl Database {
    pub fn connect() {
        let mut conn = DB_CONNECTION.lock().unwrap();
        conn.connect();
    }

    pub fn execute_query(query: &str) -> Result<(), String> {
        let conn = DB_CONNECTION.lock().unwrap();
        conn.execute_query(query)
    }
}
