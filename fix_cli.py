with open('apps/esprit-cli/src/main.rs', 'r') as f:
    c = f.read()

c = c.replace('"(?:std::env::var|process\.env\.|\"env\": \")[A-Z0-9_]+"', 'r#"(?:std::env::var|process\.env\.|"env": ")[A-Z0-9_]+"#')
c = c.replace("['\"', '(', ')', '.']", r"['\"', '(', ')', '.']")

with open('apps/esprit-cli/src/main.rs', 'w') as f:
    f.write(c)
