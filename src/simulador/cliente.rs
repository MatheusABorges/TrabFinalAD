use super::enums::Cor;

//estrutura que armazena as informações referentes a um cliente da fila
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Cliente{
    //instante de tempo em que ocorre a sua chegada
    pub chegada : f64,
    //total de tempo necessário paara executar o serviço 1
    pub servico_1 : f64,
    //total de tempo necessário paara executar o serviço 2
    pub servico_2 : f64,
    //tempo restante para a conclusão do serviço 2
    pub resta_servico_2 : f64,
    ////instante de tempo em que o serviço 1 é terminado
    pub termina_1 : f64,
    //instante de tempo em que o serviço 2 é terminado
    pub termina_2 : f64,
    //guarda a rodada em que o cliente foi gerado e é usada para o funcionamento do método batch
    pub rodada : usize,
    pub cor : Cor
}

impl Cliente{
    //Cria um novo cliente
    pub fn novo(chegada:f64, servico_1 : f64, servico_2: f64, cor : Cor, rodada : usize) -> Self{
        Self{
            chegada,
            servico_1,
            servico_2,
            resta_servico_2 : servico_2,
            termina_1 : -1.0,
            termina_2 :-1.0,
            rodada,
            cor
        }
    }

    //retorna o tempo total que o cliente passou na fila 1 de espera(sem considerar o tempo de serviço)
    pub fn tempo_w1(&self) -> f64{
         self.tempo_t1() - self.servico_1
    }

    //retorna o tempo total que o cliente passou na fila 1(considerando o tempo de serviço)
    pub fn tempo_t1(&self) -> f64{
        self.termina_1 - self.chegada
    }

    //retorna o tempo total que o cliente passou na fila 2 de espera(sem considerar o tempo de serviço)
    pub fn tempo_w2(&self) -> f64{
        self.tempo_t2() - self.servico_2
    }

    //retorna o tempo total que o cliente passou na fila 1(considerando o tempo de serviço)
    pub fn tempo_t2(&self) -> f64{
        self.termina_2 - self.termina_1
    }
}