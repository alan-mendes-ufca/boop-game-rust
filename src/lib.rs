//! Boop - jogo de tabuleiro para dois jogadores.
//!
//! Reimplementacao em Rust do projeto originalmente escrito em C
//! (`JogoBoop/`), preservando a modularizacao do original:
//!
//! | modulo Rust     | origem em C              |
//! |-----------------|--------------------------|
//! | [`tipos`]       | `funcoes.h`              |
//! | [`tabuleiro`]   | `tabuleiro/tabuleiro.c`  |
//! | [`jogada`]      | `jogada/jogada.c`        |
//! | [`graduar`]     | `graduar/graduar.c`      |
//! | [`vitoria`]     | `vitoria/vencer.c`       |
//! | [`funcoes`]     | `funcoes.c`              |

pub mod funcoes;
pub mod graduar;
pub mod jogada;
pub mod tabuleiro;
pub mod tipos;
pub mod vitoria;

pub use tipos::*;
