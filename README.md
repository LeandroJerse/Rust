# 🦀 Aprendendo Rust

Este repositório contém meus estudos e códigos em Rust, organizados por tópicos e níveis de dificuldade.

## 📚 Índice

- [Comandos Úteis](#-comandos-úteis)
- [Conceitos Estudados](#-conceitos-estudados)
  - [1. Hello World](#1-hello-world)
- [Projetos](#-projetos)
- [Recursos de Estudo](#-recursos-de-estudo)
- [Notas e Observações](#-notas-e-observações)

---

## 🛠️ Comandos Úteis

### Compilação e Execução
```bash
# Compilar arquivo .rs diretamente
rustc main.rs

# Verificar se o código compila (sem gerar binário)
cargo check

# Compilar projeto (gera binário em target/debug/)
cargo build

# Compilar projeto otimizado para produção
cargo build --release

# Compilar e executar
cargo run
```

### Gerenciamento de Projetos
```bash
# Criar novo projeto
cargo new nome_do_projeto

# Criar novo projeto como biblioteca
cargo new nome_do_projeto --lib

# Atualizar dependências
cargo update

# Limpar arquivos de build
cargo clean
```

### Documentação e Testes
```bash
# Gerar e abrir documentação
cargo doc --open

# Executar testes
cargo test

# Formatar código
cargo fmt

# Verificar código com clippy (linter)
cargo clippy
```

---

## 📖 Conceitos Estudados

### 1. Hello World
**Data:** Início dos estudos  
**Diretório:** `hello_world/`

#### O que aprendi:
- Estrutura básica de um programa Rust
- Função `main()` como ponto de entrada
- Macro `println!()` para imprimir no console
- Diferença entre compilar com `rustc` e usar Cargo
- Estrutura de um projeto Cargo (`Cargo.toml`, `src/`)

#### Códigos:
- **`hello_world/main.rs`** - Hello World básico compilado com `rustc`
- **`hello_world/hello_cargo/`** - Hello World usando Cargo

#### Conceitos-chave:
- `fn main()` - Função principal
- `println!()` - Macro para impressão (note o `!`)
- Cargo como gerenciador de pacotes e build system

---

## 🚀 Projetos

_Seção a ser preenchida conforme novos projetos forem desenvolvidos_

| Nome | Descrição | Conceitos Aplicados | Status |
|------|-----------|---------------------|---------|
| - | - | - | - |

---

## 📚 Recursos de Estudo

### Livros e Documentação
- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Tópicos avançados
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)

### Exercícios e Prática
- [Rustlings](https://github.com/rust-lang/rustlings) - Exercícios interativos
- [Exercism - Rust Track](https://exercism.org/tracks/rust)
- [Advent of Code](https://adventofcode.com/) - Desafios de programação

### Comunidade
- [Rust Users Forum](https://users.rust-lang.org/)
- [r/rust (Reddit)](https://www.reddit.com/r/rust/)
- [Rust Brasil (Telegram)](https://t.me/rustlangbr)

---

## 📝 Notas e Observações

### Convenções de Nomenclatura
- **Arquivos e diretórios:** `snake_case`
- **Variáveis e funções:** `snake_case`
- **Structs, Enums e Traits:** `PascalCase`
- **Constantes:** `SCREAMING_SNAKE_CASE`

### Sintaxe Importante
- `;` - Statements terminam com ponto e vírgula
- `!` - Indica uma macro (ex: `println!`, `vec!`)
- `//` - Comentário de linha
- `/* */` - Comentário de bloco
- `///` - Comentário de documentação

### Conceitos a Estudar
- [ ] Variáveis e Mutabilidade
- [ ] Tipos de Dados
- [ ] Funções
- [ ] Controle de Fluxo (if, loop, while, for)
- [ ] Ownership (Posse)
- [ ] Borrowing e References
- [ ] Slices
- [ ] Structs
- [ ] Enums e Pattern Matching
- [ ] Gerenciamento de Erros
- [ ] Generics
- [ ] Traits
- [ ] Lifetimes
- [ ] Testes
- [ ] Closures
- [ ] Iterators
- [ ] Smart Pointers
- [ ] Concorrência
- [ ] Async/Await

---

## 🎯 Próximos Passos

- [ ] Variáveis e tipos de dados
- [ ] Sistema de ownership
- [ ] Structs e métodos
- [ ] Enums e pattern matching
- [ ] Criar primeiro projeto prático

---

## 📊 Progresso

- **Início dos estudos:** Novembro 2025
- **Conceitos dominados:** 1
- **Projetos completos:** 0
- **Exercícios resolvidos:** 0

---

**Última atualização:** 08/11/2025

