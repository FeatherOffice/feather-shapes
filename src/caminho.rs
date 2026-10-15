//! Caminhos vetoriais normalizados na caixa da forma, nos dois formatos de texto que os
//! aplicativos gravam:
//!
//! - **SVG1**: `SVG1\n<regra>\n` seguido de um comando por linha (`M x y`, `L x y`,
//!   `Q cx cy x y`, `C c1x c1y c2x c2y x y`, `Z`), com coordenadas de 0 a 1. A regra é
//!   `normal` (não-zero) ou `par` (par/ímpar). Linhas que começam com `#` são metadados
//!   do aplicativo e são ignoradas aqui.
//! - **Polígonos**: `u,v;u,v;…|u,v;…` (anéis separados por `|`), o resultado das operações
//!   booleanas.

use serde::Serialize;

/// Um comando do caminho, já em coordenadas do documento.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Segmento {
    M([f32; 2]),
    L([f32; 2]),
    Q([f32; 2], [f32; 2]),
    C([f32; 2], [f32; 2], [f32; 2]),
    Z,
}

/// Cabeçalho do formato SVG1.
pub const CABECALHO_SVG1: &str = "SVG1\n";

/// O texto está no formato SVG1?
pub fn eh_svg1(dados: &str) -> bool {
    dados.starts_with(CABECALHO_SVG1)
}

/// Lê um caminho SVG1 e o coloca na `caixa` (`[x, y, largura, altura]`). Devolve a regra
/// (`true` = par/ímpar) e os segmentos. Só aceita números finitos e comandos normalizados.
pub fn decodificar_svg1(dados: &str, caixa: [f32; 4]) -> Result<(bool, Vec<Segmento>), String> {
    let [x0, y0, largura, altura] = caixa;
    if caixa.iter().any(|n| !n.is_finite()) {
        return Err("Caixa SVG inválida.".into());
    }
    let mut linhas = dados
        .strip_prefix(CABECALHO_SVG1)
        .ok_or("Elemento não é SVG.")?
        .lines();
    let par = match linhas.next() {
        Some("par") => true,
        Some("normal") => false,
        _ => return Err("Regra de preenchimento SVG inválida.".into()),
    };
    let mut segmentos = Vec::new();
    for linha in linhas {
        if linha.starts_with("#CON ") { continue; }
        if segmentos.len() >= 100_000 {
            return Err("SVG excede o limite de segmentos.".into());
        }
        let mut campos = linha.split_whitespace();
        let comando = campos.next().ok_or("Comando SVG vazio.")?;
        let mut ponto = || -> Result<[f32; 2], String> {
            let mut numero = || {
                campos
                    .next()
                    .and_then(|s| s.parse::<f32>().ok())
                    .filter(|n| n.is_finite())
                    .ok_or_else(|| "Coordenada SVG inválida.".to_string())
            };
            let x = numero()?;
            let y = numero()?;
            Ok([x0 + x * largura, y0 + y * altura])
        };
        let segmento = match comando {
            "M" => Segmento::M(ponto()?),
            "L" => Segmento::L(ponto()?),
            "Q" => Segmento::Q(ponto()?, ponto()?),
            "C" => Segmento::C(ponto()?, ponto()?, ponto()?),
            "Z" => Segmento::Z,
            _ => return Err("Comando SVG inválido.".into()),
        };
        if campos.next().is_some() {
            return Err("Comando SVG tem coordenadas excedentes.".into());
        }
        segmentos.push(segmento);
    }
    if !matches!(segmentos.first(), Some(Segmento::M(_))) {
        return Err("SVG sem início de caminho.".into());
    }
    Ok((par, segmentos))
}

