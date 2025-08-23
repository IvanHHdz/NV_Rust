# Estructuras

Las estructuras o `Struct` de Rust funcionan de manera similar a como en otros lenguajes son las clases. 

Funcionan de manera similar a como funcionan las tuplas: podemos tener datos de distintos tipos agrupados en una misma variable. La diferencia con las tuplas es el hecho que podemos darle nombre a cada una de esas variables. Esto significa que, por ejemplo, si la llavamos `usuario` podemos tener una una variable con nombre `nombre` dentro de la estructura, y en lugar de acceder a ella como `usuario.0` accedemos a ella como `usuario.nombre`.

Ejemplo:
```rust
fn main(){
    // ejemplo como tupla
    let usuario1 = ("Ernesto", "Worthing", 28, "Londres");
    println!("Tupla:");
    println!("Nombre completo - {} {}", usuario1.0, usuario1.1);
    println!("Edad - {}", usuario1.2);
    println!("Ciudad - {}", usuario1.3);
    /*
        Tupla:
        Nombre completo - Ernesto Worthing
        Edad - 28
        Ciudad - Londres
    */

    // ejemplo con estructura
    let usuario2 = Usuario {
        nombre: "Ernesto".to_string(),
        apellido: "Worthing".to_string(),
        edad: 28,
        ciudad: "Londres".to_string()
    };

    println!("\nEstructura:");
    println!("Nombre completo - {} {}", usuario2.nombre, usuario2.apellido);
    println!("Edad - {}", usuario2.edad);
    println!("Ciudad - {}", usuario2.ciudad);
    /*
        Estructura:
        Nombre completo - Ernesto Worthing
        Edad - 28
        Ciudad - Londres
    */
}

// aquí definimos la estructura
struct Usuario {        // utilizamos struct para declararlas
    nombre: String,
    apellido: String,
    edad: i32,
    ciudad: String
}
```

Como podemos ver, las estructuras se declaran fuera del `main()` para poder acceder a ellas desde cualquier parte del programa. Debemos declarar el tipo de cada una de las variables, y al crear una **instancia** o copia de la estructura usamos una sintaxis similar a la que usamos al declarar la estructura. Y a la hora de instanciar la estructura, debemos llenar todos los parámetros. También podemos volverla mutable con `mut` como cualquier otra variable, permitiendo cambiar las variables del mismo.

Aunque, como podemos ver, el crear una instancia de puede ser un poco largo, por lo que lo podemos reducir al escribir una función que retorne la instancia de la estuctura, facilitando su creación:
```rust
fn main(){
    // ejemplo con una mutable
    let mut usuario = crear_usuarios("Ernesto".to_string(), "Worthing".to_string(), 28, "Londres".to_string());
    
    // ejemplo modificando un valor
    usuario.apellido = "Moncrieff".to_string();
    
    println!("Nombre completo - {} {}", usuario.nombre, usuario.apellido);
    println!("Edad - {}", usuario.edad);
    println!("Ciudad - {}", usuario.ciudad);

    /*
        Nombre completo - Ernesto Moncrieff
        Edad - 28
        Ciudad - Londres
    */
}

// esta función llenará los datos, nos retornará una instancia
fn crear_usuarios(nombre: String, apellido: String, edad: i32, ciudad: String) -> Usuario {
    Usuario {
        nombre: nombre,
        apellido: apellido,
        edad: edad,
        ciudad: ciudad
    }
}

// aquí definimos la estructura
struct Usuario {
    nombre: String,
    apellido: String,
    edad: i32,
    ciudad: String
}
```

Notemos que en la función `crear_usuarios()` las variables tienen el mismo nombre que sus respectivas variables en la estructura, por lo que se repiten. Rust nos ayuda a simplificar eso cuando esto ocurre, reescribiendo la función como:
```rust
fn crear_usuarios(nombre: String, apellido: String, edad: i32, ciudad: String) -> Usuario {
    Usuario {
        nombre,
        apellido,
        edad,
        ciudad
    }
}
```

A esto se le conoce como _field init shorthand_.

