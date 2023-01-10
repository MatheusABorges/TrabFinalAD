use std::collections::VecDeque;

use super::{enums::{Cor, TipoEvento}, cliente::Cliente, evento::Evento, estatisticas::{exponencial::AmostraExp, numero_clientes::NClientes, espera::EstatisticasEspera}};

const RHO_02 : usize =  1_000;
const RHO_04 : usize =  6_000;
const RHO_06 : usize =  8_000;
const RHO_08 : usize =  4_000;
const RHO_09 : usize =  10_000;
const PERCENTIL_TSTUDENT : f64 = 1.96;

struct  ConfiancaAtingida{
    w1 : bool,
    w2 : bool,
    t1 : bool,
    t2 : bool,
    n1 : bool,
    n2 : bool,
    nq1 : bool,
    nq2 : bool,
    finalizou : bool
}

impl ConfiancaAtingida {
    fn novo() -> Self {
        Self { w1: false, w2: false, t1: false, t2: false, n1: false, n2: false, nq1: false, nq2: false,finalizou:false}
    }

    fn todas_confiancas_atingidas(&self) -> bool {
        self.w1 && self.w2 && self.t1 && self.t2 && self.n1 && self.n2 && self.nq1 && self.nq2 && !self.finalizou
    }
}

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
    //taxa de utilização informada na inicialização do simulador
    rho : f64,
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
    //Estrutura que armazena as estatísticas N1, N2, Nq1 e Nq2 de todas as rodadas
    n_clientes_total : NClientes,
    //Estrutura de dados que contabiliza as estatísticas dos clientes a cada fim de rodada
    estatisticas_clientes_rodada : EstatisticasEspera,
    //Estrutura de dados que armazena as estatísticas de espera dos clientes de todas as rodadas
    estatisticas_clientes_total : EstatisticasEspera,
    //Armazena o número da rodada atual
    rodada_atual : usize,
    //Armazena a quantidade total de rodadas solicitadas pelo usuário
    max_rodadas : usize,
    //Serve para verificar caso o intervalo de confiança de 95% já foi atingido para uma certa métrica
    confianca_atingida : ConfiancaAtingida,
    //Guarda o limte superior do percentil da distribuição chi² passada na criação do simulador
    chi_sqr_up : f64,
    //Guarda o limte inferior do percentil da distribuição chi² passada na criação do simulador
    chi_sqr_low : f64
}

impl Simulador {
    //Função que retorna um simulador não determinístico com seed fornecida pelo SO
    pub fn novo(rho : f64, max_chegadas : usize, max_rodadas : usize, chi_sqr_up : f64, chi_sqr_low : f64) -> Self {
        Self { 
            ocupa_servidor: None,
            tempo_ocioso: 0.0,
            fila_1: VecDeque::new(),
            fila_2: VecDeque::new(), 
            lista_eventos: Vec::new(), 
            tempo: 0.0, 
            lambda:rho/2.0, 
            mu:1.0,
            rho,
            gera_exp : AmostraExp::novo(false,0),
            n_chegadas: 0,
            max_chegadas,
            esta_ocioso : true,
            n_clientes : NClientes::novo(),
            n_clientes_total : NClientes::novo(),
            estatisticas_clientes_rodada: EstatisticasEspera::novo(),
            estatisticas_clientes_total : EstatisticasEspera::novo(),
            rodada_atual : 0,
            max_rodadas,
            confianca_atingida : ConfiancaAtingida::novo(),
            chi_sqr_low,
            chi_sqr_up
        }
    }

