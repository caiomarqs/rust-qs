use std::io;

fn main() {

	let mut input = String::new();
    let mut a: u64 = 0;
    let mut b: u64 = 1;
	let mut aux: u64 = a + b;

	println!("Insira qual posição na quecia voce que: ");
	io::stdin().read_line(&mut input).expect("Não foi possivel ler o input!!");
	let input: u64 = input.trim().parse().expect("Não foi possivel converter o input");

	//a soma do anterior mas o atual é o resultande da posição
	for item in 1..(input + 1){
		if item	== input{
			println!("|Pos\t|Val\t|");
			println!("|{}\t|{}\t|", item, aux);	
		}
		
		aux = a + b;
		a = b;
		b = aux;
	}
}
