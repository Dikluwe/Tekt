---
description: Inicialização única do projeto legado para refatoração Tekt (Passos 1 e 1.5)
---

# Init Legado (Setup Único)

Este workflow é executado **uma única vez** no início da refatoração de um projeto legado para a Arquitetura Cristalina (**Tekt**). Ele prepara o terreno para que qualquer IA, mesmo sem acesso ao Manifesto original, entenda a estrutura de camadas.

## 📐 A Estrutura de Camadas Tekt

Durante toda a refatoração, a IA deve respeitar esta hierarquia de pastas e responsabilidades:

| Camada | Nome | Responsabilidade | Conteúdo Permitido |
| :--- | :--- | :--- | :--- |
| **`00_nucleo/`** | **L0: Especificação** | A Semente Cristalina. Define o "O Quê" sem o "Como". | Markdown Specs, Contratos (Interfaces), Tipos e Enums Puros. |
| **`01_core/`** | **L1: Lógica Pura** | O Cérebro Incorruptível. Algoritmos determinísticos. | Lógica Pura, Computação, Transformação de Dados. **Zero I/O.** |
| **`02_shell/`** | **L2: Aplicação** | A Casca. Orquestração de casos de uso e estado. | Services, Handlers, Orquestradores de fluxo. |
| **`03_infra/`** | **L3: Efeitos** | O Braço. Implementação física do mundo externo. | Database, API Clients, Filesystem, Drivers, Renderizadores. |
| **`04_wiring/`** | **L4: Composição** | A Cola. Onde o sistema ganha vida e integra com o velho. | entry-points (`main`), Dependency Injection, Wiring Parcial. |
| **`20_lab/`** | **L20: Arena** | O Legado (Quarentena). Código radioativo READ-ONLY. | Código antigo movido mas ainda não refatorado. |

---

## Passos de Inicialização

### 1. Armar o Container Cristalino (Passo 1)
// turbo
1. Rode o script de infraestrutura na raiz do projeto legado para criar a estrutura Tekt:
```bash
bash init-tekt.sh
```

2. Mova todo o código fonte legado (pastas como `src/`, `crates/`, `lib/`, `tests/`) para dentro de `20_lab/`. 
   - **IMPORTANTE:** `20_lab/` é uma quarentena. A partir daqui, ela deve ser tratada como **Read-Only**.

3. Ajuste o sistema de build (`Cargo.toml`, `package.json`, etc.) para que todos os paths apontem para dentro de `20_lab/`. Verifique se o projeto continua compilando e passando nos testes originais.

4. Execute o commit de checkpoint:
```bash
git add . && git commit -m "chore: isolamento topológico do legado para arena L20 (Tekt)"
```

### 2. Criar o Mapa de Demolição (Passo 1.5)
O Mapa de Demolição é o GPS da migração. Forneça o seguinte prompt à IA:

> "Sua tarefa é agir como o Arquiteto de Mapeamento Tekt. Analise a pasta selecionada (ex: `20_lab/src`).
> 
> **OBJETIVO:** Criar o arquivo `LEGACY_MAP.md` na raiz com o checklist de todos os arquivos a serem refatorados.
> 
> **FORMATO:**
> - [ ] 20_lab/caminho/do/arquivo.rs | [ ] L0 Spec (em 00_nucleo)
> 
> **CONTEXTO ARQUITETURAL:** Você deve usar as pastas `00` a `04` para a reconstrução. `00` (Spec), `01` (Core/Lógica), `02` (Application/Shell), `03` (Infra/I/O) e `04` (Wiring). O original em `20_lab` é imutável.
> 
> Apenas crie o arquivo `LEGACY_MAP.md` e pare."

5. Commit do mapa inicial:
```bash
git add LEGACY_MAP.md && git commit -m "docs: criado GPS da migração (LEGACY_MAP.md)"
```

## Resultado Esperado
- Estrutura `00_nucleo` a `04_wiring` operantes.
- Legado isolado e compilando dentro de `20_lab/`.
- `LEGACY_MAP.md` pronto para guiar o workflow `/refatorar-arquivo`.
