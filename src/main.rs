mod simulador;

use simulador::cliente::Cliente;
fn main(){
    let a = Cliente::novo(1.0,15);
    println!("{:?}", a);
}