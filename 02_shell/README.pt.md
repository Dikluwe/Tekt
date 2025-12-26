### 2. README.pt.md (Versão em Português)

# /02_shell — A Superfície

> **A Face do Cristal.** Tudo que é tocável externamente.

## Propósito

Este diretório contém **Adaptadores Primários**: componentes que recebem entrada do mundo externo (Usuários, Requisições HTTP, Comandos CLI) e as traduzem em chamadas para o domínio core.

---

## 💎 Formalismo Matemático ($\mathcal{L}_2$)

Para manter a integridade do core enquanto interage com ambientes de alta entropia, a Shell segue o **Morfismo de Tradução**:

* **Projeção de Interface**: Seja $W$ o Mundo (Dados Externos) e $C$ o Domínio Core. A Shell é uma função de mapeamento $f: W \to C$.
* **Garantia de Tipos**: A shell deve garantir que para cada entrada externa $x \in W$, a transformação $f(x)$ resulte em um objeto de domínio válido $y \in L_1$.
* **Invariante de Desacoplamento**: A Shell é proibida de interagir com a camada de Infraestrutura $L_3$.
$$\text{dep}(L_2) \cap L_3 = \emptyset$$
* **Composição**: A Shell interage com $L_1$, mas é montada/composta pela camada $L_4$ (Wiring).

---

## O Que Vive Aqui

* 🖥️ **Componentes de UI**: React, Vue, Svelte ou views em JS puro.
* 🌐 **Controllers de API**: Endpoints REST e resolvers GraphQL.
* ⌨️ **Interfaces de CLI**: Manipuladores de argumentos de linha de comando.

---

## Regras de Dependência

> [!IMPORTANT]
> **PODE importar**: `01_core` (para usar a lógica de domínio).
> **NÃO PODE importar**: `03_infra` (para evitar acoplamento direto com DB/Rede).

* ✅ `02_shell`  `01_core`
* ❌ `02_shell`  `03_infra` (A infraestrutura deve ser injetada via `04_wiring`).

## Protocolo de IA (Auditoria de Isomorfismo)

1. **Validação de Entrada**: A IA deve garantir que todos os dados externos sejam validados antes de tocar em .
2. **UI Sem Estado**: Componentes de interface devem ser o mais funcionais possível, delegando a lógica ao Core.
3. **Sem I/O Direto**: A IA não deve gerar chamadas de `fetch` ou `sql` dentro deste diretório.

---

### Exemplo

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/user-registration.md
 */

// ✅ CORRECT - Maps external interaction to Core logic
import { validateUser } from '../../01_core/domain/user-logic';

// ❌ WRONG - Direct Infrastructure access
// import { db } from '../../03_infra/persistence'; // FORBIDDEN!

export function RegistrationController(req, res) {
  // Logic flows from Shell to Core
  const isValid = validateUser(req.body);
  // Implementation continues...
}

```
