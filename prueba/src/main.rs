fn main(){
    let persona = crear_persona!("Juan", 30, "Calle Falsa 123", "555-1234", "juan@example.com");
    println!("{:?}", persona);

    let persona = crear_persona!("Pablo", 41);
    println!("{:?}", persona);
}

#[derive(Debug)]
struct Persona {
    nombre: String,
    edad: u8,
    direccion: String,
    telefono: String,
    email: String,
}

#[macro_export]
macro_rules! crear_persona {
    ($nombre:expr, $edad:expr, $direccion:expr, $telefono:expr, $email:expr) => {
        Persona {
            nombre: $nombre.to_string(),
            edad: $edad,
            direccion: $direccion.to_string(),
            telefono: $telefono.to_string(),
            email: $email.to_string(),
        }
    };
    ($nombre:expr, $edad:expr) => {
        Persona {
            nombre: $nombre.to_string(),
            edad: $edad,
            direccion: "N/A".to_string(),
            telefono: "N/A".to_string(),
            email: "N/A".to_string(),
        }
    };
}