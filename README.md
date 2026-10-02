# ConectaStore

O ConectaStore é um sistema de recomendação de produtos desenvolvido em Rust. O projeto utiliza grafos para representar relações de similaridade entre produtos e o algoritmo BFS para percorrer essas relações e encontrar produtos relacionados.

## Objetivo

O objetivo do projeto é representar produtos e suas relações através de um grafo, permitindo gerar recomendações a partir de um produto escolhido pelo usuário.

No sistema, cada produto representa um vértice do grafo e cada conexão representa uma relação de similaridade entre dois produtos.

## Funcionalidades

O sistema possui um menu no terminal com as seguintes opções:

1. Cadastrar produto
2. Consultar produto
3. Listar produtos
4. Adicionar conexão entre produtos
5. Gerar recomendações
6. Executar teste de desempenho
0. Sair

Também são realizadas validações para evitar produtos com IDs repetidos, conexões de um produto com ele mesmo e conexões duplicadas.

## Estruturas de dados utilizadas

### HashMap

O HashMap é utilizado para armazenar os produtos utilizando o ID como chave.

Exemplo:

```text
1 -> Notebook
2 -> Mouse
3 -> Teclado
```

Também é utilizado na representação do grafo.

### Vec

Cada produto possui uma lista com os IDs dos produtos relacionados a ele.

Exemplo:

```text
1 -> [2, 3]
2 -> [1, 4]
3 -> [1, 5]
```

Essa estrutura representa uma lista de adjacência.

### HashSet

O HashSet é utilizado durante a execução do BFS para armazenar os vértices que já foram visitados.

Isso impede que um produto seja processado várias vezes e evita recomendações duplicadas.

### VecDeque

O VecDeque é utilizado como fila durante a execução do BFS.

O primeiro produto inserido na fila é também o primeiro a ser processado.

## Representação do grafo

O sistema utiliza um grafo não direcionado e não ponderado.

Os vértices representam produtos e as arestas representam relações de similaridade.

Exemplo:

```text
        Notebook
        /      \
     Mouse    Teclado
       |         |
   Mousepad    Headset
```

Como o grafo é não direcionado, quando o produto A é conectado ao produto B, o produto B também é conectado ao produto A.

Não foram utilizados pesos nas arestas. Todas as relações de similaridade possuem a mesma importância dentro desta implementação.

## Algoritmo de recomendação

O algoritmo utilizado para percorrer o grafo é o BFS (Busca em Largura).

O BFS começa no produto escolhido pelo usuário e visita primeiro os produtos diretamente conectados a ele. Depois continua percorrendo as próximas conexões.

Um HashSet é utilizado para registrar os produtos já visitados e impedir que o algoritmo processe o mesmo produto mais de uma vez.

## Como executar

É necessário ter Rust e Cargo instalados.

Dentro da pasta do projeto, execute:

```bash
cargo run
```

Para executar a versão otimizada:

```bash
cargo run --release
```

## Como executar os testes

Execute:

```bash
cargo test
```

Atualmente o projeto possui:

- 3 testes unitários;
- 1 teste de integração.

Os testes verificam a criação de conexões, a prevenção de conexões duplicadas, o funcionamento do BFS sem recomendações repetidas e o fluxo completo de cadastro e recomendação.

## Exemplo de uso

Um exemplo simples é cadastrar os produtos:

```text
1 - Notebook
2 - Mouse
3 - Teclado
4 - Mousepad
5 - Headset
```

Depois podem ser criadas as conexões:

```text
1 <-> 2
1 <-> 3
2 <-> 4
3 <-> 5
```

Ao solicitar recomendações a partir do produto 1, o sistema percorre o grafo e apresenta:

```text
Mouse
Teclado
Mousepad
Headset
```

## Arquitetura do projeto

```text
conectastore/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── integracao.rs
├── Cargo.toml
└── README.md
```

### main.rs

Contém o menu e a interação do usuário com o sistema.

### lib.rs

Contém as estruturas e funções principais do projeto, como cadastro de produtos, consulta, criação de conexões e BFS.

### tests/integracao.rs

Contém o teste de integração do sistema.

## Testes de desempenho

Os testes foram executados utilizando:

```bash
cargo run --release
```

Resultados obtidos:

| Quantidade de produtos | Produtos encontrados | Tempo |
|---|---:|---:|
| 100 | 99 | 11,3 µs |
| 1.000 | 999 | 74,8 µs |
| 10.000 | 9.999 | 664,7 µs |

Os valores podem variar de acordo com o computador e com cada execução.

## Vídeo pitch

https://youtu.be/pMNb4JSHDGA