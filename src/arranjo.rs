//! Arranjos do editor de diagramas: a árvore de itens (texto, pai e nível), as operações de
//! edição (adicionar, excluir, mover, promover e rebaixar) e a posição de cada item numa
//! caixa. É a lógica pura do editor; quem aplica no documento e desfaz é o aplicativo.

use crate::diagramas::{ItemDiagrama, RotuloItem, CORES, LIGACAO};
use crate::tipo::TipoForma;

/// Até quantos itens um diagrama aceita.
pub const MAX_ITENS: usize = 64;

/// Como os itens se distribuem na caixa.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arranjo {
    Lista,
    Processo,
    Ciclo,
    Organograma,
}

impl Arranjo {
    pub const TODOS: [Self; 4] = [Self::Lista, Self::Processo, Self::Ciclo, Self::Organograma];

    /// Nome estável, gravado pelos aplicativos.
    pub fn identificador(self) -> &'static str {
        match self {
            Self::Lista => "lista",
            Self::Processo => "processo",
            Self::Ciclo => "ciclo",
            Self::Organograma => "organograma",
        }
    }

    /// Lê o nome gravado; desconhecido vira lista.
    pub fn ler(nome: &str) -> Self {
        match nome {
            "ciclo" => Self::Ciclo,
            "processo" => Self::Processo,
            "organograma" => Self::Organograma,
            _ => Self::Lista,
        }
    }

    /// Estilo do conector entre itens ligados (`cotovelo` no organograma, `reto` nos demais).
    pub fn estilo_conector(self) -> &'static str {
        if self == Self::Organograma { "cotovelo" } else { "reto" }
    }
}

/// Um item do diagrama.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoDiagrama {
    pub id: u64,
    pub texto: String,
    /// Profundidade na hierarquia: raiz = 0, filho = 1, neto = 2.
    pub linha: usize,
    pub pai: Option<u64>,
}

/// Acrescenta um item (filho de `pai`, se houver) com um id livre no fim do espaço de ids.
pub fn adicionar(nos: &mut Vec<NoDiagrama>, texto: String, pai: Option<u64>) -> Result<(), String> {
    if nos.len() >= MAX_ITENS {
        return Err("O diagrama aceita até 64 itens.".into());
    }
    if pai.is_some_and(|id| !nos.iter().any(|n| n.id == id)) {
        return Err("O item de origem não existe.".into());
    }
    let id = (0..65)
        .map(|i| u64::MAX - i)
        .find(|id| !nos.iter().any(|n| n.id == *id))
        .unwrap_or(u64::MAX - 65);
    nos.push(NoDiagrama { id, texto, linha: 0, pai });
    ordenar(nos);
    Ok(())
}

/// Exclui o item `i`; os filhos sobem para o pai dele.
pub fn excluir(nos: &mut Vec<NoDiagrama>, i: usize) -> Result<(), String> {
    if nos.len() <= 1 {
        return Err("Mantenha pelo menos um item.".into());
    }
    if i >= nos.len() {
        return Err("Esse item não existe.".into());
    }
    let removido = nos.remove(i);
    for no in nos.iter_mut() {
        if no.pai == Some(removido.id) {
            no.pai = removido.pai;
        }
    }
    ordenar(nos);
    Ok(())
}

/// Troca o item `i` com o irmão anterior (`delta < 0`) ou seguinte. Devolve se mudou.
pub fn mover(nos: &mut Vec<NoDiagrama>, i: usize, delta: isize) -> bool {
    if i >= nos.len() {
        return false;
    }
    let pai = nos[i].pai;
    let destino = if delta < 0 {
        (0..i).rev().find(|&j| nos[j].pai == pai)
    } else {
        ((i + 1)..nos.len()).find(|&j| nos[j].pai == pai)
    };
    if let Some(j) = destino {
        nos.swap(i, j);
        ordenar(nos);
        return true;
    }
    false
}

/// Troca o arranjo: ao mudar, o organograma pendura todos na raiz e os demais desfazem a
/// hierarquia.
pub fn trocar_arranjo(nos: &mut Vec<NoDiagrama>, de: Arranjo, para: Arranjo) {
    if de != para {
        let raiz = nos.first().map(|n| n.id);
        for (i, n) in nos.iter_mut().enumerate() {
            n.pai = if para == Arranjo::Organograma && i > 0 { raiz } else { None };
        }
    }
    ordenar(nos);
}

