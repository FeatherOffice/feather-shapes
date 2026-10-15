//! Lê o atributo `d` de um caminho SVG e o transforma em linhas poligonais (curvas e arcos
//! viram pequenos segmentos). É o que os ícones do Tabler precisam; o motor só desenha linhas.
//! Comandos aceitos: M L H V C S Q T A Z, maiúsculos (absolutos) e minúsculos (relativos).

use std::f32::consts::PI;

/// Uma linha contínua do desenho; `fechado` repete o primeiro ponto no fim.
#[derive(Clone, Debug, PartialEq)]
pub struct Traco {
    pub pontos: Vec<[f32; 2]>,
    pub fechado: bool,
}

struct Leitor<'a> {
    s: &'a [u8],
    i: usize,
}

impl Leitor<'_> {
    fn pular(&mut self) {
        while self.i < self.s.len() && (self.s[self.i].is_ascii_whitespace() || self.s[self.i] == b',') {
            self.i += 1;
        }
    }

    /// Há mais um número adiante (e não uma letra de comando)?
    fn tem_numero(&mut self) -> bool {
        self.pular();
        self.s.get(self.i).is_some_and(|c| c.is_ascii_digit() || matches!(c, b'-' | b'+' | b'.'))
    }

    fn numero(&mut self) -> Option<f32> {
        self.pular();
        let ini = self.i;
        if matches!(self.s.get(self.i), Some(b'-' | b'+')) {
            self.i += 1;
        }
        let mut ponto = false;
        while let Some(&c) = self.s.get(self.i) {
            if c.is_ascii_digit() {
                self.i += 1;
            } else if c == b'.' && !ponto {
                ponto = true;
                self.i += 1;
            } else {
                break;
            }
        }
        if matches!(self.s.get(self.i), Some(b'e' | b'E')) && self.s.get(self.i + 1).is_some_and(|c| c.is_ascii_digit() || matches!(c, b'-' | b'+')) {
            self.i += 2;
            while self.s.get(self.i).is_some_and(u8::is_ascii_digit) {
                self.i += 1;
            }
        }
        std::str::from_utf8(&self.s[ini..self.i]).ok()?.parse().ok()
    }

    /// Os sinalizadores do arco são um caractere só (`0` ou `1`), sem separador obrigatório.
    fn sinalizador(&mut self) -> Option<bool> {
        self.pular();
        let c = *self.s.get(self.i)?;
        (c == b'0' || c == b'1').then(|| {
            self.i += 1;
            c == b'1'
        })
    }
}

fn cubica(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], saida: &mut Vec<[f32; 2]>) {
    for k in 1..=14 {
        let t = k as f32 / 14.0;
        let u = 1.0 - t;
        let f = |a: f32, b: f32, c: f32, d: f32| u * u * u * a + 3.0 * u * u * t * b + 3.0 * u * t * t * c + t * t * t * d;
        saida.push([f(p0[0], p1[0], p2[0], p3[0]), f(p0[1], p1[1], p2[1], p3[1])]);
    }
}

fn quadratica(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2], saida: &mut Vec<[f32; 2]>) {
    for k in 1..=10 {
        let t = k as f32 / 10.0;
        let u = 1.0 - t;
        saida.push([u * u * p0[0] + 2.0 * u * t * p1[0] + t * t * p2[0], u * u * p0[1] + 2.0 * u * t * p1[1] + t * t * p2[1]]);
    }
}

