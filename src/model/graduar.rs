//! Graduacao de trios de gatinhos. Origem: `graduar/graduar.c`.
//!
//! O original em C tinha o bug #7: ao encontrar varias linhas de 3 que
//! compartilham pecas (ex.: 4 gatinhos seguidos contem duas linhas de 3
//! sobrepostas), ele podia graduar a mesma peca mais de uma vez. Aqui cada
//! posicao so pode entrar em um unico trio: assim que um trio e aceito, suas
//! tres casas ficam marcadas como consumidas e nenhum outro trio pode
//! reutiliza-las.

use crate::model::tipos::*;

/// Gradua todos os trios de gatinhos do jogador presentes no tabuleiro.
///
/// Remove as pecas do tabuleiro e devolve 3 gatoes a mao por trio graduado.
/// Retorna quantos trios foram graduados.
pub fn graduar(tabuleiro: &mut Tabuleiro, jogador: Jogador, maos: &mut Maos) -> usize {
    let trios = trios_de_gatinhos(tabuleiro, jogador);
    for trio in &trios {
        for &(linha, coluna) in trio.iter() {
            tabuleiro.set(linha, coluna, None);
        }
        maos.de(jogador).graduar_trio();
    }
    trios.len()
}

/// Lista as posicoes dos trios de gatinhos do jogador, sem alterar o tabuleiro.
///
/// Varre o tabuleiro em ordem deterministica (linha, depois coluna, depois
/// `DIRECOES_LINHA`) marcando as casas ja usadas por um trio aceito, de forma
/// que nenhuma peca participe de dois trios ao mesmo tempo.
pub fn trios_de_gatinhos(tabuleiro: &Tabuleiro, jogador: Jogador) -> Vec<[(usize, usize); 3]> {
    let mut consumida = [[false; TAMANHO_TABULEIRO]; TAMANHO_TABULEIRO];
    let mut trios = Vec::new();

    for linha in 0..TAMANHO_TABULEIRO {
        for coluna in 0..TAMANHO_TABULEIRO {
            for &direcao in DIRECOES_LINHA.iter() {
                let Some(posicoes) = alinhamento(linha, coluna, direcao) else {
                    continue;
                };

                if posicoes.iter().any(|&(l, c)| consumida[l][c]) {
                    continue;
                }

                let todas_gatinhos_do_jogador = posicoes.iter().all(|&(l, c)| {
                    matches!(
                        tabuleiro.get(l, c),
                        Some(Gato { peca: Peca::Gatinho, dono }) if dono == jogador
                    )
                });

                if todas_gatinhos_do_jogador {
                    for &(l, c) in posicoes.iter() {
                        consumida[l][c] = true;
                    }
                    trios.push(posicoes);
                }
            }
        }
    }

    trios
}

