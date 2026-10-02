use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Instant;

use conectastore::{
    Produto,
    cadastrar_produto,
    consultar_produto,
    adicionar_conexao,
    recomendar_bfs,
};

fn ler_texto(mensagem: &str) -> String {
    print!("{}", mensagem);

    io::stdout().flush().unwrap();

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .unwrap();

    entrada.trim().to_string()
}

fn ler_numero(mensagem: &str) -> Option<u32> {
    let entrada = ler_texto(mensagem);

    entrada.parse::<u32>().ok()
}

fn medir_desempenho(quantidade: u32) {
    let mut grafo: HashMap<u32, Vec<u32>> = HashMap::new();

    for id in 1..quantidade {
        adicionar_conexao(&mut grafo, id, id + 1);
    }

    let inicio = Instant::now();

    let recomendacoes = recomendar_bfs(&grafo, 1);

    let duracao = inicio.elapsed();

    println!(
        "BFS com {} produtos: {} recomendacoes em {:?}",
        quantidade,
        recomendacoes.len(),
        duracao
    );
}

fn main() {
    let mut produtos: HashMap<u32, Produto> = HashMap::new();
    let mut grafo: HashMap<u32, Vec<u32>> = HashMap::new();

    loop {
        println!();
        println!("========= CONECTASTORE =========");
        println!("1 - Cadastrar produto");
        println!("2 - Consultar produto");
        println!("3 - Listar produtos");
        println!("4 - Adicionar conexao entre produtos");
        println!("5 - Gerar recomendacoes");
        println!("6 - Teste de desempenho");
        println!("0 - Sair");

        let opcao = ler_texto("Escolha uma opcao: ");

        match opcao.as_str() {
            "1" => {
                let id = match ler_numero("ID do produto: ") {
                    Some(valor) => valor,
                    None => {
                        println!("ID invalido.");
                        continue;
                    }
                };

                if produtos.contains_key(&id) {
                    println!("Ja existe um produto com esse ID.");
                    continue;
                }

                let nome = ler_texto("Nome: ");
                let categoria = ler_texto("Categoria: ");

                let produto = Produto {
                    id,
                    nome,
                    categoria,
                };

                cadastrar_produto(&mut produtos, produto);

                println!("Produto cadastrado com sucesso.");
            }

            "2" => {
                let id = match ler_numero("ID do produto: ") {
                    Some(valor) => valor,
                    None => {
                        println!("ID invalido.");
                        continue;
                    }
                };

                consultar_produto(&produtos, id);
            }

            "3" => {
                if produtos.is_empty() {
                    println!("Nenhum produto cadastrado.");
                } else {
                    println!();
                    println!("========= PRODUTOS CADASTRADOS =========");
                    println!();

                    let mut ids: Vec<u32> =
                        produtos.keys().copied().collect();

                    ids.sort();

                    for id in ids {
                        if let Some(produto) = produtos.get(&id) {
                            println!(
                                "{} - {} - {}",
                                produto.id,
                                produto.nome,
                                produto.categoria
                                
                            );
                        }
                    }
                    println!();
                    println!("=========================================");
                }
            }

            "4" => {
                let produto_a = match ler_numero("ID do primeiro produto: ") {
                    Some(valor) => valor,
                    None => {
                        println!("ID invalido.");
                        continue;
                    }
                };

                let produto_b = match ler_numero("ID do segundo produto: ") {
                    Some(valor) => valor,
                    None => {
                        println!("ID invalido.");
                        continue;
                    }
                };

                if !produtos.contains_key(&produto_a)
                    || !produtos.contains_key(&produto_b)
                {
                    println!("Um dos produtos nao existe.");
                    continue;
                }

                if produto_a == produto_b {
                    println!("Nao e possivel conectar um produto a ele mesmo.");
                    continue;
                }

                adicionar_conexao(
                    &mut grafo,
                    produto_a,
                    produto_b,
                );

                println!("Conexao adicionada com sucesso.");
            }

            "5" => {
                let id = match ler_numero(
                    "ID do produto para gerar recomendacoes: "
                ) {
                    Some(valor) => valor,
                    None => {
                        println!("ID invalido.");
                        continue;
                    }
                };

                if !produtos.contains_key(&id) {
                    println!("Produto nao encontrado.");
                    continue;
                }

                let recomendacoes = recomendar_bfs(&grafo, id);

                if recomendacoes.is_empty() {
                    println!("Nenhuma recomendacao encontrada.");
                } else {
                    println!("Produtos recomendados:");

                    for id_recomendado in recomendacoes {
                        if let Some(produto) =
                            produtos.get(&id_recomendado)
                        {
                            println!(
                                "- {} ({})",
                                produto.nome,
                                produto.categoria
                            );
                        }
                    }
                }
            }

            "6" => {
                println!("Teste de desempenho:");

                medir_desempenho(100);
                medir_desempenho(1_000);
                medir_desempenho(10_000);
            }

            "0" => {
                println!("Encerrando o ConectaStore.");
                break;
            }

            _ => {
                println!("Opcao invalida.");
            }
        }
    }
}