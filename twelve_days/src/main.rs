fn main() {
	let frases = [
		"A partridge in a pear tree.", 
		"Two turtle doves,",
		"Three French hens,",
		"Four calling birds,",
		"Five golden rings,",
		"Six geese a-laying,",
		"Seven swans a-swimming,",
		"Eight maids a-milking,",
		"Nine ladies dancing,",
		"Ten lords a-leaping,",
		"Eleven pipers piping,",
		"Twelve drummers drumming,",
	];

	
	for (i, _item) in frases.iter().enumerate() {
	
		if i == 0 {
			println!("On the first day of Christmas,my true love sent to me");
			println!("{}", frases[0]);
		}
		else {
			println!("On the first day of Christmas,my true love sent to me");

			for j in (1..(i + 1)).rev() {
				println!("{}", frases[j]);
			}

			println!("And {}", frases[0].to_lowercase());			
		}

		println!("\n");
	}
}
