use rand::Rng;
use std::f32::consts::PI;
use std::io;

fn main() {

    println!("Ingresar el número de individuos de la población:");

    let mut poblacion = String::new();

    io::stdin()
        .read_line(&mut poblacion)
        .expect("Error al leer la entrada");

    // Parsear la cadena a un número entero de 32 bits
    let poblacion_numero: i32 = match poblacion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No ingresaste un número válido.");
            return;
        }
    };

    println!("Ingresaste el número: {}", poblacion_numero);

    let mut matriz: Vec<Vec<f32>> = Vec::new();

    for _ in 0..poblacion_numero{
        let vector = iniciar_individuo();
        
        println!("El contenido del vector es: {:?}", vector);
        let resultado = fun(vector.clone());
        println!("{}", resultado);

        matriz.push(vector);
    }

    

}

fn iniciar_individuo() -> Vec<f32>{
    // inicializacion de individuo no? (10 randoms? entre -5.12 y +5.12)
    let mut vector: Vec<f32> = Vec::new();

    for _ in 0..10 {
        let numero_random = rand::thread_rng().gen_range(-5.12..5.12) as f32;
        let numero_random_redondeado = (numero_random * 1000.0).round() / 1000.0;
        vector.push(numero_random_redondeado); // bajar presicion a 3 digitos?
    }

    return vector;
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

fn posicion_padre_ruleta(x: &Vec<f32>) -> i32 {
    let t: f32 = x.iter().sum();
    let r: f32 = rand::thread_rng().gen_range(0.0..t);
    let mut posicion_padre = 0;

    let mut sum: f32 = 0.0;
    for i in x {
        sum += i;
        if sum >= r {
            break;
        }
        posicion_padre += 1;
    }

    return posicion_padre;
}