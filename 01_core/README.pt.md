# /01_core — O Núcleo

> Lógica pura. O coração determinístico do sistema.

---

## Propósito

Este estrato contém apenas lógica de domínio essencial: entidades, regras de negócio, algoritmos puros. Nada aqui depende de banco de dados, rede, sistema de arquivos, ou qualquer estado externo.

O Núcleo é a fase estrutural mais estável do sistema. Código aqui existe independentemente do tempo, das tecnologias de infraestrutura e dos canais de entrada.

---

## Restrição Absoluta: Zero I/O

> Código neste diretório **não deve**:
> - Acessar banco de dados ou fazer requisições de rede
> - Ler ou escrever arquivos
> - Acessar o relógio do sistema ou estado global mutável
> - Importar bibliotecas externas (apenas biblioteca padrão da linguagem)

Qualquer necessidade de I/O deve ser expressa como uma interface em `00_nucleo/contracts/` e implementada em `03_infra/`.

---

## O que pertence aqui

- Entidades de domínio e Value Objects
- Validação de regras de negócio
- Algoritmos e transformações puras
- Definições de interfaces (não implementações)
- Erros de domínio

---

## Estrutura

```
01_core/
├── entities/     # Entidades de domínio e Value Objects
├── domain/       # Regras de negócio e serviços puros
└── algorithms/   # Algoritmos e transformações
```

---

## Regra de Dependência

- **Pode importar**: `00_nucleo` (para implementar contratos e satisfazer specs)
- **Proibido**: `02_shell`, `03_infra`, `04_wiring`, `_lab`

---

## Exemplo

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/validacao-usuario.md
 * @layer L1
 */

// ✅ Correto — função pura, sem dependências externas
export function validarEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

// ❌ Errado — I/O externo viola a pureza do estrato
// import axios from 'axios';
// export async function verificarEmail(email: string) {
//   return await axios.get(`/api/verify?email=${email}`);
// }
```
