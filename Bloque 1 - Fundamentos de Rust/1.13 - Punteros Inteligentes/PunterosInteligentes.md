# Punteros Inteligentes

Los punteros inteligentes son punteros que poseen ciertas capacidades que los vuelven especiales. Si bien este concepto no es único de Rust, debido a sus particularidades tienen ciertas diferencias esenciales con los punteros de otros lenguajes (como C++). 

Como sabemos, los punteros son algo común en Rust, se utilizan con `&` y apuntan a datos. Mientras que los punteros inteligentes no solo apuntan o "toman prestado" el valor de la variable a la que apuntan; sino que algunos "poseen" los datos.

Sin embargo, los punteros inteligentes también son sumamente comunes en Rust. De hecho, hemos utilizado punteros inteligentes desde el principio: las [Strings](../1.06%20-%20Datos%20Compuestos/DatosCompuestos.md), los [Vectores](../1.06%20-%20Datos%20Compuestos/DatosCompuestos.md), entre otros, cuentan como punteros inteligentes.

Crear punteros inteligentes normalmente se hace por medio de estructuras con traits especiales, como `Deref` y `Drop`. Para entender cómo funcionan, vamos a analizar un par de punteros que se incluyen en la librería estándar de Rust.

# `Box<T>`
El primer puntero inteligente del que hablaremos es box (`Box<T>`), el cual nos permite guardar información en la `heap` de la memoria en lugar de la `stack`.

Si bien, en muchos casos esto es completamente innecesario, resulta de mucha utilidad en situaciones concretas como cuando el tamaño de una estructura es desconocido en tiempo de compilación. Un uso muy interesante es el de crear tipos recursivos (tipos que pueden contenerse a sí mismos).

Por ejemplo, una ejercicio clásico de algoritmos para C++ es crear un árbol binario. En C++ esto se logra haciendo uso de punteros. Para lograr algo similar en Rust, debemos utilizar `Box<T>`. Esto, debido a cómo funcionan las referencias y la creación de estructuras, pues al decir que una estructura "apunta" a otra de su mismo tipo sería como decir que, por así decirlo, un nodo del árbol almacena otro nodo de arbol que almacena otro nodo de árbol...

Por lo que Rust no puede saber cuanta memoria necesitamos, pues para Rust algo así necesita memoria infinita. Entonces, en lugar de hacerlo así, usamos un puntero inteligente. Este, al tener un tamaño conocido no presenta problemas para Rust, pues verá que dicho nodo solo posee un `Box<T>` de tamaño conocido.

En el ejemplo siguiente vemos cómo se implementaría un árbol. Solamente agregando el método para crear y añadir, para simplificar.

