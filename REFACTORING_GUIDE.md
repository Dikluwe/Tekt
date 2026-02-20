# Guia de Refatoração: Aplicando Tekt em Projetos Existentes (Ex: Typst)

Se você tem um projeto maduro, como o compilador `Typst`, que cresceu organicamente ou que já é gigantesco, e decidiu aplicar a **Arquitetura Cristalina (Tekt)**, saiba de um fato: **Não tente arrumar tudo de uma vez. O LLM vai alucinar.**

Projetos legados são como rochas de entropia alta. A refatoração cristalina deve ser feita por *clivagem pontual*: você isola um pedaço funcional, dissolve ele, e o recristaliza na nova topologia.

Siga exatamente os passos abaixo.

---

## 🏗 Passo 1: O "Container" Cristalino
O seu projeto antigo provavelmente tem suas próprias pastas (`src/`, `lib/`, `tests/`). Não as exclua ainda. Elas são agora a sua "Arena Experimental" (`L20`).

Rode o script de infraestrutura no topo do repositório:
```bash
bash init-tekt.sh
```

Isto criará as hierarquias `00_nucleo` a `04_wiring`.
A partir de hoje, **nenhum código novo** deve ir para a estrutura velha.
Toda refatoração ocorrerá "cortando" a pasta velha e "colando" nas novas.

---

## 🗺️ Passo 1.5: O Mapa de Demolição (Checklist de Legado)
Um projeto como o *Typst* tem centenas de arquivos. Se você e a IA tentarem converter de forma *ad-hoc*, vocês perderão o rastro de quais arquivos do legado já foram lidos. 

**Crie um arquivo na raiz chamado `LEGACY_MAP.md` e preencha com a lista dos arquivos antigos.** Use-o como o painel de rastreio da migração.

Cole este **Prompt de Mapeamento**:
> "Sua primeira tarefa não é alterar código, mas mapear o legado. 
> Analise a pasta selecionada (ex: `20_lab/crates/typst-syntax/src`). 
> Crie no diretório raiz do nosso projeto um novo arquivo `LEGACY_MAP.md`. Nele, liste em formato de Markdown checkboxes todos os arquivos `.rs` achados nessa pasta. 
> Mas **ATENÇÃO**, abaixo de CADA arquivo `.rs` listado, você deve indentar 4 subj-checkboxes vazias exatas: 
> `  - *↳ [ ] L0 (Spec Criada)*`
> `  - *↳ [ ] L1 (Mecânica Pura Extraída)*`
> `  - *↳ [ ] L2/L3 (I/O e Superfície Isoladas)*`
> `  - *↳ [ ] L4 (Wiring Configurado)*`
> Apenas crie este arquivo de checklist e pare."

**Exemplo de como ficará o seu `LEGACY_MAP.md`:**
```markdown
# Controle de Migração Tekt (Typst)

## Módulo: Parser
- [ ] `20_lab/crates/typst-syntax/src/parser.rs` 
  - *↳ [ ] L0 (Spec Criada)*
  - *↳ [ ] L1 (Mecânica Pura Extraída)*
  - *↳ [ ] L2/L3 (I/O e Superfície Isoladas)*
  - *↳ [ ] L4 (Wiring Configurado)*
- [ ] `20_lab/crates/typst-syntax/src/lexer.rs`
- [ ] `20_lab/crates/typst-syntax/src/span.rs`
```
*(Nota: O agente de IA deve ser instruído a marcar com um `x` cada etapa concluída neste mapa antes de passar para o próximo arquivo do legado).*

---

## 🔍 Passo 2: A Engenharia Reversa (Criar o `L0` a partir do legado)
O Axioma 1 da Tekt bloqueia a Inteligência Artificial de escrever ou refatorar código que não tenha Spec pronta no `L0`. Mas o código antigo já existe. Como fazemos?
Através da **Engenharia Reversa de Spec**.

Abra sua IDE (Cursor/Cline) com as `.agentrules` já carregadas e **SELECIONE O ARQUIVO OU MÓDULO ANTIGO** que quer migrar (ex: `src/parser.rs`).

Cole este **Prompt de Refatoração Tekt**:
> "Sua tarefa como Arquiteto Cristalino começou.
> Este arquivo ou módulo legado que anexei deve ser refatorado para a Arquitetura Tekt.
> Porém, pelo Invariante de Nucleação, você NÃO PODE simplesmente reescrevê-lo nas pastas de L1-L4.
> **PASSO ÚNICO AGORA:** Leia o código legado e faça a engenharia reversa das regras de negócio que ele executa. Crie e escreva um arquivo exaustivo de Markdown na pasta `00_nucleo/specs/<nome>.md` e as interfaces Puras em `00_nucleo/contracts/`.
> Apenas crie os arquivos de `L0` e pare. Não altere o legado ainda."

---

## 💎 Passo 3: A Clivagem (O Nascimento de `L1` e `L3`)
Com os Markdown e Interfaces prontas no `L0` (e só agora lidos e perfeitamente entendidos pelo LLM), a refatoração segura começou. A IA agora fará o seu trabalho seguindo a Spec, sem precisar deduzir "por que o arquivo legado puxava o banco de dados dentro do regex parser".

Envie o segundo **Prompt Executivo**:
> "As specs estão formadas no `00_nucleo`. Leia-as.
> Agora ative as regras da Tekt para esse módulo:
> 1. Escreva a Lógica Pura (Sem I/O) matemática e algoritmos no `01_core/`. Eles devem implementar as interfaces de L0.
> 2. Qualquer regra que faça leitura/escrita física ou libs pesadas, implemente nas classes do `03_infra/`.
> 3. Crie os controladores de superfície no `02_shell/`.
> 4. Faça o Wiring final na sua composição.
> Ao terminar, confirme se a L0 e o Cabeçalho de Tipologia estão em todos os arquivos gerados. Eu irei deletar o código legado do módulo anterior e ligar a chave do L4."

---

## 🚨 O Perigo da Sedimentação em Projetos Complexos (Typst)
Se você estiver adaptando sistemas de "Hard Science" ou parsers de PDF/linguagens hiper otimizados (como **Typst em Rust**), tenha CUIDADO com o `01_core`.

Se o LLM não conseguir isolar uma chamada nativa porque é "cara demais em memória para a abstração" do `L1`, utilize a **Emergency Protocol** do `.agentrules`:
*   Escreva no ADR do `L0` dizendo que aquele pedaço quebra a topologia por Performance.
*   Assine o arquivo gerado com a instrução explícita de exceção e deixe o `L3` encostar no `L1`.

Refatorar uma vez e deixá-la purificada no Core (`L1`) pagará dividendos exponenciais no futuro porque, quando a base do sistema estiver reescrita na nova malha isolada, você e seu agente poderão escrever *Features* 5x mais rápidos sem medo de regressão sistêmica cruzada.
