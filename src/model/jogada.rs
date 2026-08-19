//! Validacao da jogada e efeito boop. Origem: `jogada/jogada.c`.

use crate::model::tipos::*;
use std::fmt;

/// Uma jogada ja convertida para indices do tabuleiro (base 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jogada {
    pub peca: Peca,
    pub linha: usize,
    pub coluna: usize,
}

/// Motivo pelo qual uma jogada foi recusada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JogadaInvalida {
    ForaDoTabuleiro,
    CelulaOcupada,
    SemPecas(Peca),
}

impl fmt::Display for JogadaInvalida {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JogadaInvalida::ForaDoTabuleiro => {
                write!(f, "Posição fora dos limites do tabuleiro!")
            }
            JogadaInvalida::CelulaOcupada => write!(f, "Célula ocupada!"),
            JogadaInvalida::SemPecas(Peca::Gatinho) => {
                write!(f, "Você não tem gatinhos disponíveis!")
            }
            JogadaInvalida::SemPecas(Peca::Gatao) => {
                write!(f, "Você não tem gatões disponíveis!")
            }
        }
    }
}

/// Verifica se a peca pode ser colocada na casa indicada. Nao altera nada.
pub fn verificar_jogada(
    tabuleiro: &Tabuleiro,
    linha: usize,
    coluna: usize,
    peca: Peca,
    mao: &Mao,
) -> Result<(), JogadaInvalida> {
    if linha >= TAMANHO_TABULEIRO || coluna >= TAMANHO_TABULEIRO {
        return Err(JogadaInvalida::ForaDoTabuleiro);
    }
    if !tabuleiro.vazia(linha, coluna) {
        return Err(JogadaInvalida::CelulaOcupada);
    }
    if !mao.tem(peca) {
        return Err(JogadaInvalida::SemPecas(peca));
    }
    Ok(())
}

/// Coloca a peca na casa, sem empurrar nada.
/// Assume que [`verificar_jogada`] ja aprovou a jogada.
pub fn colocar_peca(
    tabuleiro: &mut Tabuleiro,
    linha: usize,
    coluna: usize,
    peca: Peca,
    dono: Jogador,
    maos: &mut Maos,
) {
    maos.de(dono).colocar(peca);
    tabuleiro.set(linha, coluna, Some(Gato::novo(peca, dono)));
}

/// Aplica o boop nas 8 direcoes a partir da casa indicada.
pub fn boop_nas_oito_direcoes(
    tabuleiro: &mut Tabuleiro,
    linha: usize,
    coluna: usize,
    peca: Peca,
    maos: &mut Maos,
) {
    // As direcoes sao aplicadas em sequencia, sobre o estado corrente do
    // tabuleiro (como o C faz: cada direcao ve o resultado das anteriores).
    for direcao in DIRECOES_BOOP {
        aplicar_boop(tabuleiro, linha, coluna, direcao, peca, maos);
    }
}

/// Coloca a peca na casa e aplica o boop nas 8 direcoes.
/// Assume que [`verificar_jogada`] ja aprovou a jogada.
///
/// Esta funcao empurra sempre; a regra de que a graduacao tem prioridade
/// sobre o boop mora em `funcoes::fluxo_jogo`, que decide se chama
/// [`boop_nas_oito_direcoes`] depois de [`colocar_peca`].
pub fn faz_boop(
    tabuleiro: &mut Tabuleiro,
    linha: usize,
    coluna: usize,
    peca: Peca,
    dono: Jogador,
    maos: &mut Maos,
) {
    colocar_peca(tabuleiro, linha, coluna, peca, dono, maos);
    boop_nas_oito_direcoes(tabuleiro, linha, coluna, peca, maos);
}

