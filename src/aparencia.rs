//! Aparência de formas (preenchimento sólido ou gradiente, contorno, opacidade e sombra),
//! independente de interface e de formato de arquivo. Cores em texto `#rrggbb` ou
//! `#rrggbbaa`; vazio quer dizer "sem cor".

use feather_draw::{Cor, EstiloVetor, Parada, Ponto, Preenchimento, Retangulo, Stroke};
use serde::{Deserialize, Serialize};

/// A sombra projetada de uma forma. Cor vazia: sem sombra.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Sombra {
    /// `#rrggbb` ou `#rrggbbaa`.
    pub cor: String,
    /// O raio do desfoque, na unidade do documento.
    pub desfoque: f32,
    /// O deslocamento, na unidade do documento.
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TipoGradiente {
    Linear,
    Radial,
}

/// Uma parada do gradiente: posição de 0 a 1 e cor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PontoCor {
    pub posicao: f32,
    pub cor: String,
}

/// Gradiente com 2 a 16 paradas; `angulo` em graus (0 = da esquerda para a direita).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gradiente {
    pub tipo: TipoGradiente,
    pub angulo: f32,
    pub pontos: Vec<PontoCor>,
}

impl Default for Gradiente {
    fn default() -> Self {
        Self {
            tipo: TipoGradiente::Linear,
            angulo: 0.0,
            pontos: vec![
                PontoCor {
                    posicao: 0.0,
                    cor: "#4285f4".into(),
                },
                PontoCor {
                    posicao: 1.0,
                    cor: "#ffffff".into(),
                },
            ],
        }
    }
}

/// Confere uma cor `#RRGGBB` ou `#RRGGBBAA`; com `vazio`, aceita também "sem cor".
pub fn validar_cor(c: &str, vazio: bool) -> Result<(), String> {
    if vazio && c.is_empty() {
        return Ok(());
    }
    let h = c.strip_prefix('#').unwrap_or("");
    if ![6, 8].contains(&h.len()) || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("Use uma cor #RRGGBB ou #RRGGBBAA.".into());
    }
    Ok(())
}

/// A cor em texto como `Cor` do feather-draw; `None` para vazio ou inválido.
pub fn cor(c: &str) -> Option<Cor> {
    validar_cor(c, false).ok()?;
    let h = &c[1..];
    let byte = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok();
    Some(Cor::rgba(byte(0)?, byte(2)?, byte(4)?, if h.len() == 8 { byte(6)? } else { 255 }))
}

impl Gradiente {
    pub fn validar(&self) -> Result<(), String> {
        if !self.angulo.is_finite() || !(2..=16).contains(&self.pontos.len()) {
            return Err("Gradiente precisa de 2 a 16 pontos e ângulo válido.".into());
        }
        let mut anterior = -1.0;
        for p in &self.pontos {
            validar_cor(&p.cor, false)?;
            if !p.posicao.is_finite() || !(0.0..=1.0).contains(&p.posicao) || p.posicao < anterior {
                return Err("Posições devem estar em ordem, entre 0 e 100%.".into());
            }
            anterior = p.posicao;
        }
        Ok(())
    }

    /// O preenchimento do feather-draw sobre a caixa `r`. As paradas começam em 0 e terminam
    /// em 1 (a primeira e a última cor se estendem até as pontas).
    pub fn preenchimento(&self, r: Retangulo) -> Option<Preenchimento> {
        self.validar().ok()?;
        let mut paradas: Vec<Parada> = self.pontos.iter()
            .filter_map(|p| Some(Parada { posicao: p.posicao, cor: cor(&p.cor)? }))
            .collect();
        let primeira = *paradas.first()?;
        let ultima = *paradas.last()?;
        if primeira.posicao > 0.0 { paradas.insert(0, Parada { posicao: 0.0, cor: primeira.cor }); }
        if ultima.posicao < 1.0 { paradas.push(Parada { posicao: 1.0, cor: ultima.cor }); }
        let centro = Ponto::novo(r.x + r.largura / 2.0, r.y + r.altura / 2.0);
        Some(match self.tipo {
            TipoGradiente::Linear => {
                let (s, c) = self.angulo.to_radians().sin_cos();
                let meio = ((r.largura * c).abs() + (r.altura * s).abs()).max(1e-3) / 2.0;
                Preenchimento::Linear {
                    de: Ponto::novo(centro.x - c * meio, centro.y - s * meio),
                    ate: Ponto::novo(centro.x + c * meio, centro.y + s * meio),
                    paradas,
                }
            }
            TipoGradiente::Radial => Preenchimento::Radial {
                centro,
                raio: (r.largura.max(r.altura) / 2.0).max(1e-3),
                paradas,
            },
        })
    }
}

/// Aparência completa de uma forma.
#[derive(Clone, Debug, PartialEq)]
pub struct Aparencia {
    /// Cor sólida (vazio: sem preenchimento). O gradiente, quando existe, tem prioridade.
    pub preenchimento: String,
    pub gradiente: Option<Gradiente>,
    /// Cor do contorno (vazio: sem contorno).
    pub contorno: String,
    pub espessura: f32,
    /// Opacidade de 0 a 1 do objeto inteiro.
    pub opacidade: f32,
    pub sombra: Sombra,
}

