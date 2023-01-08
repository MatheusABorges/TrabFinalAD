mod simulador;
use simulador::{cliente::Cliente, enums::Cor};

use crate::simulador::simulador::Simulador;

use crate::simulador::estatisticas::exponencial::AmostraExp;

fn main(){
    let mut sim = Simulador::novo_det(0.5, 1.0, 10, 5);
    sim.roda_simulacao();
}