/// Aplica o boop em uma unica direcao a partir da peca recem colocada.
pub fn aplicar_boop(
    tabuleiro: &mut Tabuleiro,
    linha: usize,
    coluna: usize,
    direcao: (isize, isize),
    peca_colocada: Peca,
    maos: &mut Maos,
) {
    // Casa adjacente na direcao. Fora do tabuleiro ou vazia: nada acontece.
    let Some((alvo_linha, alvo_coluna)) = deslocar(linha, coluna, direcao) else {
        return;
    };
    let Some(gato_alvo) = tabuleiro.get(alvo_linha, alvo_coluna) else {
        return;
    };

    // Gatinho nao empurra gatao. Gatao empurra qualquer peca.
    if peca_colocada == Peca::Gatinho && gato_alvo.peca == Peca::Gatao {
        return;
    }

    // Casa seguinte, na mesma direcao, a partir do alvo.
    match deslocar(alvo_linha, alvo_coluna, direcao) {
        Some((destino_linha, destino_coluna)) => {
            if tabuleiro.vazia(destino_linha, destino_coluna) {
                // Move: alvo vai para o destino, peca e dono preservados.
                tabuleiro.set(destino_linha, destino_coluna, Some(gato_alvo));
                tabuleiro.set(alvo_linha, alvo_coluna, None);
            }
            // Destino ocupado: nada acontece (pecas lado a lado nao se boopam).
        }
        None => {
            // Destino fora do tabuleiro: a peca alvo sai e volta para a mao
            // do SEU proprio dono (nunca do jogador da vez).
            maos.de(gato_alvo.dono).devolver(gato_alvo.peca);
            tabuleiro.set(alvo_linha, alvo_coluna, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jogada_valida_em_casa_vazia() {
        let tabuleiro = Tabuleiro::novo();
        let mao = Mao::nova();
        assert_eq!(
            verificar_jogada(&tabuleiro, 2, 3, Peca::Gatinho, &mao),
            Ok(())
        );
    }

    #[test]
    fn jogada_recusada_casa_ocupada() {
        let mut tabuleiro = Tabuleiro::novo();
        tabuleiro.set(2, 3, Some(Gato::novo(Peca::Gatinho, Jogador::Um)));
        let mao = Mao::nova();
        assert_eq!(
            verificar_jogada(&tabuleiro, 2, 3, Peca::Gatinho, &mao),
            Err(JogadaInvalida::CelulaOcupada)
        );
    }

    #[test]
    fn jogada_recusada_fora_do_tabuleiro() {
        let tabuleiro = Tabuleiro::novo();
        let mao = Mao::nova();
        assert_eq!(
            verificar_jogada(&tabuleiro, 6, 0, Peca::Gatinho, &mao),
            Err(JogadaInvalida::ForaDoTabuleiro)
        );
        assert_eq!(
            verificar_jogada(&tabuleiro, 0, 100, Peca::Gatao, &mao),
            Err(JogadaInvalida::ForaDoTabuleiro)
        );
    }

    #[test]
    fn jogada_recusada_sem_pecas_na_mao() {
        let tabuleiro = Tabuleiro::novo();
        let mut mao = Mao::nova();
        mao.gatinhos = 0;
        mao.ativas = 8;
        assert_eq!(
            verificar_jogada(&tabuleiro, 0, 0, Peca::Gatinho, &mao),
            Err(JogadaInvalida::SemPecas(Peca::Gatinho))
        );
        assert_eq!(
            verificar_jogada(&tabuleiro, 0, 0, Peca::Gatao, &mao),
            Err(JogadaInvalida::SemPecas(Peca::Gatao))
        );
    }

    #[test]
    fn gatinho_nao_empurra_gatao() {
        let mut tabuleiro = Tabuleiro::novo();
        // Gatao do jogador 2 em (2, 3); destino (2, 4) vazio.
        tabuleiro.set(2, 3, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        let mut maos = Maos::novas();
        // Gatinho do jogador 1 colocado em (2, 2), boop para a direita.
        faz_boop(&mut tabuleiro, 2, 2, Peca::Gatinho, Jogador::Um, &mut maos);

        // O gatao nao se move.
        assert_eq!(
            tabuleiro.get(2, 3),
            Some(Gato::novo(Peca::Gatao, Jogador::Dois))
        );
        assert_eq!(tabuleiro.get(2, 4), None);
        assert_eq!(
            tabuleiro.get(2, 2),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um))
        );
    }

    #[test]
    fn gatao_empurra_gatinho() {
        let mut tabuleiro = Tabuleiro::novo();
        tabuleiro.set(2, 3, Some(Gato::novo(Peca::Gatinho, Jogador::Dois)));
        let mut maos = Maos::novas();
        // Da um gatao ao Jogador::Um para que a jogada seja realista
        // (uma mao real so tem gatao apos um trio se graduar).
        maos.de(Jogador::Um).gatinhos = 7;
        maos.de(Jogador::Um).gatoes = 1;
        faz_boop(&mut tabuleiro, 2, 2, Peca::Gatao, Jogador::Um, &mut maos);

        assert_eq!(tabuleiro.get(2, 3), None);
        assert_eq!(
            tabuleiro.get(2, 4),
            Some(Gato::novo(Peca::Gatinho, Jogador::Dois))
        );
        assert_eq!(
            tabuleiro.get(2, 2),
            Some(Gato::novo(Peca::Gatao, Jogador::Um))
        );
    }

    #[test]
    fn empurrao_bloqueado_por_peca_no_destino() {
        let mut tabuleiro = Tabuleiro::novo();
        tabuleiro.set(2, 3, Some(Gato::novo(Peca::Gatinho, Jogador::Dois)));
        tabuleiro.set(2, 4, Some(Gato::novo(Peca::Gatinho, Jogador::Um)));
        let mut maos = Maos::novas();
        maos.de(Jogador::Um).gatinhos = 7;
        maos.de(Jogador::Um).gatoes = 1;
        faz_boop(&mut tabuleiro, 2, 2, Peca::Gatao, Jogador::Um, &mut maos);

        // Nada se move: destino ja ocupado.
        assert_eq!(
            tabuleiro.get(2, 3),
            Some(Gato::novo(Peca::Gatinho, Jogador::Dois))
        );
        assert_eq!(
            tabuleiro.get(2, 4),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um))
        );
    }

    #[test]
    fn peca_empurrada_para_fora_volta_para_a_mao_do_dono_correto() {
        let mut tabuleiro = Tabuleiro::novo();
        // Peca do adversario (Jogador::Dois) perto da borda, prestes a ser
        // empurrada para fora do tabuleiro pelo Jogador::Um.
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatinho, Jogador::Dois)));
        let mut maos = Maos::novas();
        maos.de(Jogador::Dois).colocar(Peca::Gatinho); // essa peca ja esta em jogo
        maos.de(Jogador::Um).gatinhos = 7;
        maos.de(Jogador::Um).gatoes = 1;

        let gatinhos_j1_antes = maos.ver(Jogador::Um).gatinhos;
        let gatinhos_j2_antes = maos.ver(Jogador::Dois).gatinhos;

        // Jogador::Um coloca gatao em (0, 1); direcao (0, -1) alcanca (0, 0)
        // e o destino seguinte (0, -1) fica fora do tabuleiro.
        faz_boop(&mut tabuleiro, 0, 1, Peca::Gatao, Jogador::Um, &mut maos);

        assert_eq!(tabuleiro.get(0, 0), None);
        assert_eq!(
            tabuleiro.get(0, 1),
            Some(Gato::novo(Peca::Gatao, Jogador::Um))
        );
        // A peca empurrada era do Jogador::Dois: deve voltar para a mao dele,
        // nunca para a mao do jogador da vez (Jogador::Um).
        assert_eq!(maos.ver(Jogador::Dois).gatinhos, gatinhos_j2_antes + 1);
        assert_eq!(maos.ver(Jogador::Um).gatinhos, gatinhos_j1_antes);
    }

    #[test]
    fn peca_no_canto_empurrada_em_diagonal_sai_do_tabuleiro() {
        let mut tabuleiro = Tabuleiro::novo();
        // Gatinho do jogador 2 no canto (0, 0).
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatinho, Jogador::Dois)));
        let mut maos = Maos::novas();
        maos.de(Jogador::Dois).colocar(Peca::Gatinho);
        maos.de(Jogador::Um).gatinhos = 7;
        maos.de(Jogador::Um).gatoes = 1;
        let gatinhos_j2_antes = maos.ver(Jogador::Dois).gatinhos;

        // Jogador::Um coloca um gatao em (1, 1); direcao diagonal (-1, -1)
        // alcanca (0, 0) e o destino seguinte (-1, -1) esta fora do tabuleiro.
        faz_boop(&mut tabuleiro, 1, 1, Peca::Gatao, Jogador::Um, &mut maos);

        assert_eq!(tabuleiro.get(0, 0), None);
        assert_eq!(
            tabuleiro.get(1, 1),
            Some(Gato::novo(Peca::Gatao, Jogador::Um))
        );
        assert_eq!(maos.ver(Jogador::Dois).gatinhos, gatinhos_j2_antes + 1);
    }
}
