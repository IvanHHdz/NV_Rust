# Reglas de propiedad (_Ownership_)

El _Ownership_ o las reglas de propiedad son la característica más única de Rust, teniendo multiples implicaciones profundas en todo el lenguaje. Es gracias a estas reglas que Rust es capaz de manejar la memoria de manera segura sin necesidad de un recolector de basura (_garbage collector_) como lo hacen lenguajes como C++. Razón por la cual entender bien el _ownership_ es **fundamental** en Rust. 

Pero, ¿Qué es el _Ownership_? Se trata de un conjunto de reglas que dictan cómo Rust interactua con la memoria. 

Como mencionábamos antes, esta es una forma distinta a como lo hacen el resto de lenguajes. La mayoría utiliza un recolector de basura para deshechar aquella memoria que no se necesite más; otros es el programador quien debe encargarse de asignar y liberar explícitamente la memoria que utiliza.

Rust utiliza el _ownership_: un conjunto de reglas de propiedad que se verifican al momento de compilar el programa, y a menos que todas las reglas se cumplan, el progama no compilará.

Las **reglas de propiedad** son las siguientes:
- Cada valor en Rust tiene un propietario.
- Solo puede haber un propietario a la vez.
- Cuando el propietario quede fuera de alcance, se elimina el valor.

# Cadenas de texto

[Anteriormente](../1.2%20-%20Conceptos%20básicos/ConceptosBásicos.md) se habló de los tipos de datos. Y como suele pasar cuando se enseña un lenguaje de programación, no se mencionó el tipo "cadena de texto". La razón de esto siempre es el hecho que este tipo no es como el resto, y que posee ciertas características especiales que sirven para explicar ciertos conceptos. En Rust, esto no es distinto.

Nos hemos saltado explícitamente las cadenas de texto para poder explicar con ellas las reglas de propiedad de mejor manera, pues en estas se reflejan con gran claridad. Esto debido a que es un tipo de dato más complejo que los vistos [anteriormente](../1.2%20-%20Conceptos%20básicos/ConceptosBásicos.md). La diferencia principal es el hecho que su tamaño es conocido en tiempo de compilación, lo que permite guardarlos en la `stack` de la memoria y realizar copias de los mismos con facilidad. Esto no es así con las cadenas de texto (que entre otras cosas, se guardan en la `heap` de la memoria).

En esta parte nos enfocaremos únicamente en los aspectos de las cadenas de texto que tienen relación con las reglas de propiedad, pues estos mismos principios aplican a otros tipos de datos complejos (aquellos que no vimos [anteriormente](../1.2%20-%20Conceptos%20básicos/ConceptosBásicos.md)). Más adelante volveremos a profundizar las cadenas de texto. Primero que nada, hay que mencionar que en Rust existen dos tipos de cadenas de texto: `&str` y `String`.

Una `&str` es como las vistas [anteriormente](../1.2%20-%20Conceptos%20básicos/ConceptosBásicos.md), cadenas de texto literales, que son inmutables:
```rust
fn main(){
    let a = "Cadena de texto";
    let b = "Inmutable";
    let c = "&str";

    println!("Esta es una {a}, la cual es {b}. Es de tipo {c}.");
}
```

Estas de aquí, en sí, son referencias a valores binarios que declaramos de manera explícita. No pueden cambiar una vez declaradas, aunque pueden reescribirse usando `mut`:
```rust
fn main(){
    let a = "Cadena de texto";
    let mut b = "Inmutable";
    let c = "&str";

    println!("Esta es una {a}, la cual es {b}. Es de tipo {c}.");

    b = "Mutable";

    println!("Aunque puedo volverla {b}.");
}
```

Este tipo es útil, solo tiene un problema: su valor debe ser conocido en tiempo de compilación. Lo cual es inconveniente si lo que deseamos tener es, por ejemplo, el nombre del usuario. Para casos así se utiliza el segundo tipo: `String`.

Este tipo, a diferencia del primero, se guarda en el `heap` de la memoria y puede almacenar texto de un tamaño desconocido en tiempo de compilación. Podemos crear una `String` a partir de una cadena de texto literal usando la función `from()` del módulo `String`:
```rust
fn main(){
    let cadena = String::from("Ferris");
}
```

