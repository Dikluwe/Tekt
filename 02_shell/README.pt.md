# /02_shell — A Casca

> Tradução entre o mundo externo e o Núcleo.

---

## Propósito

Este estrato contém os adaptadores primários: componentes que recebem entrada do mundo externo — usuários, requisições HTTP, comandos CLI — e traduzem para chamadas ao domínio em `01_core`.

A Casca é o plano de clivagem primário: separa a estabilidade conceitual do Núcleo da variabilidade dos canais de entrada.

---

## O que pertence aqui

- Componentes de UI (React, Vue, Svelte)
- Controllers de API REST e resolvers GraphQL
- Handlers de CLI
- Validação de entrada (formato e tipo, não regras de negócio)
- Projeção de saída (transformação de resposta para o cliente)

---

## Regra de Dependência

- **Pode importar**: `00_nucleo`, `01_core`
- **Proibido**: `03_infra`, `04_wiring`, `_lab`

> A Casca nunca acessa infraestrutura diretamente. Dependências de `03_infra` são injetadas via `04_wiring`.

---

## Exemplo

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/cadastro-usuario.md
 * @layer L2
 */
import { validarUsuario } from '../../01_core/domain/usuario';

// ✅ Correto — traduz entrada externa para lógica do Core
export function ControllerCadastro(req, res) {
  const resultado = validarUsuario(req.body);
  if (!resultado.valido) {
    return res.status(400).json({ erros: resultado.erros });
  }
  // continua...
}

// ❌ Errado — acesso direto à infraestrutura
// import { db } from '../../03_infra/persistence'; // PROIBIDO
```

---

## Protocolo para Agentes de IA

- Validar toda entrada externa antes de passar ao Core
- Não gerar chamadas `fetch`, `sql` ou acesso a arquivos neste estrato
- Componentes de UI devem ser o mais funcionais possível — lógica no Core
