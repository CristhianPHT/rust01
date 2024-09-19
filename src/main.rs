mod calculadora;
// use calculadora::{sumar as sum, restar as res, multiplicar as mul, division as divi};
// use calculadora::raiz;
// mod sumatoria;
mod validaciones;   // Declaro en módulo validaciones
use validaciones as vali; // Renombro el módulo validaciones por "vali"

fn main() {
  let num: i32;
  num = vali::int();
  let seg: i32 = vali::int();
  println!("Elementos; primero: {}, segundo: {}",num,seg);
  // println!("Sumar: {}\nRestar: {}\nMultiplicar: {}\nDividir: {:?}",sum(num,seg),res(num,seg),mul(num,seg),divi(num,seg))
  // println!("La suma de pares positivos hasta {} es: {}", numero, sumar::pares_n(numero));
}