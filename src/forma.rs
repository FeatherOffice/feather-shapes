//! O modelo neutro de forma (`tipo + ajustes + aparência + texto`) e o contrato com os
//! aplicativos: a geometria no retângulo pedido, a `Cena` do feather-draw (tela, SVG e PDF)
//! e o teste de acerto. A biblioteca não conhece slide, célula nem parágrafo: o aplicativo
//! guarda a [`Forma`] no seu elemento e decide a âncora.

use feather_draw::{Caminho, Cena, Comando, Figura, Ponto, Retangulo, Transformacao};

use crate::aparencia::Aparencia;
use crate::tipo::TipoForma;

/// Uma forma pronta para desenhar.
#[derive(Clone, Debug, PartialEq)]
pub struct Forma {
    pub tipo: TipoForma,
    /// Ajustes da geometria (as alças `adj` do DrawingML). Hoje leem ajustes o retângulo
    /// (4 raios relativos ao menor lado, um por canto), o retângulo arredondado
    /// (`[topo, base]`, padrão 0,16) e o setor (`[de, até]` em graus).
    pub ajustes: Vec<f32>,
    pub aparencia: Aparencia,
    /// Texto dentro da forma; o aplicativo o desenha na [`Forma::area_texto`].
    pub texto: Option<String>,
    /// Giro em graus, no sentido horário da tela, em torno do centro da caixa.
    pub rotacao: f32,
}

impl Forma {
    /// Forma do tipo pedido com a aparência padrão, sem ajustes, texto ou giro.
    pub fn nova(tipo: TipoForma) -> Self {
        Self { tipo, ajustes: Vec::new(), aparencia: Aparencia::default(), texto: None, rotacao: 0.0 }
    }

    /// A geometria da forma na caixa `r` (sem giro).
    pub fn geometria(&self, r: Retangulo) -> Option<Geometria> {
        geometria(self.tipo, &self.ajustes, r)
    }

    /// As figuras do feather-draw: a sombra (se houver), o corpo preenchido e contornado e os
    /// detalhes só de traço, já girados.
    pub fn figuras(&self, r: Retangulo) -> Vec<Figura> {
        let Some(g) = self.geometria(r) else { return Vec::new() };
        let giro = giro(r, self.rotacao);
        let mut saida = Vec::new();
        let preenchida = g.preenchida.transformar(giro);
        if !preenchida.comandos.is_empty() {
            if let Some(estilo) = self.aparencia.estilo_sombra() {
                let deslocar = Transformacao::transladar(self.aparencia.sombra.dx, self.aparencia.sombra.dy);
                saida.push(Figura::Caminho { caminho: preenchida.transformar(deslocar), estilo });
            }
            saida.push(Figura::Caminho { caminho: preenchida, estilo: self.aparencia.estilo_vetor(r) });
        }
        if !g.tracos.comandos.is_empty() {
            saida.push(Figura::Caminho { caminho: g.tracos.transformar(giro), estilo: self.aparencia.estilo_traco() });
        }
        saida
    }

    /// Uma `Cena` com a forma na caixa `r`. A cena vai da origem até o canto da caixa.
    pub fn cena(&self, r: Retangulo) -> Cena {
        let mut cena = Cena::nova((r.x + r.largura).max(0.0), (r.y + r.altura).max(0.0));
        for f in self.figuras(r) {
            cena.adicionar(f);
        }
        cena
    }

    /// A área onde o texto da forma é escrito (hoje a própria caixa).
    pub fn area_texto(&self, r: Retangulo) -> Retangulo {
        r
    }

    /// O ponto (`x`, `y`) acerta a forma? Dentro do preenchimento (furos não contam) ou a até
    /// `tolerancia` + meia espessura do contorno ou de um traço aberto.
    pub fn acerto(&self, r: Retangulo, x: f32, y: f32, tolerancia: f32) -> bool {
        let Some(g) = self.geometria(r) else { return false };
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        // Desfaz o giro no ponto em vez de girar a geometria.
        let p = giro(r, -self.rotacao).aplicar(Ponto::novo(x, y));
        let folga = tolerancia.max(0.0) + self.aparencia.espessura.max(0.0) / 2.0;
        let regioes = g.preenchida.aproximar(0.15);
        if !self.tipo.so_traco() && dentro(&regioes, p) {
            return true;
        }
        regioes.iter().chain(g.tracos.aproximar(0.15).iter())
            .any(|s| distancia_subcaminho(&s.pontos, s.fechado, p) <= folga)
    }
}

