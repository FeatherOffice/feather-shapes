//! Diagramas (a galeria de SmartArt): modelos vetoriais por categoria. Não são SmartArt
//! OOXML: os nós são formas nativas com texto editável e os conectores são linhas, devolvidos
//! como [`ItemDiagrama`] neutros para cada aplicativo transformar no seu elemento.

use crate::tipo::TipoForma;

/// As categorias de diagrama da galeria.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Categoria {
    Listas, Processos, Ciclos, Hierarquias, Organogramas, Relacoes,
    Matrizes, Piramides, Imagens, Venn, Radiais, Engrenagens, Fluxo, Comparacao,
}

impl Categoria {
    pub const TODAS: [Self; 14] = [
        Self::Listas, Self::Processos, Self::Ciclos, Self::Hierarquias,
        Self::Organogramas, Self::Relacoes, Self::Matrizes, Self::Piramides,
        Self::Imagens, Self::Venn, Self::Radiais, Self::Engrenagens,
        Self::Fluxo, Self::Comparacao,
    ];
    /// Nome para mostrar.
    pub fn rotulo(self) -> &'static str {
        match self {
            Self::Listas => "Listas", Self::Processos => "Processos",
            Self::Ciclos => "Ciclos", Self::Hierarquias => "Hierarquias",
            Self::Organogramas => "Organogramas", Self::Relacoes => "Relações",
            Self::Matrizes => "Matrizes", Self::Piramides => "Pirâmides",
            Self::Imagens => "Com imagens", Self::Venn => "Venn",
            Self::Radiais => "Radiais", Self::Engrenagens => "Engrenagens",
            Self::Fluxo => "Fluxo", Self::Comparacao => "Comparação",
        }
    }
    /// O arranjo do editor que melhor reorganiza os itens desta categoria.
    pub fn arranjo(self) -> crate::arranjo::Arranjo {
        use crate::arranjo::Arranjo;
        match self {
            Self::Processos | Self::Fluxo => Arranjo::Processo,
            Self::Ciclos | Self::Radiais | Self::Engrenagens => Arranjo::Ciclo,
            Self::Hierarquias | Self::Organogramas => Arranjo::Organograma,
            _ => Arranjo::Lista,
        }
    }
    /// Identificador estável em texto.
    pub fn identificador(self) -> &'static str {
        match self {
            Self::Listas => "listas", Self::Processos => "processos",
            Self::Ciclos => "ciclos", Self::Hierarquias => "hierarquias",
            Self::Organogramas => "organogramas", Self::Relacoes => "relacoes",
            Self::Matrizes => "matrizes", Self::Piramides => "piramides",
            Self::Imagens => "imagens", Self::Venn => "venn",
            Self::Radiais => "radiais", Self::Engrenagens => "engrenagens",
            Self::Fluxo => "fluxo", Self::Comparacao => "comparacao",
        }
    }
    /// A categoria pelo identificador.
    pub fn pelo_identificador(id: &str) -> Option<Self> {
        Self::TODAS.into_iter().find(|c| c.identificador() == id)
    }
}

/// Paleta dos nós, na ordem dos itens.
pub const CORES: [&str; 5] = ["#3478D4", "#179C87", "#F39A37", "#755BC5", "#D65E76"];
/// Contorno dos nós sem preenchimento (títulos).
pub const BORDA: &str = "#D3DFEC";
/// Cor das linhas de ligação.
pub const LIGACAO: &str = "#8296AC";
/// Cor do texto sobre fundo claro.
pub const TEXTO: &str = "#263A53";

/// O estilo do texto de um nó (centralizado na horizontal e na vertical).
#[derive(Clone, Debug, PartialEq)]
pub struct RotuloItem {
    pub cor: String,
    pub tamanho: f32,
    pub negrito: bool,
}

/// Uma peça do diagrama, posicionada: forma com texto, ou linha de ligação (sem rótulo,
/// do ponto (`x`, `y`) até (`x + largura`, `y + altura`)).
#[derive(Clone, Debug, PartialEq)]
pub struct ItemDiagrama {
    pub tipo: TipoForma,
    pub x: f32,
    pub y: f32,
    pub largura: f32,
    pub altura: f32,
    pub texto: String,
    pub preenchimento: String,
    pub contorno: String,
    pub espessura: f32,
    /// `None` nas linhas: o aplicativo usa o estilo de texto padrão dele.
    pub rotulo: Option<RotuloItem>,
}