    //Função que retorna um simulador determinístico com seed informada na sua criação
    pub fn novo_det(rho : f64, max_chegadas : usize, max_rodadas : usize, chi_sqr_up : f64, chi_sqr_low : f64, seed: u64) -> Self {
        Self { 
            ocupa_servidor: None,
            tempo_ocioso: 0.0,
            fila_1: VecDeque::new(),
            fila_2: VecDeque::new(), 
            lista_eventos: Vec::new(), 
            tempo: 0.0, 
            lambda : rho/2.0, 
            mu : 1.0,
            rho,
            gera_exp : AmostraExp::novo(true, seed),
            n_chegadas: 0,
            max_chegadas,
            esta_ocioso : true,
            n_clientes : NClientes::novo(),
            n_clientes_total : NClientes::novo(),
            estatisticas_clientes_rodada : EstatisticasEspera::novo(),
            estatisticas_clientes_total : EstatisticasEspera::novo(),
            rodada_atual : 0,
            max_rodadas,
            confianca_atingida : ConfiancaAtingida::novo(),
            chi_sqr_low,
            chi_sqr_up
        }
    }

    //Função que executa a simulação
    pub fn roda_simulacao(&mut self){
        //lida com a execução do periodo transiente
        self.trata_periodo_transiente();
        //enquanto o número da rodada atual for menor que a quantidade de rodadas do input, novas rodadas são geradas
        while self.rodada_atual < self.max_rodadas {
            //armazena o tempo em que a rodada inicia
            let tempo_inicio = self.tempo;
            self.inicia_rodada();
            //coleta as estatísticas de tempo de espera dos clientes da rodada
            self.coleta_estatisticas_cliente();
            //contabiliza as estatísticas N1, N2, Nq1 e Nq2 da rodada usando o tempo decorrido como sendo o tempo do
            //fim da rodada meno o inicio da rodada
            self.contabiliza_estatisticas_n(self.tempo - tempo_inicio);
            // if self.rodada_atual > 1 {
            //     self.verifica_confianca_media();
            // }
        }
        self.exibe_resultado();
    }


    //Função que trata do periodo transiente, ou seja, elimina um número de chegadas determinado
    //a depender do valor de rho inputado
    pub fn trata_periodo_transiente(&mut self){
        //Calcula o tamanho da fase transiente dependendo do rho fornecido
        let tamanho_fase = Self::tamanho_fase_transiente(self.rho);

        let amostra_chegada = self.gera_exp.amostra_exp(self.lambda);
        //Adiciona a chegada inicial à lista de eventos
        self.adiciona_evento(Evento::novo(TipoEvento::CHEGADA, 
            self.tempo + amostra_chegada, 
            self.tempo));

        //Continua a tratar eventos até que a lista de eventos esvazie(bug) ou que a fase transiente acabe
        while &mut self.lista_eventos.len() > &mut 0 && self.n_chegadas < tamanho_fase {
            //Verifica se o evento é uma chegada
            if matches!(self.trata_evento().tipo, TipoEvento::CHEGADA) {
                //Incrementa o contador de chegadas por rodada
                self.n_chegadas += 1;
            }
        }
        //Atualiza a contagem de N1, N2, Nq1 e Nq2 atuais da fila ao fim da fase transiente para que suas
        //médias possam ser calculadas mais adiante
        self.atualiza_contagem_clientes();
    }

