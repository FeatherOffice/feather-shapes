//! Galeria de formas vetoriais desenhadas por caminho SVG1 normalizado (fluxogramas, balões,
//! faixas, ondas, matemáticas e botões de ação) e a rota dos conectores que ligam duas formas.
//! A geometria é a mesma na prévia da galeria, na tela, no PDF e no MCP.

/// Categoria, identificador estável e nome de cada opção da galeria.
pub const FORMAS: &[(&str, &str, &str)] = &[
    ("Fluxogramas", "decisao", "Decisão"),
    ("Fluxogramas", "processo", "Processo"),
    (
        "Fluxogramas",
        "processo_predefinido",
        "Processo predefinido",
    ),
    ("Fluxogramas", "terminador", "Início / fim"),
    ("Fluxogramas", "entrada", "Entrada / saída"),
    ("Fluxogramas", "documento", "Documento"),
    ("Fluxogramas", "armazenamento", "Armazenamento"),
    ("Fluxogramas", "conexao", "Conexão de página"),
    ("Balões", "fala", "Balão de fala"),
    ("Balões", "pensamento", "Balão de pensamento"),
    ("Faixas e fitas", "faixa", "Faixa"),
    ("Faixas e fitas", "fita", "Fita dobrada"),
    ("Ondas e explosões", "onda", "Onda"),
    ("Ondas e explosões", "explosao", "Explosão"),
    ("Matemáticas", "mais", "Mais"),
    ("Matemáticas", "menos", "Menos"),
    ("Matemáticas", "vezes", "Multiplicação"),
    ("Matemáticas", "divisao", "Divisão"),
    ("Matemáticas", "igual", "Igual"),
    ("Matemáticas", "diferente", "Diferente"),
    ("Ação", "acao_inicio", "Início"),
    ("Ação", "acao_anterior", "Anterior"),
    ("Ação", "acao_proximo", "Próximo"),
    ("Ação", "acao_info", "Informação"),
];

fn oval(cx: f32, cy: f32, rx: f32, ry: f32) -> String {
    let mut s = String::new();
    for i in 0..48 {
        let a = i as f32 * std::f32::consts::TAU / 48.0;
        s.push_str(&format!(
            "{} {} {}\n",
            if i == 0 { "M" } else { "L" },
            cx + rx * a.cos(),
            cy + ry * a.sin()
        ));
    }
    s.push_str("Z\n");
    s
}