De igual forma, podemos crear instancias en base a otra instancia (como una plantilla) cambiando valores que necesitáramos. A esto se le llama _struct update syntax_. Por ejemplo:
```rust
fn main(){
    // creamos la primera instancia
    let usuario1 = Usuario {
        nombre: "Ernesto".to_string(),
        apellido: "Moncrieff".to_string(),
        edad: 28,
        ciudad: "Londres".to_string()
    };

    println!("Estructura - 1:");
    println!("Nombre completo - {} {}", usuario1.nombre, usuario1.apellido);
    println!("Edad - {}", usuario1.edad);
    println!("Ciudad - {}", usuario1.ciudad);
    /*
        Estructura - 1:
        Nombre completo - Ernesto Moncrieff
        Edad - 28
        Ciudad - Londres
    */

    // creamos la segunda instancia
    let usuario2 = Usuario {
        nombre: "Algernon".to_string(), // declaramos los parámetros que cambiarán
        ..usuario1  // de esta forma indicamos que el resto de parámetros serán iguales que en usuario1
    };

    println!("\nEstructura - 2:");
    println!("Nombre completo - {} {}", usuario2.nombre, usuario2.apellido);
    println!("Edad - {}", usuario2.edad);
    println!("Ciudad - {}", usuario2.ciudad);

    /*
        Estructura - 2:
        Nombre completo - Algernon Moncrieff
        Edad - 28
        Ciudad - Londres
     */
}

// aquí definimos la estructura
struct Usuario {        // utilizamos struct para declararlas
    nombre: String,
    apellido: String,
    edad: i32,
    ciudad: String
}
```

Cabe desctacar que esto vuelve inválido a `usuario1` después de declarar `usuario2` ya que este último toma la propiedad de los datos del primero (los datos tipo `String`). Para evitarlo, se debería usar `.clone()` y **no usar el _struct update syntax_** para evitar tomar la propiedad de dichos valores y evitar que `usuario1` deje de ser válido.

Aparte de esta forma de crear estructuras, existen otras dos: las _tuple structs_ y las _uni-like structs_. 

Las funcionan como un tipo especial de tuplas que nos permite diferenciar entre una tupla común y una "especial". Por ejemplo, si tenemos dos tuplas tipos de tuplas, una que simboliza colores RGB y otra un punto en el espacio, podemos hacerlo de la siguiente manera:
```rust
struct Color(i32, i32, i32);
struct Punto(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origen = Punto(0, 0, 0);
}
```

Así, si bien ambas tuplas tienen los mismos atributos (`(i32, i32, i32)`), no son iguales. 

Por otro lado, tenemos aquellas estructuras que no poseen valores, es decir, su valor es `()`. Las cuales se declaran de la siguiente manera:
```rust
struct Void;

fn main() {
    let a = Void;
}
```

Como dato extra, es posible imprimir en pantalla las estructuras que creamos direstamente ya que Rust añade una funcionalidad para imprimir información de debug. Añadiendo `#[derive(Debug)]` justo antes de declarar la estructura e imprimiéndola con `:?`. De la siguiente manera:

```rust
fn main(){
    let usuario1 = Usuario {
        nombre: "Ernesto".to_string(),
        apellido: "Moncrieff".to_string(),
        edad: 28,
        ciudad: "Londres".to_string()
    };

    println!("{usuario1:?}");
    // Usuario { nombre: "Ernesto", apellido: "Moncrieff", edad: 28, ciudad: "Londres" }
}

// aquí definimos la estructura
#[derive(Debug)]
struct Usuario {
    nombre: String,
    apellido: String,
    edad: i32,
    ciudad: String
}
```

Otra forma es haciendo uso del macro `dbg!()` de la siguiente forma:
```rust
fn main(){
    let usuario1 = Usuario {
        nombre: "Ernesto".to_string(),
        apellido: "Moncrieff".to_string(),
        edad: 28,
        ciudad: "Londres".to_string()
    };

    dbg!(usuario1);
    /*
        [test.rs:9:5] usuario1 = Usuario {
            nombre: "Ernesto",
            apellido: "Moncrieff",
            edad: 28,
            ciudad: "Londres",
        }
    */
}

// aquí definimos la estructura
#[derive(Debug)]
struct Usuario {
    nombre: String,
    apellido: String,
    edad: i32,
    ciudad: String
}
```

# Métodos

