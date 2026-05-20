# Lições para Tekt v1.4

**Estado**: VIVO — cresce em paralelo aos passos do typst-crystalline.

**Origem**: destilação de inversões metodológicas descobertas durante a refatoração de typst (typst-crystalline) com base no Manifesto Tekt v1.3.

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
