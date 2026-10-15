//! A biblioteca de ícones: os ícones de contorno do **Tabler Icons**
//! (https://github.com/tabler/tabler-icons, licença MIT — o aviso está em `dados/TABLER-LICENSE.txt`),
//! 5 mil e tantos, embutidos no programa. O nome do ícone (`home`, `arrow-right`…) é o que o
//! aplicativo grava; o desenho sai como traços redondos (sem arquivo de imagem, sem rede), e
//! recolorir e redimensionar não perdem qualidade.
//!
//! Para atualizar o conjunto: `tools/gerar_tabler.py`.

use std::collections::HashMap;
use std::sync::OnceLock;

use feather_draw::{Caminho, Cena, Comando, Cor, EstiloVetor, Extremo, Figura, Juncao, Ponto, Retangulo, Stroke};

use crate::svgpath;

static DADOS: &str = include_str!("../dados/tabler-outline.tsv");

/// O texto da licença MIT do Tabler Icons, para a tela "Sobre" dos aplicativos.
pub const LICENCA_TABLER: &str = include_str!("../dados/TABLER-LICENSE.txt");

/// Um ícone da biblioteca.
#[derive(Debug, PartialEq, Eq)]
pub struct Icone {
    /// O nome do Tabler (`home`, `arrow-right`…): é o que vai gravado no arquivo.
    pub id: &'static str,
    pub categoria: &'static str,
    /// Palavras-chave em inglês, separadas por vírgula.
    pub tags: &'static str,
    /// Os caminhos SVG (atributo `d`), separados por `|`.
    caminhos: &'static str,
}

impl Icone {
    /// Os caminhos SVG originais, no sistema de coordenadas 24 × 24 do Tabler.
    pub fn caminhos_svg(&self) -> impl Iterator<Item = &'static str> {
        self.caminhos.split('|')
    }

    /// O nome para mostrar ("arrow-right" → "arrow right").
    pub fn rotulo(&self) -> String {
        self.id.replace('-', " ")
    }
}

fn indice() -> &'static (Vec<Icone>, HashMap<&'static str, usize>) {
    static I: OnceLock<(Vec<Icone>, HashMap<&'static str, usize>)> = OnceLock::new();
    I.get_or_init(|| {
        let todos: Vec<Icone> = DADOS
            .lines()
            .filter_map(|l| {
                let mut c = l.splitn(4, '\t');
                Some(Icone { id: c.next()?, categoria: c.next()?, tags: c.next()?, caminhos: c.next()? })
            })
            .collect();
        let por_id = todos.iter().enumerate().map(|(i, ic)| (ic.id, i)).collect();
        (todos, por_id)
    })
}

/// Todos os ícones, em ordem alfabética.
pub fn todos() -> &'static [Icone] {
    &indice().0
}

/// Os nomes da primeira versão dos ícones (desenhados à mão) → o nome no Tabler.
const ANTIGOS: &[(&str, &str)] = &[
    ("seta_direita", "arrow-right"), ("seta_esquerda", "arrow-left"), ("seta_cima", "arrow-up"), ("seta_baixo", "arrow-down"), ("estrela", "star"), ("coracao", "heart"),
    ("xis", "x"), ("mais", "plus"), ("alvo", "target"), ("raio", "bolt"), ("escudo", "shield"), ("casa", "home"), ("lampada", "bulb"), ("engrenagem", "settings"),
    ("nuvem", "cloud"), ("sol", "sun"), ("lua", "moon"), ("relogio", "clock"), ("bandeira", "flag"), ("pin", "map-pin"), ("usuario", "user"), ("email", "mail"),
    ("documento", "file-text"), ("barras", "chart-bar"), ("cadeado", "lock"), ("balao", "message"), ("lupa", "search"), ("globo", "world"),
];

/// O ícone de nome `id` (aceita também os nomes antigos em português).
pub fn buscar_por_id(id: &str) -> Option<&'static Icone> {
    let (todos, por_id) = indice();
    let id = ANTIGOS.iter().find(|(a, _)| *a == id).map_or(id, |(_, n)| *n);
    por_id.get(id).map(|&i| &todos[i])
}

