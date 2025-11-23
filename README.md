# 🦀 Aprendendo Rust

Este repositório contém meus estudos e códigos em Rust, organizados por tópicos e níveis de dificuldade.

## 📚 Índice

- [Comandos Úteis](#-comandos-úteis)
- [Conceitos Estudados](#-conceitos-estudados)
  - [1. Hello World](#1-hello-world)
  - [2. Jogo de Adivinhação](#2-jogo-de-adivinhação-guessing-game)
  - [3. Variáveis e Mutabilidade](#3-variáveis-e-mutabilidade)
  - [4. Controle de Fluxo - Condicionais](#4-controle-de-fluxo---condicionais)
  - [5. Controle de Fluxo - Loops](#5-controle-de-fluxo---loops)
  - [6. Structs e Métodos](#6-structs-e-métodos)
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

### 3. Variáveis e Mutabilidade
**Data:** Novembro 2025  
**Diretório:** `variables/`

#### O que aprendi:
- **Variáveis imutáveis por padrão** - Variáveis em Rust são imutáveis por padrão
- **Variáveis mutáveis** - Uso de `let mut` para permitir mudanças
- **Interpolação de strings** - Uso de `{}` no `println!()` para formatar valores

#### Códigos:
- **`variables/src/main.rs`** - Exemplo de variável mutável

#### Conceitos-chave:
```rust
let mut x = 5;              // Variável mutável
println!("the value of x is {}", x);  // Interpolação
x = 6;                       // Modificar valor
```

---

### 4. Controle de Fluxo - Condicionais
**Data:** Novembro 2025  
**Diretório:** `branches/`

#### O que aprendi:
- **Estruturas condicionais** - Uso de `if` e `else`
- **Expressões booleanas** - Comparações retornam valores booleanos
- **Blocos condicionais** - Sintaxe de `if/else` em Rust

#### Códigos:
- **`branches/src/main.rs`** - Exemplo de condicionais

#### Conceitos-chave:
```rust
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

---

### 5. Controle de Fluxo - Loops
**Data:** Novembro 2025  
**Diretório:** `loops/`

#### O que aprendi:
- **Loop infinito** - Uso de `loop` para loops infinitos
- **Retornar valores de loops** - Usar `break` com valor para retornar
- **Loops while** - Uso de `while` para loops condicionais
- **Loops for** - Iterar sobre coleções e ranges
- **Ranges** - Uso de `(0..4)` e `.rev()` para iterar em ordem reversa
- **Método iter()** - Iterar sobre arrays/coleções

#### Códigos:
- **`loops/src/main.rs`** - Exemplos de todos os tipos de loops

#### Conceitos-chave:
```rust
// Loop infinito com retorno
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // Retorna valor
    }
};

// Loop while
while number != 0 {
    println!("{}", number);
    number -= 1;
}

// Loop for sobre coleção
for element in collection.iter() {
    println!("{}", element);
}

// Loop for sobre range reverso
for number in (0..4).rev() {
    println!("{}", number);
}
```

---

### 6. Structs e Métodos
**Data:** Novembro 2025  
**Diretório:** `rectangle/`

#### O que aprendi:
- **Definir structs** - Criar tipos customizados com `struct`
- **Campos de struct** - Definir propriedades de uma struct
- **Implementação de métodos** - Uso de `impl` para adicionar métodos
- **Métodos associados** - Funções que retornam instâncias (construtores)
- **Self e &self** - Referências para a própria instância
- **Métodos mutáveis** - Uso de `&mut self` para modificar a struct
- **Métodos com múltiplos parâmetros** - Métodos que recebem referências a outras instâncias
- **Comparações entre structs** - Comparar propriedades de diferentes instâncias
- **Derive Debug** - Atributo `#[derive(Debug)]` para imprimir structs
- **Field init shorthand** - Sintaxe curta quando nome do campo = variável
- **Formatação Debug** - Uso de `{:?}` para imprimir structs

#### Códigos:
- **`rectangle/src/main.rs`** - Struct Rectangle com métodos

#### Conceitos-chave:
```rust
#[derive(Debug)]  // Permite imprimir com {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Método associado (construtor) - chamado com ::
    fn new_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }  // Field init shorthand
    }
    
    // Método de instância (empresta self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Método mutável (empresta mutável)
    fn grow_width(&mut self) {
        self.width += 1;
    }
    
    // Método com múltiplos parâmetros (compara com outra instância)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Uso de método associado (não precisa de instância)
let mut rect = Rectangle::new_rectangle(30, 50);
println!("Area: {}", rect.area());
println!("Rectangle: {:?}", rect);  // Debug formatting

let rect1 = Rectangle::new_rectangle(30, 50);
let rect2 = Rectangle::new_rectangle(10, 40);
let rect3 = Rectangle::new_rectangle(60, 45);
println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
```

---

## 🚀 Projetos

| Nome | Descrição | Conceitos Aplicados | Status |
|------|-----------|---------------------|---------|
| **Guessing Game** | Jogo de adivinhação de números | Variáveis mutáveis, Input, Loops, Pattern Matching, Crates externos | ✅ Completo |
| **Variables** | Estudo de variáveis mutáveis | Variáveis, Mutabilidade, Interpolação | ✅ Completo |
| **Branches** | Estruturas condicionais | if/else, Expressões booleanas | ✅ Completo |
| **Loops** | Todos os tipos de loops | loop, while, for, ranges, iterators | ✅ Completo |
| **Rectangle** | Struct com métodos | Structs, impl, métodos, Debug trait | ✅ Completo |

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
- [x] Variáveis e Mutabilidade ✅
- [x] Tipos de Dados (básico: String, u32, u32) ✅
- [x] Funções (básico: definição e chamada) ✅
- [x] Controle de Fluxo (if, loop, while, for) ✅
- [ ] Ownership (Posse) - *próximo tópico*
- [x] Borrowing e References - *básico: &self, &mut self*
- [ ] Slices
- [x] Structs ✅
- [x] Enums e Pattern Matching - *básico: Ordering e match*
- [x] Gerenciamento de Erros - *básico: expect() e Result*
- [ ] Generics
- [x] Traits - *básico: Debug, Rng*
- [ ] Lifetimes
- [ ] Testes
- [ ] Closures
- [x] Iterators - *básico: .iter(), ranges*
- [ ] Smart Pointers
- [ ] Concorrência
- [ ] Async/Await

---

## 🎯 Próximos Passos

- [x] ~~Variáveis e tipos de dados~~ ✅
- [x] ~~Criar primeiro projeto prático~~ ✅
- [x] ~~Structs e métodos~~ ✅
- [x] ~~Controle de fluxo completo (if, loops)~~ ✅
- [ ] **Ownership e Borrowing** - *Próximo tópico principal*
- [ ] Enums mais complexos e pattern matching avançado
- [ ] Tratamento de erros com Result e Option
- [ ] Coleções (Vec, HashMap, etc)
- [ ] Módulos e organização de código

---

## 📊 Progresso

- **Início dos estudos:** Novembro 2025
- **Conceitos dominados:** 6 (Hello World, Guessing Game, Variables, Branches, Loops, Structs)
- **Projetos completos:** 5 (Guessing Game, Variables, Branches, Loops, Rectangle)
- **Crates utilizadas:** 1 (rand)
- **Conceitos fundamentais aprendidos:** 
  - ✅ Variáveis e mutabilidade
  - ✅ Controle de fluxo (if, loop, while, for)
  - ✅ Structs e métodos
  - ✅ Borrowing básico (&self, &mut self)
  - ✅ Traits básicas (Debug, Rng)
  - ✅ Pattern matching básico

---

**Última atualização:** 13/11/2025

