//! Validacao da jogada e efeito boop. Origem: `jogada/jogada.c`.

use crate::tipos::*;
use std::fmt;

/// Motivo pelo qual uma jogada foi recusada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JogadaInvalida {
    ForaDoTabuleiro,
    CelulaOcupada,
    SemPecas(Peca),
}

impl fmt::Display for JogadaInvalida {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Verifica se a peca pode ser colocada na casa indicada. Nao altera nada.
pub fn verificar_jogada(
    _tabuleiro: &Tabuleiro,
    _linha: usize,
    _coluna: usize,
    _peca: Peca,
    _mao: &Mao,
) -> Result<(), JogadaInvalida> {
    todo!()
}

/// Coloca a peca na casa e aplica o boop nas 8 direcoes.
/// Assume que [`verificar_jogada`] ja aprovou a jogada.
pub fn faz_boop(
    _tabuleiro: &mut Tabuleiro,
    _linha: usize,
    _coluna: usize,
    _peca: Peca,
    _dono: Jogador,
    _maos: &mut Maos,
) {
    todo!()
}

/// Aplica o boop em uma unica direcao a partir da peca recem colocada.
pub fn aplicar_boop(
    _tabuleiro: &mut Tabuleiro,
    _linha: usize,
    _coluna: usize,
    _direcao: (isize, isize),
    _peca_colocada: Peca,
    _maos: &mut Maos,
) {
    todo!()
}
