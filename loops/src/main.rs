// Rust tem tres lacos de repetição loop, while e for
fn main() {
	looping();
	tie_while();
	tie_for();
	tie_for_range();
}

fn looping() {
	let mut i = 0;

	// O loop executa até um break, se não tiver isso executará infinitamente
	// Neste caso é criado uma condição para o break
	loop {
		println!("loop!");
		i = i + 1;

		if i == 5 {
			break;
		}
	}
}

fn tie_while() {

	let mut i = 0;

	// O 'while' diferente do 'loop', a condição vai na assinatura do laço
	// Assim é possivel fazer o mesmo processo com menos linhas e sem a declaração do 'break'
	while i < 5 {
		println!("while!");
		i = i + 1;
	}

	// Percorre matizes com 'while' não é tão inteligente, pois para cada execução do loop
	// o compilador verefica, a condição verdadeira de cada execução!!
}


fn tie_for() {
	let lista = [10, 20, 30];

	// O for em Rust, é um foreach de uma linguagem procedural comum
	// ele declara um item com um valor para cada exceução do loop dento de Range ou Coleção
	for item in lista.iter() {
		println!("item: {}", item);
	}		
}

fn tie_for_range() {

	// Para a criaçãoo de coleções de numeros é possivel fazer (1..4)
	// Cria uma coleção com numeros nesse intervalo, esse seria [1,2,3]
	for n in (1..4).rev() {
		println!("for_range: {}", n);
	}
}
