use std::{cell::RefCell, rc::{Rc, Weak}};  // importaciones necesarias

fn main(){
    // crearemos el nodo A
    let a = Rc::new(RefCell::new(Nodo {
        valor: "Nodo A".to_string(),
        hacia: None,    // no apunta a nadie
        desde: None     // nadie lo apunta
    }));
    // creamos el nodo B
    let b = Rc::new(RefCell::new(Nodo {
        valor: "Nodo B".to_string(),
        hacia: Some(Rc::clone(&a)), // apunta hacia A, con strong reference, por lo que tiene ownership
        desde: None                 // nadie apunta hacia B
    }));
    
    // ahora haremos que A diga que B lo apunta
    // primero, debemos de tomar una referencia mutable
    // y acceder a su propiedad .desde
    // para asignarle el valor de B
    // como es un Option<T>, debemos usar Some()
    // y para no crear referencias cíclicas, usamos una weak reference 
    // esto lo hacemos con Rc::downgrade()
    a.borrow_mut().desde = Some(Rc::downgrade(&b));

    // si imprimimos, veremos que se crearon las referencias correctamente
    dbg!(&a);
    dbg!(&b.borrow().hacia);

    dbg!(&b);
    if let Some(desde) = a.borrow().desde.as_ref().and_then(|w| w.upgrade()) {
        dbg!(&desde);
    } else {
        println!("a.desde es None");
    }   // claro que acceder a .desde es un poco más complicado y requiere más cuidado
}

#[derive(Debug)]
struct Nodo {
    valor: String,
    hacia: Option<Rc<RefCell<Nodo>>>,
    desde: Option<Weak<RefCell<Nodo>>>
}