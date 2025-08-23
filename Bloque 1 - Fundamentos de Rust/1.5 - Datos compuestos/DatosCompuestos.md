# Cadenas de Texto

Anteriormente hemos hablado un poco de las cadenas de texto. Sin embargo, el tema fue tratado de manera superficial, pues no era el tema central. Razón por la cual, como mencionábamos antes, volvemos a hablar de las cadenas de texto.

En Rust, como tal, existe un solo tipo de cadena de texto: las `str`. Estas usualmente se utilizan como referencias, razón por la cual se mencionó que su tipo era `&str` (una referencia a un dato tipo `str`). Por otro lado, tenemos el tipo `String`, el cual viene con la librería estándar de Rust (no en el lenguaje como tal), las cuales son mutables. Ambas utilizan codificación UTF-8 y son, en sí, colecciones de datos binarios.

Para crear una `String` tenemos varias formas:
```rust
fn main(){
    // por medio de String::new()
    let mut s1 = String::new();             // crea una String vacía
    
    // por medio del método .to_string() para str
    let mut s2 = "Jhonny Be ".to_string();      // crea una String con el valor de la str, en este caso "Jhonny Be "

    // utilizando la función String::from()
    let mut s3 = String::from("Ferris");    // el contenido inicial será "Ferris"
}
```

Al codificarse en UTF-8, tenemos la posibilidad de escribir cualquier tipo de carácter que sea posible en dicha codificación, lo que incluye prácticamente todos los carácteres que podríamos necesitar. 

Y como mencionábamos, la particularidad de las `String` es su capacidad de mutar. Podemos hacerlas mutar de las siguientes formas:
```rust
fn main(){
    let mut s1 = String::from("Ferris");
    let mut s2 = "Jhonny Be ".to_string();

    // añadir una str al final
    s1.push_str(" está contento");  // es posible también hacerlo con variables
    s1.push_str(" de tu");          // de tipo &str
    s1.push_str(" progreso ^^");    // tomando en cuenta que toma propiedad
    println!("{s1}");   // imprime "Ferris está contento de tu progreso ^^"

    // añadir un carácter al final
    s2.push('G');   // "Jhonny Be G"
    s2.push('o');   // "Jhonny Be Go"
    s2.push('o');   // "Jhonny Be Goo"
    s2.push('d');   // "Jhonny Be Good"
    s2.push('e');   // "Jhonny Be Goode"
    println!("{s2}")
}
```

También tenemos la capacidad de concatenar dos `String`, de dos formas distintas: con el simbolo `+` y con el macro `format!()`:
```rust
fn main(){
    let s1 = String::from("september");
    let s2 = String::from("25");
    let s3 = String::from("24");

    // usando +
    let s = s1 + &s2 + &s3; // cancatenamos tres Strings
    println!("{s}");        // imprime "september2524"
    // hay que tomar en cuenta que lo que hace es tomar la primer String
    // y concatenarle copias de las siguientes
    // por lo que s1 es inválida ahora.

    let s1 = String::from("september");

    // también podemos añadir str de por medio, para darle mejor formato
    let s = s1 + " " + &s2 + ", 20" + &s3;  
    println!("{s}");    // imprime "september 25, 2024"

    let s1 = String::from("september");

    // también podemos hacer lo mismo haciendo uso de format!()
    let s = format!("{s1} {s2}, 20{s3}");   // esta forma es más fácil de entender
    println!("{s}");    // imprime "september 25, 2024"
}
```

Ahora, una pequeña particularidad de Rust, indexar (acceder a un elemento, en este caso un carácter) una `String`. Posiblemente crea que se realiza de la misma forma que en otros lenguajes, es decir `cadena[0]` por ejemplo. Pues, eso no funciona en Rust.

No entraremos en detalles del porqué no existe la indexación para `String` en Rust. Solamente mencionaremos que es por la forma en que Rust guarda las `String`. Misma razón por la cual, si bien es posible, no es muy recomendado hacer uso de _slicing_ en `String`, por la posibilidad de hacerlo mal, lo que probocaría el programa se paniquee (`panicked`, es un _runtime error_).

Por otro lado, Rust nos provee de una herramienta que nos puede ser útil para indexar una `String`, que son los métodos `.chars()` y `.bytes()`:
```rust
fn main() {
    let s = String::from("¡Hola!");
    
    for c in s.chars() {
        println!("{c}");    // se mostrará en pantalla cada carácter de s
    }
}
```

# Arreglos

Uno de los dos tipos de datos compuestos (junto a las tuplas) que provee Rust por defecto. Estos funcionan de igual manera a como lo hacen los arreglos en lenguajes como C++: todos sus elementos deben ser del mismo tipo, tienen un tamaño fijo y se puede acceder a cada valor individual:

