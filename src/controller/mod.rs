//! Camada de controller: liga a view ao model. Recebe a jogada lida pela
//! view, aplica as regras do model na ordem certa e pede a view para
//! redesenhar.
//!
//! | modulo    | origem em C            |
//! |-----------|------------------------|
//! | [`turno`] | `fluxoJogo` (funcoes.c)|
//! | [`jogo`]  | `main.c`               |

pub mod jogo;
pub mod turno;
