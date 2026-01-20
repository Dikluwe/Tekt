# Padrão de Arquitetura Cristalina

<div align="center">

**Um framework de topologia reforçada para desenvolvimento resistente a IA**

[![Versão](https://img.shields.io/badge/versão-1.1-blue.svg)](./MANIFESTO.pt.md)
[![Licença](https://img.shields.io/badge/licença-MIT-green.svg)](./LICENSE)

[**Manifesto**](./MANIFESTO.pt.md) • [**Início Rápido**](#início-rápido) • [**Documentação**](#documentação) • [**Exemplos**](#exemplos)

</div>

---

## 💎 Fundamentação Matemática

O Padrão Cristalino trata a arquitetura de software como um **Espaço Topológico** regido por leis matemáticas estritas para minimizar a entropia estrutural $H$.

### Estrutura Central

* **Topologia do Sistema** ($\mathcal{T}$): Um Grafo Acíclico Direcionado (DAG) onde nós são camadas $L_n$ e arestas são morfismos de dependência
* **Poset de Dependência**: Conjunto parcialmente ordenado $(L, \preceq)$ seguindo:
  $$L_0 \preceq L_1 \preceq \{L_2, L_3\} \preceq L_4$$
  onde $L_0$ (Núcleo) é o **elemento inferior** ($\bot$)
* **Controle de Entropia**: A **Invariante de Nucleação** impõe:
  $$\text{Código} \neq \emptyset \iff \text{Espec} \neq \emptyset$$

---

## Início Rápido

### 1. Criar Projeto do Template

```bash
git clone https://github.com/your-org/crystalline-architecture-standard.git meu-projeto
cd meu-projeto
```

### 2. Inicializar Estrutura

```bash
# Gerar mapas de navegação
cargo run --bin cartographer
# Output: PROJECT_MAP.md, MAPs de camadas, MAPs de módulos
```

### 3. Criar Sua Primeira Feature

#### Passo A: Escrever Especificação (Nucleação)

```bash
cat > 00_nucleo/specs/user-login.md <<'EOF'
# Feature: Login de Usuário

## Requisitos
1. Aceitar email + senha
2. Retornar token JWT (expira em 24h)
3. Rate limit: 5 tentativas por minuto

## Regras de Negócio
- Senhas devem ser hasheadas com bcrypt (cost=12)
- Tentativas falhadas disparam backoff exponencial
EOF
```

#### Passo B: Implementar Lógica Central (Pura)

```typescript
// 01_core/domain/auth.ts
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/user-login.md
 * @topology L1
 */
export function validateCredentials(
  email: string,
  password: string,
  hashedPassword: string
): boolean {
  // Validação pura (sem I/O)
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(email)) return false;
  
  return bcrypt.compareSync(password, hashedPassword);
}
```

#### Passo C: Adicionar Persistência (Infraestrutura)

```typescript
// 03_infra/database/user-repository.ts
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/user-login.md
 * @topology L3
 */
import { IUserRepository } from '../../01_core/contracts/user-repository';

export class UserRepository implements IUserRepository {
  async findByEmail(email: string): Promise<User | null> {
    return await db.users.findUnique({ where: { email } });
  }
}
```

#### Passo D: Conectar Dependências (Composição)

```typescript
// 04_wiring/main.ts
import { UserRepository } from '../03_infra/database/user-repository';
import { AuthService } from '../01_core/domain/auth-service';
import { AuthController } from '../02_shell/api/auth-controller';

const userRepo = new UserRepository(prisma);
const authService = new AuthService(userRepo);
const authController = new AuthController(authService);
```

### 4. Validar Estrutura

```bash
npm run crystalline:lint
# ✅ Nucleação: OK (todos os arquivos têm @spec)
# ✅ Gravidade: OK (sem dependências reversas)
# ✅ Pureza: OK (sem I/O em 01_core)
```

---

## A Estrutura do Retículo

A estrutura física de pastas atua como uma "restrição de hardware" para geração de lógica pela IA.

```
seu-projeto/
├── 00_nucleo/     # 📋 Especificações, ADRs, Contratos (⊥ A Semente)
├── 01_core/       # 💎 Lógica pura, zero I/O (O Cristal)
├── 02_shell/      # 🖥️  UI, API, CLI (Adaptadores Primários)
├── 03_infra/      # 🔌 Banco de Dados, Rede (Adaptadores Secundários)
├── 04_wiring/     # ⚡ Injeção de Dependência, main() (⊤ A Composição)
└── _lab/          # 🧪 Experimentos (Quarentena)
```

---

## Princípios Fundamentais

| # | Princípio | Propriedade Formal | Descrição |
|---|-----------|-------------------|-----------|
| 1 | **Nucleação** | Axiomatização | Especificações antes do código. Sem spec → Sem código. |
| 2 | **Contenção** | Fronteira Topológica | Estrutura de pastas como barreira física. |
| 3 | **Gravidade** | Aciclicidade Direcionada | Dependências apontam apenas para camadas inferiores. |
| 4 | **Darwinismo** | Isolamento | Código do Lab deve ser normalizado antes da produção. |

---

## Regras de Dependência

```mermaid
graph TD
    subgraph Crystal ["Sistema Central (O Cristal)"]
        direction TB
        N("00_nucleo<br>(Especificações) ⊥")
        C("01_core<br>(Lógica Pura)")
    end

    subgraph Adapters ["Adaptadores (A Borda)"]
        direction TB
        S("02_shell<br>(UI/API/CLI)")
        I("03_infra<br>(Banco/Rede)")
    end

    W("04_wiring<br>(Raiz de Composição) ⊤")
    L("_lab<br>(Quarentena)")

    %% Dependências (setas apontam para o que é dependido)
    C --> N
    S --> C
    I --> C
    W --> S
    W --> I
    W --> C
    L -.-> N
    
    %% Estilos
    classDef nucleus fill:#e1f5fe,stroke:#01579b,stroke-width:2px,color:black;
    classDef core fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:black;
    classDef adapters fill:#fff3e0,stroke:#ef6c00,stroke-width:2px,color:black;
    classDef wiring fill:#f3e5f5,stroke:#7b1fa2,stroke-width:2px,color:black;
    classDef lab fill:#ffebee,stroke:#c62828,stroke-dasharray: 5 5,color:black;

    class N nucleus;
    class C core;
    class S,I adapters;
    class W wiring;
    class L lab;
```

### Lendo o Diagrama

- **Setas sólidas** (→): Dependências diretas (permitidas)
- **Setas tracejadas** (⋯): Referência indireta (apenas specs)
- **Símbolos**: 
  - $\bot$ (inferior): Núcleo é a fundação
  - $\top$ (superior): Wiring vê tudo
- **Código de cores**:
  - 🔵 Azul: Especificações (fonte da verdade)
  - 🟢 Verde: Lógica pura (determinística)
  - 🟠 Laranja: Fronteiras de I/O
  - 🟣 Roxo: Camada de composição
  - 🔴 Vermelho: Zona de quarentena

**Regra de Dependência**: Setas apontam **para** dependências. Setas reversas violam a gravidade.

---

## Protocolo de IA

### Para Assistentes de IA (Cursor, Copilot, Claude)

#### 1. Carregamento de Contexto (Ordem de Prioridade)

```
Tarefa: "Implementar processamento de pagamento"

Passo 1: Ler PROJECT_MAP.md
         ↓
Passo 2: Navegar para 00_nucleo_MAP.md
         ↓
Passo 3: Verificar se 00_nucleo/specs/payment-processing.md existe
         ├─ SIM → Ler spec, prosseguir com implementação
         └─ NÃO → PARAR. Criar spec primeiro (Trava de Nucleação)
         
Passo 4: Ler contratos relevantes em 00_nucleo/contracts/
Passo 5: Implementar na camada apropriada (01_core, 02_shell, 03_infra)
Passo 6: Conectar em 04_wiring/
```

#### 2. Cabeçalho de Linhagem Obrigatório

Todo arquivo DEVE incluir:

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/<nome-feature>.md
 * @contract 00_nucleo/contracts/<interface>.md (se aplicável)
 * @topology L[n]
 * @updated YYYY-MM-DD
 */
```

#### 3. Regras Específicas por Camada

| Camada | Pode Importar De | Não Pode Importar De | Operações Permitidas |
|--------|------------------|----------------------|----------------------|
| L₀ (Nucleus) | — | — | Apenas especificações (sem código) |
| L₁ (Core) | L₀ | L₂, L₃, L₄, Lab | Funções puras, sem I/O |
| L₂ (Shell) | L₀, L₁ | L₃, L₄, Lab | Lógica UI/API, tradução |
| L₃ (Infra) | L₀, L₁ | L₂, L₄, Lab | Operações I/O, persistência |
| L₄ (Wiring) | Todas exceto Lab | — | Apenas configuração DI |
| Lab | L₀ (apenas specs) | Todas | Experimentos (volátil) |

#### 4. Checklist Pré-Salvamento

Antes de salvar qualquer arquivo, verificar:
- [ ] Tag `@spec` presente e aponta para arquivo existente
- [ ] Sem imports proibidos (verificar tabela acima)
- [ ] Se em `01_core/`: absolutamente zero operações I/O
- [ ] Implementação está conforme requisitos da spec
- [ ] Sem dependências circulares

---

## Documentação

### Documentos Centrais

| Documento | Descrição |
|-----------|-----------|
| [**MANIFESTO.pt.md**](./MANIFESTO.pt.md) | Documento constitucional com fundamentos matemáticos |
| [**MANIFESTO.md**](./MANIFESTO.md) | English version |
| [**PROJECT_MAP.md**](./PROJECT_MAP.md) | Mapa de navegação (gerado automaticamente) |

### Guias por Camada

| Camada | Guia | Propósito |
|--------|------|-----------|
| L₀ | [00_nucleo/README.md](./00_nucleo/README.md) | Escrita de especificações |
| L₁ | [01_core/README.md](./01_core/README.md) | Diretrizes de lógica pura |
| L₂ | [02_shell/README.md](./02_shell/README.md) | Padrões de adaptador |
| L₃ | [03_infra/README.md](./03_infra/README.md) | Configuração de infraestrutura |
| L₄ | [04_wiring/README.md](./04_wiring/README.md) | Configuração DI |
| Lab | [_lab/README.md](./_lab/README.md) | Protocolos de experimento |

### Configuração para IA

| Arquivo | Propósito | Alvo |
|---------|-----------|------|
| [.cursorrules](./.cursorrules) | Regras do Cursor IDE | Cursor |
| [.agentrules](./.agentrules) | Regras gerais para LLM | Claude, GPT-4, Gemini |

---

## Mapeamento para Padrões da Indústria

| Cristalina | Clean Architecture | Hexagonal | DDD |
|------------|-------------------|-----------|-----|
| `00_nucleo` | — | — | Linguagem Ubíqua |
| `01_core` | Entidades | Núcleo da Aplicação | Camada de Domínio |
| `02_shell` | Adaptadores de Interface | Adaptadores Primários | Camada de Aplicação |
| `03_infra` | Frameworks & Drivers | Adaptadores Secundários | Infraestrutura |
| `04_wiring` | Main | — | Raiz de Composição |

---

## Exemplos

| Domínio | Repositório | Status |
|---------|-------------|--------|
| Compilador (Typst) | [typst-crystalline](https://github.com/Dikluwe/typst-crystalline) | 🚧 Em Progresso |
| Web API (E-commerce) | [examples/shop/](./examples/shop/) | 📝 Planejado |
| Embarcado (IoT) | [examples/iot/](./examples/iot/) | 📝 Planejado |

---

## Ferramentas

### Cartographer (Gerador de Mapas Fractais)

Gera mapas de navegação hierárquicos:

```bash
cargo run --bin cartographer
# Cria: PROJECT_MAP.md, 00_nucleo/00_nucleo_MAP.md, etc.
```

### Crystalline Linter

Valida invariantes arquiteturais:

```bash
npm run crystalline:lint
# Verifica: nucleação, gravidade, pureza, isomorfismo
```

### Integração CI/CD

Adicione em `.github/workflows/crystalline-ci.yml`:

```yaml
name: Verificação de Integridade Cristalina
on: [push, pull_request]
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Validar Estrutura
        run: npm run crystalline:lint
```

---

## Contribuindo

Veja [CONTRIBUTING.md](./CONTRIBUTING.md) para diretrizes.

**Regra Principal**: Todas as contribuições devem seguir o Protocolo de Nucleação (spec antes do código).

---

## Licença

Licença MIT — Use livremente em qualquer projeto.

---

## Citação

Se você usar a Arquitetura Cristalina em pesquisa, por favor cite:

```bibtex
  @misc{crystalline2025,
  title={Crystalline Architecture: A Topology-Enforced Framework for AI-Resistant Software Development},
  author={Diego Kluwe de Souza},
  year={2025},
  howpublished={\url{https://github.com/Dikluwe/crystalline-architecture-standard}}
}
```
