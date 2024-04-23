use plotters::prelude::*;
use rand::Rng;
use std::f32::consts::PI;
use std::io;

fn main() {
    let (poblacion_numero, probabilidad_mutacion_numero, generaciones_numero) = input_parametros();

    const EJECUCIONES: i32 = 30;
    let mut vec_min_ejecuciones: Vec<f32> = Vec::new();
    for ejecucion in 0..EJECUCIONES {
        println!("Ejecución número: {}", ejecucion);
        let min_ejecucion = generar_solucion(
            poblacion_numero,
            probabilidad_mutacion_numero,
            generaciones_numero,
            ejecucion,
        );
        vec_min_ejecuciones.push(min_ejecucion);
    }
    let index_mejor_ejecucion = vec_min_ejecuciones
        .iter()
        .position(|&x| x == *vec_min_ejecuciones.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
        .unwrap();

    let mejor_ejecucion = vec_min_ejecuciones[index_mejor_ejecucion];

    println!();
    println!("La mejor ejecución fue la número: {}", index_mejor_ejecucion);
    println!("Con un valor de: {}", mejor_ejecucion);
    println!(
        "El promedio de aptitudes: {}",
        vec_min_ejecuciones.iter().sum::<f32>() / vec_min_ejecuciones.len() as f32
    );
    println!(
        "mediana de las aptitudes: {:?}",
        mediana(vec_min_ejecuciones.as_mut()).unwrap()
    );
    println!(
        "desviación estándar es: {:?}",
        desviacion_estandar(vec_min_ejecuciones.as_mut()).unwrap()
    );
}

fn generar_solucion(
    poblacion_numero: usize,
    probabilidad_mutacion_numero: f32,
    generaciones_numero: usize,
    numero_ejecucion: i32,
) -> f32 {
    let mut matriz: Vec<Vec<f32>> = Vec::new();
    for _ in 0..poblacion_numero {
        let vector = iniciar_individuo();

        let resultado = fun(&vector);
        matriz.push(vector);
    }

    let mut hist_aptitudes: Vec<f32> = Vec::new();
    for _ in 0..generaciones_numero {
        let mut vec_aptitudes = Vec::new();

        for i in 0..poblacion_numero {
            let aptitud = fun(&matriz[i]);
            vec_aptitudes.push(aptitud);
        }

        // permutar basandose en la aptitud de los individuos
        let mut criterio_permutacion: Vec<usize> = (0..vec_aptitudes.len()).collect();
        criterio_permutacion
            .sort_by(|&a, &b| vec_aptitudes[a].partial_cmp(&vec_aptitudes[b]).unwrap());
        let matriz_ordenada: Vec<Vec<f32>> = criterio_permutacion
            .iter()
            .map(|&i| matriz[i].clone())
            .collect();
        // println!("NUEVA GENERACIÓN");

        let mut matriz_siguiente_generacion: Vec<Vec<f32>> = Vec::new();
        while matriz_siguiente_generacion.len() < poblacion_numero - 1 {
            let posicion_padre_1 = posicion_padre_ruleta(&vec_aptitudes) as usize;
            let posicion_padre_2 = posicion_padre_ruleta(&vec_aptitudes) as usize;
            let padre_1 = matriz[posicion_padre_1].clone();
            let padre_2 = matriz[posicion_padre_2].clone();

            cruza_blx(
                padre_1,
                padre_2,
                &mut matriz_siguiente_generacion,
                probabilidad_mutacion_numero,
                poblacion_numero,
            );
        }
        matriz_siguiente_generacion.push(matriz_ordenada[0].clone());
        matriz = matriz_siguiente_generacion;

        // Guardar la aptitud mínima de la generación
        let mut min = 0.0;
        if vec_aptitudes.is_empty() {
            None
        } else {
            min = vec_aptitudes
                .iter()
                .skip(1)
                .fold(vec_aptitudes[0], |min, &x| if x < min { x } else { min });
            Some(min)
        };
        hist_aptitudes.push(min);
    }

    let mut vec_aptitudes = Vec::new();

    for i in 0..poblacion_numero {
        let aptitud = fun(&matriz[i]);
        vec_aptitudes.push(aptitud);
    }

    // permutar basandose en la aptitud de los individuos
    let mut criterio_permutacion: Vec<usize> = (0..vec_aptitudes.len()).collect();
    criterio_permutacion.sort_by(|&a, &b| vec_aptitudes[a].partial_cmp(&vec_aptitudes[b]).unwrap());
    let matriz_ordenada: Vec<Vec<f32>> = criterio_permutacion
        .iter()
        .map(|&i| matriz[i].clone())
        .collect();
    let mejor_vector: Vec<f32> = matriz_ordenada[0].clone();
    let resultado_mejor = fun(&mejor_vector);

    println!("\nMejor matriz");
    _imprimir_matriz(&matriz);

    print!("El mejor hijo fue: ");
    imprimir_fila(&mejor_vector);
    println!("Con la aptitud siendo: {}", resultado_mejor);
    println!("Se termino la ejecución.");

    generar_grafico_aptitud(
        hist_aptitudes.clone(),
        generaciones_numero as i32,
        numero_ejecucion,
    )
    .unwrap();
    let mut min = 0.0;
    if hist_aptitudes.is_empty() {
        None
    } else {
        min = hist_aptitudes
            .iter()
            .skip(1)
            .fold(hist_aptitudes[0], |min, &x| if x < min { x } else { min });
        Some(min)
    };

    min
}

fn cruza_blx(
    x: Vec<f32>,
    y: Vec<f32>,
    matriz: &mut Vec<Vec<f32>>,
    mutacion: f32,
    poblacion: usize,
) {
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

    if matriz.len() < poblacion - 1 {
        matriz.push(hijo1);
    }
    if matriz.len() < poblacion - 1 {
        matriz.push(hijo2);
    }

    return;
}

fn mutar(hijo: Vec<f32>, mutacion: f32) -> Vec<f32> {
    let mut contador = 0;
    let mut hijo_mutado: Vec<f32> = Vec::new();

    for _ in 0..10 {
        let numero_random = rand::thread_rng().gen_range(0.0..1.0) as f32;

        if numero_random < mutacion {
            let valor_mutado = rand::thread_rng().gen_range(-5.12..5.12) as f32;
            let valor_mutado_redondeado = (valor_mutado * 100.0).round() / 100.0;

            hijo_mutado.push(valor_mutado_redondeado);
        } else {
            hijo_mutado.push(hijo[contador]);
        }

        contador = contador + 1;
    }

    return hijo_mutado;
}

fn posicion_padre_ruleta(x: &Vec<f32>) -> i32 {
    let mut aptitud_inversa: Vec<f32> = Vec::new();
    for &aptitud in x {
        aptitud_inversa.push(1.0 / aptitud); // Calcula el inverso de la aptitud
    }

    let t: f32 = aptitud_inversa.iter().sum();
    let r: f32 = rand::thread_rng().gen_range(0.0..t);
    let mut posicion_padre = 0;

    let mut sum: f32 = 0.0;
    for i in aptitud_inversa {
        sum += i;
        if sum >= r {
            break;
        }
        posicion_padre += 1;
    }

    return posicion_padre;
}

fn iniciar_individuo() -> Vec<f32> {
    let mut vector: Vec<f32> = Vec::new();

    for _ in 0..10 {
        let numero_random = rand::thread_rng().gen_range(-5.12..5.12) as f32;
        let numero_random_redondeado = (numero_random * 100.0).round() / 100.0;
        vector.push(numero_random_redondeado);
    }

    return vector;
}

fn fun(x: &Vec<f32>) -> f32 {
    const D: f32 = 10.0;
    let mut resultado: f32 = 10.0 * D;

    for i in 0..10 {
        resultado += (x[i] * x[i]) - (10.0 * (f32::cos(2.0 * PI * x[i])));
    }

    return resultado;
}

fn generar_grafico_aptitud(
    hist_aptitudes: Vec<f32>,
    generaciones: i32,
    num_ejecucion: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("images/{}.png", num_ejecucion);
    let root = BitMapBackend::new(&path, (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("aptitud por generacion", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..generaciones as f32, 0f32..200f32)?;
    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        hist_aptitudes
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, *y)),
        &RED,
    ))?;
    root.present()?;

    Ok(())
}