/// Os segmentos como `Caminho` do feather-draw.
pub fn para_caminho(par: bool, segmentos: &[Segmento]) -> feather_draw::Caminho {
    use feather_draw::{Comando, Ponto, RegraPreenchimento};
    let p = |q: [f32; 2]| Ponto::novo(q[0], q[1]);
    let mut c = feather_draw::Caminho::novo();
    c.regra = if par { RegraPreenchimento::ParImpar } else { RegraPreenchimento::NaoZero };
    for s in segmentos {
        c.adicionar(match *s {
            Segmento::M(a) => Comando::Mover(p(a)),
            Segmento::L(a) => Comando::Linha(p(a)),
            Segmento::Q(a, b) => Comando::Quadratica { controle: p(a), fim: p(b) },
            Segmento::C(a, b, d) => Comando::Cubica { controle1: p(a), controle2: p(b), fim: p(d) },
            Segmento::Z => Comando::Fechar,
        });
    }
    c
}

/// Lê os anéis `u,v;…|…` (coordenadas de 0 a 1) e os coloca no retângulo, em `f64`.
pub fn decodificar_poligonos(dados: &str, x: f64, y: f64, w: f64, h: f64) -> Result<Vec<Vec<[f64; 2]>>, String> {
    let mut resultado = Vec::new();
    for anel in dados.split('|').filter(|s| !s.is_empty()) {
        let mut pontos = Vec::new();
        for par in anel.split(';') {
            let (a, b) = par.split_once(',').ok_or("Caminho vetorial inválido.")?;
            let u: f64 = a.parse().map_err(|_| "Caminho vetorial inválido.")?;
            let v: f64 = b.parse().map_err(|_| "Caminho vetorial inválido.")?;
            if !u.is_finite() || !v.is_finite() { return Err("Caminho contém coordenadas inválidas.".into()); }
            pontos.push([x + w * u, y + h * v]);
        }
        if pontos.len() >= 3 { resultado.push(pontos); }
    }
    if resultado.is_empty() { return Err("O caminho vetorial está vazio.".into()); }
    Ok(resultado)
}

/// Grava os anéis no formato `u,v;…|…`, normalizados na caixa `[x, y, w, h]`.
pub fn codificar_poligonos(contornos: &[Vec<[f64; 2]>], x: f64, y: f64, w: f64, h: f64) -> String {
    contornos.iter().map(|c| c.iter().map(|p|
        format!("{:.9},{:.9}", (p[0] - x) / w, (p[1] - y) / h)
    ).collect::<Vec<_>>().join(";")).collect::<Vec<_>>().join("|")
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn svg1_le_curvas_na_caixa_e_recusa_lixo() {
        let (par, s) = decodificar_svg1("SVG1\npar\nM 0 0\nC 0.5 0 1 0.5 1 1\nZ\n#CON 1 2 reto\n", [10.0, 20.0, 100.0, 50.0]).unwrap();
        assert!(par);
        assert_eq!(s, vec![Segmento::M([10.0, 20.0]), Segmento::C([60.0, 20.0], [110.0, 45.0], [110.0, 70.0]), Segmento::Z]);
        assert!(decodificar_svg1("SVG1\nnormal\nL 0 0\n", [0.0; 4]).is_err());
        assert!(decodificar_svg1("SVG1\nnormal\nM 0 NaN\n", [0.0, 0.0, 1.0, 1.0]).is_err());
        assert!(decodificar_svg1("SVG1\nnormal\nM 0 0 1\n", [0.0, 0.0, 1.0, 1.0]).is_err());
        assert!(decodificar_svg1("xx", [0.0, 0.0, 1.0, 1.0]).is_err());
        let c = para_caminho(false, &s);
        assert_eq!(c.comandos.len(), 3);
    }

    #[test]
    fn poligonos_ida_e_volta() {
        let aneis = vec![vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0]]];
        let texto = codificar_poligonos(&aneis, 0.0, 0.0, 10.0, 5.0);
        assert_eq!(texto, "0.000000000,0.000000000;1.000000000,0.000000000;1.000000000,1.000000000");
        assert_eq!(decodificar_poligonos(&texto, 0.0, 0.0, 10.0, 5.0).unwrap(), aneis);
        assert!(decodificar_poligonos("", 0.0, 0.0, 1.0, 1.0).is_err());
    }
}
