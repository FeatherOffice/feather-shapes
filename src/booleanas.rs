//! Operações booleanas de formas vetoriais (união, interseção, subtração, combinação e
//! fragmentação), sem rasterização e sem dependências externas. Trabalha em `f64` sobre
//! anéis poligonais: externos com área positiva, furos com área negativa.

use std::collections::{HashMap, HashSet};

use crate::tipo::TipoForma;

/// Ponto em `f64`: o recorte precisa da precisão extra.
pub type Ponto = [f64; 2];
/// Anéis de uma região: externos com área positiva, furos com área negativa.
pub type Contornos = Vec<Vec<Ponto>>;
type Chave = (i64, i64);

const PRECISAO: f64 = 0.00001;
const ESCALA_CHAVE: f64 = 100_000.0;
/// Quantas formas uma operação aceita de uma vez.
pub const MAX_FORMAS: usize = 16;

/// Operação disponível em Inserir > Organizar > Mesclar formas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperacaoBooleana {
    Uniao,
    Intersecao,
    Subtracao,
    Combinacao,
    Fragmentacao,
}

fn chave(p: Ponto) -> Chave {
    ((p[0] * ESCALA_CHAVE).round() as i64, (p[1] * ESCALA_CHAVE).round() as i64)
}

fn cruz(a: Ponto, b: Ponto) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

fn sub(a: Ponto, b: Ponto) -> Ponto {
    [a[0] - b[0], a[1] - b[1]]
}

fn interp(a: Ponto, b: Ponto, t: f64) -> Ponto {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
}

/// Área com sinal do anel.
pub fn area(p: &[Ponto]) -> f64 {
    if p.len() < 3 { return 0.0; }
    p.iter().zip(p.iter().cycle().skip(1)).take(p.len())
        .map(|(a, b)| cruz(*a, *b)).sum::<f64>() * 0.5
}

/// O ponto está dentro do anel (par/ímpar)?
pub fn contem_poligono(p: Ponto, pol: &[Ponto]) -> bool {
    if pol.len() < 3 { return false; }
    let mut dentro = false;
    for (a, b) in pol.iter().zip(pol.iter().cycle().skip(1)).take(pol.len()) {
        if (a[1] > p[1]) != (b[1] > p[1]) {
            let x = a[0] + (p[1] - a[1]) * (b[0] - a[0]) / (b[1] - a[1]);
            if x > p[0] { dentro = !dentro; }
        }
    }
    dentro
}

/// Orientação dos anéis: contorno externo positivo; furos, negativos.
pub fn contem(p: Ponto, pol: &Contornos) -> bool {
    let soma: i32 = pol.iter().filter(|c| contem_poligono(p, c))
        .map(|c| if area(c) > 0.0 { 1 } else { -1 }).sum();
    soma != 0
}

/// Retângulo com raio por canto (`[sup. esq., sup. dir., inf. dir., inf. esq.]`).
pub fn arredondado(x: f64, y: f64, w: f64, h: f64, raios: [f64; 4]) -> Vec<Ponto> {
    let mut p = Vec::new();
    let cantos = [
        (x + w - raios[1], y + raios[1], -90.0_f64),
        (x + w - raios[2], y + h - raios[2], 0.0),
        (x + raios[3], y + h - raios[3], 90.0),
        (x + raios[0], y + raios[0], 180.0),
    ];
    for (i, (cx, cy, ini)) in cantos.into_iter().enumerate() {
        for j in 0..=12 {
            let a = (ini + (j as f64 * 90.0 / 12.0)).to_radians();
            p.push([cx + raios[(i + 1) % 4] * a.cos(), cy + raios[(i + 1) % 4] * a.sin()]);
        }
    }
    p
}

#[derive(Clone, Copy)]
struct Segmento {
    a: Ponto,
    b: Ponto,
}

fn arestas(pol: &Contornos) -> Vec<Segmento> {
    pol.iter().flat_map(|c| c.iter().zip(c.iter().cycle().skip(1))
        .take(c.len()).map(|(a, b)| Segmento { a: *a, b: *b })).filter(|s|
            (s.a[0] - s.b[0]).abs() > PRECISAO || (s.a[1] - s.b[1]).abs() > PRECISAO
        ).collect()
}

