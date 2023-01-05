use super::enums::TipoEvento;

#[derive(Debug)]
pub struct Evento{
    tipo : TipoEvento,
    tempo : f64
}