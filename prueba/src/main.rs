fn main(){
    let v = "hola".to_string();
    let b = Corrupto{
        valor: v.clone(),
        puntero: &v,
    };

    drop(v);

    dbg!(&b);
}

#[derive(Debug)]
struct Corrupto<'a> {
    valor: String,
    puntero: &'a String,
}