//! Orquestracao de um turno: aplica a jogada sobre o model, na ordem certa.
//! Origem: `fluxoJogo` em `funcoes.c`.

use crate::model::graduar;
use crate::model::jogada::{self, Jogada};
use crate::model::tipos::*;

/// Executa o turno: coloca a peca, aplica o boop e gradua os trios resultantes
/// (do jogador da vez e do adversario).
///
/// A graduacao tem prioridade sobre o boop: se a propria colocacao ja fechou um
/// trio de gatinhos, o empurrao daquele turno nao acontece.
pub fn fluxo_jogo(tabuleiro: &mut Tabuleiro, jogada: &Jogada, jogador: Jogador, maos: &mut Maos) {
    jogada::colocar_peca(
        tabuleiro,
        jogada.linha,
        jogada.coluna,
        jogada.peca,
        jogador,
        maos,
    );

    // Sem esta condicional o boop empurraria justamente as pecas vizinhas que
    // formam o trio com a peca recem colocada, desfazendo o alinhamento antes
    // de a graduacao ser avaliada. Como todo turno termina graduando os dois
    // jogadores, nao existe trio pendente no inicio do turno: qualquer trio
    // visto aqui foi criado por esta jogada, e so o jogador da vez pode te-lo
    // formado (a colocacao nao move nenhuma peca do adversario).
    let jogada_fechou_trio = !graduar::trios_de_gatinhos(tabuleiro, jogador).is_empty();

    if !jogada_fechou_trio {
        jogada::boop_nas_oito_direcoes(tabuleiro, jogada.linha, jogada.coluna, jogada.peca, maos);
    }

    // O boop pode formar trio para qualquer um dos dois jogadores, entao
    // graduamos ambos apos a jogada.
    graduar::graduar(tabuleiro, jogador, maos);
    graduar::graduar(tabuleiro, jogador.oponente(), maos);
}
