fn main() {
	// Tipos escalares --> são os tipos primarios: inteiros, ponto flutuante, bool, e chars


	// Tipos inteiros --> basicamentes são os numero n fracionarios, é são dependetes da arquitetura
	//					 8-bit, 16-bit, 32-bit ...., mas são diferenciados pele o peso do sinal do numero
	//					 existem os negativos e os positivos, e são diferntciadaos pelo o prefixo:
	//				     i --> para que possume negativos || u --> somente positivos
	// ---IMPORTANTE---
	// Variantes com sinal armazena pode armazanar a mesma quantidade de numeros que as variantes sem sinal
	// Mas existe um porem, os com sinais guardam ate a "metade"
	// para o positivo e negativo, do que a com o sinal guardaria
	// e.g:
	//		const POSITIVO: u8 = 255 //--> o u8 por exempo guarda de 0 a 255
	//		const NEGATIVO: i8 = 127 //--> com i8 não é possivel gardar ate 255,
									 //	   pq metade foi para o neagtivo então i8 vai de -128 a 127

	//Exemplos de inteiros
	const POSITIVO: u8 = 255;
	const NEGATIVO: i8 = -128;

	println!("POSITIVO: {}", POSITIVO);
	println!("NEGATIVO: {}", NEGATIVO);

	// Além dos tipos para as arquitetura padrões, exitem os 'isize' e o 'usize'
	// O 'u' e o 'i', usando a mesma lógica dos anterios,
	// mas o 'size' significa que o numero tem o limite da arquitetura do processador que está compilando
	// e.g:
	// 		const NUMERO: usize = 5555555 // --> i386 == u32
	//									  // --> AMD64 == u64
	//									  // --> ARM == u32
	//									  // --> ARM64 == u64
	// Desta forma este número pode gerar uma exção quando for maior do que a arquitetura permite processar

	const NUMERO: usize = 55555;
	const NUMERO64: u64 = NUMERO as u64; // -> possivel convesão entre usize e os outros inteiros
	println!("USIZE TYPE: {}", NUMERO64);	

	// Os inteiros conseguem receber alguns simbolos, para o usdo de Decimal, Hexa, Binario e Byte
	// _ -> Separador visual, para leitura do codigo
	// x -> para Hexa
	// o -> para octal
	// b -> binario
	// b'<char>' -> Byte

	let decimal_visual = 100_000;
	let hexadecial = 0xff;
	let octal = 0o77;
	let binario = 0b1111_0000;
	let byte = b'A'; // --> Aqui há uma conversão implícita para 'u8'

	println!("decimal_visual: {}", decimal_visual);
	println!("hexadecial: {}", hexadecial);
	println!("octal: {}", octal);
	println!("binario: {}", binario);
	println!("byte: {}", byte);


	// Tipos ponto flutuantes
	// Seguem a lǵoica dos inteiros, mas basicamente exitem dois: 'f32' e 'f64'
	// Por padrão é sempre atribuido os f64, para ter mais precisão,
	// mesmo podendo custar mais processamento.

	let flutuante64 = 6.4; // --> f64
	let flutuante32: f32 = 3.2;

	println!("flutuante64: {}", flutuante64);
	println!("flutuante32: {}", flutuante32); 

	// Operações matemáticas --> 
	// sem os crates, rust só opera matematica simples, adição, sub, multiplicação, divisão e resto
	let adicao = flutuante32 + flutuante64;
	let subtracao = flutuante64 - flutuante32;
	let divisao = flutuante64 / flutuante32;
	let multiplicacao = flutuante64 * flutuante32;
	let resto = 7 % 2;

	println!("adicão: {}", adicao);
	println!("subtração: {}", subtracao);
	println!("divisão: {}", divisao);
	println!("multiplicação: {:.2}", multiplicacao); //Multipicação com 'f's precisa ser trada!!!!
													 // Por isso o :.2 -->  :.<n>
	println!("resto: {}", resto);


	// Booleanos, são como em todas as linguagens
	// Não aceita numeros como seus valores, não faz esse tipo de convesão implicita
	let t = true;
	let f: bool = false;

	println!("t: {}", t);
	println!("f: {}", f);


	// chars --> Em rust chars não são somente caracteres, são valores unicodes,
	//  		 então funcionam com, emojis, ediogramas e acentos;
	//			 São indicados por aspas simples;
	let z = 'z';
	let zz = 'ℤ';
	let emoji = '👻';
	
	println!("z: {}", z);
	println!("zz: {}", zz);
	println!("emoji: {}", emoji);

	// Tipos compostos são as tuplas e os vetores, 
	// então funcionam como em outras linguagens bem semelhante ao TypeScript
	let tupla: (i32, f64, &str) = (500, 6.4, "oi");
	println!("tupla1: {} | tupla2: {} | tupla3: {}", tupla.0, tupla.1, tupla.2);
	 													// |--> O elemento da tupla é acessado via '.'

	//é possivel relizar o destruting em tuplas,
	//o rust é bem restrito para variaveis não usadas
	// para as variaveis da tupla que não for usar, é bom colocar o prefixo '_'
	let (_tupla1, tupla2, _tupla3) = tupla;
	println!("tupla2: {}", tupla2);
	

	// matrizes em rust são de um unico de tipo de valores, e são de tamanhos fixo :(
	// Exite o vetor na lib padrão do Rust, e else podem variar de tamanho
	let matrix = [5, 2, 5];

	// Percorre sempre a referencia do vetor, não ele por sí só
	for i in &matrix {
		println!("Matrix: {}", i);
	}


	// Pontos interressantes de Rust, é que ele não acessa numero fora da matriz,
	// como em C que pode apontar para um enderço de memoria invalido,
	// então trava o programa e runtime, um dos explos de "panic!" do rust!
	
}