Al igual que en otros lenguajes, cuando se tienen clases (o en este caso, estructuras) siempre se tiene la posibilidad de crear métodos asociados a estas. Los métodos funcionan igual que las funciones, y (en Rust) su única diferencia es el contexto en que estas son declaradas. Pues, a diferencia de las funciones, los métodos son creados para las estructuras, enums o traits.

Para declarar un método, a diferencia de otros lenguajes en los que esto se hace junto a la declaración de la clase, en Rust no se hace en el bloque de `struct`, sino en otro bloque llamado `impl` (implementación). Y para llamar los métodos, lo hacemos de la misma manera que en otros lenguajes como C++, Python, etc. Ejemplo:
```rust
fn main(){
    let usuario1 = Usuario {
        nombre: "Ernesto".to_string(),
        apellido: "Moncrieff".to_string(),
        edad: 28,
        ciudad: "Londres".to_string()
    };

    usuario1.saludar();
}

// aquí definimos la estructura
struct Usuario {
    nombre: String,
    apellido: String,
    edad: i32,
    ciudad: String
}

// aquí definimos los métodos
impl Usuario {
    // método llamado .saludar()
    fn saludar(&self){  // el primer parámetro será self, &self para que sea referencia
    // si necesitaramos que fuera mutable, sería &mut self
    // esto será siempre que se necesite utilizar los valores de la instancia
        println!("Hola!\nMi nombre es {}.", self.nombre);
        // self funciona como 
    }
}
```

Es importante mencionar que a diferencia de lenguajes como C++, en donde las referencias deben llamar sus métodos de manera distinta (usando el operador `->` por ejemplo), en Rust las referencias lo hacen de la misma forma, usando el `.`.

Por otro lado, podemos agregar de igual forma todos los parámetros que deseemos los métodos y hacerlos retornar cualquier clase de valor, igual que como las funciones. Y podemos agregar todos los métodos que necesitemos, al igual que hacerlo en todos los bloques `impl` que queramos.

A todas las funciones que se crean dentro de un bloque `impl` se les conoce como funciones asociadas (_associated functions_) porque están asociadas a lo que siga del `impl`. Las funciones asociadas que no sean métodos (es decir, aquellas que no tengan el parámetro `self`) podemos llamarlas sin necesidad de instanciar la estructura, ¡Incluso pueden servir para crear instancias! Ese es el caso de `String::from()` por ejemplo. Por ejemplo, con la estructura que hemos estado utilizando, podemos crearle una función análoga a los contructores de otros lenguajes:
```rust
fn main(){
    // lo instanciamos usando su constructor
    let usuario1 = Usuario::crear_usuarios("Ernesto".to_string(), "Moncrieff".to_string(), 28, "Londres".to_string());

    // usamos un método
    usuario1.saludar();
    /*
        Hola!
        Mi nombre es Ernesto.
    */

    // lo imprimimos usando su método
    usuario1.imprimir();
    /*
    [test.rs:37:9] self = Usuario {
        nombre: "Ernesto",
        apellido: "Moncrieff",
        edad: 28,
        ciudad: "Londres",
    }
    */
}

// aquí definimos la estructura
#[derive(Debug)]    // añadimos para poder imprimirlo
struct Usuario {
    nombre: String,
    apellido: String,
    edad: i32,
    ciudad: String
}

// aquí definimos los métodos
impl Usuario {
    // le creamos su constructor
    fn crear_usuarios(nombre: String, apellido: String, edad: i32, ciudad: String) -> Usuario {
        Usuario {
            nombre,
            apellido,
            edad,
            ciudad
        }
    }

    // creamos un método
    fn saludar(&self){
        println!("Hola!\nMi nombre es {}.\n", self.nombre);
    }

    // creamos un método que lo imprime
    fn imprimir(&self){
        dbg!(self);
    }
}
```

# Enums

Los enums, o enumeraciones, son un tipo de dato que permite definirse en base a sus posibles variantes. 

La forma más sencilla de explicarlo es por medio de un ejemplo, por lo cual vamos a tomar el ejemplo de las figuras geométricas. 

Imaginemos que tenemos 3 posibles formas geométricas: rectángulo, triángulo y círculo. Por obvias razones, una forma no puede ser un rectángulo y un círculo al mismo tiempo, pues son dos figuras completamente distintas (aunque ambas son figuras después de todo). Por lo que podemos tener:
```rust
fn main(){
    // creamos/instanciamos tres figuras
    let a = Shape::Rectangulo;
    let b = Shape::Triangulo;
    let c = Shape::Circulo;
}

// aquí definimos el enum
enum Shape {
    Rectangulo,
    Triangulo,
    Circulo 
}
```

