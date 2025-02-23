# Todo List

Este é um projeto de um gerenciador de tarefas simples, desenvolvido em Rust utilizando Axum e outras bibliotecas auxiliares. O projeto permite adicionar, buscar, atualizar e deletar tarefas.


<div align="center"><img src="readme_assets/00.png" width="400%"></div>

## Funcionalidades

- Adicionar uma nova tarefa
- Listar todas as tarefas
- Buscar uma tarefa por ID
- Atualizar uma tarefa existente
- Deletar uma tarefa

<div align="center"><img src="readme_assets/01.png" width="400%"></div>

## Tecnologias Utilizadas

- **Rust**: Linguagem de programação utilizada para desenvolver o projeto.
- **Axum**: Framework web utilizado para construir a API.
- **SQLx**: Biblioteca para interagir com o banco de dados PostgreSQL.
- **Tera**: Motor de templates utilizado para renderizar as páginas HTML.
- **Tokio**: Runtime assíncrona utilizada para executar o servidor.

<div align="center"><img src="readme_assets/02.png" width="400%"></div>

## Configuração

1. Clone o repositório:
    ```sh
    git clone https://github.com/seu-usuario/todo-list.git
    cd todo-list
    ```

2. Configure o banco de dados PostgreSQL no arquivo [.env](http://_vscodecontentref_/3):
    ```properties
    DATABASE_URL=postgresql://usuario:senha@localhost:5432/nome_do_banco
    ```

3. Instale as dependências e compile o projeto:
    ```sh
    cargo build
    ```

4. Execute o servidor:
    ```sh
    cargo run
    ```

<div align="center"><img src="readme_assets/03.png" width="400%"></div>

## Endpoints

- `GET /todos`: Lista todas as tarefas.
- `POST /todos`: Adiciona uma nova tarefa.
- `GET /todos/{id}`: Busca uma tarefa por ID.
- `PUT /todos/{id}`: Atualiza uma tarefa por ID.
- `PATCH /todos/{id}`: Atualiza parcialmente uma tarefa por ID.
- `DELETE /todos/{id}`: Deleta uma tarefa por ID.