# Ferris Boy

<p align="center">
  <img src="assets/FerrisBoy.png" alt="FerrisBoy Logo" width="200">
</p>

**FerrisBoy** is an experimental and amateur Game Boy emulator written in **Rust**.

## 🔧 Installation

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version) and `cargo`

### Build and Run

Clone the repository and build the emulator:

```sh
git clone https://github.com/scmeek/ferrisboy.git
cd ferrisboy
cargo build --release
```

## 🏗️ Roadmap

### ✅ Current Progress

- [ ] CPU instruction decoding & execution
- [ ] Memory management
- [ ] Basic graphics rendering
- [ ] Sound emulation
- [ ] Save states and battery-backed RAM

## 📜 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 📚 References

- [Pan Docs](https://gbdev.io/pandocs/)
- [Gameboy CPU (LR35902) instruction set](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Gameboy Development Manual V1.1](https://archive.org/details/GameBoyProgManVer1.1)
- [The Cycle-Accurate Game Boy Docs](https://github.com/rockytriton/LLD_gbemu/raw/main/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf)
- [Game Boy: Complete Technical Reference](https://github.com/rockytriton/LLD_gbemu/raw/main/docs/gbctr.pdf)
- [LLD_gbemu Repo](https://github.com/rockytriton/LLD_gbemu)
