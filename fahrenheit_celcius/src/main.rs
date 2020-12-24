use std::io;
use std::any::type_name;

// Não entendo como funciona a aplicação de generics aqui
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn far_to_cel(far: i32) -> i32 {
	// Divisões só operão com f64????
	(f64::from(far - 32) * (5.0 / 9.0)) as i32
}

fn cel_to_far(cel: i32) -> i32 {
	(f64::from(cel) * (5.0 / 9.0) + f64::from(32))  as i32
}

fn far_convertion() -> (i32, i32) {

	println!("Insira a temperatura em Fahrenheit: ");

	// A varavel pode ter uma declaração tardia
	// Explicação de variavel não lidade e.g:
	// 		let mut far: i32 = 0;
	// até a atribuição de um novo valor para essa variavel, ela não é usada
	// então é melhor que a atribuição de valor seja dada tardiamente
	// declara a varavel sem atribuir valor
	let far: i32;

	loop {
		let mut input = String::new();
		    
		io::stdin()
				.read_line(&mut input)
				.expect("Não foi possivel ler o Fahrenheit!!");
				
		let input: i32 = match input.trim().parse() {
			Ok(n) => n,
			Err(_) => {
				println!("Insira uma temperatura válida para Fahrenheit, tente novamente: ");
				continue;
			}
		};

		if type_of(input) == "i32" {
			//Primeira atribuição de valor para essa variavel
			far = input;
			break;
		}
	}
	
	let celcius = far_to_cel(far);
	(far, celcius)
}


fn celcius_convertion() -> (i32, i32) {

	println!("Insira a temperatura em Celcius: ");
	let celcius: i32;

	loop {
		let mut input = String::new();
		    
		io::stdin()
				.read_line(&mut input)
				.expect("Não foi possivel ler o Celcius!!");

		let input: i32 = match input.trim().parse() {
			Ok(n) => n,
			Err(_) => {
				println!("Insira uma temperatura válida para Celcius, tente novamente: ");
				continue;
			}
		};

		if type_of(input) == "i32" {
			celcius = input;
			break;
		}
	}
	
	let far = cel_to_far(celcius);
	(far, celcius)
}


fn print_msg(far: i32, celcius: i32) {
	println!("\n| ºF\t| ºC\t|");
	println!("| {}\t| {:.0}\t|\n", far, celcius);
}


fn main() {
    println!("----- CONVERSOR DE TEMPERATURA: ºF <-> ºC -----");
	println!("Faça escolha de umas opções abaixo:");
	println!("1) ºF para ºC");
	println!("2) ºC para ºF");
	println!("3) Sair");
	
	let mut option: u32;
	
	loop {
		let mut input = String::new();

		io::stdin()
				.read_line(&mut input)
				.expect("Não foi possivel ler a opção!");

		option = match input.trim().parse() {
			Ok(o) => o,
			Err(_) => {
				println!("Insira uma opção valida");
				0
			}
		};

		if option == 1 {
			let (far, celcius) = far_convertion();
			print_msg(far, celcius);
			break;
		}
		if option == 2 {
			let (far, celcius) = celcius_convertion();
			print_msg(far, celcius);
			break;
		}
		if option == 3 {
			println!("Tchau!!");
			break;
		}				
	}		
}
