fn main() {
    let mut arbol: Nodo<i32> = Nodo::nuevo(20);

    let datos = [1, 4, 21, 6, 21, 23, 24, 3, 13, 5, 17, 4, 9, 32, 5];
    println!("Agregamos datos : {datos:?}");

    for ele in datos {
        arbol.agregar(ele);
    }

    let arbolito = arbol.recorrer();
    println!("{arbolito:?}");
    
    let valor = arbol.buscar(&23);
    println!("23 {valor}")
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
                else if valor > *nodo {
                    rama_der.agregar(valor);
                }
            }
        }
    }

    fn recorrer(&self) -> Vec<&T> {
        let mut arbol: Vec<&T> = Vec::new();
        match self {
            Nodo::Hoja => arbol,
            Nodo::Rama { nodo, rama_izq, rama_der } => {
                arbol.extend(rama_izq.recorrer());
                arbol.push(nodo);
                arbol.extend(rama_der.recorrer());
                arbol
            }
        }
    }

    fn buscar(&self, dato: &T) -> bool {
        match self {
            Nodo::Hoja => false,
            Nodo::Rama { nodo, rama_izq, rama_der } => {
                if nodo == dato{
                    true
                }
                else {
                    rama_izq.buscar(dato) || rama_der.buscar(dato)
                }
            }
        }
    }
}