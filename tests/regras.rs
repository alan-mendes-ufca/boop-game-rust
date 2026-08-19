//! Testes de integracao das regras do jogo, escritos a partir da
//! especificacao (nao a partir da implementacao) contra a API publica do
//! crate `boop`.
//!
//! Enquanto os modulos de regra ainda contem `todo!()`, este
//! arquivo deve continuar COMPILANDO; os testes so devem passar quando as
//! implementacoes estiverem prontas.

use boop::controller::turno::fluxo_jogo;
use boop::model::graduar::graduar;
use boop::model::jogada::{faz_boop, verificar_jogada, Jogada, JogadaInvalida};
use boop::model::tipos::{Gato, Jogador, Mao, Maos, Peca, Tabuleiro};
use boop::model::vitoria::{estado_do_jogo, verifica_vitoria, verificar_empate, FimDeJogo};
use boop::view::entrada::{parse_jogada, EntradaInvalida};

// ---------------------------------------------------------------------
// Auxiliares de cenario
// ---------------------------------------------------------------------

/// Coloca uma peca no tabuleiro e mantem a mao do dono em sincronia (via
/// `Mao::colocar`), como uma jogada realmente colocada faria.
///
/// Para gatoes, que a mao inicial nao possui, "empresta" um gatinho e o
/// transforma em gatao antes de colocar, preservando a invariante
/// `gatinhos + gatoes + ativas == 8` (equivalente a um trio ja graduado
/// antes do cenario comecar).
fn poe(
    tabuleiro: &mut Tabuleiro,
    maos: &mut Maos,
    linha: usize,
    coluna: usize,
    peca: Peca,
    dono: Jogador,
) {
    if peca == Peca::Gatao && !maos.ver(dono).tem(Peca::Gatao) {
        let mao = maos.de(dono);
        assert!(
            mao.gatinhos > 0,
            "sem gatinho para converter em gatao no cenario de teste"
        );
        mao.gatinhos -= 1;
        mao.gatoes += 1;
    }
    assert!(
        maos.de(dono).colocar(peca),
        "mao de {:?} nao tem {:?} disponivel para o cenario",
        dono,
        peca
    );
    tabuleiro.set(linha, coluna, Some(Gato::novo(peca, dono)));
}

/// `true` se todas as posicoes dadas estao vazias.
fn todas_vazias(tabuleiro: &Tabuleiro, posicoes: &[(usize, usize)]) -> bool {
    posicoes.iter().all(|&(l, c)| tabuleiro.vazia(l, c))
}

// ---------------------------------------------------------------------
// 1. verificar_jogada
// ---------------------------------------------------------------------

mod verificar_jogada_regras {
    use super::*;

    #[test]
    fn aceita_colocar_em_casa_vazia_quando_ha_peca_na_mao() {
        let tabuleiro = Tabuleiro::novo();
        let mao = Mao::nova();
        assert_eq!(
            verificar_jogada(&tabuleiro, 3, 3, Peca::Gatinho, &mao),
            Ok(())
        );
    }

    #[test]
    fn recusa_casa_ja_ocupada() {
        let mut tabuleiro = Tabuleiro::novo();
        tabuleiro.set(1, 1, Some(Gato::novo(Peca::Gatinho, Jogador::Dois)));
        let mao = Mao::nova();
        assert_eq!(
            verificar_jogada(&tabuleiro, 1, 1, Peca::Gatinho, &mao),
            Err(JogadaInvalida::CelulaOcupada)
        );
    }

    #[test]
    fn recusa_coordenada_fora_do_tabuleiro() {
        let tabuleiro = Tabuleiro::novo();
        let mao = Mao::nova();
        // Linha fora dos limites.
        assert_eq!(
            verificar_jogada(&tabuleiro, 6, 2, Peca::Gatinho, &mao),
            Err(JogadaInvalida::ForaDoTabuleiro)
        );
        // Coluna fora dos limites.
        assert_eq!(
            verificar_jogada(&tabuleiro, 2, 6, Peca::Gatinho, &mao),
            Err(JogadaInvalida::ForaDoTabuleiro)
        );
    }

