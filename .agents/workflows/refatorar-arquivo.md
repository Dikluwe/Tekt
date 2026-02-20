---
description: Ciclo de refatoração Tekt por arquivo legado (Passos 2 a 4 com Rollback)
---

# Refatorar Arquivo (Ciclo Por Arquivo)

Este workflow é o **loop repetível** para cada arquivo listado no `LEGACY_MAP.md`. Execute-o uma vez para cada arquivo que deseja migrar do legado (L20) para a Arquitetura Cristalina.

## Pré-requisitos
- Workflow `/init-legado` já executado.
- `LEGACY_MAP.md` criado com a lista de arquivos.
- `.agentrules` carregado no workspace.

## Passos

### 1. Engenharia Reversa — Criar o L0 (Passo 2)
Selecione o próximo arquivo não marcado no `LEGACY_MAP.md` e cole o prompt:

> "Inicie a sua rotina de Arquiteto Cristalino.
> O nosso alvo atual de refatoração é o arquivo legado indicado. Pelo Invariante de Nucleação, você NÃO PODE reescrevê-lo nas pastas de L1-L4 ainda.
> **PASSO ÚNICO:** Leia criticamente o código fonte deste arquivo legado no workspace e faça a **Engenharia Reversa** de sua intenção.
>
> Você deve focar em gerar a 'Semente' documentada desta funcionalidade. Crie o arquivo EXATO em `00_nucleo/specs/<nome_da_feature>.md`.
>
> A sua Spec (Markdown) DEVE conter:
> 1. **Objetivo Central:** O que este arquivo antigo tentava fazer?
> 2. **Atomização da Lógica Pura (Para o futuro L1):** Quebre a lógica em suas **menores unidades possíveis (Funções Atômicas)**.
> 3. **Efeitos Colaterais Identificados (Para os futuros Contratos L3):** Liste exaustivamente todo I/O detectado.
> 4. **Glossário / Assinaturas (Opcional):** Documente as estruturas de dados principais.
>
> Apenas crie este arquivo de Spec no `L0` e os contratos correspondentes no `00_nucleo/contracts/`. Pare quando finalizar o L0. Marque `x` na checklist L0 do `LEGACY_MAP.md` e aguarde minhas instruções para o passo da Clivagem."

### 2. Auto-Revisão da IA (Passo 2.5)
Force a IA a auditar o próprio trabalho antes de você gastar tempo lendo:

> "Antes de prosseguirmos, faça uma auto-auditoria estrita da Spec que você acabou de criar. Reabra o arquivo legado original e compare-o exaustivamente com o Markdown do `L0` que você gerou.
> 1. Você ignorou alguma tratativa de erro original?
> 2. Você esqueceu de mapear alguma variável de ambiente oculta, leitura de args, chamadas de sistema ou I/O na lista de Colaterais?
> 3. A Lógica Pura elencada engloba 100% dos algoritmos subjacentes do arquivo velho?
> Se encontrar QUALQUER omissão, reescreva/atualize o arquivo Markdown e os Contratos L0 agora. Se estiver tudo completo, responda apenas: 'Spec Auditada e Completa'."

**Agora é sua vez (Auditoria Humana):** Leia o Markdown gerado em `00_nucleo/specs/`. Se encontrar defeitos:

> "Sua extração de Spec no `L0` falhou. Você ignorou a regra `[NOME]` que estava no código legado. Reabra o código fonte e ATUALIZE o L0 com esse pedaço que faltava."

Só avance quando a Spec estiver completa.

### 3. Clivagem — Criar L1, L2, L3 e L4 (Passo 3)

> "As specs estão formadas no `00_nucleo`. Leia-as.
> Agora ative as regras da Tekt para esse módulo:
> 1. Escreva a Lógica Pura (Sem I/O) no `01_core/`. **REGRA DE OURO: ATOMIZAÇÃO.** Funções minúsculas e de propósito único.
> 2. Qualquer regra de I/O identificada, implemente nas classes do `03_infra/`.
> 3. Crie os controladores/orquestradores de superfície no `02_shell/`.
> 4. Faça o Wiring final na sua composição.
> Ao terminar, confirme se a L0 e o Cabeçalho de Tipologia estão preenchidos. Eu ligarei a nova chave do L4."

### 4. Wiring Parcial — Integrar com o Legado (Passo 4)

> "A Clivagem Tekt foi um sucesso. Temos L1, L2, L3 e L4 isolados.
> **REGRA GERAL:** A pasta `L20` (Legado) é ESTRITAMENTE READ-ONLY. Você não tem permissão para editar nenhum arquivo lá dentro.
> **PASSO ÚNICO:** Vá até a nossa camada de composição `04_wiring` e crie o ponto de entrada (adapter/main) para testar este fluxo. Importe a nossa nova arquitetura (L2, L3) e conecte-a com o resto do sistema, importando as sobras DIRETAMENTE do módulo em `L20`.
> O objetivo é manter a Quarentena intocada enquanto o L4 estrangula o fluxo. Me avise quando o cargo compilar com sucesso."

Rode os testes. Se compilou e rodou, siga para o próximo arquivo do `LEGACY_MAP.md`.

### 🛑 Rollback (Se o Passo 4 falhar)
Se houver erros de compilação, NUNCA faça gambiarras no L4:

> "ABORTAR WIRING. Tivemos um erro de compilação. A peça Cristalina nova não casou com o Legado.
> **PASSO ÚNICO:** Pare o que está fazendo no `L4`. O problema nunca é o `L4`, o problema foi a nossa Extração. Leia o log de erro do Terminal/Cargo. Descubra qual função, tipo ou estrutura o legado requeria que NÓS esquecemos.
> Volte imediatamente ao `00_nucleo/specs` e `00_nucleo/contracts`. Adicione a especificação que faltou. Recrie a topologia (L1, L3) que falhou.
> NÃO crie workarounds no Wiring. Corrija o mal pela Semente.
> Acabe o L0 e o L1/L3 e só então me avise para tentarmos o Wiring de novo."

## Ciclo Completo
```
┌─────────────────────────────────────────────┐
│  Selecionar próximo arquivo do LEGACY_MAP   │
└──────────────────┬──────────────────────────┘
                   ▼
┌──────────────────────────────────────┐
│  Passo 2: Engenharia Reversa (L0)   │
└──────────────────┬───────────────────┘
                   ▼
┌──────────────────────────────────────┐
│  Passo 2.5: Auto-Revisão + Humano   │◄──┐
└──────────────────┬───────────────────┘   │
                   ▼                       │
┌──────────────────────────────────────┐   │
│  Passo 3: Clivagem (L1/L2/L3/L4)   │   │
└──────────────────┬───────────────────┘   │
                   ▼                       │
┌──────────────────────────────────────┐   │
│  Passo 4: Wiring Parcial + Testes   │   │
└──────────┬───────────────┬───────────┘   │
           │               │               │
        ✅ OK           ❌ Falhou ─────────┘
           │           (Rollback p/ L0)
           ▼
┌──────────────────────────────────────┐
│  Marcar [x] no LEGACY_MAP e repetir │
└──────────────────────────────────────┘
```
