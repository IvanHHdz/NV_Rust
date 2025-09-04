# Punteros Inteligentes

Los punteros inteligentes son punteros que poseen ciertas capacidades que los vuelven especiales. Si bien este concepto no es único de Rust, debido a sus particularidades tienen ciertas diferencias esenciales con los punteros de otros lenguajes (como C++). 

Como sabemos, los punteros son algo común en Rust, se utilizan con `&` y apuntan a datos. Mientras que los punteros inteligentes no solo apuntan o "toman prestado" el valor de la variable a la que apuntan; sino que algunos "poseen" los datos.

Sin embargo, los punteros inteligentes también son sumamente comunes en Rust. De hecho, hemos utilizado punteros inteligentes desde el principio: las [Strings](../1.06%20-%20Datos%20Compuestos/DatosCompuestos.md), los [Vectores](../1.06%20-%20Datos%20Compuestos/DatosCompuestos.md), entre otros, cuentan como punteros inteligentes.

Crear punteros inteligentes normalmente se hace por medio de estructuras con traits especiales, como `Deref` y `Drop`. Para entender cómo funcionan, vamos a analizar un par de punteros que se incluyen en la librería estándar de Rust.

# `Box<T>`
El primer puntero inteligente del que hablaremos es box (`Box<T>`), el cual nos permite guardar información en la `heap` de la memoria en lugar de la `stack`.

Si bien, en muchos casos esto es completamente innecesario, resulta de mucha utilidad en situaciones concretas como cuando el tamaño de una estructura es desconocido en tiempo de compilación. Un uso muy interesante es el de crear tipos recursivos (tipos que pueden contenerse a sí mismos).

```rust
fn main() {
    let arbol = Nodo::nuevo(20);
    dbg!(arbol);
}

#[derive(Debug)]
enum Nodo {
    Hoja,
    Rama {
        rama_izq: Nodo,
        valor: i32,
        rama_der: Nodo,
    },
}

impl Nodo {
    fn nuevo(raiz: i32) -> Nodo {
        Nodo::Rama { 
            rama_izq: Nodo::Hoja, 
            valor: raiz, 
            rama_der: Nodo::Hoja 
        }
    }
}
```