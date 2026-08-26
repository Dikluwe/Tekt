# ADR-0003: Materialização segregada e evidência independente

**Status**: `PROPOSTO`
**Data**: 2026-08-24

---

## Contexto

Tekt trata o prompt em L₀ como origem causal da materialização e já separa testes da
implementação pelo Protocolo A/B. O Agente A recebe o prompt e produz código; o Agente B
recebe o mesmo prompt e produz testes sem conhecer a saída de A. Código e testes são
materializações causalmente simultâneas da mesma spec, mas não são produzidos sob o
mesmo contexto de solução. Essa independência reduz vazamento e também funciona como
oráculo de completude do prompt.

O Protocolo A/B, contudo, não define ainda quem transforma partes da intenção em
observáveis executáveis, quem tenta falsificar esses observáveis, quem sela o contrato
e quem emite o veredito final. Sem estender a segregação a esses papéis, A e B podem ser
independentes entre si e ainda assim depender de um critério fraco ou circular criado
fora do protocolo.

O problema não é exclusivo de IA. Antes de agentes generativos, compiladores, type
checkers, TDD, equipes independentes de QA, property testing, fuzzing, mutation
testing, CI, builds reproduzíveis e translation validation já separavam produção de
verificação. A propriedade útil desses mecanismos não é serem humanos ou mecânicos,
mas o produtor não controlar simultaneamente obrigação, solução e veredito.

A pilha de verificação vigente termina em julgamento humano. Isso cria uma dependência
operacional incompatível com nucleações autônomas longas. Remover o humano sem
substituir sua autoridade apenas concentra o poder no agente implementador. Usar vários
agentes com o mesmo contexto também não resolve: nomes ou sessões diferentes não
demonstram independência se todos conhecem a solução ou podem reescrever os critérios.

Experimentos no Tekt Linter introduziram validação direcional de refinamento entre duas
revisões imutáveis. Um contrato declara fatos observáveis; a transformação resulta em
`Preserved`, `Violated` com testemunha, ou `Unknown`. O mecanismo permite verificar
uma transformação concreta, mas ainda deixa aberta a autoridade para escrever e
aprovar o próprio contrato.

Esta ADR formaliza a resposta arquitetural: **materialização segregada**.

---

## Decisão

Toda materialização autônoma que altere semântica deve carregar evidência produzida sob
autoridade segregada. Nenhum participante pode controlar simultaneamente:

1. a obrigação;
2. o critério mecanicamente observável;
3. a solução;
4. os ataques ao critério;
5. o veredito final.

### Papel não é agente

Tekt define **papéis de autoridade**, não uma quantidade ou tecnologia de agentes. Um
papel pode ser executado por IA, programa determinístico, serviço de CI, humano ou
combinação desses. A independência é propriedade das entradas, permissões, ordem causal
e artefatos; não da identidade nominal do executor.

Os papéis mínimos são:

| Papel | Responsabilidade | Não pode |
|------|------------------|----------|
| Autor da intenção | produzir ou selecionar o prompt causal | aprovar a própria materialização |
| Autor do contrato | projetar observáveis a partir do prompt e baseline | ler a implementação candidata |
| Autor dos oráculos | produzir casos positivos independentes | adaptar os casos ao patch candidato |
| Adversário | produzir mutações semanticamente relevantes | corrigir ou escolher a implementação |
| Implementador | materializar prompt e contrato já selados | editar contrato, baseline ou oráculos protegidos |
| Verificador | executar gates e emitir recibo | escrever qualquer artefato verificado |

Uma execução pode usar mais papéis, mas não pode fundir autor do contrato,
implementador e verificador sob a mesma autoridade de escrita.

### Extensão do Protocolo A/B

Esta decisão preserva, em vez de substituir, a simultaneidade independente de A/B:

```text
                         ┌─ Agente A: código ─────┐
prompt + contrato selado ┤                        ├─ verificação
                         └─ Agente B: testes ─────┘
```

A e B continuam sem conhecer a materialização um do outro. O novo protocolo acrescenta
fases anteriores para contrato e contestação e uma fase posterior para julgamento
segregado. O Agente B pode continuar produzindo testes funcionais e arquiteturais; o
autor dos oráculos de refinamento pode ser o próprio B quando suas permissões e entradas
forem as mesmas do Protocolo A/B, ou um terceiro papel quando o domínio exigir ataque
especializado.

“Simultâneo” significa que código e testes derivam da mesma versão congelada da spec e
nenhum deriva da saída do outro. Não significa que o mesmo agente os escreve na mesma
sessão.

### Prompt como raiz da cadeia

O contrato de refinamento não substitui o prompt. Ele é uma **projeção executável e
limitada** da intenção registrada no prompt. O prompt continua sendo a raiz de
autoridade e passa a poder declarar:

