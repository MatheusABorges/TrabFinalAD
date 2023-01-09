mod simulador;
use simulador::{cliente::Cliente, enums::Cor};

use crate::simulador::simulador::Simulador;

use crate::simulador::estatisticas::exponencial::AmostraExp;

fn main(){
    //let mut sim = Simulador::novo_det(0.4, 1000000, 10, 5);
    //let mut sim = Simulador::novo(0.4, 10_000_000, 10);
    let mut sim = Simulador::novo(0.4, 10_000_000, 1);
    sim.testa_periodo_transiente(20_000, 6_000);
}