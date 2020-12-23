fn main() {
    let mut  x = 5;
	println!("x: {}", x);
    let x = 6;
	println!("x: {}", x);


    // Constatnes em rust, são declaradas com a palavra reservada 'const'
    // Elas se diferencial das variaveis imutaveis em:
    // - Elas uma vez declarasdas, são sempre, mas sempre constantes
   	//	 diferentes das variaveis imutaveis que podem receber o 'mut'
   	// - Devem ser declaradas com o tipo de dado que a constate armazena
   	// e.g: const NUMERO: u32 = 100;

	const VALOR_MAXIMO: u32 = 100;

 	println!("VALOR_MAXIMO: {}", VALOR_MAXIMO);
}
