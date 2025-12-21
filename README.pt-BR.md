Feeds — Applet COSMIC (Português)
================================

Descrição curta
---------------

Applet para painel COSMIC que busca e mostra itens de feeds RSS/Atom.
Usa `libcosmic` + `iced` para a UI e `reqwest` + `rss` para rede e parsing.

Pré-requisitos
--------------

Certifique-se de ter Rust, Just e as bibliotecas do sistema instaladas.

### 1. Instalar Rust e Just
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar Just
cargo install just
```

### Dependências do Sistema
- **Rust**: Instale via [rustup](https://rustup.rs/)
- **Just**: Instale via Cargo: `cargo install just`
- **Bibliotecas do Sistema**:
  - Para Debian/Ubuntu: `sudo apt install libdbus-3.0-dev libgtk-3-dev`
  - Para Fedora: `sudo dnf install dbus-devel gtk3-devel`
  - Para Arch: `sudo pacman -S dbus gtk3`
- **libcosmic**: Instale via Cargo: `cargo install libcosmic`
- **iced**: Instale via Cargo: `cargo install iced`
- **reqwest** e **rss**: Estas bibliotecas são incluídas nas dependências do projeto, então são instaladas automaticamente quando você executar `just`.

Compilar e Instalar
-------------------

Clone o repositório e use o just para instalar:

```bash
git clone https://github.com/marcossl10/cosmic-simple-feeds.git
cd cosmic-simple-feeds
git submodule update --init --recursive
just
sudo just install
```

Se o ícone do applet permanecer em cache no painel COSMIC após a instalação, faça logout/login para forçar a atualização da sessão.

Configuração
------------

O app usa `cosmic-config` para persistir os feeds do usuário. Por
padrão vem com um feed de exemplo; gerencie feeds pela opção
"Gerenciar" no popup do applet.
