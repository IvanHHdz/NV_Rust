# Pruebas (_Tests_)

Aunque nuestro código compile, siempre existe la posibilidad que no haga las cosas como se suponía que debía hacerlas. Razón por la cual debemos verificar a detalle todo nuestro código. Una forma sencilla de hacer esto más rápido es por medio de pruebas o _tests_ que verifiquen que nuestro código haga lo que se supone que debe hacer. 

En palabras de Dijkstra:
> El testeo de un programa es una forma muy efectiva de mostrar la presencia de bugs, pero es lastimosamente inadecuada para mostrar su ausencia.

Por lo que vamos a aprender a realizar testeo/pruebas.

Supongamos primero que tenemos la siguiente función:
```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}
```

La cual queremos testear para verificar que funciona. Para esto, utilizaremos una función que la pruebe y verifique que es correcta utilizando la macro `assert_eq!()`. La crearemos de la siguiente forma:
```rust
#[test]             // este atributo la vuelve una función para tests
fn prueba1() {      // le damos un nombre descriptivo
    let resultado = sumar(1, 1);    // tendremos una variable para el resultado
    assert_eq!(resultado, 2);       // compararemos
}
```

Y para ejecutar las pruebas, utilizando cargo, usaremos:
```shell
cargo test
```
Y, en este caso, obtendremos:
```
running 1 test
test prueba1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

> Nota: es recomendable colocar todas las pruebas dentro de un módulo específico, el cual deberá tener arriba la línea: `#[cfg(test)]`. A partir de ahora vamos a asumir que lo colocó en un módulo aparte.

También podemos colocar más tests, tantos como queramos. Cada prueba se realiza en un hilo separado, así que en caso que un test falle y llame a `panic!()`, solamente afectará a dicho hilo. Cuando el hilo principal detecta que uno de los hilos que ejecuta tests ha muerto marca que ha fallado.

Por ejemplo, si tenemos:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]             // este atributo la vuelve una función para tests
    fn prueba1() {      // le damos un nombre descriptivo
        let resultado = sumar(1, 1);    // tendremos una variable para el resultado
        assert_eq!(resultado, 2);       // compararemos
    }
    
    #[test]
    fn prueba2() {      // creamos otro test que fallará
        panic!("Algo salió mal")    // fallamos el test
    }
}
```

Y lo ejecutamos, obtendremos:
```
running 2 tests
test tests::prueba1 ... ok
test tests::prueba2 ... FAILED

failures:

---- tests::prueba2 stdout ----

thread 'tests::prueba2' panicked at src/main.rs:21:9:
Algo salió mal
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::prueba2

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin testeo`
```

Como podemos ver, nos indica que `prueba2` ha fallado, y nos indica el error con que crasheó.

Además de la macro `assert_eq!()` que hemos usado, también podemos usar `assert!()` la cual recibirá un dato booleano que interpretará como exitoso si es `true` y fallido si es `false`. Otra macro que puede sernos útil es `assert_ne!()` la cual funciona a la inversa de `assert_eq!()`. `assert_eq!()` dará `Ok` si ambos parámetros ingresados son iguales, y fallará si no; `assert_ne!()` fallará si ambos valores son iguales, y dará `Ok` si ambos son distintos.

Estas macros también pueden aceptar un parámetro extra que sirve para enviar un mensaje en caso que el test fallé. Por ejemplo, si tenemos:
```rust
#[test]             // este atributo la vuelve una función para tests
fn prueba1() {      // le damos un nombre descriptivo
    let resultado = sumar(1, 1);    // tendremos una variable para el resultado
    assert_eq!(resultado, 2,        // compararemos
    "Los axiomas de la matemática convencional parecen fallar aquí."   // mensaje en caso de fallo
    );       
}
```

Así, si el `resultado` no es igual a `2` se mostrará un mensaje con la descripción que proporcionamos.

