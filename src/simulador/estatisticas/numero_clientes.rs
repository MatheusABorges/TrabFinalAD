//Estrutura que armazena e contabiliza o número de clientes na fila e o instante de tempo a cada execução de evento
pub struct NClientes{
    //Número atual de clientes na fila 1 + número de clientes em serviço da cor branca
    pub n1 : usize,
    //Número atual de clientes na fila 1
    pub nq1 : usize,
    //Número atual de clientes na fila 2 + número de clientes em serviço da cor branca
    pub n2 : usize,
    //Número atual de clientes na fila 2
    pub nq2 : usize,
    //Média de clientes na fila 1 + número de clientes em serviço da cor branca
    pub e_n1 : f64,
    //Média de clientes na fila 1
    pub e_nq1 : f64,
    //Média de clientes na fila 2 + número de clientes em serviço da cor branca
    pub e_n2 : f64,
    //Média de clientes na fila 2
    pub e_nq2 : f64,
    //Acumula o desvio padrão de todas as rodadas até a atual
    pub v_n1 : f64,
    //Acumula o desvio padrão de todas as rodadas até a atual
    pub v_n2 : f64,
    //Acumula o desvio padrão de todas as rodadas até a atual
    pub v_nq1 : f64,
    //Acumula o desvio padrão de todas as rodadas até a atual
    pub v_nq2 : f64,
    //Instante de tempo em que a fila passou a ter os valores acima
    pub t : f64
}

impl NClientes {
    pub fn novo() -> Self {
        Self{
            n1 : 0,
            e_n1 : 0.0,
            nq1 : 0,
            e_nq1 : 0.0,
            n2 : 0,
            e_n2 : 0.0,
            nq2 : 0,
            e_nq2 : 0.0,
            v_n1 : 0.0,
            v_n2 : 0.0,
            v_nq1 : 0.0,
            v_nq2 : 0.0,
            t : 0.0
        }
    }
}