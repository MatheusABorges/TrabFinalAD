use std::collections::VecDeque;

use super::{enums::{Cor, TipoEvento}, cliente::Cliente, evento::{Evento, self}, estatisticas::exponencial::amostra_exp};

pub struct Simulador{
    //Será Some(Cliente) caso exista um cliente em serviço
    //e será None caso não exista cliente em seviço
    servidor : Option<Cliente>,
    //Guarda o tempo total em que o servidor ficou ocioso
    tempo_ocioso : f64,
    //estrutura de dados fila que guarda os clientes que esperam pelo serviço 1
    fila_1 : VecDeque<Cliente>,
    //estrutura de dados fila que guarda os clientes que esperam pelo serviço 2
    fila_2 : VecDeque<Cliente>,
    //estrutura de dados array que armazena os eventos a serem processados
    lista_eventos : Vec<Evento>,
    //armazena o tempo atual da simulação
    tempo : f64,
    //taxa da exponencial que representa o instante de chegada de fregueses à fila
    lambda : f64,
    //taxa da exponencial que representa a duração dos serviços do cliente
    mu : f64
}

impl Simulador {

    pub fn evento_atual(&mut self) -> Evento {
        Evento::novo(super::enums::TipoEvento::CHEGADA, -1.0, -1.0)
    }

    //Trata inicalmente a execução de um evento e o endereça de acordo com seu tipo
    pub fn trata_evento(&mut self){
        let evento_atual = self.evento_atual();
        //atualiza o tempo atual da simulação
        self.tempo = evento_atual.tempo;
        match evento_atual.tipo {
            TipoEvento::CHEGADA => self.trata_chegada(evento_atual),
            TipoEvento::FIM_SERVICO_1 => self.trata_fim_1(evento_atual),
            TipoEvento::FIM_SERVICO_2 => self.trata_fim_2(evento_atual)
        };
    }

    //TODO: coletar as estatísticas de N e Nq
    //Trata a execução do evento do tipo Chegada
    pub fn trata_chegada(&mut self, evento_atual : Evento){
        let novo_cliente = self.inicia_cliente(&evento_atual);
        //adiciona uma nova chegada à lista de eventos
        self.adiciona_evento(Evento::novo(TipoEvento::CHEGADA, 
            self.tempo + amostra_exp(self.lambda), 
            self.tempo));
        match &mut (self.servidor){
            None => {
                //Adiciona evento fim de serviço 1 à lista de eventos
                self.adiciona_evento(Evento::novo(TipoEvento::FIM_SERVICO_1, 
                    self.tempo + novo_cliente.servico_1,
                    self.tempo));
                //Adiciona o cliente ao servidor
                self.servidor = Some(novo_cliente);
            },
            Some(cliente) =>{
                if matches!(cliente.cor, Cor::BRANCO) {
                    self.fila_1.push_back(novo_cliente);
                }else{
                    self.adiciona_evento(Evento::novo(TipoEvento::FIM_SERVICO_1, 
                        self.tempo + novo_cliente.servico_1,
                        self.tempo));
                    self.trata_interrupcao()
                }
            }
        };
    }

    pub fn trata_fim_1(&mut self, evento_atual : Evento){

    }

    pub fn trata_fim_2(&mut self, evento_atual : Evento){

    }

    pub fn trata_interrupcao(&mut self){

    }

    pub fn adiciona_evento(&mut self, evento : Evento){
        self.lista_eventos.push(evento)
    }

    //Gera um cliente que acaba de entrar na fila
    pub fn inicia_cliente(&self, evento : &Evento) -> Cliente{
        //gera a amostra de tempo total do serviço 1
        let tempo_servico_1 = amostra_exp(self.mu);
        //gera a amostra de tempo total do serviço 2
        let tempo_servico_2 = amostra_exp(self.mu);
        //Cria a instância de cliente, com seu tempo de chagada sendo o tempo atual do sistema
        //seus tempos de serviço gerados a partir de amostras exponenciais e sua cor sendo Branca
        Cliente::novo(self.tempo, tempo_servico_1, tempo_servico_2, Cor::BRANCO)
    }

}