```rust
fn main() {
    let arbol = Arbol::nuevo(20);    // creamos el árbol
    arbol.agregar(16);              // añadimos el 16
    arbol.agregar(23);              // añadimos el 23
    dbg!(arbol);                    // lo mostramos en pantalla
    // veremos algo que más o menos representa:
    /*
            20
          /   \
         16   23
    */
}

#[derive(Debug)]                    // esto nos permitirá imprimirlo
enum Arbol {                        // utilizaremos un enum para esto
    Hoja,                           // las hojas representan el final, cumplen la función de los null de C++
    Rama {                          // las ramas son nodos
    // debemos hacerlo con Box<i32> porque sí podemos conocer el tamaño de un Box<i32> en tiempo de compilación
    // de esta forma, Rust sabrá que no es de tamaño infinito
        rama_izq: Box<Arbol<i32>>,  // estos nodos tienen un lado izquiero
        nodo: i32,                  // un valor, en este caso será i32 para simplificar
        rama_der: Box<Arbol<i32>>,  // y un lado derecho
    },                              // nota: se puede generalizar con un dato genérico
}
// implementamos los métodos
impl Arbol {
    fn nuevo(raiz: i32) -> Arbol {  // para crear uno nuevo necesitamos el valor de la raiz
        Arbol::Rama {               // este irá a un nodo rama
            rama_izq: Arbol::Hoja,  // que tendrá hojas (valores nulos) a ambos lados
            valor: raiz, 
            rama_der: Arbol::Hoja 
        }
    }
    fn agregar(&mut self, valor: i32){  // para agregar también necesitamos un valor, y una referencia mutable al árbol
        match self {                    // con match verificamos si es una rama o una hoja
            Arbol::Hoja => {            // si bien, nuestro árbol no tendría que ser solo una hoja, es muy útil hacer la parte de la hoja para usar recursividad
                *self = Arbol::Rama {   // si es una hoja, la convertimos en rama con hojas a ambos lados
                    nodo: valor,                        // añadimos el valor
                    rama_izq: Box::new(Arbol::Hoja),    // y añadimos hojas a ambos lados
                    rama_der: Box::new(Arbol::Hoja)     // es importante recordar que no será necesario usar match para sacarlo, pues no es un enum como Result<T, E> o con Option<T>
                }
            },
            Arbol::Rama { rama_izq, nodo, rama_der } => {   // en caso que no sea una hoja, sino una rama, entonces tendremos que verificar si el valor iría a la izquierda o derecha
                if valor < *nodo {
                    rama_izq.agregar(valor);    // en cualquiera de los dos casos
                }                               // gregaremos el valor de forma recursiva
                else if valor > *nodo {
                    rama_der.agregar(valor);
                }
            }
        }
    }
}
```

# `Deref`

`Deref` es un trait que nos permite personalizar el comportamiento de un puntero (un puntero inteligente que nosotros creamos, por ejemplo) al utilizar el operador de desreferencia `*` (no el de multiplicación, el de desreferencia).

Para entenderlo mejor, primero vamos a ver cómo funciona el desreferenciar una referencia. Hay que recordar que en Rust es **sumamente** cuidadoso y exigente con los tipos. Por ejemplo, si comparamos un valor con su referencia, ni siquiera compilará:
```rust
fn main(){
    let valor = 23;             // tipo i32
    let referencia = &valor;    // tipo &i32

    if valor == referencia {    // no es valido, pues no es posible compara un i32 con un &i32
        println!("Son iguales!");
    }
    else {
        println!("No son iguales!");
    }
}
```

Para poder solucionar esto, debemos **desreferenciar** la referencia. Esto lo logramos con el simbolo `*` de manera similar a como se hace en lenguajes como C++:
```rust
fn main(){
    let valor = 23;             // tipo i32
    let referencia = &valor;    // tipo &i32

    if valor == *referencia {   // ahora sí será válido, pues es una comparación de i32 con i32
    // pues un *(&i32) es un i32 
        println!("Son iguales!");   // imprimiá que son iguales
    }
    else {
        println!("No son iguales!");
    }
}
```

Y lo mismo ocurre al utilizar ciertos punteros, como el visto anteriormente `Box<T>`:
```rust
fn main(){
    let valor = 23;
    let referencia = Box::new(valor);   // recordemos que Box<T> es un puntero (inteligente)

    if valor == *referencia {           // si le quitamos el * dejará de funcionar
    // pues trataría de comparar un i32 con un Box<i32>
        println!("Son iguales!");
    }
    else {
        println!("No son iguales!");
    }
}
```

La diferencia escencial entre los dos casos es que en este último crea una instancia de `Box<T>` que apunta a un valor copiado de `valor`, en lugar de hacer referencia directamente a `valor`. Pero claro, hay que recordar que `Box<T>` sigue siendo una estructura después de todo, por lo que nosotros también podemos hacer que nuestros punteros puedan comportarse así. Y es justamente lo que veremos a continuación.