/// Arco elíptico do SVG (apêndice F.6 da especificação): de `p0` a `p1`.
#[allow(clippy::too_many_arguments)]
fn arco(p0: [f32; 2], rx: f32, ry: f32, rot: f32, grande: bool, varredura: bool, p1: [f32; 2], saida: &mut Vec<[f32; 2]>) {
    let (mut rx, mut ry) = (rx.abs(), ry.abs());
    if (p0 == p1) || rx < 1e-6 || ry < 1e-6 {
        saida.push(p1);
        return;
    }
    let (s, c) = rot.to_radians().sin_cos();
    let (dx, dy) = ((p0[0] - p1[0]) / 2.0, (p0[1] - p1[1]) / 2.0);
    let (x1, y1) = (c * dx + s * dy, -s * dx + c * dy);
    let lambda = x1 * x1 / (rx * rx) + y1 * y1 / (ry * ry);
    if lambda > 1.0 {
        let k = lambda.sqrt();
        rx *= k;
        ry *= k;
    }
    let num = rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1;
    let den = rx * rx * y1 * y1 + ry * ry * x1 * x1;
    let sinal = if grande == varredura { -1.0 } else { 1.0 };
    let coef = sinal * (num / den).max(0.0).sqrt();
    let (cxl, cyl) = (coef * rx * y1 / ry, -coef * ry * x1 / rx);
    let (cx, cy) = (c * cxl - s * cyl + (p0[0] + p1[0]) / 2.0, s * cxl + c * cyl + (p0[1] + p1[1]) / 2.0);
    let angulo = |ux: f32, uy: f32, vx: f32, vy: f32| {
        (ux * vy - uy * vx).atan2(ux * vx + uy * vy)
    };
    let theta = angulo(1.0, 0.0, (x1 - cxl) / rx, (y1 - cyl) / ry);
    let mut delta = angulo((x1 - cxl) / rx, (y1 - cyl) / ry, (-x1 - cxl) / rx, (-y1 - cyl) / ry);
    if !varredura && delta > 0.0 {
        delta -= 2.0 * PI;
    } else if varredura && delta < 0.0 {
        delta += 2.0 * PI;
    }
    let passos = ((delta.abs() / (PI / 18.0)).ceil() as usize).max(2);
    for k in 1..=passos {
        let a = theta + delta * k as f32 / passos as f32;
        let (ex, ey) = (rx * a.cos(), ry * a.sin());
        saida.push([c * ex - s * ey + cx, s * ex + c * ey + cy]);
    }
    // Termina exatamente no ponto pedido (sem erro de arredondamento).
    if let Some(u) = saida.last_mut() {
        *u = p1;
    }
}

