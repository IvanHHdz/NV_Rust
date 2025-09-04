fn main() {
    let mut arbol: Nodo<i32> = Nodo::nuevo(20);

    println!("Agregamos 23, 13, 7, 16");

    arbol.agregar(23);
    arbol.agregar(13);
    arbol.agregar(7);
    arbol.agregar(16);

    dbg!(&arbol);
}

#[derive(Debug, PartialEq)]
enum Nodo<T> {
    Hoja,
    Rama {
        nodo: T,
        rama_izq: Box<Nodo<T>>,
        rama_der: Box<Nodo<T>>,
    },
}

impl <T: PartialOrd> Nodo<T> {
    fn nuevo(raiz: T) -> Nodo<T> {
        Nodo::Rama { 
            rama_izq: Box::new(Nodo::Hoja), 
            nodo: raiz, 
            rama_der: Box::new(Nodo::Hoja)
        }
    }

    fn agregar (&mut self, valor: T){
        match self {
            Nodo::Hoja => {
                *self = Nodo::Rama { 
                    nodo: valor, 
                    rama_izq: Box::new(Nodo::Hoja), 
                    rama_der: Box::new(Nodo::Hoja)
                }
            },
            Nodo::Rama { rama_izq, nodo, rama_der } => {
                if valor < *nodo {
                    rama_izq.agregar(valor);
                }
                else {
                    rama_der.agregar(valor);
                }
            }
        }
    }
}