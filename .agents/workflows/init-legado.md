---
description: Inicialização única do projeto legado para refatoração Tekt (Passos 1 e 1.5)
---

# Init Legado (Setup Único)

Este workflow é executado **uma única vez** no início da refatoração de um projeto legado para a Arquitetura Cristalina (Tekt).

## Pré-requisitos
- Repositório do projeto legado clonado localmente.
- Arquivo `init-tekt.sh` disponível (do repositório Tekt).
- Arquivo `.agentrules` copiado para a raiz do projeto alvo.

## Passos

### 1. Armar o Container Cristalino (Passo 1)
// turbo
1. Rode o script de infraestrutura na raiz do projeto legado:
```bash
bash init-tekt.sh
```
Isto criará as pastas `00_nucleo` a `04_wiring` vazias.

2. Mova todo o código fonte legado para a pasta `20_lab/` (Quarentena):
```bash
mkdir -p 20_lab
# Mova pastas de código (ex: crates/, src/, tests/, docs/) para 20_lab/
```

3. Ajuste o arquivo de build da raiz (`Cargo.toml`, `package.json`, etc.) para apontar os caminhos para `20_lab/`. Confirme que o projeto compila normalmente após a movimentação.

4. Faça o commit de checkpoint:
```bash
git add . && git commit -m "chore: isolamento topológico do legado para arena L20"
```

### 2. Criar o Mapa de Demolição (Passo 1.5)
Cole o seguinte prompt para o Agente de IA:

> "Sua primeira tarefa não é alterar código, mas mapear o legado.
> Analise a pasta selecionada (ex: `20_lab/crates/typst-syntax/src`).
> Crie no diretório raiz do nosso projeto um novo arquivo `LEGACY_MAP.md`. Nele, liste em formato de Markdown checkboxes todos os arquivos `.rs` achados nessa pasta.
> Mas **ATENÇÃO**, abaixo de CADA arquivo `.rs` listado, você deve adicionar UMA única sub-checkbox indentada informando o status da Engenharia Reversa inicial:
> `  - *↳ [ ] L0 (Spec Criada)*`
> Apenas crie este arquivo de checklist e pare."

5. Commit do mapa:
```bash
git add LEGACY_MAP.md && git commit -m "docs: criado LEGACY_MAP.md com checklist de migração"
```

## Resultado Esperado
- Pastas `00_nucleo` a `04_wiring` vazias e prontas.
- Código legado intacto em `20_lab/`.
- Build funcional apontando para `20_lab/`.
- `LEGACY_MAP.md` com a lista de todos os arquivos a migrar.

**Próximo passo:** Use o workflow `/refatorar-arquivo` para cada arquivo listado no `LEGACY_MAP.md`.
