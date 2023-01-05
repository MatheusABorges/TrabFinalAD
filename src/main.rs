mod simulador;
use simulador::cliente::Cliente;
use simulador::enums::Cor;
fn main(){
    let mut a = Cliente::novo(1.0,Cor::PRETO);
    a.inicia_1 = 123.12;
    println!("{:?}", a);
}