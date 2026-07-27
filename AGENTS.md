# AGENTS.md — Diretrizes Unificadas do Agente (Arquitetura Tekt)

Este arquivo é a **Fonte Única da Verdade (SSOT)** de regras e restrições estruturais para Agentes de IA (Antigravity, Claude Code, Cursor, Aider, etc.) trabalhando no repositório **Tekt**.

---

## 1. O Que É Este Projeto

**Tekt** é uma especificação de meta-arquitetura — uma estrutura para manter coerência topológica em sistemas desenvolvidos ou refatorados com auxílio de IA, evitando o crescimento amorfo e intraduzível.

**Hipótese Central:** Manter prompts estruturados versionados dentro do projeto em `00_nucleo/` (causalidade explícita) e impor estrita gravidade de dependências entre camadas garante a integridade estrutural ao longo de evoluções prolongadas.

---

## 2. O Reticulado de Camadas (Five-Layer Lattice + Substrato)

```text
L₄ (Wiring)     04_wiring/     Composition root, DI — importa todas as camadas (exceto Lab)
L₂ (Shell)      02_shell/      Adaptadores primários, I/O, HTTP/CLI — importa L₀, L₁
L₃ (Infra)      03_infra/      Persistência, drivers, APIs externas — importa L₀, L₁
L₁ (Core)       01_core/       Lógica pura, zero I/O — importa L₀ apenas
L₀ (Nucleus)    00_nucleo/     Prompts, ADRs e Relatórios apenas — ZERO CÓDIGO
Lab (Workbench) lab/           Experimentos e testes de bancada — importa L₀ apenas
L₂₀ (Substrate) lab/legado/    Substrato de código original lido em modo Brownfield (read-only)
```

### Leis de Gravidade e Restrições Absolutas:

* **Lei da Gravidade:** Dependências fluem **apenas para baixo**. Nenhuma camada numerada ($L_1$–$L_3$) importa de `Lab`. Somente $L_4$ enxerga todas as demais.
* **Restrição de $L_1$ (Núcleo Puro):** ZERO I/O. Proibido acessar banco de dados, rede, sistema de arquivos, relógio do sistema (`Date.now()`), ou estado global mutável.
* **Restrição de $L_4$ (Fiação Pura):** ZERO LÓGICA DE NEGÓCIO. Qualquer `if/else` contendo regras de negócio em $L_4$ é um defeito estrutural.
* **Gravidade Fina (Intra-camada - L9):** A ordenação de diretórios expressa estabilidade dentro da mesma camada:
  * **Phase:** Unidade de transformação isolada (ex: `parse`, `eval`, `layout`).
  * **Container:** Módulo que orquestra suas fases filhas sem conter regras e quebra ciclos de importação.
  * **Facade:** Entrada única do container vista pelos consumidores externos.

---

## 3. Modos de Operação do Agente (Lição L1)

O agente deve identificar em qual modo de operação está trabalhando:

### A. Modo Greenfield (Prompt → Código)
Usado quando um novo componente é criado do zero.
1. Inspecionar `00_nucleo/prompts/` por uma Spec existente.
2. Se não existir $\rightarrow$ **PARE.** Solicite a criação do prompt $L_0$ antes de gerar o código.

### B. Modo Brownfield (Código Original $\rightarrow$ Exegese $\rightarrow$ Spec $L_0$ $\rightarrow$ Código Regenerado)
Usado ao migrar ou refatorar código legado ($L_{20}$).
1. Ler o código legado em $L_{20}$ ou na área de quarentena.
2. Executar a fase de **Exegese/Arqueologia**: reconstruir mentalmente a especificação $L_0$ que o código *deveria* ter tido.
3. Gerar ou atualizar a Spec estruturada em `00_nucleo/prompts/`.
4. Materializar o código limpo na camada Tekt correspondente ($L_1$–$L_4$).

---

## 4. Tríade da Nucleação (Lição L5)

Toda nucleação produz obrigatoriamente **três artefatos**:

1. **Spec / Prompt ($L_0$):** O contrato causal (ex-ante).
2. **Código + Testes ($L_1$–$L_4$):** A materialização.
3. **Relatório de Execução Mínimo:** Registro curto do que aconteceu durante a geração (descobertas em voo, alternativas descartadas, divergências justificadas).

---

## 5. Prevenção de Vazamento de Contexto (Protocolo A/B - Lição L6)

Para evitar que os testes apenas "validem os vícios da implementação":
* Os testes devem ser gerados estritamente com base na **Spec $L_0$**, testando o comportamento como uma caixa-preta.
* Se os testes não puderem ser escritos apenas lendo a Spec $L_0$, a especificação está incompleta e deve ser corrigida.

---

## 6. Stack de Verificação em 4 Camadas (Lições L8 e L11)

Toda alteração deve ser validada na seguinte pilha:

| Camada | O Que Verifica | Erro Capturado | Ferramenta/Frequência |
| :--- | :--- | :--- | :--- |
| **1. Lint** | Forma e gravidade | Violação estrutural | Contínuo / CI |
| **2. Testes** | Especificação $L_0$ | Divergência da Spec | Por nucleação |
| **3. Oráculo** | Fidelidade ao substrato | Regressão em relação ao original | Por marco / release |
| **4. Humano** | Julgamento | O que os 3 acima não alcançam | Por ADR |

### Reclamações Atestadas no Oráculo (Lição L11)
Relatórios de paridade com o sistema original devem conter obrigatoriamente a tríade de atestação:
* **Executor:** O comando exato executado (ex: `mutool trace`, `cargo test --bench`).
* **Recibo (Receipt):** Os dados numéricos/estruturais brutos retornados.
* **Atestador (Attester):** A comparação determinística pass/fail em relação ao esperado.

---

## 7. Cabeçalho Obrigatório de Linhagem

Todo arquivo gerado ou modificado em $L_1$, $L_2$, $L_3$ ou $L_4$ DEVE conter obrigatoriamente no topo:

```typescript
/**
 * Crystalline Lineage
 * @prompt 00_nucleo/prompts/<nome_do_prompt>.md
 * @layer L<0_a_4>
 * @updated YYYY-MM-DD
 */
```

---

## 8. Workflows do Agente (`.agents/workflows/`)

Ao realizar operações estruturadas complexas, invoque os workflows específicos:

* **`init-legado.md`**: Inicializa um projeto legado isolando a base em $L_{20}$ (read-only).
* **`gerar-spec.md`**: Conduz a exegese e gera a Spec $L_0$ para um componente.
* **`clivar-modulo.md`**: Divide um módulo extenso em suas camadas apropriadas ($L_1$/$L_2$/$L_3$).
* **`auditar-spec.md`**: Audita a qualidade e completude dos prompts $L_0$.
* **`integrar-legado.md`**: Conecta o novo código refatorado e faz o wiring parcial.

---

## 9. Documentos de Referência

* `README.md` — Referência rápida da arquitetura e tabela de dependências.
* `MANIFESTO.md` — Manifesto conceitual do Tekt.
* `LESSONS.md` — Aprendizados metodológicos $v1.4$ (documento vivo de descobertas).
