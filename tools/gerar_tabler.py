#!/usr/bin/env python3
"""Gera dados/tabler-outline.tsv a partir do pacote npm @tabler/icons (MIT).

Uso (numa pasta de trabalho qualquer, com o pacote já extraído):
    npm pack @tabler/icons@3.49.0 && mkdir pacote && tar xzf tabler-icons-*.tgz -C pacote
    python3 -I tools/gerar_tabler.py pacote/package dados

Cada linha: id <TAB> categoria <TAB> tags separadas por vírgula <TAB> caminhos `d` separados por `|`.
Só os ícones de contorno (24×24, traço 2): são só `<path>`, o que cabe nos traços do motor.
"""
import glob, json, os, re, shutil, sys

origem, destino = sys.argv[1], sys.argv[2]
meta = json.load(open(os.path.join(origem, "icons.json"), encoding="utf-8"))
linhas = []
for arq in sorted(glob.glob(os.path.join(origem, "icons", "outline", "*.svg"))):
    nome = os.path.splitext(os.path.basename(arq))[0]
    svg = open(arq, encoding="utf-8").read()
    caminhos = []
    for m in re.finditer(r"<path\b([^>]*)/>", svg):
        atributos = m.group(1)
        if 'stroke="none"' in atributos:  # o quadro de 24×24 que não desenha nada
            continue
        d = re.search(r'\bd="([^"]*)"', atributos)
        if d:
            caminhos.append(re.sub(r"\s+", " ", d.group(1)).strip())
    if not caminhos:
        continue
    info = meta.get(nome, {})
    tags = ",".join(str(t).replace("\t", " ").replace(",", " ") for t in info.get("tags", []))
    linhas.append("\t".join([nome, info.get("category", ""), tags, "|".join(caminhos)]))
os.makedirs(destino, exist_ok=True)
open(os.path.join(destino, "tabler-outline.tsv"), "w", encoding="utf-8").write("\n".join(linhas) + "\n")
shutil.copy(os.path.join(origem, "LICENSE"), os.path.join(destino, "TABLER-LICENSE.txt"))
print(f"{len(linhas)} ícones")
