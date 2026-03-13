# /04_wiring — A Fiação

> O único ponto onde tudo se conecta.

---

## Propósito

Este estrato é a raiz de composição: o único lugar que conhece todos os estratos e os conecta. Aqui vivem o `main()`, a configuração de injeção de dependência e a inicialização do ambiente.

A Fiação é o ponto de materialização total — onde as definições abstratas de `00_nucleo` encontram suas implementações concretas de `03_infra`.

---

## Propriedade Fundamental

A Fiação tem lógica próxima de zero. Ela orquestra — não cria. Qualquer `if/else` relacionado a regras de negócio encontrado aqui é um defeito estrutural e pertence a `01_core`.

---

## O que pertence aqui

- Ponto de entrada da aplicação (`main()`, `index.ts`, `app.py`)
- Configuração de injeção de dependência
- Carregamento de variáveis de ambiente
- Inicialização do servidor

---

## Regra de Dependência

Este é o único estrato que pode importar de todas as camadas numeradas.

- ✅ `04_wiring` → `00_nucleo`
- ✅ `04_wiring` → `01_core`
- ✅ `04_wiring` → `02_shell`
- ✅ `04_wiring` → `03_infra`
- ❌ `04_wiring` → `_lab`

---

## Exemplo

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/bootstrap-aplicacao.md
 * @layer L4
 */
import { IRepositorioUsuario } from '../01_core/domain/usuario';
import { ControllerUsuario } from '../02_shell/api/controller-usuario';
import { RepositorioUsuarioPostgres } from '../03_infra/database/repositorio-usuario-postgres';

// Injeção de dependência — o único lugar onde implementações são atribuídas a interfaces
const repositorio: IRepositorioUsuario = new RepositorioUsuarioPostgres(config.database);
const controller = new ControllerUsuario(repositorio);

export function main() {
  const app = criarServidor();
  app.use('/usuarios', controller.rotas);
  app.listen(3000);
}

main();
```

---

## Protocolo para Agentes de IA

- Não gerar lógica de negócio neste estrato
- Verificar que implementações de `03_infra` satisfazem as interfaces exigidas por `02_shell` e definidas em `01_core`
- Garantir que variáveis de ambiente obrigatórias são validadas no bootstrap
