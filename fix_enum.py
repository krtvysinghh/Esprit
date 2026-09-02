import re
with open('apps/esprit-cli/src/main.rs', 'r') as f:
    c = f.read()

enum_addition = """
    Debate { topic: String },
    EnvScaffold,
    Docker,
    Score { file: String },
"""

c = re.sub(r'(enum Commands \{.*?)\}', r'\1' + enum_addition + '}', c, flags=re.DOTALL)

with open('apps/esprit-cli/src/main.rs', 'w') as f:
    f.write(c)