Aquí, como podemos intuir, lo que hacemos es crear tres figuras (una de cada tipo) en el `main()` y definir las figuras como el enum `Shape`. Así como los tenemos su utilidad es similar a un booleano que en lugar de tomar los valores de `true` y `false` toma los valores de `Rectangulo`, `Triangulo` y `Circulo`. Podemos evaluarlo usando `match` (más adelante lo veremos a mayor detalle):
```rust
fn main(){
    let a = Shape::Rectangulo;
    let b = Shape::Triangulo;
    let c = Shape::Circulo;

    match a {
        Shape::Rectangulo => println!("Es un rectángulo!"),
        Shape::Triangulo  => println!("Es un triángulo!"),
        Shape::Circulo    => println!("Es un círculo!")
    }

    // imprimirá
    // "Es un rectángulo!"
    // pues a es un rectángulo
}

// aquí definimos el enum
enum Shape {
    Rectangulo,
    Triangulo,
    Circulo
}
```

Sin embargo, es posible asociarle datos a cada uno. ¡Incluso asociarle datos distintos a cada uno! Por ejemplo, el rectángulo podría necesitar dos lados; el triángulo dos lados y un ángulo; el círculo solo el radio:

```rust
fn main(){
    // creamos/instanciamos tres figuras
    let a = Shape::Rectangulo {
        alto: 2.4,
        ancho: 1.2
    };
    let b = Shape::Triangulo {
        lado_1: 3.1,
        lado_2: 5.2,
        angulo: 1.57
    };
    let c = Shape::Circulo {
        radio: 1.0
    };
}

// aquí definimos el enum
enum Shape {
    Rectangulo {
        alto: f64,
        ancho: f64
    },
    Triangulo {
        lado_1: f64,
        lado_2: f64,
        angulo: f64,
    },
    Circulo {
        radio: f64
    } 
}
```

Como podemos ver, es como si cada uno fuera una estructura distinta e independiente. Y, en resumidas cuentas, eso es. Puede tomar las distintas formas que toman las estructuras. Aquí un ejemplo que muestra un enum con varios tipos:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

# Match

Anteriormente hemos mencionado al `match`, y como dijimos antes, hablaremos más de él. Consiste en otra forma de control de flujo, similar al `if`, pero con particularidades. En otros lenguajes el `switch` cumple un rol similar, pero no del todo. 

Al igual que el `switch` de, por ejemplo, C++, `match` verifica si una variable cumple con algo. Con `switch` es su valor, `match` verifica el tipo. Lo cual combina muy bien con los enums, como veíamos antes.

Con los `match` podemos hacer que cada una de las variantes de un enum haga algo distinto. Antes hemos visto que podíamos hacer que dijera cual era, lo cual también lo podemos meter en una función:
```rust
fn tipo(shape: &Shape) {
    match shape {
        Shape::Rectangulo => println!("Es un rectángulo!"),
        Shape::Triangulo  => println!("Es un triángulo!"),
        Shape::Circulo    => println!("Es un círculo!")
    }
}
```

Este funcionaría para el primer ejemplo, es decir, para el que no le asocia valores a cada variante. Si cada variante tiene sus propios datos asociados, se deben mencionar también:

```rust
fn tipo(shape: &Shape) {
    match shape {
        Shape::Rectangulo { alto: _, ancho: _ } => println!("Es un rectángulo!"),
        Shape::Triangulo { lado_1: _, lado_2: _, angulo: _ } => println!("Es un triángulo!"),
        Shape::Circulo { radio: _ } => println!("Es un círculo!")
    }
}
```

Y, como veíamos antes, si puede estar en una función, puede ser un método. 

Es importante mencionar que el `match` debe de cubir **todos los casos posibles**. Es decir, que si elimináramos uno de los posibles casos (por ejemplo, eliminamos el del círculo) no compilaría. Aún así, Rust nos permite considerar todos aquellos posibles casos que no hemos considerado antes, es decir, el resto de casos (funciona similar al `default` de un `switch`). Esto utilizando un `_`:

