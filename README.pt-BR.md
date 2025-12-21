Feeds — Applet COSMIC (Português)
================================

Descrição curta
---------------

Applet para painel COSMIC que busca e mostra itens de feeds RSS/Atom.
Baseado no template do COSMIC, usa `libcosmic` + `iced` para a UI e
`reqwest` + `rss` para rede e parsing.

Compilar & instalar
-------------------

Compile com Cargo (ou use o `justfile`):

```bash
cargo build --release
sudo just install
```

Executar
--------

Após instalar, adicione o applet ao painel COSMIC (entrada de
aplicação: `com.github-pop-os.cosmic-app-template`). Se o painel mostrar
um ícone em cache, faça logout/login para forçar a atualização da
sessão.

Configuração
------------

O app usa `cosmic-config` para persistir os feeds do usuário. Por
padrão vem com um feed de exemplo; gerencie feeds pela opção
"Gerenciar" no popup do applet.

Contribuindo
------------

Abra issues ou pull requests à vontade. O projeto é licenciado sob a
licença MIT (veja `LICENSE`).

Licença
-------
Este projeto é distribuído sob a licença MIT. Veja `LICENSE`.
