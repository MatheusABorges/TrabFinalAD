use rand::{prelude::*, distributions::Uniform};

//Retorna uma exponencial com a semente proveninente do próprio sistema
pub fn amostra_exp(lambda : f64) -> f64 {
    //usando o menor positivo possível como limite inferior para não lidar com ln(0)
    -1.0*(thread_rng().sample(Uniform::new(std::f64::MIN_POSITIVE,1.0 as f64)).ln())/lambda
}