Y, como habíamos mencionado, este tipo **sí es mutable**:
```rust
fn main(){
    let mut cadena = String::from("Ferris");
    println!("{cadena}");
    cadena.push_str(" está feliz de tu progreso!");
    println!("{cadena}");
}
```

Pero, ¿Por qué existe esta diferencia? ¿Por qué las `String` pueden mutar mientras que las cadenas de texto literal no pueden? 

Para entenderlo mejor, tenemos que entender la manera en que se guardan dichos datos en memoria. Como mencionábamos antes, mientras las cadenas de texto literal se guardan en el `stack` de la memoria, las `String` se guardan en el `heap`. Esto debido a que mientras las cadenas de texto literal sabemos en tiempo de compilación el tamaño que tendrán en cualquier parte de nuestro código, las `String` no. El hecho que no sepamos qué tamaño tendrán  hace necesario que se almecenen en el `heap` de la memoria, permitiendo que puedan ser mutables. Pero esto tiene ciertas implicaciones: debemos solicitar y liberar la memoria que usamos.

Solicitarla no es complicado, lo hacemos, por ejemplo, al ejecutar `String::from()`. Pero liberarla sí requiere mayor análisis. En muchos lenguajes, incluído C++ por ejemplo, se tiene un recolector de basura (_garbage collector_) que se encarga de revisar periódicamente y liberar aquella memoria que ya no se utilice. En otros lenguajes es tarea del programador, quien debe tener cuidado de liberar la memoria (o desperdiciaría memoria al no liberarla), no llamarla antes de tiempo (por que ocasionaría problemas si llamara a una variable que ya eliminó) ni llamarla dos veces (¡más problemas aún!).

Rust trabaja distinto. [Antes](../1.4%20-%20Funciones%20y%20Bloques/FuncionesYBloques.md) hablamos de los alcances de variables y los bloques. Rust se encarga de liberar la memoria que utiliza cierta variable una vez se sale del alcance de la misma. Esto, de manera automática llamando una función llamada `drop()` al cerrar las llaves:

```rust
fn main(){
    {   // entramos en un bloque
        // aquí solicitamos memoria en el heap para guardar la variable cadena, 
        // que guardará una copia de la cadena de texto "Ferris"
        let cadena = String::from("Ferris");    

        // utilizamos la variable
        println!("{cadena}")
    }   // salimos del bloque, por lo que el alcance de la variable cadena termina
        // como el alcance de la variable terminó, se libera automáticamente la memoria que usaba
        // lo que quiere decir que se llamó automáticamente la función drop()
        // y por eso la variable deja de existir
}
```

Hasta aquí, nada raro.

# Mover

Ahora, analicemos lo que ocurre en el siguiente ejemplo:

```rust
fn main() {
    let x = 23;
    let y = x;
}
```

Parece bastante claro, ¿no? Creamos una variable `x` de tipo `i32` con el valor de `23`. Luego, creamos otra variable `y` de tipo `i32` que **copia** el valor de `x`, es decir `23`. Por lo que al final tenemos 2 variables, `x` y `y`, con el mismo valor `23`.

Y eso es justamente lo que ocurrió.

Ahora, pasemos a otro ejemplo:
```rust
fn main() {
    let s1 = String::from("Ferris");
    let s2 = s1;
}
```

Se ve muy similar, ¿no? ¿Acaso pasó lo mismo? Pues si asumimos que ocurrió lo mismo, entonces tendríamos dos variables, `s1` y `s2`, que tienen el mismo valor `"Ferris"`. Por lo que podríamos imprimir ambas, por lo que si tenemos:
```rust
fn main() {
    let s1 = String::from("Ferris");
    let s2 = s1;
    println!("{s1}");
    println!("{s2}");
}
```

Debería compilar, ¿no?

No, no compilará. Esto debido a que no funciona igual que antes. Las `String`, como mencionabamos antes, son tipos de datos complejos, por lo que no se hace así.

