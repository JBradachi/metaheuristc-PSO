use crate::problema::*;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::hc::*;

fn calcular_fitness(
    posicao: &[f64],
    problema: &ProblemaEnsopado,
    limiar: f64,
) -> f64 {
    let solucao_binaria: Vec<bool> = posicao.iter().map(|&p| p > limiar).collect();

    // Se encontrarmos um par incompatível, a solução é inválida. Fitness = 0.
    for &(ing1_idx, ing2_idx) in &problema.pares_incompativeis {
        if solucao_binaria[ing1_idx] && solucao_binaria[ing2_idx] {
            return 0.00001; // Retorna um valor mínimo para evitar ficar preso no zero absoluto.
        }
    }

    let mut sabor_total = 0.0;
    let mut peso_total = 0.0;

    for (i, &incluido) in solucao_binaria.iter().enumerate() {
        if incluido {
            peso_total += problema.ingredientes[i].peso;
            sabor_total += problema.ingredientes[i].sabor;
        }
    }
    
    if peso_total > problema.peso_max {
        // A penalidade gradual para o peso 
        let excesso = peso_total - problema.peso_max;
        let penalidade = 100.0 * excesso;
        return (sabor_total - penalidade).max(0.00001);
    }

    // Se a solução passou por todas as verificações, o fitness é o sabor total.
    sabor_total
}

fn gerar_solucao_valida(problema: &ProblemaEnsopado, rng: &mut impl Rng) -> Vec<bool> {
    let num_ingredientes = problema.ingredientes.len();
    let mut solucao = vec![false; num_ingredientes];
    let mut peso_atual = 0.0;

    // Criar e embaralhar a ordem dos candidatos
    let mut candidatos: Vec<usize> = (0..num_ingredientes).collect();
    candidatos.shuffle(rng);

    // Tentar adicionar cada candidato
    for &idx in &candidatos {
        // Verificar se a adição é segura
        let peso_potencial = peso_atual + problema.ingredientes[idx].peso;

        // Verifica o peso
        if peso_potencial > problema.peso_max {
            continue; // Pula para o próximo candidato
        }

        // Verifica incompatibilidades com os já escolhidos
        let mut tem_conflito = false;
        for (i, &incluido) in solucao.iter().enumerate() {
            if incluido {
                // Verifica se o par (i, idx) é incompatível
                if problema.pares_incompativeis.contains(&(i, idx)) || problema.pares_incompativeis.contains(&(idx, i)) {
                    tem_conflito = true;
                    break;
                }
            }
        }

        if tem_conflito {
            continue; // Pula para o próximo candidato
        }
        
        // Se for seguro, adicionar à solução
        solucao[idx] = true;
        peso_atual = peso_potencial;
    }

    solucao
}

fn gerar_posicao_de_solucao(solucao: &[bool], rng: &mut impl Rng) -> Vec<f64> {
    solucao
        .iter()
        .map(|&valido| if valido { rng.gen_range(0.7..1.0) } else { rng.gen_range(0.0..0.3) })
        .collect()
}