    #[test]
    fn recusa_quando_jogador_nao_tem_a_peca_na_mao() {
        let tabuleiro = Tabuleiro::novo();
        let mut mao = Mao::nova();
        // Mao inicial nao tem gatoes.
        assert_eq!(
            verificar_jogada(&tabuleiro, 0, 0, Peca::Gatao, &mao),
            Err(JogadaInvalida::SemPecas(Peca::Gatao))
        );

        // Gatinhos esgotados tambem devem ser recusados.
        mao.gatinhos = 0;
        mao.ativas = 8;
        assert_eq!(
            verificar_jogada(&tabuleiro, 0, 0, Peca::Gatinho, &mao),
            Err(JogadaInvalida::SemPecas(Peca::Gatinho))
        );
    }
}

// ---------------------------------------------------------------------
// 2 e 4. Boop basico e bloqueio
// ---------------------------------------------------------------------

mod boop_basico {
    use super::*;

    #[test]
    fn peca_adjacente_e_empurrada_uma_casa_para_longe_e_origem_fica_vazia() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // Peca alvo em (3, 3); sera colocada uma peca em (3, 2), empurrando
        // o alvo para (3, 4).
        poe(
            &mut tabuleiro,
            &mut maos,
            3,
            3,
            Peca::Gatinho,
            Jogador::Dois,
        );

        faz_boop(&mut tabuleiro, 3, 2, Peca::Gatinho, Jogador::Um, &mut maos);

        assert_eq!(
            tabuleiro.get(3, 4),
            Some(Gato::novo(Peca::Gatinho, Jogador::Dois)),
            "a peca alvo deveria ter sido empurrada uma casa para longe"
        );
        assert!(
            tabuleiro.vazia(3, 3),
            "a casa de origem da peca empurrada deveria ficar vazia"
        );
        assert_eq!(
            tabuleiro.get(3, 2),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um)),
            "a peca recem-colocada permanece onde foi jogada"
        );
    }

    #[test]
    fn casas_nao_adjacentes_nao_sao_afetadas() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(
            &mut tabuleiro,
            &mut maos,
            0,
            5,
            Peca::Gatinho,
            Jogador::Dois,
        );

        faz_boop(&mut tabuleiro, 3, 3, Peca::Gatinho, Jogador::Um, &mut maos);

        assert_eq!(
            tabuleiro.get(0, 5),
            Some(Gato::novo(Peca::Gatinho, Jogador::Dois)),
            "peca fora da vizinhanca de 8 casas nao deveria se mover"
        );
    }

    #[test]
    fn empurrao_bloqueado_quando_destino_esta_ocupado() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // Alvo em (3, 3) e destino (3, 4) ja ocupado por outra peca.
        poe(
            &mut tabuleiro,
            &mut maos,
            3,
            3,
            Peca::Gatinho,
            Jogador::Dois,
        );
        poe(&mut tabuleiro, &mut maos, 3, 4, Peca::Gatinho, Jogador::Um);

        faz_boop(&mut tabuleiro, 3, 2, Peca::Gatinho, Jogador::Um, &mut maos);

        // Nada se move: as duas pecas continuam onde estavam.
        assert_eq!(
            tabuleiro.get(3, 3),
            Some(Gato::novo(Peca::Gatinho, Jogador::Dois)),
            "peca bloqueada nao deveria se mover"
        );
        assert_eq!(
            tabuleiro.get(3, 4),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um)),
            "peca no destino nao deveria ser afetada"
        );
    }
}

// ---------------------------------------------------------------------
// 3. Regra de forca entre gatinho e gatao
// ---------------------------------------------------------------------

mod boop_forca_das_pecas {
    use super::*;

    #[test]
    fn gatinho_nao_empurra_gatao() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(&mut tabuleiro, &mut maos, 3, 3, Peca::Gatao, Jogador::Dois);

        faz_boop(&mut tabuleiro, 3, 2, Peca::Gatinho, Jogador::Um, &mut maos);

