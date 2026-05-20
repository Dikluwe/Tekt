---
description: Passo 3 da Refatoração Tekt - Clivagem e Construção (L1/L2/L3)
---

# /clivar-modulo

Este comando utiliza a Spec documentada em `00_nucleo/specs` para reconstruir a funcionalidade nas camadas cristalinas sob a disciplina do **Regime do Sistema** (Tekt v1.4), garantindo a separação estrita de lógica e efeitos.

## 💎 O Prompt de Clivagem (v1.4)

> "As specs e contratos estão formados no `00_nucleo`. Leia-os.
> Agora, execute a **Clivagem** seguindo as regras da Tekt para este módulo:
> 
> 1. **L1 (Core):** Escreva a Lógica Pura (Sem I/O) no `01_core/`. **REGRA DE OURO DA ATOMIZAÇÃO:** Não repita as funções gigantes e emaranhadas do legado! Você deve quebrar a matemática e os algoritmos crus do `20_lab` em funções puras, isoladas e de **propósito único**. Use o código original em `20_lab` como referência constante para garantir a precisão matemática, mas NÃO COPIE a topologia/estrutura velha.
> 2. **L2 (Application):** Crie os orquestradores e serviços de superfície no `02_shell/`.
> 
> **ATENÇÃO ÀS REGRAS ESTritas DE CLIVAGEM (v1.4):**
> - **O LEGADO É SAGRADO (READ-ONLY):** Em NENHUMA hipótese ou situação você pode editar os arquivos dentro de `20_lab`. Ele apenas fornece a referência matemática.
> - **PROIBIDA A COMPILAÇÃO IMEDIATA:** Não tente forçar o projeto a compilar executando `cargo build` ou ferramentas similares enquanto constrói L1-L3. A integração só ocorre no L4 (Fluxo de Integração). 
> - **ROLLBACK DE ERROS NA CLIVAGEM:** Se perceber, durante a escrita do código, que o legado possui dependências ou estruturas matemáticas complexas que a Spec `L0` não cobriu, você DEVE interromper a construção L1-L3, reportar o erro ao usuário, e sugerir voltar para o `00_nucleo/specs`.
> - **ORÁCULO DE FIDELIDADE (L2):** Avalie a paridade funcional contra o oráculo (a referência executável original). Atribua e documente a classificação de paridade: `implementado`, `implementado⁺`, `parcial`, `ausente` ou `scope-out`.
> - **LAUDO DE EXECUÇÃO PROSPECTIVO (L5):** Ao final do processo de clivagem, escreva o laudo mínimo viável no rodapé do arquivo de Spec com as seções: *O que o prompt pediu*, *O que foi entregue*, e *Descobertas no caminho (divergências ou decisões tácitas)*.
> 
> Ao terminar de escrever as funções de L1-L3, confirme se a L0, os cabeçalhos de tipologia e o Laudo de Execução estão preenchidos. Me avise para prosseguirmos ao Wiring."
