extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Adivinhe o número!");
	
	// 'let' sem o 'mut' para constantes
	// Necessta da importacção da class/struct Rng;
	let num_secreto = rand::thread_rng().gen_range(1, 101);

	// println!("O número secreto é {}", num_secreto);

	// o loop excuta até que exista um break, ou uma exeção
	loop {
		println!("Por Favor insira seu chute: ");

		// Váriaveis precisam do "mut" para declara como mutaveis
		let mut chute = String::new(); // -> new String();
									   // os '::' são para chamar funções associadas
									   // funcoes associadas == metodos estáticos                         

		// Em Rust, se usa muito API flunce, 
		// que são essas chamdas encadeas de metódos
		io::stdin()
			.read_line(&mut chute) // '&' é o simbolo para indicar uma referencia
			.expect("Não foi possivel ler o chute!");
		
		// Sobreescrita de variaveis, setando como constante
		// O expect, está presente em quese todo objeto,
		// É quase um tradicional try/catch
		// let chute: u32 = chute.trim().parse()
		//	.expect("Por Favor insira um número!");


		// Trocando um .expect pelo o match, voce trata o erro, e não cospe ele na pilha
		// Match paternrs podem ser adicionados a constantes
		// Assim é possivel fazer o tratamento na declaração da constante
		let chute: u32 = match chute.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Por favor insira um número, tente novamente");
				continue; // para continuar o loop
			}
		};
		
		// Template string --> "string {}", variavel
		println!("Você chutou {}", chute);
		
		
		// Match parece ser um switch case o match necessita sempre, de todas as opções possivies
		// cmp compare -> Referente ao qualquer tipo de objeto
		// Sempre que for indicar uma variavel já declada também deve se usar o '&'
		match chute.cmp(&num_secreto) {
			Ordering::Less => println!("O seu número está abaixo"),
			Ordering::Greater => println!("O seu número está acima"),
			Ordering::Equal => {
				println!("Você acertou!!");
				break;
			} 
		}	
	}
}