        assert_eq!(
            tabuleiro.get(3, 3),
            Some(Gato::novo(Peca::Gatao, Jogador::Dois)),
            "gatao nao deveria ser movido por um gatinho"
        );
        assert!(
            tabuleiro.vazia(3, 4),
            "destino deveria continuar vazio, pois nada se moveu"
        );
    }

    #[test]
    fn gatao_empurra_gatinho() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(
            &mut tabuleiro,
            &mut maos,
            3,
            3,
            Peca::Gatinho,
            Jogador::Dois,
        );

        faz_boop(&mut tabuleiro, 3, 2, Peca::Gatao, Jogador::Um, &mut maos);

        assert!(tabuleiro.vazia(3, 3));
        assert_eq!(
            tabuleiro.get(3, 4),
            Some(Gato::novo(Peca::Gatinho, Jogador::Dois))
        );
    }

    #[test]
    fn gatao_empurra_outro_gatao() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(&mut tabuleiro, &mut maos, 3, 3, Peca::Gatao, Jogador::Dois);

        faz_boop(&mut tabuleiro, 3, 2, Peca::Gatao, Jogador::Um, &mut maos);

        assert!(tabuleiro.vazia(3, 3));
        assert_eq!(
            tabuleiro.get(3, 4),
            Some(Gato::novo(Peca::Gatao, Jogador::Dois))
        );
    }

    #[test]
    fn gatinho_empurra_outro_gatinho() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(
            &mut tabuleiro,
            &mut maos,
            3,
            3,
            Peca::Gatinho,
            Jogador::Dois,
        );

        faz_boop(&mut tabuleiro, 3, 2, Peca::Gatinho, Jogador::Um, &mut maos);

        assert!(tabuleiro.vazia(3, 3));
        assert_eq!(
            tabuleiro.get(3, 4),
            Some(Gato::novo(Peca::Gatinho, Jogador::Dois))
        );
    }
}

// ---------------------------------------------------------------------
// 5. Peca empurrada para fora do tabuleiro
// ---------------------------------------------------------------------

mod boop_para_fora_do_tabuleiro {
    use super::*;

    #[test]
    fn peca_do_adversario_empurrada_para_fora_volta_para_a_mao_do_proprio_dono() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // A peca que sera empurrada para fora pertence ao ADVERSARIO
        // (Jogador::Dois) do jogador da vez (Jogador::Um), na borda inferior.
        poe(
            &mut tabuleiro,
            &mut maos,
            5,
            3,
            Peca::Gatinho,
            Jogador::Dois,
        );

        let mao_um_antes = *maos.ver(Jogador::Um);
        let mao_dois_antes = *maos.ver(Jogador::Dois);

        // Jogador::Um coloca em (4, 3); a direcao (1, 0) alcanca (5, 3) e o
        // destino seguinte (6, 3) fica fora do tabuleiro.
        faz_boop(&mut tabuleiro, 4, 3, Peca::Gatinho, Jogador::Um, &mut maos);

        assert!(
            tabuleiro.vazia(5, 3),
            "peca empurrada para fora some do tabuleiro"
        );
        assert_eq!(
            tabuleiro.get(4, 3),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um)),
            "a peca recem colocada permanece no tabuleiro"
        );

        // A mao do DONO da peca empurrada (Jogador::Dois) ganhou a peca de
        // volta e sua contagem de pecas ativas caiu.
        let mao_dois_depois = *maos.ver(Jogador::Dois);
        assert_eq!(mao_dois_depois.gatinhos, mao_dois_antes.gatinhos + 1);
        assert_eq!(mao_dois_depois.ativas, mao_dois_antes.ativas - 1);
        assert_eq!(mao_dois_depois.total(), 8);

        // A mao do jogador da vez (Jogador::Um) so mudou pela peca que ele
        // proprio colocou, nao recebeu a peca do adversario.
        let mao_um_depois = *maos.ver(Jogador::Um);
        assert_eq!(mao_um_depois.gatinhos, mao_um_antes.gatinhos - 1);
        assert_eq!(mao_um_depois.ativas, mao_um_antes.ativas + 1);
        assert_eq!(mao_um_depois.gatoes, mao_um_antes.gatoes);
        assert_eq!(mao_um_depois.total(), 8);
    }

    #[test]
    fn peca_propria_empurrada_para_fora_tambem_volta_para_a_mao_de_quem_a_possuia() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // Desta vez a peca empurrada para fora e do PROPRIO jogador da vez.
        poe(&mut tabuleiro, &mut maos, 0, 2, Peca::Gatinho, Jogador::Um);
        let ativas_antes = maos.ver(Jogador::Um).ativas;
        let gatinhos_antes = maos.ver(Jogador::Um).gatinhos;

        // Direcao (-1, 0) a partir de (1, 2) alcanca (0, 2); destino
        // seguinte (-1, 2) fica fora do tabuleiro.
        faz_boop(&mut tabuleiro, 1, 2, Peca::Gatao, Jogador::Dois, &mut maos);

        assert!(tabuleiro.vazia(0, 2));
        assert_eq!(maos.ver(Jogador::Um).gatinhos, gatinhos_antes + 1);
        assert_eq!(maos.ver(Jogador::Um).ativas, ativas_antes - 1);
    }
}

