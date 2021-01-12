// Imagine uma função que extrai a o indicie da ultima letra da primeira palavra de uma string, 
// e que essa string por alguma razao apos ser executada sai de contexto, ou sejá limpa etc...
// Esse valor do indice ficaria invalido caso quisesse trabalhar com ele para manupipular a string
// Para isso que se tem o slice de string, que basicamente faz uma especie de destructing em idicies
// desejados de uma string e eles ficam atribuidos a referencia da string assim quando a string para de 
// existir eles tambem sumirão!
// como no exempo abiaxo

fn main() {
    let s = String::from("texto longo");

    let texto = &s[0..5];
    let longo = &s[6..11];
}