pub fn existe(id: &str) -> bool {
    buscar_por_id(id).is_some()
}

/// Português → as palavras do Tabler (inglês) que valem como busca.
const TRADUCAO: &[(&str, &str)] = &[
    ("casa", "home,house"), ("lar", "home"), ("coracao", "heart"), ("amor", "heart"), ("estrela", "star"), ("seta", "arrow"), ("direita", "right"), ("esquerda", "left"),
    ("cima", "up"), ("baixo", "down"), ("usuario", "user"), ("pessoa", "user"), ("pessoas", "users"), ("grupo", "users,group"), ("equipe", "users,team"), ("configuracao", "settings"),
    ("configuracoes", "settings"), ("engrenagem", "settings,gear"), ("ajustes", "settings,adjustments"), ("busca", "search"), ("pesquisa", "search"), ("lupa", "search,zoom"),
    ("email", "mail"), ("carta", "mail"), ("mensagem", "message"), ("conversa", "message,chat"), ("telefone", "phone"), ("celular", "device-mobile,phone"), ("calendario", "calendar"),
    ("agenda", "calendar"), ("relogio", "clock"), ("hora", "clock,time"), ("tempo", "clock,time"), ("grafico", "chart"), ("graficos", "chart"), ("barras", "chart-bar"),
    ("pizza", "chart-pie"), ("linha", "line"), ("pasta", "folder"), ("arquivo", "file"), ("documento", "file,document"), ("texto", "text"), ("nuvem", "cloud"), ("sol", "sun"),
    ("lua", "moon"), ("noite", "moon"), ("chuva", "rain,cloud-rain"), ("neve", "snowflake"), ("raio", "bolt"), ("trovao", "bolt"), ("energia", "bolt,battery"), ("bateria", "battery"),
    ("lampada", "bulb"), ("ideia", "bulb"), ("cadeado", "lock"), ("trancar", "lock"), ("chave", "key"), ("escudo", "shield"), ("seguranca", "shield,lock"), ("bandeira", "flag"),
    ("local", "map-pin"), ("localizacao", "map-pin"), ("mapa", "map"), ("camera", "camera"), ("foto", "photo,camera"), ("imagem", "photo,image"), ("video", "video"),
    ("musica", "music"), ("som", "volume,music"), ("livro", "book"), ("escola", "school"), ("faculdade", "school,building"), ("predio", "building"), ("empresa", "building"),
    ("loja", "building-store,shopping"), ("carro", "car"), ("onibus", "bus"), ("trem", "train"), ("aviao", "plane"), ("bicicleta", "bike"), ("barco", "ship,boat"),
    ("cafe", "coffee"), ("comida", "tools-kitchen,pizza"), ("bebida", "glass,cup"), ("sino", "bell"), ("notificacao", "bell"), ("lixo", "trash"), ("lixeira", "trash"),
    ("editar", "edit,pencil"), ("lapis", "pencil"), ("caneta", "pen"), ("copiar", "copy"), ("colar", "clipboard"), ("baixar", "download"), ("enviar", "send,upload"),
    ("link", "link"), ("corrente", "link"), ("impressora", "printer"), ("imprimir", "printer"), ("olho", "eye"), ("ver", "eye"), ("alerta", "alert"), ("aviso", "alert"),
    ("perigo", "alert-triangle"), ("informacao", "info-circle"), ("ajuda", "help"), ("duvida", "help,question"), ("bussola", "compass"), ("mundo", "world"), ("globo", "world"),
    ("planeta", "world,planet"), ("arvore", "tree"), ("folha", "leaf"), ("planta", "plant,leaf"), ("fogo", "flame"), ("chama", "flame"), ("diamante", "diamond"), ("trofeu", "trophy"),
    ("premio", "trophy,award"), ("presente", "gift"), ("carrinho", "shopping-cart"), ("compras", "shopping-cart,shopping-bag"), ("dinheiro", "currency-dollar,coin,cash"),
    ("moeda", "coin"), ("cartao", "credit-card"), ("foguete", "rocket"), ("alvo", "target"), ("objetivo", "target"), ("quebra-cabeca", "puzzle"), ("joinha", "thumb-up"),
    ("curtir", "thumb-up,heart"), ("compartilhar", "share"), ("marcador", "bookmark"), ("etiqueta", "tag"), ("filtro", "filter"), ("lista", "list"), ("grade", "layout-grid,grid"),
    ("computador", "device-desktop,device-laptop"), ("notebook", "device-laptop"), ("tela", "device-desktop"), ("processador", "cpu"), ("dados", "database"), ("banco", "database"),
    ("codigo", "code"), ("cerebro", "brain"), ("microscopio", "microscope"), ("ciencia", "flask,atom"), ("atomo", "atom"), ("ferramenta", "tool,tools"), ("martelo", "hammer"),
    ("tinta", "paint,brush"), ("pintar", "paint,brush"), ("cor", "palette,color"), ("regua", "ruler"), ("tesoura", "scissors"), ("cortar", "scissors,crop"), ("zoom", "zoom"),
    ("mais", "plus"), ("menos", "minus"), ("fechar", "x"), ("cancelar", "x"), ("confirmar", "check"), ("certo", "check"), ("ok", "check"), ("errado", "x"), ("sair", "logout"),
    ("entrar", "login"), ("menu", "menu"), ("voltar", "arrow-back,arrow-left"), ("avancar", "arrow-forward,arrow-right"), ("atualizar", "refresh"), ("girar", "rotate"),
    ("saude", "heartbeat,stethoscope"), ("hospital", "building-hospital"), ("remedio", "pill"), ("medico", "stethoscope"), ("dente", "dental"), ("bebe", "baby-carriage"),
    ("animal", "paw"), ("cachorro", "dog"), ("gato", "cat"), ("passaro", "feather,bird"), ("peixe", "fish"), ("flor", "flower"), ("fruta", "apple"), ("maca", "apple"),
    ("esporte", "ball-football,trophy"), ("futebol", "ball-football"), ("jogo", "device-gamepad"), ("dado", "dice"), ("festa", "confetti,balloon"), ("balao", "balloon"),
    ("telescopio", "telescope"), ("satelite", "satellite"), ("antena", "antenna"), ("wifi", "wifi"), ("bluetooth", "bluetooth"), ("sinal", "antenna-bars-5"), ("tomada", "plug"),
    ("fone", "headphones"), ("microfone", "microphone"), ("alto-falante", "speakerphone,volume"), ("cinema", "movie"), ("filme", "movie"), ("teatro", "masks-theater"),
    ("juiz", "gavel,scale"), ("lei", "gavel,scale"), ("justica", "scale"), ("balanca", "scale"), ("bomba", "bomb"), ("pe", "foot"), ("mao", "hand-stop,hand"), ("dedo", "pointer"),
    ("cursor", "pointer,cursor-text"), ("apresentacao", "presentation"), ("slide", "presentation"), ("projetor", "presentation"), ("quadro", "chalkboard"), ("tabela", "table"),
];

