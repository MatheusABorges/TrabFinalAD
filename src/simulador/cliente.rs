use super::enums::Cor;

//estrutura que armazena as informações referentes a um cliente da fila
#[derive(Debug)]
pub struct Cliente{
    //instante de tempo em que ocorre a sua chegada
    pub chegada : f64,
    //total de tempo necessário paara executar o serviço 1
    pub servico_1 : f64,
    //total de tempo necessário paara executar o serviço 2
    pub servico_2 : f64,
    //tempo restante para a conclusão do serviço 2
    pub resta_servico_2 : f64,
    //instante de tempo em que o serviço 1 é iniciado
    pub inicia_1 : f64,
    //instante de tempo em que o serviço 2 é iniciado
    pub inicia_2 : f64,
    ////instante de tempo em que o serviço 1 é terminado
    pub termina_1 : f64,
    //instante de tempo em que o serviço 2 é terminado
    pub termina_2 : f64,
    pub cor : Cor
}

impl Cliente{
    //Cria um novo cliente
    pub fn novo(chegada_:f64, servico_1_ : f64, servico_2_ : f64, cor_ : Cor) -> Self{
        Self{
            chegada : chegada_,
            servico_1 : servico_1_,
            servico_2 : servico_2_,
            resta_servico_2: servico_2_,
            inicia_1 : -1.0,
            inicia_2 :-1.0,
            termina_1 : -1.0,
            termina_2 :-1.0,
            cor : cor_
        }
    }

    //retorna o tempo total que o cliente passou na fila 1 de espera(sem considerar o tempo de serviço)
    pub fn tempo_w1(&self) -> f64{
        self.inicia_1 - self.chegada
    }

    //retorna o tempo total que o cliente passou na fila 1(considerando o tempo de serviço)
    pub fn tempo_t1(&self) -> f64{
        self.tempo_w1() + self.servico_1
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