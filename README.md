🦀 Rust Investment Wallet

Aplicação Fullstack de uma Carteira de Investimentos, desenvolvida em Rust como projeto de conclusão do Santander Bootcamp de Rust AI Developer.

O projeto permite cadastrar, consultar e atualizar ativos de investimento, utilizando uma API REST, banco de dados PostgreSQL, autenticação e uma interface web.

📋 Sobre o projeto

A aplicação foi desenvolvida a partir do projeto-base da Digital Innovation One:

rust-fullstack-carteira-investimentos

Durante o desafio, o projeto foi evoluído para representar os investimentos com informações mais completas.

Cada ativo pode possuir:

Nome do ativo;
Ticker;
Tipo do ativo;
Quantidade;
Valor unitário.
🚀 Melhorias implementadas

Além da estrutura original do projeto, foram implementadas as seguintes melhorias:

Cadastro completo de investimentos

Os ativos passaram a armazenar:

name — nome do ativo;
ticker — código do ativo, como BTC ou ETH;
asset_type — tipo do investimento;
quantity — quantidade de unidades;
unit_value — valor de cada unidade.
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

Também foi implementada a normalização dos dados, como remoção de espaços desnecessários e conversão do ticker para letras maiúsculas.

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

As senhas são armazenadas utilizando hash, e o token de autenticação é utilizado para manter o usuário autenticado durante a navegação.

📡 API

A API possui operações relacionadas aos ativos:

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

As alterações de estrutura são controladas por migrations do SQLx.

A tabela assets possui os principais campos:

Campo	Descrição
id	Identificador do ativo
name	Nome do investimento
ticker	Código do ativo
asset_type	Tipo do investimento
quantity	Quantidade de unidades
unit_value	Valor unitário
🧪 Testes

O projeto possui testes automatizados para as principais operações da API.

Os testes podem ser executados utilizando:

cargo test


A integração contínua também é realizada pelo GitHub Actions, garantindo que o projeto seja compilado e validado automaticamente.

⚙️ Como executar

É necessário possuir:

Rust;
Cargo;
Docker;
PostgreSQL.

Clone o repositório:

git clone (https://github.com/digitalinnovationone/rust-fullstack-carteira-investimentos.git)


Inicie os serviços necessários utilizando Docker conforme a configuração do projeto.

Depois execute:

cargo run


Para executar os testes:

cargo test


Como o desenvolvimento deste desafio foi realizado em um ambiente público sem acesso local ao Rust e ao Docker, a validação principal da compilação e dos testes foi realizada por meio do GitHub Actions.

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

Também aprendi a evoluir um projeto existente de forma incremental, mantendo os testes e verificando as alterações por meio do CI.

🎯 Objetivo do desafio

Este projeto foi desenvolvido como parte do desafio de conclusão do Santander Bootcamp de Rust AI Developer, utilizando o projeto-base disponibilizado pela Digital Innovation One como referência.

A principal evolução realizada foi transformar o cadastro básico de ativos em uma estrutura mais completa de investimentos, adicionando ticker, tipo, quantidade, validações e testes.

👨‍💻 Autor

Enzo Costa Silva

Projeto desenvolvido para fins educacionais como parte do Santander Bootcamp de Rust AI Developer.