Pero, para probar correctamente un programa no solo debe ser capaz de no crashear cuando se supone que no debe hacerlo, sino que debe crashear cuando se supone que debe hacerlo. Por ejemplo, consideremos la siguiente función:
```rust
fn division(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Ha intentado realizar una operación que no es válida en este ni en ningún otros sistema lógico-algebraico.");
    }
    else {
        a / b
    }
}
```

Como vemos, esta función debería llamar a `panic!()` si `b = 0`. Y la podemos probar si lo hace correctamente de la siguiente manera:
```rust
#[test]
#[should_panic]     // esto indica que debería crashear, si no lo hace, se considerará que falló.
fn prueba3() {
    let a = 10;
    let b = 0;

    let resultado = division(a, b);

    assert_eq!(resultado, 10);
}
```

Pero, ¿Y si tenemos multiples `panic!()`? Podemos especificar con qué mensaje se supone que debería fallar. Esto de la siguiente manera:
```rust
#[test]
#[should_panic(expected = "Ha intentado realizar una operación que no es válida en este ni en ningún otros sistema lógico-algebraico.")]
fn prueba3() {
    let a = 10;
    let b = 0;

    let resultado = division(a, b);

    assert_eq!(resultado, 10);
}
```

De esta forma, si llega a fallar con otro mensaje se nos indicará que el test ha fallado.

De igual forma, podemos utilizar `Result<T, E>` para verificar las pruebas. Por ejemplo, supongamos que tenemos la siguiente función, que fue una de las que vimos en la sección de manejo de errores:

```rust
fn dividir(a: i32, b: i32) -> Result<i32, ()> {
    if b == 0 { // verificará si el denominador es 0, si lo es retornará
        Err(()) // retorno "vacío"
    }
    else {
        Ok(a / b)   // retorno en caso que sí sea válido
    }
}
```

La podemos verificar con:
```rust
#[test]
fn prueba3() -> Result<(), ()> {    // colocamos los datos de retorno
// el tipo de Ok() debe ser (), el de Err() puede ser cualquiera
// pero utilizaremos () para ambos para simplificar en este ejemplo
    let a = 10;
    let b = 1;

    // utilizando un match, retornamos los valores como indicamos
    match dividir(a, b) {  
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}
```

Esto hará que pase el test si se retorna un `Ok()` y lo fallará si se recibe un `Err()`. Esto ya que lo que hará será algo similar a tener `assert!(value.is_err())`.

## Opciones

Como habíamos mencionado, los test se ejecutan en paralelo, cada uno en un hilo distinto existiendo un hilo maestro o principal que los coordina. Sin embargo, existen casos en los que esto no será lo mejor, por ejemplo cuando tenemos recursos que se comparten (veremos todo esto en detalle a su tiempo), como una variable global.

Para estos casos es mejor que las pruebas se ejecuten de una en una (secuencialmente). Esto lo podemos lograr ejecutando las pruebas con el siguiente comando:
```shell
cargo test -- --test-threads=1
```

Por defecto, también, se omite cualquier dato que se imprima en pantalla (por ejemplo con un `println!()`). Para permitir que se muestre en pantalla todo lo que el hilo vaya a mostrar, debemos ejecuarlo con:
```shell
cargo test -- --show-output
```

De igual forma, existen ocasiones en que podríamos querer que solo se ejecute una prueba o un grupo de pruebas. Esto lo logramos de la siguiente manera:
```shell
cargo test prueba1  # solo ejecutará la prueba llamada `prueba1`
cargo test agregar  # solo ejecutará las pruebas que contengan `agregar` en su nombre
```

Podemos también hacer que ciertas pruebas no se realicen a menos que se especifique que queremos realizarlas. Esto agregando el atributo:
```rust
#[ignore]
```

Después de `#[test]`. Así estas pruebas solo se ejecutarán cuando se llamen de la siguiente forma:
```shell
cargo test -- --ignored
```