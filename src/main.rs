use std::io;

fn fahrenheit_a_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_a_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn leer_numero() -> f64 {
    loop {
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la línea");

        match entrada.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => {
                println!("Por favor, introduce un número válido:");
            }
        }
    }
}

fn main() {
    println!("Conversor de temperaturas (Celsius ↔ Fahrenheit)");
    println!("Elige una opción:");
    println!("1) Celsius a Fahrenheit");
    println!("2) Fahrenheit a Celsius");

    let mut opcion = String::new();
    io::stdin()
        .read_line(&mut opcion)
        .expect("Error al leer la opción");

    let opcion = opcion.trim();

    match opcion {
        "1" => {
            println!("Introduce la temperatura en Celsius:");
            let c = leer_numero();
            let f = celsius_a_fahrenheit(c);
            println!("{c} °C son {f} °F");
        }
        "2" => {
            println!("Introduce la temperatura en Fahrenheit:");
            let f = leer_numero();
            let c = fahrenheit_a_celsius(f);
            println!("{f} °F son {c} °C");
        }
        _ => {
            println!("Opción no válida");
        }
    }

}