// ---------------------------------------------------------------------
// 6, 7 e 8. Graduacao de trios
// ---------------------------------------------------------------------

mod graduacao_de_trios {
    use super::*;

    #[test]
    fn trio_horizontal_gradua_e_dono_recebe_tres_gatoes() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        let posicoes = [(2, 0), (2, 1), (2, 2)];
        for &(l, c) in &posicoes {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatinho, Jogador::Um);
        }

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 1);
        assert!(todas_vazias(&tabuleiro, &posicoes));
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
        assert_eq!(maos.ver(Jogador::Um).ativas, 0);
        assert_eq!(maos.ver(Jogador::Um).total(), 8);
    }

    #[test]
    fn trio_vertical_gradua_e_dono_recebe_tres_gatoes() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        let posicoes = [(0, 4), (1, 4), (2, 4)];
        for &(l, c) in &posicoes {
            poe(
                &mut tabuleiro,
                &mut maos,
                l,
                c,
                Peca::Gatinho,
                Jogador::Dois,
            );
        }

        let quantos = graduar(&mut tabuleiro, Jogador::Dois, &mut maos);

        assert_eq!(quantos, 1);
        assert!(todas_vazias(&tabuleiro, &posicoes));
        assert_eq!(maos.ver(Jogador::Dois).gatoes, 3);
    }

    #[test]
    fn trio_diagonal_gradua_e_dono_recebe_tres_gatoes() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // Diagonal principal (direcao (1, 1)).
        let posicoes = [(1, 1), (2, 2), (3, 3)];
        for &(l, c) in &posicoes {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatinho, Jogador::Um);
        }

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 1);
        assert!(todas_vazias(&tabuleiro, &posicoes));
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
    }

    #[test]
    fn trio_diagonal_secundaria_gradua_e_dono_recebe_tres_gatoes() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // Diagonal secundaria (direcao (-1, 1)): sobe uma linha e avanca
        // uma coluna a cada passo.
        let posicoes = [(5, 0), (4, 1), (3, 2)];
        for &(l, c) in &posicoes {
            poe(
                &mut tabuleiro,
                &mut maos,
                l,
                c,
                Peca::Gatinho,
                Jogador::Dois,
            );
        }

        let quantos = graduar(&mut tabuleiro, Jogador::Dois, &mut maos);

        assert_eq!(quantos, 1);
        assert!(todas_vazias(&tabuleiro, &posicoes));
        assert_eq!(maos.ver(Jogador::Dois).gatoes, 3);
    }

    #[test]
    fn quatro_gatinhos_em_linha_graduam_apenas_um_trio() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        let posicoes = [(4, 0), (4, 1), (4, 2), (4, 3)];
        for &(l, c) in &posicoes {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatinho, Jogador::Um);
        }

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 1, "quatro em linha devem formar apenas um trio");
        // Exatamente uma peca deveria sobrar no tabuleiro.
        let restantes = posicoes
            .iter()
            .filter(|&&(l, c)| !tabuleiro.vazia(l, c))
            .count();
        assert_eq!(restantes, 1, "uma peca deveria sobrar sem formar novo trio");
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
        assert_eq!(maos.ver(Jogador::Um).ativas, 1);
        assert_eq!(maos.ver(Jogador::Um).total(), 8);
    }

    #[test]
    fn trio_de_gatoes_nao_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        let posicoes = [(0, 0), (0, 1), (0, 2)];
        for &(l, c) in &posicoes {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatao, Jogador::Um);
        }

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 0);
        assert!(!todas_vazias(&tabuleiro, &posicoes));
        assert_eq!(
            maos.ver(Jogador::Um).ativas,
            3,
            "nenhuma peca deveria ter saido do tabuleiro"
        );
    }

    #[test]
    fn trio_com_donos_diferentes_nao_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(&mut tabuleiro, &mut maos, 1, 0, Peca::Gatinho, Jogador::Um);
        poe(&mut tabuleiro, &mut maos, 1, 1, Peca::Gatinho, Jogador::Um);
        poe(
            &mut tabuleiro,
            &mut maos,
            1,
            2,
            Peca::Gatinho,
            Jogador::Dois,
        );

        let quantos_um = graduar(&mut tabuleiro, Jogador::Um, &mut maos);
        let quantos_dois = graduar(&mut tabuleiro, Jogador::Dois, &mut maos);

        assert_eq!(quantos_um, 0);
        assert_eq!(quantos_dois, 0);
        assert!(!tabuleiro.vazia(1, 0));
        assert!(!tabuleiro.vazia(1, 1));
        assert!(!tabuleiro.vazia(1, 2));
    }
}

