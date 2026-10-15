//! Geometria vetorial das formas básicas com desenho próprio (círculo, anel, moldura, cubo,
//! cilindro, sol, lua, coração, nuvem, estrelas…). Coordenadas na unidade do documento;
//! nenhuma dependência de interface. Curvas complexas são amostradas em pontos vetoriais
//! (não rasterizadas).

use crate::tipo::TipoForma;

type Ponto = [f32; 2];

/// Contornos preenchidos (com orientação para os furos) e traços abertos/de detalhe.
#[derive(Clone, Debug, PartialEq)]
pub struct Desenho {
    pub contornos: Vec<Vec<Ponto>>,
    pub tracos: Vec<Vec<Ponto>>,
}

fn oval(cx: f32, cy: f32, rx: f32, ry: f32, inicio: f32, fim: f32, passos: usize) -> Vec<Ponto> {
    (0..=passos).map(|i| {
        let t = inicio + (fim - inicio) * i as f32 / passos as f32;
        [cx + rx * t.cos(), cy + ry * t.sin()]
    }).collect()
}

/// Área com sinal do polígono (positiva no sentido dos contornos externos).
pub fn area(p: &[Ponto]) -> f32 {
    p.iter().zip(p.iter().cycle().skip(1)).take(p.len())
        .map(|(a, b)| a[0] * b[1] - a[1] * b[0]).sum::<f32>() * 0.5
}

fn anel_externo_e_furo(mut exterior: Vec<Ponto>, mut interior: Vec<Ponto>) -> Vec<Vec<Ponto>> {
    if area(&exterior) < 0.0 { exterior.reverse(); }
    if area(&interior) > 0.0 { interior.reverse(); }
    vec![exterior, interior]
}

fn estrela(pontas: usize) -> Vec<Ponto> {
    (0..pontas * 2).map(|i| {
        let ang = -std::f32::consts::FRAC_PI_2
            + std::f32::consts::TAU * i as f32 / (pontas * 2) as f32;
        let r = if i % 2 == 0 { 0.49 } else { 0.22 };
        [0.5 + r * ang.cos(), 0.5 + r * ang.sin()]
    }).collect()
}

fn coracao() -> Vec<Ponto> {
    (0..128).map(|i| {
        let t = i as f32 * std::f32::consts::TAU / 128.0;
        let xx = 16.0 * t.sin().powi(3);
        let yy = 13.0 * t.cos() - 5.0 * (2.0 * t).cos()
            - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();
        [0.5 + xx / 40.0, 0.52 - yy / 39.0]
    }).collect()
}

fn nuvem() -> Vec<Ponto> {
    // Silhueta Catmull-Rom fechada: a interpolação mantém todos os lóbulos vetoriais.
    let controles: [Ponto; 13] = [
        [0.14, 0.75], [0.07, 0.62], [0.15, 0.48],
        [0.28, 0.44], [0.34, 0.25], [0.50, 0.17],
        [0.67, 0.28], [0.73, 0.39], [0.87, 0.41],
        [0.95, 0.55], [0.88, 0.73], [0.69, 0.81],
        [0.37, 0.80],
    ];
    let mut pontos = Vec::with_capacity(controles.len() * 10);
    let n = controles.len();
    for i in 0..n {
        let a = controles[(i + n - 1) % n];
        let b = controles[i];
        let c = controles[(i + 1) % n];
        let d = controles[(i + 2) % n];
        for j in 0..10 {
            let t = j as f32 / 10.0;
            let t2 = t * t;
            let t3 = t2 * t;
            let eixo = |k: usize| 0.5 * (2.0 * b[k]
                + (-a[k] + c[k]) * t
                + (2.0*a[k] - 5.0*b[k] + 4.0*c[k] - d[k]) * t2
                + (-a[k] + 3.0*b[k] - 3.0*c[k] + d[k]) * t3);
            pontos.push([eixo(0), eixo(1)]);
        }
    }
    pontos
}

/// Quais tipos têm geometria própria neste módulo.
pub fn suporta(tipo: TipoForma) -> bool {
    matches!(tipo,
        TipoForma::Circulo | TipoForma::Anel | TipoForma::Arco | TipoForma::Corda | TipoForma::Cubo
        | TipoForma::Cilindro | TipoForma::Moldura | TipoForma::CantoEmL | TipoForma::Placa
        | TipoForma::Sol | TipoForma::Lua | TipoForma::Coracao | TipoForma::Raio | TipoForma::Nuvem
        | TipoForma::Estrela4 | TipoForma::Estrela6 | TipoForma::Estrela8
        | TipoForma::Estrela10 | TipoForma::Estrela12
    )
}

