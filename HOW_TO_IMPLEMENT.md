# Como Implementar a Arquitetura Cristalina (Tekt) na Prática

A beleza da Arquitetura Cristalina não está apenas na teoria matemática, mas no processo mecânico e inevitável de construir software de dentro para fora. Este guia é para você, desenvolvedor humano, e para o seu agente de IA.

Vamos simular a criação de uma funcionalidade extremamente comum: **"Criar um endpoint para cadastrar um novo usuário verificando se o e-mail já existe".**

O fluxo de trabalho na Tekt *sempre* segue esta ordem restrita: Invariante de Nucleação $\to$ Core Pura $\to$ Interfaces Sujeitas a Entropia $\to$ Composição. 

---

## 🏗 Passo 0: O Scaffold do Projeto
Antes de começar a gerar lógica, o esqueleto do "lattice" (grade arquitetural) precisa existir. Seu projeto deve iniciar com estas pastas literais na raiz:

```
meu-projeto/
├── 00_nucleo/     (L0: Onde nascem os markdown e contratos da sua Feature)
├── 01_core/       (L1: Regras vitais sem I/O nem dependências externas)
├── 02_shell/      (L2: Seus Controladores de API, rotas, CLI, telas)
├── 03_infra/      (L3: Suas queries em DB, requests externos, leitura de arquivos)
├── 04_wiring/     (L4: Arquivo main, Injeção de dependência e inicialização)
├── 10_bedrock/    (Docker, Nix, Node versions)
├── 11_tools/      (Linters, config de linter)
└── 20_lab/        (Pasta para scripts temporários ou testes de LLMs sujos)
```

---

## 📜 Passo 1: A Nucleação (Definir o `L0`)
*"Sem spec, sem código."* Nunca escreva uma regra de negócio que não tenha sido declarada primeiro. 

Começamos criando um documento vivo no núcleo:

**`00_nucleo/specs/cadastro-usuario.md`**
```markdown
# Feature: Cadastro de Usuário
**Descrição:** Permite criar um novo usuário no sistema.

### Invariantes (Regras Duras)
- O e-mail deve ter um formato válido `@`.
- Senhas não podem ter menos de 8 caracteres.
- Se o e-mail já estiver cadastrado, a operação deve falhar graciosamente (Erro 409).
- A senha deve ser recebida crua, mas NUNCA salva crua. Deve passar por Hashing BCRYPT (Cost 10).
```

Defina também o **Contrato** estrutural (Interface). Este arquivo guiará a Infra e a Lógica.

**`00_nucleo/contracts/user-repository.interface.ts`**
```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/cadastro-usuario.md
 * @topology L0
 */
export interface UserEntity {
  id?: string;
  email: string;
  passwordHash: string;
}

export interface IUserRepository {
  findByEmail(email: string): Promise<UserEntity | null>;
  save(user: UserEntity): Promise<UserEntity>;
}

export interface IPasswordHasher {
  hash(plain: string): Promise<string>;
}
```

---

## 💎 Passo 2: O Cristal / Lógica Pura (`L1`)
Com a intenção nucleada, vamos criar o cérebro que ignora de onde vêm os dados e para onde eles vão. Zero dependência de frameworks.

**`01_core/use_cases/register-user.case.ts`**
```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/cadastro-usuario.md
 * @topology L1
 */
import { IUserRepository, IPasswordHasher, UserEntity } from '../../00_nucleo/contracts/user-repository.interface';

// Erros de domínio
export class InvalidEmailError extends Error {}
export class EmailAlreadyExistsError extends Error {}
export class WeakPasswordError extends Error {}

export class RegisterUserUseCase {
  // Recebe as dependências de infra por injeção
  constructor(
    private readonly userRepository: IUserRepository,
    private readonly passwordHasher: IPasswordHasher
  ) {}

  async execute(email: string, plainPassword: string): Promise<UserEntity> {
    // 1. Validações puras (Axiomas locais)
    if (!email.includes('@')) throw new InvalidEmailError('Email inválido');
    if (plainPassword.length < 8) throw new WeakPasswordError('Senha menor que 8 caracteres');

    // 2. I/O (Através das interfaces Puras)
    const existingUser = await this.userRepository.findByEmail(email);
    if (existingUser) {
      throw new EmailAlreadyExistsError('Este e-mail já está em uso');
    }

    const passwordHash = await this.passwordHasher.hash(plainPassword);

    // 3. Efeito final
    const newUser = { email, passwordHash };
    return this.userRepository.save(newUser);
  }
}
```

---

## 🔌 Passo 3: Infra / Mundo Real (`L3`)
Agora sujamos as mãos implementando os contratos que nasceram no `L0`. É a camada de contato elétrico.