fn sem_acento(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            c => c,
        })
        .collect()
}

/// Os ícones mais usados, na ordem em que aparecem quando a busca está vazia.
const POPULARES: &[&str] = &[
    "home", "heart", "star", "check", "x", "plus", "minus", "arrow-right", "arrow-left", "arrow-up", "arrow-down", "user", "users", "settings", "search", "mail", "phone", "calendar",
    "clock", "chart-bar", "chart-pie", "chart-line", "folder", "file-text", "cloud", "sun", "moon", "bulb", "lock", "key", "shield", "flag", "map-pin", "camera", "photo", "video",
    "music", "book", "school", "building", "car", "plane", "bike", "coffee", "battery", "wifi", "bell", "trash", "pencil", "copy", "download", "upload", "link", "printer", "eye",
    "alert-triangle", "info-circle", "help", "compass", "world", "tree", "leaf", "flame", "bolt", "diamond", "trophy", "gift", "shopping-cart", "building-store", "currency-dollar",
    "coin", "credit-card", "rocket", "target", "puzzle", "thumb-up", "message", "send", "share", "bookmark", "tag", "filter", "list", "layout-grid", "device-laptop", "device-mobile",
    "cpu", "database", "code", "brain", "microscope", "flask", "atom", "tool", "hammer", "palette", "ruler", "scissors", "zoom-in", "presentation", "table", "mood-smile",
];

