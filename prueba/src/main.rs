use std::ops::Deref;

fn main(){
    let puntero = Puntero::nuevo(23);
    let resultado = suma(&*puntero, &puntero);
    println!("{resultado}");
    drop(puntero);
    println!("Después del drop!");
}

#[derive(Debug)]
struct Puntero(i32);   // este será nuestro nuevo puntero inteligente

impl Puntero{
    fn nuevo(valor: i32) -> Self {    // método para instanciar
        Self(valor)
    }
}

impl Drop for Puntero {
    fn drop(&mut self) {
        println!("Liberando el valor de {:?}", self.0);
    }
}

impl Deref for Puntero {
    type Target = i32;        // veremos esto más adelante

    // esta será la función que se llamará al usar *
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn suma(a: &i32, b: &i32) -> i32{
    *a + *b
}