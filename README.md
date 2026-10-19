# feather-shapes

Motor de **formas, ícones e SmartArt 100% Rust** da suíte **FeatherWorks**: as
mesmas formas, os mesmos ícones e os mesmos diagramas no FeatherSheet (forma
solta sobre a planilha), no FeatherDocs (forma no texto) e no FeatherSlide
(forma no slide), com a mesma geometria, a mesma aparência e a mesma
exportação.

É uma **biblioteca**, não um aplicativo e **não tem interface Iced**: cada
aplicativo encaixa a forma no seu documento e a desenha na tela do seu jeito.
A `feather-shapes` calcula a geometria e entrega uma `Cena` do
[`feather-draw`](https://github.com/feather-works/feather-draw), que sai em **tela**, **SVG** e **PDF** a
partir das mesmas formas.

> **Estado do documento.** Este README é a **especificação de suporte**: lista
> todas as formas, todos os ícones e todos os layouts de SmartArt que a
> biblioteca vai ter e todas as formas de personalização. A meta é **100%
> personalizável**: nada de aparência fixa dentro do motor. Cada item traz o
> estado real no código de hoje. Desde a v0.1.0 (14/10/2026) o código vive
> neste repositório, extraído do `featherslide-core` do monorepo FeatherWorks;
> o FeatherSlide já usa a biblioteca e guarda só o que é dele (`Elemento`,
> `Sessao`, desfazer e o conversor para a `Primitiva` de desenho).
>
> | Marca | Significado |
> |---|---|
> | Pronto | existe no código atual e tem teste |
> | Parcial | existe uma parte: a geometria sem ajustes, o conector sem setas, o layout sem todas as variantes |
> | Planejado | ainda não existe |
>
> As colunas `prstGeom` e `dgm` usam os nomes do formato Office Open XML
> (DrawingML) e servem de contrato para importar e exportar `.xlsx`, `.docx`
> e `.pptx`. Nomes em pt-BR seguem o Office em português quando há equivalente.

## Por que existe

Hoje as formas, os ícones e os diagramas vivem dentro do FeatherSlide, presos
ao `Elemento` e à `Sessao` dele. O FeatherSheet e o FeatherDocs precisam das
mesmas coisas, e a tentação seria copiar o código. O resultado seria o que
já aconteceu com a interface: três retângulos arredondados diferentes, três
seletores de ícone, e uma forma que muda de cara ao copiar de um aplicativo
para o outro.

A `feather-shapes` existe para três coisas:

1. **Uma forma, três aplicativos.** O mesmo tipo, os mesmos ajustes e a mesma
   aparência geram o mesmo desenho na planilha, no documento, no slide e no
   PDF impresso.
2. **Personalização total.** Tudo que o Office deixa mudar (e o que ele não
   deixa) é uma propriedade declarativa, salva no arquivo do aplicativo, sem
   depender de código dentro do motor.
3. **Privacidade e independência.** O motor não faz acesso à rede, não tem
   telemetria e não conhece planilha, texto corrido nem slide. Os ícones vêm
   embutidos no pacote; nenhum dado do usuário sai da máquina.

## Decisões de projeto

- **Rust puro**, sem biblioteca em C por trás. A dependência de peso é o
  `feather-draw`, também Rust puro.
- **Sem Iced.** A saída é uma `Cena` (formas vetoriais) mais alças de edição
  como dados. Iced, SVG e PDF são adaptadores.
- **Modelo neutro.** A forma é `tipo + ajustes + aparência + texto`. Não existe
  `Elemento`, `Slide`, `Célula` nem `Parágrafo` aqui: cada aplicativo guarda a
  forma dentro do seu próprio elemento e decide a âncora.
- **O que fica nos aplicativos.** `Elemento`, `Tipo`, `Sessao`/`Mudanca`
  (desfazer e refazer) e o despacho do quadro continuam no FeatherSlide, e
  equivalentes no Sheet e no Docs. A biblioteca devolve valores novos; quem
  desfaz é o aplicativo.
- **Determinismo.** A mesma forma e a mesma aparência produzem a mesma `Cena`
  e o mesmo SVG, byte a byte.
- **Cores só como dados.** O motor não tem cor fixa escondida; paleta, contorno
  e opacidade vêm da aparência (o padrão é a paleta da suíte).
- **Compatibilidade com Office como contrato.** O nome `prstGeom` e a lista de
  ajustes (`avLst`) de cada forma seguem o DrawingML, para ler e gravar sem
  perder a edição.
- **Português do Brasil por padrão**, com nomes de formas e de layouts no idioma
  do Office em português e busca sem acento.
- **Ícones embutidos.** Tabler Icons (MIT), sem baixar nada.

## O que existe hoje (resumo honesto)

Tudo o que está marcado como Pronto abaixo existe neste repositório (os nomes de
arquivo citados são os de origem no `featherslide-core`; hoje `formas.rs` é
`poligonos.rs`, `novas_formas.rs` é `basicas.rs` e `outras_galerias.rs` é
`galeria.rs`):

- **Formas.** 7 básicas (Retângulo, Retângulo arredondado, Elipse, Setor,
  Triângulo, Seta, Linha) mais as listas `ADICIONAIS` (`formas.rs`) e
  `FORMAS` (`outras_galerias.rs`): losango, círculo, anel, arco, corda,
  cubo, cilindro, moldura, canto em L, placa, sol, lua, coração, raio, nuvem,
  estrelas de 4, 6, 8, 10 e 12 pontas, triângulo retângulo, polígonos de 5, 6,
  7, 8, 10 e 12 lados, trapézio, paralelogramo, estrela, cruz, setas,
  chevron e documento; fluxogramas (8), balões (2), faixas e fitas (2), ondas
  e explosões (2), matemáticas (6) e botões de ação (4). Conectores reto,
  cotovelo e curvo que seguem as formas, desenho livre, as 5 operações
  booleanas (União, Interseção, Subtração, Combinação, Fragmentação, até 16
  formas) e WordArt com 4 transformações (Reto, Arco, Onda, Inclinado) e 4 estilos prontos (Sólido, Contornado, Dourado, Rosa; `wordart::ESTILOS`).
  O menu tem 79 entradas.
- **Ajustes.** Só o Setor (ângulos) e os retângulos (raio por canto) leem
  `ajustes`. As demais formas ainda não têm alças.
- **Ícones.** Tabler outline, 5184 ícones em 41 categorias, busca em pt e en
  sem acento, recolorir, espessura do traço e trocar ícone.
- **SmartArt.** 14 categorias de diagramas (Listas, Processos, Ciclos,
  Hierarquias, Organogramas, Relações, Matrizes, Pirâmides, Com imagens,
  Venn, Radiais, Engrenagens, Fluxo, Comparação) feitos de **formas nativas
  com texto**. O editor altera textos, adiciona, exclui e move itens (até 64),
  escolhe o arranjo (Lista, Processo, Ciclo, Organograma) e o nível no
  organograma. Ele **não troca de categoria** e **não é SmartArt OOXML**.
- **Formatar forma.** Preenchimento (nenhum, sólido, gradiente linear e radial
  com paradas, predefinições, inverter, transparência), contorno (mostrar,
  cor, espessura) e sombra (predefinições, cor, desfoque, direção, distância,
  transparência, dx e dy).
- **Sem busca nem recentes** na galeria de formas.

---

## 1. Catálogo de formas

Os Ids `F001` em diante são **estáveis**: nunca se reaproveita nem se
renumera; forma nova entra no fim da sua categoria com Id novo. A coluna
**Ajustes** lista as alças (`adj`) que a forma terá; o estado da coluna
**Estado** vale para a **forma** (geometria desenhada e inserível), não para
os ajustes. Os ajustes só estão prontos onde a célula diz "pronto" (Setor e
retângulos); nas demais formas todos os ajustes são Planejado.

Observações sobre o mapeamento do que existe hoje:

- Os nomes do menu do Slide foram ligados à `prstGeom` mais próxima. Três
  ligações precisam de confirmação no código: "Documento" de `formas.rs`
  (aqui `foldedCorner`), "Armazenamento" de `outras_galerias.rs` (aqui
  `flowChartMagneticDisk`) e "Faixa" e "Fita dobrada" (aqui `ribbon2` e `ribbon`).
- Explosão 1 e a Onda existem como entrada de menu; a segunda explosão e a
  onda dupla não.
- Polígonos de 9 e 11 lados não existem no Office; entram em "Extras" como
  casos fixos do polígono regular de N lados.

### Linhas e conectores

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F001 | Linha | `line` | - | Pronto |
| F002 | Seta de linha (conector reto) | `straightConnector1` | - | Parcial |
| F003 | Conector em cotovelo (2 segmentos) | `bentConnector2` | - | Planejado |
| F004 | Conector em cotovelo | `bentConnector3` | adj1 (posição do dobra) | Parcial |
| F005 | Conector em cotovelo (4 segmentos) | `bentConnector4` | adj1, adj2 | Planejado |
| F006 | Conector em cotovelo (5 segmentos) | `bentConnector5` | adj1, adj2, adj3 | Planejado |
| F007 | Conector curvo (2 segmentos) | `curvedConnector2` | - | Planejado |
| F008 | Conector curvo | `curvedConnector3` | adj1 | Parcial |
| F009 | Conector curvo (4 segmentos) | `curvedConnector4` | adj1, adj2 | Planejado |
| F010 | Conector curvo (5 segmentos) | `curvedConnector5` | adj1, adj2, adj3 | Planejado |

### Retângulos

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F011 | Retângulo | `rect` | - | Pronto |
| F012 | Retângulo de cantos arredondados | `roundRect` | adj (raio por canto: pronto) | Pronto |
| F013 | Retângulo com um canto cortado | `snip1Rect` | adj | Planejado |
| F014 | Retângulo com dois cantos cortados (mesmo lado) | `snip2SameRect` | adj1, adj2 | Planejado |
| F015 | Retângulo com dois cantos cortados (diagonal) | `snip2DiagRect` | adj1, adj2 | Planejado |
| F016 | Retângulo com canto cortado e canto arredondado | `snipRoundRect` | adj1, adj2 | Planejado |
| F017 | Retângulo com um canto arredondado | `round1Rect` | adj | Planejado |
| F018 | Retângulo com dois cantos arredondados (mesmo lado) | `round2SameRect` | adj1, adj2 | Planejado |
| F019 | Retângulo com dois cantos arredondados (diagonal) | `round2DiagRect` | adj1, adj2 | Planejado |

### Formas básicas

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F020 | Elipse | `ellipse` | - | Pronto |
| F021 | Círculo | `ellipse (proporção travada 1:1)` | - | Pronto |
| F022 | Triângulo | `triangle` | adj (posição do vértice) | Pronto |
| F023 | Triângulo retângulo | `rtTriangle` | - | Pronto |
| F024 | Paralelogramo | `parallelogram` | adj | Pronto |
| F025 | Trapézio | `trapezoid` | adj | Pronto |
| F026 | Losango | `diamond` | - | Pronto |
| F027 | Pentágono | `pentagon` | hf, vf | Pronto |
| F028 | Hexágono | `hexagon` | adj, vf | Pronto |
| F029 | Heptágono | `heptagon` | hf, vf | Pronto |
| F030 | Octógono | `octagon` | adj | Pronto |
| F031 | Decágono | `decagon` | vf | Pronto |
| F032 | Dodecágono | `dodecagon` | - | Pronto |
| F033 | Setor (pizza) | `pie` | adj1, adj2 (ângulos: pronto) | Pronto |
| F034 | Corda | `chord` | adj1, adj2 | Pronto |
| F035 | Gota | `teardrop` | adj | Planejado |
| F036 | Moldura | `frame` | adj1 | Pronto |
| F037 | Meia moldura | `halfFrame` | adj1, adj2 | Planejado |
| F038 | Canto em L | `corner` | adj1, adj2 | Pronto |
| F039 | Faixa diagonal | `diagStripe` | adj | Planejado |
| F040 | Cruz | `plus` | adj | Pronto |
| F041 | Placa | `plaque` | adj | Pronto |
| F042 | Cilindro | `can` | adj | Pronto |
| F043 | Cubo | `cube` | adj | Pronto |
| F044 | Bisel | `bevel` | adj | Planejado |
| F045 | Anel | `donut` | adj | Pronto |
| F046 | Proibido | `noSmoking` | adj | Planejado |
| F047 | Arco em bloco | `blockArc` | adj1, adj2, adj3 | Planejado |
| F048 | Canto dobrado (documento) | `foldedCorner` | adj | Pronto |
| F049 | Rosto sorridente | `smileyFace` | adj | Planejado |
| F050 | Coração | `heart` | - | Pronto |
| F051 | Raio | `lightningBolt` | - | Pronto |
| F052 | Sol | `sun` | adj | Pronto |
| F053 | Lua | `moon` | adj | Pronto |
| F054 | Nuvem | `cloud` | - | Pronto |
| F055 | Arco | `arc` | adj1, adj2 | Pronto |
| F056 | Par de colchetes | `bracketPair` | adj | Planejado |
| F057 | Par de chaves | `bracePair` | adj | Planejado |
| F058 | Colchete esquerdo | `leftBracket` | adj | Planejado |
| F059 | Colchete direito | `rightBracket` | adj | Planejado |
| F060 | Chave esquerda | `leftBrace` | adj1, adj2 | Planejado |
| F061 | Chave direita | `rightBrace` | adj1, adj2 | Planejado |
| F062 | Guias de pasta (placa) | `plaqueTabs` | - | Planejado |
| F063 | Guias de pasta (quadrada) | `squareTabs` | - | Planejado |
| F064 | Guias de pasta (canto) | `cornerTabs` | - | Planejado |
| F065 | Engrenagem de 6 dentes | `gear6` | adj1, adj2 | Planejado |
| F066 | Engrenagem de 9 dentes | `gear9` | adj1, adj2 | Planejado |
| F067 | Funil | `funnel` | - | Planejado |
| F068 | Fatia de pizza | `pieWedge` | - | Planejado |
| F069 | Gráfico em cruz | `chartPlus` | - | Planejado |
| F070 | Gráfico em estrela | `chartStar` | - | Planejado |
| F071 | Gráfico em X | `chartX` | - | Planejado |

### Setas largas

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F072 | Seta à direita | `rightArrow` | adj1, adj2 | Pronto |
| F073 | Seta à esquerda | `leftArrow` | adj1, adj2 | Pronto |
| F074 | Seta para cima | `upArrow` | adj1, adj2 | Pronto |
| F075 | Seta para baixo | `downArrow` | adj1, adj2 | Pronto |
| F076 | Seta dupla horizontal | `leftRightArrow` | adj1, adj2 | Pronto |
| F077 | Seta dupla vertical | `upDownArrow` | adj1, adj2 | Planejado |
| F078 | Seta de quatro direções | `quadArrow` | adj1, adj2, adj3 | Planejado |
| F079 | Seta de três direções (esquerda, direita e cima) | `leftRightUpArrow` | adj1, adj2, adj3 | Planejado |
| F080 | Seta dobrada | `bentArrow` | adj1 a adj4 | Planejado |
| F081 | Seta em U | `uturnArrow` | adj1 a adj5 | Planejado |
| F082 | Seta esquerda e para cima | `leftUpArrow` | adj1, adj2, adj3 | Planejado |
| F083 | Seta dobrada para cima | `bentUpArrow` | adj1, adj2, adj3 | Planejado |
| F084 | Seta curva à direita | `curvedRightArrow` | adj1, adj2, adj3 | Planejado |
| F085 | Seta curva à esquerda | `curvedLeftArrow` | adj1, adj2, adj3 | Planejado |
| F086 | Seta curva para cima | `curvedUpArrow` | adj1, adj2, adj3 | Planejado |
| F087 | Seta curva para baixo | `curvedDownArrow` | adj1, adj2, adj3 | Planejado |
| F088 | Seta listrada à direita | `stripedRightArrow` | adj1, adj2 | Planejado |
| F089 | Seta entalhada à direita | `notchedRightArrow` | adj1, adj2 | Planejado |
| F090 | Seta pentagonal | `homePlate` | adj | Planejado |
| F091 | Divisa (chevron) | `chevron` | adj | Pronto |
| F092 | Texto explicativo com seta à direita | `rightArrowCallout` | adj1 a adj4 | Planejado |
| F093 | Texto explicativo com seta para baixo | `downArrowCallout` | adj1 a adj4 | Planejado |
| F094 | Texto explicativo com seta à esquerda | `leftArrowCallout` | adj1 a adj4 | Planejado |
| F095 | Texto explicativo com seta para cima | `upArrowCallout` | adj1 a adj4 | Planejado |
| F096 | Texto explicativo com seta dupla horizontal | `leftRightArrowCallout` | adj1 a adj4 | Planejado |
| F097 | Texto explicativo com seta dupla vertical | `upDownArrowCallout` | adj1 a adj4 | Planejado |
| F098 | Texto explicativo com seta de quatro direções | `quadArrowCallout` | adj1 a adj4 | Planejado |
| F099 | Seta circular | `circularArrow` | adj1 a adj5 | Planejado |
| F100 | Seta circular à esquerda | `leftCircularArrow` | adj1 a adj5 | Planejado |
| F101 | Seta circular dupla | `leftRightCircularArrow` | adj1 a adj5 | Planejado |
| F102 | Seta em pincelada | `swooshArrow` | adj1, adj2 | Planejado |

### Equações

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F103 | Mais | `mathPlus` | adj1 | Pronto |
| F104 | Menos | `mathMinus` | adj1 | Pronto |
| F105 | Multiplicação | `mathMultiply` | adj1 | Pronto |
| F106 | Divisão | `mathDivide` | adj1, adj2, adj3 | Pronto |
| F107 | Igual | `mathEqual` | adj1, adj2 | Pronto |
| F108 | Diferente | `mathNotEqual` | adj1, adj2, adj3 | Pronto |

### Fluxograma

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F109 | Processo | `flowChartProcess` | - | Pronto |
| F110 | Processo alternativo | `flowChartAlternateProcess` | - | Planejado |
| F111 | Decisão | `flowChartDecision` | - | Pronto |
| F112 | Dados (entrada e saída) | `flowChartInputOutput` | - | Pronto |
| F113 | Processo predefinido | `flowChartPredefinedProcess` | - | Pronto |
| F114 | Armazenamento interno | `flowChartInternalStorage` | - | Planejado |
| F115 | Documento | `flowChartDocument` | - | Pronto |
| F116 | Vários documentos | `flowChartMultidocument` | - | Planejado |
| F117 | Terminador (início e fim) | `flowChartTerminator` | - | Pronto |
| F118 | Preparação | `flowChartPreparation` | - | Planejado |
| F119 | Entrada manual | `flowChartManualInput` | - | Planejado |
| F120 | Operação manual | `flowChartManualOperation` | - | Planejado |
| F121 | Conector | `flowChartConnector` | - | Planejado |
| F122 | Conector de outra página | `flowChartOffpageConnector` | - | Pronto |
| F123 | Cartão | `flowChartCard` | - | Planejado |
| F124 | Fita perfurada | `flowChartPunchedTape` | - | Planejado |
| F125 | Junção de soma | `flowChartSummingJunction` | - | Planejado |
| F126 | Ou | `flowChartOr` | - | Planejado |
| F127 | Intercalar | `flowChartCollate` | - | Planejado |
| F128 | Classificar | `flowChartSort` | - | Planejado |
| F129 | Extrair | `flowChartExtract` | - | Planejado |
| F130 | Mesclar | `flowChartMerge` | - | Planejado |
| F131 | Dados armazenados | `flowChartOnlineStorage` | - | Planejado |
| F132 | Atraso | `flowChartDelay` | - | Planejado |
| F133 | Armazenamento de acesso sequencial | `flowChartMagneticTape` | - | Planejado |
| F134 | Disco magnético (armazenamento) | `flowChartMagneticDisk` | - | Pronto |
| F135 | Armazenamento de acesso direto | `flowChartMagneticDrum` | - | Planejado |
| F136 | Exibição | `flowChartDisplay` | - | Planejado |
| F137 | Armazenamento fora de linha | `flowChartOfflineStorage` | - | Planejado |
| F138 | Cartão perfurado | `flowChartPunchedCard` | - | Planejado |

### Estrelas e faixas

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F139 | Explosão 1 | `irregularSeal1` | - | Pronto |
| F140 | Explosão 2 | `irregularSeal2` | - | Planejado |
| F141 | Estrela de 4 pontas | `star4` | adj | Pronto |
| F142 | Estrela de 5 pontas | `star5` | adj, hf, vf | Pronto |
| F143 | Estrela de 6 pontas | `star6` | adj, hf, vf | Pronto |
| F144 | Estrela de 7 pontas | `star7` | adj, hf, vf | Planejado |
| F145 | Estrela de 8 pontas | `star8` | adj | Pronto |
| F146 | Estrela de 10 pontas | `star10` | adj, hf | Pronto |
| F147 | Estrela de 12 pontas | `star12` | adj | Pronto |
| F148 | Estrela de 16 pontas | `star16` | adj | Planejado |
| F149 | Estrela de 24 pontas | `star24` | adj | Planejado |
| F150 | Estrela de 32 pontas | `star32` | adj | Planejado |
| F151 | Fita inclinada para cima | `ribbon2` | adj1, adj2 | Pronto |
| F152 | Fita inclinada para baixo | `ribbon` | adj1, adj2 | Pronto |
| F153 | Fita curva para cima | `ellipseRibbon2` | adj1, adj2, adj3 | Planejado |
| F154 | Fita curva para baixo | `ellipseRibbon` | adj1, adj2, adj3 | Planejado |
| F155 | Pergaminho vertical | `verticalScroll` | adj | Planejado |
| F156 | Pergaminho horizontal | `horizontalScroll` | adj | Planejado |
| F157 | Onda | `wave` | adj1, adj2 | Pronto |
| F158 | Onda dupla | `doubleWave` | adj1, adj2 | Planejado |

### Textos explicativos (balões)

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F159 | Balão retangular | `wedgeRectCallout` | adj1, adj2 | Planejado |
| F160 | Balão retangular arredondado | `wedgeRoundRectCallout` | adj1, adj2, adj3 | Pronto |
| F161 | Balão oval | `wedgeEllipseCallout` | adj1, adj2 | Planejado |
| F162 | Balão de pensamento (nuvem) | `cloudCallout` | adj1, adj2 | Pronto |
| F163 | Texto explicativo em linha 1 | `callout1` | adj1 a adj4 | Planejado |
| F164 | Texto explicativo em linha 2 | `callout2` | adj1 a adj6 | Planejado |
| F165 | Texto explicativo em linha 3 | `callout3` | adj1 a adj8 | Planejado |
| F166 | Texto explicativo com borda e linha 1 | `borderCallout1` | adj1 a adj4 | Planejado |
| F167 | Texto explicativo com borda e linha 2 | `borderCallout2` | adj1 a adj6 | Planejado |
| F168 | Texto explicativo com borda e linha 3 | `borderCallout3` | adj1 a adj8 | Planejado |
| F169 | Texto explicativo com barra e linha 1 | `accentCallout1` | adj1 a adj4 | Planejado |
| F170 | Texto explicativo com barra e linha 2 | `accentCallout2` | adj1 a adj6 | Planejado |
| F171 | Texto explicativo com barra e linha 3 | `accentCallout3` | adj1 a adj8 | Planejado |
| F172 | Texto explicativo com barra, borda e linha 1 | `accentBorderCallout1` | adj1 a adj4 | Planejado |
| F173 | Texto explicativo com barra, borda e linha 2 | `accentBorderCallout2` | adj1 a adj6 | Planejado |
| F174 | Texto explicativo com barra, borda e linha 3 | `accentBorderCallout3` | adj1 a adj8 | Planejado |

### Botões de ação

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F175 | Botão em branco | `actionButtonBlank` | - | Planejado |
| F176 | Botão Início | `actionButtonHome` | - | Pronto |
| F177 | Botão Ajuda | `actionButtonHelp` | - | Planejado |
| F178 | Botão Informação | `actionButtonInformation` | - | Pronto |
| F179 | Botão Avançar ou Próximo | `actionButtonForwardNext` | - | Pronto |
| F180 | Botão Voltar ou Anterior | `actionButtonBackPrevious` | - | Pronto |
| F181 | Botão Fim | `actionButtonEnd` | - | Planejado |
| F182 | Botão Começo | `actionButtonBeginning` | - | Planejado |
| F183 | Botão Retornar | `actionButtonReturn` | - | Planejado |
| F184 | Botão Documento | `actionButtonDocument` | - | Planejado |
| F185 | Botão Som | `actionButtonSound` | - | Planejado |
| F186 | Botão Filme | `actionButtonMovie` | - | Planejado |

### Extras FeatherWorks (sem equivalente no Office)

| Id | Forma (pt-BR) | `prstGeom` (OOXML) | Ajustes (alças) | Estado |
|---|---|---|---|---|
| F187 | Polígono regular de N lados (3 a 64) | `extra:poligono` | lados, giro | Planejado |
| F188 | Polígono regular de 9 lados (eneágono) | `extra:eneagono` | - | Planejado |
| F189 | Polígono regular de 11 lados (hendecágono) | `extra:hendecagono` | - | Planejado |
| F190 | Estrela de N pontas (3 a 64) | `extra:estrela_n` | pontas, raio interno | Planejado |
| F191 | Pílula (cápsula) | `extra:pilula` | - | Planejado |
| F192 | Alvo (círculos concêntricos) | `extra:alvo` | anéis, espessura | Planejado |
| F193 | Espiral | `extra:espiral` | voltas, crescimento | Planejado |
| F194 | Onda senoidal | `extra:onda_senoidal` | ciclos, amplitude | Planejado |
| F195 | Onda quadrada e dente de serra | `extra:onda_pulso` | ciclos, tipo | Planejado |
| F196 | Mira em cruz | `extra:mira` | furo central | Planejado |
| F197 | Seta de cotação (linha de dimensão com texto) | `extra:cotacao` | afastamento, extensão | Planejado |
| F198 | Seta curva de linha (arco com ponta) | `extra:seta_arco` | ângulos, raio | Planejado |
| F199 | Seta de linha dobrada | `extra:seta_dobrada` | pontos de dobra | Planejado |
| F200 | Anel parcial (coroa circular) | `extra:coroa` | ângulos, espessura | Planejado |
| F201 | Meia-lua ao centro | `extra:crescente` | fase | Planejado |
| F202 | Cruz de Malta | `extra:cruz_malta` | - | Planejado |
| F203 | Hexágono de colmeia (grade) | `extra:colmeia` | linhas, colunas | Planejado |
| F204 | Seta tripla (chevrons em sequência) | `extra:chevrons` | quantidade, passo | Planejado |
| F205 | Resistor (símbolo técnico) | `extra:resistor` | - | Planejado |
| F206 | Capacitor (símbolo técnico) | `extra:capacitor` | - | Planejado |
| F207 | Indutor (símbolo técnico) | `extra:indutor` | voltas | Planejado |
| F208 | Aterramento (símbolo técnico) | `extra:terra` | - | Planejado |

### Resumo do catálogo de formas

| Categoria | Formas |
|---|---|
| Linhas e conectores | 10 |
| Retângulos | 9 |
| Formas básicas | 52 |
| Setas largas | 31 |
| Equações | 6 |
| Fluxograma | 30 |
| Estrelas e faixas | 20 |
| Textos explicativos (balões) | 16 |
| Botões de ação | 12 |
| Extras FeatherWorks (sem equivalente no Office) | 22 |
| **Total** | **208** |

| Estado | Formas |
|---|---|
| Pronto | 68 |
| Parcial | 3 |
| Planejado | 137 |
| **Total** | **208** |

### 1.1 Conectores

| Id | Recurso | Estado |
|---|---|---|
| C01 | Conector reto, cotovelo e curvo ligado a duas formas, que segue as formas ao mover | Pronto |
| C02 | Pontos de conexão por forma (4 a 8, conforme a `prstGeom`) | Parcial |
| C03 | Conector desfaz junto com a forma (um único passo de desfazer) | Pronto |
| C04 | Setas e pontas nas extremidades do conector (13 tipos, tamanho) | Planejado |
| C05 | Reencaminhar automaticamente em torno de outras formas | Planejado |
| C06 | Ponto de ancoragem escolhido pelo usuário | Planejado |
| C07 | Conector que parte de uma forma e termina livre | Planejado |
| C08 | Ajuste do dobra do conector em cotovelo e da curvatura | Planejado |
| C09 | Rótulo de texto no meio do conector | Planejado |
| C10 | Salto de linha ao cruzar outra linha (ponte) | Planejado |

### 1.2 Desenho livre e edição de pontos

| Id | Recurso | Estado |
|---|---|---|
| D01 | Desenho livre (caminho) importado e inserido | Pronto |
| D02 | Caminho SVG (`svgpath.rs`) lido para a geometria | Pronto |
| D03 | Curva de Bézier (ferramenta: clique e arraste) | Planejado |
| D04 | Forma livre (polígono clicado ponto a ponto) | Planejado |
| D05 | Rabisco (traço à mão livre com suavização) | Planejado |
| D06 | Caneta com pressão (largura variável, `feather-ui`/tinta) | Planejado |
| D07 | Editar pontos: mover nó, adicionar, excluir | Planejado |
| D08 | Tipo de nó: canto, liso, simétrico; alças de tangente | Planejado |
| D09 | Abrir e fechar caminho; juntar e dividir | Planejado |
| D10 | Simplificar caminho (menos nós) | Planejado |
| D11 | Converter qualquer forma em caminho editável | Parcial |
| D12 | Borracha de pontos e de trechos | Planejado |

### 1.3 Operações booleanas

| Id | Operação | Estado |
|---|---|---|
| B01 | União | Pronto |
| B02 | Interseção | Pronto |
| B03 | Subtração | Pronto |
| B04 | Combinação (união com o furo da sobreposição) | Pronto |
| B05 | Fragmentação | Pronto |
| B06 | Até 16 formas na mesma operação | Pronto |
| B07 | Operação sobre formas com curvas preservadas (hoje a geometria sai como polígono) | Planejado |
| B08 | Resultado herdar a aparência da primeira forma ou da escolhida | Planejado |
| B09 | Contorno deslocado (offset, para dentro e para fora) | Planejado |

### 1.4 Agrupar, alinhar, ordem e transformações

| Id | Recurso | Estado |
|---|---|---|
| G01 | Agrupar e desagrupar | Pronto |
| G02 | Alinhar à página (esquerda, centro, direita, topo, meio, base) | Pronto |
| G03 | Alinhar entre si | Pronto |
| G04 | Distribuir na horizontal e na vertical | Pronto |
| G05 | Girar por ângulo (campo e alça) | Pronto |
| G06 | Girar 90 graus e espelhar na horizontal e na vertical | Planejado |
| G07 | Trazer para a frente, enviar para trás, avançar, recuar | Planejado |
| G08 | Grupos aninhados com nome e painel de seleção | Planejado |
| G09 | Distribuir por espaçamento exato e igualar tamanho | Planejado |
| G10 | Duplicar com deslocamento e repetição (matriz de cópias) | Planejado |
| G11 | Girar em torno de um ponto (pivô) | Planejado |

---

## 2. Ícones

Biblioteca **Tabler Icons** (MIT) embutida no pacote, dados em
`dados/tabler-outline.tsv` (atribuição em `dados/TABLER-LICENSE.txt`).

| Id | Recurso | Detalhe | Estado |
|---|---|---|---|
| I01 | Tabler outline | 5184 ícones | Pronto |
| I02 | Categorias | 41 categorias | Pronto |
| I03 | Busca em português e inglês | sem acento, sem diferenciar maiúscula | Pronto |
| I04 | Recolorir | cor sólida | Pronto |
| I05 | Espessura do traço | em pontos | Pronto |
| I06 | Trocar o ícone mantendo posição, tamanho, cor e traço | | Pronto |
| I07 | Tabler filled | variante preenchida (cerca de 1000 ícones) | Planejado |
| I08 | Recentes | últimos ícones usados, por usuário | Planejado |
| I09 | Favoritos | marcar e fixar no topo | Planejado |
| I10 | Ícones do usuário | importar SVG (um ou uma pasta), com validação e limpeza | Planejado |
| I11 | Pacotes de ícones extras | importar um `.fico` (conjunto nomeado com licença) | Planejado |
| I12 | Converter ícone em forma editável | traços viram caminho; cada parte vira forma | Planejado |
| I13 | Cor por parte do ícone (duas cores) | | Planejado |
| I14 | Gradiente e contorno no ícone | | Planejado |
| I15 | Busca por sinônimo e por marcadores (tags) | | Planejado |
| I16 | Pré-visualização em vários tamanhos | | Planejado |
| I17 | Arrastar para a página; inserir vários de uma vez | | Planejado |
| I18 | Exportar como SVG, PDF e PNG | | Parcial |

Há também a galeria de formas: **busca por nome** (pt-BR e inglês) e
**recentes** hoje não existem (Planejado).

---

## 3. SmartArt (diagramas)

O estado atual são **formas nativas com texto** arranjadas por regra; não é
SmartArt OOXML e não troca de categoria. O objetivo é cobrir os layouts mais
usados de cada **família do Office**, ligados às categorias atuais, e
exportar como SmartArt de verdade (`dgm`) além de importá-lo como formas
agrupadas.

Famílias do Office e categorias atuais do FeatherSlide:

| Família do Office | Categorias atuais do Slide |
|---|---|
| Lista | Listas |
| Processo | Processos, Fluxo |
| Ciclo | Ciclos |
| Hierarquia | Hierarquias, Organogramas |
| Relação | Relações, Venn, Radiais, Engrenagens, Comparação |
| Matriz | Matrizes |
| Pirâmide | Pirâmides |
| Imagem | Com imagens |

Marcação: **Parcial** = existe layout equivalente nas categorias atuais (sem
todas as variantes); **Planejado** = ainda não existe. A coluna `dgm` traz o
nome do layout no Office Open XML e **precisa ser conferida** com a lista
oficial antes de implementar a importação.

### Família Lista (categoria atual: Listas)

| Id | Layout (pt-BR) | Layout OOXML (`dgm`, a confirmar) | Estado |
|---|---|---|---|
| D001 | Lista de blocos básica | `default` | Parcial |
| D002 | Lista vertical básica | `vList2` | Parcial |
| D003 | Lista de marcadores verticais | `vList5` | Planejado |
| D004 | Lista de caixa vertical | `list1` | Planejado |
| D005 | Lista de colunas | `hList1` | Planejado |
| D006 | Lista de caixas empilhadas | `vList3` | Planejado |
| D007 | Lista de blocos horizontal | `hList3` | Planejado |
| D008 | Lista de pilha | `lProcess2` | Planejado |
| D009 | Lista de cartões | `hList6` | Planejado |
| D010 | Lista de guias | `hList2` | Planejado |
| D011 | Lista de zigue-zague | `hList7` | Planejado |
| D012 | Lista de destaques | `vList6` | Planejado |
| D013 | Lista trapezoidal | `hList9` | Planejado |
| D014 | Lista de marcadores com imagens | `vList4` | Planejado |
| D015 | Lista de painéis | `hList8` | Planejado |

### Família Processo (categoria atual: Processos)

| Id | Layout (pt-BR) | Layout OOXML (`dgm`, a confirmar) | Estado |
|---|---|---|---|
| D016 | Processo básico | `process1` | Parcial |
| D017 | Processo de seta contínua | `hProcess3` | Planejado |
| D018 | Processo de seta de divisa | `chevron1` | Parcial |
| D019 | Processo de divisas ascendente | `chevron2` | Planejado |
| D020 | Processo vertical | `process2` | Parcial |
| D021 | Processo de blocos contínuos | `hProcess9` | Planejado |
| D022 | Processo de etapas | `hProcess4` | Planejado |
| D023 | Processo em seta ascendente | `arrow2` | Planejado |
| D024 | Processo de lista de divisa | `lProcess3` | Planejado |
| D025 | Processo de zigue-zague | `process5` | Planejado |
| D026 | Processo de linha do tempo | `hProcess11` | Planejado |
| D027 | Processo de curva | `process4` | Planejado |
| D028 | Processo repetido | `process3` | Planejado |
| D029 | Processo acentuado | `hProcess1` | Planejado |
| D030 | Processo de etapas de escada | `StepUpProcess` | Planejado |
| D031 | Processo de seta dividida | `hProcess6` | Planejado |
| D032 | Processo de rotação | `process6` | Planejado |

### Família Ciclo (categoria atual: Ciclos)

| Id | Layout (pt-BR) | Layout OOXML (`dgm`, a confirmar) | Estado |
|---|---|---|---|
| D033 | Ciclo básico | `cycle2` | Parcial |
| D034 | Ciclo de texto | `cycle1` | Planejado |
| D035 | Ciclo contínuo em bloco | `cycle8` | Planejado |
| D036 | Ciclo de seta circular | `cycle5` | Planejado |
| D037 | Ciclo de engrenagem | `gear1` | Parcial |
| D038 | Ciclo radial | `radial3` | Planejado |
| D039 | Ciclo segmentado | `cycle3` | Planejado |
| D040 | Ciclo de múltiplas direções | `cycle7` | Planejado |
| D041 | Ciclo de rosca | `cycle4` | Planejado |
| D042 | Ciclo de setas dispersas | `cycle6` | Planejado |

### Família Hierarquia (categoria atual: Hierarquias e organogramas)

| Id | Layout (pt-BR) | Layout OOXML (`dgm`, a confirmar) | Estado |
|---|---|---|---|
| D043 | Organograma | `orgChart1` | Parcial |
| D044 | Hierarquia | `hierarchy1` | Parcial |
| D045 | Hierarquia horizontal | `hierarchy3` | Planejado |
| D046 | Organograma de nome e cargo | `hierarchy2` | Planejado |
| D047 | Organograma de meio-círculo | `hierarchy5` | Planejado |
| D048 | Hierarquia em tabela | `hierarchy4` | Planejado |
| D049 | Hierarquia de rótulos | `hierarchy6` | Planejado |
| D050 | Organograma horizontal de vários níveis | `hierarchy3` | Planejado |
| D051 | Hierarquia em árvore invertida | `hierarchy1` | Planejado |
| D052 | Hierarquia de painel de lista | `hierarchy1` | Planejado |

### Família Relação (categoria atual: Relações, Venn, radiais, engrenagens e comparação)

| Id | Layout (pt-BR) | Layout OOXML (`dgm`, a confirmar) | Estado |
|---|---|---|---|
| D053 | Venn básico | `venn1` | Parcial |
| D054 | Venn linear | `venn2` | Planejado |
| D055 | Venn empilhado | `venn3` | Planejado |
| D056 | Venn segmentado | `venn1` | Planejado |
| D057 | Radial básico | `radial1` | Parcial |
| D058 | Radial em cluster | `radial5` | Planejado |
| D059 | Cluster radial dividido | `radial2` | Planejado |
| D060 | Radial de cunha | `radial6` | Planejado |
| D061 | Alvo básico | `target3` | Planejado |
| D062 | Alvo de setas | `target1` | Planejado |
| D063 | Alvo de funil | `funnel1` | Planejado |
| D064 | Funil básico | `funnel1` | Planejado |
| D065 | Equilíbrio | `balance1` | Planejado |
| D066 | Balança de comparação | `balance1` | Planejado |
| D067 | Engrenagens | `gear1` | Parcial |
| D068 | Setas opostas | `opposingArrows` | Planejado |
| D069 | Setas convergentes | `convergingArrows` | Planejado |
| D070 | Setas divergentes | `divergingArrows` | Planejado |
| D071 | Relação de contraponto | `hierarchy2` | Planejado |
| D072 | Lista de relação | `hList1` | Planejado |
| D073 | Comparação de duas colunas | `hierarchy2` | Parcial |
| D074 | Relação de agrupamento | `default` | Planejado |
| D075 | Relação de mais e menos | `default` | Planejado |
| D076 | Fluxo de relação de ciclo | `cycle2` | Planejado |

### Família Matriz (categoria atual: Matrizes)

| Id | Layout (pt-BR) | Layout OOXML (`dgm`, a confirmar) | Estado |
|---|---|---|---|
| D077 | Matriz básica | `matrix3` | Parcial |
| D078 | Matriz com títulos | `matrix1` | Parcial |
| D079 | Matriz de grade | `matrix2` | Planejado |
| D080 | Matriz de cartões | `matrix4` | Planejado |

### Família Pirâmide (categoria atual: Pirâmides)

| Id | Layout (pt-BR) | Layout OOXML (`dgm`, a confirmar) | Estado |
|---|---|---|---|
| D081 | Pirâmide básica | `pyramid1` | Parcial |
| D082 | Pirâmide invertida | `pyramid2` | Planejado |
| D083 | Pirâmide segmentada | `pyramid3` | Planejado |
| D084 | Pirâmide de lista | `pyramid4` | Planejado |
| D085 | Pirâmide cônica | `pyramid5` | Planejado |
| D086 | Pirâmide com texto lateral | `pyramid4` | Planejado |

### Família Imagem (categoria atual: Com imagens)

| Id | Layout (pt-BR) | Layout OOXML (`dgm`, a confirmar) | Estado |
|---|---|---|---|
| D087 | Imagens e legendas | `pList1` | Parcial |
| D088 | Lista de imagens circulares | `pList2` | Planejado |
| D089 | Imagens em destaque | `pictureOrgChart` | Planejado |
| D090 | Círculos de imagens | `pictureAccentList` | Planejado |
| D091 | Lista de blocos com imagem | `pictureCaptionList` | Planejado |
| D092 | Organograma com imagens | `pictureOrgChart` | Planejado |
| D093 | Imagens em mosaico | `pictureGrid` | Planejado |
| D094 | Moldura de imagens | `framedTextPicture` | Planejado |
| D095 | Imagens em tira de filme | `filmstripList` | Planejado |

**Total de layouts: 95** em 8 famílias (Pronto: 0; Parcial: 17; Planejado: 78).

### 3.1 Editor de SmartArt

| Id | Recurso | Estado |
|---|---|---|
| E01 | Editar o texto de cada item | Pronto |
| E02 | Adicionar, excluir e mover itens (até 64) | Pronto |
| E03 | Arranjo: Lista, Processo, Ciclo, Organograma | Pronto |
| E04 | Nível do item no organograma | Pronto |
| E05 | Painel de texto com recuo por nível (Tab e Shift+Tab) | Planejado |
| E06 | Promover e rebaixar item | Planejado |
| E07 | Adicionar item acima, abaixo, depois, assistente (organograma) | Planejado |
| E08 | Trocar de layout mantendo o texto e a estrutura | Planejado |
| E09 | Trocar de categoria (hoje o diagrama fica na categoria em que nasceu) | Planejado |
| E10 | Estilos do SmartArt (cores do tema, 3D plano, bisel) | Planejado |
| E11 | Esquemas de cores (coloridos, tons de um acento, gradiente) | Planejado |
| E12 | Formatar uma forma individual dentro do diagrama | Planejado |
| E13 | Inverter da direita para a esquerda | Planejado |
| E14 | Imagens dentro dos itens e legendas | Planejado |
| E15 | Converter para formas (desagrupar o diagrama) | Parcial |
| E16 | Converter texto em SmartArt (lista de tópicos vira diagrama) | Planejado |
| E17 | Redefinir o desenho (voltar ao arranjo automático) | Planejado |
| E18 | Dimensionar texto automaticamente para caber | Planejado |
| E19 | Animar por item (para o FeatherSlide) | Planejado |
| E20 | Texto alternativo do diagrama | Planejado |

---

## 4. Personalização: 100% das propriedades

A regra é simples: **tudo que aparece na forma tem propriedade**. Nenhum valor
de aparência fica dentro do código de uma forma. As propriedades são agrupadas
por elemento e endereçáveis por caminho (`forma.contorno.tracejado`,
`forma.efeitos.sombra.desfoque`), o que permite aplicar modelos, herdar do
tema e sobrepor só o necessário.

Regras gerais:

- **Herança em três níveis**: padrão do tema, forma padrão do usuário, valor da
  forma. O que não for definido herda do nível de cima.
- **Valores validados** (finitos, dentro de faixas) com erro tipado; nunca
  pânico, nunca NaN na `Cena`.
- **Serializáveis**: cada propriedade tem representação estável para os
  arquivos `.fshz`, `.fdoz` e `.fsdz`. Campo novo entra com valor padrão e
  `serde(default)`, para o arquivo antigo abrir.
- **Unidades**: tamanhos e contornos em pontos; margens e posições aceitam
  também centímetros, milímetros e polegadas na interface (o motor guarda em
  pontos).
- Propriedades de uma **forma** valem também para o **grupo**, o conector e o
  caminho, exceto onde a tabela disser o contrário (imagem, ícone, tabela e
  gráfico não aceitam preenchimento de forma; hoje `aceita(tipo)` recusa
  esses tipos).

### 5.1 Geometria

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P001 | Tamanho (largura e altura) | Elemento.largura e altura, em pontos | Pronto |
| P002 | Posição (X e Y) | coordenadas do quadro do app | Pronto |
| P003 | Giro | graus, sentido horário | Pronto |
| P004 | Espelhar na horizontal e na vertical | sem alterar o texto | Planejado |
| P005 | Ajustes da forma (alças amarelas) | um valor por ajuste; hoje só Setor e retângulos | Parcial |
| P006 | Raio por canto nos retângulos | quatro raios independentes | Pronto |
| P007 | Proporção travada ao redimensionar | com Shift ou por propriedade | Planejado |
| P008 | Tamanho e posição por unidade | pt, cm, mm, pol, relativo ao quadro | Planejado |
| P009 | Editar pontos (nós de Bézier) | mover, adicionar, excluir, tipo de nó (liso, simétrico, canto) | Planejado |
| P010 | Converter forma em caminho editável | preservando o desenho | Parcial |
| P011 | Ponto de giro (pivô) livre | fora do centro | Planejado |
| P012 | Redimensionar a partir do centro | Alt | Planejado |
| P013 | Ajuste fino com teclado | passo configurável | Planejado |
| P014 | Grade e guias inteligentes | encaixe nas bordas, centros e distâncias iguais | Planejado |

### 5.2 Preenchimento

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P015 | Sem preenchimento |  | Pronto |
| P016 | Cor sólida | RGB, hexadecimal, tema | Pronto |
| P017 | Transparência do preenchimento | 0 a 100% | Pronto |
| P018 | Gradiente linear com paradas | ângulo e paradas de cor | Pronto |
| P019 | Gradiente radial com paradas | centro e raio | Pronto |
| P020 | Gradiente retangular | a partir de um canto ou do centro | Planejado |
| P021 | Gradiente de caminho (segue o contorno) | paradas | Planejado |
| P022 | Número ilimitado de paradas do gradiente | hoje: lista de pontos de cor (limite a confirmar) | Parcial |
| P023 | Transparência por parada | alfa em cada parada | Planejado |
| P024 | Predefinições de gradiente e inverter |  | Pronto |
| P025 | Preenchimento com imagem | esticar, lado a lado, ajustar, deslocamento, escala | Planejado |
| P026 | Preenchimento com textura | biblioteca de texturas embutidas | Planejado |
| P027 | Preenchimento com padrão (hachura) | 48 padrões, cores de frente e de fundo | Planejado |
| P028 | Preenchimento do fundo (slide) | herdar do tema | Planejado |
| P029 | Girar o preenchimento com a forma |  | Planejado |
| P030 | Cor do tema com matiz, saturação e brilho | derivadas do tema do documento | Planejado |
| P031 | Conta-gotas | pegar cor da tela | Planejado |

### 5.3 Contorno

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P032 | Mostrar ou ocultar |  | Pronto |
| P033 | Cor sólida |  | Pronto |
| P034 | Espessura | em pontos | Pronto |
| P035 | Transparência do contorno |  | Planejado |
| P036 | Contorno com gradiente |  | Planejado |
| P037 | Tracejado | contínuo, ponto, traço, traço-ponto, personalizado por lista de comprimentos | Planejado |
| P038 | Tipo de junção | arredondada, chanfrada, pontiaguda (com limite) | Planejado |
| P039 | Tipo de ponta (cap) | plana, arredondada, quadrada | Planejado |
| P040 | Seta de início | tipo (13), largura e comprimento | Planejado |
| P041 | Seta de fim | tipo (13), largura e comprimento | Planejado |
| P042 | Contorno composto (duplo, triplo) | linhas paralelas com espaço | Planejado |
| P043 | Alinhamento do contorno | dentro, centro ou fora da borda | Planejado |
| P044 | Contorno com padrão ou imagem |  | Planejado |

### 5.4 Efeitos

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P045 | Sombra externa: cor, desfoque, deslocamento X e Y | predefinições | Pronto |
| P046 | Sombra externa: direção, distância e transparência |  | Pronto |
| P047 | Sombra externa: tamanho e inclinação |  | Planejado |
| P048 | Sombra interna |  | Planejado |
| P049 | Sombra em perspectiva |  | Planejado |
| P050 | Brilho (glow) | raio e cor | Planejado |
| P051 | Bordas suaves | raio | Planejado |
| P052 | Reflexo | tamanho, distância, transparência | Planejado |
| P053 | Bisel | superior e inferior, largura e altura | Planejado |
| P054 | Formato 3D: profundidade e contorno |  | Planejado |
| P055 | Rotação 3D (X, Y, Z) e perspectiva |  | Planejado |
| P056 | Material e iluminação 3D |  | Planejado |
| P057 | Opacidade geral do elemento | aparência.opacidade | Pronto |
| P058 | Desfoque do elemento |  | Planejado |
| P059 | Modo de mesclagem | multiplicar, tela, sobrepor | Planejado |
| P060 | Recorte por máscara de outra forma |  | Planejado |

### 5.5 Texto na forma

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P061 | Texto dentro da forma | conteúdo livre | Pronto |
| P062 | Fonte, tamanho, cor, negrito, itálico, sublinhado |  | Pronto |
| P063 | Espaçamento entre letras |  | Pronto |
| P064 | Alinhamento horizontal |  | Pronto |
| P065 | Alinhamento vertical | topo, meio, base | Parcial |
| P066 | Margens internas | por lado | Planejado |
| P067 | Direção do texto | horizontal, vertical, girado 90 ou 270, empilhado | Planejado |
| P068 | Ajuste automático | reduzir texto, ajustar forma ao texto, sem ajuste | Planejado |
| P069 | Quebra de linha na forma | ativa ou desativa | Planejado |
| P070 | Colunas de texto | número e espaçamento | Planejado |
| P071 | Texto como WordArt (transformação) | Reto, Arco, Onda, Inclinado | Pronto |
| P072 | WordArt: preenchimento, contorno e efeitos do texto | gradiente, imagem, sombra, brilho, reflexo | Planejado |
| P073 | WordArt: mais transformações | círculo, ponte, funil, cascata, ondas e as 40 do Office | Planejado |
| P074 | Texto em caminho (acompanha uma curva) |  | Planejado |
| P075 | Retângulo de texto personalizado por forma | área de texto da prstGeom | Planejado |
| P076 | Caracteres de lista e recuo |  | Planejado |

### 5.6 Estilos, temas e padrões

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P077 | Estilos rápidos de forma | grade com 42 estilos do tema | Planejado |
| P078 | Cores do tema do documento | a forma segue o tema | Planejado |
| P079 | Predefinições de preenchimento e sombra |  | Pronto |
| P080 | Definir como forma padrão | novas formas herdam aparência e texto | Planejado |
| P081 | Estilos salvos pelo usuário | nome, exportar e importar | Planejado |
| P082 | Copiar formatação (pincel) | uma vez ou fixa | Planejado |
| P083 | Colar formatação especial | só preenchimento, só contorno etc. | Planejado |
| P084 | Modelo de forma (.fshape) | todas as propriedades sem a geometria | Planejado |

### 5.7 Acessibilidade

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P085 | Texto alternativo | título e descrição | Pronto |
| P086 | Marcar como decorativo | fora da leitura em voz alta | Planejado |
| P087 | Ordem de leitura | separada da ordem de empilhamento | Planejado |
| P088 | Verificação de contraste texto e fundo | aviso abaixo de 4,5:1 | Planejado |
| P089 | Nome da forma | painel de seleção | Planejado |

### 5.8 Organização e bloqueio

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P090 | Agrupar e desagrupar |  | Pronto |
| P091 | Ordem de empilhamento (frente, trás, avançar, recuar) |  | Planejado |
| P092 | Alinhar à página e entre si |  | Pronto |
| P093 | Distribuir na horizontal e na vertical |  | Pronto |
| P094 | Bloquear posição e tamanho |  | Planejado |
| P095 | Bloquear proporção |  | Planejado |
| P096 | Bloquear só o texto, só o giro, só a seleção | permissões finas | Planejado |
| P097 | Ocultar e mostrar (painel de seleção) |  | Planejado |
| P098 | Cópia como formas, como imagem ou como SVG |  | Planejado |

### 5.9 Âncora e posicionamento

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P099 | Âncora no slide (FeatherSlide) | coordenadas do slide | Pronto |
| P100 | Âncora na célula (FeatherSheet) | canto superior esquerdo e inferior direito, mover e dimensionar com as células | Planejado |
| P101 | Opção de mover sem dimensionar com as células |  | Planejado |
| P102 | Âncora no parágrafo, no caractere ou na página (FeatherDocs) |  | Planejado |
| P103 | Flutuante com quebra de texto (FeatherDocs) | em linha, quadrado, justo, acima e abaixo, atrás, na frente | Planejado |
| P104 | Distância do texto em volta | por lado | Planejado |

### 5.10 Interação e animação

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P105 | Hiperlink | endereço na forma | Pronto |
| P106 | Ação ao clicar | ir ao slide, abrir arquivo, executar macro, som | Parcial |
| P107 | Dica de tela | texto ao passar o mouse | Planejado |
| P108 | Animação de entrada | Aparecer, Esmaecer, Voar, Zoom, Revelar | Pronto |
| P109 | Animação de ênfase | Pulsar | Pronto |
| P110 | Duração, atraso e direção |  | Pronto |
| P111 | Animação de saída e trajetória |  | Planejado |
| P112 | Mais efeitos de entrada e ênfase | girar, rebater, dividir, roleta | Planejado |
| P113 | Gatilhos e sequência (com anterior, após anterior) | início da animação | Parcial |
| P114 | Animar por letra, palavra ou parágrafo |  | Planejado |

### 5.11 Imagem e ícone dentro da forma

| Id | Propriedade | Opções | Estado |
|---|---|---|---|
| P115 | Recortar imagem em uma forma | qualquer forma do catálogo | Parcial |
| P116 | Recolorir ícone | cor sólida | Pronto |
| P117 | Espessura do traço do ícone |  | Pronto |
| P118 | Trocar o ícone mantendo estilo |  | Pronto |
| P119 | Ícone com gradiente ou contorno separado |  | Planejado |

### Resumo das propriedades

| Estado | Propriedades |
|---|---|
| Pronto | 34 |
| Parcial | 7 |
| Planejado | 78 |
| **Total** | **119** |

---

## 5. Contrato com os aplicativos

A biblioteca **não conhece o documento**. O aplicativo entrega a forma como
dados e recebe de volta geometria e alças:

```text
Forma         tipo (Id da seção 1), ajustes[], aparencia, texto?, grupo[]?
Aparencia     preenchimento, contorno, efeitos, opacidade
Texto         conteúdo, estilo, margens, ajuste, direção
Cena          (feather-draw) formas vetoriais prontas para tela, SVG e PDF
Alca          posição na cena, tipo (canto, lado, giro, ajuste N), cursor
```

O que a biblioteca entrega a cada aplicativo:

| Entrega | Para quê | Estado |
|---|---|---|
| `Forma::cena(...)` | desenho na tela, no SVG e no PDF | Pronto (`Forma::cena` e `Forma::figuras`; o Slide segue desenhando com a `Primitiva` própria) |
| Conversor `Cena` para `Primitiva` | o Slide usar a biblioteca sem trocar o desenho | Pronto (`featherslide_core::formas_comuns`, da `Geometria` para `Primitiva`) |
| `Forma::acerto(x, y, tolerancia)` | teste de clique (dentro do preenchimento, sobre o contorno, sobre o texto) | Parcial (preenchimento com furos, contorno e giro prontos; sobre o texto, planejado) |
| `Forma::alcas(...)` | alças de redimensionar, girar e ajustar, como dados para a GUI | Planejado |
| Ajustes validados | `ajustar(n, valor)` com limite por forma | Parcial (Setor e retângulos) |
| Caixa de texto interna | retângulo onde o texto da forma cabe (`rect` do DrawingML) | Planejado |
| Pontos de conexão | para os conectores | Parcial |
| Operações booleanas | recebe formas, devolve caminho(s) | Pronto |
| Ícones | busca, categorias e SVG do ícone | Pronto |
| Diagramas | `Diagrama` (itens e níveis) para formas agrupadas | Pronto (4 arranjos) |
| Serialização | `to_bytes` e `from_bytes` estáveis, versionadas, para `.fshz`, `.fdoz` e `.fsdz` | Planejado |
| Miniatura | `Cena` de pré-visualização para galeria e estilos | Parcial |

O que fica **no aplicativo**:

| Aplicativo | Âncora e documento | Desfazer |
|---|---|---|
| FeatherSheet | célula inicial e final (mover e dimensionar com as células), camada de desenho da aba | histórico do Sheet |
| FeatherDocs | em linha, no parágrafo, flutuante com quebra, página | histórico do Docs |
| FeatherSlide | `Elemento` no slide, animação, ação ao clicar | `Sessao` e `Mudanca` |

Regras do contrato:

- A mesma `Forma` com os mesmos dados gera sempre a mesma `Cena`.
- Nenhuma operação altera a `Forma` de entrada; sempre devolve valor novo.
- Recursos de edição (alças, acerto) trabalham em coordenadas do documento do
  aplicativo, sem presumir DPI.
- Formas desconhecidas lidas de arquivos do Office nunca são descartadas:
  guarda-se o XML original e desenha-se um substituto.

---

## 6. Importação e exportação

| Formato | Direção | O que carrega | Estado |
|---|---|---|---|
| **.fsdz** (FeatherSlide) | ler e gravar | formas, ajustes, aparência, ícones, diagramas | Pronto (formato do próprio Slide, que a biblioteca passará a ler e gravar) |
| **.fshz** (FeatherSheet) | ler e gravar | formas sobre a planilha com âncora na célula | Planejado |
| **.fdoz** (FeatherDocs) | ler e gravar | formas no texto, em linha ou flutuantes | Planejado |
| **.pptx** | importar | `p:sp` com `a:prstGeom` e `a:custGeom`, `p:cxnSp` (conectores), `p:grpSp` (grupos) | Parcial (o Slide abre formas básicas) |
| **.pptx** | exportar | formas com `prstGeom`, ajustes (`a:avLst`), preenchimento, contorno, efeitos | Planejado |
| **.docx** | importar e exportar | `wps:wsp` dentro de `mc:AlternateContent` e `w:drawing` | Planejado |
| **.xlsx** | importar e exportar | `xdr:sp`, `xdr:cxnSp`, `xdr:grpSp` em `xl/drawings/drawingN.xml`, com `twoCellAnchor` | Planejado |
| **SmartArt OOXML (`dgm`)** | importar | `dgm:dataModel`, `dgm:layoutDef`, `dgm:colorsDef`, `dgm:styleDef`; lido como **formas agrupadas** (usando o `drawing` de reserva do arquivo quando existir) | Planejado |
| **SmartArt OOXML (`dgm`)** | exportar | modelo de dados + layout equivalente + desenho de reserva | Planejado |
| **.odp, .ods, .odg, .odt** | importar e exportar | `draw:custom-shape`, `draw:g`, `draw:connector`, `draw:path` | Planejado |
| **SVG** | exportar | formas e ícones, determinístico; grupos nomeados | Parcial (ícone e caminho; forma completa Planejado) |
| **SVG** | importar | como ícone do usuário (caminhos simples) ou forma editável | Planejado |
| **PDF** | exportar | PDF 1.4 vetorial pelas mesmas formas | Pronto (pela `Cena` e pelo `feather-draw`) |
| **PNG** | exportar | rasterização da `Cena` com resolução configurável | Planejado |
| **Modelo `.fshape`** | importar e exportar | aparência sem geometria (estilos salvos, forma padrão) | Planejado |

Quando uma forma do Office não tem equivalente (ex.: efeito 3D que a
biblioteca ainda não desenha), a leitura preserva o XML original e mostra a
forma com a parte suportada.

---

## 7. Como usar

No `Cargo.toml` do aplicativo (por tag, no estilo do `feather-ui`):

```toml
[dependencies]
feather-shapes = { git = "https://github.com/feather-works/feather-shapes", tag = "v0.1.0" }
```

Para desenvolver a biblioteca e o aplicativo lado a lado:

```toml
[patch."https://github.com/feather-works/feather-shapes"]
feather-shapes = { path = "../feather-shapes" }
```

Enquanto a tag não for publicada no GitHub, cada raiz de build do monorepo usa
o `[patch]` acima com o caminho da cópia local.

API da v0.1.0 (os nomes da seção 5 que ainda não existem, como `alcas`, estão
marcados como Planejado):

```rust
use feather_shapes::{Forma, TipoForma, Aparencia, Gradiente};
use feather_shapes::feather_draw::{Retangulo, Cor};

// Forma do catálogo pelo id estável (aceita também `seta_direita` e `F072`).
let mut forma = Forma::nova(TipoForma::pelo_id("seta-direita").unwrap());
forma.aparencia = Aparencia {
    gradiente: Some(Gradiente::default()),
    contorno: "#182030".into(),
    espessura: 2.0,
    ..Aparencia::default()
};
forma.rotacao = -8.0;
let caixa = Retangulo::novo(0.0, 0.0, 240.0, 96.0);
let cena = forma.cena(caixa);                         // Cena do feather-draw
let svg = feather_shapes::feather_draw::exportar_svg(&cena).unwrap();
let dentro = forma.acerto(caixa, 120.0, 48.0, 3.0);   // teste de clique

// Ícones: busca em pt ou en, sem acento; cena com cor e espessura.
let icone = feather_shapes::icones::buscar("calendario", 1)[0];
let cena_icone = feather_shapes::icones::cena(icone.id, caixa, Cor::rgb(26, 115, 232), 1.0);

// SmartArt: textos com nível viram formas posicionadas e conectores.
use feather_shapes::arranjo::{diagrama, Arranjo};
let d = diagrama(Arranjo::Organograma,
    &[("Diretoria".into(), 0), ("Produto".into(), 1), ("Engenharia".into(), 1)],
    [0.0, 0.0, 720.0, 360.0]).unwrap();
// d.itens: ItemDiagrama (tipo, caixa, texto, cores, rótulo); d.conectores: (de, ate, estilo)
let modelo = feather_shapes::diagramas::modelo(feather_shapes::Categoria::Ciclos, 0.0, 0.0, 480.0, 270.0);
```

## Desenvolvimento e testes

```bash
cargo test
cargo clippy --all-targets
```

Os testes puros migraram do `featherslide-core` junto com o código (v0.1.0):

| Arquivo | Testes | O que cobre |
|---|---|---|
| `tipo.rs` | 1 | ids, códigos `F0xx` e `prstGeom` únicos e de ida e volta |
| `poligonos.rs` | 3 | contorno de todas as formas adicionais, polígonos e triângulo retângulo, direção das setas |
| `basicas.rs` | 3 | formas com desenho próprio, furos do anel e da moldura, arco aberto |
| `galeria.rs` | 3 | geometria de cada entrada das galerias, rota dos conectores, traço livre |
| `caminho.rs` | 2 | caminhos SVG1 e anéis `u,v` de ida e volta |
| `booleanas.rs` | 3 | as 5 operações, furos, interseção vazia e regiões do catálogo |
| `forma.rs` | 4 | geometria finita de todo o catálogo, `Cena` determinística e SVG, acerto com furo e giro, ajustes |
| `aparencia.rs` | 3 | cores, gradiente e limites, estilo do feather-draw |
| `wordart.rs` | 3 | transformações e estilos prontos do texto |
| `svgpath.rs` | 3 | leitura de caminho SVG |
| `icones.rs` | 4 | catálogo, busca, nomes antigos, categorias e cena do ícone |
| `diagramas.rs` | 2 | diagramas por categoria e escala |
| `arranjo.rs` | 5 | níveis, excluir, mover, promover e rebaixar, limite de itens, arranjos dentro da área |

Os testes que dependem de `Sessao` e `Mudanca` (desfazer) ficaram no FeatherSlide
e chamam a biblioteca.

Regras para contribuir:

- **Toda forma nova** entra com: Id novo na seção 1, nome `prstGeom`, ajustes
  documentados, teste de geometria (contorno dentro do retângulo, sem NaN),
  teste de determinismo (SVG idêntico em duas chamadas) e a linha na tabela.
- **Toda propriedade nova** entra com: valor padrão que reproduz o desenho
  atual, validação com erro tipado, serialização com `serde(default)` e a
  linha na seção 4. Nada de cor, tamanho ou espaçamento fixo dentro de uma
  forma.
- Só marque Pronto quando existir teste. Parcial descreve o que falta.
- Sem emoji nem caractere fazendo papel de ícone; texto em português do
  Brasil; commits com o prefixo de quem fez (`claude:`, `gemini:`).
- Sem dependência com biblioteca em C por trás.
- Não afirme que testes Cargo foram executados sem executá-los.
- Ler `feather-ui/GUIA-JANELAS.md` antes de qualquer tela que use a biblioteca.

## Licença

GPL-3.0 (ver `LICENSE`). Os ícones são do projeto
[Tabler Icons](https://tabler.io/icons) (MIT), copyright dos autores do Tabler;
o texto da licença acompanha o pacote em `dados/TABLER-LICENSE.txt`. Texturas,
padrões e paletas que entrarem terão atribuição própria em `dados/`.
