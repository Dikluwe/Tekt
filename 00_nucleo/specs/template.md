# Spec: [Nome da Feature]

> **Linhagem**: Este documento é a semente desta feature. Todo código derivado deve referenciar este arquivo.

---

## Visão Geral

**O que é**: Descrição concisa da feature e seu propósito no domínio.

**Por que existe**: Qual problema resolve. Qual valor entrega.

---

## Requisitos Funcionais

- **[REQ-01]**: Descrever o comportamento esperado com precisão.
- **[REQ-02]**: Cada requisito deve ser verificável.
- **[REQ-03]**: Evitar ambiguidade — "rápido" não é requisito, "< 200ms" é.

## Requisitos Não-Funcionais

- **Performance**: ex. Latência < 100ms para 95% das requisições
- **Segurança**: ex. Seguir ADR-0005 para criptografia
- **Confiabilidade**: ex. Operação deve ser idempotente

---

## Critérios de Aceitação

Cenários que provam que a implementação satisfaz a spec.

```gherkin
Scenario: [Nome do cenário]
  Given [Pré-condição precisa]
  When [Ação determinística]
  Then [Estado resultante esperado]
```

---

## Posicionamento no Lattice

Como esta feature se distribui pelos estratos:

| Camada | Componente | Responsabilidade |
|--------|------------|------------------|
| `01_core` | `[NomeDaLógica]` | Regras puras de negócio |
| `02_shell` | `[NomeDoController]` | Tradução de entrada e projeção de saída |
| `03_infra` | `[NomeDoRepositório]` | Persistência e efeitos colaterais |

---

## Rastreabilidade

- **Contrato**: [link para 00_nucleo/contracts/interface.md]
- **ADR relacionado**: [link para 00_nucleo/adr/NNNN.md] (se aplicável)
- **Dependências externas**: Listar ferramentas de terceiros permitidas em L₃

---

## Notas

Casos de borda, armadilhas conhecidas, ou riscos estruturais a evitar.
