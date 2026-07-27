# Lições para Tekt v1.4

**Estado**: VIVO — cresce em paralelo aos passos do typst-crystalline.

**Origem**: destilação de inversões metodológicas descobertas durante a refatoração de typst (typst-crystalline) com base no Manifesto Tekt v1.3. A partir de L7, também dos projectos-irmãos (crystalline-lint, lente/tekt-cargo-dsm).

**Função**: colher, em forma pura e independente do typst, ensinamentos que devem entrar no Manifesto v1.4. Cada entrada é candidata a Princípio, Mecânica ou Padrão.

**Regra de inclusão**: só entra aqui o que é **inversão genuína** face a v1.3 — não refinos de detalhe nem afirmações que o Manifesto já cobre implicitamente. Refinos vivem em ADRs locais.

**Regra de forma**: cada lição é descrita em três blocos curtos:
- **v1.3 dizia (ou assumia)** — o que estava posto, explícita ou implicitamente
- **Descoberta empírica** — o que o projecto mostrou que era insuficiente ou errado
- **Forma candidata para v1.4** — proposta concreta de princípio, mecânica ou vocabulário

A discussão longa fica nos ADRs e relatórios originais; aqui é destilação.

---

## L1 — Modo brownfield: a flecha causal invertida

**v1.3 assumia**: prompt antes de código. Greenfield implícito em todo o Manifesto. A nucleação é o ponto de partida; o código é materialização.

**Descoberta empírica**: em refatoração, o código vem antes do prompt. A nucleação não pode ser o primeiro acto — tem de ser precedida por uma operação de **inferência reversa**, onde o código existente é lido para reconstruir o L₀ que ele *teria tido* se fosse greenfield. Sem este passo, o agente regenera no escuro.

**Forma candidata para v1.4**: distinguir dois modos operacionais de Tekt.

- **Modo greenfield** — prompt → código. Tekt v1.3 canónico.
- **Modo brownfield** — código → inferência → prompt → código regenerado.

O modo brownfield introduz uma fase nova antes da nucleação, ainda sem nome canónico (candidatos: *exegese*, *arqueologia*, *exumação*). Esta fase produz um artefacto intermediário que serve de matéria-prima ao prompt.

**Evidência viva**: typst-crystalline opera integralmente em modo brownfield. A vanilla typst em `lab/typst-original/` é o substrato; cada passo de nucleação foi precedido por leitura do código vanilla correspondente.

---

## L2 — Oráculo externo: referência executável como segunda fonte de verdade

**v1.3 dizia**: o prompt é a verdade. O linter verifica estrutura. O humano verifica fidelidade. Não há nenhuma terceira fonte.

**Descoberta empírica**: em brownfield, existe uma **referência executável** — o sistema original — contra a qual é possível medir fidelidade do código regenerado. Esta referência não substitui o julgamento humano, mas serve de **oráculo parcial**: permite paridade observacional automatizada, divergência declarada com justificação, e graduação de "implementado" em vez de binário existe/não-existe.

**Forma candidata para v1.4**: introduzir o conceito de **oráculo de fidelidade**.

- Em greenfield, o oráculo é apenas o prompt + julgamento humano.
- Em brownfield, há um segundo oráculo: o sistema-fonte ou um corpus de regressão.
- O oráculo executável permite **medição** onde antes só havia argumentação.

Vocabulário de paridade que emergiu e merece formalização:

- `implementado` — feature presente, sem ressalvas
- `implementado⁺` — feature presente com aproximação documentada
- `parcial` — captura existe; consumo divergente ou ausente
- `ausente` — não capturado
- `scope-out` — ADR explícita declara fora do escopo

**Evidência viva**: ADR-0033 (paridade funcional vanilla), ADR-0054 (perfil observacional graded), ADR-0075 (vanilla integration), `lab/parity/`.

---

## L3 — Lab como bancada: ciência de código

**v1.3 dizia**: `_lab` é arena descartável. Código volátil, sem linhagem, isolado do sistema principal. Migração da arena para o sistema exige reescrita completa.

**Descoberta empírica**: o lab tem um papel mais ambicioso que arena de spikes — pode ser **plataforma de experimentos comparativos** onde se prova que uma implementação é melhor que outra por critérios mensuráveis. Isto introduz em Tekt uma fonte de verdade que v1.3 não previu: **evidência empírica como argumento arquitectural**, ao lado do argumento escrito em ADR.

**Forma candidata para v1.4**: lab passa a ter dois papéis distintos.

