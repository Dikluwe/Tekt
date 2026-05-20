---
description: Passo 2 da Refatoração Tekt - Engenharia Reversa (L0)
---

# /gerar-spec

Este comando realiza a **Engenharia Reversa** de um arquivo ou módulo legado sob o **Modo Brownfield** (Tekt v1.4), destilando sua intenção e lógica para a camada `00_nucleo/specs` por meio de uma operação de **inferência reversa** (*arqueologia/exegese* do código legado).

## 🧪 O Prompt de Engenharia Reversa (Modo Brownfield)

Selecione o arquivo alvo no `20_lab/` e envie:

> "Inicie a sua rotina de Arquiteto Cristalino em **Modo Brownfield**.
> O nosso alvo atual de refatoração é o arquivo legado indicado. Pelo Invariante de Nucleação, você NÃO PODE reescrevê-lo nas pastas de L1-L4 ainda.
> 
> **PASSO 1: Diagnóstico e Inventário-Primeiro (M2):** Antes de gerar a spec, faça um inventário rápido do arquivo legado: linhas de código, complexidade percebida, dependências e dependentes diretos dentro do sistema.
> 
> **PASSO 2: Inferência Reversa (L1):** Leia criticamente o código fonte legado no workspace para reconstruir o L₀ que ele *teria tido* se fosse greenfield. Crie o arquivo de Spec em `00_nucleo/specs/<nome_da_feature>.md`.
> 
> A sua Spec (Markdown) DEVE conter:
> 1. **Objetivo Central:** O que este arquivo antigo tentava fazer?
> 2. **Atomização da Lógica Pura (L1):** Destrinche o arquivo antigo em suas menores unidades funcionais (Funções Atômicas).
> 3. **Efeitos Colaterais Identificados (L3):** Liste I/O, chamadas de sistema, variáveis de ambiente ou dependências externas.
> 4. **Glossário / Assinaturas:** Documente as structs, enums e assinaturas principais.
> 5. **Bases para o Oráculo de Fidelidade (L2):** Descreva o perfil observacional esperado do código e como medir a equivalência funcional vs. a referência executável legada (ex.: testes diferenciais, valores de tolerância).
> 6. **Template de Laudo de Execução (L5):** Reserve uma seção de histórico/laudo no rodapé contendo: *O que o prompt pediu*, *O que foi entregue*, e *Descobertas no caminho (decisões tácitas/divergências)*.
> 
> Apenas crie o L0 e os contratos necessários. Pare ao finalizar. Marque `x` no `LEGACY_MAP.md`."

## 🔍 Auto-Auditoria (Passo 2.5)

Após a geração, force a revisão:

> "Faça uma auto-auditoria estrita da Spec criada. Compare-a com o arquivo legado original.
> 1. Alguma tratativa de erro foi ignorada?
> 2. Variáveis de ambiente ou I/O ocultos foram mapeados?
> 3. A Lógica Pura engloba 100% dos algoritmos?
> Se houver omissão, corrija a Spec. Se estiver perfeito, responda: 'Spec Auditada e Completa'."