pub fn solve_pso(problema: &ProblemaEnsopado) -> (Vec<bool>, f64, Vec<bool>, f64) {
    // --- Parâmetros do PSO ---
    let num_particulas = 100;
    let num_iteracoes = 50;
    let dimensoes = problema.ingredientes.len();
    let limiar_conversao = 0.5;
    let w_max = 0.9;
    let w_min = 0.4;
    let c1 = 2.0;
    let c2 = 2.0;
    let v_max = 1.0;
    let mut sol_inicial = Particula{
                ..Default::default()
            };

    let mut iteracoes_sem_melhora = 0;
    const LIMITE_ESTAGNACAO: usize = 15;

    let mut rng = rand::thread_rng();
    let mut enxame: Vec<Particula> = Vec::with_capacity(num_particulas);

    for _ in 0..num_particulas {
        let solucao_valida = gerar_solucao_valida(problema, &mut rng);

        let posicao_inicial: Vec<f64> = solucao_valida
            .iter()
            .map(|&valido| if valido { rng.gen_range(0.7..1.0) } else { rng.gen_range(0.0..0.3) })
            .collect();
        
        let p = Particula {
            posicao: posicao_inicial.clone(),
            velocidade: (0..dimensoes).map(|_| rng.gen_range(-0.1..0.1)).collect(),
            melhor_posicao: posicao_inicial,
            melhor_fitness: -f64::INFINITY, // Será calculado a seguir
        };
        enxame.push(p);
    }
    let mut melhor_posicao_global = vec![0.0; dimensoes];
    let mut melhor_fitness_global = -f64::INFINITY;

    for p in &mut enxame {
        let fitness = calcular_fitness(&p.posicao, problema, limiar_conversao);
        p.melhor_fitness = fitness;
        let solucao_binaria_melhorada: Vec<bool> = p.posicao.iter().map(|&pos| pos > limiar_conversao).collect();
        p.melhor_posicao = gerar_posicao_de_solucao(&solucao_binaria_melhorada, &mut rng);
        if fitness > melhor_fitness_global {
            melhor_fitness_global = fitness;
            let solucao_binaria_melhorada: Vec<bool> = p.posicao.iter().map(|&pos| pos > limiar_conversao).collect();
            melhor_posicao_global = gerar_posicao_de_solucao(&solucao_binaria_melhorada, &mut rng);
            sol_inicial.melhor_fitness = fitness;
            sol_inicial.melhor_posicao = melhor_posicao_global.clone();
            sol_inicial.posicao = melhor_posicao_global.clone();
            sol_inicial.velocidade = p.velocidade.clone();

        }
    }

    for iter in 0..num_iteracoes {
        let gbest_anterior = melhor_fitness_global;
        let w = w_max - (iter as f64 / num_iteracoes as f64) * (w_max - w_min);
        for p in &mut enxame {
            let mut c2_ajustado = c2;
            if iteracoes_sem_melhora > LIMITE_ESTAGNACAO {
                // Força a partícula a fugir do gbest.
                c2_ajustado = -c2; 
                // reseta o contador para não ficar no modo repulsivo para sempre
                if iteracoes_sem_melhora > LIMITE_ESTAGNACAO + 5 {
                    iteracoes_sem_melhora = 0;
                }
            }
            for i in 0..dimensoes {
                let r1: f64 = rng.r#gen();
                let r2: f64 = rng.r#gen();
                let componente_cognitivo = c1 * r1 * (p.melhor_posicao[i] - p.posicao[i]);
                let componente_social = c2_ajustado * r2 * (melhor_posicao_global[i] - p.posicao[i]);
                let nova_velocidade = w * p.velocidade[i] + componente_cognitivo + componente_social;
                p.velocidade[i] = nova_velocidade.max(-v_max).min(v_max);
                p.posicao[i] += p.velocidade[i];
                p.posicao[i] = p.posicao[i].max(0.0).min(1.0);
            }

            let mut fitness_atual = calcular_fitness(&p.posicao, problema, limiar_conversao);
            if fitness_atual == melhor_fitness_global{
                let solucao_pso: Vec<bool> = p.posicao.iter().map(|&pos| pos > limiar_conversao).collect();

                let (solucao_final, _) = busca_local_exaustiva(&solucao_pso, problema, limiar_conversao);
                let posicao: Vec<f64> = solucao_final
                    .iter()
                    .map(|&valido| if valido { rng.gen_range(0.7..1.0) } else { rng.gen_range(0.0..0.3) })
                    .collect();
                p.posicao = posicao.clone();
                fitness_atual = calcular_fitness(&p.posicao, problema, limiar_conversao);
            }
            if fitness_atual > p.melhor_fitness {
                p.melhor_fitness = fitness_atual;
                let solucao_binaria_melhorada: Vec<bool> = p.posicao.iter().map(|&pos| pos > limiar_conversao).collect();
                p.melhor_posicao = gerar_posicao_de_solucao(&solucao_binaria_melhorada, &mut rng);
            }

            if fitness_atual > melhor_fitness_global {
                melhor_fitness_global = fitness_atual;
                let solucao_binaria_melhorada: Vec<bool> = p.posicao.iter().map(|&pos| pos > limiar_conversao).collect();
                melhor_posicao_global = gerar_posicao_de_solucao(&solucao_binaria_melhorada, &mut rng);
            }
        }
        // println!(
        //     "Iteração {}: Melhor Fitness Global = {:.2}",
        //     iter, melhor_fitness_global
        // );
        if melhor_fitness_global > gbest_anterior {
            iteracoes_sem_melhora = 0;
        } else {
            iteracoes_sem_melhora += 1;
        }
    }
    
    let solucao_final_binaria: Vec<bool> = melhor_posicao_global.iter().map(|&p| p > limiar_conversao).collect();
    
    let mut sabor_final = 0.0;
    let mut peso_final = 0.0;
    let mut valida = true;

    for &(ing1_idx, ing2_idx) in &problema.pares_incompativeis {
        if solucao_final_binaria[ing1_idx] && solucao_final_binaria[ing2_idx] {
            valida = false;
            break;
        }
    }
    
    if valida {
        for (i, &incluido) in solucao_final_binaria.iter().enumerate() {
            if incluido {
                peso_final += problema.ingredientes[i].peso;
            }
        }
        if peso_final > problema.peso_max {
            valida = false;
        }
    }

    if valida {
        for (i, &incluido) in solucao_final_binaria.iter().enumerate() {
             if incluido {
                sabor_final += problema.ingredientes[i].sabor;
            }
        }
        let solucao_inicial_binaria : Vec<bool> = sol_inicial.melhor_posicao.iter().map(|&p| p > limiar_conversao).collect();
        (solucao_final_binaria, sabor_final, solucao_inicial_binaria, sol_inicial.melhor_fitness)
    } else {
        // Se, por algum motivo, a melhor solução ainda for inválida, retorne nulo.
        (vec![false; dimensoes], 0.0, vec![false; dimensoes], 0.0)
    }
}