- **Arena** (papel v1.3) — código experimental sem linhagem, descartável.
- **Bancada** (papel novo) — experimentos controlados com hipótese, controlo, medição. Produz **resultados** que podem fundamentar decisões arquiteturais.

A bancada permite que uma ADR seja fundamentada não apenas por raciocínio escrito, mas por experimento reproduzível registado em L_lab. Isto aproxima Tekt do método científico aplicado.

**Pergunta em aberto**: que critérios separam um experimento de bancada de uma decisão pronta para promoção? Provavelmente um **gradiente de prova** com patamares declarados.

**Evidência viva**: o próprio typst-crystalline opera como bancada para Tekt — `lab/parity/` mede paridade vs. vanilla com corpus controlado e relatórios datados.

---

## L4 — Dois regimes de custo: sistema vs. lab

**v1.3 dizia**: implicitamente, todo código é caro. O prompt é caro. A disciplina de linhagem é alta em todos os estratos.

**Descoberta empírica**: trabalhando com IA, o custo de gerar código é radicalmente assimétrico face ao custo de gerar prompt. Código exploratório descartável é **barato**. Forçar o mesmo regime de disciplina sobre código que existirá por horas e sobre código que existirá por anos é desperdício.

**Forma candidata para v1.4**: declarar explicitamente **dois regimes de custo** com disciplinas distintas.

- **Regime do sistema** — `00_nucleo` até `04_wiring`. Código caro, prompt caro, linhagem obrigatória, linter estrito.
- **Regime do lab** — `_lab/`. Código barato, prompt opcional, hipóteses livres, medição obrigatória apenas quando o experimento for usado como evidência.

E entre os dois, **fluxo unidireccional**: lab → sistema requer **acto de promoção** — reescrita com prompt completo, em camadas, com linhagem, testes. Sistema → lab é livre (extrair para experimentar).

**Pergunta em aberto**: o acto de promoção precisa de critérios formais? Provavelmente sim — caso contrário a fronteira erode.

---

## L5 — Laudo prospectivo: deixar legível em rewind

**v1.3 dizia**: o prompt é causa, o código é resultado. O ciclo de geração não pós-produz outro artefacto.

**Descoberta empírica**: a geração real tem **descobertas no caminho** — escolhas tácitas do agente, limitações encontradas, alternativas que se revelaram redundantes (ex.: descobrir que uma estrutura proposta já existia, ou que uma divergência da spec era desnecessária). Sem registo destas descobertas, o futuro leitor não consegue distinguir o que foi decidido com base no prompt do que foi descoberto durante a execução.

**Forma candidata para v1.4**: cada nucleação produz três artefactos, não dois.

- **Prompt** em L₀ — causa.
- **Código + testes** nos estratos correspondentes — materialização.
- **Laudo de execução** — registo curto do que aconteceu entre o prompt e o resultado: descobertas, alternativas rejeitadas in-flight, decisões tácitas, divergências da spec.

O laudo é **prospectivo**: escrito agora para ser lido por quem chegar depois. Não substitui o prompt; complementa-o.

**Pergunta crítica em aberto**: qual é a forma mínima de laudo? O typst-crystalline acumulou laudos extensos (relatórios consolidados, diagnósticos pre-implementação, inventários) que crescem além do prompt original. Encontrar o **laudo mínimo viável** é o ponto de optimização declarado pelo autor — o que preserva legibilidade prospectiva sem inflacionar a camada documental.

**Hipótese a testar**: o laudo mínimo viável tem 3 secções e cabe em meia página: *o que o prompt pediu*, *o que foi entregue*, *descobertas no caminho*.

---

## L6 — Geração isolada: combate ao vazamento de contexto

**v1.3 assumia**: O mesmo agente pode gerar o código e os testes simultaneamente no mesmo ciclo a partir da especificação, sem viés estrutural ou de informação.

**Descoberta empírica**: O mesmo agente gerando ambos sofre de **vazamento de contexto** (*context leakage*). O agente de IA gera testes enviesados pela implementação que ele próprio acabou de escrever, testando a "coincidência" e os atalhos de sua própria implementação e não a especificação estrita da Spec L₀ de forma agnóstica.

**Forma candidata para v1.4**: Designar **dois agentes autônomos e isolados** para a fase de materialização:

- **Agente A (Implementador)**: Recebe a Spec L₀ e gera exclusivamente o código nos estratos correspondentes.
- **Agente B (Testador)**: Recebe exclusivamente a Spec L₀ e gera os testes independentemente, sem enxergar de forma alguma o código gerado pelo Agente A.

