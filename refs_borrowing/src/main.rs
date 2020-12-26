fn main() {

	// Para "solucionar" e ter uma forma facil de manter a variavel no escopo
	// é possivel utilizar a referencia da variavel para a manupilação,
	// seguindo o exemplo anteriror

	let mut s = String::from("texto");

	// Aqui eu indo para função, qual é o valor que seu parametro dev apontar que é o ponteiro de 's'
	// Por isso que 's' ainda continua no escopo
	// Ver coments de 'tamanho_string()'
	// Em outras palavras passamos a referencia de 's' para a função, sem tomar sua posse
	// Esse tipo de operação é chamado de Borrowing(emprestimo)
	let tamanho = tamanho_string(&s);
	
	println!("texto: {}, tamanho: {}", s, tamanho);

	// Se quisermos alterar uma referencia temos que indicar o 'mut' na passagem da referencia
	modifica_borrowing_mut(&mut s);

	// A referencia mutavel só é permitida uam vez para cada variavel mutavel dentro do escpo
	let mut s2 = String::from("texto");

	let s3 = &mut s2; // --> Compila
	// let s4 = &mut s2; // --> Não compila, pois se deve usar a primeiro referencia mutavel
						 // para que não ocorra em dois lugares do codigo 
						 // a alteração na varivel no mesmo lugar do heap no memso momento

	println!("s3: {}", s3);

	// mas é possivel criando um novo escopo refazer a referencia

	{
		// Esse trecho de código irá compilar pois está dentro de um escopo garentido que a referencia
		// não vai ser alterada/acessada no mesmo mo mento que esse trecho excutar
		let s5 = &mut s2;
		println!("s5: {}", s5);	
	}


	// Outro ponto importante é que quando a referencia é só de leitura, ela pode ser feita varias vezes
	// mas se isso acontecer não é possivel fazer uma referencia mutavel 
	// pois atrela aos poblemas anteriores, esses problemas são conhecidos como "data race"
	// e.g:

	let mut s6 = String::from("texto");

	let r1 = &s6; 
	let r2 = &s6; 
	// let r3 = &mut s6; // Não é possivel utilizar, as decima é utilizadas ao mesmo tempo que 'r3'

	//println!("{}, {}, and {}", r1, r2, r3);
	println!("{} {}", r1, r2);


	// Mas para casos que a utilização da mutavel for depois da utilização das somente leitura
	// a referencia mutavle funciona
	let r4 = &s6;
	let r5 = &s6;
	println!("{} {}", r4, r5);

	let r6 = &mut s6;
	println!("{}", r6);


	// Em liguagens que o programador, gerencia os ponteiros é possivel fazer ponteiros soltos
	// o maior exemplo é uma função que retorna uma referencia de uma variavel que deveria existir
	// somente no escopo da função
	// let tentativa_referencia = tentativa_referencia_fn(); // --> Ler coments dessa função

	
	

}

// Aqui ao inves de dizer que o tipo do parametro é 'String'
// Diz que o tipo de parametro é '&String'
// Isso que dizer que, ao inves de receber o move do ponteiro de alguma variavel
// é coomo se a função recebesse somente o "valor" contido nesse ponteiro
// teoricamente é como : texto -> varaivel passada -> ponteiro
// como se o texto apontace para a variavel acima no escpo da função,
// assim não movendo para dentro da função!!!!
fn tamanho_string(texto: &String) -> usize {
	let size = texto.len();
	size	
}


// Esse metodo não funciona, pois quando pegamos algo emprestado não é possivel altera-lo!!!!!
//fn modifica_borrowing(s: &String) {
//	s.push_str(" modificado");
//}


// Diferente do de cima, é q esse metodo irar compilar 
// pois estamos idicando que estamos recebendo um emprestimo que pode ser alterado
fn modifica_borrowing_mut(s: &mut String) {
	s.push_str(" modificado");
}


// Essa função não compilará, pois não é possivel retornar a referencia da variavel que irar morrer
// no fim da excução da função
//fn tentativa_referencia_fn() -> &String{
//	let s = String::from("texto");
//	&s
//}