/// Os traços do caminho `d`. Entrada inválida devolve o que deu para ler até ali.
pub fn tracos(d: &str) -> Vec<Traco> {
    let mut l = Leitor { s: d.as_bytes(), i: 0 };
    let mut v: Vec<Traco> = Vec::new();
    let (mut atual, mut inicio) = ([0.0_f32; 2], [0.0_f32; 2]);
    let (mut ctrl_c, mut ctrl_q): (Option<[f32; 2]>, Option<[f32; 2]>) = (None, None);
    let mut cmd = b' ';
    loop {
        l.pular();
        let Some(&c) = l.s.get(l.i) else { break };
        if c.is_ascii_alphabetic() {
            cmd = c;
            l.i += 1;
            if matches!(cmd, b'z' | b'Z') {
                if let Some(t) = v.last_mut() {
                    if t.pontos.last() != Some(&inicio) {
                        t.pontos.push(inicio);
                    }
                    t.fechado = true;
                }
                atual = inicio;
                ctrl_c = None;
                ctrl_q = None;
                continue;
            }
        } else if !l.tem_numero() {
            break;
        }
        let rel = cmd.is_ascii_lowercase();
        let base = if rel { atual } else { [0.0, 0.0] };
        let mut num = || l.numero();
        let (mut novo_c, mut novo_q) = (None, None);
        match cmd.to_ascii_uppercase() {
            b'M' => {
                let (Some(x), Some(y)) = (num(), num()) else { break };
                atual = [base[0] + x, base[1] + y];
                inicio = atual;
                v.push(Traco { pontos: vec![atual], fechado: false });
                // Pares seguintes de um `M` são retas (`L`).
                cmd = if rel { b'l' } else { b'L' };
            }
            b'L' | b'H' | b'V' | b'C' | b'S' | b'Q' | b'T' | b'A' => {
                if v.is_empty() {
                    v.push(Traco { pontos: vec![atual], fechado: false });
                }
                let t = v.last_mut().map(|t| &mut t.pontos);
                let Some(pts) = t else { break };
                match cmd.to_ascii_uppercase() {
                    b'L' => {
                        let (Some(x), Some(y)) = (num(), num()) else { break };
                        atual = [base[0] + x, base[1] + y];
                        pts.push(atual);
                    }
                    b'H' => {
                        let Some(x) = num() else { break };
                        atual[0] = if rel { atual[0] + x } else { x };
                        pts.push(atual);
                    }
                    b'V' => {
                        let Some(y) = num() else { break };
                        atual[1] = if rel { atual[1] + y } else { y };
                        pts.push(atual);
                    }
                    b'C' => {
                        let (Some(a), Some(b), Some(c2), Some(d2), Some(e), Some(f)) = (num(), num(), num(), num(), num(), num()) else { break };
                        let (p1, p2, p3) = ([base[0] + a, base[1] + b], [base[0] + c2, base[1] + d2], [base[0] + e, base[1] + f]);
                        cubica(atual, p1, p2, p3, pts);
                        novo_c = Some(p2);
                        atual = p3;
                    }
                    b'S' => {
                        let (Some(c2), Some(d2), Some(e), Some(f)) = (num(), num(), num(), num()) else { break };
                        let p1 = ctrl_c.map_or(atual, |k| [2.0 * atual[0] - k[0], 2.0 * atual[1] - k[1]]);
                        let (p2, p3) = ([base[0] + c2, base[1] + d2], [base[0] + e, base[1] + f]);
                        cubica(atual, p1, p2, p3, pts);
                        novo_c = Some(p2);
                        atual = p3;
                    }
                    b'Q' => {
                        let (Some(a), Some(b), Some(e), Some(f)) = (num(), num(), num(), num()) else { break };
                        let (p1, p2) = ([base[0] + a, base[1] + b], [base[0] + e, base[1] + f]);
                        quadratica(atual, p1, p2, pts);
                        novo_q = Some(p1);
                        atual = p2;
                    }
                    b'T' => {
                        let (Some(e), Some(f)) = (num(), num()) else { break };
                        let p1 = ctrl_q.map_or(atual, |k| [2.0 * atual[0] - k[0], 2.0 * atual[1] - k[1]]);
                        let p2 = [base[0] + e, base[1] + f];
                        quadratica(atual, p1, p2, pts);
                        novo_q = Some(p1);
                        atual = p2;
                    }
                    _ => {
                        let (Some(rx), Some(ry), Some(rot)) = (num(), num(), num()) else { break };
                        let (Some(grande), Some(varre)) = (l.sinalizador(), l.sinalizador()) else { break };
                        let (Some(x), Some(y)) = (l.numero(), l.numero()) else { break };
                        let destino = [base[0] + x, base[1] + y];
                        arco(atual, rx, ry, rot, grande, varre, destino, pts);
                        atual = destino;
                    }
                }
            }
            _ => break,
        }
        ctrl_c = novo_c;
        ctrl_q = novo_q;
    }
    v.retain(|t| t.pontos.len() > 1);
    v
}

#[cfg(test)]
mod testes {
    use super::*;

    fn caixa(t: &[Traco]) -> [f32; 4] {
        let mut c = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
        for p in t.iter().flat_map(|t| &t.pontos) {
            c = [c[0].min(p[0]), c[1].min(p[1]), c[2].max(p[0]), c[3].max(p[1])];
        }
        c
    }

    #[test]
    fn retas_relativas_e_fechamento() {
        let t = tracos("M5 12l-2 0l9 -9l9 9l-2 0");
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].pontos, vec![[5.0, 12.0], [3.0, 12.0], [12.0, 3.0], [21.0, 12.0], [19.0, 12.0]]);
        let q = tracos("M0 0h10v10h-10z");
        assert!(q[0].fechado && q[0].pontos.first() == q[0].pontos.last());
        assert_eq!(caixa(&q), [0.0, 0.0, 10.0, 10.0]);
    }

    #[test]
    fn circulo_por_dois_arcos() {
        // O círculo que o Tabler usa: move relativo e dois arcos de meia volta.
        let t = tracos("M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0");
        let c = caixa(&t);
        for (a, b) in c.iter().zip([3.0, 3.0, 21.0, 21.0]) {
            assert!((a - b).abs() < 0.05, "{c:?}");
        }
    }

    #[test]
    fn arco_com_sinalizadores_colados_e_curvas() {
        let t = tracos("M5 12v7a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-7");
        assert_eq!(t[0].pontos.last(), Some(&[19.0, 12.0]));
        let c = tracos("M0 0c2 0 4 2 4 4s2 4 4 4q2 0 2 2t2 2");
        assert_eq!(c[0].pontos.last(), Some(&[12.0, 12.0]));
        assert!(tracos("lixo").is_empty());
    }
}
