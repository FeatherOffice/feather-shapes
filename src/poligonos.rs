//! Formas poligonais: o contorno é uma lista de vértices normalizados (0 a 1) na caixa da
//! forma. A geometria é calculada aqui e compartilhada por tela, PDF, SVG e MCP.

use crate::tipo::TipoForma;

/// Formas adicionais da galeria, na ordem do menu (o rótulo é [`TipoForma::nome`]).
pub const ADICIONAIS: &[TipoForma] = &[
    TipoForma::Losango,
    TipoForma::Circulo,
    TipoForma::Anel,
    TipoForma::Arco,
    TipoForma::Corda,
    TipoForma::Cubo,
    TipoForma::Cilindro,
    TipoForma::Moldura,
    TipoForma::CantoEmL,
    TipoForma::Placa,
    TipoForma::Sol,
    TipoForma::Lua,
    TipoForma::Coracao,
    TipoForma::Raio,
    TipoForma::Nuvem,
    TipoForma::Estrela4,
    TipoForma::Estrela6,
    TipoForma::Estrela8,
    TipoForma::Estrela10,
    TipoForma::Estrela12,
    TipoForma::TrianguloRetangulo,
    TipoForma::Heptagono,
    TipoForma::Decagono,
    TipoForma::Dodecagono,
    TipoForma::Pentagono,
    TipoForma::Hexagono,
    TipoForma::Octogono,
    TipoForma::Trapezio,
    TipoForma::Paralelogramo,
    TipoForma::Estrela5,
    TipoForma::Cruz,
    TipoForma::SetaEsquerda,
    TipoForma::SetaDupla,
    TipoForma::SetaCima,
    TipoForma::SetaBaixo,
    TipoForma::Chevron,
    TipoForma::Documento,
];

fn radial(n: usize, estrela: bool) -> Vec<[f32; 2]> {
    let total = if estrela { n * 2 } else { n };
    (0..total)
        .map(|i| {
            let angulo =
                -std::f32::consts::FRAC_PI_2 + std::f32::consts::TAU * i as f32 / total as f32;
            let raio = if estrela && i % 2 == 1 { 0.22 } else { 0.5 };
            [0.5 + raio * angulo.cos(), 0.5 + raio * angulo.sin()]
        })
        .collect()
}

fn seta() -> Vec<[f32; 2]> {
    vec![
        [0.0, 0.3],
        [0.6, 0.3],
        [0.6, 0.0],
        [1.0, 0.5],
        [0.6, 1.0],
        [0.6, 0.7],
        [0.0, 0.7],
    ]
}

/// Vértices normalizados (0 a 1) das formas poligonais; `None` para as demais.
pub fn normalizados(tipo: TipoForma) -> Option<Vec<[f32; 2]>> {
    Some(match tipo {
        TipoForma::Losango => vec![[0.5, 0.0], [1.0, 0.5], [0.5, 1.0], [0.0, 0.5]],
        TipoForma::Pentagono => radial(5, false),
        TipoForma::Hexagono => radial(6, false),
        TipoForma::Octogono => radial(8, false),
        TipoForma::TrianguloRetangulo => vec![[0.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        TipoForma::Heptagono => radial(7, false),
        TipoForma::Decagono => radial(10, false),
        TipoForma::Dodecagono => radial(12, false),
        TipoForma::Estrela5 => radial(5, true),
        TipoForma::Trapezio => vec![[0.2, 0.0], [0.8, 0.0], [1.0, 1.0], [0.0, 1.0]],
        TipoForma::Paralelogramo => vec![[0.25, 0.0], [1.0, 0.0], [0.75, 1.0], [0.0, 1.0]],
        TipoForma::Cruz => vec![
            [0.35, 0.0],
            [0.65, 0.0],
            [0.65, 0.35],
            [1.0, 0.35],
            [1.0, 0.65],
            [0.65, 0.65],
            [0.65, 1.0],
            [0.35, 1.0],
            [0.35, 0.65],
            [0.0, 0.65],
            [0.0, 0.35],
            [0.35, 0.35],
        ],
        TipoForma::SetaEsquerda => seta().into_iter().map(|[u, v]| [1.0 - u, v]).collect(),
        TipoForma::SetaCima => seta().into_iter().map(|[u, v]| [v, 1.0 - u]).collect(),
        TipoForma::SetaBaixo => seta().into_iter().map(|[u, v]| [1.0 - v, u]).collect(),
        TipoForma::SetaDupla => vec![
            [0.0, 0.5],
            [0.25, 0.0],
            [0.25, 0.3],
            [0.75, 0.3],
            [0.75, 0.0],
            [1.0, 0.5],
            [0.75, 1.0],
            [0.75, 0.7],
            [0.25, 0.7],
            [0.25, 1.0],
        ],
        TipoForma::Chevron => vec![
            [0.0, 0.0],
            [0.6, 0.0],
            [1.0, 0.5],
            [0.6, 1.0],
            [0.0, 1.0],
            [0.4, 0.5],
        ],
        TipoForma::Documento => vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 0.85],
            [0.85, 0.93],
            [0.7, 0.96],
            [0.55, 0.92],
            [0.4, 0.84],
            [0.25, 0.8],
            [0.1, 0.84],
            [0.0, 0.9],
        ],
        _ => return None,
    })
}