O sucesso da clivagem é validado quando a união das materializações dos dois agentes compila e passa na suite de testes sob isolamento de caixa-preta.

**Nota (2026-07)**: o protocolo A/B tem uma propriedade adicional descoberta depois — é também um **oráculo de completude da spec**. Se o Agente B não consegue gerar testes apenas com o L₀, o prompt está subespecificado. O nucleation lock ganha um mecanismo de auditoria.

---

## L7 — Compressão: o recurso escasso é o contexto, não o código

**v1.3 assumia**: o problema é o crescimento amorfo; a estrutura é a resposta. A estrutura aparece como fim — preserva-se porque é o invariante.

**Descoberta empírica**: em três projectos independentes (typst-crystalline, crystalline-lint, lente), a estrutura revelou-se meio para um fim mais preciso: fazer o contexto certo caber na decisão do agente. A lente destila a frase que falta ao Manifesto: *comprimir o programa a uma forma que cabe na decisão*. O prompt é a unidade carregável endereçada por componente; o laudo mínimo é compressão da execução; a ADR é compressão da decisão; a DSM é compressão da forma; a granulação de passo (M1) é compressão do trabalho.

**Forma candidata para v1.4**: princípio da **Compressão** (ou Endereçamento): o lattice é um esquema de endereçamento de contexto para agentes estatísticos. Estrutura existe para que, a cada decisão, exactamente o contexto relevante seja carregável — nem mais, nem menos. M1, L5 e a legibilidade por agentes derivam deste princípio, não o precedem.

**Evidência viva**: README da lente ("comprime o programa a uma forma que cabe na decisão"; secção "Para agentes de IA" — JSON como contexto comprimido e verificável); hipótese do laudo mínimo (L5). Corroboração externa: ICM (arXiv:2603.16021) aplica o mesmo princípio à entrega de contexto em runtime, com orçamentos de tokens por camada.

---

## L8 — Pilha de verificação: quatro camadas, quatro classes de falha

**v1.3 dizia**: dois verificadores — o linter (estrutura) e o humano (fidelidade). L2 introduziu o oráculo executável em brownfield; ficou por formalizar a arquitectura completa de verificação.

**Descoberta empírica**: a verificação real opera em quatro camadas com custos e cadências distintas, cada uma apanhando uma classe de falha que as outras não apanham:

| Camada | O que verifica | Classe de falha | Cadência |
|--------|----------------|-----------------|----------|
| lint | forma legal | violação estrutural | contínuo (CI) |
| testes | spec declarada | divergência do prompt | por nucleação |
| oráculo | fidelidade ao substrato | divergência do original | por marco |
| humano | julgamento | o que nenhuma das três alcança | por ADR |

E o oráculo tem dois habitats que a prática usou sem distinguir: o **substrato legado** (o sistema original integral, read-only) e o **corpus oracular** (fixtures curados, mínimos, para medição diferencial).

**Forma candidata para v1.4**: declarar a pilha explicitamente, com cadência e classe de falha por camada. O oráculo executável torna-se cidadão do lattice — não um adendo brownfield — com os dois habitats nomeados: **substrato legado** (L₂₀) e **corpus oracular** (lab/ ou fixtures).

**Evidência viva**: `lab/parity/` (typst-crystalline); `oraculo/biteproof/` e fixtures v01–v14 (crystalline-lint); workflow `init-legado.md` (L₂₀ read-only).

---

## L9 — Gravidade fina: a ordem total dentro dos estratos

**v1.3 dizia**: gravidade entre estratos (L₄ → L₀). Sobre dependências intra-estrato, silêncio.

**Descoberta empírica**: a prática numerou fatias dentro de L1 — a lente tem `05_investiga`, `06_resolve`, `07_filtro`, `08_ranking`, `09_estrutura`, todos L1, com gravidade entre si (investiga → resolve). Os estratos são classes de equivalência grosseiras de uma ordem de estabilidade mais fina. E ciclos intra-camada emergem quando a semântica exige iteração: a introspecção eval↔layout do typst seria um ciclo entre fases se o contêiner `engine` não possuísse o loop.

**Forma candidata para v1.4**: **gravidade fina** — a numeração das directorias expressa ordem de estabilidade também dentro do estrato, verificável por DSM triangular-superior. Vocabulário intra-estrato:

- **fase** — unidade de transformação (parse, eval, layout)
- **contêiner** — módulo que possui a orquestração dos filhos e não transforma; **quebrador de ciclos**: o pai itera para que os filhos não se importem entre si
- **fachada** — o contêiner visto de fora: nome único para o fecho, ponto de entrada dos consumidores

