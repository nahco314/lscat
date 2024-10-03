# lscat

ls for directories, cat for files

## Why?

In order to search for files in stages while examining the directory structure, you may need to perform a loop of ls and appending the path.

In such cases, you can save yourself the trouble of editing the command name (to cat from ls) when you reach the file.

## Installation

### on unix (linux, macos)

You can install it with a standalone installer or cargo.

```bash
# standalone installer
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nahco314/lscat/releases/latest/download/lscat-installer.sh | sh

# or cargo
cargo install --git https://github.com/nahco314/lscat
```

### on windows

You can install it with a standalone installer or cargo.

**Attention: I have not tested this on Windows!**

```powershell
# standalone installer
powershell -ExecutionPolicy ByPass -c "irm https://github.com/nahco314/lscat/releases/latest/download/lscat-installer.ps1 | iex"

# or cargo
cargo install --git https://github.com/nahco314/lscat
```

## Usage

```bash
$ lscat .
Cargo.lock  Cargo.toml  LICENSE  README.md  src  target

$ lscat ./src/
main.rs

$ lscat ./src/main.rs
fn main() {
    println!("Hello, world!");
}
```

You can use it as a replacement for `ls`. (I'm using it as an alias)

```shell
# on your shell rc file
alias ls="lscat"
```

## Settings

The commands used as ls and cat can be set using environment variables.

- ls
  - env: LSCAT_LS
  - default on unix: `ls --color=always`
  - default on windows: `dir`
- cat
  - env: LSCAT_CAT
  - default on unix: `cat`
  - default on windows: `type`