/// Procura ícones por nome, categoria ou palavra-chave, em português ou inglês: «casa»,
/// «seta direita», «chart». Todas as palavras digitadas precisam casar. Sem texto, devolve os mais usados.
pub fn buscar(termo: &str, limite: usize) -> Vec<&'static Icone> {
    let (todos, _) = indice();
    let palavras: Vec<String> = sem_acento(termo).split_whitespace().map(String::from).collect();
    if palavras.is_empty() {
        return POPULARES.iter().filter_map(|id| buscar_por_id(id)).take(limite).collect();
    }
    // Cada palavra vira a lista de alternativas: ela mesma e as traduções.
    let alternativas: Vec<Vec<String>> = palavras
        .iter()
        .map(|p| {
            let mut v = vec![p.clone()];
            v.extend(TRADUCAO.iter().filter(|(pt, _)| pt == p).flat_map(|(_, en)| en.split(',').map(String::from)));
            v
        })
        .collect();
    let mut achados: Vec<(u32, &Icone)> = Vec::new();
    'icones: for ic in todos {
        let mut total = 0;
        for alts in &alternativas {
            let melhor = alts.iter().map(|a| pontos(ic, a)).max().unwrap_or(0);
            if melhor == 0 {
                continue 'icones;
            }
            total += melhor;
        }
        achados.push((total, ic));
    }
    achados.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.id.len().cmp(&b.1.id.len())).then(a.1.id.cmp(b.1.id)));
    achados.into_iter().take(limite).map(|(_, i)| i).collect()
}

/// Quanto o ícone casa com a palavra (0 = não casa): nome igual, começo do nome, parte do nome, palavra-chave, categoria.
fn pontos(ic: &Icone, palavra: &str) -> u32 {
    if ic.id == palavra {
        return 100;
    }
    if ic.id.split('-').any(|p| p == palavra) {
        return if ic.id.starts_with(palavra) { 90 } else { 80 };
    }
    if ic.id.starts_with(palavra) {
        return 70;
    }
    if ic.id.contains(palavra) {
        return 50;
    }
    if ic.tags.split(',').any(|t| t == palavra) {
        return 40;
    }
    if ic.tags.contains(palavra) {
        return 25;
    }
    if sem_acento(ic.categoria).contains(palavra) {
        return 10;
    }
    0
}

/// As categorias do Tabler, em ordem alfabética, com a quantidade de ícones de cada uma.
pub fn categorias() -> Vec<(&'static str, usize)> {
    let mut contagem: HashMap<&'static str, usize> = HashMap::new();
    for ic in todos() {
        *contagem.entry(ic.categoria).or_default() += 1;
    }
    let mut v: Vec<_> = contagem.into_iter().collect();
    v.sort_by(|a, b| a.0.cmp(b.0));
    v
}

/// Os ícones de uma categoria, em ordem alfabética.
pub fn da_categoria(categoria: &str) -> Vec<&'static Icone> {
    todos().iter().filter(|i| i.categoria == categoria).collect()
}

/// Os traços do ícone ajustados ao menor lado da caixa e centrados, na unidade do documento,
/// e a escala usada (lado / 24). O traço original do Tabler tem 2 unidades em 24.
pub fn tracos(ic: &Icone, x: f32, y: f32, largura: f32, altura: f32) -> (Vec<Vec<[f32; 2]>>, f32) {
    let lado = largura.abs().min(altura.abs());
    let (ox, oy) = (x + (largura - lado) / 2.0, y + (altura - lado) / 2.0);
    let escala = lado / 24.0;
    let linhas = ic.caminhos
        .split('|')
        .flat_map(svgpath::tracos)
        .map(|t| t.pontos.iter().map(|p| [ox + p[0] * escala, oy + p[1] * escala]).collect())
        .collect();
    (linhas, escala)
}

