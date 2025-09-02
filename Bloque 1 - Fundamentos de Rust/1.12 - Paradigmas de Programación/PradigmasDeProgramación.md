# Programación Funcional

Como dijimos al [principio de este curso](../1.01%20-%20Introducción/Introducción.md), Rust es un lenguaje multiparadigma. Lo que significa que toma elementos de varios paradigmas de programación. Uno de los que ha influenciado mucho en Rust es el paradigma funcional.

No entraremos a detalle a hablar de qué es o cómo se aplica el paradigma funcional, pues nos alargaríamos mucho. Resumiendo: Programación funcional es un paradigma de programación que se basa en el uso de funciones puras, inmutabilidad y evaluación de expresiones matemáticas, evitando efectos secundarios y estados mutables.

Ahora, nos centraremos en explicar cómo Rust incorpora elementos de la programación funcional dentro de sí.

## Closures
Las _Closures_ son funciones anónimas (sin nombre) de clausura (que guardan variables de su entorno).

De la misma forma que ocurre en otros lenguajes, en Rust podemos declarar este tipo de funciones, las cuales son similares a, por ejemplo, funciones `lambda` de Python.

Por ejemplo, imaginemos que deseamos crear una función que sume dos números. La forma habitual es de la siguiente manera:
```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}
```
Las closures son una forma simplificada de escribir esto mismo:
```rust
// la original puede escribirse en una línea
fn sumar(a: i32, b: i32) -> i32 {a + b}

// esta es una clousure
let sumar = |a: i32, b:i32| -> i32 {a + b};
// podemos simplificarla omitiendo tipos, pues Rust puede inferirlos aquí
let sumar = |a, b| {a + b};
// incluso esas llaves son innecesarias
let sumar = |a, b| a + b;
```
Veamosla en acción:
```rust
fn main() {
    let sumar = |a, b| a + b;           // declaramos la closure aqui
    let resultado = sumar(2, 2);        // aquí la llamamos
    println!("resultado = {resultado}");// imprimirá "resultado = 4"
} 
```
Ahora bien, que Rust infiera los tipos no significa que podemos usar cualquier tipo cuando queramos, eso provocaría errores. Rust infiere el tipo con la primera llamada de la función, por lo que si lo utilizamos con un tipo, no podemos utilizarla luego con otro. Por ejemplo:
```rust
fn main() {
    let sumar = |a, b| a + b;
    
    // la usamos con i32
    let resultado = sumar(2, 2);
    println!("resultado = {resultado}");

    // la usamos con f64
    let resultado = sumar(2.1, 2.3);
    println!("resultado = {resultado}");
} 
```
No compilará, pues nos dirá que la segunda está tratando de usar la función con `f64` cuando la función es `i32`.

Como habíamos mencionado anteriormente, las closures pueden tomar, guardar o capturar datos y variables del entorno donde se crean. Esto las vuelve muy útiles, y al mismo tiempo trae consigo muchas consideraciones. Lo más importante, ¿cómo captura dichos datos? Existen 3 formas en que puede capturar un dato: referencia inmutable, referencia mutable y tomando propiedad.

Veamos el primer caso, por ejemplo:
```rust
fn main() {
    // usaremos Strings para ejemplificar
    let s1 = String::from("cadena de texto");

    // creamos una closure que usará una referencia inmutable
    let print = || println!("{s1}");

    println!("{s1} antes");     // imprimimos antes
    print();                    // imprimimos en la función
    println!("{s1} después");   // imprimimos después
} 
```
Y esto compilará sin problemas, pues usa referencia inmutable. 

Por otro lado, veamos una referencia mutable. El siguiente ejemplo, no compilará, veamos porqué:
```rust
fn main() {
    let mut s1 = String::from("cadena de texto");   // se crea s1

    let mut print = || s1.push_str(" número 1.");   // print toma referencia mutable (mutable borrowing)

    println!("{s1} antes");                         // ! trata de tomar referencia inmutable, pero el borrowing de print aún no termian
    print();                                        // ultima llamada de print, termina el borrowing
    println!("{s1} después");
} 
``` 
Para solucionarlo, no hacemos la referencia inmutable antes:
```rust
fn main() {
    let mut s1 = String::from("cadena de texto");   // se crea s1

    let mut print = || s1.push_str(" número 1.");   // print toma referencia mutable (mutable borrowing)

    print();                                        // ultima llamada de print, termina el borrowing
    println!("{s1} después");                       // ahora sí puede hacer referencia inmutable
} 
```
Y por último, para poder mover la propiedad (_ownership_) de una variable a la función, debemos usar `move`. Esto de momento no es muy útil, pero tiene utilidades al ser utilizado con hilos, tema que veremos más adelante.
```rust
thread::spawn(move || println!("From thread: {list:?}"))    // ejemplo moviendo ownership
```