```rust
fn tipo(shape: &Shape) {
    match shape {
        Shape::Rectangulo { alto: _, ancho: _ } => println!("Es un rectángulo!"),
        _ => println!("No es un rectángulo!")
    }
}
```
Este código sí compilaría.

Antes no lo hemos mencionado, pero las similitudes que tienen las variantes en los enums y las estructuras va más allá de la capacidad de almacenar datos, incluye también la capacidad de tener funciones asociadas (métodos). Pero, ¿cómo lo haríamos si son 3 tipos distintos? Simple, con match:

```rust
fn main(){
    // creamos/instanciamos tres figuras
    let a = Shape::Rectangulo {
        alto: 2.4,
        ancho: 1.2
    };
    let b = Shape::Triangulo {
        lado_1: 3.1,
        lado_2: 5.2,
        angulo: 1.57
    };
    let c = Shape::Circulo {
        radio: 1.0
    };

    // imprimimos sus tipos
    a.tipo();
    b.tipo();
    c.tipo();
    /*
        Es un rectángulo!
        Es un triángulo!
        Es un círculo!
    */
}

// aquí definimos el enum
enum Shape {
    Rectangulo {
        alto: f64,
        ancho: f64
    },
    Triangulo {
        lado_1: f64,
        lado_2: f64,
        angulo: f64,
    },
    Circulo {
        radio: f64
    } 
}

// creamos las funciones asociadas
impl Shape {
    fn tipo(&self) {
        match self {
            Shape::Rectangulo { alto: _, ancho: _ } => println!("Es un rectángulo!"),
            Shape::Triangulo { lado_1: _, lado_2: _, angulo: _ } => println!("Es un triángulo!"),
            Shape::Circulo { radio: _ } => println!("Es un círculo!")
        }
    }
}
```

Y de igual forma, podemos crear métodos que utilicen los datos de cada variante. Como por ejemplo, calcular el área:
```rust
fn main(){
    // creamos/instanciamos tres figuras
    let a = Shape::Rectangulo {
        alto: 2.4,
        ancho: 1.2
    };
    let b = Shape::Triangulo {
        lado_1: 3.1,
        lado_2: 5.2,
        angulo: 1.57
    };
    let c = Shape::Circulo {
        radio: 1.0
    };

    // imprimimos sus tipos
    a.tipo();
    b.tipo();
    c.tipo();
    /*
        Es un rectángulo!
        Es un triángulo!
        Es un círculo!
    */

    // calculamos sus áreas
    println!("{}", a.area());
    println!("{}", b.area());
    println!("{}", c.area());
    /*
        2.88
        8.059997444430588
        3.1415926
    */
}

// aquí definimos el enum
enum Shape {
    Rectangulo {
        alto: f64,
        ancho: f64
    },
    Triangulo {
        lado_1: f64,
        lado_2: f64,
        angulo: f64,
    },
    Circulo {
        radio: f64
    } 
}

// creamos las funciones asociadas
impl Shape {
    fn tipo(&self) {
        match self {
            Shape::Rectangulo { alto: _, ancho: _ } => println!("Es un rectángulo!"),
            Shape::Triangulo { lado_1: _, lado_2: _, angulo: _ } => println!("Es un triángulo!"),
            Shape::Circulo { radio: _ } => println!("Es un círculo!")
        }
    }

    fn area(&self) -> f64 {
        match self {
            Shape::Rectangulo { alto, ancho } => alto * ancho,
            Shape::Triangulo { lado_1, lado_2, angulo } => { // creamos bloques para el valor de retorno para cada variante
                0.5 * lado_1 * lado_2 * angulo.sin()    // .sin() aplica seno a la variable
            }
            Shape::Circulo { radio } => 3.1415926 * radio.powi(2),  // .powi() aplica una potencia
        }
    }
}
```

# `Option`

De entre todos los enums, hay uno que se usará mucho, proveniente de la librería estándar `std` y que resulta extremadamente útil: `Option`.

Imaginemos que tenemos una lista no vacía, y accedemos al primer elemento: pasa lo obvio, nos da el valor del primer elemento. Pero, ¿Y si la lista está vacía? Pues nos retorna... ¿Nada?

