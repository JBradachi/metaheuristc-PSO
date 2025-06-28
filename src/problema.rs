use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug)]
pub struct Ingrediente {
    pub sabor: f64,
    pub peso: f64,
    
}

#[derive(Default, Debug)]
pub struct ProblemaEnsopado {
    pub ingredientes: Vec<Ingrediente>,
    pub pares_incompativeis: HashSet<(usize, usize)>,
    pub peso_max: f64,
}

#[derive(Default, Debug)]
pub struct Particula {
    pub posicao: Vec<f64>,
    pub velocidade: Vec<f64>,
    pub melhor_posicao: Vec<f64>,
    pub melhor_fitness: f64,
}

fn read_floats(text: &str) -> Vec<f64> {
    text.split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect()
}

impl Particula {
    pub fn new(dimensoes: usize) -> Self {
        let mut rng = rand::thread_rng();
        Particula {
            posicao: (0..dimensoes).map(|_| rng.r#gen::<f64>()).collect(),
            velocidade: (0..dimensoes).map(|_| rng.gen_range(-0.1..0.1)).collect(),
            melhor_posicao: vec![0.0; dimensoes],
            melhor_fitness: -1.0, 
        }
    }
}
impl ProblemaEnsopado {
    pub fn load_from(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Arquivo não pode ser aberto");
        let mut lines = BufReader::new(file).lines();

        // Primeira linha:
        // N: número de ingredientes
        // I: número de ingredientes incompatíveis
        // W: peso máximo
        let first_line = lines.next().unwrap().unwrap(); // sabemos que essa linha existe
        let nums = read_floats(&first_line);
        let num_ingred = nums[0] as usize;
        let num_incompat: usize = nums[1] as usize;
        let peso_max = nums[2];

        // Inicialização do vetor de ingredientes com o tamanho certo
        let mut ingredientes: Vec<Ingrediente> = Vec::with_capacity(num_ingred);
        ingredientes.resize_with(num_ingred, Default::default);

        let mut pares_incompativeis: HashSet<(usize, usize)> = HashSet::new();

        // Próximos N números: sabores
        lines.next(); // linha em branco
        let mut i = 0;
        while i < num_ingred {
            let line = lines.next().unwrap().unwrap();
            for &sabor in read_floats(&line).iter() {
                ingredientes[i].sabor = sabor;
                i += 1;
            }
        }

        // Próximos N números: pesos
        lines.next(); // linha em branco
        let mut i = 0;
        while i < num_ingred {
            let line = lines.next().unwrap().unwrap();
            for &peso in read_floats(&line).iter() {
                ingredientes[i].peso = peso;
                i += 1;
            }
        }

        // Próximas I linhas: pares j,k de incompatibilidades
        lines.next(); // linha em branco
        for _ in 0..num_incompat {
            let line = lines.next().unwrap().unwrap();
            let pair = read_floats(&line);
            let j = pair[0] as usize - 1;
            let k = pair[1] as usize - 1;
            pares_incompativeis.insert((j, k)); 
            // pares_incompativeis[j].insert(k);
            // pares_incompativeis[k].insert(j); se precisar descomente, com isso gera um grafo sem direção
        }
        ProblemaEnsopado {
            ingredientes,
            pares_incompativeis,
            peso_max,
        }
    }
}