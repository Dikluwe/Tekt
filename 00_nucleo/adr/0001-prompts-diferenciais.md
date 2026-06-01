# ADR-0001: Prompts diferenciais para desenvolvimento brownfield

**Status**: `ACEITO`
**Data**: 2026-05-25

---

## Contexto

O manifesto Tekt define o prompt em L₀ como origem causal reproduzível: "preciso o
suficiente para que uma nova execução produza resultado estruturalmente equivalente".
O `00_nucleo/prompts/template.md` atual assume modo greenfield — o prompt contém o
estado completo do componente e gera o código do zero.

Em desenvolvimento contínuo (modo brownfield, descrito em `LESSONS.pt.md`, lição L1),
essa forma produz repetição: decisões de arquitetura já registradas em ADRs são
reescritas, e blocos de código inalterados são recopiados a cada revisão. O próprio
`LESSONS.pt.md` (lição L5) registra que os laudos "crescem além do prompt original" e
deixa em aberto a forma do "laudo mínimo viável".

Esta decisão fecha essa pergunta para o caso dos prompts de alteração.

---

## Decisão

Adotar dois templates de prompt em L₀, um por modo operacional:

| Template | Modo | Função |
|----------|------|--------|
| `template.md` | Greenfield | Criação de componente novo. Estado completo. Gera código do zero. Inalterado por esta ADR. |
| `template-diff.md` | Brownfield | Revisão/alteração de componente existente. Apenas o delta (decisões locais, código alterado, novos testes). |

O prompt diferencial segue cinco regras de redução:

1. **Sem contexto redundante.** Não reescrever o que o sistema já faz.
2. **Referência em vez de repetição.** Decisões já tomadas são citadas pelo seu ADR ou prompt de origem, não rejustificadas.
3. **Só o código alterado.** Trechos inalterados são omitidos com marcador (ex.: `// ... campos mantidos ...`).
4. **Instruções diretas.** Texto como conjunto de ações (adicionar/modificar), sem narração ou histórico embutido.
5. **Critérios de aceitação estritos.** Checklist exato: testes que devem passar e arquivos exatos modificados.

### Mitigação obrigatória do campo `@base`

Um prompt diferencial não é reproduzível isoladamente — depende do estado anterior do
componente. A rastreabilidade é preservada pela soma `prompt-base + prompt-diferencial`.

Para isso, o cabeçalho de `template-diff.md` tem o campo **obrigatório** `@base`,
apontando para o prompt de origem (greenfield ou um diferencial anterior) sobre o qual
o delta opera. Sem `@base` válido, a nucleação diferencial é inválida e o linter deve
rejeitá-la, da mesma forma que rejeita um `@prompt` ausente.

---

## Prompts Afetados

| Prompt | Natureza da mudança |
|--------|---------------------|
| prompts/template.md | Nenhuma. Permanece como template greenfield canônico. |
| prompts/template-diff.md | Criação. Novo template brownfield. |

---

## Consequências

**Positivas**: redução da carga de leitura e de manutenção em prompts de alteração;
fim da recópia de decisões e de código inalterado; alinhamento explícito com os dois
modos operacionais já descritos em `LESSONS.pt.md` (L1).

**Negativas**: o prompt diferencial perde a propriedade de reprodutibilidade isolada
declarada no manifesto. Um delta sozinho não gera o componente — a origem causal passa
a ser o par `@base + delta`, não o prompt único. Esta é uma flexibilização consciente
de um invariante central de L₀, aceita em troca da redução de redundância, e contida
pela obrigatoriedade do `@base`.

**Neutras**: aumenta em um o número de templates a manter. A escolha de qual template
usar passa a ser uma decisão explícita no início de cada nucleação (novo vs. alteração).

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Substituir o template único pelo diferencial | Um só template a manter | Perde a capacidade greenfield de gerar do zero; quebra reprodutibilidade sem mitigação |
| Manter só o template completo (status quo) | Reprodutibilidade isolada preservada | Mantém a redundância diagnosticada em L5 |
| Conviver + Mitigado (escolhida) | Reduz redundância sem perder greenfield; rastreabilidade via `@base` | Dois templates; invariante de L₀ flexibilizado para o caso brownfield |

---

## Referências

- `MANIFESTO.pt.md` — prompt como origem causal reproduzível; limitação declarada sobre divergência de L₀
- `LESSONS.pt.md` — L1 (modo brownfield), L5 (laudo mínimo viável)
- `00_nucleo/prompts/template.md` — template greenfield
- `00_nucleo/prompts/template-diff.md` — template diferencial criado por esta ADR
