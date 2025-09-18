fn main(){
    let usuario = Persona {
        nombre: String::from("Ivan"),
        edad: 20,
    };
    match usuario {
        Persona { nombre, edad: e @ 18..=25 } => {
            println!("Hola {}, eres un adulto joven de {} años.", nombre, e);
        }
        Persona { nombre, edad: e @ 0..=17 } => {
            println!("Hola {}, eres menor de edad, solo tienes {} años.", nombre, e);
        }
        Persona { nombre, edad: e @ 26..=65 } => {
            println!("Hola {}, eres un adulto de {} años.", nombre, e);
        }
        Persona { nombre, edad: e @ 66..=120 } => {
            println!("Hola {}, eres un adulto mayor de {} años.", nombre, e);
        }
        _ => {
            println!("Hola, no se pudo determinar tu grupo de edad.");
        }
    }
}

struct Persona {
    nombre: String,
    edad: u8,
}