### 2. README.pt.md (Versão em Português)

# /00_nucleo — A Semente

> **A Fonte da Verdade.** Ponto zero da cristalização.

## Propósito

Este diretório contém o **material genético** do projeto: especificações, regras de negócio, decisões de arquitetura e contratos de interface.

---

## 💎 Formalismo Matemático ($\mathcal{L}_0$)

Para garantir a integridade estrutural e minimizar a entropia, o Núcleo segue estas restrições formais:

* **Axiomatização**: Seja $S$ o conjunto de Especificações $s \in \{specs, contracts, adr\}$.
* **Invariante de Nucleação**: A existência de qualquer arquivo de implementação $c \in \{01, 02, 03, 04\}$ exige um mapeamento prévio para uma especificação .
$$\forall c \in C, \exists s \in S : P(c, s)$$

* **Propriedade do Ínfimo**: $00\_nucleo$ é o ínfimo do conjunto parcialmente ordenado de dependências $(X, \le)$. Nenhuma camada pode existir abaixo dela.
* **Isomorfismo Estrutural**: A implementação deve ser uma representação isomórfica da estrutura definida nos contratos $s$.

---

## A Trava de Nucleação

> [!CAUTION]
> **Sem especificação aqui = Nenhum código pode ser escrito.**

Antes de QUALQUER funcionalidade ser implementada de `01_core` até `04_wiring`, um documento correspondente DEVE existir neste diretório.

## Estrutura de Diretórios

```
00_nucleo/
├── specs/           # Especificações do recurso
├── contracts/       # Contratos e tipos de interface
└── adr/             # Registros de decisões de arquitetura
```

* **specs/**: Especificações de funcionalidades, regras de negócio e histórias de usuário.
* **contracts/**: Contratos de interface, tipos TypeScript e schemas de API.
* **adr/**: Registro de Decisões de Arquitetura (Formato: `NNNN-titulo-com-hifens.md`).

## Regras

1. **Leitura Prévia**: A IA deve ler este diretório antes de qualquer geração de código para alinhar-se ao "material genético".
2. **Aprovação Humana**: Todas as especificações exigem validação manual para evitar requisitos "alucinados".
3. **Rastreabilidade**: Cada arquivo de código deve rastrear até um documento específico neste diretório.
4. **Imutabilidade**: Especificações aprovadas são imutáveis; mudanças exigem um novo ADR ($$\Delta s \iff \exists adr_{new}$$).

## Links de Templates

* [Template de ADR](./adr/template.md)
* [Template de Spec](./specs/.template.md)
* [Template de Contrato](./contracts/.template.md)

---
