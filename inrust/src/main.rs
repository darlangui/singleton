mod database;
mod lazy_init;
mod logger;

use crate::database::{Database, DB_CONNECTION};
use crate::lazy_init::{change, do_a_call, other_do_a_call, ARRAY, OTHER_ARRAY};
use crate::logger::{error, info};

fn singleton() {
    println!("Initializing lazy...");

    let mut global_state = 0u32;

    change(&mut global_state);

    println!("Global state: {}", global_state);

    change(&mut global_state);

    println!("Global state: {}", global_state);

    println!("Other");

    do_a_call();
    do_a_call();
    do_a_call();

    println!("Called {}", ARRAY.lock().unwrap().len());

    other_do_a_call();
    other_do_a_call();
    other_do_a_call();

    let array = OTHER_ARRAY.lock().unwrap();
    println!("Called {} times: {:?}", array.len(), array);
    drop(array);

    *OTHER_ARRAY.lock().unwrap() = vec![3, 4, 5];

    println!("New singleton object: {:?}", OTHER_ARRAY.lock().unwrap());
    println!("end lazy...");
}

fn database() {
    Database::connect();

    let result = Database::execute_query("SELECT * FROM usuarios");
    println!("Resultado da query: {:?}", result);

    fn outro_modulo() {
        let result = Database::execute_query("INSERT INTO logs VALUES (now(), 'teste')");
        println!("Resultado da segunda query: {:?}", result);
    }

    outro_modulo();

    {
        let conn = DB_CONNECTION.lock().unwrap();
        println!("URL do banco: {}", conn.url);
    }
}

fn logger() {
    info("Aplicação iniciada");

    fn alguma_operacao() {
        info("Realizando operação");
        // Simular erro
        error("Falha ao processar dados");
    }

    alguma_operacao();

    info("Aplicação encerrada");
}

fn main() {
    singleton();
    database();
    logger();
}