Imaginémoslo así: en memoria, existe un lugar donde están los datos que representan una cadena de texto `"Ferris"`. Al crear la variable `s1` obtenemos un puntero que señala a la posición de `"Ferris"` en memoria, por lo que podemos decir que `s1` es propietaria de `"Ferris"`. Al crear segunda variable `s2` se crea otro puntero que señala a `"Ferris"`. Suena lógico y sencillo, sin embargo, hay un problema: según las reglas de propiedad, un valor no puede tener dos propietarios, y aquí tenemos un valor (`"Ferris"`) con dos propietarios (`s1` y `s2`). La solución es sencilla: le quitamos a `s1` la propiedad que tiene sobre `"Ferris"` y se la damos a `s2`, a esta acción se le llama **mover**. Pero, si `s2` tiene la propiedad de `"Ferris"`, y `s1` ha perdido propiedad sobre el valor que tenía... ¿Qué pasará cuando lea `s1`? Un error. `s1` no puede ser leída porque ha dejado de existir, o por lo menos dejó de tener el valor y propiedad sobre `"Ferris"`. 

Por lo que:
```rust
fn main() {
    let s1 = String::from("Ferris");    // creamos s1, con propiedad sobre "Ferris"
    let s2 = s1;                        // creamos s2, que ahora tendrá la propiedad sobre "Ferris"
    println!("{s2}");                   // para este punto, s1 ha dejado de existir
}
```

Algo similar ocurre al reescribir una variable de este estilo con un valor diferente:
```rust
fn main() {
    let mut s1 = String::from("Ferris");    // hay en memoria un espacio para "Ferris"
    s1 = String::from("Tux");               // ahora hay un espacio en memoria para "Tux"
    // como nadie tiene pertenencia sobre "Ferris", se ejecuta drop() para él
    // esto libera la memoria que "Ferris" tenía
    println!("{s1}");                       // imprime "Tux"
}
```

# Clonar y copiar

Como mencionábamos antes, en un caso como el siguiente se **copian** los valores:
```rust
fn main() {
    let x = 23;
    let y = x;
}
```
Este código, a diferencia del que vimos con `String`, sí es válido. Esto debido a que los valores de tipo `i32` se copian. Esto debido a que el `i32`, todos los demás tipos de datos vistos [anteriormente](../1.2%20-%20Conceptos%20básicos/ConceptosBásicos.md) junto a tuplas con únicamente esos mismos tipos de datos (veremos las tuplas después) implementan el trato de `copy`, ya que se almacenan en el `stack` de la memoria y debido a que su tamaño sí es conocido en tiempo de compilación.

Por otro lado, para que esto funcione:

```rust
fn main() {
    let s1 = String::from("Ferris");
    let s2 = s1;
    println!("{s1}");
    println!("{s2}");
}
```

deberemos **clonar** el valor de `s1` a `s2`. Pues no implementa el trato de `copy`, sino de `clone`. Esto lo logramos ejecutando el método `.clone()`. Esto puede llegar a ser costoso en términos de tiempo de ejecución en ciertos casos, pues ejecutará código extra para realizar la copia.
```rust
fn main() {
    let s1 = String::from("Ferris");
    let s2 = s1.clone();
    println!("{s1}");
    println!("{s2}");
}
```

Y lo que ocurre aquí es justamente lo que hablamos al principio: Creamos una variable `s1` que tiene propiedad sobre `"Ferris"`. Luego, creamos otra variable `s2` que clonará el valor de `s1`, es decir `"Ferris"`. Así que habrán dos valores (`"Ferris"` estará dos veces en memoria) y dos variables (`s1` y `s2`); una variable para cada valor. Por lo que todas las reglas de propiedad se siguen cumpliendo.

# Funciones

