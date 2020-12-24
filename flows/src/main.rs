// Controles de fulxo são blocos de controle: if, while, loop, for ....
// são aqueles que depende de uma condição para sua excução
fn main() {
	let n = 5;
	
	// o if em rust não nececita dos (...), para expressar a condição do bloco
	if n <= 5 {
		println!("n é menor ou igual a 5 | n: {}", n);
	}
	else {
		println!("n é maior do que 5 | n: {}", n);
	}

	primeiro_divisor(6);
	divisil_por(6);
	divisil_por(5);
	println!("if let: {}", if_let(true));
}

fn primeiro_divisor(dividendo: i32) {

	println!("----- O número : {} -----", dividendo);

	// Rust tem blocos de elif que nem todas as linguagens
	// como são encadeados cai na prieira condição verdadeira.
	// Ponto importante dos elif, é que muitos não é boa pratica,
	// existe um bloco de expração mais adquado, o 'match'
	if dividendo % 4 == 0 {
		println!("é divisivel por 4")
	}
	else if dividendo % 3 == 0 {
		println!("é divisivel por 3")
	}
	else if dividendo % 2 == 0 {
		println!("é divisivel por 2")
	}
	else {
		println!("não é divisivel por 4, 3 ou 2")
	}
}

fn divisil_por(dividendo: i32) {

	println!("----- O número : {} -----", dividendo);
	
	// Esse diferente do de cima não é encadeado, então cai em todas as condições verdadeiras
	if dividendo % 4 == 0 {
		println!("é divisivel por 4")
	}
	if dividendo % 3 == 0 {
		println!("é divisivel por 3")
	}
	if dividendo % 2 == 0 {
		println!("é divisivel por 2")
	}
	if dividendo % 4 != 0 && dividendo % 3 != 0 && dividendo % 2 != 0 {
		println!("não é divisivel por 4, 3 ou 2")
	}
}

// Aparentemente o rust não tem expressão ternaria, mas tem algumas outras expressões interessantes
// Como o 'if let', que basicamente armazena o retorno de um if em uma varaiavel
// O interessante é que os dois caminhos do if devem retonar uma expressão de mesmo tipo
// Por que em tempo de compilação o Rust avalia, qual é o valor que a varavel tem
// e se é valida em todos os pontos do codigo
// Pode se fazer paralelo com o typescript, que por conta do JS, 
// premite que uma varivel recebar "dois" tipos, 
// mas necessita de muitas verificações no codigo por conta disso
fn if_let(condicao: bool) -> i32{
	let retorno = if condicao {
		5
	}
	else {
		6
	};

	retorno
}
