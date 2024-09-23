mod calculadora; // Muchas operaciones matemáticas
use calculadora::{sumar as sum, restar as res, multiplicar as mul, division as divi, potencia as pote,Salida as Sal};

mod validaciones;   // Declaro en módulo validaciones
use validaciones as vali; // Renombro el módulo validaciones por "vali"

mod view_calculador;

fn main() {
  view_calculador::user_calduladora();
}