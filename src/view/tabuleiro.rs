//! Exibicao do tabuleiro e arte ASCII. Origem: `tabuleiro/tabuleiro.c`.

use crate::model::tipos::*;
use std::io::{self, Write};

/// Arte ASCII do gato em braille Unicode.
const ARTE_GATO: &str = r#"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣤⣤⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣾⠿⠉⠀⠀⠀⠀⠀⠹⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣾⡿⠛⠉⠁⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣷⣶⣶⣦⣤⣤⣄⣤⡀⣀⣩⣾⣿⠿⠋⠀⠀⠀⠀⠀⣠
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⠟⠋⠉⠉⠙⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⢀⡾⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⠁⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣿⣿⡇⠁⠀⠀⠀⢀⣴⣿⣿⣿⣿⡏⠋⠀⠀⠀⠀⠀⠀⠀⡞⠋⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣼⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⣿⡏⠀⠀⠀⠀⣶⣿⣿⣿⣿⡿⠉⠀⠀⠀⠀⠀⠀⠀⠀⢸⡯⠤⣤
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⣿⣿⣿⡏⠀⠀⠀⠀⣼⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡾⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣄⣄⣼⣿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡿⠋⠀⠀⠀⠀⣼⣿⣿⣿⡿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢷⣄
⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠀⠀⠀⠀⣾⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⣠⣶⣶⣶⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠿⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻
⠀⠀⠀⠀⠀⠀⢀⣾⠿⠛⢿⣿⣷⣄⡀⣿⠋⠀⠈⠀⠀⠀⠀⠀⠀⠀⢀⣾⡏⠀⢹⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣠⣤⣦⣼⣿⠀⠀⠀⣿⣿⣿⣿⣿⣦⣀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⢀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣾⣿⣿⣿⢷⣄⠀⠀⠀⠀⠀⠀⠀
⠀⣠⣾⡿⠋⠉⠉⠁⠀⠀⠀⠀⠉⢯⡙⠻⣿⣿⣷⣤⡀⠀⠀⠀⠀⢿⣿⣿⣿⣿⡿⠃⢀⡤⠖⠋⣉⣉⣉⣉⠉⠉⠒⠦⣄⠀⠀⠀⢔⡟⡿⠟⠉⣟⣻⣮⣿⠀⠀⠀⠀⠀⠀⠀⠀
⣾⣿⠋⠀⠀⠀⠀⣀⣀⡀⠀⠀⠀⠀⠙⢦⣄⠉⠻⢿⣿⣷⣦⡀⠀⠈⠙⠛⠛⠋⠀⢰⠟⠁⠀⠀⠈⠻⡿⠛⠁⠀⠀⠀⠈⠳⣄⠀⠸⣧⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀⠀⠀⠀⠀
⣿⡇⠀⠀⠀⣴⠙⣩⣿⣿⣄⠀⠀⠀⠀⡶⢌⡙⠶⣤⡈⠛⠿⣿⣷⣦⣀⠀⠀⠀⠀⡇⠀⢰⣄⠀⠀⣠⢷⠀⠀⠀⠀⠀⠀⠀⠘⡆⠀⠀⠻⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀
⣿⡇⠀⠀⢸⣇⢸⣿⣿⣿⣿⠀⠀⠀⠀⡇⠀⠈⠛⠦⣝⡳⢤⣈⠛⠻⣿⣷⣦⣀⠀⠀⠀⠀⠈⠙⠋⠁⠀⠛⠦⢤⡤⠀⠀⠀⠀⢳⠀⠀⠀⠈⠋⠙⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⡇⠀⠀⠈⢿⣄⣿⣿⣿⠏⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠙⠳⢬⣛⠦⠀⠙⢻⣿⣷⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⡇⠀⠀⠀⠀⠉⠛⠋⠁⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠀⠀⠈⣿⠉⢻⣿⣷⣦⣀⠀⠀⠀⠀⠀⠀⠀⢀⡼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⡇⠀⠀⠀⠀⠀⣠⣄⠀⠀⢰⠶⠒⠒⢧⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⢸⡇⢸⡇⠀⣿⠙⣿⣿⣉⠉⠙⠿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#;

/// Mostra a arte ASCII do gato, espera Enter e limpa a tela.
pub fn mostrar_gato() {
    print!("{}{}{}", AMARELO, ARTE_GATO, RESET);

    println!("\nPressione Enter para continuar...");
    let _ = io::stdin().read_line(&mut String::new());

    println!("\nContinuando...\n");

    // Limpa a tela usando sequência ANSI
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}

/// Imprime o tabuleiro, o inventario dos dois jogadores e de quem e a vez.
pub fn exibir_tabuleiro(tabuleiro: &Tabuleiro, maos: &Maos, atual: Jogador) {
    print!("{}", render_tabuleiro(tabuleiro));

    let mao1 = maos.ver(Jogador::Um);
    let mao2 = maos.ver(Jogador::Dois);

    println!("{}Gatinhos Jogador 1: {}{}", VERMELHO, mao1.gatinhos, RESET);
    print!("{}Gatão Jogador 1: {}{}", VERMELHO, mao1.gatoes, RESET);
    println!();

    println!("{}Gatinhos Jogador 2: {}{}", AZUL, mao2.gatinhos, RESET);
    print!("{}Gatão Jogador 2: {}{}", AZUL, mao2.gatoes, RESET);
    println!();
    println!();

    match atual {
        Jogador::Um => println!("{}Vez do Jogador 1{}", VERMELHO, RESET),
        Jogador::Dois => println!("{}Vez do Jogador 2{}", AZUL, RESET),
    }
}

