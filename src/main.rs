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
    let poblacion_numero: usize = match poblacion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No ingresaste un número válido.");
            return;
        }
    };

    println!("Ingresar la probabilidad (0.0 al 1.0) de mutar: ");

    let mut probabilidad_mutacion = String::new();

    io::stdin()
        .read_line(&mut probabilidad_mutacion)
        .expect("Error al leer la entrada");

    // Parsear la cadena a un número entero de 32 bits
    let probabilidad_mutacion_numero: f32 = match probabilidad_mutacion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No ingresaste un número válido.");
            return;
        }
    };

    println!("Ingresar el numero de generaciones a generar: ");

    let mut generaciones = String::new();

    io::stdin()
        .read_line(&mut generaciones)
        .expect("Error al leer la entrada");

    // Parsear la cadena a un número entero de 32 bits
    let generaciones_numero: usize = match generaciones.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No ingresaste un número válido.");
            return;
        }
    };

    println!("Ingresaste la población de {} individuos", poblacion_numero);
    println!("Ingresaste la probabilidad de mutar de: {}", probabilidad_mutacion_numero);
    println!("El número de generaciones que tendra seran de: {}", generaciones_numero);

    let mut matriz: Vec<Vec<f32>> = Vec::new();

    for _ in 0..poblacion_numero{
        let vector = iniciar_individuo();
        
        println!("El contenido del vector es: {:?}", vector);
        let resultado = fun(vector.clone());
        println!("{}", resultado);

        matriz.push(vector);
    }

    for _ in 0..generaciones_numero{
        let mut vec_aptitudes = Vec::new();

        for i in 0..poblacion_numero {
            let aptitud = fun(matriz[i].clone());
            vec_aptitudes.push(aptitud);
        }

        // permutar basandose en la aptitud de los individuos
        let mut criterio_permutacion: Vec<usize> = (0..vec_aptitudes.len()).collect();
        criterio_permutacion.sort_by(|&a, &b| vec_aptitudes[a].partial_cmp(&vec_aptitudes[b]).unwrap());

        // Aplicar el criterio de permutación a la matriz
        let matriz_ordenada: Vec<Vec<f32>> = criterio_permutacion.iter().map(|&i| matriz[i].clone()).collect();

        println!("NUEVA GENERACIÖN");

        let mut matriz_siguiente_generacion: Vec<Vec<f32>> = Vec::new();

        while matriz_siguiente_generacion.len() < poblacion_numero - 1{
            let posicion_padre_1 = posicion_padre_ruleta(&vec_aptitudes) as usize;
            let posicion_padre_2 = posicion_padre_ruleta(&vec_aptitudes) as usize;
            let padre_1 = matriz[posicion_padre_1].clone();
            let padre_2 = matriz[posicion_padre_2].clone();

            cruza_blx(padre_1, padre_2, &mut matriz_siguiente_generacion, probabilidad_mutacion_numero, poblacion_numero);
        }
        
        matriz_siguiente_generacion.push(matriz_ordenada[0].clone());

        matriz = matriz_siguiente_generacion;
        

        imprimir_matriz(&matriz);
    }

    let mut vec_aptitudes = Vec::new();

        for i in 0..poblacion_numero {
            let aptitud = fun(matriz[i].clone());
            vec_aptitudes.push(aptitud);
        }


    let mut criterio_permutacion: Vec<usize> = (0..vec_aptitudes.len()).collect();
    criterio_permutacion.sort_by(|&a, &b| vec_aptitudes[a].partial_cmp(&vec_aptitudes[b]).unwrap());

    // Aplicar el criterio de permutación a la matriz
    let matriz_ordenada: Vec<Vec<f32>> = criterio_permutacion.iter().map(|&i| matriz[i].clone()).collect();
    let mejor_vector: Vec<f32> = matriz_ordenada[0].clone();
    let resultado_mejor = fun(mejor_vector.clone());

    print!("El mejor hijo fue: ");
    imprimir_fila(&mejor_vector);
    print!("Con la aptitud siendo: {}", resultado_mejor);
    println!("Se termino el programa.")

}

fn imprimir_matriz(matriz: &Vec<Vec<f32>>) {
    for fila in matriz {
        imprimir_fila(fila);
        let resultado = fun(fila.clone());
        println!("{} ", resultado);
    }
}

fn imprimir_fila(fila: &[f32]) {
    for &elemento in fila {
        print!("{} ", elemento);
    }
    println!();
}

fn cruza_blx(x: Vec<f32>, y: Vec<f32>, matriz: &mut Vec<Vec<f32>>, mutacion:f32, poblacion:usize){
    let mut contador = 0;
    let alfa = rand::thread_rng().gen_range(0.00..1.00) as f32;

    let mut hijo1: Vec<f32> = Vec::new();
    let mut hijo2: Vec<f32> = Vec::new();

    for _ in 0..10 {
        let maximo = x[contador].max(y[contador]);
        let minimo = x[contador].min(y[contador]);
        let diferencia = maximo - minimo;
        let diferencia_alfa = diferencia + alfa;

        let valor_minimo = if minimo - diferencia_alfa < -5.12 {
            -5.12
        } else {
            minimo - diferencia_alfa
        };

        let valor_maximo = if maximo + diferencia_alfa > 5.12 {
            5.12
        } else {
            maximo + diferencia_alfa 
        };

        let valor_hijo1 = rand::thread_rng().gen_range(valor_minimo..valor_maximo) as f32;
        let valor_hijo2 = rand::thread_rng().gen_range(valor_minimo..valor_maximo) as f32;
        let valor_hijo1_redondeado = (valor_hijo1 * 100.0).round() / 100.0;
        let valor_hijo2_redondeado = (valor_hijo2 * 100.0).round() / 100.0;

        hijo1.push(valor_hijo1_redondeado);
        hijo2.push(valor_hijo2_redondeado);

        contador = contador + 1;
    }

    hijo1 = mutar(hijo1, mutacion);
    hijo2 = mutar(hijo2, mutacion);

    if matriz.len() < poblacion - 1{
        matriz.push(hijo1);
    }
    if matriz.len() < poblacion - 1{
        matriz.push(hijo2);
    }

    return;
}  

fn mutar(hijo: Vec<f32>, mutacion: f32) -> Vec<f32> {
    let mut contador = 0;
    let mut hijo_mutado: Vec<f32> = Vec::new();

    for _ in 0..10{
        let numero_random = rand::thread_rng().gen_range(0.0..1.0) as f32;

        if numero_random < mutacion {
            let valor_mutado = rand::thread_rng().gen_range(-5.12..5.12) as f32;
            let valor_mutado_redondeado = (valor_mutado * 100.0).round() / 100.0;

            hijo_mutado.push(valor_mutado_redondeado);
        }
        else {
            hijo_mutado.push(hijo[contador]);
        }

        contador = contador + 1;
    }

    return hijo_mutado
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

fn iniciar_individuo() -> Vec<f32>{
    // inicializacion de individuo no? (10 randoms? entre -5.12 y +5.12)
    let mut vector: Vec<f32> = Vec::new();

    for _ in 0..10 {
        let numero_random = rand::thread_rng().gen_range(-5.12..5.12) as f32;
        let numero_random_redondeado = (numero_random * 100.0).round() / 100.0;
        vector.push(numero_random_redondeado);
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