/// As figuras do ícone na caixa `r`, com a cor e a espessura relativa do traço
/// (1 = o traço original do Tabler; 0 ou inválido também vale 1).
pub fn figuras(ic: &Icone, r: Retangulo, cor: Cor, espessura: f32) -> Vec<Figura> {
    let (linhas, escala) = tracos(ic, r.x, r.y, r.largura, r.altura);
    let relativa = if espessura > 0.0 && espessura.is_finite() { espessura } else { 1.0 };
    let mut stroke = Stroke::novo(cor, 2.0 * escala * relativa);
    stroke.extremo = Extremo::Redondo;
    stroke.juncao = Juncao::Redonda;
    let estilo = EstiloVetor { preenchimento: None, stroke: Some(stroke), opacidade: 1.0, texto: None };
    linhas.into_iter().filter(|l| l.len() > 1).map(|l| {
        let mut c = Caminho::novo();
        for (i, p) in l.iter().enumerate() {
            let p = Ponto::novo(p[0], p[1]);
            c.adicionar(if i == 0 { Comando::Mover(p) } else { Comando::Linha(p) });
        }
        Figura::Caminho { caminho: c, estilo: estilo.clone() }
    }).collect()
}

/// A `Cena` do ícone `id` (aceita os nomes antigos) na caixa `r`; `None` se não existe.
pub fn cena(id: &str, r: Retangulo, cor: Cor, espessura: f32) -> Option<Cena> {
    let ic = buscar_por_id(id)?;
    let mut cena = Cena::nova((r.x + r.largura).max(0.0), (r.y + r.altura).max(0.0));
    for f in figuras(ic, r, cor, espessura) {
        cena.adicionar(f);
    }
    Some(cena)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn todos_desenham_dentro_do_quadrado() {
        assert!(todos().len() > 5000);
        for ic in todos() {
            let (linhas, escala) = tracos(ic, 0.0, 0.0, 24.0, 24.0);
            assert_eq!(escala, 1.0);
            assert!(!linhas.is_empty(), "{} sem desenho", ic.id);
            for [x, y] in linhas.into_iter().flatten() {
                assert!(x.is_finite() && y.is_finite() && (-1.5..=25.5).contains(&x) && (-1.5..=25.5).contains(&y), "{} sai do quadrado: {x},{y}", ic.id);
            }
        }
    }

    #[test]
    fn busca_em_portugues_e_ingles() {
        let ids = |t: &str| buscar(t, 10).iter().map(|i| i.id).collect::<Vec<_>>();
        assert_eq!(ids("home")[0], "home");
        assert!(ids("casa").contains(&"home"), "{:?}", ids("casa"));
        assert!(ids("coração").contains(&"heart"));
        assert_eq!(ids("seta direita")[0], "arrow-right");
        assert!(ids("RELÓGIO").contains(&"clock"));
        assert!(ids("xyzzyqq").is_empty());
        // Sem texto: os mais usados, em ordem.
        assert_eq!(ids("")[0], "home");
        assert_eq!(buscar("", 5).len(), 5);
    }

    #[test]
    fn nomes_antigos_ainda_abrem() {
        assert_eq!(buscar_por_id("coracao").map(|i| i.id), Some("heart"));
        assert!(!existe("não existe"));
    }

    #[test]
    fn categorias_e_cena_colorida() {
        let cats = categorias();
        assert!(cats.len() >= 30, "{}", cats.len());
        assert_eq!(cats.iter().map(|c| c.1).sum::<usize>(), todos().len());
        assert!(da_categoria(cats[0].0).iter().all(|i| i.categoria == cats[0].0));
        let vermelho = Cor::rgb(255, 0, 0);
        let c = cena("heart", Retangulo::novo(0.0, 0.0, 48.0, 48.0), vermelho, 2.0).unwrap();
        assert!(!c.figuras.is_empty());
        let Figura::Caminho { estilo, .. } = &c.figuras[0] else { panic!() };
        let s = estilo.stroke.as_ref().unwrap();
        assert_eq!((s.cor, s.largura), (vermelho, 8.0));
        assert!(cena("nao-existe", Retangulo::default(), vermelho, 1.0).is_none());
        assert!(feather_draw::exportar_svg(&c).is_ok());
    }
}