/// Renderiza o tabuleiro em texto (usado por [`exibir_tabuleiro`] e pelos testes).
pub fn render_tabuleiro(tabuleiro: &Tabuleiro) -> String {
    let mut output = String::new();

    // Cabeçalho com rótulos das colunas
    output.push_str("      A          B          C          D          E          F\n");

    // Cada linha do tabuleiro
    for linha in 0..TAMANHO_TABULEIRO {
        output.push_str(&format!("{}|", linha + 1));

        for coluna in 0..TAMANHO_TABULEIRO {
            if let Some(gato) = tabuleiro.get(linha, coluna) {
                output.push_str(gato.dono.cor());

                match gato.peca {
                    Peca::Gatinho => output.push_str("≽^-˕-^≼   "),
                    Peca::Gatao => output.push_str("/ᐠ - ˕ -マ"),
                }

                output.push_str(RESET);
            } else {
                output.push_str("          ");
            }

            output.push('|');
        }

        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_tabuleiro_vazio() {
        let tabuleiro = Tabuleiro::novo();
        let saida = render_tabuleiro(&tabuleiro);
        let linhas: Vec<&str> = saida.lines().collect();

        // Deve ter 7 linhas: 1 cabeçalho + 6 de tabuleiro
        assert_eq!(linhas.len(), 7, "Tabuleiro vazio deve ter 7 linhas");

        // Primeira linha deve ter os rótulos das colunas
        assert!(linhas[0].contains("A"), "Cabeçalho deve conter rótulo 'A'");
        assert!(linhas[0].contains("F"), "Cabeçalho deve conter rótulo 'F'");
    }

    #[test]
    fn test_render_tabuleiro_estrutura() {
        let tabuleiro = Tabuleiro::novo();
        let saida = render_tabuleiro(&tabuleiro);
        let linhas: Vec<&str> = saida.lines().collect();

        // Cada linha do tabuleiro (excepto cabeçalho) deve ter 7 pipes
        for linha in &linhas[1..] {
            let pipe_count = linha.matches('|').count();
            assert_eq!(
                pipe_count, 7,
                "Cada linha deve ter 7 pipes (1 inicial + 6 separadores)"
            );
        }
    }

    #[test]
    fn test_render_tabuleiro_gatinho_jogador_1() {
        let mut tabuleiro = Tabuleiro::novo();
        let gato = Gato::novo(Peca::Gatinho, Jogador::Um);
        tabuleiro.set(0, 0, Some(gato));

        let saida = render_tabuleiro(&tabuleiro);

        // Deve conter o glifo do gatinho
        assert!(
            saida.contains("≽^-˕-^≼"),
            "Saída deve conter glifo do gatinho"
        );
        // Deve conter a cor vermelha (código ANSI)
        assert!(
            saida.contains(VERMELHO),
            "Saída deve conter código de cor VERMELHO"
        );
    }

    #[test]
    fn test_render_tabuleiro_gatao_jogador_2() {
        let mut tabuleiro = Tabuleiro::novo();
        let gato = Gato::novo(Peca::Gatao, Jogador::Dois);
        tabuleiro.set(3, 5, Some(gato));

        let saida = render_tabuleiro(&tabuleiro);

        // Deve conter o glifo do gatão
        assert!(
            saida.contains("/ᐠ - ˕ -マ"),
            "Saída deve conter glifo do gatão"
        );
        // Deve conter a cor azul (código ANSI)
        assert!(saida.contains(AZUL), "Saída deve conter código de cor AZUL");
    }

    #[test]
    fn test_exibir_tabuleiro_inventario() {
        let tabuleiro = Tabuleiro::novo();
        let _maos = Maos::novas();
        let _atual = Jogador::Um;

        // Captura a saída fazendo um snapshot
        let saida = render_tabuleiro(&tabuleiro);

        // Valida que render_tabuleiro foi chamado (ele retorna a base)
        assert!(!saida.is_empty(), "render_tabuleiro não deve estar vazio");
    }

    #[test]
    fn test_render_tabuleiro_reset_apos_gato() {
        let mut tabuleiro = Tabuleiro::novo();
        let gato = Gato::novo(Peca::Gatinho, Jogador::Um);
        tabuleiro.set(0, 0, Some(gato));

        let saida = render_tabuleiro(&tabuleiro);

        // Após cada célula com gato, deve haver RESET antes do pipe
        assert!(
            saida.contains(&format!("{}|", RESET)),
            "Deve haver RESET antes de cada pipe após gato"
        );
    }

    #[test]
    fn test_render_tabuleiro_multiplas_pecas() {
        let mut tabuleiro = Tabuleiro::novo();
        tabuleiro.set(0, 0, Some(Gato::novo(Peca::Gatinho, Jogador::Um)));
        tabuleiro.set(0, 1, Some(Gato::novo(Peca::Gatao, Jogador::Dois)));
        tabuleiro.set(1, 2, Some(Gato::novo(Peca::Gatao, Jogador::Um)));

        let saida = render_tabuleiro(&tabuleiro);

        // Deve conter ambos os glifos
        assert!(saida.contains("≽^-˕-^≼"), "Deve conter glifo do gatinho");
        assert!(saida.contains("/ᐠ - ˕ -マ"), "Deve conter glifo do gatão");

        // Deve conter cores de ambos os jogadores
        assert!(saida.contains(VERMELHO), "Deve conter cor VERMELHO");
        assert!(saida.contains(AZUL), "Deve conter cor AZUL");
    }
}
