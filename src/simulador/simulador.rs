use std::collections::VecDeque;

use super::{enums::{Cor, TipoEvento}, cliente::Cliente, evento::Evento, estatisticas::{exponencial::AmostraExp, numero_clientes::NClientes, espera::EstatisticasEspera}};

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
    //estrutura que cuida da geração de números aleatórios com distribuição exponencial
    gera_exp : AmostraExp,
    //contador que armazena a quantidade de chegadas que ocorreram na rodada atual
    n_chegadas : usize,
    //armazena o máximo de chegadas por rodada
    max_chegadas : usize,
    //flag que indica se o sistema atualmente encontra-se ocioso
    esta_ocioso : bool,
    //Estrutura que contabiliza as alterações de número de clientes no sistema a cada execução de eventos
    //e as médias dessas estatísticas ao fim das rodadas
    n_clientes : NClientes,
    //Estrutura de dados que contabiliza as estatísticas dos clientes a cada fim de rodada
    estatisticas_clientes : EstatisticasEspera,
    //Armazena o número da rodada atual
    rodada_atual : usize,
    //Armazena a quantidade total de rodadas solicitadas pelo usuário
    max_rodadas : usize
}

impl Simulador {
    //Função que retorna um simulador não determinístico com seed fornecida pelo SO
    pub fn novo(lambda : f64, mu : f64, max_chegadas : usize, max_rodadas : usize) -> Self {
        Self { 
            ocupa_servidor: None,
            tempo_ocioso: 0.0,
            fila_1: VecDeque::new(),
            fila_2: VecDeque::new(), 
            lista_eventos: Vec::new(), 
            tempo: 0.0, 
            lambda, 
            mu,
            gera_exp : AmostraExp::novo(false,0),
            n_chegadas: 0,
            max_chegadas,
            esta_ocioso : true,
            n_clientes : NClientes::novo(),
            estatisticas_clientes : EstatisticasEspera::novo(),
            rodada_atual : 0,
            max_rodadas
        }
    }

    //Função que retorna um simulador determinístico com seed informada na sua criação
    pub fn novo_det(lambda : f64, mu : f64, max_chegadas : usize, max_rodadas : usize, seed: u64) -> Self {
        Self { 
            ocupa_servidor: None,
            tempo_ocioso: 0.0,
            fila_1: VecDeque::new(),
            fila_2: VecDeque::new(), 
            lista_eventos: Vec::new(), 
            tempo: 0.0, 
            lambda, 
            mu,
            gera_exp : AmostraExp::novo(true, seed),
            n_chegadas: 0,
            max_chegadas,
            esta_ocioso : true,
            n_clientes : NClientes::novo(),
            estatisticas_clientes : EstatisticasEspera::novo(),
            rodada_atual : 0,
            max_rodadas
        }
    }

    pub fn roda_simulacao(&mut self){
        self.inicia_rodada()
    }

    //Função que inicia as rodadas e grava as estatísticas obtidas nas variáveis de estado do servidor
    pub fn inicia_rodada(&mut self){
        let amostra_chegada = self.gera_exp.amostra_exp(self.lambda);
        //Adiciona a chegada inicial à lista de eventos
        self.adiciona_evento(Evento::novo(TipoEvento::CHEGADA, 
            self.tempo + amostra_chegada, 
            self.tempo));
        while &mut self.lista_eventos.len() > &mut 0 {
            self.trata_evento();
        }
    }
    //Retorna o próximo evento a ser tratado da lista de eventos
    pub fn evento_atual(&mut self) -> Option<Evento> {
        //Retorna None caso não existam eventos na lista de eventos
        if self.lista_eventos.is_empty(){
            return None;
        }
        let (mut tempo_min, mut index) : (f64, usize) = (f64::MAX, 0);
        //Busca pelo evento com menor instante de execução
        for (i, event) in self.lista_eventos.iter().enumerate() {
            if tempo_min > event.tempo {
                tempo_min = event.tempo;
                index = i;    
            }
        }
        Some(self.lista_eventos.remove(index))
    }

    //Trata inicalmente a execução de um evento e o endereça de acordo com seu tipo
    pub fn trata_evento(&mut self){
        //Caso exista algum evento na lista de eventos
        if let Some(evento_atual) = self.evento_atual(){
            //contabiliza o tempo ocioso do simulador
            if self.esta_ocioso {
                self.tempo_ocioso += evento_atual.tempo - self.tempo;
                self.esta_ocioso = false;
            }
            //atualiza o tempo atual da simulação
            self.tempo = evento_atual.tempo;
            match evento_atual.tipo {
                TipoEvento::CHEGADA => self.trata_chegada(evento_atual),
                TipoEvento::FimServico1 => self.trata_fim_1(evento_atual),
                TipoEvento::FimServico2 => self.trata_fim_2(evento_atual)
            };
            self.contabiliza_clientes();
        }else{//Caso não existam eventos na lista de eventos
            panic!("Erro: tentando recuperar evento atual com a lista vazia");
        }
    }

