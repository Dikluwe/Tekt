### 2. README.pt.md (Versão em Português)

# /_lab — A Rinha

> **O Caos Controlado.** A zona de experimentação em quarentena.

## Propósito

Este diretório é um **sandbox** para experimentos, benchmarks e provas de conceito. O código aqui é volátil, de alta entropia e pode ser deletado a qualquer momento.

---

## 💎 Formalismo Matemático ($\mathcal{L}_{lab}$)

A Rinha é definida como uma **Zona de Alta Entropia** onde as restrições estruturais são suspensas para permitir a exploração:

* **Maximização de Entropia**: Diferente do Core, o Lab permite que $H \to \max$. Não existem invariantes de qualidade de código ou isolamento de I/O.
* **Isolamento Absoluto**: O sistema principal $S = \{L_0, L_1, L_2, L_3, L_4\}$ é um conjunto fechado em relação ao Lab.
$$\text{dep}(S) \cap \text{Lab} = \emptyset$$
* **Seleção Darwinista (Normalização)**: A transição do Lab ($x$) para o Core ($x'$) não é um mapeamento direto, mas uma **Função de Normalização** $N(x) = x'$, onde todo débito técnico é removido e a lógica é re-axiomatizada.

---

## A Regra de Quarentena

> [!CAUTION]
> **QUARENTENA ABSOLUTA**
> O sistema principal (00-04) **NUNCA** importa nada de `_lab`.

## Ciclo de Vida: O Caminho Darwinista

1. **Experimentar**: Escrever código "sujo" para explorar possibilidades.
2. **Benchmark**: Medir e comparar diferentes abordagens matematicamente.
3. **Decidir**: Escolher a abordagem mais eficiente (O Vencedor).
4. **Cristalizar**: **REESCREVER DO ZERO** no `01_core`. Copiar e colar diretamente é uma violação da integridade estrutural.
5. **Deletar**: Expurgar o experimento para manter o retículo limpo.

---

## Zona de Liberdade (A Zona "Sem Trava")

Este é o ÚNICO lugar onde a **Trava de Nucleação** é suspensa:

* ✅ A IA pode gerar código sem uma especificação prévia no `00_nucleo`.
* ✅ Operações de I/O, bibliotecas externas e valores "hardcoded" são permitidos.
* ✅ Código bagunçado e fora dos padrões é encorajado em prol da velocidade.

---

### Por que reescrever?

O código do Lab é otimizado para **velocidade de exploração**, não para **qualidade de produção**. Ele pode conter atalhos, falta de tratamento de erros ou dependências que violam a regra de Zero I/O. Reescrever garante que o código promovido atenda aos padrões do Cristal.

---