// ---------------------------------------------------------------------
// 9, 10 e 11. Vitoria
// ---------------------------------------------------------------------

mod condicoes_de_vitoria {
    use super::*;

    #[test]
    fn tabuleiro_e_maos_iniciais_nao_produzem_vitoria() {
        let tabuleiro = Tabuleiro::novo();
        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), None);
    }

    #[test]
    fn trio_horizontal_de_gatoes_vence_para_o_dono_certo() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        for &(l, c) in &[(3, 0), (3, 1), (3, 2)] {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatao, Jogador::Um);
        }

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Um));
    }

    #[test]
    fn trio_vertical_de_gatoes_vence_para_o_dono_certo() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        for &(l, c) in &[(2, 5), (3, 5), (4, 5)] {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatao, Jogador::Dois);
        }

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Dois));
    }

    #[test]
    fn trio_diagonal_de_gatoes_vence_para_o_dono_certo() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        for &(l, c) in &[(3, 3), (4, 4), (5, 5)] {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatao, Jogador::Um);
        }

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Um));
    }

    #[test]
    fn trio_diagonal_secundaria_de_gatoes_vence_para_o_dono_certo() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        for &(l, c) in &[(5, 0), (4, 1), (3, 2)] {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatao, Jogador::Dois);
        }

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Dois));
    }

    #[test]
    fn trio_de_gatoes_com_donos_misturados_nao_vence() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(&mut tabuleiro, &mut maos, 0, 0, Peca::Gatao, Jogador::Um);
        poe(&mut tabuleiro, &mut maos, 0, 1, Peca::Gatao, Jogador::Dois);
        poe(&mut tabuleiro, &mut maos, 0, 2, Peca::Gatao, Jogador::Um);

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), None);
    }

    #[test]
    fn trio_de_gatinhos_nao_vence() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        for &(l, c) in &[(0, 0), (0, 1), (0, 2)] {
            poe(&mut tabuleiro, &mut maos, l, c, Peca::Gatinho, Jogador::Um);
        }

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), None);
    }

    #[test]
    fn cama_cheia_vence_para_o_jogador_com_oito_pecas_ativas() {
        let tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 8,
        };

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Dois));
    }

    #[test]
    fn sete_pecas_ativas_nao_e_cama_cheia() {
        let tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        maos.jogador1 = Mao {
            gatinhos: 1,
            gatoes: 0,
            ativas: 7,
        };

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), None);
    }
}

// ---------------------------------------------------------------------
// 12. Empate e prioridade de estado_do_jogo
// ---------------------------------------------------------------------

mod empate_e_estado_do_jogo {
    use super::*;

