use std::collections::{HashMap, HashSet, VecDeque};

pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
}

pub fn cadastrar_produto(
    produtos: &mut HashMap<u32, Produto>,
    produto: Produto,
) {
    produtos.insert(produto.id, produto);
}

pub fn consultar_produto(
    produtos: &HashMap<u32, Produto>,
    id: u32,
) {
    if let Some(produto) = produtos.get(&id) {
        println!(
            "Produto encontrado: {} - {}",
            produto.nome,
            produto.categoria
        );
    } else {
        println!("Produto nao encontrado.");
    }
}

pub fn adicionar_conexao(
    grafo: &mut HashMap<u32, Vec<u32>>,
    produto_a: u32,
    produto_b: u32,
) {
    let conexoes_a = grafo.entry(produto_a).or_insert(Vec::new());

    if !conexoes_a.contains(&produto_b) {
        conexoes_a.push(produto_b);
    }

    let conexoes_b = grafo.entry(produto_b).or_insert(Vec::new());

    if !conexoes_b.contains(&produto_a) {
        conexoes_b.push(produto_a);
    }
}

pub fn recomendar_bfs(
    grafo: &HashMap<u32, Vec<u32>>,
    inicio: u32,
) -> Vec<u32> {
    let mut visitados: HashSet<u32> = HashSet::new();
    let mut fila: VecDeque<u32> = VecDeque::new();
    let mut recomendacoes: Vec<u32> = Vec::new();

    visitados.insert(inicio);
    fila.push_back(inicio);

    while let Some(atual) = fila.pop_front() {
        if let Some(vizinhos) = grafo.get(&atual) {
            for vizinho in vizinhos {
                if !visitados.contains(vizinho) {
                    visitados.insert(*vizinho);
                    fila.push_back(*vizinho);
                    recomendacoes.push(*vizinho);
                }
            }
        }
    }

    recomendacoes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_adicionar_conexao() {
        let mut grafo: HashMap<u32, Vec<u32>> = HashMap::new();

        adicionar_conexao(&mut grafo, 1, 2);

        assert!(grafo.get(&1).unwrap().contains(&2));
        assert!(grafo.get(&2).unwrap().contains(&1));
    }

    #[test]
    fn testa_bfs_sem_duplicatas() {
        let mut grafo: HashMap<u32, Vec<u32>> = HashMap::new();

        adicionar_conexao(&mut grafo, 1, 2);
        adicionar_conexao(&mut grafo, 1, 3);
        adicionar_conexao(&mut grafo, 2, 3);

        let recomendacoes = recomendar_bfs(&grafo, 1);

        assert_eq!(recomendacoes.len(), 2);
        assert!(recomendacoes.contains(&2));
        assert!(recomendacoes.contains(&3));
        assert!(!recomendacoes.contains(&1));
    }

    #[test]
    fn testa_conexao_sem_duplicacao() {
     let mut grafo: HashMap<u32, Vec<u32>> = HashMap::new();

        adicionar_conexao(&mut grafo, 1, 2);
        adicionar_conexao(&mut grafo, 1, 2);

        assert_eq!(grafo.get(&1).unwrap().len(), 1);
        assert_eq!(grafo.get(&2).unwrap().len(), 1);
    }
}