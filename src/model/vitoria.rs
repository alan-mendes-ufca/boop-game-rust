//! Condicoes de fim de jogo. Origem: `vitoria/vencer.c`.

use crate::model::tipos::*;

/// Como a partida terminou.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FimDeJogo {
    Vitoria(Jogador),
    Empate,
}

/// Retorna o vencedor, se houver.
///
/// Vitoria pode ocorrer por:
/// 1. Cama cheia: um jogador tem todas as 8 pecas ativas no tabuleiro.
/// 2. Trio de gatoes: 3 gatoes consecutivos do mesmo dono em qualquer direcao.
pub fn verifica_vitoria(tabuleiro: &Tabuleiro, maos: &Maos) -> Option<Jogador> {
    // Checar cama cheia: prioridade ao jogador 1
    if maos.ver(Jogador::Um).ativas == PECAS_POR_JOGADOR {
        return Some(Jogador::Um);
    }
    if maos.ver(Jogador::Dois).ativas == PECAS_POR_JOGADOR {
        return Some(Jogador::Dois);
    }

    // Varrer o tabuleiro procurando por trio de gatoes.
    // Basta que as duas casas seguintes existam: exigir uma quarta casa
    // deixaria de detectar trios encostados na borda.
    for linha in 0..TAMANHO_TABULEIRO {
        for coluna in 0..TAMANHO_TABULEIRO {
            let Some(gato1) = tabuleiro.get(linha, coluna) else {
                continue;
            };
            if gato1.peca != Peca::Gatao {
                continue;
            }

            for direcao in DIRECOES_LINHA {
                let Some((l1, c1)) = deslocar(linha, coluna, direcao) else {
                    continue;
                };
                let Some((l2, c2)) = deslocar(l1, c1, direcao) else {
                    continue;
                };

                // Trio so vale com tres gatoes do mesmo dono (o original
                // aceitava gatinhos e comparava a cor por ponteiro).
                match (tabuleiro.get(l1, c1), tabuleiro.get(l2, c2)) {
                    (Some(gato2), Some(gato3))
                        if gato2.peca == Peca::Gatao
                            && gato3.peca == Peca::Gatao
                            && gato1.dono == gato2.dono
                            && gato2.dono == gato3.dono =>
                    {
                        return Some(gato1.dono);
                    }
                    _ => {}
                }
            }
        }
    }

    None
}

/// `true` se nenhum dos jogadores tem pecas na mao para jogar.
pub fn verificar_empate(maos: &Maos) -> bool {
    maos.ver(Jogador::Um).sem_pecas() && maos.ver(Jogador::Dois).sem_pecas()
}

