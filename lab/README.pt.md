# /_lab — A Arena

> Zona de quarentena. Experimentos vivem e morrem aqui.

---

## Propósito

Este estrato existe para código exploratório, protótipos e experimentos que ainda não satisfazem os invariantes do sistema principal. É um espaço de alta entropia deliberadamente isolado.

---

## Regras

**Dentro da Arena:**
- Código não precisa de linhagem completa
- Regras de pureza e isolamento são relaxadas
- Experimentos podem referenciar specs de `00_nucleo` para orientação
- Nenhuma restrição de I/O ou dependências externas

**Fora da Arena:**
- Nenhum estrato do sistema principal pode importar de `_lab`
- Código da Arena não é promovido diretamente ao sistema principal

---

## Migração para o Sistema Principal

Para que código da Arena migre para `01_core`, `02_shell` ou `03_infra`, ele deve ser **inteiramente reescrito** satisfazendo:

1. Linhagem rastreável até `00_nucleo`
2. Todas as regras de dependência do estrato de destino
3. Cabeçalho `@spec` apontando para documento existente

Copiar e colar da Arena para o sistema principal é uma violação estrutural. A reescrita é obrigatória.

---

## Estrutura sugerida

```
_lab/
├── experiments/   # Experimentos pontuais
├── spikes/        # Investigações técnicas de prazo curto
└── prototypes/    # Protótipos funcionais para validação
```

---

## Quando usar a Arena

- Investigar se uma biblioteca resolve o problema antes de comprometer com ela
- Validar uma hipótese de implementação sem contaminar o sistema
- Prototipar rapidamente para testar com usuários
- Explorar refatorações antes de executá-las no sistema principal
