# /_lab — A Arena

> Zona de quarentena. Experimentos vivem e morrem aqui.

---

## Propósito

Este estrato desempenha dois papéis fundamentais na ciência do código e na exploração do sistema:

1. **A Arena (Papel v1.3)**: Espaço de alta entropia para código exploratório, protótipos e spikes descartáveis que ainda não satisfazem os invariantes do sistema principal.
2. **A Bancada (Papel v1.4)**: Plataforma para experimentos controlados com hipótese, controle e medição. Produz dados e evidências empíricas para fundamentar decisões arquiteturais registradas em ADRs (ex.: medir paridade ou performance comparativa).

---

## Regras e Regimes de Custo (v1.4)

A Tekt v1.4 formaliza o **Regime do lab** (baixo custo, flexibilidade e foco científico) em oposição ao **Regime do sistema** (alto custo, linhagem obrigatória, linter estrito):

**Dentro do Lab (`_lab/`):**
- **Código barato**: Desenvolvimento livre, sem obrigatoriedade de linhagem completa em todas as etapas de desenvolvimento.
- **Prompt opcional**: Não há necessidade de nucleação rígida em L₀ para experimentos voláteis.
- **Hipóteses livres**: Relaxamento de regras de pureza e isolamento para prototipagem rápida.
- **Medição obrigatória**: Apenas quando o experimento for utilizado como evidência para uma decisão arquitetural em ADR.

**Fora do Lab:**
- Nenhum estrato do sistema principal (`L₁` a `L₄`) pode importar de `_lab`.

---

## O Ato de Promoção (Lab → Sistema)

A migração de um componente do Lab para o sistema principal requer um **Ato de Promoção** formal (e não mera cópia):

1. **Reescrita Orientada**: O componente é reescrito a partir de um prompt completo em `00_nucleo/` (Nucleação).
2. **Normalização**: Adaptação estrutural do código para respeitar todos os invariantes do estrato de destino (`L₁` a `L₄`).
3. **Linhagem e Testes**: Inclusão de testes gerados simultaneamente e do cabeçalho de linhagem obrigatório `@prompt`.

---

## Estrutura sugerida

```
_lab/
├── experiments/   # Experimentos pontuais
├── spikes/        # Investigações técnicas de prazo curto
└── prototypes/    # Protótipos funcionais para validação
```

---

## Quando usar a Arena

- Investigar se uma biblioteca resolve o problema antes de comprometer com ela
- Validar uma hipótese de implementação sem contaminar o sistema
- Prototipar rapidamente para testar com usuários
- Explorar refatorações antes de executá-las no sistema principal
