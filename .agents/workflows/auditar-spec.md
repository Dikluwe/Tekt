---
description: Passo 2.5 da Refatoração Tekt - Auto-Revisão e Auditoria do L0
---

# /auditar-spec

Este comando obriga a IA a realizar uma **Auditoria Estrita** na Especificação L0 que acabou de ser gerada, garantindo que nenhum detalhe do código legado tenha sido perdido antes da clivagem.

## 👁 O Prompt de Auto-Revisão

Após a execução do `/gerar-spec`, envie:

> "Antes de prosseguirmos, faça uma auto-auditoria estrita da Spec L0 que você acabou de criar. Reabra o arquivo legado original e compare-o exaustivamente com o Markdown do `L0` gerado. 
> 
> 1. Você ignorou alguma tratativa de erro original? 
> 2. Você esqueceu de mapear alguma variável de ambiente oculta, leitura de args, chamadas de sistema ou instâncias de I/O na lista de Colaterais (L3)? 
> 3. A Lógica Pura (L1) elencada engloba 100% dos algoritmos subjacentes do arquivo velho?
> 
> Se encontrar QUALQUER omissão, reescreva/atualize o arquivo Markdown e os Contratos L0 agora cobrindo essa falha. 
> Se você tiver certeza absoluta que o L0 mapeia toda a intenção do código legado sem perdas, responda apenas: 'Spec Auditada e Completa'."

---

## 🛑 Prompt de Correção Humana (Fallback)

Se, após a auto-revisão da IA, você (Engenheiro) ler a Spec e encontrar falhas na extração, use este prompt para forçar a correção:

> "Sua extração de Spec no `L0` falhou. Você ignorou a regra `[NOME_DA_REGRA_OU_FUNÇÃO]` que estava no código legado. Reabra o código fonte, estude a parte que você perdeu e ATUALIZE o arquivo Markdown e os Contratos no L0 com esse pedaço que faltava."
