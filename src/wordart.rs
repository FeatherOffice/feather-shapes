//! Texto artístico (WordArt): a deformação dos contornos do texto. O conteúdo continua
//! editável; o aplicativo resolve os contornos da fonte e aplica [`transformar`] a cada ponto.

/// A transformação aplicada ao texto. O nome em [`Transformacao::nome`] é o que se grava.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Transformacao {
    #[default]
    Reto,
    Arco,
    Onda,
    Inclinado,
}

impl Transformacao {
    pub const TODOS: [Self; 4] = [Self::Reto, Self::Arco, Self::Onda, Self::Inclinado];
    /// Nome estável, gravado no arquivo.
    pub fn nome(self) -> &'static str {
        match self { Self::Reto => "reto", Self::Arco => "arco", Self::Onda => "onda", Self::Inclinado => "inclinado" }
    }
    /// Lê o nome gravado; desconhecido vira [`Transformacao::Reto`].
    pub fn ler(nome: &str) -> Self {
        Self::TODOS.into_iter().find(|t| t.nome() == nome).unwrap_or_default()
    }
}

impl std::fmt::Display for Transformacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self { Self::Reto => "Reto", Self::Arco => "Arco", Self::Onda => "Onda", Self::Inclinado => "Inclinado" })
    }
}

/// Deforma um ponto do contorno do texto dentro da caixa `base` (`[x, y, largura, altura]`),
/// mantendo curvas e texto separados. `intensidade` vai de -1 a 1.
pub fn transformar([x, y]: [f32; 2], base: [f32; 4], modo: Transformacao, intensidade: f32) -> [f32; 2] {
    let [bx, by, w, h] = base;
    let u = ((x - bx) / w.max(1.0)).clamp(0.0, 1.0);
    let a = intensidade.clamp(-1.0, 1.0);
    match modo {
        Transformacao::Reto => [x, y],
        Transformacao::Arco => [x, y - h * a * 0.5 * (std::f32::consts::PI * u).sin()],
        Transformacao::Onda => [x, y + h * a * 0.25 * (std::f32::consts::TAU * u).sin()],
        Transformacao::Inclinado => [x + (by + h / 2.0 - y) * a * 0.4, y],
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    #[test]
    fn arco_preserva_extremos_e_eleva_centro() {
        let b = [0.0, 0.0, 100.0, 40.0];
        assert_eq!(transformar([0.0, 20.0], b, Transformacao::Arco, 1.0), [0.0, 20.0]);
        assert!((transformar([50.0, 20.0], b, Transformacao::Arco, 1.0)[1]).abs() < 0.01);
    }
    #[test]
    fn nomes_ida_e_volta() {
        for t in Transformacao::TODOS {
            assert_eq!(Transformacao::ler(t.nome()), t);
        }
        assert_eq!(Transformacao::ler("xyz"), Transformacao::Reto);
        assert_eq!(transformar([3.0, 4.0], [0.0, 0.0, 10.0, 10.0], Transformacao::Reto, 1.0), [3.0, 4.0]);
    }
}