struct Montador {
    itens: Vec<ItemDiagrama>,
}

impl Montador {
    fn novo() -> Self { Self { itens: Vec::new() } }
    #[allow(clippy::too_many_arguments)]
    fn forma(&mut self, tipo: TipoForma, nome: &str, x: f32, y: f32, w: f32, h: f32, cor: &str) {
        self.itens.push(ItemDiagrama {
            tipo, x, y, largura: w, altura: h, texto: nome.into(),
            preenchimento: cor.into(), contorno: if cor.is_empty() { BORDA } else { cor }.into(),
            espessura: 1.4,
            rotulo: Some(RotuloItem {
                cor: if cor.is_empty() { TEXTO } else { "#FFFFFF" }.into(),
                tamanho: if w < 105.0 || h < 45.0 { 15.0 } else { 20.0 },
                negrito: !nome.is_empty(),
            }),
        });
    }
    fn linha(&mut self, ax: f32, ay: f32, bx: f32, by: f32) {
        self.itens.push(ItemDiagrama {
            tipo: TipoForma::Linha, x: ax, y: ay, largura: bx - ax, altura: by - ay,
            texto: String::new(), preenchimento: String::new(),
            contorno: LIGACAO.into(), espessura: 3.0, rotulo: None,
        });
    }
    fn titulo(&mut self, nome: &str, x: f32, y: f32, w: f32) {
        self.forma(TipoForma::CaixaTexto, nome, x, y, w, 33.0, "");
    }
    fn caixa(&mut self, nome: &str, x: f32, y: f32, w: f32, h: f32, cor: usize) {
        self.forma(TipoForma::RetanguloArredondado, nome, x, y, w, h, CORES[cor % CORES.len()]);
    }
    fn circulo(&mut self, nome: &str, x: f32, y: f32, d: f32, cor: usize) {
        self.forma(TipoForma::Elipse, nome, x, y, d, d, CORES[cor % CORES.len()]);
    }
}

/// Largura do quadro de referência dos modelos.
pub const LARGURA_BASE: f32 = 720.0;
/// Altura do quadro de referência dos modelos.
pub const ALTURA_BASE: f32 = 360.0;