```yaml
refinement:
  protocol: tekt-segregated-refinement-v1
  contract: 00_nucleo/refinement/contracts/equation-accessibility.toml
  baseline: 781b207b4a5de9c2bfbe5819918a193d1d9293e5
  unknown_policy: block

  observables:
    - equation.alt
    - equation.tagging
    - equation.supplement

  required_evidence:
    positive_oracles: 2
    negative_mutations: 3
    unknown_oracles: 1
```

O campo `observables` delimita o que o contrato precisa representar. O autor do
contrato pode escolher linguagem, queries e normalizações, mas não reduzir
silenciosamente a intenção a uma propriedade mais fácil.

### Ordem causal obrigatória

A execução segue fases seladas:

```text
prompt + baseline
        ↓
contrato candidato
        ↓
oráculos positivos + mutações adversariais
        ↓
gate discriminatório do contrato
        ↓
contrato selado
        ↓
implementação
        ↓
gate de refinamento
        ↓
certificado da transformação
```

O autor do contrato trabalha antes de existir patch candidato e não recebe raciocínio,
diff ou artefatos privados do implementador. O implementador recebe o contrato já
selado como entrada somente leitura. Alterar prompt, baseline, contrato ou oráculos
durante a materialização invalida o selo e reinicia a cadeia.

### Manifesto de execução

Cada cadeia possui um manifesto canônico, versionado e serializado
deterministicamente:

```json
{
  "protocol_version": 1,
  "prompt_path": "00_nucleo/prompts/entities/equation.md",
  "prompt_hash": "abc12345",
  "baseline_oid": "781b207...",
  "contract_path": "00_nucleo/refinement/contracts/equation-accessibility.toml",
  "required_observables": ["equation.alt", "equation.tagging"],
  "unknown_policy": "block"
}
```

O manifesto congela prompt, baseline, escopo observável e política de incerteza. Todos
os recibos referenciam seu hash. Identidade de sessão ou alegação textual de autoria é
metadado; a garantia vem das capacidades efetivamente concedidas.

### Gate discriminatório do contrato

Um contrato não é confiável apenas por ser válido sintaticamente. Antes da
implementação, um verificador mecânico exige:

```text
casos positivos válidos          → Preserved
mutações semanticamente negativas → Violated
construções deliberadamente opacas → Unknown
repetição e reordenação            → resultado determinístico
```

O contrato recebe um `mutation_score`:

```text
mutações corretamente rejeitadas / mutações válidas
```

Na primeira versão, o selo exige `mutation_score = 1.0`. Mutação relevante sobrevivente
invalida o contrato; `Unknown` não conta como mutação rejeitada, salvo quando o oráculo
foi explicitamente criado para testar opacidade.

O contrato deve distinguir:

- fato ausente conhecido;
- observável não encontrado;
- identidade ambígua;
- construção opaca;
- parser sem suporte;
- orçamento esgotado.

Nenhuma dessas condições pode ser convertida implicitamente em `Preserved`.

### Selo e certificado

O gate do contrato produz um selo somente leitura:

```json
{
  "protocol": "tekt-segregated-refinement-v1",
  "manifest_hash": "...",
  "prompt_hash": "...",
  "contract_hash": "...",
  "baseline_oid": "...",
  "positive_suite_hash": "...",
  "mutation_suite_hash": "...",
  "mutation_score": 1.0,
  "verdict": "sealed"
}
```

Depois da materialização, o verificador produz um certificado:

```json
{
  "protocol": "tekt-segregated-refinement-v1",
  "seal_hash": "...",
  "baseline_oid": "...",
  "implementation_oid": "...",
  "refinement_verdict": "preserved",
  "architecture_verdict": "clean"
}
```

O certificado não prova equivalência funcional geral. Ele atesta que uma transformação
concreta preservou o fragmento observável do contrato selado, sob as versões, budgets e
oráculos registrados.

### Isolamento exigido

Agentes independentes devem operar em sandboxes, worktrees ou ambientes equivalentes
com capacidades mínimas. O protocolo registra e verifica:

- allowlist de artefatos legíveis por papel;
- allowlist de caminhos graváveis por papel;
- ausência de contexto conversacional compartilhado proibido;
- commits ou pacotes de saída separados;
- troca apenas por artefatos canônicos;
- ordem causal entre contrato, oráculos e implementação;
- hashes antes e depois de cada fase;
- verificador final sem autoridade de escrita.

Não é necessário provar independência psicológica ou estatística entre modelos. É
necessário provar que nenhum papel recebeu capacidade suficiente para modificar a
obrigação, a solução e o veredito no mesmo ciclo.

### Nova forma da pilha de verificação

Para nucleações autônomas, a quarta camada deixa de significar obrigatoriamente
“humano” e passa a significar **autoridade de julgamento segregada**:

| Camada | Verifica | Executor típico |
|-------|----------|-----------------|
| Lint | forma e gravidade | programa determinístico |
| Testes | conformidade com a spec | runner independente |
| Oráculo | fidelidade e poder discriminatório | corpus + mutações |
| Julgamento segregado | validade da obrigação e da cadeia | papéis independentes + gates mecânicos |