/// Avalia o estado da partida: vitoria, empate ou jogo em andamento.
///
/// Vitoria tem prioridade sobre empate.
pub fn estado_do_jogo(tabuleiro: &Tabuleiro, maos: &Maos) -> Option<FimDeJogo> {
    if let Some(vencedor) = verifica_vitoria(tabuleiro, maos) {
        Some(FimDeJogo::Vitoria(vencedor))
    } else if verificar_empate(maos) {
        Some(FimDeJogo::Empate)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabuleiro_vazio_nenhuma_vitoria() {
        let tabuleiro = Tabuleiro::novo();
        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), None);
    }

    #[test]
    fn trio_horizontal_gatoes_jogador_um() {
        let mut tabuleiro = Tabuleiro::novo();
        // Colocar 3 gatoes seguidos na linha 0, colunas 0, 1, 2
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 1, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Um));
    }

    #[test]
    fn trio_horizontal_gatoes_jogador_dois() {
        let mut tabuleiro = Tabuleiro::novo();
        // Colocar 3 gatoes seguidos na linha 2, colunas 1, 2, 3
        tabuleiro.set(2, 1, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        tabuleiro.set(2, 2, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        tabuleiro.set(2, 3, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));

        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Dois));
    }

    #[test]
    fn trio_vertical_gatoes() {
        let mut tabuleiro = Tabuleiro::novo();
        // Colocar 3 gatoes seguidos na coluna 3, linhas 1, 2, 3
        tabuleiro.set(1, 3, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(2, 3, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(3, 3, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Um));
    }

    #[test]
    fn trio_diagonal_descendo_direita() {
        let mut tabuleiro = Tabuleiro::novo();
        // Direcao (1, 1): descendo para a direita
        // Posicoes: (0, 0), (1, 1), (2, 2)
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        tabuleiro.set(1, 1, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        tabuleiro.set(2, 2, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));

        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Dois));
    }

    #[test]
    fn trio_diagonal_descendo_esquerda() {
        let mut tabuleiro = Tabuleiro::novo();
        // Direcao (-1, 1): descendo para a esquerda
        // Posicoes: (3, 0), (2, 1), (1, 2)
        tabuleiro.set(3, 0, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(2, 1, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(1, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Um));
    }

    #[test]
    fn trio_gatoes_donos_misturados_nao_vence() {
        let mut tabuleiro = Tabuleiro::novo();
        // Colocar 3 gatoes seguidos, mas com donos diferentes
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 1, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        tabuleiro.set(0, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), None);
    }

    #[test]
    fn trio_gatinhos_nao_vence() {
        let mut tabuleiro = Tabuleiro::novo();
        // Colocar 3 gatinhos seguidos (nao devem contar como vitoria)
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatinho, Jogador::Um)));
        tabuleiro.set(0, 1, Some(Gato::novo(Peca::Gatinho, Jogador::Um)));
        tabuleiro.set(0, 2, Some(Gato::novo(Peca::Gatinho, Jogador::Um)));

        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), None);
    }

    #[test]
    fn trio_misto_gatao_e_gatinho_nao_vence() {
        let mut tabuleiro = Tabuleiro::novo();
        // Mistura de gatao e gatinho
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 1, Some(Gato::novo(Peca::Gatinho, Jogador::Um)));
        tabuleiro.set(0, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let maos = Maos::novas();
        assert_eq!(verifica_vitoria(&tabuleiro, &maos), None);
    }

    #[test]
    fn cama_cheia_jogador_um() {
        let tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();

        // Jogador Um com 8 pecas ativas
        maos.jogador1 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 8,
        };

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Um));
    }

    #[test]
    fn cama_cheia_jogador_dois() {
        let tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();

        // Jogador Dois com 8 pecas ativas
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 8,
        };

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Dois));
    }

    #[test]
    fn cama_cheia_prioridade_jogador_um() {
        // Quando ambos temos cama cheia, jogador Um vence (prioridade)
        let tabuleiro = Tabuleiro::novo();
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

        assert_eq!(verifica_vitoria(&tabuleiro, &maos), Some(Jogador::Um));
    }

    #[test]
    fn verificar_empate_com_maos_vazias() {
        let mut maos = Maos::novas();

        // Jogador Um: sem pecas na mao (0 gatinhos, 0 gatoes, 3 ativas)
        maos.jogador1 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 3,
        };

        // Jogador Dois: sem pecas na mao (0 gatinhos, 0 gatoes, 5 ativas)
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 5,
        };

        assert!(verificar_empate(&maos));
    }

    #[test]
    fn verificar_empate_falso_jogador_um_tem_pecas() {
        let mut maos = Maos::novas();

        // Jogador Um: ainda tem gatinhos
        maos.jogador1 = Mao {
            gatinhos: 1,
            gatoes: 7,
            ativas: 0,
        };

        // Jogador Dois: vazio
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 8,
            ativas: 0,
        };

        assert!(!verificar_empate(&maos));
    }

    #[test]
    fn estado_do_jogo_vitoria_jogador_um() {
        let mut tabuleiro = Tabuleiro::novo();
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 1, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let maos = Maos::novas();

        assert_eq!(
            estado_do_jogo(&tabuleiro, &maos),
            Some(FimDeJogo::Vitoria(Jogador::Um))
        );
    }

    #[test]
    fn estado_do_jogo_empate() {
        let tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();

        // Ambos os jogadores sem pecas na mao (0 gatinhos, 0 gatoes)
        // mas nenhum tem cama cheia (menos de 8 ativas cada)
        maos.jogador1 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 4,
        };
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 0,
            ativas: 4,
        };

        assert_eq!(estado_do_jogo(&tabuleiro, &maos), Some(FimDeJogo::Empate));
    }

    #[test]
    fn estado_do_jogo_em_andamento() {
        let tabuleiro = Tabuleiro::novo();
        let maos = Maos::novas();

        // Maos iniciais normais: cada jogador tem 8 gatinhos
        assert_eq!(estado_do_jogo(&tabuleiro, &maos), None);
    }

    #[test]
    fn estado_do_jogo_vitoria_tem_prioridade_sobre_empate() {
        // Caso improvavel: temos vitoria E maos vazias (vitoria prevalece)
        let mut tabuleiro = Tabuleiro::novo();
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 1, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        tabuleiro.set(0, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let mut maos = Maos::novas();
        maos.jogador1 = Mao {
            gatinhos: 0,
            gatoes: 5,
            ativas: 3, // 3 no tabuleiro (o trio)
        };
        maos.jogador2 = Mao {
            gatinhos: 0,
            gatoes: 8,
            ativas: 0,
        };

        assert_eq!(
            estado_do_jogo(&tabuleiro, &maos),
            Some(FimDeJogo::Vitoria(Jogador::Um))
        );
    }
}

#[cfg(test)]
mod tests_borda {
    use super::*;

    /// Regressao: um trio encostado na borda direita tem que vencer.
    /// A primeira versao exigia uma quarta casa na direcao e perdia esses casos.
    #[test]
    fn trio_de_gatoes_na_borda_direita_vence() {
        let mut tabuleiro = Tabuleiro::novo();
        for coluna in 3..6 {
            tabuleiro.set(0, coluna, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        }
        assert_eq!(
            verifica_vitoria(&tabuleiro, &Maos::novas()),
            Some(Jogador::Dois)
        );
    }

    /// Regressao: idem na borda inferior.
    #[test]
    fn trio_de_gatoes_na_borda_inferior_vence() {
        let mut tabuleiro = Tabuleiro::novo();
        for linha in 3..6 {
            tabuleiro.set(linha, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));
        }
        assert_eq!(
            verifica_vitoria(&tabuleiro, &Maos::novas()),
            Some(Jogador::Um)
        );
    }

    /// Regressao: diagonal terminando no canto inferior direito.
    #[test]
    fn trio_de_gatoes_na_diagonal_ate_o_canto_vence() {
        let mut tabuleiro = Tabuleiro::novo();
        for passo in 3..6 {
            tabuleiro.set(passo, passo, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        }
        assert_eq!(
            verifica_vitoria(&tabuleiro, &Maos::novas()),
            Some(Jogador::Dois)
        );
    }
}