/// O modelo da categoria no quadro de referência de 720 × 360, na ordem de desenho
/// (conectores ao fundo, nós à frente).
pub fn modelo_base(categoria: Categoria) -> Vec<ItemDiagrama> {
    let mut m = Montador::novo();
    match categoria {
        Categoria::Listas => {
            for (i, nome) in ["Ideia", "Planejamento", "Execução", "Resultado"].into_iter().enumerate() {
                let cy = 21.0 + i as f32 * 82.0;
                m.circulo(&(i + 1).to_string(), 75.0, cy + 8.0, 54.0, i);
                m.caixa(nome, 157.0, cy, 485.0, 66.0, i);
            }
        }
        Categoria::Processos => {
            for (i, nome) in ["Início", "Preparar", "Executar", "Concluir"].into_iter().enumerate() {
                m.forma(TipoForma::Chevron, nome, 13.0 + i as f32 * 176.0, 122.0, 168.0, 115.0, CORES[i]);
            }
        }
        Categoria::Ciclos => {
            for (ax, ay, bx, by) in [(360.0, 73.0, 535.0, 163.0), (585.0, 203.0, 365.0, 296.0),
                (325.0, 296.0, 122.0, 203.0), (122.0, 156.0, 319.0, 73.0)] {
                m.linha(ax, ay, bx, by);
            }
            for (i, (nome, x, y)) in [
                ("Planejar", 300.0, 8.0), ("Fazer", 515.0, 138.0),
                ("Checar", 300.0, 230.0), ("Agir", 85.0, 138.0)
            ].into_iter().enumerate() { m.circulo(nome, x, y, 122.0, i); }
        }
        Categoria::Hierarquias | Categoria::Organogramas => {
            let org = categoria == Categoria::Organogramas;
            m.linha(360.0, 92.0, 360.0, 140.0);
            m.linha(150.0, 140.0, 570.0, 140.0);
            for x in [150.0, 360.0, 570.0] { m.linha(x, 140.0, x, 195.0); }
            if !org {
                for (ax, bx) in [(150.0, 95.0), (150.0, 205.0), (570.0, 505.0), (570.0, 615.0)] {
                    m.linha(ax, 259.0, ax, 282.0); m.linha(ax, 282.0, bx, 282.0);
                    m.linha(bx, 282.0, bx, 302.0);
                }
            }
            m.caixa(if org { "Diretoria" } else { "Nível 1" }, 270.0, 24.0, 180.0, 68.0, 0);
            for (i, (nome, x)) in [
                (if org { "Produto" } else { "Ramo A" }, 56.0),
                (if org { "Engenharia" } else { "Ramo B" }, 266.0),
                (if org { "Operações" } else { "Ramo C" }, 476.0)
            ].into_iter().enumerate() {
                m.caixa(nome, x, 195.0, 188.0, 64.0, i + 1);
            }
            if !org {
                for (i, x) in [39.0, 159.0, 449.0, 569.0].into_iter().enumerate() {
                    m.caixa(&format!("Item {}", i+1), x, 302.0, 105.0, 50.0, i);
                }
            }
        }
        Categoria::Relacoes => {
            for (x, y) in [(130.0, 75.0), (580.0, 75.0), (130.0, 290.0), (580.0, 290.0)] {
                m.linha(360.0, 180.0, x, y);
            }
            for (i, (nome, x, y)) in [
                ("Equipe", 40.0, 20.0), ("Cliente", 500.0, 20.0),
                ("Fornecedor", 40.0, 245.0), ("Parceiro", 500.0, 245.0)
            ].into_iter().enumerate() { m.caixa(nome, x, y, 180.0, 90.0, i+1); }
            m.circulo("Projeto", 277.0, 97.0, 166.0, 0);
        }
        Categoria::Matrizes => {
            for (i, (nome, x, y)) in [
                ("Importante", 120.0, 35.0), ("Prioritário", 375.0, 35.0),
                ("Rotineiro", 120.0, 185.0), ("Planejar", 375.0, 185.0)
            ].into_iter().enumerate() {
                m.caixa(nome, x, y, 225.0, 133.0, i);
            }
            m.titulo("Matriz 2 × 2", 205.0, 325.0, 310.0);
        }
        Categoria::Piramides => {
            // Três patamares largos, com cima menor que a base.
            m.forma(TipoForma::Triangulo, "Visão", 278.0, 5.0, 164.0, 118.0, CORES[0]);
            m.forma(TipoForma::Trapezio, "Estratégia", 210.0, 121.0, 300.0, 108.0, CORES[1]);
            m.forma(TipoForma::Trapezio, "Operações", 113.0, 236.0, 494.0, 115.0, CORES[2]);
        }
        Categoria::Imagens => {
            // Área de fotografia substituível por imagem nativa posteriormente
            // (inserção de fotos nos nós de SmartArt é uma operação da seção 9.2).
            for (i, (nome, x)) in [("Foto 1", 32.0), ("Foto 2", 271.0), ("Foto 3", 510.0)].into_iter().enumerate() {
                m.forma(TipoForma::RetanguloArredondado, "", x, 32.0, 190.0, 215.0, "#E8EFF8");
                m.forma(TipoForma::Moldura, "", x+20.0, 50.0, 150.0, 156.0, CORES[i]);
                m.forma(TipoForma::Sol, "", x+68.0, 65.0, 40.0, 40.0, "#F8C966");
                m.forma(TipoForma::Triangulo, "", x+38.0, 137.0, 115.0, 56.0, "#8ED4C3");
                m.caixa(nome, x, 260.0, 190.0, 59.0, i);
            }
        }
        Categoria::Venn => {
            // Camadas claras permitem visualizar a interseção dos círculos.
            m.forma(TipoForma::Elipse, "Pessoas", 120.0, 45.0, 300.0, 270.0, "#6CACE4A0");
            m.forma(TipoForma::Elipse, "Processos", 300.0, 45.0, 300.0, 270.0, "#59BEA7A0");
            m.titulo("Interseção", 277.0, 172.0, 166.0);
        }
        Categoria::Radiais => {
            for (x, y) in [(360.0, 46.0), (570.0, 153.0), (475.0, 306.0),
                (235.0, 306.0), (150.0, 153.0)] {
                m.linha(360.0, 180.0, x, y);
            }
            for (i, (nome, x, y)) in [
                ("1", 310.0, 5.0), ("2", 543.0, 120.0),
                ("3", 433.0, 275.0), ("4", 187.0, 275.0), ("5", 117.0, 120.0)
            ].into_iter().enumerate() { m.circulo(nome, x, y, 80.0, i); }
            m.circulo("Centro", 293.0, 113.0, 134.0, 0);
        }
        Categoria::Engrenagens => {
            m.linha(310.0, 163.0, 420.0, 190.0);
            for (i, (x, y, d)) in [(120.0, 75.0, 235.0), (370.0, 110.0, 200.0)].into_iter().enumerate() {
                // Engrenagem vetorial simplificada: estrela dentada mais miolo.
                m.forma(TipoForma::Estrela12, "", x, y, d, d, CORES[i]);
                m.circulo(if i == 0 { "A" } else { "B" }, x + d * 0.31, y + d * 0.31, d * 0.38, i+2);
            }
        }
        Categoria::Fluxo => {
            m.linha(360.0, 80.0, 360.0, 116.0);
            m.linha(360.0, 214.0, 360.0, 257.0);
            m.linha(360.0, 257.0, 550.0, 257.0);
            m.linha(360.0, 257.0, 170.0, 257.0);
            m.forma(TipoForma::Elipse, "Início", 278.0, 16.0, 164.0, 64.0, CORES[0]);
            m.forma(TipoForma::Losango, "Decisão", 278.0, 116.0, 164.0, 100.0, CORES[2]);
            m.caixa("Sim", 70.0, 257.0, 200.0, 68.0, 1);
            m.caixa("Não", 450.0, 257.0, 200.0, 68.0, 3);
        }
        Categoria::Comparacao => {
            m.titulo("Opção A", 92.0, 21.0, 200.0);
            m.titulo("Opção B", 428.0, 21.0, 200.0);
            m.caixa("Vantagens", 50.0, 72.0, 290.0, 108.0, 0);
            m.caixa("Vantagens", 380.0, 72.0, 290.0, 108.0, 1);
            m.caixa("Limitações", 50.0, 193.0, 290.0, 108.0, 2);
            m.caixa("Limitações", 380.0, 193.0, 290.0, 108.0, 3);
            m.linha(360.0, 40.0, 360.0, 314.0);
        }
    }
    m.itens
}