    //TODO: coletar as estatísticas de N e Nq
    //Trata a execução do evento do tipo Chegada
    pub fn trata_chegada(&mut self, evento_atual : Evento){
        let novo_cliente = self.inicia_cliente();
        //Gera uma amostra exponencial com taxa lambda para uma nova chegada
        let amostra_chegada = self.gera_exp.amostra_exp(self.lambda);
        //adiciona uma nova chegada à lista de eventos caso o limite de chegadas não tenha sido atingido
        if self.max_chegadas > self.n_chegadas {
            self.n_chegadas += 1;
            self.adiciona_evento(Evento::novo(TipoEvento::CHEGADA, 
                self.tempo + amostra_chegada, 
                self.tempo));
        }
        //Verifica a existência de clientes sendo servidos
        match &mut (self.ocupa_servidor){
            //Caso não exista cliente no servidor
            None => {
                //Adiciona evento fim de serviço 1 à lista de eventos
                self.adiciona_evento(Evento::novo(TipoEvento::FimServico1, 
                    self.tempo + novo_cliente.servico_1,
                    self.tempo));
                //Adiciona o cliente ao ocupa_servidor
                self.ocupa_servidor = Some(novo_cliente);
            },
            //Caso exista cliente no servidor
            Some(cliente) =>{
                //Caso o cliente seja da cor branca
                if matches!(cliente.cor, Cor::BRANCO) {
                    self.fila_1.push_back(novo_cliente);
                }else{
                    self.adiciona_evento(Evento::novo(TipoEvento::FimServico1, 
                        self.tempo + novo_cliente.servico_1,
                        self.tempo));
                    self.trata_interrupcao();
                    //Remove o cliente antigo do servidor dando lugar ao novo da cor branca
                    self.ocupa_servidor = Some(novo_cliente);
                }
            }
        };
    }