Si bien el ejemplo anterior parece sencillo, muestra algo más importante. ¿Cómo debemos manejar aquello que es "nada"? Otros lenguajes se utilizan datos nulos: `None`, `Null`, etc. Un valor nulo significa que no hay nada, y en la mayoría de lenguajes se considera que un dato es, esencialmente, nulo o no nulo.

Esto no representa ningún problema y es sencillo de implementar... en principio. Es extremadamente sencillo llegar a tratar de usar un dato nulo como no nulo, lo que termina en un error.

Por esto mismo, en Rust, se ha optado por una alternativa. No se elimina el concepto de nulo, porque el mismo es útil, ya que expresa que un valor no puede ser utilizado por... alguna razón. Aún así, elimina los datos nulos. En su lugar, utiliza un enum que indica si un dato es nulo o no: `Option<T>`. Que es definido de la siguiente manera:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Hasta el momento, entendemos todo lo que significa eso: un enum que puede ser `None` o `Some(T)`. El `<T>` es un dato genérico, cosa de la que aún no hablamos, pero que significa que `Some(T)` puede tener un parámetro `T` de cualquier tipo. Todos están incluídos por defecto, por lo que podemos usarlos tal cual. Por lo que `T` puede representar cualquier tipo. Por ejemplo:
```rust
let a = Some(3.3);              // Option<f64>
let b = Some(String::new());    // Option<String>
let c: Option<i32> = None;
```

Un momento, ¿No habíamos dicho que no existía un dato nulo? Correcto, no existe. Ese `None` que vemos es una variante de `Option`, en específico de `Option<i32>`, como podemos ver en el tipo (debemos declararlo explícitamente).

Ahora bien, ¿Qué ocurrirá si trato de sumarle algo a un `None`? ¿O a un `Option` con valor en `Some()`?
```rust
fn main() {
    let x = 5;  // es i32
    let y: Option<i32> = Some(5);
    let z: Option<i32> = None;

    // debería dar 10
    let sum = x + y;
    println!("{sum}");

    // debería dar 5 (?)
    let sum = x + z;
    println!("{sum}");
    
    // debería dar 5 (?)
    let sum = y + z;
    println!("{sum}");

}
```

La respuesta es sencilla: error de compilación.

Los `Option<T>` no se suman, ni entre ellos ni con valores de tipo `T`. Ninguna de esas sumas se llevará a cabo. Para llevarla a cabo, necesitamos utilizar `match`:
```rust
fn main() {
    let x = 5;  // es i32
    let y: Option<i32> = Some(5);
    let z: Option<i32> = None;

    // dará 10
    let sum = match y {
        Some(t) => t + x,
        None => x
    };
    println!("{sum}");

    // dará 5
    let sum = match z {
        Some(t) => t + x,
        None => x
    };
    println!("{sum}");
    
    // dará 5 
    let sum = match y {
        Some(t) => t,
        None => 0
    } + match z {
        Some(t) => t,
        None => 0
    };
    println!("{sum}");
}
```
Es importante mencionar que, igual que antes, el `match` debe cubrir **todos los casos posibles**.

# `if let`

Es una combinación de `if` y `let` (también podemos añadirle `else`) que funciona similar a `match`, pero simplificando. Cada `if - let` funciona como un brazo individual del `match`. Por ejemplo, una función como la vista antes:
```rust
fn tipo(shape: &Shape) {
    match shape {
        Shape::Rectangulo { alto: _, ancho: _ } => println!("Es un rectángulo!"),
        _ => println!("No es un rectángulo!")
    }
}
```
Podemos reescribirla como:
```rust
fn tipo(shape: &Shape) {
    if let Shape::Rectangulo { alto: _, ancho: _ } = shape {
        println!("Es un rectángulo!")
    } else {
        println!("No es un rectángulo!")
    }
}
```

Igual que siempre, podemos colocar tantos `else - if` como queramos. Y el `else` es opcional. También, como es lógico, funciona con `Option<T>`. Podemos reescribir esto, por ejemplo:

```rust
fn main() {
    let x = 5;  
    let y: Option<i32> = Some(5);

    let sum = match y {
        Some(t) => t + x,
        None => x
    };
    println!("{sum}");
}
```

Como esto:

```rust
fn main() {
    let x = 5;  
    let y: Option<i32> = Some(5);

    let sum = if let Some(t) = y {
        t + x
    } else {
        x
    };
    println!("{sum}");
}
```