/// Rebaixa (`aprofundar`: vira filho do irmão anterior) ou promove o item `i` no
/// organograma. Devolve se mudou.
pub fn alterar_nivel(nos: &mut Vec<NoDiagrama>, arranjo: Arranjo, i: usize, aprofundar: bool) -> bool {
    if i >= nos.len() || arranjo != Arranjo::Organograma {
        return false;
    }
    let pai = nos[i].pai;
    let novo = if aprofundar {
        (0..i).rev().find(|&j| nos[j].pai == pai).map(|j| Some(nos[j].id))
    } else {
        pai.and_then(|p| nos.iter().find(|n| n.id == p).map(|n| n.pai))
    };
    if let Some(novo) = novo {
        nos[i].pai = novo;
        ordenar(nos);
        return true;
    }
    false
}

/// Põe os itens em ordem de árvore (pai antes dos filhos) e recalcula o nível. Itens cujo
/// pai sumiu voltam para a raiz.
pub fn ordenar(nos: &mut Vec<NoDiagrama>) {
    fn visitar(pai: Option<u64>, nivel: usize, nos: &[NoDiagrama], saida: &mut Vec<NoDiagrama>) {
        if nivel > 64 {
            return;
        }
        for no in nos.iter().filter(|n| n.pai == pai) {
            if saida.iter().any(|n| n.id == no.id) {
                continue;
            }
            let mut no = no.clone();
            no.linha = nivel;
            saida.push(no.clone());
            visitar(Some(no.id), nivel + 1, nos, saida);
        }
    }
    let mut saida = Vec::new();
    visitar(None, 0, nos, &mut saida);
    for no in nos.iter() {
        if !saida.iter().any(|n| n.id == no.id) {
            let mut no = no.clone();
            no.pai = None;
            no.linha = 0;
            saida.push(no);
        }
    }
    *nos = saida;
}

/// A caixa (`[x, y, largura, altura]`) do item `i` no arranjo, dentro de `area`.
pub fn retangulo(arranjo: Arranjo, nos: &[NoDiagrama], i: usize, area: [f32; 4]) -> [f32; 4] {
    if arranjo == Arranjo::Organograma {
        posicao_arvore(nos, i, area)
    } else {
        posicao(arranjo, i, nos.len(), area)
    }
}

/// Os pares (origem, destino), por índice, que levam conector no arranjo.
pub fn ligacoes(arranjo: Arranjo, nos: &[NoDiagrama]) -> Vec<(usize, usize)> {
    match arranjo {
        Arranjo::Lista => vec![],
        Arranjo::Organograma => nos
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                n.pai
                    .and_then(|p| nos.iter().position(|n| n.id == p))
                    .map(|p| (p, i))
            })
            .collect(),
        Arranjo::Processo => (1..nos.len()).map(|i| (i - 1, i)).collect(),
        Arranjo::Ciclo => {
            if nos.len() > 1 {
                (0..nos.len()).map(|i| (i, (i + 1) % nos.len())).collect()
            } else {
                vec![]
            }
        }
    }
}

fn posicao_arvore(nos: &[NoDiagrama], i: usize, [x, y, w, h]: [f32; 4]) -> [f32; 4] {
    fn folhas(id: u64, nos: &[NoDiagrama], nivel: usize) -> usize {
        if nivel > 64 {
            return 1;
        }
        let filhos: Vec<_> = nos.iter().filter(|n| n.pai == Some(id)).collect();
        if filhos.is_empty() {
            1
        } else {
            filhos
                .into_iter()
                .map(|n| folhas(n.id, nos, nivel + 1))
                .sum()
        }
    }
    let folha = |n: &NoDiagrama| !nos.iter().any(|f| f.pai == Some(n.id));
    let total = nos.iter().filter(|n| folha(n)).count().max(1);
    let inicio = nos[..i].iter().filter(|n| folha(n)).count();
    let span = folhas(nos[i].id, nos, 0);
    let cw = w / total as f32;
    let passo = h / (nos.iter().map(|n| n.linha).max().unwrap_or(0) + 1) as f32;
    let bw = (cw * 0.85).min(180.);
    let bh = (passo * 0.6).min(75.);
    [
        x + (inicio as f32 + span as f32 / 2.) * cw - bw / 2.,
        y + nos[i].linha as f32 * passo,
        bw,
        bh,
    ]
}

