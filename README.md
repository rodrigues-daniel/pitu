# 🦀 pitu

> ⚠️ **Status: Alpha** — projeto em desenvolvimento ativo. Espere mudanças de API e funcionalidades incompletas.

**pitu** é uma ferramenta de linha de comando (_CLI_) escrita em Rust que funciona como um scaffolding para acelerar a criação e o desenvolvimento de projetos Rust voltados para a web. Com pitu, você inicializa projetos, gera estruturas de código e integra outras ferramentas do ecossistema Rust web de forma rápida e padronizada.

---

## ✨ Funcionalidades (planejadas / em desenvolvimento)

- Scaffolding de projetos Rust web com estrutura pré-configurada
- Geração de módulos, handlers, rotas e modelos
- Integração com frameworks e ferramentas populares do ecossistema Rust web
- Configuração automatizada de dependências no `Cargo.toml`
- Templates customizáveis
- Suporte a múltiplos frameworks (planejado)

---

## 🛠️ Tecnologias

- [Rust](https://www.rust-lang.org/)
- [Clap](https://docs.rs/clap) — parsing de argumentos de linha de comando

---

## 📦 Instalação

> Ainda não disponível em crates.io. Para usar durante a fase alpha, compile a partir do código fonte.

```bash
git clone https://github.com/seu-usuario/pitu.git
cd pitu
cargo install --path .
```

---

## 🚀 Uso

```bash
# Criar um novo projeto
pitu new meu-projeto

# Gerar um componente / módulo
pitu generate handler usuario

# Ver ajuda
pitu --help
```

> Os comandos acima são exemplos do que está sendo desenvolvido e podem mudar conforme o projeto evolui.

---

## 🗺️ Roadmap

- [ ] Comando `new` — inicializar projeto base
- [ ] Comando `generate` — geração de código (handlers, rotas, modelos)
- [ ] Suporte a templates customizados
- [ ] Integração com ferramentas de migração de banco de dados
- [ ] Publicação no crates.io
- [ ] Documentação completa

---

## 🤝 Contribuindo

O projeto está em fase **alpha** e contribuições são bem-vindas! Sinta-se à vontade para abrir _issues_ com sugestões, bugs ou dúvidas.

1. Faça um fork do repositório
2. Crie uma branch para sua feature (`git checkout -b feature/minha-feature`)
3. Commit suas mudanças (`git commit -m 'feat: adiciona minha feature'`)
4. Push para a branch (`git push origin feature/minha-feature`)
5. Abra um Pull Request

---

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

---

<p align="center">feito com ❤️ e 🦀 Rust</p>