Ahora bien, ¿Qué ocurre si pasamos datos como parámetros? Exactamente lo mismo que si asignamos: la propiedad se moverá o copiará dependiendo del caso. Por ejemplo:
```rust
fn main(){
    let s1 = String::from("Duke");
    let s2 = String::from("Tux");
    let s3 = String::from("Ferris");

    // esto hará que nadie tenga la propiedad de "Duke"
    // por lo que se ejecuta drop() y "Duke" deja de existir
    remove_ownership(s1);    

    // esto moverá la propiedad de "Ferris" desde s3 a s2
    // esto no solo hará que s3 deje de tener la propiedad por lo que s3 está inválida
    // sino que eliminará la pertenencia de "Tux" que tenía s2
    // por lo que se ejcuta drop() y "Tux" deja de existir
    let s2 = take_back_ownership(s3);

    // por lo que la única variable con valor es s2
    // y su valor es "Ferris"
    println!("La mejor mascota es {s2}!");

    // es importante mencionar, que tanto s1 como s3 no son válidas
    // y el compilador nos impedirá llamarlas, pues no tienen propiedad sobre ningún valor
}

fn remove_ownership(s: String){
    println!("Ahora nadie tiene propiedad sobre {s}");
    // ejecutará drop() sobre el valor que s tenga
}

fn take_back_ownership(s: String) -> String {
    println!("Acabamos de mover la propiedad de {s}");
    s
    // retornará'la propiedad sobre el valor que tenga s
}
```

Pero, ¿Qué ocurrirá si necesitamos, por ejemplo, retornar una valor específico respecto a la `String`? Por ejemplo, el tamaño:
```rust
fn main(){
    let s1 = String::from("Ferris");    // creamos una variable s1
    // creamos una variable size que tendrá el tamaño de s1, que será retornado por lenght()
    let size = lenght(s1);              

    // mostramos el tamaño
    println!("La String tiene un tamaño de: {size}");

}

fn lenght(s : String) -> i32 {
    let mut l = 0;          // creamos una variable l para el tamaño
    for _ in s.chars() {    // recorremos la String caracter por caracter con el método .chars()
        l += 1;             // por cada iteración, sumamos 1
    }
    l                       // retornamos l
}   // al salir del alcance, se ejecuta drop() sobre el valor de s 
```

Parece bien, pero tiene un problema: `s1` deja de ser válida. Y sería muy útil poder volverla a usar, especialmente para, por ejemplo, decir qué tiene ese tamaño. Podríamos probar retornar ambos valores:
```rust
fn main(){
    let s1 = String::from("Ferris");    // creamos una variable s1

    // creamos una variable size que tendrá el tamaño de s1, que será retornado por lenght()
    // también volvemos a declarar s1, que tomará el otro valor que retorne lenght "Ferris"
    let (size, s1) = lenght(s1);              

    // mostramos el tamaño y la String
    println!("{s1} tiene un tamaño de: {size}");
}

fn lenght(s : String) -> (i32, String) {
    let mut l = 0;          // creamos una variable l para el tamaño
    for _ in s.chars() {    // recorremos la String caracter por caracter con el método .chars()
        l += 1;             // por cada iteración, sumamos 1
    }
    (l, s)                  // retornamos l y s
}
```

Esto es correcto, sí, pero es un poco... impráctico. En lugar de **mover** la propiedad de `"Ferris"` a la función, sería mejor poder pasarlo como una **referencia**, sin mover su propiedad, ¿No?

# Referencias y _borrowing_

Pues bien, sí es posible pasar datos por referencia. Una referenica consiste en hacer referencia a un valor sin necesidad de tomar la propiedad del mismo. Para hacer una referencia, se utiliza el simbolo `&`, se debe dar a entender que se usará una referencia tanto en la declaración de la función como al llamar la misma. Aquí el ejemplo de antes con referencias:
```rust
fn main(){
    let s1 = String::from("Ferris");    // creamos una variable s1

    // creamos una variable size que tendrá el tamaño de s1, que será retornado por lenght()
    // ahora pasamos a "Ferris" como una referencia
    let size = lenght(&s1);              

    // mostramos el tamaño y la String
    println!("{s1} tiene un tamaño de: {size}");

}

fn lenght(s : &String) -> i32 {
    let mut l = 0;          // creamos una variable l para el tamaño
    for _ in s.chars() {    // recorremos la String caracter por caracter con el método .chars()
        l += 1;             // por cada iteración, sumamos 1
    }
    l                       // retornamos l 
}                           // la referencia deja de existir al dejar su alcance
// sin embargo, no se ejecuta drop() pues s nunca tuvo la propiedad de ningún valor
```