fn mediana(vec: &mut Vec<f32>) -> Option<f32> {
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = vec.len();
    if len == 0 {
        return None;
    }
    let mid = len / 2;
    if len % 2 == 0 {
        Some((vec[mid - 1] + vec[mid]) / 2.0)
    } else {
        Some(vec[mid])
    }
}

fn desviacion_estandar(vec: &mut Vec<f32>) -> Option<f32> {
    let mediana = vec.iter().sum::<f32>() / vec.len() as f32;
    let varianza = vec.iter().map(|x| (x - mediana).powi(2)).sum::<f32>() / vec.len() as f32;
    Some(varianza.sqrt())
}

fn input_parametros() -> (usize, f32, usize) {
    println!("Ingresar el número de individuos de la población:");
    let mut poblacion = String::new();
    io::stdin()
        .read_line(&mut poblacion)
        .expect("Error al leer la entrada");
    let poblacion_numero: usize = match poblacion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No ingresaste un número válido.");
            0
        }
    };
    println!("Ingresar la probabilidad (0.0 al 1.0) de mutar: ");
    let mut probabilidad_mutacion = String::new();
    io::stdin()
        .read_line(&mut probabilidad_mutacion)
        .expect("Error al leer la entrada");
    let probabilidad_mutacion_numero: f32 = match probabilidad_mutacion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No ingresaste un número válido.");
            0.0
        }
    };
    println!("Ingresar el numero de generaciones a generar: ");
    let mut generaciones = String::new();
    io::stdin()
        .read_line(&mut generaciones)
        .expect("Error al leer la entrada");
    let generaciones_numero: usize = match generaciones.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No ingresaste un número válido.");
            0
        }
    };
    println!("Ingresaste la población de {} individuos", poblacion_numero);
    println!(
        "Ingresaste la probabilidad de mutar de: {}",
        probabilidad_mutacion_numero
    );
    println!(
        "El número de generaciones que tendra seran de: {}",
        generaciones_numero
    );

    (poblacion_numero, probabilidad_mutacion_numero, generaciones_numero)
}

fn _imprimir_matriz(matriz: &Vec<Vec<f32>>) {
    for fila in matriz {
        print!("Gen del vector: ");
        imprimir_fila(fila);
        let resultado = fun(&fila);
        println!("Resultado: {} ", resultado);
    }
}

fn imprimir_fila(fila: &[f32]) {
    for &elemento in fila {
        print!("{} ", elemento);
    }
    println!();
}