/// A geometria de uma forma: regiões preenchidas (furos com orientação oposta, regra
/// não-zero) e detalhes abertos que só levam traço.
#[derive(Clone, Debug, PartialEq)]
pub struct Geometria {
    pub preenchida: Caminho,
    pub tracos: Caminho,
}

impl Geometria {
    fn vazia() -> Self {
        Self { preenchida: Caminho::novo(), tracos: Caminho::novo() }
    }
}

fn poligono(c: &mut Caminho, pontos: &[[f32; 2]], fechar: bool) {
    let mut it = pontos.iter();
    let Some(p) = it.next() else { return };
    c.adicionar(Comando::Mover(Ponto::novo(p[0], p[1])));
    for p in it {
        c.adicionar(Comando::Linha(Ponto::novo(p[0], p[1])));
    }
    if fechar {
        c.adicionar(Comando::Fechar);
    }
}

fn giro(r: Retangulo, graus: f32) -> Transformacao {
    if graus == 0.0 || !graus.is_finite() {
        return Transformacao::identidade();
    }
    let (cx, cy) = (r.x + r.largura / 2.0, r.y + r.altura / 2.0);
    Transformacao::transladar(cx, cy)
        .compor(Transformacao::girar(graus.to_radians()).compor(Transformacao::transladar(-cx, -cy)))
}

/// A geometria do tipo com os ajustes, na caixa `r`. `None` para caixa inválida (não finita
/// ou sem área, exceto a linha) ou tipo sem desenho.
pub fn geometria(tipo: TipoForma, ajustes: &[f32], r: Retangulo) -> Option<Geometria> {
    let (x, y, w, h) = (r.x, r.y, r.largura, r.altura);
    if [x, y, w, h].iter().any(|n| !n.is_finite()) {
        return None;
    }
    let mut g = Geometria::vazia();
    if tipo == TipoForma::Linha {
        poligono(&mut g.tracos, &[[x, y], [x + w, y + h]], false);
        return Some(g);
    }
    if w <= 0.0 || h <= 0.0 {
        return None;
    }
    if tipo == TipoForma::CaixaTexto {
        poligono(&mut g.preenchida, &[[x, y], [x + w, y], [x + w, y + h], [x, y + h]], true);
        return Some(g);
    }
    if let Some(id) = tipo.id_galeria() {
        let texto = crate::galeria::geometria(id).ok()?;
        let (par, segmentos) = crate::caminho::decodificar_svg1(&texto, [x, y, w, h]).ok()?;
        let caminho = crate::caminho::para_caminho(par, &segmentos);
        if tipo.so_traco() { g.tracos = caminho } else { g.preenchida = caminho }
        return Some(g);
    }
    if let Some(d) = crate::basicas::desenho(tipo, x, y, w, h) {
        for c in &d.contornos { poligono(&mut g.preenchida, c, true); }
        for t in &d.tracos { poligono(&mut g.tracos, t, false); }
        return Some(g);
    }
    let regioes = crate::booleanas::regioes(tipo, ajustes, f64::from(x), f64::from(y), f64::from(w), f64::from(h)).ok()??;
    for anel in &regioes {
        let pontos: Vec<[f32; 2]> = anel.iter().map(|p| [p[0] as f32, p[1] as f32]).collect();
        poligono(&mut g.preenchida, &pontos, true);
    }
    Some(g)
}

/// A geometria normalizada: a mesma forma na caixa 0 a 1.
pub fn geometria_normalizada(tipo: TipoForma, ajustes: &[f32]) -> Option<Geometria> {
    geometria(tipo, ajustes, Retangulo::novo(0.0, 0.0, 1.0, 1.0))
}

/// Regra não-zero sobre os subcaminhos aproximados (abertos são fechados implicitamente).
fn dentro(subcaminhos: &[feather_draw::Subcaminho], p: Ponto) -> bool {
    let mut voltas = 0i32;
    for s in subcaminhos {
        let n = s.pontos.len();
        if n < 3 { continue; }
        for i in 0..n {
            let a = s.pontos[i];
            let b = s.pontos[(i + 1) % n];
            let lado = (b.x - a.x) * (p.y - a.y) - (p.x - a.x) * (b.y - a.y);
            if a.y <= p.y {
                if b.y > p.y && lado > 0.0 { voltas += 1; }
            } else if b.y <= p.y && lado < 0.0 {
                voltas -= 1;
            }
        }
    }
    voltas != 0
}