impl Default for Aparencia {
    /// A aparência padrão da suíte: azul com contorno escuro de 2 unidades.
    fn default() -> Self {
        Self {
            preenchimento: "#4285f4".into(),
            gradiente: None,
            contorno: "#202124".into(),
            espessura: 2.0,
            opacidade: 1.0,
            sombra: Sombra::default(),
        }
    }
}

impl Aparencia {
    /// Confere cores, gradiente e limites (espessura 0 a 50, opacidade 0 a 1, desfoque 0 a 14,
    /// deslocamento da sombra até 200).
    pub fn validar(&self) -> Result<(), String> {
        validar_cor(&self.preenchimento, true)?;
        validar_cor(&self.contorno, true)?;
        validar_cor(&self.sombra.cor, true)?;
        if let Some(g) = &self.gradiente {
            g.validar()?;
        }
        if !self.espessura.is_finite()
            || !(0.0..=50.0).contains(&self.espessura)
            || !self.opacidade.is_finite()
            || !(0.0..=1.0).contains(&self.opacidade)
            || !self.sombra.desfoque.is_finite()
            || !(0.0..=14.0).contains(&self.sombra.desfoque)
            || !self.sombra.dx.is_finite()
            || !self.sombra.dy.is_finite()
            || self.sombra.dx.abs() > 200.0
            || self.sombra.dy.abs() > 200.0
        {
            return Err("Transparência, contorno ou sombra fora dos limites.".into());
        }
        Ok(())
    }

    /// O estilo do feather-draw para uma forma na caixa `r` (o gradiente se estende sobre ela).
    pub fn estilo_vetor(&self, r: Retangulo) -> EstiloVetor {
        let preenchimento = self.gradiente.as_ref().and_then(|g| g.preenchimento(r))
            .or_else(|| cor(&self.preenchimento).map(Preenchimento::Solido));
        let stroke = cor(&self.contorno)
            .filter(|_| self.espessura > 0.0 && self.espessura.is_finite())
            .map(|c| Stroke::novo(c, self.espessura));
        EstiloVetor {
            preenchimento,
            stroke,
            opacidade: if self.opacidade.is_finite() { self.opacidade.clamp(0.0, 1.0) } else { 1.0 },
            texto: None,
        }
    }

    /// O estilo só de traço (detalhes abertos da forma): a cor do contorno, sem preenchimento.
    pub fn estilo_traco(&self) -> EstiloVetor {
        EstiloVetor { preenchimento: None, ..self.estilo_vetor(Retangulo::default()) }
    }

    /// O estilo da sombra (cor da sombra, sem contorno), ou `None` sem sombra.
    pub fn estilo_sombra(&self) -> Option<EstiloVetor> {
        let c = cor(&self.sombra.cor)?;
        Some(EstiloVetor {
            preenchimento: Some(Preenchimento::Solido(c)),
            stroke: None,
            opacidade: self.estilo_vetor(Retangulo::default()).opacidade,
            texto: None,
        })
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn aparencia() -> Aparencia {
        Aparencia {
            preenchimento: "#4285f4".into(),
            gradiente: Some(Gradiente::default()),
            contorno: "#112233".into(),
            espessura: 2.0,
            opacidade: 0.7,
            sombra: Sombra { cor: "#00000080".into(), desfoque: 4.0, dx: 3.0, dy: 5.0 },
        }
    }

    #[test]
    fn cores_validas_e_invalidas() {
        assert!(validar_cor("#12345", false).is_err());
        assert!(validar_cor("", true).is_ok());
        assert!(validar_cor("", false).is_err());
        assert!(validar_cor("#11223344", false).is_ok());
        assert_eq!(cor("#ff000080"), Some(Cor::rgba(255, 0, 0, 128)));
        assert_eq!(cor("#00ff00"), Some(Cor::rgb(0, 255, 0)));
        assert_eq!(cor("verde"), None);
    }

    #[test]
    fn recusa_gradiente_e_limites_invalidos() {
        assert!(aparencia().validar().is_ok());
        let mut a = aparencia();
        a.gradiente.as_mut().unwrap().pontos[0].posicao = f32::NAN;
        assert!(a.validar().is_err());
        let mut a = aparencia();
        a.opacidade = 2.0;
        assert!(a.validar().is_err());
        let mut a = aparencia();
        a.sombra.dx = 500.0;
        assert!(a.validar().is_err());
    }

    #[test]
    fn estilo_vetor_usa_gradiente_contorno_e_opacidade() {
        let a = aparencia();
        let e = a.estilo_vetor(Retangulo::novo(0.0, 0.0, 100.0, 50.0));
        assert!(e.valido());
        assert!(matches!(e.preenchimento, Some(Preenchimento::Linear { .. })));
        assert_eq!(e.stroke.as_ref().map(|s| s.largura), Some(2.0));
        assert_eq!(e.opacidade, 0.7);
        let mut g = Gradiente { tipo: TipoGradiente::Radial, ..Gradiente::default() };
        g.pontos[0].posicao = 0.3;
        assert!(matches!(g.preenchimento(Retangulo::novo(0.0, 0.0, 10.0, 10.0)), Some(Preenchimento::Radial { ref paradas, .. }) if paradas.len() == 3));
        let sem = Aparencia { preenchimento: String::new(), gradiente: None, contorno: String::new(), ..Aparencia::default() };
        let e = sem.estilo_vetor(Retangulo::default());
        assert!(e.preenchimento.is_none() && e.stroke.is_none());
        assert!(a.estilo_sombra().is_some() && sem.estilo_sombra().is_none());
    }
}
