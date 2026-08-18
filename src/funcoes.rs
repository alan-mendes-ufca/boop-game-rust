//! Entrada do jogador e fluxo do turno. Origem: `funcoes.c`.

use crate::tipos::*;
use std::fmt;
use std::io::{BufRead, Write};

/// Uma jogada ja convertida para indices do tabuleiro (base 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jogada {
    pub peca: Peca,
    pub linha: usize,
    pub coluna: usize,
}

/// Motivo pelo qual a entrada digitada nao pode ser interpretada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntradaInvalida {
    Formato,
    TipoPeca,
    Linha,
    Coluna,
}

impl fmt::Display for EntradaInvalida {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Interpreta uma linha digitada no formato `g 1 A`.
pub fn parse_jogada(_entrada: &str) -> Result<Jogada, EntradaInvalida> {
    todo!()
}

/// Pede uma jogada ao jogador ate receber uma entrada bem formada.
/// `Ok(None)` significa fim da entrada (EOF).
pub fn ler_jogada(
    _entrada: &mut impl BufRead,
    _saida: &mut impl Write,
) -> std::io::Result<Option<Jogada>> {
    todo!()
}

/// Executa o turno: coloca a peca, aplica o boop e gradua os trios resultantes
/// (do jogador da vez e do adversario). Origem: `fluxoJogo` em `funcoes.c`.
pub fn fluxo_jogo(
    _tabuleiro: &mut Tabuleiro,
    _jogada: &Jogada,
    _jogador: Jogador,
    _maos: &mut Maos,
) {
    todo!()
}
