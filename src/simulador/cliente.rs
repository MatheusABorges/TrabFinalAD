//estrutura que armazena as informações referentes a um cliente da fila
#[derive(Debug)]
pub struct Cliente{
    //instante de tempo em que ocorre a sua chegada
    chegada : f64,
    //total de tempo necessário paara executar o serviço 1
    servico_1 : f64,
    //total de tempo necessário paara executar o serviço 2
    servico_2 : f64,
    //instante de tempo em que o serviço 1 é iniciado
    inicia_1 : f64,
    //instante de tempo em que o serviço 2 é iniciado
    inicia_2 : f64,
    ////instante de tempo em que o serviço 1 é terminado
    termina_1 : f64,
    //instante de tempo em que o serviço 2 é terminado
    termina_2 : f64,
    cor : u32
}

impl Cliente{
    //Cria um novo cliente
    pub fn novo(chegada_:f64, cor_:u32) -> Self{
        Self{
            chegada : chegada_,
            servico_1 : -1.0,
            servico_2 : -1.0,
            inicia_1 : -1.0,
            inicia_2 :-1.0,
            termina_1 : -1.0,
            termina_2 :-1.0,
            cor : cor_
        }
    }
}