/// Contorno de uma forma poligonal no retângulo (`x`, `y`, `largura`, `altura`); `None` para
/// os tipos que não são polígonos simples.
pub fn pontos(tipo: TipoForma, x: f32, y: f32, largura: f32, altura: f32) -> Option<Vec<[f32; 2]>> {
    Some(
        normalizados(tipo)?
            .into_iter()
            .map(|[u, v]| [x + u * largura, y + v * altura])
            .collect(),
    )
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn todas_as_formas_tem_contorno_dentro_da_caixa() {
        for &tipo in ADICIONAIS {
            if crate::basicas::suporta(tipo) {
                let d = crate::basicas::desenho(tipo, 10.0, 20.0, 200.0, 100.0).unwrap();
                assert!(!d.contornos.is_empty() || !d.tracos.is_empty());
            } else {
                let p = pontos(tipo, 10.0, 20.0, 200.0, 100.0).unwrap();
                assert!(p.len() >= 3);
                assert!(p.iter().all(|[x, y]|
                    (10.0..=210.0).contains(x) && (20.0..=120.0).contains(y)));
            }
        }
        assert!(pontos(TipoForma::CaixaTexto, 0.0, 0.0, 1.0, 1.0).is_none());
    }

    #[test]
    fn novos_poligonos_e_triangulo_retangulo_tem_vertices_corretos() {
        for (tipo, quantidade) in [
            (TipoForma::TrianguloRetangulo, 3),
            (TipoForma::Heptagono, 7),
            (TipoForma::Decagono, 10),
            (TipoForma::Dodecagono, 12),
        ] {
            let pts = pontos(tipo, 10.0, 20.0, 200.0, 100.0).unwrap();
            assert_eq!(pts.len(), quantidade, "{}", tipo.id());
            assert!(pts.iter().all(|[x, y]| (10.0..=210.0).contains(x) && (20.0..=120.0).contains(y)));
        }
        assert_eq!(
            pontos(TipoForma::TrianguloRetangulo, 10.0, 20.0, 200.0, 100.0).unwrap(),
            vec![[10.0, 20.0], [210.0, 120.0], [10.0, 120.0]],
        );
    }

    #[test]
    fn setas_apontam_para_o_lado_escolhido() {
        for (tipo, ponta) in [
            (TipoForma::SetaEsquerda, [0.0, 0.5]),
            (TipoForma::SetaCima, [0.5, 0.0]),
            (TipoForma::SetaBaixo, [0.5, 1.0]),
        ] {
            assert_eq!(pontos(tipo, 0.0, 0.0, 1.0, 1.0).unwrap()[3], ponta);
        }
    }
}