fn posicao(arranjo: Arranjo, i: usize, n: usize, [x, y, w, h]: [f32; 4]) -> [f32; 4] {
    let nf = n as f32;
    match arranjo {
        Arranjo::Lista => {
            let altura = (h / nf).min(70.);
            [x + w * 0.15, y + i as f32 * h / nf, w * 0.7, altura * 0.8]
        }
        Arranjo::Processo => {
            let colunas = n.min(6);
            let linhas = n.div_ceil(colunas);
            let cw = w / colunas as f32;
            let ch = h / linhas as f32;
            [
                x + (i % colunas) as f32 * cw,
                y + (i / colunas) as f32 * ch + ch * 0.2,
                cw * 0.8,
                ch * 0.6,
            ]
        }
        Arranjo::Organograma => {
            if i == 0 {
                return [x + w * 0.35, y, w * 0.3, h * 0.2];
            }
            let colunas = (n - 1).min(5);
            let linhas = (n - 1).div_ceil(colunas);
            let cw = w / colunas as f32;
            let ch = h * 0.65 / linhas as f32;
            [
                x + ((i - 1) % colunas) as f32 * cw,
                y + h * 0.35 + ((i - 1) / colunas) as f32 * ch,
                cw * 0.85,
                ch * 0.7,
            ]
        }
        Arranjo::Ciclo => {
            let d = (w.min(h) * 0.3).min(w.min(h) * 1.7 / nf.max(4.)).max(6.);
            let t = i as f32 * std::f32::consts::TAU / nf - std::f32::consts::FRAC_PI_2;
            [
                x + w / 2. + (w - d) / 2. * t.cos() - d / 2.,
                y + h / 2. + (h - d) / 2. * t.sin() - d / 2.,
                d,
                d,
            ]
        }
    }
}

/// Um conector do diagrama: liga o item `de` ao item `ate` (índices em `itens`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conector {
    pub de: usize,
    pub ate: usize,
    /// `reto` ou `cotovelo` (ver `galeria::rota_conector`).
    pub estilo: &'static str,
}

/// Um diagrama montado: itens posicionados (neutros) e conectores entre eles.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagrama {
    pub nos: Vec<NoDiagrama>,
    pub itens: Vec<ItemDiagrama>,
    pub conectores: Vec<Conector>,
}

/// Monta um diagrama a partir de textos com nível (0 = raiz). No organograma, o pai de cada
/// item é o último item anterior com nível menor; nos outros arranjos o nível é ignorado.
/// `area` é `[x, y, largura, altura]`. Aceita até [`MAX_ITENS`] itens.
pub fn diagrama(arranjo: Arranjo, textos: &[(String, usize)], area: [f32; 4]) -> Result<Diagrama, String> {
    if textos.is_empty() || textos.len() > MAX_ITENS {
        return Err("O diagrama precisa de 1 a 64 itens.".into());
    }
    if area.iter().any(|v| !v.is_finite()) || area[2] <= 0.0 || area[3] <= 0.0 {
        return Err("Área do diagrama inválida.".into());
    }
    let mut nos = Vec::with_capacity(textos.len());
    let mut pilha: Vec<(usize, u64)> = Vec::new();
    for (i, (texto, nivel)) in textos.iter().enumerate() {
        let id = i as u64 + 1;
        let pai = if arranjo == Arranjo::Organograma {
            while pilha.last().is_some_and(|(n, _)| n >= nivel) {
                pilha.pop();
            }
            pilha.last().map(|(_, id)| *id)
        } else {
            None
        };
        pilha.push((*nivel, id));
        nos.push(NoDiagrama { id, texto: texto.clone(), linha: 0, pai });
    }
    ordenar(&mut nos);
    let itens = (0..nos.len()).map(|i| {
        let [x, y, w, h] = retangulo(arranjo, &nos, i, area);
        let cor = CORES[i % CORES.len()];
        ItemDiagrama {
            tipo: if arranjo == Arranjo::Ciclo { TipoForma::Elipse } else { TipoForma::RetanguloArredondado },
            x, y, largura: w, altura: h,
            texto: nos[i].texto.clone(),
            preenchimento: cor.into(),
            contorno: cor.into(),
            espessura: 1.4,
            rotulo: Some(RotuloItem { cor: "#FFFFFF".into(), tamanho: (h * 0.3).clamp(6.0, 20.0), negrito: true }),
        }
    }).collect();
    let conectores = ligacoes(arranjo, &nos).into_iter()
        .map(|(de, ate)| Conector { de, ate, estilo: arranjo.estilo_conector() })
        .collect();
    Ok(Diagrama { nos, itens, conectores })
}

