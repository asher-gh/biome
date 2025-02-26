<p align="center">
    <img alt="Biome - Toolchain of the web"
         src="https://raw.githubusercontent.com/biomejs/resources/main/biome-logo-slogan.svg"
         width="400">
</p>

<div align="center">

[![Discord chat][discord-badge]][discord-url]
[![CI on main][ci-badge]][ci-url]
[![npm version][npm-badge]][npm-url]
[![VSCode version][vscode-badge]][vscode-url]
[![Open VSX version][open-vsx-badge]][open-vsx-url]

[discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=green
[discord-url]: https://discord.gg/BypW39g6Yc
[ci-badge]: https://github.com/biomejs/biome/actions/workflows/main.yml/badge.svg
[ci-url]: https://github.com/biomejs/biome/actions/workflows/main.yml
[npm-badge]: https://badgen.net/npm/v/@biomejs/biome?icon=npm&color=green&label=%40biomejs%2Fbiome
[npm-url]: https://www.npmjs.com/package/@biomejs/biome/v/latest
[vscode-badge]: https://badgen.net/vs-marketplace/v/biomejs.biome?label=vscode&icon=visualstudio&color=green
[vscode-url]: https://marketplace.visualstudio.com/items?itemName=biomejs.biome
[open-vsx-badge]: https://badgen.net/open-vsx/version/biomejs/biome?label=open-vsx&color=green
[open-vsx-url]: https://open-vsx.org/extension/biomejs/biome


</div>

**Biome** is a toolchain for web projects, it aims to provide developer tools to maintain the health of said projects.

### Installation

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### Usage

```shell
# format files
npx @biomejs/biome format --write ./src

# lint files
npx @biomejs/biome lint ./src

# run format, lint, etc. and apply the safe suggestions
npx @biomejs/biome check --apply ./src

# check all files against format, lint, etc. in CI environments
npx @biomejs/biome ci ./src
```

If you want to give Biome a run without installing it, use the [online playground](https://biomejs.dev/playground/), compiled to WebAssembly.

## Documentation

Check out our [homepage][biomejs] to learn more about Biome,
or directly head to the [Getting Started guide][getting-started] to start using Biome.

## About Biome

**Biome** formats and lints your code in a [fraction of a second][bench].

**Biome** [doesn't require Node.js](https://biomejs.dev/guides/manual-installation/) to function.

**Biome** has first-class LSP support, with a sophisticated parser that represents the source text in full fidelity and top-notch error recovery.

**Biome** supports JavaScript, TypeScript, JSON, and more. It aims to support [all main languages][language-support] of modern web development.

**Biome** has sane defaults and it doesn't require configuration.

**Biome** helps you as much as possible by displaying detailed and contextualized diagnostics. Read more about our [project philosophy][biome-philosophy].

**Biome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelize work, caching, and configuration.

**Biome** is [MIT licensed](https://github.com/biomejs/biome/tree/main/LICENSE-MIT) or [Apache 2.0 licensed](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE) and moderated under the [Contributor Covenant Code of Conduct](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md).


## Sponsors

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://shiguredo.jp/" target="_blank"><img src="https://images.opencollective.com/shiguredo/8552ade/logo.png?height=80" width="80"></a>
      </td>
    </tr>
  </tbody>
</table>


[bench]: https://github.com/biomejs/biome/blob/main/benchmark/README.md
[biomejs]: https://biomejs.dev/
[biome-philosophy]: https://biomejs.dev/internals/philosophy/
[language-support]: https://biomejs.dev/internals/language-support/
[getting-started]: https://biomejs.dev/guides/getting-started/
