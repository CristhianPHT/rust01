mod calculadora; // Muchas operaciones matemáticas
use calculadora::{sumar as sum, restar as res, multiplicar as mul, division as divi, potencia as pote};
// mod sumatoria;
mod validaciones;   // Declaro en módulo validaciones
use validaciones as vali; // Renombro el módulo validaciones por "vali"

fn user_calduladora(){
  loop {
    let opcion: i8;
    println!("1: Sumar números.");
    println!("2: Restar números.");
    // println!("3: Multiplicar números.");
    // println!("4: Dividir número.");
    // println!("5: Potencia de un número.");
    // println!("6: Raiz de un número.");
    // println!("7: Historial operaciones.");
    // println!("8: Salir.");
    println!("Eliga una opción: ");
    opcion = vali::int() as i8;
    if opcion < 1 || opcion > 8 {
      println!("La opción no está disponible, ingrese una opción válida.")
    } else if opcion == 8{
      println!("Saliendo del programa.");
      break;
    } else {
      println!("Ingrese el primer número: ");
      let uno = vali::int();
      println!("Ingrese el segúndo número: ");
      let dos = vali::int();
      let resultado = match opcion  {
        1 => sum(uno,dos),
        2 => res(uno, dos),
        3 => mul(uno, dos),
        4 => divi(uno, dos),
        5 => pote(uno, dos),
        6 => calculadora::raiz(uno, dos),
        // 7 => Result<String>(println!("Historial.")),
        _ => calculadora::Salida::Err("Opción no válida".to_string())
      };
      match resultado {
        calculadora::Salida::IntMin(val) => println!("El resultado es: {}", val),
        calculadora::Salida::IntMax(val) => println!("El resultado es: {}", val),
        calculadora::Salida::Float(val) => println!("El resultado es: {}", val),
        calculadora::Salida::Err(msg) => println!("Error: {}", msg),
      }
      // println!("El resultado es: {:?}", resultado)
    }
  }
}

fn main() {
  user_calduladora();
  // let seg: i32 = vali::int();
  // println!("Elementos; primero: {}, segundo: {}",num,seg);
  // println!("Sumar: {}\nRestar: {}\nMultiplicar: {}\nDividir: {:?}",sum(num,seg),res(num,seg),mul(num,seg),divi(num,seg))
  // println!("La suma de pares positivos hasta {} es: {}", numero, sumar::pares_n(numero));
}