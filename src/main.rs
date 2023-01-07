mod simulador;
use simulador::{cliente::Cliente, enums::Cor};

use crate::simulador::simulador::Simulador;

fn main(){
    let sim = Simulador::novo(0.5,1.0, 5);

    let cliente = Cliente::novo(0.0, 1.0, 1.0, Cor::PRETO);

    println!("{:?}", cliente);
}