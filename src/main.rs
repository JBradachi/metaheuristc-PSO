use metaheuristc_pso::{problema::ProblemaEnsopado, pso::solve_pso, *};
fn main() {
    let mut problema = ProblemaEnsopado::load_from("instances/ep02.dat");
    println!("ep01");

    for i in 0..30 {
        let tempo_inicial = std::time::Instant::now();
        let (solucao_inicial, melhor_solucao) = solve_pso(&mut problema);
        let tempo_final = tempo_inicial.elapsed();
        println!("Iteração {:?}:", i + 1);
        println!("- Solução inicial {:?}", solucao_inicial);
        println!("- Solução final {:?}", melhor_solucao);
        println!("- Tempo: {:?}", tempo_final);
    }
}