**`03_infra/database/postgres-user.repository.ts`**
```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/cadastro-usuario.md
 * @topology L3
 */
import { IUserRepository, UserEntity } from '../../00_nucleo/contracts/user-repository.interface';
import { PrismaClient } from '@prisma/client'; // L3 tem permissão para importar libs externas pesadas

export class PostgresUserRepository implements IUserRepository {
  constructor(private db: PrismaClient) {}

  async findByEmail(email: string): Promise<UserEntity | null> {
    const user = await this.db.user.findUnique({ where: { email } });
    return user ? { id: user.id, email: user.email, passwordHash: user.passwordHash } : null;
  }

  async save(user: UserEntity): Promise<UserEntity> {
    const created = await this.db.user.create({ data: { email: user.email, passwordHash: user.passwordHash } });
    return { id: created.id, email: created.email, passwordHash: created.passwordHash };
  }
}
```

**`03_infra/cryptography/bcrypt.hasher.ts`**
```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/cadastro-usuario.md
 * @topology L3
 */
import { IPasswordHasher } from '../../00_nucleo/contracts/user-repository.interface';
import * as bcrypt from 'bcrypt'; 

export class BcryptHasher implements IPasswordHasher {
  async hash(plain: string): Promise<string> {
    return bcrypt.hash(plain, 10);
  }
}
```

---

## 🖥 Passo 4: A Superfície / Shell (`L2`)
A camada que fala com os humanos e a rede. Sem regras absolutas de negócio, apenas validação de requisição (schemas HTTP) e tradução de rotas em Casos de Uso.

**`02_shell/http/api/controllers/user.controller.ts`**
```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/cadastro-usuario.md
 * @topology L2
 */
import { RegisterUserUseCase, EmailAlreadyExistsError, InvalidEmailError, WeakPasswordError } from '../../../01_core/use_cases/register-user.case';

export class UserController {
  constructor(private registerUserUseCase: RegisterUserUseCase) {}

  async handleRegister(req: any, res: any): Promise<void> {
    try {
      // Traduz JSON para o formato bruto que o Core espera
      const { email, password } = req.body; 
      
      const user = await this.registerUserUseCase.execute(email, password);
      
      res.status(201).json({ id: user.id, email: user.email });
    } catch (error) {
       // O L2 traduz "Falhas Purificadas de Negócio" (L1) em códigos HTTP 
      if (error instanceof EmailAlreadyExistsError) {
        res.status(409).json({ error: error.message });
      } else if (error instanceof InvalidEmailError || error instanceof WeakPasswordError) {
        res.status(400).json({ error: error.message });
      } else {
        res.status(500).json({ error: 'Internal Server Error' });
      }
    }
  }
}
```

---

## ⚡ Passo 5: Fiação / O Topo da Pirâmide (`L4`)
O Deus Ex Machina. A única camada cujo trabalho é importar todo mundo do `L1, L2, L3` para uni-los, compondo a injeção mecânica para o servidor inicializar. Nenhuma lógica, apenas instanciação e bootstrap.

**`04_wiring/main.ts`**
```typescript
/**
 * Crystalline Lineage
 * @topology L4
 */
import { PrismaClient } from '@prisma/client';
import express from 'express';

// Imports da topologia
import { PostgresUserRepository } from '../03_infra/database/postgres-user.repository';
import { BcryptHasher } from '../03_infra/cryptography/bcrypt.hasher';
import { RegisterUserUseCase } from '../01_core/use_cases/register-user.case';
import { UserController } from '../02_shell/http/api/controllers/user.controller';

// 1. Instanciar infraestrutura (L3)
const db = new PrismaClient();
const userRepository = new PostgresUserRepository(db);
const hasher = new BcryptHasher();

// 2. Instanciar as lógicas de negócio com dependências injetadas (L1)
const registerUseCase = new RegisterUserUseCase(userRepository, hasher);

// 3. Instanciar a apresentação (L2)
const userController = new UserController(registerUseCase);

// 4. Iniciar framework (Server)
const app = express();
app.use(express.json());

app.post('/users', (req, res) => userController.handleRegister(req, res));

app.listen(3000, () => {
  console.log('💎 Cristal Materializado e rodando na porta 3000');
});
```

---
## Resumo do que Você/Agente Fez:
1. `L0`: Prometeu como iria funcionar (Regras & Contratos).
2. `L1`: Fez a conta da matemática de negócio (Lógica pura, zero *requests/databases*).
3. `L3`: Sujou as mãos instalando os drivers de banco de dados e libs e cumpriu os Contratos Prometidos.
4. `L2`: Abriu uma "estrada de serviço" para o usuário humano da Web/API solicitar a lógica pura.
5. `L4`: Juntou todas as peças na caixa principal (Startup da Aplicação).