fn parametro(a: Ponto, b: Ponto, p: Ponto) -> Option<f64> {
    let d = sub(b, a);
    let den = d[0] * d[0] + d[1] * d[1];
    if den < PRECISAO * PRECISAO { return None; }
    let t = ((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / den;
    let q = interp(a, b, t);
    ((q[0] - p[0]).abs() < PRECISAO && (q[1] - p[1]).abs() < PRECISAO && (-1e-8..=1.0 + 1e-8).contains(&t))
        .then_some(t.clamp(0.0, 1.0))
}

fn separar_arestas(s: &[Segmento]) -> Vec<Segmento> {
    let mut ts = vec![vec![0.0, 1.0]; s.len()];
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            let (a, b) = (s[i], s[j]);
            let r = sub(a.b, a.a);
            let t = sub(b.b, b.a);
            let q = sub(b.a, a.a);
            let det = cruz(r, t);
            if det.abs() > 1e-10 {
                let u = cruz(q, t) / det;
                let v = cruz(q, r) / det;
                if (-1e-8..=1.0 + 1e-8).contains(&u) && (-1e-8..=1.0 + 1e-8).contains(&v) {
                    ts[i].push(u.clamp(0.0, 1.0));
                    ts[j].push(v.clamp(0.0, 1.0));
                }
            } else if cruz(q, r).abs() <= PRECISAO {
                for p in [b.a, b.b] { if let Some(t) = parametro(a.a, a.b, p) { ts[i].push(t); } }
                for p in [a.a, a.b] { if let Some(t) = parametro(b.a, b.b, p) { ts[j].push(t); } }
            }
        }
    }
    let mut partes = Vec::new();
    for (e, cuts) in s.iter().zip(&mut ts) {
        cuts.sort_by(f64::total_cmp);
        cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
        for janela in cuts.windows(2) {
            let a = interp(e.a, e.b, janela[0]);
            let b = interp(e.a, e.b, janela[1]);
            if chave(a) != chave(b) { partes.push(Segmento { a, b }); }
        }
    }
    partes
}

fn pertence(a: bool, b: bool, op: OperacaoBooleana) -> bool {
    match op {
        OperacaoBooleana::Uniao | OperacaoBooleana::Fragmentacao => a || b,
        OperacaoBooleana::Intersecao => a && b,
        OperacaoBooleana::Subtracao => a && !b,
        OperacaoBooleana::Combinacao => a != b,
    }
}

fn remontar_aneis(arestas: &[Segmento]) -> Result<Contornos, String> {
    let mut saidas: HashMap<Chave, Vec<usize>> = HashMap::new();
    for (i, e) in arestas.iter().enumerate() { saidas.entry(chave(e.a)).or_default().push(i); }
    let mut usado = vec![false; arestas.len()];
    let mut resultado = Vec::new();
    for inicio in 0..arestas.len() {
        if usado[inicio] { continue; }
        let primeiro = chave(arestas[inicio].a);
        let mut indice = inicio;
        let mut anel = Vec::new();
        for _ in 0..=arestas.len() {
            if usado[indice] { return Err("A malha vetorial criou uma conexão ambígua.".into()); }
            usado[indice] = true;
            let e = arestas[indice];
            anel.push(e.a);
            let fim = chave(e.b);
            if fim == primeiro { break; }
            let candidatos = saidas.get(&fim).ok_or("A operação produziu uma borda aberta.")?;
            let dir = sub(e.b, e.a);
            let proximo = candidatos.iter().copied().filter(|i| !usado[*i])
                .min_by(|&i, &j| {
                    let di = sub(arestas[i].b, arestas[i].a);
                    let dj = sub(arestas[j].b, arestas[j].a);
                    let ai = cruz(dir, di).atan2(dir[0]*di[0] + dir[1]*di[1]);
                    let aj = cruz(dir, dj).atan2(dir[0]*dj[0] + dir[1]*dj[1]);
                    ai.total_cmp(&aj)
                });
            indice = proximo.ok_or("A operação produziu um contorno incompleto.")?;
        }
        if anel.len() >= 3 && area(&anel).abs() > PRECISAO { resultado.push(anel); }
    }
    Ok(resultado)
}

/// Recorta dois conjuntos de contornos fechados. Preserva componentes e vazios.
pub fn calcular(a: &Contornos, b: &Contornos, op: OperacaoBooleana) -> Result<Contornos, String> {
    if a.is_empty() {
        return Ok(if op == OperacaoBooleana::Uniao || op == OperacaoBooleana::Combinacao { b.clone() } else { Vec::new() });
    }
    if b.is_empty() {
        return Ok(if op == OperacaoBooleana::Intersecao { Vec::new() } else { a.clone() });
    }
    let mut todas = arestas(a);
    todas.extend(arestas(b));
    let partes = separar_arestas(&todas);
    let mut bordas = Vec::new();
    let mut inseridas = HashSet::new();
    for e in partes {
        let d = sub(e.b, e.a);
        let comprimento = (d[0]*d[0] + d[1]*d[1]).sqrt();
        if comprimento < PRECISAO { continue; }
        let delta = (comprimento * 0.0001).clamp(0.000001, 0.0001);
        let normal = [-d[1] / comprimento * delta, d[0] / comprimento * delta];
        let meio = interp(e.a, e.b, 0.5);
        let esquerdo = [meio[0] + normal[0], meio[1] + normal[1]];
        let direito = [meio[0] - normal[0], meio[1] - normal[1]];
        let le = pertence(contem(esquerdo, a), contem(esquerdo, b), op);
        let ld = pertence(contem(direito, a), contem(direito, b), op);
        if le == ld { continue; }
        let orientada = if le { e } else { Segmento { a: e.b, b: e.a } };
        if inseridas.insert((chave(orientada.a), chave(orientada.b))) {
            bordas.push(orientada);
        }
    }
    remontar_aneis(&bordas)
}

