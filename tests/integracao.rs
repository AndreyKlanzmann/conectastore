use std::collections::HashMap;

use conectastore::{
    Produto,
    cadastrar_produto,
    adicionar_conexao,
    recomendar_bfs,
};

#[test]
fn testa_fluxo_de_recomendacao() {
    let mut produtos: HashMap<u32, Produto> = HashMap::new();

    let produto1 = Produto {
        id: 1,
        nome: String::from("Notebook"),
        categoria: String::from("Eletronicos"),
    };

    let produto2 = Produto {
        id: 2,
        nome: String::from("Mouse"),
        categoria: String::from("Eletronicos"),
    };

    let produto3 = Produto {
        id: 3,
        nome: String::from("Mousepad"),
        categoria: String::from("Eletronicos"),
    };

    cadastrar_produto(&mut produtos, produto1);
    cadastrar_produto(&mut produtos, produto2);
    cadastrar_produto(&mut produtos, produto3);

    let mut grafo: HashMap<u32, Vec<u32>> = HashMap::new();

    adicionar_conexao(&mut grafo, 1, 2);
    adicionar_conexao(&mut grafo, 2, 3);

    let recomendacoes = recomendar_bfs(&grafo, 1);

    assert_eq!(produtos.len(), 3);
    assert!(recomendacoes.contains(&2));
    assert!(recomendacoes.contains(&3));
    assert!(!recomendacoes.contains(&1));
}