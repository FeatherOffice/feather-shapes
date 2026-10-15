//! O catálogo de formas: cada tipo tem um identificador estável em texto (o que vai gravado
//! nos arquivos dos aplicativos), o nome em português, o `Id` do catálogo do README (`F001`…)
//! e o nome `prstGeom` do DrawingML quando existe equivalente no Office.

/// Família da forma no catálogo (as seções do README).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CategoriaForma {
    LinhasEConectores,
    Retangulos,
    Basicas,
    SetasLargas,
    Equacoes,
    Fluxograma,
    EstrelasEFaixas,
    Baloes,
    BotoesDeAcao,
    /// Caixa de texto: retângulo sem desenho próprio, só a área do texto.
    Texto,
}

impl CategoriaForma {
    /// Nome para mostrar.
    pub const fn nome(self) -> &'static str {
        match self {
            Self::LinhasEConectores => "Linhas e conectores",
            Self::Retangulos => "Retângulos",
            Self::Basicas => "Formas básicas",
            Self::SetasLargas => "Setas largas",
            Self::Equacoes => "Equações",
            Self::Fluxograma => "Fluxograma",
            Self::EstrelasEFaixas => "Estrelas e faixas",
            Self::Baloes => "Textos explicativos (balões)",
            Self::BotoesDeAcao => "Botões de ação",
            Self::Texto => "Texto",
        }
    }
}

macro_rules! catalogo {
    ($( $variante:ident => ($id:literal, $nome:literal, $codigo:expr, $prst:expr, $categoria:ident) ),* $(,)?) => {
        /// Um tipo de forma do catálogo. A ordem das variantes **não** é contrato de arquivo:
        /// o que se grava é [`TipoForma::id`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum TipoForma { $( $variante ),* }

        impl TipoForma {
            /// Todos os tipos, na ordem do catálogo.
            pub const TODOS: &'static [TipoForma] = &[ $( TipoForma::$variante ),* ];

            /// Identificador estável em texto (`retangulo`, `seta-direita`…).
            pub const fn id(self) -> &'static str {
                match self { $( TipoForma::$variante => $id ),* }
            }

            /// Nome em português, como aparece na galeria.
            pub const fn nome(self) -> &'static str {
                match self { $( TipoForma::$variante => $nome ),* }
            }

            /// O `Id` do catálogo do README (`F001`…), quando a forma já está catalogada.
            pub const fn codigo(self) -> Option<&'static str> {
                match self { $( TipoForma::$variante => $codigo ),* }
            }

            /// O nome `prstGeom` do DrawingML (Office Open XML), quando há equivalente.
            pub const fn prst(self) -> Option<&'static str> {
                match self { $( TipoForma::$variante => $prst ),* }
            }

            /// A família da forma.
            pub const fn categoria(self) -> CategoriaForma {
                match self { $( TipoForma::$variante => CategoriaForma::$categoria ),* }
            }
        }
    };
}