O humano torna-se autoridade excepcional para insuficiência da intenção, conflito
entre interpretações ou ampliação de efeitos externos. Não é dependência obrigatória
do ciclo normal depois que o prompt e as políticas de autonomia já concederam a
autoridade necessária.

### Estrutura recomendada em L₀

```text
00_nucleo/refinement/
├── manifests/
├── contracts/
├── oracles/
│   ├── positive/
│   ├── negative/
│   └── unknown/
├── seals/
└── certificates/
```

Contratos e manifestos são ex-ante. Selos e certificados são recibos ex-post. Nenhum
deles é código executável; executores e adapters permanecem fora de L₀.

### Incremento inicial

A primeira materialização desta ADR deve ser independente de fornecedor de agentes:

1. definir schemas canônicos para manifesto, atestação, selo e certificado;
2. permitir que um prompt referencie um manifesto de refinamento;
3. validar hashes, ordem causal e capacidades declaradas;
4. executar oráculos positivos, negativos e inconclusivos;
5. selar apenas contratos com poder discriminatório completo;
6. verificar a revisão implementada contra o selo;
7. deixar a orquestração automática de agentes como adapter posterior.

O linter verifica o protocolo; não se torna o orquestrador universal de agentes.

---

## Prompts Afetados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `prompts/template.md` | adicionar seção opcional de refinamento segregado para nucleações semânticas |
| `prompts/template-diff.md` | adicionar manifesto, baseline, observáveis e política de `Unknown` |
| futuro prompt do verificador | definir schemas, gates e certificados sem acoplamento a fornecedor de IA |

Documentos globais afetados em revisão posterior: `MANIFESTO.pt.md`, `MANIFESTO.md`,
`AGENTS.md`, `00_nucleo/README.pt.md`, `00_nucleo/README.md` e `LESSONS.pt.md`.

---

## Consequências

**Positivas**: permite desenvolvimento semanticamente autônomo sem concentrar autoria,
implementação e aprovação num único agente; transforma parte da intenção do prompt em
obrigação executável; expõe contratos vacuosos por mutação; preserva incerteza como
`Unknown`; produz recibos reproduzíveis; generaliza práticas mecânicas anteriores à IA;
mantém Tekt independente de modelos e orquestradores específicos.

**Negativas**: multiplica artefatos e execuções; exige isolamento real de capacidades;
contratos e mutações podem continuar incompletos; aumenta custo computacional; hashes
não provam independência quando o ambiente mente sobre permissões; conflitos de
interpretação ainda podem bloquear a cadeia.

**Neutras**: humanos continuam possíveis, mas deixam de ser etapa obrigatória no ciclo
normal; múltiplos agentes são uma implementação comum, não um princípio; o selo prova
somente o fragmento observável e a transformação concreta.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Um agente escreve contrato, código e testes | simples e barato | contradiz o Protocolo A/B; prova circular |
| Manter somente A/B | mecanismo conhecido e já validado contra vazamento | não segrega autoria do contrato, adversário, selo e veredito |
| Vários agentes com contexto completo compartilhado | diversidade aparente | independência nominal; todos podem adaptar critérios à solução |
| Humano aprova todo contrato | julgamento amplo | impede autonomia e não escala com nucleações contínuas |
| Contrato gerado automaticamente do código | fácil de manter sincronizado | descreve a solução em vez da intenção; regressão pode virar novo contrato |
| Somente mutation testing | mede força dos testes | não ancora mutações na intenção causal nem congela autoridade |
| Materialização segregada (escolhida) | separa poderes, preserva autonomia e gera evidência verificável | requer protocolo, sandboxes, artefatos e gates adicionais |

---

## Questões em Aberto

1. Como medir diversidade semântica mínima entre mutações produzidas por agentes
   diferentes?
2. Quando uma mutação é inválida por contradizer o próprio prompt?
3. Qual política resolve interpretações incompatíveis sem introduzir humano obrigatório?
4. Como atestar tecnicamente que um agente não recebeu o patch candidato fora do canal
   declarado?
5. Quais mudanças são “semânticas” e exigem o protocolo completo?
6. Como expirar selos quando parser, extrator ou budgets mudam?
7. Como manter certificados pequenos sem perder recibos reproduzíveis?

Essas questões não impedem o primeiro incremento mecânico, mas impedem declarar que o
protocolo substitui prova formal ou julgamento em domínios não observáveis.

---

## Referências

- `MANIFESTO.pt.md` — prompt como origem causal e paradigma de verificação.
- `LESSONS.pt.md` — L6 (Protocolo A/B), L8 (pilha de verificação) e L11
  (alegações atestadas).
- ADR-0001 — prompts diferenciais e baseline causal.
- `00_nucleo/prompts/template.md` e `template-diff.md`.
- Tekt Linter ADR-0019 — validação direcional de refinamento sobre fatos observáveis.
- Alive2 — translation validation de transformações concretas.
- Mutation testing, property-based testing, fuzzing e reproducible builds como
  precedentes mecânicos de verificação independente.
