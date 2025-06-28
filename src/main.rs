use metaheuristc_pso::{problema::ProblemaEnsopado, pso::solve_pso, *};

fn visualiza_solucao(sol: Vec<bool>) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    
    for (i, &incluido) in sol.iter().enumerate() {
        if incluido {
            indices.push(i);
        }
    }
    indices
}
fn main() {
    let mut problema = ProblemaEnsopado::load_from("instances/ep10.dat");
    println!("ep10");

    for i in 0..30 {
        let tempo_inicial = std::time::Instant::now();

        let (solucao_final, melhor_solucao
            , solucao_inicial, fitness_inicial) 
            = solve_pso(&mut problema);

        let tempo_final = tempo_inicial.elapsed();
        println!(" ----------------------- \n
                    Iteração {:?}:", i + 1);
        println!("- Solução inicial \n{:?}", visualiza_solucao(solucao_inicial));
        println!("- Resultado inicial: {:?}", fitness_inicial);
        println!("- Solução final \n{:?}", visualiza_solucao(solucao_final));
        println!("- Resultado final: {:?}", melhor_solucao);
        println!("- Tempo: {:?}", tempo_final);
    }
}
