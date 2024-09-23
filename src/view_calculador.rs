mod calculadora; // Muchas operaciones matemáticas
use calculadora::{sumar as sum, restar as res, multiplicar as mul, division as divi, potencia as pote,Salida as Sal};
use crate::calculadora::Salida;
mod validaciones;   // Declaro en módulo validaciones
use validaciones as vali; // Renombro el módulo validaciones por "vali"

pub fn user_calduladora(){
  let mut historial: Vec<Vec<calculadora::Salida>> = vec![];
  loop {
    let opcion: i8;
    println!("1: Sumar números.");
    println!("2: Restar números.");
    println!("3: Multiplicar números.");
    println!("4: Dividir número.");
    println!("5: Potencia de un número.");
    println!("6: Raiz de un número.");
    println!("7: Historial operaciones.");
    println!("8: Salir.");
    println!("Eliga una opción: ");
    opcion = vali::int() as i8;
    if opcion < 1 || opcion > 8 {
      println!("La opción no está disponible, ingrese una opción válida.")
    } else if opcion == 8{
      println!("Saliendo del programa.");
      break;
    } else if opcion == 7{
      println!("Mostrando Historial.");
      for uni in &historial{
        print!("[");
        let len = uni.len();
        for (i, bidi) in uni.iter().enumerate() {
          match bidi{
            calculadora::Salida::IntMin(min) => print!("{} ",min),
            calculadora::Salida::IntMax(max) => print!("{} ",max),
            calculadora::Salida::Float(ff) => print!("{} ",ff),
            calculadora::Salida::Err(ss) => print!("{} ",ss),
            calculadora::Salida::Signo(sig) => print!("{} ",sig),
          }
          if i == len - 2 {
            print!("= ");
          }
        }
        println!("]");
      }
    } else {
      println!("Ingrese el primer número: ");
      let uno = vali::int();
      println!("Ingrese el segúndo número: ");
      let dos = vali::int();
      let mut signo = String::new();
      let resultado = match opcion  {
        1 => {signo = "+".to_string(); sum(uno,dos)},
        2 => {signo = "-".to_string(); res(uno, dos)},
        3 => {signo = "*".to_string(); mul(uno, dos)},
        4 => {signo = "/".to_string(); divi(uno, dos)},
        5 => {signo = "^".to_string(); pote(uno, dos)},
        6 => {signo = "^1/".to_string(); calculadora::raiz(uno, dos)},
        _ => calculadora::Salida::Err("Opción no válida".to_string())
      };
      match resultado {
        calculadora::Salida::IntMin(val) => println!("El resultado es: {}", val),
        calculadora::Salida::IntMax(val) => println!("El resultado es: {}", val),
        calculadora::Salida::Float(val) => println!("El resultado es: {}", val),
        calculadora::Salida::Signo(ref val) => println!("El resultado es Signo?: {}", val),
        calculadora::Salida::Err(ref msg) => println!("Error: {}", msg),
      }
      historial.push(calculadora::newlista(&Sal::IntMin(uno),&Sal::Signo(signo),&Sal::IntMin(dos),&resultado.clone()));
    }
  }
}