    //Trata a execução do evento de fim do serviço 1
    pub fn trata_fim_1(&mut self, evento_atual : Evento){
        //Recupera quem está dentro do servidor atualmente
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
                    //OBS: não foi necessário incluir o cliente no servidor pois ele já estava no mesmo
                    self.adiciona_evento(Evento::novo(TipoEvento::FimServico2, 
                        self.tempo + self.ocupa_servidor.unwrap().resta_servico_2,
                        self.tempo));
                //Caso a fila 2 possua clientes
                }else{
                    //Adiciona o cliente que terminou seu ser atendido pelo serviço 1 à fila 2
                    self.fila_2.push_back(*cliente_atual);
                    //Recupera o primeiro da fila 2 e o fornece serviço do tipo 2
                    self.ocupa_servidor = self.fila_2.pop_front();
                    //Adiciona o evento de fim do serviço 2 à lista de eventos
                    self.adiciona_evento(Evento::novo(TipoEvento::FimServico2, 
                        self.tempo + self.ocupa_servidor.unwrap().resta_servico_2,
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

    //Trata a execução do evento de fim do serviço 2
    pub fn trata_fim_2(&mut self, evento_atual : Evento){
        //Recupera quem está dentro do servidor atualmente
        if let Some(cliente_atual) = &mut self.ocupa_servidor {
            //Armazena o instante em que o cliente finaliza o serviço 2
            cliente_atual.termina_2 = self.tempo;
            //Zera o tempo de serviço 2 restante para o cliente
            cliente_atual.resta_servico_2 = 0.0;
            
            //Gera um erro caso a fila 1 esteja ocupada
            if !self.fila_1.is_empty(){
                panic!("Erro: A fila 1 possuí clientes enquanto um cliente da está sendo atendido pelo serviço 2");
            }

            //Contabiliza todas as estatísticas do cliente que acaba de deixar o sistema
            self.contabiliza_estatisticas_cliente();

            //Caso a fila 2 esteja vazia
            if self.fila_2.is_empty(){
                //Remove o cliente atual do servidor
                self.ocupa_servidor = None;
                //Declara o servidor como estando em estado ocioso
                self.esta_ocioso = true;
            //Caso a fila 2 não esteja vazia
            }else{
                //Recupera o primeiro cliente da fila 2 e o concede serviço
                self.ocupa_servidor = self.fila_2.pop_front();
                //Adiciona o evento fim de serviço 2 à lista de eventos
                self.adiciona_evento(Evento::novo(TipoEvento::FimServico2, 
                    self.tempo + self.ocupa_servidor.unwrap().resta_servico_2,
                    self.tempo));
            }
        }else { //interrompe a execução do programa pois temos um erro
            panic!("Erro: Evento fim de serviço 2 tratado sem clientes no servidor");
        }
    }

    //Interrompe o cliente que está atualmente no servidor e excluí seu evento de término de serviço
    //da lista de eventos
    pub fn trata_interrupcao(&mut self){
        //Remove os eventos do tipo Fim Serviço 2 da lista de eventos
        self.lista_eventos.retain(|event| {
            if matches!(event.tipo, TipoEvento::FimServico2){
                let tempo_atual = self.tempo;
                //Recupera o cliente ocupante do servidor
                if let Some(cliente) = &mut self.ocupa_servidor{
                    //Ajusta o tempo restante para o término do serviço 2 para o cliente atualmente no servidor
                    cliente.resta_servico_2 = event.tempo - tempo_atual;
                    //Reenvia o cliente para a fila 2
                    self.fila_2.push_front(*cliente);
                }
                //retorna falso, dessa forma, removendo o evento da lista
                return false;
            }
            //caso o evento não seja do tipo FimServico2 ele é mantido na lista
            true
        });
    }

    //TODO: push in an ordered way
    pub fn adiciona_evento(&mut self, evento : Evento){
        self.lista_eventos.push(evento)
    }

    //Gera um cliente que acaba de entrar na fila
    pub fn inicia_cliente(&mut self) -> Cliente{
        //gera a amostra de tempo total do serviço 1
        let tempo_servico_1 = self.gera_exp.amostra_exp(self.mu);
        //gera a amostra de tempo total do serviço 2
        let tempo_servico_2 = self.gera_exp.amostra_exp(self.mu);
        //Cria a instância de cliente, com seu tempo de chagada sendo o tempo atual do sistema
        //seus tempos de serviço gerados a partir de amostras exponenciais e sua cor sendo Branca
        Cliente::novo(self.tempo, tempo_servico_1, tempo_servico_2, Cor::BRANCO)
    }

    //Contabiliza as estatísticas correspondentes às médias do número de clientes na fila
    fn contabiliza_clientes(&mut self){
        let tempo_decorrido = self.tempo - self.n_clientes.t;
        self.n_clientes.e_n1 += tempo_decorrido * self.n_clientes.n1 as f64;
        self.n_clientes.e_nq1 += tempo_decorrido * self.n_clientes.nq1 as f64;
        self.n_clientes.e_n2 += tempo_decorrido * self.n_clientes.n2 as f64;
        self.n_clientes.e_nq2 += tempo_decorrido * self.n_clientes.nq2 as f64;
        self.atualiza_contagem_clientes();
    }

    //Atualiza a estrutura de dados que contabiliza o número de clientes atual presente na fila
    fn atualiza_contagem_clientes(&mut self){
        self.n_clientes.t = self.tempo;
        self.n_clientes.nq1 = self.fila_1.len();
        self.n_clientes.nq2 = self.fila_2.len();
        //Verifica o cliente atual presente no servidor
        match &self.ocupa_servidor{
            //Caso não exista nenhum
            None => {
                self.n_clientes.n1 = self.n_clientes.nq1;
                self.n_clientes.n2 = self.n_clientes.nq2;
            },
            Some(cliente) => {
                //Caso exista 1 da cor branca
                if matches!(cliente.cor, Cor::BRANCO){
                    self.n_clientes.n1 = self.n_clientes.nq1 + 1;
                    self.n_clientes.n2 = self.n_clientes.nq2;
                }else{//Caso exista 1 da cor preta
                    self.n_clientes.n2 = self.n_clientes.nq2 + 1;
                    self.n_clientes.n1 = self.n_clientes.nq1;
                }
            }
        }
    }

    //Registra no estado do simulador as estatísticas de um cliente que acaba de deixar o sistema
    fn contabiliza_estatisticas_cliente(&mut self) {
        if let Some(cliente) = self.ocupa_servidor{
            self.estatisticas_clientes.e_t1 += cliente.tempo_t1();
            self.estatisticas_clientes.e_t2 += cliente.tempo_t2();
            self.estatisticas_clientes.e_w1 += cliente.tempo_w1();
            self.estatisticas_clientes.e_w2 += cliente.tempo_w2();
            self.estatisticas_clientes.e_x1 += cliente.servico_1;
            self.estatisticas_clientes.e_x2 += cliente.servico_2;
            self.estatisticas_clientes.v_w1 += cliente.servico_1.powi(2);
            self.estatisticas_clientes.v_w2 += cliente.servico_2.powi(2);
        }
    }
}