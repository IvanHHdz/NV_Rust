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

# `Rc<T>`

Hasta el momento, hemos visto que cada valor tiene un propietario. Esto, en general, es así. Sin embargo, existen casos muy específicos en los cuales se vuelve necesario o útil es hecho que un mismo dato pueda tener multiples propietarios. Para lograr esto, utilizamos el puntero `Rc<T>` (por _reference counting_). Este puntero mantiene un seguimiento de las referencias que se hacen a un dato, para que, si no quedan más referencias, se libere el dato.

Por ejemplo, veamos el siguiente ejemplo. Imaginemos que tenemos tres ciudades: Ciudad A, Ciudad B y Ciudad C. Las ciudades C y B conectan con la ciudad A por medio de caminos: la ciudad A no conecta con nadie. Podemos vizualizarlo de la siguiente manera: 
```rust
use std::rc::Rc;    // importamos Rc<T>

fn main(){
    // creamos la ciudad A
    let ciudad_a = Rc::new(Ciudad::nueva("A".to_string(), Camino::NoCamino));   // no tiene caminos
    println!("Contador de referencias = {}", Rc::strong_count(&ciudad_a));      // imprimimos las referencias, solo tiene una (la de la variable ciudad_a)

    // creamos la ciudad B
    let ciudad_b = Rc::new(Ciudad::nueva("B".to_string(), Camino::CaminoHacia(Rc::clone(&ciudad_a))));  // esta conecta con la ciudad A
    println!("Contador de referencias = {}", Rc::strong_count(&ciudad_a));      // volvemos a imprimir las referencias, ahora son 2
    
    {   // podemos hacer esto dentro de bloques
        // creamos la ciudad C
        let ciudad_c = Rc::new(Ciudad::nueva("C".to_string(), Camino::CaminoHacia(Rc::clone(&ciudad_a))));  // esta también conecta con la ciudad A
        println!("Contador de referencias = {}", Rc::strong_count(&ciudad_a));      // imprimimos una vez más las referencias, ahora son 3
    }
    
    // si verificamos fuera del bloque, notaremos que ahora solo son 2 referencias, ya que la tercera (que era de ciudad_c) ya no es válida, pues estamos fuera de su alcance
    println!("Contador de referencias = {}", Rc::strong_count(&ciudad_a));
    
}

// enum para los caminos
enum Camino {
    NoCamino,
    CaminoHacia(Rc<Ciudad>)
}

// estructura para las ciudades
struct Ciudad {
    nombre: String, // cada ciudad tiene un nombre
    camino: Camino  // y un camino
}

// la función para crear ciudades
impl Ciudad {
    fn nueva(nombre: String, camino: Camino) -> Self{
        Self { 
            nombre, 
            camino 
        }
    }
}
```

Es importante mencionar que estas son referencias inmutables. Así que, ¿cómo hacemos para crear referencias mutables de este estilo?

# `RefCell<T>`

Para lograr implementar multiples referencias mutables primero necesitamos analizar porqué no funcionará hacerlo con `Rc<T>`. Veamos el siguiente ejemplo (no compila):
```rust
use std::rc::Rc;    // importamos Rc<T>

fn main(){
    // creamos la ciudad A
    let ciudad_a = Rc::new(Ciudad::nueva("A".to_string(), Camino::NoCamino));   // no tiene caminos
    println!("Contador de referencias = {}", Rc::strong_count(&ciudad_a));      // imprimimos las referencias, solo tiene una (la de la variable ciudad_a)

    // creamos la ciudad B
    let ciudad_b = Rc::new(Ciudad::nueva("B".to_string(), Camino::CaminoHacia(Rc::clone(&ciudad_a))));  // esta conecta con la ciudad A
    println!("Contador de referencias = {}", Rc::strong_count(&ciudad_a));      // volvemos a imprimir las referencias, ahora son 2
        
    ciudad_b.mostrar();     // La ciudad B tiene un camino hacia A.

    // queremos cambiar el nombre de ambas ciudades
    ciudad_a.nombre = "París".to_string();
    ciudad_b.nombre = "Roma".to_string();

    ciudad_b.mostrar();     // queremos que imprima: "La ciudad Roma tiene un camino hacia París."
}

// enum para los caminos
#[derive(Debug)]
enum Camino {
    NoCamino,
    CaminoHacia(Rc<Ciudad>)
}

// estructura para las ciudades
#[derive(Debug)]
struct Ciudad {
    nombre: String, // cada ciudad tiene un nombre
    camino: Camino  // y un camino
}

// la función para crear ciudades
impl Ciudad {
    fn nueva(nombre: String, camino: Camino) -> Self{
        Self { 
            nombre, 
            camino 
        }
    }

    // implementamos una función para mostrar
    fn mostrar(&self) {
        let camino = match &self.camino {
            Camino::NoCamino => &"ninguna parte".to_string(),
            Camino::CaminoHacia(ciudad) => &(ciudad.nombre)
        };
        println!("La ciudad {} tiene un camino hacia {}.", self.nombre, camino);
    }
}
```

Nos dirá, entre otras cosas: `cannot mutate immutable variable 'ciudad_a'`. Esto ocurre debido a que el compilador de rust es incapaz de asegurar que lo que tratamos de hacer es válido, debido a las reglas del _ownership_. Y agregarle `mut` a la declaración de la variable tampoco funcionará, pues no implementa mutabilidad. Entonces, ¿qué hacemos?

