### 2. README.pt.md (Versão em Português)

# /03_infra — A Fundação

> **O Suporte do Cristal.** Efeitos colaterais controlados e persistência externa.

## Propósito

Este diretório contém **Adaptadores Secundários**: implementações das interfaces definidas no Core. Ele lida com o trabalho "sujo" de interagir com o mundo físico (Bancos de Dados, APIs Externas, Sistemas de Arquivos).

---

## 💎 Formalismo Matemático ($\mathcal{L}_3$)

Diferente do Core, a camada de Infraestrutura admite a existência de estado e efeitos colaterais, mas os sujeita à **Realização de Interface**:

* **Realização de Efeito**: Seja $I \subset L_1$ um conjunto de interfaces abstratas. A Infraestrutura fornece um conjunto de implementações concretas $M$ tal que existe um morfismo de realização $r: M \to I$.
* **Encapsulamento de Efeito**: Seja $\mathcal{E}$ o conjunto de efeitos colaterais. Embora $\text{SideEffects}(L_3) \neq \emptyset$, esses efeitos devem ser contidos nos limites da implementação, nunca vazando seus tipos internos para $L_1$.
* **Inversão de Dependência**: $L_3$  depende de $L_1$ para saber *o que* implementar, mas $L_1$ nunca sabe *como* $L_3$ funciona.
$$L_3 \implies L_1$$

---

## O Que Vive Aqui

* 🗄️ **Repositórios**: Implementações de banco de dados SQL/NoSQL.
* ☁️ **Clientes Externos**: SDKs para AWS, Stripe, SendGrid, etc.
* 📂 **Storage**: Drivers de sistema de arquivos.
* 📡 **Gateways**: Wrappers para microserviços externos.

---

## Regras de Dependência

> [!IMPORTANT]
> **PODE importar**: `01_core` (para implementar suas interfaces).
> **NÃO PODE importar**: `02_shell`, `04_wiring`.

* ✅ `03_infra`  `01_core` (Implementando Interfaces de Domínio)
* ❌ `03_infra`  `02_shell` (Infraestrutura nunca conversa com a UI)

## Protocolo de IA (Auditoria de Isomorfismo)

1. **Aderência ao Contrato**: A IA deve verificar se cada classe em $L_3$ segue estritamente uma interface de $L_1$ ou $L_0$.
2. **Apenas Implementação**: Esta camada deve conter a lógica de "como falar com a ferramenta", não a "lógica de negócio".
3. **Tradução de Erros**: A IA deve mapear erros de infraestrutura (HTTP 500, SQL Timeout) para Erros de Domínio definidos em $L_1$.

---

### Example

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/contracts/user-repository.md
 */

// ✅ CORRECT - Implements an interface from Core
import { UserRepository } from '../../01_core/domain/interfaces';
import { Database } from './db-client';

export class SqlUserRepository implements UserRepository {
  // Logic restricted to data persistence
  async save(user) {
    return await Database.insert('users', user);
  }
}

// ❌ WRONG - Business logic leaked into infra
// if (user.age < 18) throw Error(); // THIS BELONGS IN 01_CORE!

```
