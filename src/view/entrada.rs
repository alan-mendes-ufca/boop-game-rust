//! Camada de view para a entrada do jogador: prompt, leitura e traducao do
//! texto digitado em uma [`Jogada`]. Origem: `funcoes.c`.

use crate::model::jogada::Jogada;
use crate::model::tipos::*;
use std::fmt;
use std::io::{BufRead, Write};

/// Motivo pelo qual a entrada digitada nao pode ser interpretada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntradaInvalida {
    Formato,
    TipoPeca,
    Linha,
    Coluna,
}

impl fmt::Display for EntradaInvalida {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntradaInvalida::Formato => {
                write!(f, "Formato inválido! Use: tipo linha coluna (ex: g 1 A).")
            }
            EntradaInvalida::TipoPeca => write!(
                f,
                "Tipo de peça inválido! Use 'g' para gatinho ou 'G' para gatão."
            ),
            EntradaInvalida::Linha => write!(f, "Linha inválida! Use um número de 1 a 6."),
            EntradaInvalida::Coluna => write!(f, "Coluna inválida! Use uma letra de A a F."),
        }
    }
}

/// Interpreta uma linha digitada no formato `g 1 A`.
pub fn parse_jogada(entrada: &str) -> Result<Jogada, EntradaInvalida> {
    // Separa por qualquer sequencia de espacos em branco (cobre espacos
    // extras e o \r\n do fim de linha no Windows).
    let campos: Vec<&str> = entrada.split_whitespace().collect();
    if campos.len() != 3 {
        return Err(EntradaInvalida::Formato);
    }

    // Campo 1: tipo da peca, exatamente um caractere.
    let mut chars_tipo = campos[0].chars();
    let Some(char_tipo) = chars_tipo.next() else {
        return Err(EntradaInvalida::TipoPeca);
    };
    if chars_tipo.next().is_some() {
        return Err(EntradaInvalida::TipoPeca);
    }
    let peca = Peca::do_char(char_tipo).ok_or(EntradaInvalida::TipoPeca)?;

    // Campo 2: linha de 1 a 6, convertida para indice base 0.
    let numero_linha: i32 = campos[1].parse().map_err(|_| EntradaInvalida::Linha)?;
    if numero_linha < 1 || numero_linha > TAMANHO_TABULEIRO as i32 {
        return Err(EntradaInvalida::Linha);
    }
    let linha = (numero_linha - 1) as usize;

    // Campo 3: coluna de A a F (maiuscula ou minuscula), convertida para
    // indice base 0.
    let mut chars_coluna = campos[2].chars();
    let Some(char_coluna) = chars_coluna.next() else {
        return Err(EntradaInvalida::Coluna);
    };
    if chars_coluna.next().is_some() {
        return Err(EntradaInvalida::Coluna);
    }
    let char_coluna_maiusculo = char_coluna.to_ascii_uppercase();
    if !char_coluna_maiusculo.is_ascii_uppercase() {
        return Err(EntradaInvalida::Coluna);
    }
    let indice_coluna = (char_coluna_maiusculo as u8) - b'A';
    if indice_coluna as usize >= TAMANHO_TABULEIRO {
        return Err(EntradaInvalida::Coluna);
    }
    let coluna = indice_coluna as usize;

    Ok(Jogada {
        peca,
        linha,
        coluna,
    })
}

