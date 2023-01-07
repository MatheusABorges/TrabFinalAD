use std::collections::VecDeque;

use super::{enums::{Cor, TipoEvento}, cliente::Cliente, evento::{Evento, self}, estatisticas::exponencial::amostra_exp};

pub struct Simulador{
    //Será Some(Cliente) caso exista um cliente em serviço
    //e será None caso não exista cliente em seviço
    ocupa_servidor : Option<Cliente>,
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
    mu : f64,
    //armazena os clientes que finalizaram seus serviços para futura coleta e tratamento das estatísticas
    clientes_finalizados: Vec<Cliente>,
    n_chegadas : u64,
    max_chegadas : u64
}

impl Simulador {

    pub fn novo(lambda : f64, mu : f64, max_chegadas : u64) -> Self {
        Self { 
            ocupa_servidor: None,
            tempo_ocioso: 0.0,
            fila_1: VecDeque::new(),
            fila_2: VecDeque::new(), 
            lista_eventos: Vec::new(), 
            tempo: 0.0, 
            lambda, 
            mu,
            clientes_finalizados: Vec::new(),
            n_chegadas: 0,
            max_chegadas
        }
    }

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
            TipoEvento::FimServico1 => self.trata_fim_1(evento_atual),
            TipoEvento::FimServico2 => self.trata_fim_2(evento_atual)
        };
    }

    //TODO: coletar as estatísticas de N e Nq
    //Trata a execução do evento do tipo Chegada
    pub fn trata_chegada(&mut self, evento_atual : Evento){
        let novo_cliente = self.inicia_cliente();
        //adiciona uma nova chegada à lista de eventos
        self.adiciona_evento(Evento::novo(TipoEvento::CHEGADA, 
            self.tempo + amostra_exp(self.lambda), 
            self.tempo));
        match &mut (self.ocupa_servidor){
            None => {
                //Adiciona evento fim de serviço 1 à lista de eventos
                self.adiciona_evento(Evento::novo(TipoEvento::FimServico1, 
                    self.tempo + novo_cliente.servico_1,
                    self.tempo));
                //Adiciona o cliente ao ocupa_servidor
                self.ocupa_servidor = Some(novo_cliente);
            },
            Some(cliente) =>{
                if matches!(cliente.cor, Cor::BRANCO) {
                    self.fila_1.push_back(novo_cliente);
                }else{
                    self.adiciona_evento(Evento::novo(TipoEvento::FimServico1, 
                        self.tempo + novo_cliente.servico_1,
                        self.tempo));
                    self.trata_interrupcao()
                }
            }
        };
    }

    //Trata a execução do evento de fim do serviço 1
    pub fn trata_fim_1(&mut self, evento_atual : Evento){
        if let Some(cliente_atual) = &mut self.ocupa_servidor {
            //armazena o tempo em que o cliente termina o serviço 1
            cliente_atual.termina_1 = self.tempo;
            //muda a cor do cliente pois o mesmo só terá serviço 2 pela frente
            cliente_atual.cor = Cor::PRETO;
            //Caso a fina 1 esteja vazia
            if self.fila_1.is_empty() {
                //Caso a fila 2 esteja vazia
                if self.fila_2.is_empty(){
                    //Adiciona o evento de fim do serviço 2 à lista de eventos
                    self.adiciona_evento(Evento::novo(TipoEvento::FimServico2, 
                        self.tempo + self.ocupa_servidor.unwrap().servico_2,
                        self.tempo));
                //Caso a fila 2 possua clientes
                }else{
                    //Adiciona o cliente que terminou seu ser atendido pelo serviço 1 à fila 2
                    self.fila_2.push_back(*cliente_atual);
                    //Recupera o primeiro da fila 2 e o fornece serviço do tipo 2
                    self.ocupa_servidor = self.fila_2.pop_front();
                    //Adiciona o evento de fim do serviço 2 à lista de eventos
                    self.adiciona_evento(Evento::novo(TipoEvento::FimServico2, 
                        self.tempo + self.ocupa_servidor.unwrap().servico_2,
                        self.tempo));
                }
            }else {
                //Adiciona o cliente que terminou seu serviço 1 à fila 2
                self.fila_2.push_back(*cliente_atual);
                //Recupera o primeiro cliente da fila 1 e o adiciona ao servidor
                self.ocupa_servidor = self.fila_1.pop_front();
                //Cria um novo envento do tipo FimServico1 e o adciona à lista de eventos
                self.adiciona_evento(Evento::novo(TipoEvento::FimServico1, 
                    self.tempo + self.ocupa_servidor.unwrap().servico_1,
                    self.tempo));
            } 
        } else { //interrompe a execução do programa pois temos um erro
            panic!("Erro: Evento fim de serviço 1 tratado sem clientes no servidor");
        }
    }

    pub fn trata_fim_2(&mut self, evento_atual : Evento){

    }

    pub fn trata_interrupcao(&mut self){

    }

    //TODO: push in an ordered way
    pub fn adiciona_evento(&mut self, evento : Evento){
        self.lista_eventos.push(evento)
    }

    //Gera um cliente que acaba de entrar na fila
    pub fn inicia_cliente(&self) -> Cliente{
        //gera a amostra de tempo total do serviço 1
        let tempo_servico_1 = amostra_exp(self.mu);
        //gera a amostra de tempo total do serviço 2
        let tempo_servico_2 = amostra_exp(self.mu);
        //Cria a instância de cliente, com seu tempo de chagada sendo o tempo atual do sistema
        //seus tempos de serviço gerados a partir de amostras exponenciais e sua cor sendo Branca
        Cliente::novo(self.tempo, tempo_servico_1, tempo_servico_2, Cor::BRANCO)
    }

    //TODO: remove this
    pub fn temp_amostra(&self) -> f64 {
        amostra_exp(self.lambda)
    }
}