    #[test]
    fn empate_quando_as_duas_maos_estao_vazias() {
        let mut maos = Maos::novas();
        maos.jogador1 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 8,
        };
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 8,
        };

        assert!(verificar_empate(&maos));
    }

    #[test]
    fn sem_empate_quando_um_jogador_ainda_tem_pecas_na_mao() {
        let mut maos = Maos::novas();
        maos.jogador1 = Mao {
            gatinhos: 0,
            gatoes: 1,
            ativas: 7,
        };
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 8,
        };

        assert!(!verificar_empate(&maos));
    }

    #[test]
    fn estado_do_jogo_relata_em_andamento_no_inicio() {
        let tabuleiro = Tabuleiro::novo();
        let maos = Maos::novas();
        assert_eq!(estado_do_jogo(&tabuleiro, &maos), None);
    }

    #[test]
    fn estado_do_jogo_relata_empate_quando_nao_ha_vitoria() {
        let tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // Maos sem pecas para jogar, mas SEM cama cheia (ativas < 8) e sem
        // nenhum trio de gatoes no tabuleiro: nao ha vitoria possivel, so
        // empate.
        maos.jogador1 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 3,
        };
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 2,
        };

        assert_eq!(estado_do_jogo(&tabuleiro, &maos), Some(FimDeJogo::Empate));
    }

    #[test]
    fn estado_do_jogo_prioriza_vitoria_sobre_empate() {
        // Ambas as maos estao vazias (condicao de empate satisfeita), mas ha
        // tambem um trio de gatoes no tabuleiro: a vitoria deve prevalecer.
        let mut tabuleiro = Tabuleiro::novo();
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 1, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let mut maos = Maos::novas();
        maos.jogador1 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 0,
        };
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 0,
        };

        // A condicao de empate, isoladamente, seria verdadeira aqui.
        assert!(verificar_empate(&maos));
        // Mas o estado do jogo deve reportar a vitoria, nao o empate.
        assert_eq!(
            estado_do_jogo(&tabuleiro, &maos),
            Some(FimDeJogo::Vitoria(Jogador::Um))
        );
    }
}

// ---------------------------------------------------------------------
// 13. parse_jogada
// ---------------------------------------------------------------------

mod parse_jogada_regras {
    use super::*;

    #[test]
    fn aceita_entrada_valida_no_canto_superior_esquerdo() {
        assert_eq!(
            parse_jogada("g 1 A"),
            Ok(Jogada {
                peca: Peca::Gatinho,
                linha: 0,
                coluna: 0
            })
        );
    }

    #[test]
    fn aceita_entrada_valida_no_canto_inferior_direito() {
        assert_eq!(
            parse_jogada("G 6 F"),
            Ok(Jogada {
                peca: Peca::Gatao,
                linha: 5,
                coluna: 5
            })
        );
    }

    #[test]
    fn aceita_coluna_minuscula() {
        assert_eq!(
            parse_jogada("g 4 d"),
            Ok(Jogada {
                peca: Peca::Gatinho,
                linha: 3,
                coluna: 3
            })
        );
    }

    #[test]
    fn recusa_linha_zero() {
        assert_eq!(parse_jogada("g 0 A"), Err(EntradaInvalida::Linha));
    }

    #[test]
    fn recusa_linha_maior_que_seis() {
        assert_eq!(parse_jogada("g 7 A"), Err(EntradaInvalida::Linha));
    }

    #[test]
    fn recusa_coluna_alem_de_f() {
        assert_eq!(parse_jogada("g 1 G"), Err(EntradaInvalida::Coluna));
    }

    #[test]
    fn recusa_tipo_de_peca_desconhecido() {
        assert_eq!(parse_jogada("x 1 A"), Err(EntradaInvalida::TipoPeca));
    }

    #[test]
    fn recusa_formato_com_campos_faltando_ou_sobrando() {
        assert_eq!(parse_jogada("g 1"), Err(EntradaInvalida::Formato));
        assert_eq!(parse_jogada("g 1 A B"), Err(EntradaInvalida::Formato));
        assert_eq!(parse_jogada(""), Err(EntradaInvalida::Formato));
    }
}

// ---------------------------------------------------------------------
// 14. Invariante das pecas apos fluxo_jogo
// ---------------------------------------------------------------------

mod invariante_das_pecas {
    use super::*;

    #[test]
    fn total_de_pecas_de_cada_jogador_permanece_oito_apos_sequencia_de_jogadas() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();

