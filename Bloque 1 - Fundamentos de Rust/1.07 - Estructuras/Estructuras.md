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