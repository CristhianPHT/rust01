use std::io;
pub fn int() -> i32{
  let numero:i32;    // Declarando la variable global
  loop {
    let mut input = String::new();
    // println!("Por favor ingresa un número entero positivo:");
    io::stdin().read_line(&mut input).expect("falló en ingresar la entrada.");
    numero = match input.trim().parse::<i32>() {
      Ok(num) if num >= 0 => num,
      _ => continue,
    };
    break;
  }
  numero
}

// #[cfg(test)]
// mod test {
//   use super::*;
//   #[test]
//   fn test_int(){
//     assert_eq!(int(), 1)
//   }
// }