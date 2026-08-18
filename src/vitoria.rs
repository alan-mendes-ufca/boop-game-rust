//! Condicoes de fim de jogo. Origem: `vitoria/vencer.c`.

use crate::tipos::*;

/// Como a partida terminou.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FimDeJogo {
    Vitoria(Jogador),
    Empate,
}

/// Retorna o vencedor, se houver.
pub fn verifica_vitoria(_tabuleiro: &Tabuleiro, _maos: &Maos) -> Option<Jogador> {
    todo!()
}

/// `true` se nenhum dos jogadores tem pecas na mao para jogar.
pub fn verificar_empate(_maos: &Maos) -> bool {
    todo!()
}

/// Avalia o estado da partida: vitoria, empate ou jogo em andamento.
pub fn estado_do_jogo(_tabuleiro: &Tabuleiro, _maos: &Maos) -> Option<FimDeJogo> {
    todo!()
}