Consideremos el siguiente ejemplo:
```rust
fn main(){
    let puntero = Puntero::nuevo(23);
    dbg!(&puntero);
}

#[derive(Debug)]
struct Puntero<T>(T);   // este será nuestro nuevo puntero inteligente

impl<T> Puntero<T> {
    fn nuevo(valor: T) -> Self {    // método para instanciar
        Self(valor)
    }
}
```

Sin embargo, si tratamos de usar el operador de desreferencia `*` no funcionará, pues necesitamos darle el trait `Deref`. Este trait le dirá al compilador cómo debe manejar nuestra variable cuando se intenta desreferenciar. Veamos un ejemplo:
```rust
use std::ops::Deref;

fn main(){
    let puntero = Puntero::nuevo(23);
    dbg!(&puntero);
}

#[derive(Debug)]
struct Puntero<T>(T);   // este será nuestro nuevo puntero inteligente

impl<T> Puntero<T> {
    fn nuevo(valor: T) -> Self {    // método para instanciar
        Self(valor)
    }
}

impl<T> Deref for Puntero<T> {
    type Target = T;        // veremos esto más adelante

    // esta será la función que se llamará al usar *
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

Ahora, usamos `*puntero` el compilador lo interpretará como `*puntero.deref()` y nos dará el valor de `23` en este caso.

Ocurre algo curioso con los punteros, y es que si son pasados como referencia a una función esta los aceptará si el valor dentro de ellos es del tipo que la función utiliza. Es decir, si tenemos una función como la siguiente:
```rust
fn suma(a: &i32, b: &i32) -> i32{
    *a + *b
}
```
Podemos llamarla de la siguiente forma:
```rust
fn main(){
    let puntero = Puntero::nuevo(23);
    let resultado = suma(&*puntero, &puntero);
    println!("{resultado}");
}
```
Y será válido. El primero, `&*puntero`, será válido porque estamos desreferenciando y luego volviendolo una referencia. Mientras que en el segundo caso estamos pasando la referencia de un puntero, lo cual (debido a algo llamado _deref coercion_) hace que pasemos implícitamente el dato dentro del puntero.

Y de manera similar, también podemos usar el trait de `DerefMut` para hacer un puntero que interactue como una referencia mutable `&mut`.

# `Drop`

Otro trait que es importante mencionar es `Drop`, que nos permite customizar lo que ocurrirá cuando la variable salga de su alcance y sea dropeada.

En algunos lenguajes, es necesario hacer manualmente la liberación de memoria. Esto ya que es algún tipo de dato especial, por así decirlo (como los que manipulan archivos). En Rust, basta con implementar `Drop` y el compilador se encargará de ejecutar el código que le especifiquemos cada vez que una variable de dicho tipo tenga que ser liberada.

```rust
use std::ops::Deref;

fn main(){
    let puntero = Puntero::nuevo(23);
    let resultado = suma(&*puntero, &puntero);
    println!("{resultado}");
}   // aquí se llamará drop

#[derive(Debug)]
struct Puntero(i32);   // este será nuestro nuevo puntero inteligente

impl Puntero{
    fn nuevo(valor: i32) -> Self {    // método para instanciar
        Self(valor)
    }
}

impl Drop for Puntero {     // implementamos Drop
    fn drop(&mut self) {
        // este código se ejcutará cuando se libere la memoria de este dato
        println!("Liberando el valor de {:?}", self.0);
    }
}
```

Sin embargo, esto ocasiona automáticamente que no nos sea posible utilizar el método `.drop()` para liberar la memoria de forma anticipada. Esto debido a que Rust seguiría llamando el método al salir del alcance, lo que ocasionaría una doble liberación de memoria (problema). Por esta razón, Rust no nos permite utilizar el método `.drop()` pero sí una función especialmente diseñada para este escenario: `drop()`:
```rust
fn main(){
    let puntero = Puntero::nuevo(23);           // creamos el puntero
    let resultado = suma(&*puntero, &puntero);  // lo usamos
    println!("{resultado}");
    drop(puntero);                              // lo liberamos
    println!("Después del drop!");
}
```