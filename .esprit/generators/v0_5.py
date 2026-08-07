#!/usr/bin/env python3

from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]


def write(rel, text):
    path = ROOT / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.rstrip() + "\n", encoding="utf-8")
    print("✓", rel)


FILES = {

}
for path, content in FILES.items():
    write(path, content)

subprocess.run(["cargo", "fmt"], cwd=ROOT)
subprocess.run(["cargo", "build", "--workspace"], cwd=ROOT)
subprocess.run(["cargo", "test"], cwd=ROOT)

print()
print("==========")
print("v0.5 DONE")
print("==========")
