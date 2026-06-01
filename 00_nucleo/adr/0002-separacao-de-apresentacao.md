# ADR-0002: Apresentação como responsabilidade exclusiva de L2

**Status**: `ACEITO`
**Data**: 2026-05-25

---

## Contexto

A definição de L₂ no manifesto v1.3 enumera o que a Casca faz no sentido
entrada→Núcleo ("validação de entrada, orquestração, adaptação de formatos") mas
deixa implícita a responsabilidade no sentido inverso: o que acontece com a saída do
Núcleo quando ela precisa virar texto numa CLI, cor num terminal, resposta HTTP,
mensagem de um servidor LSP para um cliente.

A falta de declaração explícita produziu, em projeto-lente derivado deste framework,
um sintoma reproduzível: strings de apresentação aparecem misturadas à lógica de
orquestração dentro de handlers e controllers de L₂. O código continua na camada
correta, mas a apresentação fica codificada inline, sem ponto único de troca, sem
catálogo, sem possibilidade de internacionalização ou tematização sem reescrever a
orquestração.

Pela regra de inclusão de `LESSONS.pt.md`, esta descoberta não é uma inversão face a
v1.3 — é um refino do que L₂ já era. Refinos vivem em ADRs, não em LESSONS. Esta ADR
fecha o refino.

---

## Decisão

Os artefatos de apresentação — textos visíveis ao usuário, cores, formatos de saída,
estruturas de mensagens externas — pertencem exclusivamente a L₂.

A regra opera em três níveis:

**Responsabilidade.** Nenhum outro estrato participa de decisões de apresentação. L₁
não conhece textos, cores ou formatos. L₃ não escolhe o que apresentar, apenas
executa I/O do que L₂ entrega. L₄ apenas conecta componentes; não decide conteúdo.

**Externalização interna a L₂.** Dentro de L₂, apresentação é separada da orquestração.
Handlers, controllers e resolvers referenciam um catálogo de apresentação (módulo de
strings, tabela de cores, dicionário de formatos). Strings, códigos de cor e formatos
não aparecem inline no fluxo de execução.

**Localização do catálogo.** O catálogo é dado de L₂, não I/O. Regra operacional:
se a definição do catálogo não exige operação de I/O em tempo de execução, mora em L₂.
Se exigir (carregamento de arquivo, leitura de variável de ambiente, fetch remoto), o
carregamento é tarefa de L₃ — mas o conteúdo, depois de carregado, permanece domínio
de L₂.

A regra se aplica uniformemente a qualquer canal externo: CLI, GUI, resposta HTTP,
resposta GraphQL, mensagens LSP do servidor para o cliente, logs estruturados
destinados a consumo humano.

---

## Prompts Afetados

| Prompt | Natureza da mudança |
|--------|---------------------|
| MANIFESTO.pt.md | Seção L₂ (Casca) — refino para declarar explicitamente a responsabilidade de apresentação. |
| MANIFESTO.md | Seção L₂ (Shell) — refino equivalente em inglês. |
| 02_shell/README.pt.md | Acompanhamento futuro: incorporar a regra na lista "O que pertence aqui". |
| 02_shell/README.md | Acompanhamento futuro: idem, em inglês. |

---

## Consequências

**Positivas**: clarifica a fronteira de L₂ no sentido saída; impede o sintoma
diagnosticado de orquestração e apresentação amalgamadas dentro do mesmo handler;
permite internacionalização, tematização e troca de canal de apresentação sem editar
lógica de orquestração; alinha CLI, GUI, HTTP e LSP sob o mesmo princípio.

**Negativas**: introduz um nível de indireção (catálogo + referência) onde antes havia
string literal; exige disciplina contínua para não voltar a inserir strings inline
sob pressão. O linter pode ajudar (regra: literais de texto destinados ao usuário
não podem aparecer fora de módulos de catálogo), mas a implementação dessa regra é
trabalho separado.

**Neutras**: formaliza um padrão que muitos projetos adotam por hábito; não muda a
relação entre L₂ e os outros estratos, apenas explicita o que L₂ já era.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Manter implícito (status quo v1.3) | Sem mudança documental | Reproduz o sintoma diagnosticado em outros projetos do framework |
| Tratar apresentação como I/O e mover para L₃ | Simetria com persistência e rede | Errado: apresentação é decisão (o quê dizer), não efeito (como entregar); apenas a entrega física é I/O |
| Criar princípio novo "Separação de Apresentação" ao lado dos cinco existentes | Visibilidade alta no manifesto | Infla a lista de princípios para acomodar um refino de camada; princípios são para invariantes do lattice, não para responsabilidades de uma camada específica |
| Registrar em `LESSONS.pt.md` como descoberta candidata a v1.4 | Coerente com como outras descobertas estão registradas | Viola a regra de inclusão do próprio LESSONS: "refinos vivem em ADRs locais" |
| Refino em L₂ do MANIFESTO via ADR (escolhida) | Respeita a regra de LESSONS; documenta a decisão com rastreabilidade; muda o manifesto sem inflar princípios | Exige edição do manifesto em duas línguas |

---

## Referências

- `MANIFESTO.pt.md` — seção L₂ (Casca)
- `MANIFESTO.md` — seção L₂ (Shell)
- `LESSONS.pt.md` — regra de inclusão (refinos vão para ADRs)
- `00_nucleo/adr/template-adr.md` — formato seguido
