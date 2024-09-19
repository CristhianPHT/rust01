// Escribe una función que reciba un número entero positivo n y devuelva la suma de todos los números pares desde 1 hasta n.
use std::io;
fn sumar(x:i32) -> i32 {
  let mut y = 0;
  for i in 0..=x{
    if i % 3 == 0{
      y +=i;
    }
  }
  y
}
fn main() {
  let mut input = String::new();
  println!("Por favor ingresa un número entero positivo:");
  io::stdin().read_line(&mut input).expect("falló en ingresar la entrada.");
  let num: i32 = 
    match input.trim().parse() {
      Ok(num) if num > 0 => num,
      _ => {
        println!("Entrada inválida, por favor ingresa un número entero positivo.");
        return;
      }
    };
  let mostrar = sumar(num);
  println!("La suma de los números pares desde 1 hasta {} es: {}", num, mostrar);
}