Para solucionar esto, debemos de utilizar un puntero que nos ayudará a _mutar una variable inmutable_, esto por medio de _Interior mutability_, lo cual nos permite mutar datos incluso si estos tienen referencias inmutables (como sabemos, esto no está permitido por defecto). Para lograrlo, utiliza lo llamado `unsafe`, una opción avanzada que veremos más adelante, la cual nos permite, por así decirlo, tomar la responsabilidad que tiene el compilador de verificar el código, de forma que el compilador no intervendrá. Esto puede ocasionar muchos problemas si se utiliza mal, o ayudar a crear cosas que serían imposibles (como esto) de otra forma.

Este puntero es `RefCell<T>`. Veamos cómo implementarlo:
```rust
use std::{cell::RefCell, rc::Rc};    // importamos Rc<T> y RefCell<T>

fn main(){
    // creamos la ciudad A
    let ciudad_a = Rc::new(Ciudad::nueva("A".to_string(), Camino::NoCamino));   // no tiene caminos
    println!("Contador de referencias = {}", Rc::strong_count(&ciudad_a));      // imprimimos las referencias, solo tiene una (la de la variable ciudad_a)

    // creamos la ciudad B
    let ciudad_b = Rc::new(Ciudad::nueva("B".to_string(), Camino::CaminoHacia(Rc::clone(&ciudad_a))));  // esta conecta con la ciudad A
    println!("Contador de referencias = {}", Rc::strong_count(&ciudad_a));      // volvemos a imprimir las referencias, ahora son 2
        
    ciudad_b.mostrar();     // La ciudad B tiene un camino hacia A.

    // queremos cambiar el nombre de ambas ciudades
    *ciudad_a.nombre.borrow_mut() = "París".to_string();    // debemos de utilizar el método .borrow_mut() para la referencia mutable
    *ciudad_b.nombre.borrow_mut() = "Roma".to_string();     // que desreferenciamos para editar su valor

    ciudad_b.mostrar();     // queremos que imprima: "La ciudad Roma tiene un camino hacia París."
    // ahora sí lo imprime
}

// enum para los caminos
#[derive(Debug)]
enum Camino {
    NoCamino,
    CaminoHacia(Rc<Ciudad>)
}

// estructura para las ciudades
#[derive(Debug)]
struct Ciudad {
    // hay que recordar que debemos cambiar su tipo
    nombre: RefCell<String>, // cada ciudad tiene un nombre
    camino: Camino  // y un camino
}

// la función para crear ciudades
impl Ciudad {
    fn nueva(nombre: String, camino: Camino) -> Self{
        Self { 
            // hay que modificar esta parte para crear nuevos
            nombre: RefCell::new(nombre), 
            camino 
        }
    }

    fn mostrar(&self) {
        let camino = match &self.camino {
            Camino::NoCamino => &"ninguna parte".to_string(),
            Camino::CaminoHacia(ciudad) => &(*ciudad.nombre.borrow())   // para imprimirlo, debemos tomarlo prestado
        };
        println!("La ciudad {} tiene un camino hacia {}.", self.nombre.borrow(), camino);
    }
}
```

# `Weak<T>`

Es posible que haya notado una cosa curiosa, y mala, de los punteros `Rc<T>`: la posibilidad de pérdida de memoria por culpa de ellos mismos. La razón es el hecho que la memoria de estos punteros no será liberada hasta que el contador de referencias (`Rc::strong_count()`) se vuelva 0. El problema es que haciendo uso también de `RefCell<T>` nos es posible crear referencias cíclicas. Es decir, un nodo `A` que apunta a un nodo `B`; y este nodo `B` apunta al nodo `A`. De esta forma ninguno de los dos nodos será eliminado, pues su contador no llegará a 0.

Esto no lo podrá detectar el compilador, por lo que es nuestra tarea solucionarlo.

Una forma de evitar este tipo de cosas es haciendo uso de una herramienta: otro puntero. Este puntero es `Weak<T>`. Veamos un ejemplo:
```rust
use std::{cell::RefCell, rc::Rc};   // importaciones necesarias

fn main(){
    // crearemos un nodo A
    let a = Rc::new(RefCell::new(Nodo {
        valor: "Nodo A".to_string(),
        hacia: None
    }));

    // crearemos un nodo B que apuntará a A
    let b = Rc::new(RefCell::new(Nodo {
        valor: "Nodo B".to_string(),
        hacia: Some(Rc::clone(&a))
    }));

    // ambos serán punteros que pueden apuntar a multiples datos y ser mutables
    
    dbg!(&b);

    dbg!(&a);                   // estos dos son iguales
    dbg!(&b.borrow().hacia);    // es decir, apuntan a lo mismo
    // por lo que podemos decir que podemos acceder a A desde B
}

#[derive(Debug)]
struct Nodo {
    valor: String,
    hacia: Option<Rc<RefCell<Nodo>>>,
}
```

Sin embargo, si hacemos que `A` apunte de la misma forma a `B`, crearemos una referencia cíclica, y no queremos eso. Entonces utilizaremos un puntero `Weak<T>` para esto:
```rust
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
```