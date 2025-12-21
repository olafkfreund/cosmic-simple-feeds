````markdown
Feeds — Applet COSMIC (Português)
=================================

Descrição curta
---------------

Este é um pequeno applet para o painel COSMIC que busca e exibe itens de
feeds RSS/Atom. Ele usa `libcosmic` + `iced` para a interface do usuário e
`reqwest` + `rss` para análise de rede.

Pré-requisitos
--------------

Para compilar o applet, você precisa das seguintes dependências:

### 1. Instalar Rust, Cargo e Just

#### Arch Linux
```bash
sudo pacman -S rust just 
```

#### Fedora
```bash
sudo dnf install rust cargo just
```

#### Pop!_OS / Ubuntu
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install just
```

### 2. Bibliotecas do Sistema
- **Pop!_OS / Ubuntu / Debian**: `sudo apt install libdbus-3.0-dev libgtk-3-dev`
- **Fedora**: `sudo dnf install dbus-devel gtk3-devel`
- **Arch Linux**: `sudo pacman -S dbus gtk3`

- **reqwest** e **rss**: Estas dependências estão incluídas no projeto, elas serão instaladas quando você executar `just`.

### Etapas de Instalação
1. Clone o repositório:
   ```bash
   git clone https://github.com/marcossl10/cosmic-simple-feeds.git
   cd cosmic-simple-feeds
   git submodule update --init --recursive
   ```
   ...
2. sudo cargo build --release
   ...
3. Instale o applet:
   ```bash
   sudo just install
   ```

4. Se o ícone do applet estiver em cache, saia e entre novamente para atualizar a sessão.

Configuração
-------------

O aplicativo usa `cosmic-config` para persistir os feeds do usuário. Por padrão,
ele inclui um feed de exemplo; gerencie os feeds a partir da visualização "Gerenciar"
no popup do applet.

Desenvolvedores devem instalar [rustup][rustup] e considerar o uso de
`rust-analyzer` em seu editor.

[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
````