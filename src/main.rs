use std::{io, fmt};

#[derive(Debug)]
enum TemperatureScale {
	Celcius { temperature: (f32, f32), unit: String }, // 0 - 100
	Kelvin { temperature: (f32, f32), unit: String }, // 273 - 373
	Fahrenheit { temperature: (f32, f32), unit: String }, // 32 - 212
	Invalid
}

/*impl fmt::Display for TemperatureScale {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Unit: {}", self.unit)
	}
}*/

impl TemperatureScale {
	fn from(unit: &str) -> Self {
		match unit {
			"c" => Self::Celcius { temperature: (0.0, 100.0), unit: String::from("C") },
			"k" => Self::Kelvin { temperature: (273.15, 373.15), unit: String::from("K") },
			"f" => Self::Fahrenheit { temperature: (32.0, 212.0), unit: String::from("F") },
			_ => Self::Invalid
		}
	}

	fn temperature(&self) -> (f32, f32) {
		match &self {
			TemperatureScale::Celcius { temperature: (t0, t1), unit: _ } |
			TemperatureScale::Kelvin { temperature: (t0, t1), unit: _ } | 	
			TemperatureScale::Fahrenheit { temperature: (t0, t1), unit: _ }
			=> (*t0, *t1),
			_ => (0.0, 0.0)
		}
	}


}

#[derive(Debug)]
struct Temperature {
	value: f32,
	scale: TemperatureScale
}

impl Temperature {
	fn new(value: f32, unit: &str) -> Self {
		Self {
			value,
			scale: TemperatureScale::from(unit)
		}
	}

	fn to(&self, other: TemperatureScale) -> Self {
		let (x0, x1) = self.scale.temperature();
		let (y0, y1) = other.temperature();
		let result = ( ((self.value - x0) * (y1 - y0)) / (x1 - x0) ) + y0;
		let scale: TemperatureScale = other;

		Self {
			value: result,
			scale
		}
	}
}

fn main() {
	println!("\n{x} Temperature Converter {x}\n", x = "#".repeat(5));
	println!("(C = Celcius, K = Kelvin, F = Fahrenheit");

	let input_unit = loop {
		println!("Enter input unit (C/K/F):");
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
	};

	let output_unit = loop {
		println!("Enter output unit (C/K/F):");
		let mut output_unit = String::new();
		io::stdin()
			.read_line(&mut output_unit)
			.expect("Failed to read line");

		let x = output_unit.trim().to_lowercase();
		match x.as_str() {
			"c" | "k" | "f" => {
				break x;
			},
			_ => {
				println!("Invalid unit");
				continue 
			}
		};
	};

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

	// let x:u32 = x.trim().parse().expect("Invalid number");
	println!("Input Unit: {input_unit}");
	println!("Output Unit: {output_unit}");
	println!("Input value: {input}");

	let input_temperature = Temperature::new(input, &input_unit);
	let output_temperature = input_temperature.to(TemperatureScale::from(&output_unit));

	dbg!(input_temperature);
	dbg!(output_temperature);

	// println!("75C -> F = {}", output_temperature);
}
