use wasm_bindgen::prelude::*;
use web_sys::js_sys;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Nodo{
    nombre: String,
    aristas: Vec<(String, f64)>
}

#[wasm_bindgen]
impl Nodo {
    #[wasm_bindgen(constructor)]
    pub fn new(nombre: String, aristas: Vec<JsValue>) -> Nodo {
        let aristas = aristas
            .into_iter()
            .map(|val| {
                let arr = val.dyn_into::<js_sys::Array>().unwrap();
                let dest = arr.get(0).as_string().unwrap();
                let peso = arr.get(1).as_f64().unwrap();
                (dest, peso)
            })
            .collect();
        Nodo { nombre, aristas }
    }

    #[wasm_bindgen(getter)]
    pub fn nombre(&self) -> String {
        self.nombre.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn aristas(&self) -> js_sys::Array {
        let array = js_sys::Array::new();
        for (dest, peso) in &self.aristas {
            let tuple = js_sys::Array::new();
            tuple.push(&JsValue::from_str(&self.nombre));
            tuple.push(&JsValue::from_str(dest));
            tuple.push(&JsValue::from_f64(*peso));
            array.push(&tuple);
        }
        array
    }
}

#[wasm_bindgen]
pub fn dijkstra_algorithm(grafo: Vec<Nodo>) -> Vec<Nodo>{
    grafo
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prueba1(){

    }
}