/// Cor dos conectores dos diagramas.
pub const COR_CONECTOR: &str = LIGACAO;

#[cfg(test)]
mod testes {
    use super::*;

    fn arvore() -> Vec<NoDiagrama> {
        let t = |s: &str, n| (s.to_string(), n);
        diagrama(Arranjo::Organograma, &[t("Nível 1", 0), t("Ramo A", 1), t("Item 1", 2), t("Item 2", 2), t("Ramo B", 1)], [0.0, 0.0, 720.0, 360.0]).unwrap().nos
    }

    #[test]
    fn niveis_viram_hierarquia() {
        let nos = arvore();
        assert_eq!(nos.iter().map(|n| (n.texto.as_str(), n.linha)).collect::<Vec<_>>(),
            [("Nível 1", 0), ("Ramo A", 1), ("Item 1", 2), ("Item 2", 2), ("Ramo B", 1)]);
        assert_eq!(nos[2].pai, Some(nos[1].id));
        assert_eq!(ligacoes(Arranjo::Organograma, &nos).len(), 4);
    }

    #[test]
    fn excluir_promove_filhos_e_mover_preserva_subarvores() {
        let mut nos = arvore();
        assert!(mover(&mut nos, 1, 1));
        assert_eq!(nos[1].texto, "Ramo B");
        assert_eq!(nos[2].texto, "Ramo A");
        assert_eq!(nos[3].texto, "Item 1");
        excluir(&mut nos, 2).unwrap();
        assert_eq!(nos[2].texto, "Item 1");
        assert_eq!(nos[2].linha, 1);
        assert!(excluir(&mut nos, 99).is_err());
        assert!(!mover(&mut nos, 0, -1));
    }

    #[test]
    fn promover_e_rebaixar() {
        let mut nos = arvore();
        let ramo_a = nos[1].id;
        let raiz = nos[0].id;
        assert!(alterar_nivel(&mut nos, Arranjo::Organograma, 4, true));
        let b = nos.iter().position(|n| n.texto == "Ramo B").unwrap();
        assert_eq!((nos[b].pai, nos[b].linha), (Some(ramo_a), 2));
        assert!(alterar_nivel(&mut nos, Arranjo::Organograma, b, false));
        let b = nos.iter().find(|n| n.texto == "Ramo B").unwrap();
        assert_eq!((b.pai, b.linha), (Some(raiz), 1));
        assert!(!alterar_nivel(&mut nos, Arranjo::Lista, 1, true));
    }

    #[test]
    fn adicionar_limita_e_troca_de_arranjo() {
        let mut nos = arvore();
        let ramo = nos[1].id;
        adicionar(&mut nos, "Novo".into(), Some(ramo)).unwrap();
        assert!(nos.iter().any(|n| n.texto == "Novo" && n.linha == 2));
        assert!(adicionar(&mut nos, "x".into(), Some(12345)).is_err());
        trocar_arranjo(&mut nos, Arranjo::Organograma, Arranjo::Ciclo);
        assert!(nos.iter().all(|n| n.pai.is_none() && n.linha == 0));
        while nos.len() < MAX_ITENS { adicionar(&mut nos, "x".into(), None).unwrap(); }
        assert!(adicionar(&mut nos, "x".into(), None).is_err());
    }

    #[test]
    fn todos_os_arranjos_cabem_na_area() {
        for a in Arranjo::TODOS {
            assert_eq!(Arranjo::ler(a.identificador()), a);
            for n in [1usize, 3, 7, 20] {
                let textos: Vec<_> = (0..n).map(|i| (format!("Item {i}"), usize::from(i > 0))).collect();
                let d = diagrama(a, &textos, [10.0, 20.0, 700.0, 400.0]).unwrap();
                assert_eq!(d.itens.len(), n);
                for it in &d.itens {
                    assert!(it.x >= 9.9 && it.y >= 19.9 && it.x + it.largura <= 710.1 && it.y + it.altura <= 420.1, "{a:?} {n}: {it:?}");
                }
                assert!(d.conectores.iter().all(|c| c.de < n && c.ate < n));
            }
        }
        assert!(diagrama(Arranjo::Lista, &[], [0.0, 0.0, 1.0, 1.0]).is_err());
    }
}
