# Prompt Diferencial: [Nome da Alteração]

<!--
  Template brownfield. Use para ALTERAR um componente existente.
  Para criar um componente novo do zero, use template.md (greenfield).
  Regra de origem: ADR-0001.
-->

**Camada**: L[n] — [Núcleo | Casca | Infra | Fiação]
**Arquivo Alvo**: [caminho exato do arquivo a modificar]
**Passo do Roadmap**: [identificador do passo/série, se houver]
**Status**: `PROPOSTO` | `EM EXECUÇÃO` | `CONCLUÍDO`
**@base**: [caminho do prompt de origem — OBRIGATÓRIO. Ex: prompts/autenticacao-usuario.md]

<!--
  @base é o estado de partida sobre o qual este delta opera.
  Sem @base válido, este prompt não é rastreável e o linter deve rejeitá-lo.
  Pode apontar para um prompt greenfield ou para um diferencial anterior.
-->

---

## Decisões Locais

<!-- Apenas a lógica NOVA introduzida por este prompt. -->
<!-- Decisões já tomadas: cite o ADR ou prompt de origem, não rejustifique. -->
<!-- Ex.: "Seguir ADR-0003 para tratamento de caminhos." -->

- [decisão/regra nova 1]
- [decisão/regra nova 2]

---

## Código a Modificar

<!-- Apenas os excertos novos ou alterados. -->
<!-- Omita o que não muda com marcador explícito. -->

```typescript
// [arquivo alvo]
export interface Exemplo {
  // ... campos mantidos ...
  novoCampo: string; // adicionado
}
```

---

## Testes

<!-- O que precisa ser testado para ESTA modificação. -->
<!-- Não relistar testes do @base que continuam válidos. -->

- [novo caso de teste 1]
- [caso de teste alterado, se houver]

---

## Critérios de Aceitação

<!-- Checklist exato. A implementação está concluída quando todos os itens forem verdadeiros. -->

- [ ] `@base` aponta para prompt existente
- [ ] Cabeçalho de linhagem do arquivo alvo atualizado (`@updated`)
- [ ] Arquivos modificados (exatos): [lista]
- [ ] Testes que devem passar: [lista nominal]
- [ ] Sem imports proibidos para a camada L[n]
- [ ] Se L₁: zero I/O mantido
- [ ] Revisão registrada no histórico do prompt `@base`
