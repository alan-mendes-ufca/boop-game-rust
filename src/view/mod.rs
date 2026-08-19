//! Camada de view: tudo que fala com o terminal. Le a entrada do jogador e
//! desenha o tabuleiro, sem decidir nada sobre as regras do jogo.
//!
//! | modulo        | origem em C             |
//! |---------------|-------------------------|
//! | [`tabuleiro`] | `tabuleiro/tabuleiro.c` |
//! | [`entrada`]   | `funcoes.c` (leitura)   |

pub mod entrada;
pub mod tabuleiro;