catalogo! {
    Linha => ("linha", "Linha", Some("F001"), Some("line"), LinhasEConectores),
    ConectorReto => ("conector-reto", "Conector reto", Some("F002"), Some("straightConnector1"), LinhasEConectores),
    ConectorCotovelo => ("conector-cotovelo", "Conector em cotovelo", Some("F004"), Some("bentConnector3"), LinhasEConectores),
    ConectorCurvo => ("conector-curvo", "Conector curvo", Some("F008"), Some("curvedConnector3"), LinhasEConectores),
    Retangulo => ("retangulo", "Retângulo", Some("F011"), Some("rect"), Retangulos),
    RetanguloArredondado => ("retangulo-arredondado", "Retângulo de cantos arredondados", Some("F012"), Some("roundRect"), Retangulos),
    Elipse => ("elipse", "Elipse", Some("F020"), Some("ellipse"), Basicas),
    Circulo => ("circulo", "Círculo", Some("F021"), Some("ellipse"), Basicas),
    Triangulo => ("triangulo", "Triângulo", Some("F022"), Some("triangle"), Basicas),
    TrianguloRetangulo => ("triangulo-retangulo", "Triângulo retângulo", Some("F023"), Some("rtTriangle"), Basicas),
    Paralelogramo => ("paralelogramo", "Paralelogramo", Some("F024"), Some("parallelogram"), Basicas),
    Trapezio => ("trapezio", "Trapézio", Some("F025"), Some("trapezoid"), Basicas),
    Losango => ("losango", "Losango", Some("F026"), Some("diamond"), Basicas),
    Pentagono => ("pentagono", "Pentágono", Some("F027"), Some("pentagon"), Basicas),
    Hexagono => ("hexagono", "Hexágono", Some("F028"), Some("hexagon"), Basicas),
    Heptagono => ("heptagono", "Heptágono", Some("F029"), Some("heptagon"), Basicas),
    Octogono => ("octogono", "Octógono", Some("F030"), Some("octagon"), Basicas),
    Decagono => ("decagono", "Decágono", Some("F031"), Some("decagon"), Basicas),
    Dodecagono => ("dodecagono", "Dodecágono", Some("F032"), Some("dodecagon"), Basicas),
    Setor => ("setor", "Setor", Some("F033"), Some("pie"), Basicas),
    Corda => ("corda", "Corda", Some("F034"), Some("chord"), Basicas),
    Moldura => ("moldura", "Moldura", Some("F036"), Some("frame"), Basicas),
    CantoEmL => ("canto-em-l", "Canto em L", Some("F038"), Some("corner"), Basicas),
    Cruz => ("cruz", "Cruz", Some("F040"), Some("plus"), Basicas),
    Placa => ("placa", "Placa", Some("F041"), Some("plaque"), Basicas),
    Cilindro => ("cilindro", "Cilindro", Some("F042"), Some("can"), Basicas),
    Cubo => ("cubo", "Cubo", Some("F043"), Some("cube"), Basicas),
    Anel => ("anel", "Anel", Some("F045"), Some("donut"), Basicas),
    Documento => ("documento", "Documento", Some("F048"), Some("foldedCorner"), Basicas),
    Coracao => ("coracao", "Coração", Some("F050"), Some("heart"), Basicas),
    Raio => ("raio", "Raio", Some("F051"), Some("lightningBolt"), Basicas),
    Sol => ("sol", "Sol", Some("F052"), Some("sun"), Basicas),
    Lua => ("lua", "Lua", Some("F053"), Some("moon"), Basicas),
    Nuvem => ("nuvem", "Nuvem", Some("F054"), Some("cloud"), Basicas),
    Arco => ("arco", "Arco", Some("F055"), Some("arc"), Basicas),
    SetaDireita => ("seta-direita", "Seta", Some("F072"), Some("rightArrow"), SetasLargas),
    SetaEsquerda => ("seta-esquerda", "Seta à esquerda", Some("F073"), Some("leftArrow"), SetasLargas),
    SetaCima => ("seta-cima", "Seta para cima", Some("F074"), Some("upArrow"), SetasLargas),
    SetaBaixo => ("seta-baixo", "Seta para baixo", Some("F075"), Some("downArrow"), SetasLargas),
    SetaDupla => ("seta-dupla", "Seta dupla", Some("F076"), Some("leftRightArrow"), SetasLargas),
    Chevron => ("chevron", "Chevron", Some("F091"), Some("chevron"), SetasLargas),
    Mais => ("mais", "Mais", Some("F103"), Some("mathPlus"), Equacoes),
    Menos => ("menos", "Menos", Some("F104"), Some("mathMinus"), Equacoes),
    Multiplicacao => ("multiplicacao", "Multiplicação", Some("F105"), Some("mathMultiply"), Equacoes),
    Divisao => ("divisao", "Divisão", Some("F106"), Some("mathDivide"), Equacoes),
    Igual => ("igual", "Igual", Some("F107"), Some("mathEqual"), Equacoes),
    Diferente => ("diferente", "Diferente", Some("F108"), Some("mathNotEqual"), Equacoes),
    FluxoProcesso => ("fluxo-processo", "Processo", Some("F109"), Some("flowChartProcess"), Fluxograma),
    FluxoDecisao => ("fluxo-decisao", "Decisão", Some("F111"), Some("flowChartDecision"), Fluxograma),
    FluxoDados => ("fluxo-dados", "Entrada / saída", Some("F112"), Some("flowChartInputOutput"), Fluxograma),
    FluxoProcessoPredefinido => ("fluxo-processo-predefinido", "Processo predefinido", Some("F113"), Some("flowChartPredefinedProcess"), Fluxograma),
    FluxoDocumento => ("fluxo-documento", "Documento", Some("F115"), Some("flowChartDocument"), Fluxograma),
    FluxoTerminador => ("fluxo-terminador", "Início / fim", Some("F117"), Some("flowChartTerminator"), Fluxograma),
    FluxoConectorPagina => ("fluxo-conector-pagina", "Conexão de página", Some("F122"), Some("flowChartOffpageConnector"), Fluxograma),
    FluxoArmazenamento => ("fluxo-armazenamento", "Armazenamento", Some("F134"), Some("flowChartMagneticDisk"), Fluxograma),
    Explosao => ("explosao", "Explosão", Some("F139"), Some("irregularSeal1"), EstrelasEFaixas),
    Estrela4 => ("estrela-4", "Estrela (4 pontas)", Some("F141"), Some("star4"), EstrelasEFaixas),
    Estrela5 => ("estrela-5", "Estrela", Some("F142"), Some("star5"), EstrelasEFaixas),
    Estrela6 => ("estrela-6", "Estrela (6 pontas)", Some("F143"), Some("star6"), EstrelasEFaixas),
    Estrela8 => ("estrela-8", "Estrela (8 pontas)", Some("F145"), Some("star8"), EstrelasEFaixas),
    Estrela10 => ("estrela-10", "Estrela (10 pontas)", Some("F146"), Some("star10"), EstrelasEFaixas),
    Estrela12 => ("estrela-12", "Estrela (12 pontas)", Some("F147"), Some("star12"), EstrelasEFaixas),
    Faixa => ("faixa", "Faixa", Some("F151"), Some("ribbon2"), EstrelasEFaixas),
    Fita => ("fita", "Fita dobrada", Some("F152"), Some("ribbon"), EstrelasEFaixas),
    Onda => ("onda", "Onda", Some("F157"), Some("wave"), EstrelasEFaixas),
    BalaoFala => ("balao-fala", "Balão de fala", Some("F160"), Some("wedgeRoundRectCallout"), Baloes),
    BalaoPensamento => ("balao-pensamento", "Balão de pensamento", Some("F162"), Some("cloudCallout"), Baloes),
    AcaoInicio => ("acao-inicio", "Início", Some("F176"), Some("actionButtonHome"), BotoesDeAcao),
    AcaoInformacao => ("acao-informacao", "Informação", Some("F178"), Some("actionButtonInformation"), BotoesDeAcao),
    AcaoProximo => ("acao-proximo", "Próximo", Some("F179"), Some("actionButtonForwardNext"), BotoesDeAcao),
    AcaoAnterior => ("acao-anterior", "Anterior", Some("F180"), Some("actionButtonBackPrevious"), BotoesDeAcao),
    CaixaTexto => ("caixa-texto", "Caixa de texto", None, Some("rect"), Texto),
}

