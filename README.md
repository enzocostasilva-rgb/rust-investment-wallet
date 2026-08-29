🦀 Rust Investment Wallet

Aplicação Fullstack de uma Carteira de Investimentos, desenvolvida em Rust como projeto de conclusão do Santander Bootcamp de Rust AI Developer.

O projeto permite cadastrar, consultar e atualizar ativos de investimento por meio de uma API REST, utilizando PostgreSQL, autenticação e interface web.

📋 Sobre o projeto

A aplicação foi desenvolvida a partir do projeto-base disponibilizado pela Digital Innovation One (DIO).

Durante o desafio, o projeto foi evoluído para representar os investimentos com informações mais completas.

Cada ativo possui:

Nome do ativo;
Ticker;
Tipo do ativo;
Quantidade;
Valor unitário.
🚀 Melhorias implementadas
Cadastro completo de investimentos

Os ativos passaram a armazenar:

Campo	Descrição
name	Nome do ativo
ticker	Código do ativo, como BTC ou ETH
asset_type	Tipo do investimento
quantity	Quantidade de unidades
unit_value	Valor de cada unidade
Validação dos dados

Foram adicionadas validações para impedir dados inválidos, incluindo:

Nome do ativo vazio;
Ticker vazio;
Ticker com mais de 20 caracteres;
Tipo de ativo vazio;
Tipo de ativo com mais de 50 caracteres;
Quantidade menor ou igual a zero;
Valor unitário menor ou igual a zero;
Valores numéricos inválidos.

Também foi implementada a normalização dos dados, como remoção de espaços desnecessários e conversão do ticker e tipo do ativo para letras maiúsculas.

Testes automatizados

Foram adicionados testes para validar:

Criação de ativos;
Listagem de ativos;
Atualização de ativos;
Rejeição de quantidade inválida;
Rejeição de valor unitário inválido;
Normalização do ticker e do tipo do ativo.
Banco de dados

Foi criada uma migration para adicionar os novos campos à tabela assets.

🛠️ Tecnologias utilizadas
Rust
Axum
SQLx
PostgreSQL
Askama
JWT
Cookies
Serde
Docker
GitHub Actions
🔐 Autenticação

A aplicação utiliza autenticação baseada em usuário, senha, cookies e JWT.

As senhas são armazenadas utilizando hash, enquanto o token JWT é utilizado para manter o usuário autenticado durante a navegação.

📡 API
Listar ativos
GET /assets

Criar ativo
POST /assets


Exemplo:

{
  "name": "Bitcoin",
  "ticker": "BTC",
  "asset_type": "CRYPTO",
  "quantity": 0.5,
  "unit_value": 100000.0
}

Atualizar ativo
PATCH /assets


Exemplo:

{
  "id": 1,
  "name": "Ethereum",
  "ticker": "ETH",
  "asset_type": "CRYPTO",
  "quantity": 2.0,
  "unit_value": 20000.0
}

🗄️ Banco de dados

O projeto utiliza PostgreSQL.

As alterações da estrutura do banco são controladas por migrations do SQLx.

A tabela assets possui os seguintes campos principais:

Campo	Descrição
id	Identificador do ativo
name	Nome do investimento
ticker	Código do ativo
asset_type	Tipo do investimento
quantity	Quantidade de unidades
unit_value	Valor unitário
🧪 Testes

O projeto possui testes automatizados para as principais operações da aplicação.

Para executar os testes localmente:

cargo test


A integração contínua é realizada pelo GitHub Actions, que realiza a compilação e validação automatizada do projeto.

Status da validação

O workflow do GitHub Actions foi executado com sucesso após as alterações realizadas no projeto.

⚙️ Como executar

Para executar o projeto localmente, é necessário possuir:

Rust;
Cargo;
Docker;
PostgreSQL.

Clone este repositório:

git clone https://github.com/enzocostasilva-rgb/rust-investment-wallet.git
cd rust-investment-wallet


Inicie os serviços necessários utilizando Docker conforme a configuração do projeto.

Depois execute:

cargo run


Para executar os testes:

cargo test


Durante o desenvolvimento deste desafio, as alterações foram realizadas diretamente pelo GitHub devido à utilização de um computador público, sem acesso ao ambiente local de desenvolvimento. A compilação e validação do projeto foram acompanhadas por meio do GitHub Actions.

📚 O que aprendi

Durante o desenvolvimento deste projeto, pratiquei conceitos importantes de desenvolvimento Fullstack com Rust:

Organização de uma aplicação utilizando Axum;
Criação de rotas e handlers;
Integração com PostgreSQL;
Utilização do SQLx;
Criação e gerenciamento de migrations;
Autenticação utilizando JWT;
Utilização de cookies;
Validação de dados recebidos pela API;
Criação de testes automatizados;
Utilização do Git e GitHub;
Utilização do GitHub Actions para integração contínua.

Também aprendi a evoluir um projeto existente de forma incremental, realizando alterações no modelo de dados, API, banco de dados e testes, verificando continuamente se o projeto permanecia funcional.

🎯 Objetivo do desafio

Este projeto foi desenvolvido como parte do desafio de conclusão do Santander Bootcamp de Rust AI Developer.

O projeto-base disponibilizado pela Digital Innovation One foi utilizado como referência para o desenvolvimento.

A principal evolução realizada foi transformar o cadastro básico de ativos em uma estrutura mais completa de investimentos, adicionando:

Ticker;
Tipo de ativo;
Quantidade;
Validações;
Testes automatizados;
Migration para atualização do banco de dados.
👨‍💻 Autor

Enzo Costa Silva

Projeto desenvolvido para fins educacionais como parte do Santander Bootcamp de Rust AI Developer.