fn distancia_segmento(a: Ponto, b: Ponto, p: Ponto) -> f32 {
    let (vx, vy) = (b.x - a.x, b.y - a.y);
    let comprimento = vx * vx + vy * vy;
    let t = if comprimento > 0.0 { (((p.x - a.x) * vx + (p.y - a.y) * vy) / comprimento).clamp(0.0, 1.0) } else { 0.0 };
    (p.x - (a.x + vx * t)).hypot(p.y - (a.y + vy * t))
}

fn distancia_subcaminho(pontos: &[Ponto], fechado: bool, p: Ponto) -> f32 {
    let mut d = pontos.windows(2).map(|w| distancia_segmento(w[0], w[1], p)).fold(f32::INFINITY, f32::min);
    if fechado {
        if let (Some(&a), Some(&b)) = (pontos.last(), pontos.first()) {
            d = d.min(distancia_segmento(a, b, p));
        }
    }
    if pontos.len() == 1 {
        d = (pontos[0].x - p.x).hypot(pontos[0].y - p.y);
    }
    d
}

#[cfg(test)]
mod testes {
    use super::*;

    fn caixa() -> Retangulo {
        Retangulo::novo(10.0, 20.0, 200.0, 100.0)
    }

    #[test]
    fn todo_o_catalogo_tem_geometria_finita_dentro_da_caixa() {
        for &t in TipoForma::TODOS {
            let g = geometria(t, &[], caixa()).unwrap_or_else(|| panic!("{} sem geometria", t.id()));
            assert!(!g.preenchida.comandos.is_empty() || !g.tracos.comandos.is_empty(), "{}", t.id());
            for s in g.preenchida.aproximar(0.1).iter().chain(g.tracos.aproximar(0.1).iter()) {
                for p in &s.pontos {
                    assert!(p.x.is_finite() && p.y.is_finite(), "{}", t.id());
                    assert!((5.0..=215.0).contains(&p.x) && (15.0..=125.0).contains(&p.y), "{} fora: {p:?}", t.id());
                }
            }
        }
        assert!(geometria(TipoForma::Retangulo, &[], Retangulo::novo(0.0, 0.0, 0.0, 10.0)).is_none());
        assert!(geometria(TipoForma::Retangulo, &[], Retangulo::novo(f32::NAN, 0.0, 1.0, 1.0)).is_none());
    }

    #[test]
    fn cena_e_deterministica_e_gera_svg() {
        for &t in TipoForma::TODOS {
            let mut f = Forma::nova(t);
            f.aparencia.sombra.cor = "#00000040".into();
            f.aparencia.sombra.dx = 3.0;
            f.rotacao = 15.0;
            let a = f.cena(caixa());
            assert_eq!(a, f.cena(caixa()), "{}", t.id());
            assert!(!a.figuras.is_empty(), "{}", t.id());
            let svg1 = feather_draw::exportar_svg(&a).unwrap();
            assert_eq!(svg1, feather_draw::exportar_svg(&f.cena(caixa())).unwrap());
        }
    }

    #[test]
    fn acerto_respeita_furos_giro_e_tracos() {
        let r = Retangulo::novo(0.0, 0.0, 200.0, 200.0);
        let anel = Forma::nova(TipoForma::Anel);
        assert!(!anel.acerto(r, 100.0, 100.0, 0.0), "furo do anel");
        assert!(anel.acerto(r, 100.0, 15.0, 0.0));
        let ret = Forma::nova(TipoForma::Retangulo);
        assert!(ret.acerto(r, 100.0, 100.0, 0.0));
        assert!(!ret.acerto(r, 260.0, 100.0, 3.0));
        let mut fino = Forma::nova(TipoForma::Retangulo);
        fino.rotacao = 45.0;
        let r2 = Retangulo::novo(0.0, 90.0, 200.0, 20.0);
        assert!(fino.acerto(r2, 100.0, 100.0, 0.0));
        assert!(!fino.acerto(r2, 5.0, 100.0, 0.0), "o canto saiu do lugar com o giro");
        let linha = Forma::nova(TipoForma::Linha);
        assert!(linha.acerto(r, 101.0, 100.0, 2.0));
        assert!(!linha.acerto(r, 150.0, 50.0, 2.0));
        let arco = Forma::nova(TipoForma::Arco);
        assert!(!arco.acerto(r, 100.0, 100.0, 1.0));
    }

    #[test]
    fn ajustes_mudam_o_setor() {
        let a = geometria(TipoForma::Setor, &[0.0, 90.0], caixa()).unwrap();
        let b = geometria(TipoForma::Setor, &[0.0, 180.0], caixa()).unwrap();
        assert_ne!(a, b);
        assert!(geometria_normalizada(TipoForma::Coracao, &[]).is_some());
    }
}