    //Função que inicia as rodadas e grava as estatísticas obtidas nas variáveis de estado do servidor
    pub fn inicia_rodada(&mut self) {
        //incrementa o número da rodada atual
        self.rodada_atual += 1;
        //zera o contador de chegadas por rodada
        self.n_chegadas = 0;

        //Trata eventos enquanto o número de chegadas da rodada não ultrapassar o máximo estipulado como input
        //do simulador
        while &mut self.lista_eventos.len() > &mut 0 && self.n_chegadas < self.max_chegadas{
            if matches!(self.trata_evento().tipo, TipoEvento::CHEGADA){
                self.n_chegadas += 1;
            }
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

    //Trata inicalmente a execução de um evento e o endereça de acordo com seu tipo e retorna o evento tratado
    //para possíveis análises
    pub fn trata_evento(&mut self) -> Evento {
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
            //Contabiliza as estatísticas N1, N2, Nq1 e Nq2 somente se estiver fora da fase transiente
            if self.rodada_atual != 0 {self.contabiliza_clientes();}
            return  evento_atual;
        }else{//Caso não existam eventos na lista de eventos
            panic!("Erro: tentando recuperar evento atual com a lista vazia");
        }
    }

    //Trata a execução do evento do tipo Chegada
    pub fn trata_chegada(&mut self, evento_atual : Evento){
        //Gera o cliente que está chegando na chegada atual
        let novo_cliente = self.inicia_cliente();
        //Gera uma amostra exponencial com taxa lambda para uma nova chegada
        let amostra_chegada = self.gera_exp.amostra_exp(self.lambda);
        //adiciona uma nova chegada à lista de eventos 
        self.adiciona_evento(Evento::novo(TipoEvento::CHEGADA, 
            self.tempo + amostra_chegada, 
            self.tempo));

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
            //se o mesmo pertencer à rodada atual
            if self.rodada_atual != 0 && cliente_atual.rodada == self.rodada_atual{
                self.contabiliza_estatisticas_cliente();
            }

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
        Cliente::novo(self.tempo, tempo_servico_1, tempo_servico_2, Cor::BRANCO, self.rodada_atual)
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

    //Contabiliza as estatísticas E[N1], E[N2], E[Nq1] e E[Nq2] da rodada a partir
    //do tempo decorrido, que é o tempo em que a rodada terminou menos o tempo em que a rodada iniciou
    fn contabiliza_estatisticas_n(&mut self, tempo_decorrido : f64) {
        let mut estatisticas_n = NClientes::novo();

        estatisticas_n.e_n1 = self.n_clientes.e_n1/tempo_decorrido;
        estatisticas_n.e_n2 = self.n_clientes.e_n2/tempo_decorrido;
        estatisticas_n.e_nq1 = self.n_clientes.e_nq1/tempo_decorrido;
        estatisticas_n.e_nq2 = self.n_clientes.e_nq2/tempo_decorrido;

        //Zera as estatísticas acumuladas na rodada atual
        self.n_clientes.e_n1 = 0.0;
        self.n_clientes.e_n2 = 0.0;
        self.n_clientes.e_nq1 = 0.0;
        self.n_clientes.e_nq2 = 0.0;

        //Incrementa as estatísticas de média rodada atual na variável de estado do servidor
        self.n_clientes_total.e_n1 += estatisticas_n.e_n1;
        self.n_clientes_total.e_n2 += estatisticas_n.e_n2;
        self.n_clientes_total.e_nq1 += estatisticas_n.e_nq1;
        self.n_clientes_total.e_nq2 += estatisticas_n.e_nq2;
        

        //Incrementa as estatísticas do quadrado das amostras da rodada para cálculo posterior da variância
        self.n_clientes_total.v_n1 += estatisticas_n.e_n1.powi(2);
        self.n_clientes_total.v_n2 += estatisticas_n.e_n2.powi(2);
        self.n_clientes_total.v_nq1 += estatisticas_n.e_nq1.powi(2);
        self.n_clientes_total.v_nq2 += estatisticas_n.e_nq2.powi(2);
    }

    //Registra no estado do simulador as estatísticas de um cliente que acaba de deixar o sistema
    fn contabiliza_estatisticas_cliente(&mut self) {
        if let Some(cliente) = self.ocupa_servidor{
            self.estatisticas_clientes_rodada.e_t1 += cliente.tempo_t1();
            self.estatisticas_clientes_rodada.e_t2 += cliente.tempo_t2();
            self.estatisticas_clientes_rodada.e_w1 += cliente.tempo_w1();
            self.estatisticas_clientes_rodada.e_w2 += cliente.tempo_w2();
            self.estatisticas_clientes_rodada.e_x1 += cliente.servico_1;
            self.estatisticas_clientes_rodada.e_x2 += cliente.servico_2;
            self.estatisticas_clientes_rodada.v_w1 += cliente.tempo_w1().powi(2);
            self.estatisticas_clientes_rodada.v_w2 += cliente.tempo_w2().powi(2);
            self.estatisticas_clientes_rodada.v_t1 += cliente.tempo_t1().powi(2);
            self.estatisticas_clientes_rodada.v_t2 += cliente.tempo_t2().powi(2);
        }
    }

    //Função que não participa diretamente da execução do simulador mas sim
    //é usada para estimar a duração do período transiente para os valores de rho
    //e descarta da contabilização a quantidade de chegadas passada como parâmetro
    pub fn testa_periodo_transiente(&mut self, max_chegadas : usize, chegadas_descartadas : usize) {
        self.max_rodadas = 1;
        self.max_chegadas = max_chegadas;
        //Pega a primeira amostra para a chegada que iniciará a simulação
        let amostra_chegada = self.gera_exp.amostra_exp(self.lambda);
        //Adiciona a chegada inicial à lista de eventos
        self.adiciona_evento(Evento::novo(TipoEvento::CHEGADA, 
            self.tempo + amostra_chegada, 
            self.tempo));
        println!("rho, chegada");
        //Continua a tratar eventos até que a lista de eventos esvazie(bug) ou que o max de chegadas tenha
        //sido atingido
        while &mut self.lista_eventos.len() > &mut 0 && self.n_chegadas < self.max_chegadas{
            //Verifica se o evento é uma chegada
            if matches!(self.trata_evento().tipo, TipoEvento::CHEGADA) {
                //Incrementa o contador de chegadas por rodada
                self.n_chegadas += 1;
                //Só exibe estatísticas de chegadas que ocorrem depois da quantidade de chegadas descartadas recebida como parâmetro
                if self.n_chegadas > chegadas_descartadas{
                    println!("{}, {}", 1.0 - self.tempo_ocioso/self.tempo, self.n_chegadas-chegadas_descartadas);
                }
            }
        }
    }

    //Retorna o tamanho da fase transiente dependendo do valor de rho informado na inicialização do simulador
    //o tamanho da fase foi obtido através da análise contida no relatório. Caso o rho especificado não seja
    //nenhum dos que foram analisados, utilizaremos o maior intervalo dos 5, ou seja, o de rho = 0.9(10000 chegadas)
    fn tamanho_fase_transiente(rho : f64) -> usize {
        match rho.to_string().as_str() {
            "0.2" => {RHO_02},
            "0.4" => {RHO_04},
            "0.6" => {RHO_06},
            "0.8" => {RHO_08},
            "0.9" => {RHO_09},
            //Caso o rho não seja um dos estudados, o intervalo será o maior dos 5
            _ => {RHO_09}
        }
    }

    //realiza a coleta das estatísticas de espera dos clientes ao fim de cada rodada
    fn coleta_estatisticas_cliente(&mut self) {
        let mut estatisticas_rodada = EstatisticasEspera::novo();
        let n = self.max_chegadas as f64;
        //Acumula as estatísticas de média
        estatisticas_rodada.e_t1 = self.estatisticas_clientes_rodada.e_t1/n;
        estatisticas_rodada.e_t2 = self.estatisticas_clientes_rodada.e_t2/n;
        estatisticas_rodada.e_w1 = self.estatisticas_clientes_rodada.e_w1/n;
        estatisticas_rodada.e_w2 = self.estatisticas_clientes_rodada.e_w2/n;
        estatisticas_rodada.e_x1 = self.estatisticas_clientes_rodada.e_x1/n;
        estatisticas_rodada.e_x2 = self.estatisticas_clientes_rodada.e_x2/n;
        
        //Acumula a estatística de variância de w1
        estatisticas_rodada.v_w1 = self.estatisticas_clientes_rodada.v_w1/(n - 1.0);
        estatisticas_rodada.v_w1 += self.estatisticas_clientes_rodada.e_w1.powi(2)/(n*(n-1.0));

        //Acumula a estatística de variância de w2
        estatisticas_rodada.v_w2 = self.estatisticas_clientes_rodada.v_w2/(n - 1.0);
        estatisticas_rodada.v_w2 += self.estatisticas_clientes_rodada.e_w2.powi(2)/(n*(n-1.0));

        //Acumula a estatística de variância de t1
        estatisticas_rodada.v_t1 = self.estatisticas_clientes_rodada.v_t1/(n - 1.0);
        estatisticas_rodada.v_t1 += self.estatisticas_clientes_rodada.e_t1.powi(2)/(n*(n-1.0));

        //Acumula a estatística de variância de t2
        estatisticas_rodada.v_t2 = self.estatisticas_clientes_rodada.v_t2/(n - 1.0);
        estatisticas_rodada.v_t2 += self.estatisticas_clientes_rodada.e_t2.powi(2)/(n*(n-1.0));        

        //Zera as estatísticas acumuladas até agora na rodada atual, para que as da próxima rodada possa ser calculada em seguida
        self.estatisticas_clientes_rodada = EstatisticasEspera::novo();
        //Incrementa as estatísticas coletadas na variável de estado do servidor
        self.estatisticas_clientes_total.e_t1 += estatisticas_rodada.e_t1;
        self.estatisticas_clientes_total.e_t2 += estatisticas_rodada.e_t2;
        self.estatisticas_clientes_total.e_w1 += estatisticas_rodada.e_w1;
        self.estatisticas_clientes_total.e_w2 += estatisticas_rodada.e_w2;
        self.estatisticas_clientes_total.e_x1 += estatisticas_rodada.e_x1;
        self.estatisticas_clientes_total.e_x2 += estatisticas_rodada.e_x2;

        //Incrementa as variáveis que acumulam as variâncias por rodada de W1 e W2, para que suas médias sejam
        //futuramente calculadas e o intervalo de confiança do tipo t-Student seja estabelecido
        self.estatisticas_clientes_total.e_v_w1_sqr += estatisticas_rodada.v_w1;
        self.estatisticas_clientes_total.e_v_w2_sqr += estatisticas_rodada.v_w2;
        self.estatisticas_clientes_total.v_w1_sqr += estatisticas_rodada.v_w1.powi(2);
        self.estatisticas_clientes_total.v_w2_sqr += estatisticas_rodada.v_w2.powi(2);

        //Incrementa o desvio padrão, que possibilita futuramente obter também a variância amostral
        self.estatisticas_clientes_total.v_w1 += estatisticas_rodada.v_w1.sqrt();
        self.estatisticas_clientes_total.v_w2 += estatisticas_rodada.v_w2.sqrt();
        self.estatisticas_clientes_total.v_t1 += estatisticas_rodada.v_t1.sqrt();
        self.estatisticas_clientes_total.v_t2 += estatisticas_rodada.v_t2.sqrt();
    }

    //Função que verificar se o intervalo de confiança de 95% já foi atingido na rodada atual
    fn verifica_confianca_media(&mut self) {
        let n = self.rodada_atual as f64;
        /*println!("{}", ((self.estatisticas_clientes_total.v_w1/n) * PERCENTIL_TSTUDENT)/
                        (n.sqrt()*(self.estatisticas_clientes_total.e_w1/n)));*/
        if ((self.estatisticas_clientes_total.v_w1/n) * PERCENTIL_TSTUDENT)/n.sqrt() 
                < 0.05 * (self.estatisticas_clientes_total.e_w1/n) && !self.confianca_atingida.w1{
            self.confianca_atingida.w1 = true;
            if self.confianca_atingida.todas_confiancas_atingidas(){
                self.confianca_atingida.finalizou = true;
                println!("Atingiu 95% para W1 com {} rodadas", n);
            }
        }

        if ((self.estatisticas_clientes_total.v_w2/n) * PERCENTIL_TSTUDENT)/n.sqrt() 
                < 0.05 * (self.estatisticas_clientes_total.e_w2/n) && !self.confianca_atingida.w2{
            self.confianca_atingida.w2 = true;
            if self.confianca_atingida.todas_confiancas_atingidas(){
                self.confianca_atingida.finalizou = true;
                println!("Atingiu 95% para W2 com {} rodadas", n);
            }
        }

        if ((self.estatisticas_clientes_total.v_t1/n) * PERCENTIL_TSTUDENT)/n.sqrt() 
                < 0.05 * (self.estatisticas_clientes_total.e_t1/n) && !self.confianca_atingida.t1{
            self.confianca_atingida.t1 = true;
            if self.confianca_atingida.todas_confiancas_atingidas(){
                self.confianca_atingida.finalizou = true;
                println!("Atingiu 95% para T1 com {} rodadas", n);
            }
        }

        if ((self.estatisticas_clientes_total.v_t2/n) * PERCENTIL_TSTUDENT)/n.sqrt() 
                < 0.05 * (self.estatisticas_clientes_total.e_t2/n) && !self.confianca_atingida.t2{
            self.confianca_atingida.t2 = true;
            if self.confianca_atingida.todas_confiancas_atingidas(){
                self.confianca_atingida.finalizou = true;
                println!("Atingiu 95% para T2 com {} rodadas", n);
            }
        }

        let mut desvio_padrao : f64;

        desvio_padrao = self.n_clientes_total.v_n1/(n-1.0) + self.n_clientes_total.e_n1.powi(2)/(n*(n-1.0));
        desvio_padrao = desvio_padrao.sqrt();
        if (desvio_padrao * PERCENTIL_TSTUDENT)/n.sqrt() < 0.05 * (self.n_clientes_total.e_n1/n) &&
                !self.confianca_atingida.n1 {
            self.confianca_atingida.n1 = true;
            if self.confianca_atingida.todas_confiancas_atingidas(){
                self.confianca_atingida.finalizou = true;
                println!("Atingiu 95% para N1 com {} rodadas", n);
            }
        }

        desvio_padrao = self.n_clientes_total.v_n2/(n-1.0) + self.n_clientes_total.e_n2.powi(2)/(n*(n-1.0));
        desvio_padrao = desvio_padrao.sqrt();
        if (desvio_padrao * PERCENTIL_TSTUDENT)/n.sqrt() < 0.05 * (self.n_clientes_total.e_n2/n) &&
                !self.confianca_atingida.n2 {
            self.confianca_atingida.n2 = true;
            if self.confianca_atingida.todas_confiancas_atingidas(){
                self.confianca_atingida.finalizou = true;
                println!("Atingiu 95% para N2 com {} rodadas", n);
            }
        }

        desvio_padrao = self.n_clientes_total.v_nq1/(n-1.0) + self.n_clientes_total.e_nq1.powi(2)/(n*(n-1.0));
        desvio_padrao = desvio_padrao.sqrt();
        if (desvio_padrao * PERCENTIL_TSTUDENT)/n.sqrt() < 0.05 * (self.n_clientes_total.e_nq1/n) &&
                !self.confianca_atingida.nq1{
            self.confianca_atingida.nq1 = true;
            if self.confianca_atingida.todas_confiancas_atingidas(){
                self.confianca_atingida.finalizou = true;
                println!("Atingiu 95% para Nq1 com {} rodadas", n);
            }
        }

        desvio_padrao = self.n_clientes_total.v_nq2/(n-1.0) + self.n_clientes_total.e_nq2.powi(2)/(n*(n-1.0));
        desvio_padrao = desvio_padrao.sqrt();
        if (desvio_padrao * PERCENTIL_TSTUDENT)/n.sqrt() < 0.05 * (self.n_clientes_total.e_nq2/n) &&
                !self.confianca_atingida.nq2{
            self.confianca_atingida.nq2 = true;
            if self.confianca_atingida.todas_confiancas_atingidas(){
                self.confianca_atingida.finalizou = true;
                println!("Atingiu 95% para Nq2 com {} rodadas", n);
            }
        }
    }

    //Função executada ao fim da simulação, a mesma calcula o restante das estatísticas e as exibe na tela
    fn exibe_resultado(&mut self){
        let n = self.max_rodadas as f64;

        //Guarda a soma de todos os N1i, N2i, Nq1i e Nq2i colidos em nas rodadas i
        let mut e_n1 = self.n_clientes_total.e_n1;
        let mut e_n2 = self.n_clientes_total.e_n2;
        let mut e_nq1 = self.n_clientes_total.e_nq1;
        let mut e_nq2 = self.n_clientes_total.e_nq2;

        //Guarda a soma de todos os W1i, W2i, T1i e T2i colidos nas rodadas i e já os divide por N para obtermos os estimadores
        let e_w1 = self.estatisticas_clientes_total.e_w1/n;
        let e_w2 = self.estatisticas_clientes_total.e_w2/n;
        let e_t1 = self.estatisticas_clientes_total.e_t1/n;
        let e_t2 = self.estatisticas_clientes_total.e_t2/n;

        //Guarda a soma dos N1i², N2i², Nq1i² e Nq2i² acumulados nas rodadas i e os armazena para o futuro cáculo dos estimadores de suas variâncias
        let mut v_n1 = self.n_clientes_total.v_n1;
        let mut v_n2 = self.n_clientes_total.v_n2;
        let mut v_nq1 = self.n_clientes_total.v_nq1;
        let mut v_nq2 = self.n_clientes_total.v_nq2;

        //Guarda a soma dos desvios padrões de W1i, W2i, T1i e T2i acumulados nas rodadas i e os armazena para futuro cáculo dos estimadores de suas variâncias
        let dp_w1 = self.estatisticas_clientes_total.v_w1/n;
        let dp_w2 = self.estatisticas_clientes_total.v_w2/n;
        let dp_t1 = self.estatisticas_clientes_total.v_t1/n;
        let dp_t2 = self.estatisticas_clientes_total.v_t2/n;

        //Guarda os valores necessários para calcular os estimadores e intervalos de confiança das médias das variâncias por rodada i de W1i e W2i
        let mut e_v_w1_sqr= self.estatisticas_clientes_total.v_w1_sqr;
        let mut e_v_w2_sqr= self.estatisticas_clientes_total.v_w2_sqr;
        let mut v_w1_sqr = self.estatisticas_clientes_total.e_v_w1_sqr;
        let mut v_w2_sqr = self.estatisticas_clientes_total.e_v_w2_sqr;

        //Calcula os estimadores das variâncias de N1, N2, Nq1 e Nq2
        v_n1 = (v_n1/(n-1.0)) + (e_n1.powi(2)/(n*(n-1.0)));
        v_n2 = (v_n2/(n-1.0)) + (e_n2.powi(2)/(n*(n-1.0)));
        v_nq1 = (v_nq1/(n-1.0)) + (e_nq1.powi(2)/(n*(n-1.0)));
        v_nq2 = (v_nq2/(n-1.0)) + (e_nq2.powi(2)/(n*(n-1.0)));

        //Calcula os estimadores das variâncias de E[V(W1i)] e E[V(W2i)]
        v_w1_sqr = (v_w1_sqr/(n-1.0)) + (e_v_w1_sqr.powi(2)/(n*(n-1.0)));
        v_w2_sqr = (v_w2_sqr/(n-1.0)) + (e_v_w2_sqr.powi(2)/(n*(n-1.0)));

        //Calcula os estimadores de E[N1], E[N2], E[Nq1], E[Nq2]
        e_n1 /= n;
        e_n2 /= n;
        e_nq1 /= n;
        e_nq2 /= n;

        //Calcula os estimadores de E[V(W1)], E[V(W2)]
        e_v_w1_sqr /= n;
        e_v_w2_sqr /= n;


        //Abaixo encontram-se as exibições dos resultados da simulação
        println!("A simulação foi rodada Para as seguintes entradas:");
        println!("Rho :                     {}", self.rho);
        println!("Total de Rodadas :        {}", self.max_rodadas);
        println!("Chegadas por Rodada:      {}", self.max_chegadas);
        println!("Percentil Inferior Chi² : {}", self.chi_sqr_low);
        println!("Percentil Superior Chi² : {}\n", self.chi_sqr_up);
        println!("A partir das entradas, determinamos os seguintes parâmetros:");
        println!("Tamanho da Fase Transiente : {}", Self::tamanho_fase_transiente(self.rho));
        println!("Percentil t-student :        {}\n", PERCENTIL_TSTUDENT);
        println!("As seguintes estatisticas foram coletadas: ");
        println!("E[N1]  -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_n1, v_n1.sqrt()*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_n1), e_n1-PERCENTIL_TSTUDENT*v_n1.sqrt()/n.sqrt(), e_n1+PERCENTIL_TSTUDENT*v_n1.sqrt()/n.sqrt());

        println!("E[N2]  -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_n2, v_n2.sqrt()*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_n2), e_n2-PERCENTIL_TSTUDENT*v_n2.sqrt()/n.sqrt(), e_n2+PERCENTIL_TSTUDENT*v_n2.sqrt()/n.sqrt());

        println!("E[Nq1] -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_nq1, v_nq1.sqrt()*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_nq1), e_nq1-PERCENTIL_TSTUDENT*v_nq1.sqrt()/n.sqrt(), e_nq1+PERCENTIL_TSTUDENT*v_nq1.sqrt()/n.sqrt());

        println!("E[Nq2] -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_nq2, v_nq2.sqrt()*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_nq2), e_nq2-PERCENTIL_TSTUDENT*v_nq2.sqrt()/n.sqrt(), e_nq2+PERCENTIL_TSTUDENT*v_nq2.sqrt()/n.sqrt());

        println!("E[W1]  -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_w1, dp_w1*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_w1), e_w1-PERCENTIL_TSTUDENT*dp_w1/n.sqrt(),e_w1+PERCENTIL_TSTUDENT*dp_w1/n.sqrt());
        
        println!("E[W2]  -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_w2, dp_w2*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_w2), e_w2-PERCENTIL_TSTUDENT*dp_w2/n.sqrt(),e_w2+PERCENTIL_TSTUDENT*dp_w2/n.sqrt());

        println!("E[T1]  -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_t1, dp_t1*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_t1), e_t1-PERCENTIL_TSTUDENT*dp_t1/n.sqrt(),e_t1+PERCENTIL_TSTUDENT*dp_t1/n.sqrt());

        println!("E[T2]  -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_t2, dp_t2*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_t2), e_t2-PERCENTIL_TSTUDENT*dp_t2/n.sqrt(),e_t2+PERCENTIL_TSTUDENT*dp_t2/n.sqrt());
        
        println!("\nPara as cada variância teremos um intervalo de confiança Chi² e um T-Student: ");

        let precisao_chi = (self.chi_sqr_up-self.chi_sqr_low)/(self.chi_sqr_low + self.chi_sqr_up);

        print!("Chi²:      ");
        println!("V(W1)      -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            dp_w1.powi(2),precisao_chi,(n-1.0)*dp_w1.powi(2)/self.chi_sqr_up,(n-1.0)*dp_w1.powi(2)/self.chi_sqr_low);

        print!("T-Student: ");
        println!("E[Vi(W1)]  -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_v_w1_sqr,v_w1_sqr.sqrt()*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_v_w1_sqr), e_v_w1_sqr-PERCENTIL_TSTUDENT*v_w1_sqr.sqrt()/n.sqrt(),e_v_w1_sqr+PERCENTIL_TSTUDENT*v_w1_sqr.sqrt()/n.sqrt());
        
        print!("Chi²:      ");
        println!("V(W2)      -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            dp_w2.powi(2),precisao_chi,(n-1.0)*dp_w2.powi(2)/self.chi_sqr_up,(n-1.0)*dp_w2.powi(2)/self.chi_sqr_low);

        print!("T-Student: ");
        println!("E[Vi(W2)]  -> valor: {:.12} | precisão: {:.12}% | intervalo de confiança: ({:.12},{:.12})",
            e_v_w2_sqr,v_w2_sqr.sqrt()*PERCENTIL_TSTUDENT*100.0/(n.sqrt()*e_v_w2_sqr), e_v_w2_sqr-PERCENTIL_TSTUDENT*v_w2_sqr.sqrt()/n.sqrt(),e_v_w2_sqr+PERCENTIL_TSTUDENT*v_w2_sqr.sqrt()/n.sqrt());
    }
}