//! # feather-shapes
//!
//! Formas, ícones e SmartArt da suíte FeatherWorks, em Rust puro e sem interface: as mesmas
//! formas no FeatherSheet, no FeatherDocs e no FeatherSlide, com a mesma geometria e a mesma
//! aparência. A saída é uma `Cena` do [`feather_draw`], que vai para a tela, o SVG e o PDF.
//!
//! - [`tipo`]: o catálogo ([`TipoForma`]: id estável, nome, código `F0xx` e `prstGeom`);
//! - [`forma`]: o modelo neutro ([`Forma`]), a [`Geometria`], a `Cena` e o teste de acerto;
//! - [`aparencia`]: preenchimento, gradiente, contorno, opacidade e sombra;
//! - [`poligonos`], [`basicas`], [`galeria`]: a geometria de cada família de formas;
//! - [`caminho`]: os caminhos normalizados em texto (SVG1 e polígonos) que os apps gravam;
//! - [`booleanas`]: união, interseção, subtração, combinação e fragmentação;
//! - [`wordart`]: as deformações do texto artístico;
//! - [`svgpath`]: leitor do atributo `d` do SVG;
//! - [`icones`]: os ícones Tabler embutidos, com busca em português e inglês;
//! - [`diagramas`] e [`arranjo`]: os modelos de SmartArt e a lógica do editor de diagramas.
//!
//! A biblioteca não conhece documento, não guarda histórico e não acessa a rede: cada
//! função devolve valores novos e quem desfaz é o aplicativo.

pub mod aparencia;
pub mod arranjo;
pub mod basicas;
pub mod booleanas;
pub mod caminho;
pub mod diagramas;
pub mod forma;
pub mod galeria;
pub mod icones;
pub mod poligonos;
pub mod svgpath;
pub mod tipo;
pub mod wordart;

pub use aparencia::{Aparencia, Gradiente, PontoCor, Sombra, TipoGradiente};
pub use arranjo::{Arranjo, Diagrama, NoDiagrama};
pub use booleanas::OperacaoBooleana;
pub use caminho::Segmento;
pub use diagramas::{Categoria, ItemDiagrama};
pub use forma::{Forma, Geometria};
pub use tipo::{CategoriaForma, TipoForma};
pub use wordart::Transformacao;

/// Reexporta o feather-draw na versão usada pela biblioteca.
pub use feather_draw;
