### 1. README.md (English Version)

# /04_wiring — The Energy

> **The Connection.** Where all parts come together.

## Purpose

This directory is the **Composition Root**: the only place that knows about ALL layers and wires them together. It contains the `main()` function, dependency injection configuration, and environment setup.

---

## 💎 Mathematical Formalism ($\mathcal{L}_4$)

In the Crystalline Standard, the Wiring layer is the **Initial Object** ($I$) of the dependency category:

* **Universal Mapping**: For every layer $L_i \in \{L_0, L_1, L_2, L_3\}$, there exists a unique morphism $L_4 \to L_i$.
* **The Composition Operator ($\circ$)**: $L_4$ is responsible for the composition of morphisms. If $L_1$ defines an interface $f$ and $L_3$ provides an implementation $g$, $L_4$ performs the assignment $f := g$.
* **Thin Logic Constraint**: $L_4$ must have a logical complexity of nearly zero ($O(1)$). It is an orchestrator, not a creator.
$$\text{Lógica}(L_4) \to \min$$

---

## What Lives Here

* ⚡ **Entry Points**: `main()`, `index.ts`, or `app.py`.
* 💉 **Dependency Injection**: Container configuration and manual wiring.
* ⚙️ **Environment Config**: `.env` loaders and configuration objects.
* 🚀 **Bootstrap Logic**: Server startup and application initialization.

## Dependency Rules

> [!NOTE]
> This is the ONLY directory that can import from ALL numbered layers ($L_0$ to $L_3$).

* ✅ `04_wiring`  `00_nucleo` (read specs).
* ✅ `04_wiring`  `01_core` (import domain).
* ✅ `04_wiring`  `02_shell` (import UI/API).
* ✅ `04_wiring`  `03_infra` (import implementations).

## AI Protocol (Isomorphism Audit)

1. **No Hidden Logic**: AI must not generate business rules in this layer. Any `if/else` related to business belongs in $L_1$.
2. **Implementation Mapping**: AI must verify that the implementations from $L_3$ correctly satisfy the interfaces required by $L_2$ and defined in $L_1$.
3. **Environment Safety**: AI must ensure all required environment variables are validated during bootstrap.

---

/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/application-bootstrap.md
 */

// Import from ALL layers
import { IUserRepository } from '../01_core/domain/user';
import { UserController } from '../02_shell/api/user-controller';
import { PostgresUserRepository } from '../03_infra/database/postgres-user-repository';

// Environment configuration
const config = {
  database: {
    host: process.env.DB_HOST,
    port: parseInt(process.env.DB_PORT || '5432'),
  },
};

// Dependency Injection (The Assignment Morphism)
const userRepository: IUserRepository = new PostgresUserRepository(config.database);
const userController = new UserController(userRepository);

// Bootstrap
export function main() {
  const app = createServer();
  app.use('/users', userController.routes);
  app.listen(3000);
}

main();

---

## Anti-Pattern

> [!WARNING]
> Wiring code should be THIN. If you find yourself writing business logic here, it belongs in `01_core`.
---