/// Calcula o desenho de uma forma no retângulo. Furos possuem orientação oposta.
pub fn desenho(tipo: TipoForma, x: f32, y: f32, w: f32, h: f32) -> Option<Desenho> {
    if !suporta(tipo) || !w.is_finite() || !h.is_finite() || w <= 0.0 || h <= 0.0 {
        return None;
    }
    let tau = std::f32::consts::TAU;
    let (contornos, tracos): (Vec<Vec<Ponto>>, Vec<Vec<Ponto>>) = match tipo {
        TipoForma::Circulo => {
            let r = (w.min(h) / 2.0) / w;
            let ry = (w.min(h) / 2.0) / h;
            (vec![oval(0.5, 0.5, r, ry, 0.0, tau, 96)], vec![])
        }
        TipoForma::Anel => (
            anel_externo_e_furo(
                oval(0.5, 0.5, 0.49, 0.49, 0.0, tau, 96),
                oval(0.5, 0.5, 0.23, 0.23, 0.0, tau, 96),
            ), vec![]),
        TipoForma::Moldura => (
            anel_externo_e_furo(
                vec![[0.03,0.03],[0.97,0.03],[0.97,0.97],[0.03,0.97]],
                vec![[0.22,0.22],[0.78,0.22],[0.78,0.78],[0.22,0.78]],
            ), vec![]),
        TipoForma::Arco => (vec![], vec![oval(0.5,0.5,0.47,0.47,0.15*std::f32::consts::PI,1.15*std::f32::consts::PI,64)]),
        TipoForma::Corda => {
            let p = oval(0.5,0.5,0.47,0.47,-0.7*std::f32::consts::PI,0.7*std::f32::consts::PI,96);
            (vec![p], vec![])
        }
        TipoForma::Cubo => (
            vec![vec![[0.10,0.27],[0.50,0.04],[0.90,0.27],[0.90,0.74],[0.50,0.97],[0.10,0.74]]],
            vec![vec![[0.10,0.27],[0.50,0.51],[0.90,0.27]],
                 vec![[0.50,0.51],[0.50,0.97]]]
        ),
        TipoForma::Cilindro => {
            let mut exterior = oval(0.5,0.21,0.44,0.14,std::f32::consts::PI,tau,32);
            exterior.push([0.94,0.79]);
            exterior.extend(oval(0.5,0.79,0.44,0.14,0.0,std::f32::consts::PI,32).into_iter().skip(1));
            (vec![exterior],
             vec![oval(0.5,0.21,0.44,0.14,0.0,tau,64)])
        }
        TipoForma::CantoEmL => (vec![vec![[0.07,0.04],[0.36,0.04],[0.36,0.65],[0.95,0.65],[0.95,0.96],[0.07,0.96]]],vec![]),
        TipoForma::Placa => (vec![vec![[0.20,0.02],[0.80,0.02],[0.98,0.20],[0.98,0.80],
                              [0.80,0.98],[0.20,0.98],[0.02,0.80],[0.02,0.20]]],vec![]),
        TipoForma::Sol => (vec![estrela(12)],vec![]),
        TipoForma::Lua => {
            let mut curva = oval(0.54,0.5,0.48,0.48,
                -std::f32::consts::FRAC_PI_2,-3.0*std::f32::consts::FRAC_PI_2,64);
            let ponta_baixo = *curva.last().unwrap_or(&[0.54, 0.98]);
            let ponta_cima = *curva.first().unwrap_or(&[0.54, 0.02]);
            // A concavidade é a borda interior, não uma área transparente circular.
            curva.extend([ponta_baixo, [0.38,0.77], [0.32,0.50], [0.38,0.23], ponta_cima]);
            (vec![curva],vec![])
        }
        TipoForma::Coracao => (vec![coracao()],vec![]),
        TipoForma::Raio => (vec![vec![[0.58,0.02],[0.19,0.54],[0.45,0.54],[0.33,0.98],[0.84,0.39],[0.57,0.39]]],vec![]),
        TipoForma::Nuvem => (vec![nuvem()],vec![]),
        TipoForma::Estrela4 => (vec![estrela(4)],vec![]),
        TipoForma::Estrela6 => (vec![estrela(6)],vec![]),
        TipoForma::Estrela8 => (vec![estrela(8)],vec![]),
        TipoForma::Estrela10 => (vec![estrela(10)],vec![]),
        TipoForma::Estrela12 => (vec![estrela(12)],vec![]),
        _ => return None,
    };
    // Todos os contornos preenchidos externos têm orientação positiva; os furos negativa.
    let mut contornos = contornos;
    if !matches!(tipo, TipoForma::Anel | TipoForma::Moldura) {
        for c in &mut contornos { if area(c) < 0.0 { c.reverse(); } }
    }
    let conv = |p: Vec<Ponto>| -> Vec<Ponto> { p.into_iter().map(|[u,v]| [x + w*u, y + h*v]).collect() };
    Some(Desenho {
        contornos: contornos.into_iter().map(conv).collect(),
        tracos: tracos.into_iter().map(conv).collect(),
    })
}

