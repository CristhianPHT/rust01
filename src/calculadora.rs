pub fn sumar(x: i32, y:i32) ->i32{
  x+y
}
pub fn restar(x: i32, y:i32) ->i32{
  x-y
}
pub fn multiplicar(x: i32, y:i32) -> i32{
  x*y
}
pub fn division(x:i32, y:i32) -> Result<f32, String> {
  if y==0{
    Err("No puede dividirse con 0.".to_string())
  } else {
    let a:f32=x as f32; let b:f32=y as f32;
    Ok(a/b)
  }
}
pub fn potencia(x:i32, y:i32) -> i64{
  let resultado: i64 = (x as i64)**(y as i64);
  resultado
  // let a:i64 = x as i64; let b: i64 = y as i64;
  // a**b
}
pub fn raiz(x:i32, y:i32) -> Result<f32, String> {
  if x < 0 {
    Err("El número no puede ser negativo.".to_string())
  } else {
    let a: f32 = x as f32; let b: f32 = y as f32;
    Ok(a**(1.0/b))
  }
}

// Módulo de pruebas
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar() {
        assert_eq!(sumar(2, 3), 5);
    }

    #[test]
    fn test_restar() {
        assert_eq!(restar(5, 2), 3);
    }

    #[test]
    fn test_multiplicar() {
        assert_eq!(multiplicar(4, 5), 20);
    }

    #[test]
    fn test_division() {
        assert_eq!(division(10, 2), Ok(5.0));
        assert_eq!(division(10, 0), Err("No puede dividirse con 0.".to_string()));
    }

    #[test]
    fn test_potencia() {
      assert_eq!(potencia(2, 3), 8);
    }

    #[test]
    fn test_raiz() {
      assert_eq!(raiz(36, 2), Ok(6.0));
      assert_eq!(raiz(-10, 2), Err("El número no puede ser negativo.".to_string()))
    }
}