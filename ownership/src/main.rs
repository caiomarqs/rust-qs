// TUDO TA EXPLICADO NA DOCUMENTÇÂO
// https://rust-br.github.io/rust-book-pt-br/ch04-01-what-is-ownership.html#formas-de-intera%C3%A7%C3%A3o-entre-vari%C3%A1veis-e-dados-move


// Rust não possui garbage colector 
// sua parte q aloca e desaloca varaveis/expressões/declarações em memoria é diferente.
// O Rust tem um sistema de Ownership, que é um conjuto de regras determinadas em tempo de compilação
// e não implica em nenhum custo em runtime
// O ownership irá tratar problemas como dados duplicados no heap e espaços não utilizados no heap
fn main() {

	// 'string' e 'String' são coisas diferentes em Rust, uma é referente a declaração literal de um texto
	// e a outra é referente a uma declaração de uma coleção apatir de um texto literal
	// Basicamente, em outras lingugens serica como uma fosse um tipo primitivo e a outra uma instancia
	// e.g: 
	//		let s_p = "texto"; // --> Tipo literal/simples
	// 		let s_c = String::from("texto") // --> Tipo complexo/"objeto"
	//
	// Pela a diferença, um é armazenado diretamente na pilha e o outro é alocado no heap 
	// o alocado no heap tem a sua referencia na pilha, para que possa ser "achado"
	// As 'String's podem ser alteradas como se fosse um vetor ou uma coleção

	let mut s = String::from("olá");
	// aqui seria a alterção da String, com a literal não seria possivel!!!
	// isso se da pela a forma que os dois tipos lidam com a memoria!
	s.push_str(", mundo!");
	println!("{}", s);
	// Para inputs de usuraio só é valido usar o tipo String,
	// pois não é possivel saber o que o usário irá inserir
	// então precisara alocacar dinamicamente esse texto!

	// Aboradegens de tratamento da alocação de memória:
	// - Sem Garbage colcetor:
	//		Responsbilidade do progrmador, criar ponteitros, e desalocar a memoria, mas é passivel a erros
	//		O programador pode liberar a memoria antes, do ultimo uso ou dispedicar memoria
	// - Com Garbage colector:
	//		Retira as reposabilidades citadas do programdor, mas as vezes pode disperdicar memoria,
	// 		pois nem sempre o garbage colector limpa o ponteiro.
	// - Alocação da memoria, somente no escopo (Rust):
	//		O Rust, parace ter algo parecido com o GC, mas um pouco mais dinamico, dando liberdade para o
	//		desenvolvedor, a memoria é automaticamente liberada quando a variavel sai de escopo.
	//		!Assim é importante perceber a forma que uma variavel statica funciona!


	// A abordagem de Rust, se assemelha ao RAII do C++, que chama uma função drop no fim do escopo
	// Rust chama automaticamente essa função para as variaveis antes da '}'



	// Aboradegem Move -- Copia rasa(shallow copy)
	// Nesse exemplo x e y receberão um valor, mesmo y recebdo x, assim eles serão "clones"
	// Mas isso acontece só porque '5' é um tipo simples/primitivo
	let x = 5;
	let y = x;
	println!("{}", x);
	println!("{}", y);

	// Para tipos complexos, isso não acontece pois tipos complexo são alocados no heap
	// Aqui vai contecer, que s2, irá apontar pro memso endereço de memoria de s1
	// e não alocando novamente um espaço para s2
	let s1 = String::from("texto");
	let s2 = s1;


	// Bom mas isso poderia gerar um problema, que quando o escopo acabar, haveria uma tentativa dupla
	// de liberação do ponteiro no heap, mas em rust isso não acontece, pois quando acontece um move
	// de um ponteiro, a primeira variavel q aponta para o ponteiro é fica inutilizavel
	// e.g: 
	// 	println!("{}", s1); //--> não seria possivel, pois o ponteiro já foi movido para 's2'
	// então se só o 's2' for valido, não havera uma tentativa dupla de liberação do ponteiro 
	println!("{}", s2);
	// Isso tudo, é para simplesmente não existir algo custoso em copias de varaiveis, 
	// ao menos que for intencional

	// Aboradegem Clone -- Copia profunda(deep copy)
	// Basicamente essa aboradagem seria a copia que acontece com os tipos primitiovos
	// para os tipos complexos, como o Rust por padrão não executa essa opeação custosa
	// tem que ser algo indicado!!
	let s3 = String::from("texto");
	let s4 = s3.clone(); // --> Metodo padrão para esse tipo de operação que copia ponteiros no heap

	println!("s3: {}, s4: {}", s3, s4);


	// Aboradegem Copy 
	// é aboradagem para os tipos primitivos posi são aloados diretamente na pilha, assim sendo facil de
	// gerenciar, e sempre são de tamanhos conhecidos
	// importante firzar que tuplas com somente tipos primitvos o copy funciona
	// e.g:
	let tup: (i32, i32) = (4, 3);
	let tup_copy = tup;
	println!("{}", tup.1);
	println!("{}", tup_copy.1);

	// Mas para tuplas com tipos primitvos e tipos coplexos, não funcionam pois um tos argumentos,
	// pode variar e estar apontando para o heap
	// e.g:
	let tup_complex: (i32, String) = (5, String::from("texto"));
	let tup_complex_2 = tup_complex;

	// println!("{}", tup_complex.1); // -->  Não é possivel pois a variavel fica invalida
	println!("{}", tup_complex_2.1); // --> já é possivel


	// Para os parametros das funções, tudo o que foi dito funciona de maneira similar
	// E com isso tem alguns pontos interessantes!
	let texto = String::from("texto");
	posse(texto);

	// println!("texto: {}", texto); // --> aqui texto, não é mais encontrado no escopo, ele foi movido!!
								     // VER coments de posse()

	let x = 5;
	copy(x);
	println!("dps de copy: {}", x); // isso é possivel pois x(de tipo primitvo) ainda permanece na pilha
								
	// Para os retornos de função as mesmas regras são aplicadas
	let s5 = retorna_valor(); // s5 é o move do ponteiro do retorno dessa função 
							  // pois retorna um tipo coplexo
	println!("s5: {}", s5);


	let s6 = String::from("texo");
	let s7 = pega_e_retorna_valor(s6); // s7 apontará pro mesmo lugar de s6

	//println!("tentativa de s6: {}", s6); // --> Não é possivel pois foi movida
	println!("s7: {}", s7);


	// Para manter essas variaveis movidas dentro do escpo maior, 
	// é possivel dizer para a função retornar a varivel já movida, mais o resultado da função em uma tupla
	// Like a funcional way

	let s8 = String::from("texto");
	let (s9, tamanho) = tamanho_string(s8);

	println!("texto: {}, tamanho: {}", s9, tamanho);

	// As vezes isso pode ser complicado, por isso exites o simbolo '&', onde indica essa operação
}


// O arqgumento recebido por essa função deixarar de ser valido no fim do escopo dessa função
// então não será válido apos a excução da função!!!!
fn posse(texto: String) {
	println!("posse: {}", texto);
}


// O argumento recebido por essa função como é um argumento de tipo primitivo, 
// ele ainda continuirá na pilha após a execução da função
fn copy(inteiro: i32) {
	println!("copy: {}", inteiro);
}


// Essa função retorna um texto do tipo 'String', então tudo que for utilizar essa função,
// tera um 'move' do ponteiro do retorno da função
fn retorna_valor() -> String {
	let texto = String::from("retorno");
	texto
}


fn pega_e_retorna_valor(texto: String) -> String {
	texto	
}

// Aqui como em funções puras nós sbemos exatamente o que ela retorna, 
// é possivel então ter uma opção para não tirar de escopo as variaveis já movidas!
fn tamanho_string(texto: String) -> (String, usize) {
	let size = texto.len();
	(texto, size)	
}
