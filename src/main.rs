use rand::Rng;
use std::f32::consts::PI;

fn main() {
    // inicializacion de poblacion? (10 randoms? entre -5.12 y +5.12)
    let mut vector: Vec<f32> = Vec::new();
    for _ in 0..10 {
        let numero_random = rand::thread_rng().gen_range(-5.12..5.12) as f32;
        vector.push(numero_random); // bajar presicion a 3 digitos?
    }

    let resultado = fun(vector);
    println!("{}", resultado)
}

fn fun(x: Vec<f32>) -> f32 {
    const D: f32 = 10.0;
    let mut resultado: f32 = 10.0 * D;

    //sumatoria
    for i in 0..10 {
        resultado += (x[i] * x[i]) - (10.0 * (f32::cos(2.0 * PI * x[i])));
    }

    return resultado;
}
