#[derive(Debug)]
pub enum Cor {
    //cor do cliente que será atendido por serviço do tipo 1
    BRANCO,
    //cor do cliente que será atendido por serviço do tipo 2
    PRETO
}

#[derive(Debug)]
pub enum TipoEvento {
    FIM_SERVICO_1,
    FIM_SERVICO_2,
    CHEGADA
}