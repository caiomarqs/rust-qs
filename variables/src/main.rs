fn main() {

    let mut x = 5;
	println!("x: {}", x);

	x = 6;
	println!("x: {}", x);


    // Constatnes em rust, são declaradas com a palavra reservada 'const'
    // Elas se diferencial das variaveis imutaveis em:
    // - Elas uma vez declarasdas, são sempre, mas sempre constantes
   	//	 diferentes das variaveis imutaveis que podem receber o 'mut'
   	// - Devem ser declaradas com o tipo de dado que a constate armazena
   	// e.g: const NUMERO: u32 = 100;

	const VALOR_MAXIMO: u32 = 100;

 	println!("VALOR_MAXIMO: {}", VALOR_MAXIMO);



 	// ----- Sombreamento ou Shadowing -----
	// Isso acontece uma variavel imutavel é declarada,
	// e se declara outra variavel imutavel com o memso nome
	// e.g:
	// 		let varaivel = 5;
	// 		let varaivel = variavel + 5;
	// O que permanece é o último valor declado antes de seu uso!

	let y = 5;
	let y = y + 1;
	let y = y * 2; // y --> 12

	println!("y: {}", y);


	// A Diferença entre declarar a varaivel imuatevel duas vezes e utilizaro 'mut' é:
	// O TIPO da varivale delarada 2x pode ser alterado e.g:
	// 		let foo = "bar";
	//		let foo = bar.len(); // -> Retorna o tamanho da String
	// Agora com o o mut não é possivel realizar isso:
	//		let mut foo = "bar";
	//		foo = bar.len(); --> *Erro de compilação* pois o valor pode ser alterado mas o tipo não
	// Indico o uso de shadown quando a variavel é manipulado sem seu uso previo!!

	let foo = "bar";
	let foo = foo.len();

	println!("foo: {}", foo);


	let mut bar = "foo";
	println!("bar: {}", bar);

	bar = "bar";
	println!("bar: {}", bar);	
}
