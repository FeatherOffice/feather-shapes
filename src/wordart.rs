//! Texto artístico (WordArt): a deformação dos contornos do texto. O conteúdo continua
//! editável; o aplicativo resolve os contornos da fonte e aplica [`transformar`] a cada ponto.
//! [`ESTILOS`] são os estilos prontos da galeria (cor, contorno, espessura, sombra), só dados.

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

/// Sombra de um estilo pronto: cor (`#rrggbbaa`) e deslocamento em pontos.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SombraEstilo {
    pub cor: &'static str,
    pub dx: f32,
    pub dy: f32,
}

/// Um estilo pronto do texto artístico (galeria). Só dados: cada aplicativo copia os campos
/// para o próprio modelo. Cores em `#rrggbb`/`#rrggbbaa`; contorno vazio = sem contorno.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EstiloWordArt {
    /// Nome mostrado na galeria (pt-BR).
    pub nome: &'static str,
    /// Nome estável, gravado no arquivo.
    pub chave: &'static str,
    pub preenchimento: &'static str,
    pub contorno: &'static str,
    pub espessura: f32,
    pub sombra: Option<SombraEstilo>,
}

/// A sombra padrão do botão "Sombra" (liga/desliga) sobre qualquer estilo.
pub const SOMBRA_PADRAO: SombraEstilo = SombraEstilo { cor: "#20212480", dx: 3.0, dy: 3.0 };

/// Os estilos prontos, na ordem da galeria. O índice 0 é o padrão de um WordArt novo.
pub const ESTILOS: [EstiloWordArt; 4] = [
    EstiloWordArt { nome: "Sólido", chave: "solido", preenchimento: "#1a73e8", contorno: "", espessura: 0.0, sombra: None },
    EstiloWordArt { nome: "Contornado", chave: "contornado", preenchimento: "#ffffff", contorno: "#1a73e8", espessura: 1.8, sombra: None },
    EstiloWordArt { nome: "Dourado", chave: "dourado", preenchimento: "#fbbc04", contorno: "#b06000", espessura: 1.0, sombra: Some(SombraEstilo { cor: "#20212480", dx: 3.0, dy: 3.0 }) },
    EstiloWordArt { nome: "Rosa", chave: "rosa", preenchimento: "#e665a0", contorno: "#8e24aa", espessura: 1.0, sombra: Some(SombraEstilo { cor: "#8e24aa60", dx: 3.0, dy: 3.0 }) },
];

/// O estilo pelo índice da galeria; fora do intervalo vira o padrão (índice 0).
pub fn estilo(indice: usize) -> &'static EstiloWordArt {
    ESTILOS.get(indice).unwrap_or(&ESTILOS[0])
}

/// Lê a chave gravada; desconhecida vira o padrão (índice 0).
pub fn estilo_por_chave(chave: &str) -> (usize, &'static EstiloWordArt) {
    ESTILOS.iter().enumerate().find(|(_, e)| e.chave == chave).unwrap_or((0, &ESTILOS[0]))
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
    #[test]
    fn estilos_prontos_chaves_unicas_e_padrao() {
        for (i, e) in ESTILOS.iter().enumerate() {
            assert_eq!(estilo_por_chave(e.chave).0, i);
            assert!(e.preenchimento.starts_with('#'));
            assert_eq!(e.contorno.is_empty(), e.espessura == 0.0);
        }
        assert_eq!(estilo(99).chave, "solido");
        assert_eq!(estilo_por_chave("xyz").0, 0);
    }
}