/// Calcula as `TAMANHO_TRIO` posicoes a partir de `(linha, coluna)` seguindo
/// `direcao`. `None` se alguma delas sair do tabuleiro.
fn alinhamento(
    linha: usize,
    coluna: usize,
    direcao: (isize, isize),
) -> Option<[(usize, usize); TAMANHO_TRIO]> {
    let mut posicoes = [(linha, coluna); TAMANHO_TRIO];
    let mut atual = (linha, coluna);
    for posicao in posicoes.iter_mut().skip(1) {
        atual = deslocar(atual.0, atual.1, direcao)?;
        *posicao = atual;
    }
    Some(posicoes)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Coloca pecas no tabuleiro e mantem a mao em sincronia via `colocar`
    // (necessario porque `graduar_trio` tem debug_assert de `ativas >= 3`).
    // Quando a mao nao tem a peca (ex.: gatao, que comeca zerado), o
    // `colocar` falha silenciosamente e so o tabuleiro e afetado; os testes
    // que usam esse caso nunca chegam a graduar, entao a mao ficar
    // dessincronizada nao importa.
    fn coloca(
        tabuleiro: &mut Tabuleiro,
        maos: &mut Maos,
        posicoes: &[(usize, usize)],
        peca: Peca,
        dono: Jogador,
    ) {
        for &(l, c) in posicoes {
            maos.de(dono).colocar(peca);
            tabuleiro.set(l, c, Some(Gato::novo(peca, dono)));
        }
    }

    fn vazia(tabuleiro: &Tabuleiro, posicoes: &[(usize, usize)]) -> bool {
        posicoes.iter().all(|&(l, c)| tabuleiro.vazia(l, c))
    }

    #[test]
    fn trio_horizontal_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1), (0, 2)],
            Peca::Gatinho,
            Jogador::Um,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 1);
        assert!(vazia(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
        assert_eq!(maos.ver(Jogador::Um).ativas, 0);
    }

    #[test]
    fn trio_vertical_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(1, 2), (2, 2), (3, 2)],
            Peca::Gatinho,
            Jogador::Dois,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Dois, &mut maos);

        assert_eq!(quantos, 1);
        assert!(vazia(&tabuleiro, &[(1, 2), (2, 2), (3, 2)]));
        assert_eq!(maos.ver(Jogador::Dois).gatoes, 3);
    }

    #[test]
    fn trio_diagonal_principal_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (1, 1), (2, 2)],
            Peca::Gatinho,
            Jogador::Um,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 1);
        assert!(vazia(&tabuleiro, &[(0, 0), (1, 1), (2, 2)]));
    }

    #[test]
    fn trio_diagonal_secundaria_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        // direcao (-1, 1): sobe uma linha, avanca uma coluna.
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(2, 0), (1, 1), (0, 2)],
            Peca::Gatinho,
            Jogador::Dois,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Dois, &mut maos);

        assert_eq!(quantos, 1);
        assert!(vazia(&tabuleiro, &[(2, 0), (1, 1), (0, 2)]));
    }

    #[test]
    fn trio_do_adversario_nao_gradua_para_outro_jogador() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1), (0, 2)],
            Peca::Gatinho,
            Jogador::Dois,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 0);
        assert!(!vazia(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
        assert_eq!(maos.ver(Jogador::Um).ativas, 0);
        assert_eq!(maos.ver(Jogador::Dois).ativas, 3);
    }

    #[test]
    fn linha_com_donos_diferentes_nao_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1)],
            Peca::Gatinho,
            Jogador::Um,
        );
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 2)],
            Peca::Gatinho,
            Jogador::Dois,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 0);
        assert!(!vazia(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
    }

    #[test]
    fn gatinho_gatinho_gatao_nao_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1)],
            Peca::Gatinho,
            Jogador::Um,
        );
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 2)],
            Peca::Gatao,
            Jogador::Um,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 0);
        assert!(!vazia(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
    }

    #[test]
    fn trio_de_gatoes_nao_gradua() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1), (0, 2)],
            Peca::Gatao,
            Jogador::Um,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 0);
        assert!(!vazia(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
    }

    #[test]
    fn quatro_em_linha_gradua_apenas_um_trio_e_sobra_uma_peca() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Peca::Gatinho,
            Jogador::Um,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 1);
        // As tres primeiras encontradas na varredura graduam...
        assert!(vazia(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
        // ...e a quarta fica no tabuleiro, sem formar um segundo trio.
        assert!(!tabuleiro.vazia(0, 3));
        assert_eq!(maos.ver(Jogador::Um).gatoes, 3);
        assert_eq!(maos.ver(Jogador::Um).ativas, 1);
    }

    #[test]
    fn cinco_em_linha_gradua_um_trio_e_sobram_duas_pecas() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
            Peca::Gatinho,
            Jogador::Um,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 1);
        assert!(vazia(&tabuleiro, &[(0, 0), (0, 1), (0, 2)]));
        assert!(!tabuleiro.vazia(0, 3));
        assert!(!tabuleiro.vazia(0, 4));
        assert_eq!(maos.ver(Jogador::Um).ativas, 2);
    }

    #[test]
    fn dois_trios_disjuntos_graduam_duas_vezes() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1), (0, 2)],
            Peca::Gatinho,
            Jogador::Um,
        );
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(3, 0), (3, 1), (3, 2)],
            Peca::Gatinho,
            Jogador::Um,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 2);
        assert!(vazia(
            &tabuleiro,
            &[(0, 0), (0, 1), (0, 2), (3, 0), (3, 1), (3, 2)]
        ));
        assert_eq!(maos.ver(Jogador::Um).gatoes, 6);
        assert_eq!(maos.ver(Jogador::Um).ativas, 0);
    }

    #[test]
    fn tabuleiro_sem_trios_nao_altera_nada() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (5, 5)],
            Peca::Gatinho,
            Jogador::Um,
        );

        let quantos = graduar(&mut tabuleiro, Jogador::Um, &mut maos);

        assert_eq!(quantos, 0);
        assert_eq!(maos.ver(Jogador::Um).gatinhos, 6);
        assert_eq!(maos.ver(Jogador::Um).gatoes, 0);
        assert_eq!(maos.ver(Jogador::Um).ativas, 2);
        assert!(!tabuleiro.vazia(0, 0));
        assert!(!tabuleiro.vazia(5, 5));
    }

    #[test]
    fn trios_de_gatinhos_nao_altera_tabuleiro() {
        let mut tabuleiro = Tabuleiro::novo();
        let mut maos = Maos::novas();
        coloca(
            &mut tabuleiro,
            &mut maos,
            &[(0, 0), (0, 1), (0, 2)],
            Peca::Gatinho,
            Jogador::Um,
        );
        let antes = tabuleiro;

        let trios = trios_de_gatinhos(&tabuleiro, Jogador::Um);

        assert_eq!(trios.len(), 1);
        assert_eq!(tabuleiro, antes);
    }
}