/// Pede uma jogada ao jogador ate receber uma entrada bem formada.
/// `Ok(None)` significa fim da entrada (EOF).
pub fn ler_jogada(
    entrada: &mut impl BufRead,
    saida: &mut impl Write,
) -> std::io::Result<Option<Jogada>> {
    loop {
        write!(
            saida,
            "\nDigite o tipo de peça ('g' para gatinho, 'G' para gatão) e a coordenada (ex: g 1 A):\n"
        )?;
        saida.flush()?;

        let mut linha = String::new();
        let bytes_lidos = entrada.read_line(&mut linha)?;
        if bytes_lidos == 0 {
            // Fim da entrada: nao ha mais nada para ler.
            return Ok(None);
        }

        match parse_jogada(&linha) {
            Ok(jogada) => return Ok(Some(jogada)),
            Err(erro) => {
                writeln!(saida, "{}", erro)?;
                saida.flush()?;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_entrada_valida_basica() {
        let jogada = parse_jogada("g 1 A").unwrap();
        assert_eq!(
            jogada,
            Jogada {
                peca: Peca::Gatinho,
                linha: 0,
                coluna: 0,
            }
        );
    }

    #[test]
    fn parse_entrada_valida_gatao_extremos() {
        let jogada = parse_jogada("G 6 F").unwrap();
        assert_eq!(
            jogada,
            Jogada {
                peca: Peca::Gatao,
                linha: 5,
                coluna: 5,
            }
        );
    }

    #[test]
    fn parse_entrada_valida_coluna_minuscula() {
        let jogada = parse_jogada("g 3 c").unwrap();
        assert_eq!(
            jogada,
            Jogada {
                peca: Peca::Gatinho,
                linha: 2,
                coluna: 2,
            }
        );
    }

    #[test]
    fn parse_entrada_valida_espacos_extras() {
        let jogada = parse_jogada("  g   2    B  ").unwrap();
        assert_eq!(
            jogada,
            Jogada {
                peca: Peca::Gatinho,
                linha: 1,
                coluna: 1,
            }
        );
    }

    #[test]
    fn parse_entrada_valida_com_quebra_de_linha() {
        assert!(parse_jogada("g 1 A\n").is_ok());
        assert!(parse_jogada("g 1 A\r\n").is_ok());
    }

    #[test]
    fn parse_entrada_vazia_e_erro_de_formato() {
        assert_eq!(parse_jogada(""), Err(EntradaInvalida::Formato));
    }

    #[test]
    fn parse_entrada_com_dois_campos_e_erro_de_formato() {
        assert_eq!(parse_jogada("g 1"), Err(EntradaInvalida::Formato));
    }

    #[test]
    fn parse_entrada_com_quatro_campos_e_erro_de_formato() {
        assert_eq!(parse_jogada("g 1 A extra"), Err(EntradaInvalida::Formato));
    }

    #[test]
    fn parse_tipo_de_peca_invalido() {
        assert_eq!(parse_jogada("x 1 A"), Err(EntradaInvalida::TipoPeca));
    }

    #[test]
    fn parse_linha_zero_e_invalida() {
        assert_eq!(parse_jogada("g 0 A"), Err(EntradaInvalida::Linha));
    }

    #[test]
    fn parse_linha_sete_e_invalida() {
        assert_eq!(parse_jogada("g 7 A"), Err(EntradaInvalida::Linha));
    }

    #[test]
    fn parse_linha_nao_numerica_e_invalida() {
        assert_eq!(parse_jogada("g abc A"), Err(EntradaInvalida::Linha));
    }

    #[test]
    fn parse_coluna_fora_do_intervalo_e_invalida() {
        assert_eq!(parse_jogada("g 1 Z"), Err(EntradaInvalida::Coluna));
    }

    #[test]
    fn parse_coluna_com_mais_de_um_caractere_e_invalida() {
        assert_eq!(parse_jogada("g 1 AB"), Err(EntradaInvalida::Coluna));
    }

    #[test]
    fn ler_jogada_le_entrada_valida_de_primeira() {
        let mut entrada = std::io::Cursor::new(b"g 1 A\n".to_vec());
        let mut saida = Vec::new();
        let resultado = ler_jogada(&mut entrada, &mut saida).unwrap();
        assert_eq!(
            resultado,
            Some(Jogada {
                peca: Peca::Gatinho,
                linha: 0,
                coluna: 0,
            })
        );
    }

    #[test]
    fn ler_jogada_repete_ate_entrada_valida() {
        let mut entrada = std::io::Cursor::new(b"lixo\ng 2 B\n".to_vec());
        let mut saida = Vec::new();
        let resultado = ler_jogada(&mut entrada, &mut saida).unwrap();
        assert_eq!(
            resultado,
            Some(Jogada {
                peca: Peca::Gatinho,
                linha: 1,
                coluna: 1,
            })
        );
        let texto_saida = String::from_utf8(saida).unwrap();
        assert!(texto_saida.contains("Formato inválido"));
    }

    #[test]
    fn ler_jogada_retorna_none_no_fim_da_entrada() {
        let mut entrada = std::io::Cursor::new(Vec::new());
        let mut saida = Vec::new();
        let resultado = ler_jogada(&mut entrada, &mut saida).unwrap();
        assert_eq!(resultado, None);
    }
}
