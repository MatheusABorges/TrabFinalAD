mod simulador;
use simulador::{cliente::Cliente, enums::Cor};

use crate::simulador::simulador::Simulador;

use crate::simulador::estatisticas::exponencial::AmostraExp;

fn main(){
    //let mut sim = Simulador::novo_det(0.4, 1000000, 10, 5);
    //let mut sim = Simulador::novo(0.4, 10_000_000, 10);
    let mut sim = Simulador::novo(0.2, 30, 29700 , 30178.57, 29223.22);
    sim.roda_simulacao();
    /*for i in 0..20{    
        print!("{} - ", i);
        sim = Simulador::novo(0.4, 50, 15_000, 0.0, 0.0);
        println!("");
        sim.roda_simulacao();
    }*/

    //sim.testa_periodo_transiente(20_000, 10_000);
}