/// Separa os anéis em regiões independentes, cada externa com os seus furos.
pub fn componentes(contornos: Contornos) -> Vec<Contornos> {
    let mut externos: Vec<Contornos> = contornos.iter().filter(|c| area(c) > PRECISAO).map(|c| vec![c.clone()]).collect();
    let internos = contornos.into_iter().filter(|c| area(c) < -PRECISAO);
    for furo in internos {
        if let Some(pos) = externos.iter().position(|ext| contem_poligono(furo[0], &ext[0])) {
            externos[pos].push(furo);
        }
    }
    externos
}


/// As regiões de uma forma do catálogo no retângulo, em `f64`, antes de girar. `ajustes`
/// segue o aplicativo: no retângulo, 4 raios relativos ao menor lado (um por canto); no
/// arredondado, `[topo, base]` (padrão 0,16); no setor, `[de, até]` em graus.
/// `Ok(None)`: o tipo não é uma forma vetorial mesclável.
pub fn regioes(tipo: TipoForma, ajustes: &[f32], x: f64, y: f64, w: f64, h: f64) -> Result<Option<Contornos>, String> {
    Ok(Some(match tipo {
        tipo if crate::basicas::suporta(tipo) => {
            let d = crate::basicas::desenho(tipo, x as f32, y as f32, w as f32, h as f32)
                .ok_or("A forma não tem uma região fechada.")?;
            if d.contornos.is_empty() { return Err("Arcos abertos não podem ser mesclados como regiões preenchidas.".into()); }
            d.contornos.into_iter().map(|c| c.into_iter().map(|[px, py]| [f64::from(px), f64::from(py)]).collect()).collect()
        },
        TipoForma::Retangulo => {
            if ajustes.len() == 4 {
                let lim = w.min(h) * 0.5;
                let r = std::array::from_fn(|i| (f64::from(ajustes[i]) * w.min(h)).clamp(0.0, lim));
                vec![arredondado(x, y, w, h, r)]
            } else {
                vec![vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]]]
            }
        }
        TipoForma::RetanguloArredondado => {
            let topo = f64::from(ajustes.first().copied().unwrap_or(0.16)).clamp(0.0, 0.5) * w.min(h);
            let baixo = f64::from(ajustes.get(1).copied().unwrap_or(0.16)).clamp(0.0, 0.5) * w.min(h);
            vec![arredondado(x, y, w, h, [topo, topo, baixo, baixo])]
        }
        TipoForma::Elipse | TipoForma::Setor => {
            let mut c = Vec::new();
            let (de, ate, n) = if tipo == TipoForma::Setor {
                let de = f64::from(ajustes.first().copied().unwrap_or(0.0));
                let mut ate = f64::from(ajustes.get(1).copied().unwrap_or(270.0));
                if ate <= de { ate += 360.0; }
                c.push([x + w * 0.5, y + h * 0.5]);
                (de, ate, 72)
            } else { (0.0, 360.0, 128) };
            for i in 0..=n {
                let a = (de + (ate - de) * (i as f64 / n as f64)).to_radians();
                c.push([x + w * (0.5 + 0.5 * a.cos()), y + h * (0.5 + 0.5 * a.sin())]);
            }
            vec![c]
        }
        TipoForma::Triangulo => vec![vec![[x + w * 0.5, y], [x + w, y + h], [x, y + h]]],
        TipoForma::SetaDireita => {
            let t = (w * 0.4).min(h);
            vec![vec![
                [x, y + h * 0.3], [x + w - t, y + h * 0.3],
                [x + w - t, y], [x + w, y + h * 0.5],
                [x + w - t, y + h], [x + w - t, y + h * 0.7],
                [x, y + h * 0.7],
            ]]
        }
        outro => match crate::poligonos::pontos(outro, x as f32, y as f32, w as f32, h as f32) {
            Some(pontos) => vec![pontos.iter().map(|[a, b]| [f64::from(*a), f64::from(*b)]).collect()],
            None => return Ok(None),
        },
    }))
}

/// Gira os anéis `graus` em torno de `centro`.
pub fn girar(pol: &mut Contornos, centro: Ponto, graus: f64) {
    let (seno, cosseno) = graus.to_radians().sin_cos();
    for contorno in pol {
        for p in contorno {
            let [dx, dy] = sub(*p, centro);
            *p = [centro[0] + dx * cosseno - dy * seno, centro[1] + dx * seno + dy * cosseno];
        }
    }
}