```rust
fn main(){
    // ejemplo de arreglo
    let arr = [1, 2, 3, 4, 5];  // en este caso, es de tipo: [i32; 5]

    for i in arr {              // podemos iterar sobre los arreglos
        println!("{i}");        // imprimirá cada elemento
    }

    // también podemos inicializarlos con valores
    let arr = [1; 5];           // [1, 1, 1, 1, 1]

    println!("{}", arr[0]);     // imprimirá el primer elemento

    let mut arr = [1; 5];       // también pueden ser mutables    

    arr[0] = 23;                // podemos modificar un elemento

    println!("{}", arr[0]);     // imprimirá 23
}
```

Como siempre, es importante cuidar de no acceder a datos fuera del arreglo. Por ejemplo, tratar de acceder al elemento `5` de un arreglo de `3`. Pues esto hará que el programa se paniquee (_runtime error_).

# Tuplas

Como mencionábamos antes, el segundo tipo de dato compuesto por defecto en Rust son las Tuplas. Este tipo es bastante usado y funciona igual que las tuplas de otros lenguajes como Python. Son varios datos ordenados (similar a los arreglos) que pueden ser de distintos tipos. Al igual que los arreglos, su tamaño es fijo:

```rust
fn main(){
    // ejemplo de una tupla
    let tup = (2_024, 9.17, 'A', "str", String::from("ID"));   
    // en este caso, sería de tipo: (i32, f64, char, &str, String)
    let tup = (1, 2, 3);    // y este sería (i32, i32, i32)

    // de igual forma, podemos extraer los datos
    let (x, y, z) = &tup;   
    // lo hacemos por referencia para evitar tomar propiedad, pero es igual de válido hacerlo

    // también podemos acceder a un solo elemento
    let x = tup.0;
}
```

También existe una tupla especial: `()`. Esta recibe el nombre de unidad (_unit_) y es el valor de retorno vacío, equivalente a `void` de C++.

# Vectores

Los vectores funcionan igual que en lenguajes como C++, C# o Java. Son colecciones de datos del mismo tipo que colocan los datos uno al lado de otro en memoria.

Crear un vector es similar a crear un `String`:
```rust
fn main(){
    // crear vectores
    let v: Vec<i32> = Vec::new();   
    // añadimos el tipo, pues está vacío y no puede inferir el tipo

    // también podemos crearlos haciendo uso del macho vec![]
    let v = vec![1, 2, 3, 4, 5];
}
```

En el caso de usar la macro `vec![]` el tipo se infiere. Para que funciones, claro, todos los datos dentro del arreglo que toma `vec![]` deben ser del mismo tipo.

Además, a diferencia de los arreglos, los vectores no tienen un tamaño fijo, sino dinámico. 
```rust
fn main(){
    let mut v =  Vec::new();    // para que funcione, debe ser mutable
    // si bien no puede inferir el tipo, usará Vec<T>, lo cual se hablará más adelante

    // añadir nuevos elementos al final
    v.push(1);  // infiere el tipo i32
    v.push(2);
    v.push(3);
    // ahora el valor del vector es: [1, 2, 3]
}
```

También podemos acceder a los valores individuales del vector de dos formas distintas: indexación y el método `.get()`:
```rust
fn main(){
    // creamos un vector
    let v = vec![1, 2, 3, 4, 5];

    // podemos acceder a sus valores de dos formas:
    // con indexación
    let a = &v[0];
    println!("{a}");

    // con el método .get()
    let a = &v.get(0);
    match a {
        Some(a) => println!("{a}"),
        None => println!("Index inválido!")
    }

    let a = &v.get(100);
    match a {
        Some(a) => println!("{a}"),
        None => println!("Index inválido!")
    }
}
```

Como podemos ver, usar indexación funciona igual que en otros lenguajes. Por otro lado, aplicar el método `.get()` parece más largo, pero tiene una explicación. Lo que ocurre es que `.get()` no retorna el valor en sí, sino un `Option<&T>` (del cual hablaremos más adelante). Razón por la cual, para poder usar el valor debemos hacerlo dentro de un `match` (de lo cual hablaremos más adelante) que considere todos los escenarios posibles (en este caso, que el index sea válido o que no lo sea).

La ventaja de usar `.get()` en lugar de simplemente indexar es la que se ve en el ejemplo: si tratamos de usar un index inválido el programa no crashea, sino que aborda el problema (similar a usar un `try-catch`). A diferencia de indexación, que si eso ocurre el programa se paniquea (crashea).

También es importante destacar el hecho que en ambos casos que se accede a un valor del vector, se hace por referencia. Por lo cual, como ya sabemos, si volvemos a usar la variable del vector, la variable de referencia deja de ser válida:
```rust
fn main(){
    let v = vec![1, 2, 3, 4, 5];
    let a = &v[10];
    println!("{a}");

    // añadimos un nuevo elemento
    v.push(6);  // a deja de ser válida

    // la siguiente línea produciría un error de compilación
    // pues se trata de acceder a una variable que ya no es válida
    // println!("{a}");
}
```