/// O ponto está dentro do anel (regra par/ímpar de um polígono só)?
pub fn no_poligono(p: Ponto, anel: &[Ponto]) -> bool {
    let mut dentro = false;
    for (a,b) in anel.iter().zip(anel.iter().cycle().skip(1)).take(anel.len()) {
        if (a[1]>p[1]) != (b[1]>p[1]) {
            let ix = a[0]+(p[1]-a[1])*(b[0]-a[0])/(b[1]-a[1]);
            if p[0]<ix { dentro = !dentro; }
        }
    }
    dentro
}

/// Teste de acerto no preenchimento real (inclusive furos) ou no arco aberto. `caixa` é
/// `[x, y, largura, altura]`, `rotacao` em graus em torno do centro da caixa e `espessura`
/// a do contorno (o arco aceita no mínimo 5 unidades de distância).
pub fn contem_ponto(tipo: TipoForma, caixa: [f32; 4], rotacao: f32, espessura: f32, x: f32, y: f32) -> bool {
    let [ex, ey, largura, altura] = caixa;
    let (cx, cy) = (ex + largura*0.5, ey + altura*0.5);
    let (s,c) = (-rotacao).to_radians().sin_cos();
    let (dx,dy) = (x-cx, y-cy);
    let ponto = [cx + dx*c - dy*s, cy + dx*s + dy*c];
    let Some(d) = desenho(tipo, ex, ey, largura, altura) else { return false };
    if tipo == TipoForma::Arco {
        return d.tracos.iter().any(|traco| traco.windows(2).any(|ab| {
            let (a,b)=(ab[0],ab[1]);
            let vx=b[0]-a[0]; let vy=b[1]-a[1];
            let comprimento=vx*vx+vy*vy;
            let t=if comprimento>0.0 { ((ponto[0]-a[0])*vx+(ponto[1]-a[1])*vy)/comprimento } else { 0.0 };
            let q=[a[0]+vx*t.clamp(0.0,1.0),a[1]+vy*t.clamp(0.0,1.0)];
            (ponto[0]-q[0]).hypot(ponto[1]-q[1])<=espessura.max(5.0)
        }));
    }
    // Mesmo critério de orientação utilizado pela união/subtração; o interior de anel não seleciona.
    d.contornos.iter().filter(|p| no_poligono(ponto,p))
        .map(|p| if area(p)>0.0 {1} else {-1}).sum::<i32>()>0
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn todos_os_tipos_tem_geometria_editavel() {
        for &tipo in crate::poligonos::ADICIONAIS {
            if !suporta(tipo) { continue; }
            let d=desenho(tipo, 10.0, 20.0, 180.0, 120.0).unwrap();
            assert!(d.contornos.iter().all(|p| p.len()>=3));
            assert!(d.tracos.iter().all(|p| p.len()>=2));
            assert!(!d.contornos.is_empty()||!d.tracos.is_empty());
            assert!(d.contornos.iter().chain(&d.tracos).flatten()
                .all(|[x,y]| x.is_finite()&&y.is_finite()&&*x>=9.9&&*x<=190.1&&*y>=19.9&&*y<=140.1),"{tipo:?}");
        }
        assert!(desenho(TipoForma::Anel, 0.0, 0.0, 0.0, 10.0).is_none());
    }

    #[test]
    fn anel_e_moldura_tem_buraco_real() {
        for tipo in [TipoForma::Anel,TipoForma::Moldura] {
            let caixa = [0.0, 0.0, 200.0, 200.0];
            assert!(!contem_ponto(tipo,caixa,0.0,2.0,100.0,100.0),"{tipo:?}");
            assert!(contem_ponto(tipo,caixa,0.0,2.0,100.0,15.0),"{tipo:?}");
            let d=desenho(tipo,0.0,0.0,200.0,200.0).unwrap();
            assert!(area(&d.contornos[0])>0.0 && area(&d.contornos[1])<0.0);
        }
    }

    #[test]
    fn arco_aberto_nao_preenche() {
        let d=desenho(TipoForma::Arco,0.0,0.0,200.0,200.0).unwrap();
        assert!(d.contornos.is_empty());
        assert_eq!(d.tracos.len(),1);
        assert!(!contem_ponto(TipoForma::Arco,[0.0,0.0,200.0,200.0],0.0,2.0,100.0,100.0));
    }
}
