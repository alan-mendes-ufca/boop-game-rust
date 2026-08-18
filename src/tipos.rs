//! Tipos centrais do jogo.
//!
//! Este modulo e o contrato compartilhado por todos os demais. Ele substitui a
//! struct `Celula { char gato; const char *cor; }` do projeto original em C:
//! representar a peca e o dono em um unico `Option<Gato>` elimina as celulas
//! inconsistentes (peca sem cor) e o bug de comparar cores por ponteiro.

use std::fmt;

/// Lado do tabuleiro (6x6, como no original).
pub const TAMANHO_TABULEIRO: usize = 6;

/// Cada jogador possui 8 pecas no total, somando mao e tabuleiro.
pub const PECAS_POR_JOGADOR: u8 = 8;

/// Quantas pecas alinhadas formam um trio.
pub const TAMANHO_TRIO: usize = 3;

// Cores ANSI, iguais as do original.
pub const VERMELHO: &str = "\x1b[1;31m";
pub const AZUL: &str = "\x1b[1;34m";
pub const AMARELO: &str = "\x1b[1;33m";
pub const RESET: &str = "\x1b[0m";

/// As 8 direcoes do boop.
pub const DIRECOES_BOOP: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

/// As 4 direcoes usadas para varrer alinhamentos (as outras 4 sao redundantes).
pub const DIRECOES_LINHA: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (-1, 1)];

/// Tipo da peca: gatinho (`g`) ou gatao (`G`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Peca {
    Gatinho,
    Gatao,
}

impl Peca {
    /// Caractere usado na entrada e na exibicao textual.
    pub fn simbolo(self) -> char {
        match self {
            Peca::Gatinho => 'g',
            Peca::Gatao => 'G',
        }
    }

    /// Converte o caractere digitado pelo jogador. `None` se nao for `g` nem `G`.
    pub fn do_char(c: char) -> Option<Peca> {
        match c {
            'g' => Some(Peca::Gatinho),
            'G' => Some(Peca::Gatao),
            _ => None,
        }
    }
}

impl fmt::Display for Peca {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Peca::Gatinho => write!(f, "gatinho"),
            Peca::Gatao => write!(f, "gatao"),
        }
    }
}

/// Dono de uma peca. Substitui a comparacao de string de cor do original.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Jogador {
    Um,
    Dois,
}

impl Jogador {
    /// Cor ANSI associada ao jogador (vermelho para o 1, azul para o 2).
    pub fn cor(self) -> &'static str {
        match self {
            Jogador::Um => VERMELHO,
            Jogador::Dois => AZUL,
        }
    }

    /// Numero exibido na interface (1 ou 2).
    pub fn numero(self) -> u8 {
        match self {
            Jogador::Um => 1,
            Jogador::Dois => 2,
        }
    }

    pub fn oponente(self) -> Jogador {
        match self {
            Jogador::Um => Jogador::Dois,
            Jogador::Dois => Jogador::Um,
        }
    }

    /// Jogador da vez a partir do numero do turno (turno 0 = jogador 1).
    pub fn da_vez(turno: usize) -> Jogador {
        if turno % 2 == 0 {
            Jogador::Um
        } else {
            Jogador::Dois
        }
    }
}

impl fmt::Display for Jogador {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Jogador {}", self.numero())
    }
}

/// Uma peca posicionada no tabuleiro: o tipo e de quem ela e.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gato {
    pub peca: Peca,
    pub dono: Jogador,
}

impl Gato {
    pub fn novo(peca: Peca, dono: Jogador) -> Gato {
        Gato { peca, dono }
    }
}

/// Uma casa do tabuleiro: vazia ou ocupada por um gato.
pub type Celula = Option<Gato>;

/// Tabuleiro 6x6. Array fixo: sem `malloc`, sem vazamento, sem checagem de nulo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tabuleiro {
    celulas: [[Celula; TAMANHO_TABULEIRO]; TAMANHO_TABULEIRO],
}

impl Tabuleiro {
    /// Tabuleiro vazio.
    pub fn novo() -> Tabuleiro {
        Tabuleiro {
            celulas: [[None; TAMANHO_TABULEIRO]; TAMANHO_TABULEIRO],
        }
    }

    /// Conteudo de uma casa. Fora dos limites e tratado como vazio.
    pub fn get(&self, linha: usize, coluna: usize) -> Celula {
        if linha < TAMANHO_TABULEIRO && coluna < TAMANHO_TABULEIRO {
            self.celulas[linha][coluna]
        } else {
            None
        }
    }

    /// Escreve em uma casa. Ignora coordenadas fora dos limites.
    pub fn set(&mut self, linha: usize, coluna: usize, celula: Celula) {
        if linha < TAMANHO_TABULEIRO && coluna < TAMANHO_TABULEIRO {
            self.celulas[linha][coluna] = celula;
        }
    }

    /// `true` se a casa existe e esta vazia.
    pub fn vazia(&self, linha: usize, coluna: usize) -> bool {
        linha < TAMANHO_TABULEIRO && coluna < TAMANHO_TABULEIRO && self.get(linha, coluna).is_none()
    }