impl TipoForma {
    /// O tipo pelo identificador. Aceita `_` no lugar de `-` (`seta_esquerda`) e o código
    /// do catálogo (`F072`).
    pub fn pelo_id(id: &str) -> Option<TipoForma> {
        let id = id.trim();
        Self::TODOS.iter().copied().find(|t| {
            t.id() == id
                || t.id().replace('-', "_") == id
                || t.codigo() == Some(id)
        })
    }

    /// O primeiro tipo com esse nome `prstGeom` (para ler `.pptx`, `.docx` e `.xlsx`).
    pub fn pelo_prst(prst: &str) -> Option<TipoForma> {
        Self::TODOS.iter().copied().find(|t| t.prst() == Some(prst))
    }

    /// O identificador da geometria na galeria de formas vetoriais (`galeria::geometria`),
    /// para os tipos que são desenhados por caminho SVG normalizado.
    pub const fn id_galeria(self) -> Option<&'static str> {
        Some(match self {
            Self::FluxoDecisao => "decisao",
            Self::FluxoProcesso => "processo",
            Self::FluxoProcessoPredefinido => "processo_predefinido",
            Self::FluxoTerminador => "terminador",
            Self::FluxoDados => "entrada",
            Self::FluxoDocumento => "documento",
            Self::FluxoArmazenamento => "armazenamento",
            Self::FluxoConectorPagina => "conexao",
            Self::BalaoFala => "fala",
            Self::BalaoPensamento => "pensamento",
            Self::Faixa => "faixa",
            Self::Fita => "fita",
            Self::Onda => "onda",
            Self::Explosao => "explosao",
            Self::Mais => "mais",
            Self::Menos => "menos",
            Self::Multiplicacao => "vezes",
            Self::Divisao => "divisao",
            Self::Igual => "igual",
            Self::Diferente => "diferente",
            Self::AcaoInicio => "acao_inicio",
            Self::AcaoAnterior => "acao_anterior",
            Self::AcaoProximo => "acao_proximo",
            Self::AcaoInformacao => "acao_info",
            Self::ConectorReto => "conector_reto",
            Self::ConectorCotovelo => "conector_cotovelo",
            Self::ConectorCurvo => "conector_curvo",
            _ => return None,
        })
    }

    /// O tipo de uma entrada da galeria (`decisao`, `acao_inicio`…).
    pub fn pelo_id_galeria(id: &str) -> Option<TipoForma> {
        Self::TODOS.iter().copied().find(|t| t.id_galeria() == Some(id))
    }

    /// A forma é só traço (não tem região preenchida)?
    pub const fn so_traco(self) -> bool {
        matches!(self, Self::Linha | Self::Arco | Self::ConectorReto | Self::ConectorCotovelo | Self::ConectorCurvo)
    }
}

impl std::fmt::Display for TipoForma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.nome())
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn ids_e_codigos_sao_unicos_e_voltam_ao_tipo() {
        let mut ids = std::collections::HashSet::new();
        let mut codigos = std::collections::HashSet::new();
        for &t in TipoForma::TODOS {
            assert!(ids.insert(t.id()), "id repetido: {}", t.id());
            if let Some(c) = t.codigo() {
                assert!(codigos.insert(c), "código repetido: {c}");
                assert_eq!(TipoForma::pelo_id(c), Some(t));
            }
            assert_eq!(TipoForma::pelo_id(t.id()), Some(t));
            assert_eq!(TipoForma::pelo_id(&t.id().replace('-', "_")), Some(t));
            assert!(t.id().chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'));
        }
        assert_eq!(TipoForma::pelo_prst("rightArrow"), Some(TipoForma::SetaDireita));
        assert_eq!(TipoForma::pelo_id_galeria("acao_info"), Some(TipoForma::AcaoInformacao));
        assert!(TipoForma::pelo_id("nao-existe").is_none());
    }
}
