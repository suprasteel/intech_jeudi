# Installation de toolchain pour rust (windows)

-- En cas de problèmes

---

## I. Installation windows

1. https://www.rust-lang.org/tools/install
2. Continue: yes
3. Proceed: 1)
4. Tester
    - cargo new
    - rust-analyzer avec vscode

---

## II. Setup toolchain

### 1. Using C++ win compiler (recommandé)

Download the Visual Studio 2019 Build tools from the Microsoft website: https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16

After the download, while installing the Build tools, makesure that you install the required components:

    C++ build tools

---

This will download required files. Once everything is successfully installed, reboot and re-run your rust program and it will compile successfully.

Needs 5Gb of disk space.

---

### 2. Use GCC to compile (with MinGW) - alternative

install MinGw with g++ gcc development packages and then

Tape in cmd:

- `rustup uninstall toolchain stable-x86_64-pc-windows-msvc`
- `toolchain install stable-x86_64-pc-windows-gnu` (or download rustup-init for the platform of your choice at https://forge.rust-lang.org/infra/other-installation-methods.html)
- `rustup default stable-x86_64-pc-windows-gnu`

---

> Différences:<br/>

The GNU toolchain uses the MinGW build tools (mostly the linker) and produces binaries that use the GNU ABI. The MSVC toolchain uses the Visual Studio build tools and produces binaries that use the Microsoft ABI, making them more compatible with most other Windows binaries/libraries.

