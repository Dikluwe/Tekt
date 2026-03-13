# 💎 Guia de Refatoração Tekt: Tornando o Tree-sitter Bilíngue

Se você vai abrir o "motor" do Tree-sitter para injetar a bilinguidade (Backend Rust), não pode tratar o código legado em C/JS como código comum. Ele é a **Especificação Viva**. A refatoração cristalina aqui serve para garantir que o novo backend Rust seja bit-a-bit compatível com o legado em C, sem herdar sua fragilidade.

## 🏗 Passo 1: O Container de Quarentena

O Tree-sitter CLI é um projeto Rust denso. Sua primeira missão é isolar o `tree-sitter-cli/src/generate` (onde reside o "mal" da geração monolítica).

Rode o script de infraestrutura e prepare a arena:

```bash
# L20 agora aponta para o repositório oficial clonado
init-tekt.sh --target tree-sitter-official
```

---

## 🗺️ Passo 1.5: O Mapa da Máquina de Estados (`LEGACY_MAP.md`)

O Tree-sitter tem arquivos de renderização massivos. Precisamos saber o que já foi "bilíngualizado".

**Prompt de Mapeamento Tree-sitter:**

> "Mapeie os arquivos de geração do `tree-sitter-cli/src/generate/`.
> Crie o `LEGACY_MAP.md` focando especialmente em:
> * `render.rs` (O coração da emissão de C)
> * `tables.rs` (Onde as máquinas de estado são calculadas)
> Marque cada arquivo com a checkbox de status:
> `  - *↳ [ ] L0 (IR de Bilinguidade Criada)*`"

---

## 🔍 Passo 2: Engenharia Reversa da IR (Comando `/gerar-spec`)

Aqui está o segredo: para ser bilíngue, o Tree-sitter não pode pular de "Gramática" para "C". Ele precisa de uma **Representação Intermediária (IR)**.

**Prompt de Engenharia Reversa Tekt (Foco em Compiladores):**

> "Analise o código de geração em `L20/tree-sitter-cli/src/generate/render.rs`.
> Sua missão é extrair a **Semente (L0)**: a Representação Intermediária (IR) da Máquina de Estados.
> Crie `00_nucleo/specs/bilingual_ir.md` contendo:
> 1. **Atalhos de Memória:** Como as tabelas de símbolos são lidas?
> 2. **Atomização do Emitter:** Separe a *Lógica de Decisão* (quem vai para qual estado) da *Lógica de Sintaxe* (como escrever isso em C ou Rust).
> 3. **Contratos de Backend:** Defina a Interface (Trait) que ambos os Backends (C e Rust) deverão implementar.
> 
> Pare no L0. Não escreva uma linha de `parser.rs` ainda."

---

## 💎 Passo 3: A Clivagem (O Nascimento do Backend Rust)

Agora, você vai extrair o algoritmo de cálculo de estados do `render.rs` e colocá-lo no `01_core`, tornando-o agnóstico de linguagem.

**Prompt Executivo de Clivagem:**

> "Com a IR definida no `L0`, vamos clivar o `render.rs`.
> 1. **No `01_core/` (L1):** Implemente o `StateTableFlattener`. Ele deve pegar a AST da gramática e gerar uma lista de transições pura, sem saber o que é C ou Rust.
> 2. **No `03_infra/` (L3):** 
>    * Crie o `C_Emitter` (recristalizando o código legado).
>    * Crie o `Rust_Emitter` (o novo backend que gera o `parser.rs`).
> 
> 3. **Regra de Ouro:** O `Rust_Emitter` deve usar `include_bytes!` ou `static [u16]` para evitar o gargalo do `rustc` em tabelas grandes.
> Use o `L20` para garantir que o `Rust_Emitter` produza a mesma lógica que o `C_Emitter` produzia originalmente."

---

## 🔗 Passo 4: O Wiring Bilíngue (Comando `/integrar-legado`)

Este é o momento onde o CLI do Tree-sitter ganha a nova flag `--lang rust`.

**Prompt de Integração (Wiring Parcial):**

> "Vá para `04_wiring/main_adapter.rs`.
> Crie o ponto de entrada que substitui a função de geração original.
> Se o usuário passar a flag `--lang rust`, direcione o fluxo para o nosso `03_infra/Rust_Emitter`.
> Para qualquer outra flag ou comportamento ainda não refatorado, direcione (proxy) para o código em `L20`.
> O CLI deve agora ser capaz de compilar e gerar um `parser.c` (via legado) e um `parser.rs` (via cristal) simultaneamente. Teste com a gramática JSON."

---

## 🚨 Alerta Tekt para Tree-sitter: O Invariante de Performance

O Tree-sitter é usado em tempo real enquanto o usuário digita. Se o seu novo backend Rust em `L1` for mais lento que o código C original devido a abstrações "limpas" demais (como usar `HashMaps` onde o C usava `arrays` brutos):

1. **Aborte a Pureza:** Registre um ADR (Architecture Decision Record) no `L0`.
2. **Clivagem de Performance:** Use `unsafe` em Rust dentro do `L1` se for necessário para paridade de performance com o C. O cristal deve ser puro em intenção, mas pode ser "sujo" em implementação se o domínio for **Systems Programming**.
