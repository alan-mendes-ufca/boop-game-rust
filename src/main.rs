//! Loop principal do jogo. Origem: `main.c`.

use boop::tipos::{Jogador, Maos, Tabuleiro};
use boop::vitoria::FimDeJogo;
use boop::{funcoes, jogada, tabuleiro, vitoria};
use std::io;

fn main() {
    let mut tab = Tabuleiro::novo();
    let mut maos = Maos::novas();

    tabuleiro::mostrar_gato();

    let stdin = io::stdin();
    let mut entrada = stdin.lock();
    let mut saida = io::stdout();

    let mut turno: usize = 0;
    loop {
        let atual = Jogador::da_vez(turno);

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
            turno += 1;
            continue;
        }

        // Pede jogadas ate receber uma valida (ou a entrada acabar).
        let jogada_escolhida = loop {
            let leitura = match funcoes::ler_jogada(&mut entrada, &mut saida) {
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

        funcoes::fluxo_jogo(&mut tab, &jogada_escolhida, atual, &mut maos);

        if let Some(fim) = vitoria::estado_do_jogo(&tab, &maos) {
            tabuleiro::exibir_tabuleiro(&tab, &maos, atual);
            anunciar_fim(fim);
            break;
        }

        turno += 1;
    }
}

/// Mensagem final da partida.
fn anunciar_fim(fim: FimDeJogo) {
    match fim {
        FimDeJogo::Vitoria(vencedor) => println!("\n{} venceu!", vencedor),
        FimDeJogo::Empate => println!("\nEmpate! Nenhum jogador tem mais peças para jogar."),
    }
}
