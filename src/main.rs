mod simulador;
use simulador::{cliente::Cliente, enums::Cor};

#[macro_use] extern crate scan_rules;

use crate::simulador::simulador::Simulador;

use crate::simulador::estatisticas::exponencial::AmostraExp;

fn main(){
    println!("Bem vindo ao Simulador de M\\M\\1!");
    println!("Perguntas para o input dos parâmetros:");
    println!("Deseja rodar determinístico? (para SIM, digite 1, para NAO, digite 0)");
    let mut determi:usize= 0;
    let mut chegadass:usize= 0;
    let mut rodadass:usize= 0;
    let mut rho : f64 = 0.0;
    let mut chi_up : f64 = 0.0;
    let mut chi_low : f64 = 0.0;
    let mut seed : u64 = 0;

    readln! {
        (let determ: usize) => {
            println!("Deterministico: {}", determ);
            determi = determ;
        }
    }
    print!("Quantas chegadas por rodada? (inserir valor inteiro)");
    readln! {
        (let chegadas: usize) => {
            println!("Chegadas: {}", chegadas);
            chegadass = chegadas;
        }
    }
    print!("Quantas rodadas? (inserir valor inteiro)");
    readln! {
        (let rodadas: usize) => {
            println!("Rodadas: {}", rodadas);
            rodadass = rodadas;
        }
    }

    print!("Qual o rho?");
    readln! {
        (let rho_: f64) => {
            println!("Rodadas: {}", rho_);
            rho = rho_;
        }
    }

    print!("Qual o Percentil superior da Chi² a ser usada?");
    readln! {
        (let chi_up_: f64) => {
            println!("Rodadas: {}", chi_up_);
            chi_up = chi_up_;
        }
    }

    print!("Qual o Percentil inferior da Chi² a ser usada?");
    readln! {
        (let chi_low_: f64) => {
            println!("Rodadas: {}", chi_low_);
            chi_low = chi_low_;
        }
    }

    if determi == 1 {
        print!("Qual será a semente? (inserir valor inteiro)");
        readln! {
            (let seed_: u64) => {
                println!("Chegadas: {}", seed_);
                seed = seed_;
            }
        }
    }

    println!("Dados foram coletados. Agurade os resultados da simulação\n\n\n");

    let mut sim : Simulador;
    if determi == 1 {
        sim = Simulador::novo_det(rho, chegadass, rodadass, chi_up, chi_low, seed);
        sim.roda_simulacao();
    } else{
        sim = Simulador::novo(rho, chegadass, rodadass, chi_up, chi_low);
        sim.roda_simulacao();
    }
}