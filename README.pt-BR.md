Feeds — Applet COSMIC (Português)
================================

Descrição curta
---------------

Applet para painel COSMIC que busca e mostra itens de feeds RSS/Atom.
Usa `libcosmic` + `iced` para a UI e `reqwest` + `rss` para rede e parsing.

Pré-requisitos
--------------

Certifique-se de ter Rust e Just instalados em seu sistema:

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar Just
cargo install just
```

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

Contribuindo
------------

Abra issues ou pull requests à vontade. O projeto é licenciado sob a
licença MIT (veja `LICENSE`).