### Traits
Existen tres traits especiales que utilizan las funciones y sirven para que las mismas puedan ser pasadas como parámetros, lo cual es especialmenten útil con las closures. Estos traits son:
- `FnOnce` indica que la función puede ser llamada una vez. Todas las closures lo implementan por defecto. Aquellas closures que mueven los valores (_ownership_) dentro de sí solamente implementan este, pues solamente pueden ser llamadas una vez (puesto que los datos dejan de ser válidos tras la primera llamada). Ejemplo: `move || {println!("Consumí la cadena: {}", s);}`.
- `FnMut` para aquellas que pueden ser llamadas más de una vez. Aquellas closures que no mueven valores dentro (aunque puedan tener valores que cambien) lo implementan por defecto. Ejemplo: `|| {counter += 1; println!("Contador: {}", counter);}`.
- `Fn` indica que puede llamarse más de una vez y que no cambia o muta. Lo implementan aquellas closures que no capturan datos externos. Ejemplo: `|| {println!("El valor de x es: {}", x);}`.

En la librería estandar, tenemos declarado el método `.unwrap_or_else()` para `Option<T>`, el cual toma como parámetro una función. Veamos cómo lo hace:
```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```
Como podemos ver, el método toma una función tal que esta no reciva parámetros, tenga el trait de `FnOnce` y retorne un valor de tipo `T`.

## Iterator
Un iterador nos permite realizar una tarea sobre un conjunto de elementos uno a la vez. Al usarlos, no es necesario implementar dicha lógica por nosotros, sino que la implementa el iterador.

Por ejemplo:
```rust
fn main(){
    // creamos un vector para ejemplificar
    let v = vec![1, 2, 3, 4, 5];

    // creamos un iterador del vector
    let v_i = v.iter();

    // usamos el iterador en un bucle for
    for i in v_i {
        println!("number {i}");
    }
}
```
Sin embargo, los iteradores son mucho más complejos de lo que puede parecer a simple vista o con ejemplos como el anterior.

Todos los iteradores implementan el trait de `Iterator`, el cual es definido en la librería estándar de la siguiente manera (más o menos):
```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // otros métodos pre-definidos
}
```
Notemos que `type Item;` es algo nuevo, pues de momento no lo hemos visto. Lo veremos luego. De momento, lo importante es saber que esto significa que el trait necestia que especifiquemos un tipo y que ese mismo tipo será el que retorne el método `.next()`. Es importanten también mencionar que el trait de `Iterator` solamente necesita que se le implemente el método `.next()`, el cual retorna un `Some(Item)` o, cuando termina el iterador, un `None`.

Como dato extra, podemos llamar directamente el método `.next()` en un iterador, como el que definimos antes. Aunque es importante notar que debemos volver mutable al iterador. Esto debido a que `.next()` realiza cambios internos sobre los iteradores. En caso de llamarlo para otras cosas, como para un bucle `for`, no es necesario volverla mutable debido a que el propio bucle toma el _ownership_ sobre el iterador. Y de igual forma, vale la pena mencionar que el iterador es una referencia inmutable del objeto del que se crea (por ejemplo, en el caso anterior, de vector `v`). Para volverlo mutable o que tome el _ownershio_ del objeto debemos hacerlo con `.iter_mut()` o `.into_iter()` respectivamente.

### Métodos que consumen el iterador o producen uno nuevo

Cuando un método consume el iterador, se le suele llamar _consuming adapters_. Esto ocurre ya que existen muchos métodos definidos por defecto que necesitan tomar _ownership_ de los elementos del iterador al utilizar `.next()`. Por lo cual van consumiendo al iterador. Un ejemplo de estos es el método `.sum()`;
```rust
fn main(){
    // creamos un vector para ejemplificar
    let v = vec![1, 2, 3, 4, 5];
    // creamos un iterador del vector
    let v_i = v.iter();
    let suma: i32 = v_i.sum();   
    // a partir de aquí v_i deja de ser válido, pues fue consumido por .sum()
    println!("la suma es: {suma}");
}
```
Por otro lado, aquellos métodos que retornan un nuevo iterador al ejecutarse son conocidos como _iterator adapters_, consumiendo el original en el proceso. Un ejemplo de estos sería el método `.map()` que ejecuta una closure sobre cada elemento del iterador. Por ejemplo:
```rust
fn main(){
    // creamos un vector para ejemplificar
    let v = vec![1, 2, 3, 4, 5];
    // creamos un iterador del vector
    let v_i = v.iter();
    let cuadrado = v_i.map(|x| x*x);   
    // aquí v_i no es válido, pues fue consumido por .map()
    for i in cuadrado {
        println!("{i}");
    }
}
```
También podemos pasar el iterador a una estructura. Por ejemplo:
```rust
fn main(){
    // creamos un vector para ejemplificar
    let v = vec![1, 2, 3, 4, 5];
    // creamos un iterador del vector
    let v_i = v.iter();
    let cuadrado: Vec<_> = v_i.map(|x| x*x).collect();   
    
    println!("{cuadrado:?}");
}
```
Otro método muy útil es `.filter()` el cual funciona de manera similar a `.map()`, con la direfencia que utiliza una closure que retonar un valor booleano. De esta forma "filtra" los elementos del iterador original. Aquellos que retornan `true` en el closure se quedan en el nuevo, los que retornan `false` no se incluyen.

# Programación Orientada a Objetos

Cap. 18