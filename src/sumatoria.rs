// Escribe una función que reciba un número entero positivo n
//  y devuelva la suma de todos los números pares desde 1 hasta n.

pub fn pares_n(x:i32) -> i32 {
  let mut y = 0;
  for i in 0..=x{
    if i % 2 == 0{
      y +=i;
    }
  }
  y
}

// Moodulo de Test
#[cfg(test)]
mod tests{
  use super::*;
  #[test]
  fn test_pares_n(){
    assert_eq!(pares_n(8), 20);
  }
}