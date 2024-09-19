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
}