/// Geometria normalizada compartilhada por prévia, tela, PDF e MCP.
pub fn geometria(id: &str) -> Result<String, String> {
    let rect = "M 0.05 0.05\nL 0.95 0.05\nL 0.95 0.95\nL 0.05 0.95\nZ\n";
    let desenho = match id {
        "decisao" => "M 0.5 0.02\nL 0.98 0.5\nL 0.5 0.98\nL 0.02 0.5\nZ\n".into(),
        "conector_reto" => "M 0.05 0.1\nL 0.95 0.9\n".into(),
        "conector_cotovelo" => "M 0.05 0.1\nL 0.5 0.1\nL 0.5 0.9\nL 0.95 0.9\n".into(),
        "conector_curvo" => "M 0.05 0.1\nC 0.8 0.1 0.2 0.9 0.95 0.9\n".into(),
        "livre" => "M 0.05 0.75\nL 0.23 0.3\nL 0.35 0.55\nL 0.5 0.15\nL 0.7 0.8\nL 0.95 0.3\n".into(),
        "processo" => rect.into(),
        "processo_predefinido" => format!("{rect}M 0.18 0.05\nL 0.18 0.95\nM 0.82 0.05\nL 0.82 0.95\n"),
        "terminador" => "M 0.25 0.1\nL 0.75 0.1\nC 1.02 0.1 1.02 0.9 0.75 0.9\nL 0.25 0.9\nC -0.02 0.9 -0.02 0.1 0.25 0.1\nZ\n".into(),
        "entrada" => "M 0.23 0.05\nL 0.98 0.05\nL 0.77 0.95\nL 0.02 0.95\nZ\n".into(),
        "documento" => "M 0.05 0.05\nL 0.95 0.05\nL 0.95 0.8\nC 0.65 0.5 0.35 1.1 0.05 0.8\nZ\n".into(),
        "armazenamento" => "M 0.05 0.2\nC 0.05 0 0.95 0 0.95 0.2\nL 0.95 0.8\nC 0.95 1 0.05 1 0.05 0.8\nZ\nM 0.05 0.2\nC 0.05 0.4 0.95 0.4 0.95 0.2\n".into(),
        "conexao" => oval(0.5,0.5,0.45,0.45),
        "fala" => "M 0.08 0.05\nL 0.92 0.05\nQ 0.98 0.05 0.98 0.12\nL 0.98 0.68\nQ 0.98 0.75 0.92 0.75\nL 0.4 0.75\nL 0.16 0.98\nL 0.21 0.75\nL 0.08 0.75\nQ 0.02 0.75 0.02 0.68\nL 0.02 0.12\nQ 0.02 0.05 0.08 0.05\nZ\n".into(),
        "pensamento" => {
            let mut s=String::new();
            for i in 0..120 { let a=i as f32*std::f32::consts::TAU/120.0; let r=1.0+0.09*(a*9.0).cos(); s.push_str(&format!("{} {} {}\n",if i==0{"M"}else{"L"},0.5+0.43*r*a.cos(),0.37+0.30*r*a.sin())); }
            s.push_str("Z\n"); s+&oval(0.22,0.79,0.07,0.07)+&oval(0.12,0.93,0.035,0.035)
        },
        "faixa" => "M 0.02 0.2\nL 0.98 0.2\nL 0.82 0.5\nL 0.98 0.8\nL 0.02 0.8\nL 0.18 0.5\nZ\n".into(),
        "fita" => "M 0.02 0.35\nL 0.2 0.35\nL 0.2 0.15\nL 0.8 0.15\nL 0.8 0.35\nL 0.98 0.35\nL 0.88 0.6\nL 0.98 0.85\nL 0.7 0.85\nL 0.7 0.65\nL 0.3 0.65\nL 0.3 0.85\nL 0.02 0.85\nL 0.12 0.6\nZ\nM 0.2 0.35\nL 0.2 0.65\nL 0.3 0.85\nM 0.8 0.35\nL 0.8 0.65\nL 0.7 0.85\n".into(),
        "onda" => "M 0.02 0.2\nC 0.35 -0.15 0.65 0.55 0.98 0.2\nL 0.98 0.8\nC 0.65 1.15 0.35 0.45 0.02 0.8\nZ\n".into(),
        "explosao" => { let mut s=String::new(); for i in 0..24 {let a=i as f32*std::f32::consts::TAU/24.0; let r=if i%2==0{0.48}else{0.26};s.push_str(&format!("{} {} {}\n",if i==0{"M"}else{"L"},0.5+r*a.cos(),0.5+r*a.sin()));} s.push_str("Z\n");s },
        "mais" => "M 0.4 0.05\nL 0.6 0.05\nL 0.6 0.4\nL 0.95 0.4\nL 0.95 0.6\nL 0.6 0.6\nL 0.6 0.95\nL 0.4 0.95\nL 0.4 0.6\nL 0.05 0.6\nL 0.05 0.4\nL 0.4 0.4\nZ\n".into(),
        "menos" => "M 0.05 0.4\nL 0.95 0.4\nL 0.95 0.6\nL 0.05 0.6\nZ\n".into(),
        "vezes" => "M 0.1 0.23\nL 0.23 0.1\nL 0.5 0.37\nL 0.77 0.1\nL 0.9 0.23\nL 0.63 0.5\nL 0.9 0.77\nL 0.77 0.9\nL 0.5 0.63\nL 0.23 0.9\nL 0.1 0.77\nL 0.37 0.5\nZ\n".into(),
        "divisao" => geometria("menos")?.strip_prefix("SVG1\nnormal\n").unwrap_or_default().to_owned()+&oval(0.5,0.17,0.1,0.1)+&oval(0.5,0.83,0.1,0.1),
        "igual" | "diferente" => {let mut s="M 0.08 0.24\nL 0.92 0.24\nL 0.92 0.4\nL 0.08 0.4\nZ\nM 0.08 0.6\nL 0.92 0.6\nL 0.92 0.76\nL 0.08 0.76\nZ\n".to_owned();if id=="diferente"{s.push_str("M 0.68 0.05\nL 0.82 0.12\nL 0.32 0.95\nL 0.18 0.88\nZ\n");}s},
        "acao_inicio" => format!("{rect}M 0.2 0.48\nL 0.5 0.2\nL 0.8 0.48\nL 0.7 0.48\nL 0.7 0.82\nL 0.3 0.82\nL 0.3 0.48\nZ\n"),
        "acao_anterior" => format!("{rect}M 0.7 0.2\nL 0.3 0.5\nL 0.7 0.8\nZ\n"),
        "acao_proximo" => format!("{rect}M 0.3 0.2\nL 0.7 0.5\nL 0.3 0.8\nZ\n"),
        "acao_info" => format!("{rect}{}M 0.5 0.42\nL 0.5 0.8\n",oval(0.5,0.25,0.045,0.045)),
        _ => return Err("Forma de galeria desconhecida.".into()),
    };
    Ok(format!("SVG1\nnormal\n{desenho}"))
}

