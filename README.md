# Controle de Ativos Financeiros - API em Rust

Olá! Este é o repositório do meu projeto final. 

Meu objetivo aqui foi construir um backend seguro e rápido para gerenciar uma carteira de investimentos, colocando em prática o que aprendi sobre o ecossistema da linguagem Rust.

## Tecnologias e Ferramentas que utilizei
- **Rust** como linguagem principal (focando em segurança de memória).
- **Axum** para gerenciar as rotas da web.
- **SQLx + PostgreSQL** para armazenar os dados dos investimentos.
- **Askama** para conectar as lógicas do Rust direto no HTML.
- **JSON Web Tokens (JWT)** para criar a camada de autenticação do usuário.

## O que a aplicação faz?
Eu criei uma arquitetura onde o usuário pode se autenticar (gerando um cookie de sessão seguro), adicionar novos ativos na sua carteira (como ações ou fundos imobiliários) e visualizar um painel que calcula automaticamente o patrimônio total investido buscando os dados direto do banco relacional.
