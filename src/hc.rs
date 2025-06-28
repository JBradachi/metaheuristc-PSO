use crate::problema::{self, ProblemaEnsopado};

pub fn busca_local_exaustiva(
    solucao_inicial: &[bool],
    problema: &ProblemaEnsopado,
    limiar: f64,
) -> (Vec<bool>, f64) {
    let mut solucao_atual = solucao_inicial.to_vec();
    // A posição contínua não importa aqui, só o fitness da solução binária
    let mut fitness_atual = calcular_fitness_binario(&solucao_atual, problema);
    
    println!("\n--- A Iniciar Busca Local Intensiva ---");
    println!("Fitness de partida: {:.2}", fitness_atual);

    loop {
        let mut melhor_vizinho = solucao_atual.clone();
        let mut melhor_fitness_vizinho = fitness_atual;
        
        // 1. Itera sobre toda a vizinhança de 1-flip
        for i in 0..problema.ingredientes.len() {
            let mut vizinho_candidato = solucao_atual.clone();
            vizinho_candidato[i] = !vizinho_candidato[i]; // Vira o bit

            let fitness_vizinho = calcular_fitness_binario(&vizinho_candidato, problema);

            if fitness_vizinho > melhor_fitness_vizinho {
                melhor_fitness_vizinho = fitness_vizinho;
                melhor_vizinho = vizinho_candidato;
            }
        }

        // 2. Compara o melhor vizinho com a solução atual
        if melhor_fitness_vizinho > fitness_atual {
            println!("Busca Local encontrou melhora: {:.2} -> {:.2}", fitness_atual, melhor_fitness_vizinho);
            solucao_atual = melhor_vizinho;
            fitness_atual = melhor_fitness_vizinho;
        } else {
            // Se nenhum vizinho é melhor, atingimos um ótimo local.
            println!("Nenhuma melhora encontrada na vizinhança. Busca Local terminada.");
            break;
        }
    }
    
    (solucao_atual, fitness_atual)
}

// Crie esta função auxiliar que opera diretamente sobre Vec<bool> para simplificar
fn calcular_fitness_binario(solucao: &[bool], problema: &ProblemaEnsopado) -> f64 {
    // Reutiliza a lógica de calcular_fitness, mas sem a conversão de posição
    for &(ing1_idx, ing2_idx) in &problema.pares_incompativeis {
        if solucao[ing1_idx] && solucao[ing2_idx] { return -1.0; } // Use um valor negativo para inválidos
    }
    let mut sabor_total = 0.0;
    let mut peso_total = 0.0;
    for (i, &incluido) in solucao.iter().enumerate() {
        if incluido {
            peso_total += problema.ingredientes[i].peso;
            sabor_total += problema.ingredientes[i].sabor;
        }
    }
    if peso_total > problema.peso_max {
        return -(peso_total - problema.peso_max); // Retorna o excesso como negativo
    }
    sabor_total
}