/// A rota de um conector entre as caixas `a` e `b` (`[x, y, largura, altura]`), ancorada nas
/// bordas na direção do centro da outra caixa. `estilo` é `reto`, `cotovelo` ou `curvo`
/// (desconhecido vale reto). Devolve a caixa do conector e o caminho SVG1 normalizado nela.
pub fn rota_conector(estilo: &str, a: &[f32; 4], b: &[f32; 4]) -> ([f32; 4], String) {
    let centro = |r: &[f32; 4]| [r[0] + r[2] / 2.0, r[1] + r[3] / 2.0];
    let ca = centro(a);
    let cb = centro(b);
    let ancora = |r: &[f32; 4], alvo: [f32; 2]| {
        let c = centro(r);
        let dx = alvo[0] - c[0];
        let dy = alvo[1] - c[1];
        let t = 1.0
            / (dx.abs() / (r[2].abs() / 2.0).max(0.5))
                .max(dy.abs() / (r[3].abs() / 2.0).max(0.5))
                .max(1.0);
        [c[0] + dx * t, c[1] + dy * t]
    };
    let p = ancora(a, cb);
    let q = ancora(b, ca);
    let x = p[0].min(q[0]);
    let y = p[1].min(q[1]);
    let largura = (q[0] - p[0]).abs().max(1.0);
    let altura = (q[1] - p[1]).abs().max(1.0);
    let p = [(p[0] - x) / largura, (p[1] - y) / altura];
    let q = [(q[0] - x) / largura, (q[1] - y) / altura];
    let caminho = match estilo {
        "cotovelo" => format!(
            "L {} {}\nL {} {}\nL {} {}\n",
            (p[0] + q[0]) / 2.0,
            p[1],
            (p[0] + q[0]) / 2.0,
            q[1],
            q[0],
            q[1]
        ),
        "curvo" => format!(
            "C {} {} {} {} {} {}\n",
            (p[0] + q[0]) / 2.0,
            p[1],
            (p[0] + q[0]) / 2.0,
            q[1],
            q[0],
            q[1]
        ),
        _ => format!("L {} {}\n", q[0], q[1]),
    };
    ([x, y, largura, altura], format!("SVG1\nnormal\nM {} {}\n{caminho}", p[0], p[1]))
}

/// Um traço livre (pontos do documento) como caminho SVG1 aberto. Devolve a caixa e o texto.
pub fn traco_livre(pontos: &[[f32; 2]]) -> Result<([f32; 4], String), String> {
    if pontos.len() < 2
        || pontos.len() > 10_000
        || pontos.iter().flatten().any(|n| !n.is_finite())
    {
        return Err("Traço inválido ou longo demais.".into());
    }
    let x = pontos.iter().map(|p| p[0]).fold(f32::INFINITY, f32::min);
    let y = pontos.iter().map(|p| p[1]).fold(f32::INFINITY, f32::min);
    let w = (pontos
        .iter()
        .map(|p| p[0])
        .fold(f32::NEG_INFINITY, f32::max)
        - x)
        .max(1.0);
    let h = (pontos
        .iter()
        .map(|p| p[1])
        .fold(f32::NEG_INFINITY, f32::max)
        - y)
        .max(1.0);
    let mut dados = "SVG1\nnormal\n".to_owned();
    for (i, p) in pontos.iter().enumerate() {
        dados.push_str(&format!(
            "{} {} {}\n",
            if i == 0 { "M" } else { "L" },
            (p[0] - x) / w,
            (p[1] - y) / h
        ));
    }
    Ok(([x, y, w, h], dados))
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::caminho::decodificar_svg1;

    #[test]
    fn todas_as_galerias_tem_geometria_valida() {
        for &(_, id, _) in FORMAS {
            let g = geometria(id).unwrap();
            let (_, s) = decodificar_svg1(&g, [0.0, 0.0, 200.0, 120.0]).unwrap();
            assert!(s.len() >= 2, "{id}");
            assert!(crate::tipo::TipoForma::pelo_id_galeria(id).is_some(), "{id} sem tipo");
        }
        for id in ["conector_reto", "conector_cotovelo", "conector_curvo", "livre"] {
            assert!(geometria(id).is_ok());
        }
        assert!(geometria("nao_existe").is_err());
    }

    #[test]
    fn conector_liga_as_bordas() {
        let (caixa, svg) = rota_conector("reto", &[0.0, 0.0, 100.0, 100.0], &[300.0, 0.0, 100.0, 100.0]);
        assert_eq!(caixa, [100.0, 50.0, 200.0, 1.0]);
        assert_eq!(svg, "SVG1\nnormal\nM 0 0\nL 1 0\n");
        for estilo in ["reto", "cotovelo", "curvo"] {
            let (c, s) = rota_conector(estilo, &[0.0, 0.0, 100.0, 100.0], &[300.0, 200.0, 100.0, 100.0]);
            assert!(decodificar_svg1(&s, c).is_ok());
        }
    }

    #[test]
    fn traco_livre_preserva_pontos_e_recusa_invalidos() {
        let pontos = [[10.0, 20.0], [40.0, 60.0], [90.0, 30.0]];
        let (caixa, svg) = traco_livre(&pontos).unwrap();
        let (_, s) = decodificar_svg1(&svg, caixa).unwrap();
        assert_eq!(s.len(), 3);
        assert_eq!(s[2], crate::caminho::Segmento::L(pontos[2]));
        assert!(traco_livre(&[[f32::NAN, 0.0], [1.0, 2.0]]).is_err());
    }
}
