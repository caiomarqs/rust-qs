// Para os paremetros é necessario passar o tipo deles
fn outra_funcao(x: i32, y: i32) {
	println!("x: {}", x);
	println!("y: {}", y);
}

// Os retornos das funções em Rust são as últimas linhas sem o ';', 
// pois o ';' diferencia declarações de expressões
// Assim como outras linguagens de tipagem forte
// o tipo do retorno deve estar declarado na assinaturo do metodo
// e.g: 
// 		fn funcao_com_retorno() -> i32 {
//			.....
//		}
fn funcao_com_retorno(x: i32) -> i32 {
	x + 1
}

// A palavra resevado 'return' é possivel ser utilizada
// mas geralmente é utiliza para retornos dentro de condições especificas das funções
// ou até mesmo para retornos adiantatdos
fn funcao_return(x: i32) -> i32 {
	if x >= 5 {
		return 0
	}

	x + 1
}


fn main() {
	outra_funcao(5, 4);

	let retorno = funcao_com_retorno(5);
	
	println!("funcao com retorno: {}", retorno);

	// Expressões são algo que retorna algum valor como no exemplo 'funcao_com_retorno'
	// mas é possivel realizar expressões em declarações
	let y = {
		let x = 3;
		x + 1 //--> o y deverá conter o valor dessa expressão!!!!
	};

	println!("expressão em declaração: {}", y);

	println!("Funcão com return: {}", funcao_return(6));
	
}