        let jogadas: [(Jogador, Jogada); 6] = [
            (
                Jogador::Um,
                Jogada {
                    peca: Peca::Gatinho,
                    linha: 2,
                    coluna: 2,
                },
            ),
            (
                Jogador::Dois,
                Jogada {
                    peca: Peca::Gatinho,
                    linha: 3,
                    coluna: 3,
                },
            ),
            (
                Jogador::Um,
                Jogada {
                    peca: Peca::Gatinho,
                    linha: 2,
                    coluna: 3,
                },
            ),
            (
                Jogador::Dois,
                Jogada {
                    peca: Peca::Gatinho,
                    linha: 0,
                    coluna: 0,
                },
            ),
            (
                Jogador::Um,
                Jogada {
                    peca: Peca::Gatinho,
                    linha: 3,
                    coluna: 2,
                },
            ),
            (
                Jogador::Dois,
                Jogada {
                    peca: Peca::Gatinho,
                    linha: 5,
                    coluna: 5,
                },
            ),
        ];

        for (jogador, jogada) in jogadas {
            fluxo_jogo(&mut tabuleiro, &jogada, jogador, &mut maos);
            assert_eq!(
                maos.ver(Jogador::Um).total(),
                8,
                "invariante quebrada para o Jogador 1"
            );
            assert_eq!(
                maos.ver(Jogador::Dois).total(),
                8,
                "invariante quebrada para o Jogador 2"
            );
        }
    }

    #[test]
    fn fluxo_jogo_gradua_trio_formado_pela_propria_jogada() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // Duas pecas do jogador 1 ja no tabuleiro, faltando a terceira para
        // fechar o trio horizontal.
        poe(&mut tabuleiro, &mut maos, 0, 0, Peca::Gatinho, Jogador::Um);
        poe(&mut tabuleiro, &mut maos, 0, 1, Peca::Gatinho, Jogador::Um);

        let jogada = Jogada {
            peca: Peca::Gatinho,
            linha: 0,
            coluna: 2,
        };
        fluxo_jogo(&mut tabuleiro, &jogada, Jogador::Um, &mut maos);

        assert!(todas_vazias(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
        assert_eq!(maos.ver(Jogador::Um).total(), 8);
    }
}

// ---------------------------------------------------------------------
// 15. Graduacao tem prioridade sobre o boop
// ---------------------------------------------------------------------

mod graduacao_tem_prioridade_sobre_o_boop {
    use super::*;

    #[test]
    fn jogada_que_fecha_trio_pelo_meio_nao_boopa_as_pontas() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // Duas pontas do trio, com o buraco no meio: sem a prioridade da
        // graduacao o boop expulsaria (0, 0) do tabuleiro e empurraria
        // (0, 2) para (0, 3), desfazendo o alinhamento.
        poe(&mut tabuleiro, &mut maos, 0, 0, Peca::Gatinho, Jogador::Um);
        poe(&mut tabuleiro, &mut maos, 0, 2, Peca::Gatinho, Jogador::Um);
        let gatinhos_antes = maos.ver(Jogador::Um).gatinhos;

        let jogada = Jogada {
            peca: Peca::Gatinho,
            linha: 0,
            coluna: 1,
        };
        fluxo_jogo(&mut tabuleiro, &jogada, Jogador::Um, &mut maos);

        assert!(todas_vazias(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
        assert!(todas_vazias(&tabuleiro, &[(0, 3)]), "ninguem foi empurrado");
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
        // O gatinho de (0, 0) graduou; nao voltou para a mao por expulsao.
        assert_eq!(maos.ver(Jogador::Um).gatinhos, gatinhos_antes - 1);
        assert_eq!(maos.ver(Jogador::Um).ativas, 0);
        assert_eq!(maos.ver(Jogador::Um).total(), 8);
    }

    #[test]
    fn jogada_que_fecha_trio_nao_empurra_peca_do_adversario() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(&mut tabuleiro, &mut maos, 2, 1, Peca::Gatinho, Jogador::Um);
        poe(&mut tabuleiro, &mut maos, 2, 3, Peca::Gatinho, Jogador::Um);
        // Peca do adversario vizinha da casa jogada, com destino livre.
        poe(
            &mut tabuleiro,
            &mut maos,
            1,
            2,
            Peca::Gatinho,
            Jogador::Dois,
        );

        let jogada = Jogada {
            peca: Peca::Gatinho,
            linha: 2,
            coluna: 2,
        };
        fluxo_jogo(&mut tabuleiro, &jogada, Jogador::Um, &mut maos);

        // A supressao do boop e total: nem as pecas do adversario se movem.
        assert_eq!(
            tabuleiro.get(1, 2),
            Some(Gato::novo(Peca::Gatinho, Jogador::Dois))
        );
        assert!(todas_vazias(&tabuleiro, &[(0, 2)]));
        assert!(todas_vazias(&tabuleiro, &[(2, 1), (2, 2), (2, 3)]));
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
        assert_eq!(maos.ver(Jogador::Um).total(), 8);
        assert_eq!(maos.ver(Jogador::Dois).total(), 8);
    }