A la acción de crear referencias se le llama _borrowing_. Este nombre nos ayuda a entender un poco mejor también el proceso. Pues, al igual que si tomamos prestado algo no podemos decir que nos pertenece, pues no es nuestro, la propiedad de los valores referenciados funciona igual. De la misma forma, no podemos modificar algo que nos prestaron, no podemos hacerlo sin permiso. Funciona igual: la referencia también es inmutable por defecto.

Pero podemos hacerlas mutables también:

```rust
fn main() {
    let mut s = String::from("Ferris"); // claro, para eso la variable original debe ser mutable también
    change(&mut s);     // y la pasamos como una referencia (&) mutable (mut)
    println!("{s}");     // imprime "Ferris!"
}

fn change(some_string: &mut String) {   // aquí también mencionamos que será una referencia mutable
    some_string.push_str("!");
}
```

Una pequeña restricción con esto es el hecho que solo es permitido tener **una referencia mutable a la vez**. Así que, por ejemplo:
```rust
fn main() {
    let mut s = String::from("Ferris"); // claro, para eso la variable original debe ser mutable también
    let s_r = &mut s;       // creamos una referencia mutable
    change(&mut s);         // pasamos como una referencia mutable (segunda referencia) 
    println!("{s_r}");      // está tratando de usar la referencia mutable
    println!("{s}");        // imprime "Ferris!"
}

fn change(some_string: &mut String) {   // aquí también mencionamos que será una referencia mutable
    some_string.push_str("!");
}
```

Este código no compilará. Pues tenemos la referencia mutable al principio `s_r`, luego a `change` le pasamos una referencia mutable (para crearla, se eliminará `s_r`) y luego tratamos de acceder a `s_r`. Esto no es posible. Pues si bien podemos tener todas las referencias que queramos, si tenemos una referencia mutable solo podemos tener **esa** referencia.

Una última cosa referente a las referencias es apuntar a un valor inexistente. Esto es algo que puede llegar a ocurrir, y con frecuencia si no se maneja bien, en otros lenguajes como C++, pero en Rust es imposible. SI trataramos de hacer eso, el compilador nos lo impedirá. Por ejemplo:
```rust
fn main() {
    let s = invalido();
}

fn invalido() -> &String { // retorna una referencia a una String
    let s = String::from("hello"); // crea una variable s, dentro de este bloque
    &s // retorna la referencia a s
} // al terminar la función, se sale del alcance de s, por lo que se ejecuta drop()
// así que s deja de ser válida, junto a su referencia
```

Esto no compilará. Pues se está haciendo una referencia a un valor que deja de existir (mientras la referencia aún existe).

# El Compilador

Seguramente habrá notado que el compilador suele negarse a compilar a menudo, cualquier clase de posible error. Y si ha compilado el código usted mismo, habrá notado que también pone alertas por todo (aunque con alertas sí compila). 

Quizá incluso llegue a pensar que el compilador es demasiado estricto, o molesto.

No se preocupe, es comprensivo.

Esta sección no trata de convencerle de lo contrario, el compilador es estricto y exigente. Pero no lo hace por molestar. 

El compilador nos ayuda a encontrar errores en nuestros programas. Claro, los errores del compilador pueden ser molestos y frustrantes. Pero esos errores no significan que no sabe programar bien en Rust; esos errores solo significan que su programa no está haciendo lo que usted quiere de manera segura y correcta. Recuerde, no significan que usted no sea un buen programador, ¡Todos recibimos errores de compilación de vez en cuando!

Siempre que obtenga un error de compilación, léalo. El compilador trata de explicarle por qué su código no funciona como debería. Incluso léa las advertencias. De esta forma podrá no solo corregir el error, sino también entenderlo. Así podrá reducir la frecuencia con la que obtiene tales errores.

Con el tiempo, verá que muchas veces el compilador señala problemas antes de que se conviertan en errores difíciles de detectar. Y verá que el compilador nunca fue su enemigo, sino su mejor aliado.