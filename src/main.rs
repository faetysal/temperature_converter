use std::{io, fmt};

#[derive(Debug)]
#[allow(dead_code)]
enum TemperatureScaleKind {
  Celcius(String),
  Kelvin(String),
  Fahrenheit(String)
}

impl TemperatureScaleKind {
  fn unit(&self) -> String {
    match self {
      Self::Celcius(u) | Self::Kelvin(u) | Self::Fahrenheit(u) => u.to_string()
    }
  }
}
  
#[derive(Debug)]
#[allow(dead_code)]
struct TemperatureScale {
  min: f32,
  max: f32,
  kind: TemperatureScaleKind
}


impl TemperatureScale {
  fn from(unit: &str) -> Self {
    if unit == "f" {
      Self { min: 32.0, max: 212.0, kind: TemperatureScaleKind::Fahrenheit(String::from("°F")) }
    } else if unit == "k" {
      Self { min: 273.15, max: 373.15, kind: TemperatureScaleKind::Kelvin(String::from("K")) }
    } else {
      Self { min: 0.0, max: 100.0, kind: TemperatureScaleKind::Celcius(String::from("°C")) }
    }
  }

  fn temperature(&self) -> (f32, f32) {
    (self.min, self.max)
  }


}

#[derive(Debug)]
struct Temperature {
  value: f32,
  scale: TemperatureScale
}

impl fmt::Display for Temperature {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let scale_unit = self.scale.kind.unit();
    write!(f, "{}{}", self.value, scale_unit)
  }
}

impl Temperature {
  fn new(value: f32, unit: &str) -> Self {
    Self {
      value,
      scale: TemperatureScale::from(unit)
    }
  }

  fn to(&self, unit: &str) -> Self {
    let scale: TemperatureScale = TemperatureScale::from(unit);
    let (x0, x1) = self.scale.temperature();
    let (y0, y1) = scale.temperature();
    let result = ( ((self.value - x0) * (y1 - y0)) / (x1 - x0) ) + y0;

    Self {
      value: result,
      scale
    }
  }
}

fn main() {
  println!("\n{x} Temperature Converter {x}\n", x = "#".repeat(5));
  println!("(C = Celcius, K = Kelvin, F = Fahrenheit");

  let input_unit = unit_prompt("Enter input unit (C/K/F):");
  let output_unit = unit_prompt("Enter output unit (C/K/F):");

  let input: f32 = loop {
    println!("Enter input value:");
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    match input.trim().parse() {
      Ok(num) => break num,
      Err(_) => {
        println!("Invalid number");
        continue
      }
    };
  };

  let input_temperature = Temperature::new(input, &input_unit);
  let output_temperature = input_temperature.to(&output_unit);

  println!("\n{} -> {} = {}", 
    input_temperature, 
    output_temperature.scale.kind.unit(), 
    output_temperature
  );

  // println!("75C -> F = {}", output_temperature);
}

fn unit_prompt(message: &str) -> String {
  loop {
    println!("{message}");
    let mut input_unit = String::new();
    io::stdin()
      .read_line(&mut input_unit)
      .expect("Failed to read line");

    let x = input_unit.trim().to_lowercase();
    match  x.as_str() {
      "c" | "k" | "f" => break x,
      _ => {
        println!("Invalid unit");
        continue 
      }
    };
  }
}