/// Deixa todos os anéis com área positiva (formas sem furo vindas do catálogo).
pub fn orientar_positivo(pol: &mut Contornos) {
    for c in pol {
        if area(c) < 0.0 { c.reverse(); }
    }
}

/// Aplica a operação em sequência sobre as geometrias (a primeira é a base) e devolve as
/// regiões resultantes, cada uma com os seus furos. Vazio: a operação não produz área.
pub fn mesclar(geometrias: &[Contornos], op: OperacaoBooleana) -> Result<Vec<Contornos>, String> {
    let Some(primeira) = geometrias.first() else { return Ok(Vec::new()) };
    let mut partes: Vec<Contornos>;
    if op == OperacaoBooleana::Fragmentacao {
        let mut cobertura = primeira.clone();
        partes = vec![cobertura.clone()];
        for p in &geometrias[1..] {
            let mut proximas = Vec::new();
            for regiao in partes {
                let so_antes = calcular(&regiao, p, OperacaoBooleana::Subtracao)?;
                let em_ambos = calcular(&regiao, p, OperacaoBooleana::Intersecao)?;
                if !so_antes.is_empty() { proximas.push(so_antes); }
                if !em_ambos.is_empty() { proximas.push(em_ambos); }
            }
            let so_novo = calcular(p, &cobertura, OperacaoBooleana::Subtracao)?;
            if !so_novo.is_empty() { proximas.push(so_novo); }
            cobertura = calcular(&cobertura, p, OperacaoBooleana::Uniao)?;
            partes = proximas;
        }
    } else {
        let mut atual = primeira.clone();
        for p in &geometrias[1..] { atual = calcular(&atual, p, op)?; }
        partes = if atual.is_empty() { Vec::new() } else { vec![atual] };
    }
    Ok(partes.into_iter().flat_map(componentes).collect())
}

#[cfg(test)]
mod testes {
    use super::*;

    fn ret(x: f64, y: f64, w: f64, h: f64) -> Contornos {
        vec![vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]]]
    }

    fn superficie(p: &Contornos) -> f64 {
        p.iter().map(|a| area(a)).sum::<f64>()
    }

    #[test]
    fn todas_as_cinco_operacoes_sobrepostas() {
        let a = ret(0.0, 0.0, 100.0, 100.0);
        let b = ret(50.0, 0.0, 100.0, 100.0);
        for (op, esperado) in [
            (OperacaoBooleana::Uniao, 15000.0),
            (OperacaoBooleana::Intersecao, 5000.0),
            (OperacaoBooleana::Subtracao, 5000.0),
            (OperacaoBooleana::Combinacao, 10000.0),
        ] {
            let p = calcular(&a, &b, op).unwrap();
            assert!((superficie(&p) - esperado).abs() < 0.1, "{op:?}: {p:?}");
        }
        let frag = mesclar(&[a, b], OperacaoBooleana::Fragmentacao).unwrap();
        assert_eq!(frag.len(), 3);
        assert!((frag.iter().map(superficie).sum::<f64>() - 15000.0).abs() < 0.1);
    }

    #[test]
    fn furos_viram_componente_com_buraco() {
        let a = ret(0.0, 0.0, 100.0, 100.0);
        let b = ret(25.0, 25.0, 50.0, 50.0);
        let p = calcular(&a, &b, OperacaoBooleana::Subtracao).unwrap();
        assert_eq!(p.len(), 2);
        assert!((superficie(&p) - 7500.0).abs() < 0.1);
        assert!(contem([10.0, 10.0], &p));
        assert!(!contem([50.0, 50.0], &p), "furo não é superfície");
        let c = componentes(p);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 2);
    }

    #[test]
    fn intersecao_vazia_e_regioes_do_catalogo() {
        let a = ret(5.0, 5.0, 50.0, 50.0);
        let b = regioes(TipoForma::Elipse, &[], 200.0, 200.0, 40.0, 40.0).unwrap().unwrap();
        assert!(mesclar(&[a, b], OperacaoBooleana::Intersecao).unwrap().is_empty());
        assert!(regioes(TipoForma::Arco, &[], 0.0, 0.0, 10.0, 10.0).is_err());
        assert!(regioes(TipoForma::CaixaTexto, &[], 0.0, 0.0, 10.0, 10.0).unwrap().is_none());
        let mut r = regioes(TipoForma::Retangulo, &[], 0.0, 0.0, 10.0, 10.0).unwrap().unwrap();
        girar(&mut r, [5.0, 5.0], 90.0);
        assert!((superficie(&r) - 100.0).abs() < 1e-6);
    }
}