    /// `true` se a coordenada (possivelmente negativa) cai dentro do tabuleiro.
    pub fn dentro(linha: isize, coluna: isize) -> bool {
        linha >= 0
            && coluna >= 0
            && (linha as usize) < TAMANHO_TABULEIRO
            && (coluna as usize) < TAMANHO_TABULEIRO
    }

    /// Quantas pecas do jogador estao no tabuleiro.
    pub fn pecas_de(&self, jogador: Jogador) -> u8 {
        let mut total = 0;
        for linha in 0..TAMANHO_TABULEIRO {
            for coluna in 0..TAMANHO_TABULEIRO {
                if let Some(gato) = self.get(linha, coluna) {
                    if gato.dono == jogador {
                        total += 1;
                    }
                }
            }
        }
        total
    }
}

impl Default for Tabuleiro {
    fn default() -> Self {
        Tabuleiro::novo()
    }
}

/// Desloca uma coordenada por uma direcao. `None` se sair do tabuleiro.
pub fn deslocar(linha: usize, coluna: usize, direcao: (isize, isize)) -> Option<(usize, usize)> {
    let nova_linha = linha as isize + direcao.0;
    let nova_coluna = coluna as isize + direcao.1;
    if Tabuleiro::dentro(nova_linha, nova_coluna) {
        Some((nova_linha as usize, nova_coluna as usize))
    } else {
        None
    }
}

/// Pecas de um jogador: as que estao na mao e a contagem das que estao no tabuleiro.
///
/// Invariante: `gatinhos + gatoes + ativas == PECAS_POR_JOGADOR`. Toda mudanca
/// passa por `colocar`, `devolver` ou `graduar_trio`, entao nenhum modulo precisa
/// (nem deve) mexer nos campos diretamente para mover pecas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mao {
    pub gatinhos: u8,
    pub gatoes: u8,
    pub ativas: u8,
}

impl Mao {
    /// Mao inicial: 8 gatinhos, nada no tabuleiro.
    pub fn nova() -> Mao {
        Mao {
            gatinhos: PECAS_POR_JOGADOR,
            gatoes: 0,
            ativas: 0,
        }
    }

    /// `true` se ha essa peca disponivel na mao para jogar.
    pub fn tem(&self, peca: Peca) -> bool {
        match peca {
            Peca::Gatinho => self.gatinhos > 0,
            Peca::Gatao => self.gatoes > 0,
        }
    }

    /// Tira uma peca da mao e a contabiliza no tabuleiro.
    /// Retorna `false` (sem alterar nada) se o jogador nao tiver a peca.
    pub fn colocar(&mut self, peca: Peca) -> bool {
        if !self.tem(peca) {
            return false;
        }
        match peca {
            Peca::Gatinho => self.gatinhos -= 1,
            Peca::Gatao => self.gatoes -= 1,
        }
        self.ativas += 1;
        self.checar_invariante();
        true
    }

    /// Uma peca saiu do tabuleiro (empurrada para fora) e volta para a mao.
    pub fn devolver(&mut self, peca: Peca) {
        debug_assert!(self.ativas > 0, "devolver peca sem peca ativa no tabuleiro");
        self.ativas = self.ativas.saturating_sub(1);
        match peca {
            Peca::Gatinho => self.gatinhos += 1,
            Peca::Gatao => self.gatoes += 1,
        }
        self.checar_invariante();
    }

    /// Um trio de gatinhos se graduou: as 3 pecas saem do tabuleiro e voltam
    /// para a mao como gatoes.
    pub fn graduar_trio(&mut self) {
        debug_assert!(self.ativas >= 3, "graduar trio sem 3 pecas no tabuleiro");
        self.ativas = self.ativas.saturating_sub(3);
        self.gatoes += 3;
        self.checar_invariante();
    }

    /// Total de pecas do jogador; deve ser sempre `PECAS_POR_JOGADOR`.
    pub fn total(&self) -> u8 {
        self.gatinhos + self.gatoes + self.ativas
    }

    /// `true` se o jogador nao tem nenhuma peca na mao para jogar.
    pub fn sem_pecas(&self) -> bool {
        self.gatinhos == 0 && self.gatoes == 0
    }

    fn checar_invariante(&self) {
        debug_assert_eq!(
            self.total(),
            PECAS_POR_JOGADOR,
            "invariante da mao quebrada: {:?}",
            self
        );
    }
}

impl Default for Mao {
    fn default() -> Self {
        Mao::nova()
    }
}

/// As maos dos dois jogadores. Acesso por `Jogador` evita trocar os indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Maos {
    pub jogador1: Mao,
    pub jogador2: Mao,
}

impl Maos {
    pub fn novas() -> Maos {
        Maos {
            jogador1: Mao::nova(),
            jogador2: Mao::nova(),
        }
    }

    pub fn ver(&self, jogador: Jogador) -> &Mao {
        match jogador {
            Jogador::Um => &self.jogador1,
            Jogador::Dois => &self.jogador2,
        }
    }

    pub fn de(&mut self, jogador: Jogador) -> &mut Mao {
        match jogador {
            Jogador::Um => &mut self.jogador1,
            Jogador::Dois => &mut self.jogador2,
        }
    }
}

impl Default for Maos {
    fn default() -> Self {
        Maos::novas()
    }
}
