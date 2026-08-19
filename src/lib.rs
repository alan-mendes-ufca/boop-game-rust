//! Boop - jogo de tabuleiro para dois jogadores.
//!
//! Reimplementacao em Rust do projeto originalmente escrito em C
//! (`JogoBoop/`), organizada em tres camadas no estilo MVC:
//!
//! - [`model`]: dados e regras do jogo, sem nenhum IO;
//! - [`view`]: leitura da entrada e desenho do tabuleiro no terminal;
//! - [`controller`]: orquestra as duas, um turno de cada vez.
//!
//! A dependencia so aponta para dentro: o controller usa view e model, a view
//! usa o model, e o model nao conhece ninguem.

pub mod controller;
pub mod model;
pub mod view;

pub use model::tipos::*;
