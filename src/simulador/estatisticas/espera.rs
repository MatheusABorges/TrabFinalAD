//Estrutura que armazena as estatísticas de clientes a cada rodada
pub struct EstatisticasEspera{
    pub e_x2 : f64,
    pub e_x1 : f64,
    pub e_w1 : f64,
    pub e_w2 : f64,
    pub e_t1 : f64,
    pub e_t2 : f64,
    pub v_w1 : f64,
    pub v_w2 : f64,
}

impl EstatisticasEspera {
    pub fn novo() -> Self {
        Self{
            e_x1 : 0.0,
            e_x2 : 0.0,
            e_w1 : 0.0,
            e_w2 : 0.0,
            e_t1 : 0.0,
            e_t2 : 0.0,
            v_w1 : 0.0,
            v_w2 : 0.0
        }
    }
}