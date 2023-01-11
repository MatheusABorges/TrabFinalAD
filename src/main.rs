mod simulador;
use simulador::{cliente::Cliente, enums::Cor};

use crate::simulador::simulador::Simulador;

use crate::simulador::estatisticas::exponencial::AmostraExp;

fn main(){
    //let mut sim = Simulador::novo_det(0.4, 1000000, 10, 5);
    //let mut sim = Simulador::novo(0.4, 10_000_000, 10);
    let mut sim = Simulador::novo(0.10, 600, 11600, 11899.41, 11302.38);
    sim.roda_simulacao();
    /*for i in 0..20{    
        print!("{} - ", i);
        sim = Simulador::novo(0.4, 50, 15_000, 0.0, 0.0);
        println!("");
        sim.roda_simulacao();
    }*/

    println!("RODOUUU");
    //sim.testa_periodo_transiente(20_000, 10_000);
}