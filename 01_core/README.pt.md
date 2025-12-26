### 2. README.pt.md (Versão em Português)

# /01_core — O Cristal Puro

> **Lógica Platônica.** O coração das regras de negócio puras.

## Propósito

Este diretório contém **lógica de domínio pura**: entidades, algoritmos, funções matemáticas e regras de negócio com **absolutamente nenhum I/O**.

---

## 💎 Formalismo Matemático ($\mathcal{L}_1$)

Para garantir um núcleo determinístico, esta camada é definida como uma coleção de **Morfismos Puros**:

* **Pureza ($\mathcal{P}$)**: Toda função $f$ em $L_1$ deve ser uma função pura.
$$\forall x \in X, \forall t \in T : f(x, t) = f(x)$$
(O resultado depende exclusivamente da entrada, independente do tempo do sistema  ou estado externo).
* **Isolamento de Efeitos Colaterais**: O conjunto de efeitos colaterais $\mathcal{E}$ para qualquer operação nesta camada deve ser vazio.
$$\text{EfeitosColaterais}(L_1) = \emptyset$$
* **Determinismo Sem Estado**: Para qualquer estado $S$ e entrada $I$, a função de transição $\delta$ deve ser um mapeamento determinístico: $\delta: S \times I \to S'$.

---

## A Regra de Zero I/O

> [!CAUTION]
> **RESTRIÇÃO ABSOLUTA**
> O código neste diretório **NÃO DEVE**:
> * Acessar bancos de dados ou fazer requisições de rede.
> * Ler/escrever arquivos ou acessar o relógio do sistema.
> * Importar bibliotecas externas (apenas a biblioteca padrão da linguagem).
> 
> 

## Permitido

✅ **Funções puras** e estruturas de dados imutáveis.
✅ **Entidades de Domínio** e validação de regras de negócio.
✅ **Interfaces** (Definições abstratas) para dependências externas.
✅ **Algoritmos matemáticos** e computações sem estado (*stateless*).

## Estrutura de Diretórios

```
01_core/
├── entities/        # Entidades de domínio (Models, Value Objects)
├── algorithms/      # Algoritmos puros (Matemática, Transformações)
└── domain/          # Regras de negócio (Validadores, Serviços Puros)

```

## Regra de Dependência

* **Pode Importar**: `00_nucleo` (para implementar contratos e especificações).
* **Proibido**: `02_shell`, `03_infra`, `04_wiring`, `_lab`.

---

### Exemplo de Auditoria 
/**
 * Linhagem Cristalina
 * @spec 00_nucleo/specs/validacao-usuario.md
 */

// ✅ CORRETO - Morfismo Puro f: string -> boolean
export function validarEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

// ❌ ERRADO - Violação de Efeito Colateral (I/O Externo)
// import axios from 'axios';

---