    #[test]
    fn gatao_colocado_entre_gatinhos_continua_boopando() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(&mut tabuleiro, &mut maos, 2, 1, Peca::Gatinho, Jogador::Um);
        poe(&mut tabuleiro, &mut maos, 2, 3, Peca::Gatinho, Jogador::Um);
        // Um gatao na mao (equivalente a um trio graduado antes do cenario).
        let mao = maos.de(Jogador::Um);
        mao.gatinhos -= 1;
        mao.gatoes += 1;

        let jogada = Jogada {
            peca: Peca::Gatao,
            linha: 2,
            coluna: 2,
        };
        fluxo_jogo(&mut tabuleiro, &jogada, Jogador::Um, &mut maos);

        // Gatao no meio nao forma trio de gatinhos, entao o boop acontece.
        assert!(todas_vazias(&tabuleiro, &[(2, 1), (2, 3)]));
        assert_eq!(
            tabuleiro.get(2, 0),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um))
        );
        assert_eq!(
            tabuleiro.get(2, 4),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um))
        );
        assert_eq!(
            tabuleiro.get(2, 2),
            Some(Gato::novo(Peca::Gatao, Jogador::Um))
        );
        assert_eq!(maos.ver(Jogador::Um).total(), 8);
    }

    #[test]
    fn jogada_que_nao_fecha_trio_continua_boopando() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(&mut tabuleiro, &mut maos, 3, 3, Peca::Gatinho, Jogador::Um);

        let jogada = Jogada {
            peca: Peca::Gatinho,
            linha: 3,
            coluna: 2,
        };
        fluxo_jogo(&mut tabuleiro, &jogada, Jogador::Um, &mut maos);

        assert!(todas_vazias(&tabuleiro, &[(3, 3)]));
        assert_eq!(
            tabuleiro.get(3, 4),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um))
        );
        assert_eq!(
            tabuleiro.get(3, 2),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um))
        );
        assert_eq!(maos.ver(Jogador::Um).gatoes, 0);
        assert_eq!(maos.ver(Jogador::Um).total(), 8);
    }

    #[test]
    fn trio_formado_pelo_proprio_boop_ainda_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        poe(&mut tabuleiro, &mut maos, 2, 0, Peca::Gatinho, Jogador::Um);
        poe(&mut tabuleiro, &mut maos, 2, 1, Peca::Gatinho, Jogador::Um);
        poe(&mut tabuleiro, &mut maos, 3, 2, Peca::Gatinho, Jogador::Um);

        // A colocacao em (4, 2) nao fecha trio, entao o boop acontece e
        // empurra (3, 2) para (2, 2), completando a linha 2.
        let jogada = Jogada {
            peca: Peca::Gatinho,
            linha: 4,
            coluna: 2,
        };
        fluxo_jogo(&mut tabuleiro, &jogada, Jogador::Um, &mut maos);

        assert!(todas_vazias(&tabuleiro, &[(2, 0), (2, 1), (2, 2), (3, 2)]));
        assert_eq!(
            tabuleiro.get(4, 2),
            Some(Gato::novo(Peca::Gatinho, Jogador::Um))
        );
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
        assert_eq!(maos.ver(Jogador::Um).ativas, 1);
        assert_eq!(maos.ver(Jogador::Um).total(), 8);
    }
}