**Evidência viva**: estrutura da lente (`01_core` + `05`–`09`, todos L1); separação engine/export no typst-crystalline (o contêiner como fecho do fixpoint de introspecção).

---

## L10 — Deriva vocabular: três conceitos, três resoluções, nenhum nome

**v1.3 assumia**: o vocabulário do lattice (L₀–L₄, lab) é suficiente para nomear o que a prática encontra.

**Descoberta empírica**: cada projecto resolveu conceitos não nomeados à sua maneira: laudos ("L5" no README da lente vs. `00_nucleo/lessons/` na prática), oráculo (`lab/typst-original/` vs. L₂₀ dos workflows vs. `oraculo/` do linter), estrutura intra-camada (fases do engine no typst vs. fatias numeradas na lente). Três projectos, três resoluções independentes — padrão de lacuna vocabular, não de indisciplina. Conceito sem nome resolve-se localmente, e cada resolução diverge.

**Forma candidata para v1.4**: consolidação vocabular — (a) L₀ tem **três classes de artefato por posição causal**: prompt (ex-ante), laudo (ex-post), ADR (transversal); o rótulo "L5" morre; (b) substrato legado ≠ corpus oracular (ver L8); (c) fase/contêiner/fachada como gramática intra-estrato (ver L9).

**Evidência viva**: README da lente ("L5 laudos" vs. laudos em `00_nucleo/lessons/`); CLAUDE.md do Tekt (`init-legado` → L₂₀); estrutura do crystalline-lint (`oraculo/` na raiz do repo).

---

## L11 — Alegações atestadas: a camada de oráculo precisa de recibo, não de adjetivo

**v1.3/L8 assumia**: a camada de oráculo (L8) verifica "fidelidade ao substrato" — implicitamente, isto significa que alguém corre uma comparação e reporta o resultado. O formato do laudo ficou em aberto.

**Descoberta empírica**: na frente de layout matemático do typst-crystalline (P885–P917), laudos que saltaram a medição fecharam passos que na verdade não estavam corrigidos. "Os delimitadores agora escalam adequadamente com o conteúdo" (P912) fechou o passo, e foram precisas mais duas rondas (P916, P917) a insistir na mesma tabela de `mutool trace` já usada em passos anteriores da mesma frente (P901, P905, P911) até a alegação ser remedida e confirmada ainda falsa para o caso comum (frações simples, matriz 2×2 — o glifo ficava fixo; só o conteúdo alto por acaso funcionava, via um mecanismo diferente e não relacionado). O modo de falha não foi desonestidade — foi que "medido" e "afirmado" produziram laudos textualmente indistinguíveis. Nada no formato do laudo forçava a distinção.

**Forma candidata para v1.4**: pedir emprestada a tríade de *Attested Computation* do Open Knowledge Format (OKF v0.2, §10) como forma obrigatória para qualquer alegação na camada de oráculo: **executor** (o comando/ferramenta exato que produz a medição — não "medido", mas `mutool trace <ficheiro>`), **recibo** (os campos exatos que o executor tem de devolver — glyph_id, advance, posição, não "cresceu corretamente"), **atestador** (comparação determinística do recibo contra o alvo, passa/falha — não "parece certo"). Um laudo que fecha uma alegação de camada de oráculo sem os três é *não atestado*, e uma alegação não atestada não pode fechar um passo, por mais confiante que seja a prosa. Isto refina L8, não o substitui: a camada de oráculo já existia; faltava a forma que separa alegação de prova.

**Evidência viva**: typst-crystalline P901/P905/P911 (atestado — tabelas de `mutool trace` com glyph IDs e posições reais, antes e depois); P912/P913/P914 (não atestado — alegações em prosa, sem recibo, mais tarde encontradas incompletas); P916/P917 (atestação retroativa, forçada por revisão, que foi o que de facto encontrou o bug remanescente). Fonte externa: Open Knowledge Format v0.2 §10 (`GoogleCloudPlatform/knowledge-catalog`), convergindo independentemente na mesma forma executor/recibo/atestador para corpora de conhecimento mantidos por agentes.

---

## Inversões secundárias — candidatas a Mecânica, não a Princípio

As lições acima reformulam Tekt no nível de Princípios. Há também observações mais finas, candidatas a Mecânica auxiliar:

### M1 — Granulação de passo como decisão tipada

