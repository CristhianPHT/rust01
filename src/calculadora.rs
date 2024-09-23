#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum Salida{
  IntMin(i32),
  IntMax(i64),
  Float(f32),
  Signo(String),
  Err(String),
}
pub fn newlista(uno: &Salida,sig: &Salida, dos: &Salida, resul: &Salida) -> Vec<Salida> {
  let mut lista: Vec<Salida> = Vec::new();
  lista.push(uno.clone());
  lista.push(sig.clone());
  lista.push(dos.clone());
  lista.push(resul.clone());
  lista
}
pub fn sumar(x: i32, y:i32) -> Salida{
  Salida::IntMin(x+y)
}
pub fn restar(x: i32, y:i32) -> Salida{
  Salida::IntMin(x-y)
}
pub fn multiplicar(x: i32, y:i32) -> Salida{
  Salida::IntMin(x*y)
}
pub fn division(x:i32, y:i32) -> Salida {
  if y==0{
    Salida::Err("No puede dividirse con 0.".to_string())
  } else {
    let a:f32=x as f32; let b:f32=y as f32;
    Salida::Float(a/b)
  }
}
pub fn potencia(x:i32, y:i32) -> Salida{
  let mut resultado:i64 = 1;
  for _ in 0..y{
    resultado *= x as i64;
  }
  Salida::IntMax(resultado)
}
pub fn raiz(x:i32, y:i32) -> Salida {
  if x < 0 {
    Salida::Err("El número no puede ser negativo.".to_string())
  } else {
    let a: f32 = x as f32; let b: f32 = y as f32;
    Salida::Float(a.powf(1.0/b))
  }
}

// Módulo de pruebas
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar() {
        assert_eq!(sumar(2, 3), Salida::IntMin(5));
    }

    #[test]
    fn test_restar() {
        assert_eq!(restar(5, 2), Salida::IntMin(3));
    }

    #[test]
    fn test_multiplicar() {
        assert_eq!(multiplicar(4, 5), Salida::IntMin(20));
    }

    #[test]
    fn test_division() {
        assert_eq!(division(10, 2), Salida::Float(5.0));
        assert_eq!(division(10, 0), Salida::Err("No puede dividirse con 0.".to_string()));
    }

    #[test]
    fn test_potencia() {
      assert_eq!(potencia(2, 3), Salida::IntMax(8));
    }

    #[test]
    fn test_raiz() {
      assert_eq!(raiz(27, 3), Salida::Float(3.0));
      assert_eq!(raiz(-10, 5), Salida::Err("El número no puede ser negativo.".to_string()))
    }
}