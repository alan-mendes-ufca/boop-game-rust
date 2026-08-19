# 🐱 Boop - Jogo de Tabuleiro em Rust

Reimplementação em Rust do jogo de tabuleiro **Boop**, um jogo estratégico de dois jogadores onde gatos adoráveis competem pelo domínio do tabuleiro através do efeito "boop" (empurrão). Este é um port do [projeto original em C](https://github.com/alan-mendes-ufca/Boop-game).

## Regras do Jogo

### Objetivo
Ser o primeiro jogador a conseguir **três gatões (gatos adultos)** alinhados consecutivamente (horizontal, vertical ou diagonal) no tabuleiro, ou ter todas as 8 peças ativas no tabuleiro ("cama cheia").

### Componentes
- **Tabuleiro**: 6×6 células
- **Peças por jogador**: 8 gatinhos (representados por `g`), que podem se graduar em gatões (`G`)

### Como Jogar

1. **Colocação de Peças**
   - Em cada turno, o jogador coloca um gatinho ou gatão em uma célula vazia do tabuleiro
   - Formato de entrada: `tipo linha coluna` (exemplo: `g 1 A`)
   - Linhas são numeradas de 1 a 6; colunas de A a F

2. **Efeito Boop (Empurrão)**
   - Quando uma peça é colocada, ela "boopa" (empurra) todas as peças adjacentes em 8 direções
   - **Regras do Boop**:
     - Gatinhos **não podem** empurrar gatões
     - Peças empurradas movem-se uma casa na direção do empurrão
     - Se a célula de destino já estiver ocupada, a peça não se move
     - Se uma peça for empurrada para fora do tabuleiro, ela retorna ao inventário do jogador que a possui

3. **Graduação**
   - Quando **3 gatinhos do mesmo jogador** ficam alinhados consecutivamente (horizontal, vertical ou diagonal), eles se graduam
   - Os 3 gatinhos são removidos do tabuleiro
   - O jogador recebe **3 gatões** no inventário (para serem colocados em turnos futuros)
   - Cada peça só conta em um trio
   - **A graduação tem prioridade sobre o boop**: se a peça colocada já fecha o trio, o empurrão daquele turno não acontece (senão ele desfaria o alinhamento antes da graduação ser avaliada). O boop continua normal quando a jogada não fecha trio — inclusive quando é o próprio boop que forma o alinhamento

4. **Condições de Vitória**
   - **Vitória Principal**: Formar uma linha de 3 **gatões** consecutivos no tabuleiro
   - **Vitória Alternativa**: Ter as **8 peças ativas** no tabuleiro ("cama cheia")
   - **Empate**: Ambos os jogadores ficarem sem peças na mão (nenhum pode jogar)

## Como Compilar e Jogar

### Requisitos
- Rust estável (edition 2021). Instale em https://rustup.rs/

### Compilação e Execução

```bash
# Compilar e executar (modo debug)
cargo run

# Compilar otimizado (modo release)
cargo build --release
./target/release/boop

# Executar testes
cargo test

# Verificar qualidade do código
cargo clippy
```

Não há dependências externas; o projeto usa apenas a biblioteca padrão do Rust.

## Estrutura do Projeto

O `src/` é organizado em três camadas no estilo **MVC**. A dependência só
aponta para dentro: o *controller* usa *view* e *model*, a *view* usa o
*model*, e o *model* não conhece ninguém (não tem nenhum `println!` nem
`std::io`).

```
src/
├── main.rs            ponto de entrada, só chama o controller
├── lib.rs             declara as três camadas
├── model/             dados e regras do jogo, sem nenhum IO
│   ├── tipos.rs
│   ├── jogada.rs
│   ├── graduar.rs
│   └── vitoria.rs
├── view/              tudo que fala com o terminal
│   ├── tabuleiro.rs
│   └── entrada.rs
└── controller/        liga a view ao model
    ├── turno.rs
    └── jogo.rs
```

| Módulo Rust | Camada | Origem em C | Responsabilidade |
|---|---|---|---|
| `model/tipos.rs` | Model | `funcoes.h` | Definições de tipos: `Peca`, `Jogador`, constantes e estruturas de dados |
| `model/jogada.rs` | Model | `jogada/jogada.c` | A `Jogada`, validação de jogadas e aplicação do efeito boop |
| `model/graduar.rs` | Model | `graduar/graduar.c` | Detecção de alinhamentos de 3 peças e graduação |
| `model/vitoria.rs` | Model | `vitoria/vencer.c` | Detecção de condições de vitória e empate |
| `view/tabuleiro.rs` | View | `tabuleiro/tabuleiro.c` | Exibição do tabuleiro 6×6 e da arte ASCII |
| `view/entrada.rs` | View | `funcoes.c` (leitura) | Prompt, leitura e parsing do texto digitado em uma `Jogada` |
| `controller/turno.rs` | Controller | `fluxoJogo` em `funcoes.c` | Ordem de um turno: colocação → boop → graduação |
| `controller/jogo.rs` | Controller | `main.c` | Laço principal do jogo |
| `main.rs` | — | `main.c` | Ponto de entrada do executável |
| `tests/regras.rs` | — | — | Testes automatizados (novo em relação ao original) |

## O Que Mudou em Relação ao C

- **Dono da peça**: Representado por `enum Jogador` em vez de comparação de string de cor por ponteiro (bug no original)
- **Célula do tabuleiro**: Representada como `Option<Gato>`, eliminando peças sem dono e comparações de ponteiro nulo
- **Alocação de memória**: Tabuleiro é array fixo (não usa `malloc`), sem risco de vazamento ou falha de alocação
- **Graduação**: Só processa trios de gatinhos do mesmo dono, sem recontagem de linhas sobrepostas
- **Prioridade da graduação**: Jogada que fecha um trio não aplica boop, então o alinhamento não é desfeito antes de graduar (no original o boop sempre prevalecia)
- **Vitória**: Exige trio de **gatões** (não aceita gatinhos)
- **Entrada**: Validação robusta em vez de `scanf` desprotegido
- **Invariante de peças**: Contagem garantida por `gatinhos + gatoes + ativas == 8`
- **Testes**: Suite de testes automatizados com `cargo test` (não existia no original)
- **Arquitetura**: Código separado em model/view/controller, com o model livre de qualquer entrada ou saída (no original as regras e os `printf` moravam nos mesmos arquivos)

## Nota

Este é um projeto educacional, realizado como port de um jogo de tabuleiro de uma linguagem para outra, preservando as regras e a estrutura modular do original ao mesmo tempo que aproveita os benefícios da segurança de memória e tipagem do Rust.
