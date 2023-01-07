#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Cor {
    //cor do cliente que será atendido por serviço do tipo 1
    BRANCO,
    //cor do cliente que será atendido por serviço do tipo 2
    PRETO
}

#[derive(Debug)]
pub enum TipoEvento {
    FimServico1,
    FimServico2,
    CHEGADA
}