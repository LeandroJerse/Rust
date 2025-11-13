# 🦀 Aprendendo Rust

Este repositório contém meus estudos e códigos em Rust, organizados por tópicos e níveis de dificuldade.

## 📚 Índice

- [Comandos Úteis](#-comandos-úteis)
- [Conceitos Estudados](#-conceitos-estudados)
  - [1. Hello World](#1-hello-world)
  - [2. Jogo de Adivinhação](#2-jogo-de-adivinhação-guessing-game)
- [Projetos](#-projetos)
- [Recursos de Estudo](#-recursos-de-estudo)
- [Notas e Observações](#-notas-e-observações)

---

## 🛠️ Comandos Úteis

### Compilação e Execução
```bash
# Compilar arquivo .rs diretamente
rustc main.rs
rustc main.rs -O              # Com otimizações

# Verificar se o código compila (sem gerar binário)
cargo check

# Compilar projeto (gera binário em target/debug/)
cargo build

# Compilar projeto otimizado para produção (target/release/)
cargo build --release

# Compilar e executar
cargo run
cargo run --release           # Executar versão otimizada
```

### Gerenciamento de Projetos
```bash
# Criar novo projeto binário
cargo new nome_do_projeto

# Criar novo projeto como biblioteca
cargo new nome_do_projeto --lib

# Inicializar projeto na pasta atual
cargo init

# Atualizar dependências
cargo update

# Limpar arquivos de build (target/)
cargo clean
```

### Documentação e Testes
```bash
# Gerar documentação
cargo doc

# Gerar e abrir documentação no navegador
cargo doc --open

# Executar testes
cargo test

# Formatar código (requer rustfmt)
cargo fmt

# Verificar código com clippy (linter)
cargo clippy
```

### Gerenciamento de Dependências
```bash
# Adicionar dependência (requer cargo-edit)
cargo add nome_crate

# Buscar crates no crates.io
cargo search termo

# Visualizar árvore de dependências
cargo tree
```

### Informações do Sistema
```bash
# Ver versão do Cargo
cargo --version

# Ver versão do compilador Rust
rustc --version

# Listar todos os comandos disponíveis
cargo --list
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

### 2. Jogo de Adivinhação (Guessing Game)
**Data:** Novembro 2025  
**Diretório:** `guessing_game/`

#### O que aprendi:
- **Variáveis mutáveis** - Uso de `let mut` para criar variáveis que podem mudar
- **Input do usuário** - Leitura de dados com `std::io::stdin()`
- **Dependências externas** - Adicionar e usar crates (biblioteca `rand`)
- **Loops infinitos** - Uso de `loop` para repetir até uma condição
- **Pattern Matching** - Uso de `match` para comparar valores e tratar casos
- **Error Handling** - Tratamento de erros na conversão de String para número
- **Shadowing** - Reutilizar nome de variável com tipo diferente
- **Conversão de tipos** - Parse de String para u32
- **Comparações** - Uso do enum `Ordering` (Less, Equal, Greater)

#### Códigos:
- **`guessing_game/src/main.rs`** - Jogo completo de adivinhação

#### Conceitos-chave:
```rust
use std::io;                    // Importar biblioteca
use rand::Rng;                  // Importar trait

let mut guess = String::new();  // Variável mutável
io::stdin().read_line(&mut guess) // Referência mutável
    .expect("Failed");          // Tratamento de erro básico

let guess: u32 = guess.trim().parse() // Shadowing + conversão
    .expect("Please type a number!");

match guess.cmp(&secret_number) {  // Pattern matching
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;                     // Sair do loop
    }
}
```

#### Dependências utilizadas:
- `rand = "0.9.2"` - Geração de números aleatórios

---

## 🚀 Projetos

| Nome | Descrição | Conceitos Aplicados | Status |
|------|-----------|---------------------|---------|
| **Guessing Game** | Jogo de adivinhação de números | Variáveis mutáveis, Input, Loops, Pattern Matching, Crates externos | ✅ Completo |

---

## 📚 Recursos de Estudo

### 📖 Livro Principal
**The Rust Programming Language**  
*Steve Klabnik and Carol Nichols*  
Rust 2018 Edition  
[Versão online gratuita](https://doc.rust-lang.org/book/)

Este é o livro oficial de Rust, também conhecido como "The Book". Cobrindo desde conceitos básicos até tópicos avançados de forma didática e prática.

---

### Livros e Documentação Complementares
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Aprenda com exemplos práticos
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Tópicos avançados e unsafe Rust
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) - Receitas para problemas comuns

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
- [x] Variáveis e Mutabilidade
- [x] Tipos de Dados (básico: String, u32)
- [ ] Funções
- [x] Controle de Fluxo (if, loop, while, for) - *básico: loop e match*
- [ ] Ownership (Posse)
- [x] Borrowing e References - *básico: referências mutáveis*
- [ ] Slices
- [ ] Structs
- [x] Enums e Pattern Matching - *básico: Ordering e match*
- [x] Gerenciamento de Erros - *básico: expect() e Result*
- [ ] Generics
- [x] Traits - *básico: uso de traits (Rng)*
- [ ] Lifetimes
- [ ] Testes
- [ ] Closures
- [ ] Iterators
- [ ] Smart Pointers
- [ ] Concorrência
- [ ] Async/Await

---

## 🎯 Próximos Passos

- [x] ~~Variáveis e tipos de dados~~ ✅
- [x] ~~Criar primeiro projeto prático~~ ✅
- [ ] Aprofundar em Ownership e Borrowing
- [ ] Structs e métodos
- [ ] Aprofundar em Enums e pattern matching
- [ ] Tratamento de erros com Result e Option
- [ ] Coleções (Vec, HashMap, etc)

---

## 📊 Progresso

- **Início dos estudos:** Novembro 2025
- **Conceitos dominados:** 2 (Hello World, Guessing Game)
- **Projetos completos:** 1 (Guessing Game)
- **Crates utilizadas:** 1 (rand)
- **Conceitos parcialmente aprendidos:** 6 (variáveis, tipos, loops, pattern matching, borrowing, traits)

---

**Última atualização:** 13/11/2025

