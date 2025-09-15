use std::sync::{Arc, Mutex};
use std::thread;

fn main(){
    let contador = Arc::new(Mutex::new(0));
    let mut hilos = vec![];

    for _ in 0..10{
        let contador_individual = Arc::clone(&contador);
        let hilo = thread::spawn(move || {
            let mut c = contador_individual.lock().unwrap();
            println!("Mensaje del hilo #{}", *c);
            *c += 1;
        });
        hilos.push(hilo);
    }
    for hilo in hilos {
        hilo.join().unwrap();
    }

    println!("Se ejecutaron los {} hilos correctamente.",*contador.lock().unwrap())
}