O eixo "passo" não é uniforme. O typst-crystalline descobriu empiricamente que passos compõem-se em sub-passos (A/B/C/D/E/F/G) quando o trabalho real é não-uniforme, e que esta granulação é **decisão**, não detalhe. v1.4 pode formalizar uma lei de divisão: quando dividir, quando consolidar, quando agregar em série fechada.

### M2 — Diagnóstico-primeiro como padrão

Padrão recorrente: antes de prompt, inventário; antes de inventário, diagnóstico. Apareceu como "8ª aplicação do padrão diagnóstico-primeiro" em série específica. Em brownfield é quase obrigatório; em greenfield pode ser opcional.

### M3 — Ciclo de vida de ADR não é monotónico

v1.3 trata ADRs como decisões permanentes. O typst-crystalline mostrou três estados que v1.3 não previa:

- **superseded-by** — ADR substituída por outra mais nova
- **aceite estruturalmente** — aceite por força de eventos, antes de declaração formal
- **aceite retroactivo** — última condição empírica fecha após declaração estrutural

v1.4 precisa formalizar este ciclo de vida.

### M4 — Padrões de fase dentro de L₁

Algumas decisões arquiteturais não são sobre camadas, mas sobre **fases dentro de uma camada** — leitura mutável durante walk vs. leitura imutável post-walk, sub-stores trackable, sealing points. v1.4 pode catalogar padrões de "fases distintas dentro do mesmo estrato" sem promovê-los a estratos novos.

**Nota (2026-07)**: M4 ganhou vocabulário em L9 (fase / contêiner / fachada) e mecanismo (gravidade fina). A migração para a v1.4 deve fundir as duas entradas.

### M5 — Passos tipados

M1 pediu a lei de divisão; M2 observou o diagnóstico-primeiro. A forma madura: **o passo tem tipo**. Diagnóstico, arqueologia, nucleação, reconciliação e epitaxia têm laudos mínimos e verificações diferentes. A lei de divisão pedida em M1 emerge do tipo: divide-se quando o tipo muda, consolida-se quando o tipo se repete. Cada tipo declara o seu laudo mínimo e a sua verificação — diagnóstico verifica-se por inventário, nucleação por testes, reconciliação por isomorfismo, epitaxia por contenção do diff.

### M6 — Deriva onomástica

Em brownfield, o agente renomeia durante a regeneração ("melhoria" tácita de nomes), cortando a chave de junção com o oráculo: a cobertura passa a marcar falso-ausente onde existe port, e fantasma-novo onde existe renome. Regra candidata: **em brownfield, nomes são herança, não decisão** — o nome do substrato é o default; qualquer outro nome é divergência e exige ADR. Mecânica de reconciliação: inventário resolvido por grafo (não por texto), casamento estrutural (semente de matches exactos + propagação por vizinhança de arestas), e verificação da renomeação por isomorfismo de grafo módulo os ids renomeados. Evidência: correcções de nomes em curso no typst-crystalline; lente/tekt-cargo-dsm como instrumento de inventário e verificação.

---

## Espaço para crescer

Esta lista cresce. Cada vez que algo metodológico surpreender, regista-se aqui em forma destilada. O critério de entrada é simples: **se v1.3 não previa, e o projecto mostrou que era preciso, é candidato a v1.4**.

Quando uma entrada amadurece o suficiente, migra para o próximo Manifesto. O documento esvazia à medida que v1.4 absorve.

---

## Cross-references

- Manifesto Tekt v1.3 — `D:/Git/Tekt/MANIFESTO.pt.md`
- ADRs de divergência vanilla — ADR-0026, ADR-0033, ADR-0054, ADR-0075
- Diagnóstico-primeiro — séries P154A, P156B, P185A, P192A, P200A, P204A, P205A
- Ciclo de vida de ADR — ADR-0066 (superseded), ADR-0073 (aceite retroactivo), ADR-0074 (aceite final)
- Paridade observacional — `lab/parity/`, relatório consolidado P206D
- Gravidade fina e contêiner — README da lente (fatias `05`–`09` em L1); separação engine/export (typst-crystalline)
- Pilha de verificação — `oraculo/biteproof/` e fixtures v01–v14 (crystalline-lint); workflow `init-legado.md` (L₂₀)
- Deriva onomástica — correcções de nomes no typst-crystalline; mapa de reconciliação (a produzir com lente)
- Alegações atestadas — typst-crystalline P901/P905/P911 (atestado) vs. P912/P913/P914 (não atestado, corrigido depois por P916/P917); Open Knowledge Format v0.2 §10, `GoogleCloudPlatform/knowledge-catalog/okf/SPEC.md`
