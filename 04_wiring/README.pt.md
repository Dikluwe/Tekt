### 2. README.pt.md (Versão em Português)

# /04_wiring — A Energia

> **A Conexão.** Onde todas as partes se unem.

## Propósito

Este diretório é o **Composition Root**: o único lugar que conhece TODAS as camadas e as conecta. Contém o `main()`, configuração de injeção de dependência e setup de ambiente.

---

## 💎 Formalismo Matemático ($\mathcal{L}_4$)

No Padrão Cristalino, a camada de Wiring é o **Objeto Inicial** ($I$) da categoria de dependências:

* **Mapeamento Universal**: Para cada camada $L_i \in \{L_0, L_1, L_2, L_3\}$, existe um morfismo único $L_4 \to L_i$.
* **O Operador de Composição ($\circ$)**: $L_4$ é responsável pela composição de morfismos. Se $L_1$ define uma interface $f$ e $L_3$ fornece uma implementação $g$, o $L_4$ realiza a atribuição $f := g$.
* **Restrição de Lógica Fina**: $L_4$ deve ter uma complexidade lógica próxima de zero ($O(1)$). Ele é um orquestrador, não um criador.
$$\text{Lógica}(L_4) \to \min$$

---

## O Que Vive Aqui

* ⚡ **Entry Points**: `main()`, `index.ts`, ou `app.py`.
* 💉 **Injeção de Dependência**: Configuração de containers e montagem manual.
* ⚙️ **Configuração de Ambiente**: Carregadores de `.env` e objetos de config.
* 🚀 **Lógica de Bootstrap**: Inicialização de servidores e da aplicação.

---

## Regras de Dependência

> [!NOTE]
> Este é o ÚNICO diretório que pode importar de TODAS as camadas numeradas ($L_0$ a $L_3$).

* ✅ `04_wiring`  `00_nucleo` (leitura de specs).
* ✅ `04_wiring`  `01_core` (importação do domínio).
* ✅ `04_wiring`  `02_shell` (importação de UI/API).
* ✅ `04_wiring`  `03_infra` (importação de implementações).

## Protocolo de IA (Auditoria de Isomorfismo)

1. **Sem Lógica Oculta**: A IA não deve gerar regras de negócio nesta camada. Qualquer `if/else` de negócio pertence ao $L_1$.
2. **Mapeamento de Implementação**: A IA deve verificar se as implementações do $L_3$ satisfazem corretamente as interfaces exigidas pela $L_2$ e definidas no $L_1$.
3. **Segurança de Ambiente**: A IA deve garantir que todas as variáveis de ambiente necessárias sejam validadas durante o bootstrap.

---

/**
 * Crystalline Lineage / Linhagem Cristalina
 * @spec 00_nucleo/specs/application-bootstrap.md
 */

// Importar de TODAS as camadas
import { IUserRepository } from '../01_core/domain/user';
import { UserController } from '../02_shell/api/user-controller';
import { PostgresUserRepository } from '../03_infra/database/postgres-user-repository';

// Configuração de ambiente
const config = {
  database: {
    host: process.env.DB_HOST,
    port: parseInt(process.env.DB_PORT || '5432'),
  },
};

// Injeção de Dependência (O Morfismo de Atribuição)
const userRepository: IUserRepository = new PostgresUserRepository(config.database);
const userController = new UserController(userRepository);

// Inicialização
export function main() {
  const app = createServer();
  app.use('/users', userController.routes);
  app.listen(3000);
}

main();

---

### Anti-Padrão

> [!WARNING]
> Código de wiring deve ser FINO. Se você está escrevendo lógica de negócio aqui, ela pertence ao `01_core`.

---
