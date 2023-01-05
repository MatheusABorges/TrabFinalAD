use std::collections::VecDeque;

use super::{enums::Cor, cliente::Cliente, evento::Evento};

pub struct Simulador{
    //Será Some(BRANCO) se o servidor estiver ocupado com um cliente da cor branca,
    //Some(PRETO) se o servidor estiver ocupado com um cliente da cor preta,
    //e será None caso não exista cliente em seviço
    servidor : Option<Cor>,
    //Guarda o tempo total em que o servidor ficou ocioso
    tempo_ocioso : f64,
    //estrutura de dados fila que guarda os clientes que esperam pelo serviço 1
    fila_1 : VecDeque<Cliente>,
    //estrutura de dados fila que guarda os clientes que esperam pelo serviço 2
    fila_2 : VecDeque<Cliente>,
    //estrutura de dados array que armazena os eventos a serem processados
    lista_eventos : Vec<Evento>,
    //armazena o tempo atual da simulação
    tempo : f64
}