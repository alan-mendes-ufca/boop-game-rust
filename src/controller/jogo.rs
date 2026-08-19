//! Laco principal da partida: le a jogada pela view, valida contra o model e
//! manda a view redesenhar. Origem: `main.c`.

use crate::controller::turno;
use crate::model::jogada;
use crate::model::tipos::{Jogador, Maos, Tabuleiro};
use crate::model::vitoria::{self, FimDeJogo};
use crate::view::{entrada, tabuleiro};
use std::io;

/// Roda uma partida do inicio ao fim, lendo da entrada padrao.
pub fn executar() {
    let mut tab = Tabuleiro::novo();
    let mut maos = Maos::novas();

    tabuleiro::mostrar_gato();

    let stdin = io::stdin();
    let mut leitura_entrada = stdin.lock();
    let mut saida = io::stdout();

    let mut numero_turno: usize = 0;
    loop {
        let atual = Jogador::da_vez(numero_turno);

        tabuleiro::exibir_tabuleiro(&tab, &maos, atual);

        // Fim de jogo alcancado sem jogada nova (ex.: os dois jogadores
        // ficaram sem pecas). Sem esta checagem o laco de turnos pulados
        // nunca terminaria.
        if let Some(fim) = vitoria::estado_do_jogo(&tab, &maos) {
            anunciar_fim(fim);
            break;
        }

        // Sem pecas na mao: nao ha jogada possivel, pula o turno.
        if maos.ver(atual).sem_pecas() {
            println!("\n{} não tem peças na mão. Turno pulado.", atual);
            numero_turno += 1;
            continue;
        }

        // Pede jogadas ate receber uma valida (ou a entrada acabar).
        let jogada_escolhida = loop {
            let leitura = match entrada::ler_jogada(&mut leitura_entrada, &mut saida) {
                Ok(leitura) => leitura,
                Err(erro) => {
                    eprintln!("Erro de entrada/saída: {}", erro);
                    return;
                }
            };

            let Some(jogada_candidata) = leitura else {
                println!("\nEntrada encerrada. Até a próxima!");
                return;
            };

            let mao_atual = maos.ver(atual);
            match jogada::verificar_jogada(
                &tab,
                jogada_candidata.linha,
                jogada_candidata.coluna,
                jogada_candidata.peca,
                mao_atual,
            ) {
                Ok(()) => break jogada_candidata,
                Err(motivo) => println!("{}", motivo),
            }
        };

        turno::fluxo_jogo(&mut tab, &jogada_escolhida, atual, &mut maos);

        if let Some(fim) = vitoria::estado_do_jogo(&tab, &maos) {
            tabuleiro::exibir_tabuleiro(&tab, &maos, atual);
            anunciar_fim(fim);
            break;
        }

        numero_turno += 1;
    }
}

/// Mensagem final da partida.
fn anunciar_fim(fim: FimDeJogo) {
    match fim {
        FimDeJogo::Vitoria(vencedor) => println!("\n{} venceu!", vencedor),
        FimDeJogo::Empate => println!("\nEmpate! Nenhum jogador tem mais peças para jogar."),
    }
}
