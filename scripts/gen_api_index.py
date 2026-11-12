#!/usr/bin/env python3
"""从 docs/API.md 生成 docs/API_INDEX.md。"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
API = ROOT / "docs" / "API.md"
INDEX = ROOT / "docs" / "API_INDEX.md"

lines = API.read_text().splitlines()

categories = []
current = None
for i, line in enumerate(lines):
    if re.match(r'<a id="cat-[^"]+"></a>', line):
        j = i + 1
        while j < len(lines) and not lines[j].startswith("## "):
            j += 1
        if j < len(lines):
            current = lines[j][3:].strip()
            categories.append((current, []))
        continue

    m = re.match(r'<a id="method-([^"]+)"></a>', line)
    if m and current is not None:
        j = i + 1
        while j < len(lines) and not lines[j].startswith("### "):
            j += 1
        if j < len(lines):
            heading = lines[j][4:].strip()
            if " - " in heading:
                _, desc = heading.split(" - ", 1)
            else:
                desc = heading
            categories[-1][1].append((m.group(1), desc))

out = [
    "# API 方法索引",
    "",
    f"> 共 {sum(len(items) for _, items in categories)} 个方法。参数说明和调用示例见 [API.md](API.md)。",
    ">",
    "> 本文件由 `scripts/gen_api_index.py` 生成，请勿手工编辑。",
    "",
]
for name, items in categories:
    if not items:
        continue
    out.append(f"## {name}")
    out.append("")
    out.append("| 方法 | 说明 |")
    out.append("|------|------|")
    for method, desc in items:
        desc = desc.replace("|", "\\|")
        out.append(f"| [`{method}`](API.md#method-{method}) | {desc} |")
    out.append("")

INDEX.write_text("\n".join(out))
print(f"wrote {INDEX} ({sum(len(items) for _, items in categories)} methods)")
