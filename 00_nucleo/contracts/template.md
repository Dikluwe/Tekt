# Contrato: [Nome da Interface]

> **Linhagem**: Este contrato define a fronteira entre o Núcleo e suas implementações externas.

---

## Propósito

Descrever o que esta interface abstrai e qual problema de domínio ela resolve.

---

## Definição

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/[nome-da-spec].md
 * @layer L0
 */
export interface I[Entidade][Ação] {
  /**
   * Descrever o propósito do método.
   * @param input - Tipo de domínio de entrada
   * @returns Resultado esperado
   */
  nomeDoMetodo(input: TipoDeEntrada): Promise<TipoDeSaída>;
}
```

---

## Implementações

| Camada | Classe | Propósito | Status |
|--------|--------|-----------|--------|
| `03_infra` | `Sql[Nome]Repository` | Persistência em produção | Pendente |
| `_lab` | `Mock[Nome]Provider` | Testes e experimentos | Ativo |

---

## Como usar em Wiring

```typescript
// 04_wiring/main.ts
const implementacao = new SqlImplementacao();
const servico = new Servico(implementacao as INomeInterface);
```

---

## Notas

Invariantes específicos, casos de borda ou restrições que esta interface deve respeitar.