En cuanto a iterar los elementos de un vector, se hace de manera muy parecida a como se hace con los arreglos:
```rust
fn main(){
    let v = vec![1, 2, 3, 4, 5];
    for e in &v {           // podemos usar &mut v si necesitamos que sea mutable
        println!("{e}");    // imprimirá todos los elementos del vector v
    }
}
```

# Hash Maps

Funciona de manera parecida a como lo hacen los diccionarios en Python. Almacena un valor con una clave asociada. Cabe señalar que todas las claves deben ser del mismo tipo, al igual que todos los valores también.

Un pequeño detalle con respecto a los Hash Map es el hecho que sí debemos importarlos, a diferencia de los tipos visto hasta ahora, de la librería estándar desde `use::collections::HashMap`.

```rust
use std::collections::HashMap;

fn main(){
    // podemos crear un hash map vacío
    let mut hash_map = HashMap::new();
    // su tipo es definido como HashMap<K, V>
    // donde K y V son los tipos de las claves y los valores, respectivamente

    // podemos añadir valores al hash map con el método .insert()
    hash_map.insert(String::from("WW1"), 1914);
    hash_map.insert(String::from("WW2"), 1939);
    hash_map.insert(String::from("WW2"), 1939);

    // y podemos acceder a los valores usando las claves
    // por medio del método .get()
    let v = hash_map.get(&String::from("WW2"));

    // al igual que con vectores, el método .get()
    // retorna un Option<&T>
    // y para ver el valor, debemos usar match
    match v {
        Some(v) => println!("{v}"),
        None => println!("None")
    }
}
```

Es importante mencionar la cuestión de la propiedad de los valores dentro de un hash map: los que usan el `trait` de `Copy`, como `i32` se copia, los que no, como `String` se mueven y la propiedad la tiene el Hash Map.

Y al igual que con los vectores, podemos iterar sobre las entradas de un diccionario.
```rust
use std::collections::HashMap;

fn main(){
    let mut hash_map = HashMap::new();
    hash_map.insert(String::from("WW1"), 1914);
    hash_map.insert(String::from("WW2"), 1939);

    // iteramos sobre los pares de clave-valor
    for (k, v) in &hash_map {
        println!("{k} - {v}");
    }
}
```

Ahora bien, para actualizar un par de clave - valor, existen varias formas dependiendo del contexto:

```rust
use std::collections::HashMap;

fn main(){
    let mut hash_map = HashMap::new();
    hash_map.insert(String::from("WW1"), 1914);
    hash_map.insert(String::from("WW2"), 1940);

    // esta es una forma sencilla de imprimir el hash map
    println!("{hash_map:?}");   // {"WW1": 1914, "WW2": 1940}

    // podemos actializar desechando el valor existente
    hash_map.insert(String::from("WW2"), 1939);
    println!("{hash_map:?}");   // {"WW1": 1914, "WW2": 1939}

    // añadir un valor sólo si no existe ya
    hash_map.entry(String::from("WW1")).or_insert(1914);    // no hará nada, pues ya existe esa entrada
    println!("{hash_map:?}");   // {"WW1": 1914, "WW2": 1939}

    hash_map.entry(String::from("CW")).or_insert(1948);     // insertará el valor
    println!("{hash_map:?}");   // {"WW2": 1939, "WW1": 1914, "CW": 1948}

    // actualizar el valor en base al valor actual
    let years = hash_map.entry(String::from("WW1")).or_insert(0);
    // verifica si la clave existe; si no, inserta 0
    // retorna un &mut i32, es decir, una referencia mutable al valor en el HashMap

    *years *= -1;   // desreferenciamos la referencia mutable y modificamos el valor dentro del HashMap
    *years += 2025; // desreferenciamos de nuevo y sumamos 2025 al valor
    // se hace así porque years es &mut i32, no un i32 directo,
    // y para operar con i32 debemos desreferenciarlo con *

    println!("{hash_map:?}");   // {"WW1": 111, "WW2": 1939, "CW": 1948}
}
```

# `slice`

Similar a otros lenguajes, como Python, podemos guardar parte de una colección de datos en una variable. Con la particularidad de que, en el caso de Rust, estas "partes de colecciones" son referencias. A estas se les da el nombre de `slice`.

En Rust, la notación que utilizamos es `[a..b]` refiriéndonos a que tomaremos una parte de la collección que inicia en `a` y termina en `b` (no inclusivo). También podemos usar `[a..=b]`, cuya única diferencia es que incluye a `b`. También podemos omitir `a` para hacerlo desde el inicio, u omitir `b` para hacerlo hasta el final; o ambas, y lo hacemos desde el inicio hasta el final. Podemos realizarla en cualquier clase de colección que posea Rust.

```rust
fn main(){
    // ejemplo con arreglos
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &arr[3..7];
    println!("{slice:?}");  // [4, 5, 6, 7]

    // ejemplo con String
    let s = String::from("Toby Fox");
    let slice = &s[0..4];
    println!("{slice}");    // "Toby"

    // ejemplo con vectores
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &v[5..10];
    println!("{slice:?}");  // [6, 7, 8, 9, 10]
}
```
