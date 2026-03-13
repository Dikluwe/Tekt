---
description: Passo 4 da Refatoração Tekt - Wiring Parcial e Rollback
---

# /integrar-legado

Este comando realiza a composição final no `04_wiring`, conectando as novas camadas cristalinas com o restante do sistema legado em `20_lab`.

## 🔗 O Prompt de Wiring Parcial

> "A Clivagem Tekt foi um sucesso. Temos L1, L2, L3 e L4 isolados.
> 
> **REGRA DE OURO:** A pasta `20_lab` (Legado) é ESTRITAMENTE **READ-ONLY**. Você não tem permissão para editar nenhum arquivo lá dentro para importar nossos módulos novos.
> 
> **PASSO ÚNICO:** Vá até a nossa camada de composição `04_wiring` e crie o ponto de entrada (adapter/main) para integrar este fluxo. 
> 
> Neste arquivo de Wiring, importe a nossa nova arquitetura (L2, L3) e conecte-a com o resto do sistema que ainda reside na quarentena, importando as sobras DIRETAMENTE do módulo em `20_lab`.
> 
> O objetivo é manter a Quarentena intocada enquanto o L4 estrangula o fluxo e orquestra a transição híbrida. Me avise quando o cargo compilar este novo Wiring com sucesso."

---

## 🛑 Gestão de Falhas (A Regra do Rollback)

Se a compilação falhar no Wiring, **NÃO faça gambiarras no L4**. Use o prompt abaixo:

> "ABORTAR WIRING. Tivemos um erro de compilação ou regressão. A peça Cristalina nova não casou com o Legado.
> 
> **PASSO ÚNICO:** Pare o que está fazendo no `L4`. O problema foi a nossa Extração L0. Leia o log de erro do terminal. Descubra qual contrato ou tipo o legado requeria que esquecemos de mapear.
> 
> Volte imediatamente ao `00_nucleo/specs` e `00_nucleo/contracts`. Adicione a especificação que faltou. Recrie a topologia (L1 a L3) que falhou em implementar isso.
> 
> NÃO crie workarounds no Wiring. Corrija a Semente."
