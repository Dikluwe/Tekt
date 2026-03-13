---
description: Passo 2 da Refatoração Tekt - Engenharia Reversa (L0)
---

# /gerar-spec

Este comando realiza a **Engenharia Reversa** de um arquivo ou módulo legado, destilando sua intenção e lógica para a camada `00_nucleo/specs`.

## 🧪 O Prompt de Engenharia Reversa

Selecione o arquivo alvo no `20_lab/` e envie:

> "Inicie a sua rotina de Arquiteto Cristalino.
> O nosso alvo atual de refatoração é o arquivo legado indicado. Pelo Invariante de Nucleação, você NÃO PODE reescrevê-lo nas pastas de L1-L4 ainda.
> **PASSO ÚNICO:** Leia criticamente o código fonte deste arquivo legado no workspace e faça a **Engenharia Reversa** de sua intenção.
> 
> Você deve focar em gerar a 'Semente' documentada desta funcionalidade. Crie o arquivo EXATO em `00_nucleo/specs/<nome_da_feature>.md`.
> 
> A sua Spec (Markdown) DEVE conter:
> 1. **Objetivo Central:** O que este arquivo antigo tentava fazer?
> 2. **Atomização da Lógica Pura (L1):** Destrinche o arquivo antigo em suas menores unidades funcionais (Funções Atômicas).
> 3. **Efeitos Colaterais Identificados (L3):** Liste I/O, chamadas de sistema ou dependências externas.
> 4. **Glossário / Assinaturas:** Documente as structs e enums principais.
> 
> Apenas crie o L0 e os contratos necessários. Pare ao finalizar. Marque `x` no `LEGACY_MAP.md`."

## 🔍 Auto-Auditoria (Passo 2.5)

Após a geração, force a revisão:

> "Faça uma auto-auditoria estrita da Spec criada. Compare-a com o arquivo legado original.
> 1. Alguma tratativa de erro foi ignorada?
> 2. Variáveis de ambiente ou I/O ocultos foram mapeados?
> 3. A Lógica Pura engloba 100% dos algoritmos?
> Se houver omissão, corrija a Spec. Se estiver perfeito, responda: 'Spec Auditada e Completa'."
