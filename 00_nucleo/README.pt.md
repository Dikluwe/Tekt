# /00_nucleo — A Semente

> O registro causal do sistema. Todo código gerado por IA deve poder ser rastreado até um prompt aqui.

---

## Propósito

Este diretório contém os **prompts estruturados** que originaram cada componente do sistema, e os **ADRs** que documentam decisões que afetam múltiplos componentes ou a estrutura global.

O prompt não descreve o código — ele é a origem do código. A interface que um componente implementa, a forma que ele tem, as restrições que respeita: tudo isso é consequência direta do prompt que o gerou.

Um componente sem prompt correspondente em `00_nucleo` é estruturalmente ilegítimo — não porque falta documentação, mas porque sua origem é irrastreável. Não é possível reproduzir, auditar ou evoluir com consistência o que não tem origem registrada.

---

## Trava de Nucleação

> **Sem prompt aqui → nenhum código pode ser gerado.**

Antes de qualquer geração de código em `01_core`, `02_shell`, `03_infra` ou `04_wiring`, um prompt correspondente deve existir neste diretório.

Quando um agente de IA recebe uma tarefa, sua primeira ação é verificar se existe um prompt em `00_nucleo/prompts/` para o componente em questão:
- **Existe** → ler o prompt, usar como contexto, gerar ou modificar o código
- **Não existe** → parar e solicitar ao desenvolvedor que o prompt seja criado primeiro

---

## Estrutura

```
00_nucleo/
├── prompts/    # Um arquivo por componente ou feature
└── adr/        # Decisões que afetam múltiplos prompts ou o lattice
```

**`prompts/`** — Cada arquivo é o prompt estruturado que gerou um componente. Inclui contexto, restrições, a instrução em si, o resultado esperado e o histórico de revisões.

**`adr/`** — Registra decisões arquiteturais que transcendem um único componente: mudanças no lattice, adoção de nova tecnologia em L₃, redefinição de fronteiras entre estratos, alterações em restrições globais.

---

## O Prompt como Contrato

O contrato de interface de um componente não é um documento separado — é o output do prompt que o gerou. O arquivo de interface em `01_core` gerado pelo prompt **é** o contrato. `00_nucleo` guarda a origem, não uma descrição prévia do resultado.

Isso colapsa três artefatos que arquiteturas tradicionais mantêm separados — spec, contrato e código — em dois: prompt e código. A relação entre eles é causal e direta.

---

## Evolução

Quando um componente precisa mudar, o prompt muda junto. A revisão é registrada no histórico do arquivo de prompt com data, motivo e arquivos afetados. O código modificado continua rastreável à sua origem.

ADRs são criados apenas quando a mudança afeta decisões que transcendem o componente — não para cada modificação de código.

---

## Regras

1. **Sem código executável** — apenas prompts `.md` e ADRs.
2. **Um prompt por componente** — granularidade suficiente para rastreabilidade, sem fragmentação excessiva.
3. **Aprovação humana** — prompts requerem revisão antes de orientar geração.
4. **Histórico obrigatório** — toda revisão de prompt registra data, motivo e arquivos afetados.

---

## Templates

- [Template de Prompt](./prompts/template.md)
- [Template de ADR](./adr/template.md)
