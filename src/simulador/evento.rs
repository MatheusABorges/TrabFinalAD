use super::enums::TipoEvento;

#[derive(Debug)]
#[derive(Clone, Copy)]
//Estrutura que armazena os eventos da simulação
pub struct Evento {
    //armazena qual o tipo do evento
    pub tipo : TipoEvento,
    //instante de tempo em que o evento ocorrerá
    pub tempo : f64,
    //instante de tempo em que o evento foi criado
    pub criacao : f64
}

impl Evento {
    pub fn novo(tipo: TipoEvento, tempo: f64, criacao: f64) -> Self {
        Self{
            tipo,
            tempo,
            criacao
        }
    }
}