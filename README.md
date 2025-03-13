# Implementações do Padrão Singleton

Este repositório contém implementações do padrão de design Singleton em várias linguagens de programação (Kotlin, Rust e TypeScript). O objetivo é explorar diferentes abordagens de implementação deste padrão em cada linguagem, com foco em casos de uso comuns como gerenciadores de conexão de banco de dados, sistemas de logging e outras situações onde uma única instância compartilhada é necessária.

## Implementações

### Kotlin

O diretório `kotlin/` contém implementações do padrão Singleton utilizando recursos nativos da linguagem Kotlin:

- **Database Connection** (`kotlin/db/`): Implementação de um gerenciador de conexão de banco de dados que mantém uma única conexão compartilhada.
- **Logger** (`kotlin/utils/`): Sistema de log com uma única instância compartilhada por toda a aplicação.
- **Lazy Initialization** (`kotlin/lazy/`): Exemplos de diferentes abordagens de inicialização preguiçosa (lazy initialization) utilizando recursos do Kotlin como `lazy` e `by lazy`.

### Rust

O diretório `rust/` contém implementações do padrão Singleton adaptadas às particularidades do Rust:

- **Database Connection** (`rust/database/`): Implementação de um gerenciador de conexão utilizando `once_cell` ou `lazy_static`.
- **Logger** (`rust/logger/`): Sistema de log thread-safe com uma única instância.
- **Lazy Initialization** (`rust/lazy-init/`): Exemplos de inicialização preguiçosa considerando o sistema de ownership do Rust.

### TypeScript

O diretório `typescript/` contém implementações do padrão Singleton em TypeScript:

- **Database Connection** (`typescript/database/`): Pool de conexões de banco de dados como Singleton.
- **Logger** (`typescript/logger/`): Sistema de log com diferentes níveis e uma única instância compartilhada.
- **Lazy Initialization** (`typescript/lazy-init/`): Exemplos de inicialização tardia e avaliação preguiçosa em TypeScript.

## Conceitos Explorados

- Inicialização preguiçosa (Lazy Initialization)
- Thread-safety em implementações de Singleton
- Diferentes abordagens de implementação específicas de cada linguagem
- Casos de uso práticos para o padrão Singleton
- Vantagens e desvantagens de cada abordagem

## Como Executar os Exemplos

Cada diretório de linguagem contém instruções específicas sobre como compilar e executar os exemplos.

### Kotlin

```bash
cd kotlin
./gradlew run
```

ou 
```bash
kotlinc Main.kt -d myapp.jar
kotlin -classpath myapp.jar MainKt
```

### Rust

```bash
cd rust
cargo run --example database
cargo run --example logger
cargo run --example lazy_init
```

### TypeScript

```bash
cd typescript
npm install
npm run start:database
npm run start:logger
npm run start:lazy
```

## Contribuições

Sinta-se à vontade para contribuir com implementações adicionais, melhorias ou correções. Abra um Pull Request ou uma Issue para discutir novas ideias.

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo LICENSE para mais detalhes.
