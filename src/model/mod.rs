//! Camada de model: os dados e as regras do jogo, sem nenhuma entrada ou
//! saida. Nada aqui conhece o terminal nem as camadas de view e controller.
//!
//! | modulo      | origem em C             |
//! |-------------|-------------------------|
//! | [`tipos`]   | `funcoes.h`             |
//! | [`jogada`]  | `jogada/jogada.c`       |
//! | [`graduar`] | `graduar/graduar.c`     |
//! | [`vitoria`] | `vitoria/vencer.c`      |

pub mod graduar;
pub mod jogada;
pub mod tipos;
pub mod vitoria;
