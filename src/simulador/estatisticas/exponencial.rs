use rand::{prelude::*, distributions::Uniform};

#[derive(Debug)]
pub struct AmostraExp{
    seed : u64,
    e_det: bool,
    det_atual : u64
}

impl AmostraExp{

    pub fn novo(e_det : bool, seed : u64) -> Self{
        if e_det {
            return Self{
                seed,
                e_det : true,
                det_atual: 0
            }; 
        }
        else{
            return Self{
                seed : 0,
                e_det: false,
                det_atual: 0
            };
        }
    }

    pub fn novo_det(seed : u64) -> Self {
        Self{
            seed,
            e_det : true,
            det_atual: 0
        }
    }

    pub fn prox_det(&mut self,lambda : f64) -> f64 {
        self.det_atual += 1;
        self.det_atual %= self.seed;
        return (1.0/(self.det_atual as f64 + 1.1)).ln()/(-lambda);
    }

    //Retorna uma exponencial com a semente proveninente do próprio sistema
    pub fn amostra_exp(&mut self, lambda : f64) -> f64 {
        if self.e_det{
            return self.prox_det(lambda);
        }else{
            //usando o menor positivo possível como limite inferior para não lidar com ln(0)
            return -1.0*(thread_rng().sample(Uniform::new(std::f64::MIN_POSITIVE,1.0 as f64)).ln())/lambda
        }
    }
}