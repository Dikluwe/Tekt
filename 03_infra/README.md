# /04_wiring — The Wiring

> The only point where everything connects.

---

## Purpose

This stratum is the composition root: the only place that knows all strata and connects them. Here live `main()`, dependency injection configuration, and environment initialization.

The Wiring is the point of total materialization — where the abstract definitions from `00_nucleo` meet their concrete implementations from `03_infra`.

---

## Fundamental Property

The Wiring has near-zero logic. It orchestrates — it does not create. Any `if/else` related to business rules found here is a structural defect and belongs to `01_core`.

---

## What belongs here

- Application entry point (`main()`, `index.ts`, `app.py`)
- Dependency injection configuration
- Environment variable loading
- Server initialization

---

## Dependency Rule

This is the only stratum that can import from all numbered layers.

- ✅ `04_wiring` → `00_nucleo`
- ✅ `04_wiring` → `01_core`
- ✅ `04_wiring` → `02_shell`
- ✅ `04_wiring` → `03_infra`
- ❌ `04_wiring` → `_lab`

---

## Example

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/application-bootstrap.md
 * @layer L4
 */
import { IUserRepository } from '../01_core/domain/user';
import { UserController } from '../02_shell/api/user-controller';
import { PostgresUserRepository } from '../03_infra/database/postgres-user-repository';

// Dependency injection — the only place where implementations are assigned to interfaces
const repository: IUserRepository = new PostgresUserRepository(config.database);
const controller = new UserController(repository);

export function main() {
  const app = createServer();
  app.use('/users', controller.routes);
  app.listen(3000);
}

main();
```

---

## AI Agents Protocol

- Do not generate business logic in this stratum
- Verify that implementations from `03_infra` satisfy the interfaces required by `02_shell` and defined in `01_core`
- Ensure that mandatory environment variables are validated on bootstrap
