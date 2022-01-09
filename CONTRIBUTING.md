# Setup

## required software

- `go >= 1.17`; see [https://go.dev/dl/](https://go.dev/dl/) for installation instructions. I'd recommend setting up [`gvm`](https://github.com/moovweb/gvm#installing) to manage your go versions.
- [`golangci-lint`](https://golangci-lint.run/usage/install/)
- `cargo+rust >= 1.51`; see [rustup.rs](https://rustup.rs/) for toolchain installation instructions.
- an IDE that respects `.editorconfig` settings.
- [`earthly`](https://earthly.dev/get-earthly), a container-based build tool
- [`docker-compose`](https://docs.docker.com/compose/install/). You may need the [`compose switch`](https://docs.docker.com/compose/cli-command/#compose-switch) to reference `docker compose` v2 as `docker-compose`.
- `make`
- POSIX shell
- a POSIX-compliant OS, e.g. Linux, Windows Subsystem for Linux, or MacOS running an `x86_64` or `ARM64` instruction set.

VSCode users may note the recommended extensions in [`./.vscode/settings.json`](./.vscode/settings.json)

## Creating an issue

All issues should have reproduction steps to reproduce the bug in question or test cases for the desired feature. Please refrain from posting screenshots of text; instead, use code-blocks: markdown sections that delimited by lines starting with <code>```</code>.
For extra-long logs or error messages, pleas enclose your code-blocks with

```
<details><summary>Summary of the long error message or log</summary>

<!--
your novel-length error message here. Note the extra newlines between the details tags and the code block!
-->

</details>
```

## Submitting a patch

All pull requests are appreciated!

### building

The makefile is your friend for figuring out local development. For collecting test corpora, try

```sh
VERSION=${VERSION:-14}
earthly -i -a +pg-corpus-$VERSION/db ./corpus.db
```

### Commit convention

Please use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).