/// O modelo da categoria escalado para a caixa (`x`, `y`, `largura`, `altura`). Espessuras e
/// tamanhos de texto acompanham o menor fator de escala.
pub fn modelo(categoria: Categoria, x: f32, y: f32, largura: f32, altura: f32) -> Vec<ItemDiagrama> {
    let fx = largura / LARGURA_BASE;
    let fy = altura / ALTURA_BASE;
    let mut itens = modelo_base(categoria);
    for e in &mut itens {
        e.x = x + e.x * fx;
        e.y = y + e.y * fy;
        e.largura *= fx;
        e.altura *= fy;
        e.espessura *= fx.min(fy);
        if let Some(r) = &mut e.rotulo {
            r.tamanho *= fx.min(fy);
        }
    }
    itens
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn todas_as_categorias_geram_diagramas_vetoriais_na_area() {
        for cat in Categoria::TODAS {
            let itens = modelo(cat, 120.0, 90.0, 720.0, 360.0);
            assert!(itens.len() >= 3, "{:?} sem itens suficientes", cat);
            assert!(itens.iter().any(|e| !e.texto.is_empty()), "{cat:?} sem textos editáveis");
            assert!(itens.iter().all(|e| e.x >= 119.5 && e.y >= 89.5
                && e.x + e.largura >= 119.0 && e.x + e.largura <= 841.0
                && e.y + e.altura >= 89.0 && e.y + e.altura <= 451.0), "{cat:?} fora da área");
            assert!(itens.iter().all(|e| (e.tipo == TipoForma::Linha) == e.rotulo.is_none()));
            assert_eq!(Categoria::pelo_identificador(cat.identificador()), Some(cat));
        }
    }

    #[test]
    fn escala_acompanha_a_caixa() {
        let a = modelo(Categoria::Listas, 0.0, 0.0, 360.0, 180.0);
        let b = modelo_base(Categoria::Listas);
        assert_eq!(a.len(), b.len());
        assert!((a[1].largura - b[1].largura / 2.0).abs() < 1e-4);
        assert!((a[1].rotulo.as_ref().unwrap().tamanho - b[1].rotulo.as_ref().unwrap().tamanho / 2.0).abs() < 1e-4);
    }
}
