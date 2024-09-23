// Vectores en box (caja? python?)
use std::any::Any;

pub fn imprimir_vectores() {
    let mut vectores: Vec<Box<dyn Any>> = Vec::new();
    vectores.push(Box::new(1));
    vectores.push(Box::new(2.5));
    vectores.push(Box::new("Hola"));

    // Para imprimir, necesitas hacer un downcast
    for item in vectores {
        if let Some(int) = item.downcast_ref::<i32>() {
            println!("Entero: {}", int);
        } else if let Some(float) = item.downcast_ref::<f64>() {
            println!("Flotante: {}", float);
        } else if let Some(string) = item.downcast_ref::<&str>() {
            println!("Cadena: {}", string);
        }
    }
}

#[derive(Debug)]
enum NewTipo{
  Entero(i32),
  Decimal(f32),
  Texto(String),
  Binario(bool)
}
pub fn enumeracion(){
  let mut vector1: Vec<NewTipo> = vec![NewTipo::Entero(3),NewTipo::Decimal(3.3)];
  vector1.push(NewTipo::Binario(true));
  vector1.push(NewTipo::Texto(String::from("Hola?")));
  for x in &vector1{
    match x {
      NewTipo::Entero(i) => println!("Entero: {}",i),
      NewTipo::Decimal(d) => println!("Decimal: {}", d),
      NewTipo::Texto(t) => println!("Textual: {}",t),
      NewTipo::Binario(b) => println!("Booleano: {}",b)
    }
  }
  println!("Vectores enum: {:?}", vector1);
}