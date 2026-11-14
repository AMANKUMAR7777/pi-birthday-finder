# 🥧 Pi Birthday Quest (Rust Desktop GUI)

An interactive, high-performance cross-platform desktop application written in **Rust** using **`eframe` / `egui`** to discover where your date of birth is permanently etched into the infinite decimal expansion of **$\pi$**.

---

## 🌟 Key Features

* **🎂 Interactive Birthday Picker**:
  * Select Day, Month, and Year with interactive sliders, presets, and quick buttons.
  * Supports multiple date formats: `DD/MM/YYYY`, `MM/DD/YYYY`, `YYYY/MM/DD`, `DD/MM/YY`, `DD/MM`, or any custom digit sequence.
* **⚡ Live "Matrix" Pi Digit Streamer**:
  * Streaming digits of $\pi$ cascade across the screen in real-time with an active odometer counter.
  * Real-time pattern detection: locks onto your birthday sequence and flashes in neon green upon discovery!
* **🔍 Deep Pi Search Engine**:
  * Fast local embedded search across the first 10,000 digits.
  * Asynchronous online background index searching across **200,000,000 (200 Million) digits** of $\pi$ with zero UI lag (60 FPS fluid rendering).
  * Visualizes the surrounding context tape (`...before [ YOUR BIRTHDAY ] after...`).
* **📜 Official Pi Birthday Certificate**:
  * Generates an official ASCII Certificate of Existence in $\pi$ that you can copy or share.
* **💻 Cross-Platform Native Performance**:
  * 100% pure Rust compiled natively for **Windows**, **Linux**, and **macOS**.
  * Hardware accelerated rendering using `wgpu` / Vulkan / DirectX / Metal.

---

## 🚀 Download & Quick Start

### 1. Download Pre-compiled Binary
The repository includes pre-compiled binaries under the [`bin/`](bin/) folder:
- **Linux (x86_64)**: [`bin/pi-birthday-finder-linux-x86_64`](bin/pi-birthday-finder-linux-x86_64)

To run on Linux:
```bash
chmod +x bin/pi-birthday-finder-linux-x86_64
./bin/pi-birthday-finder-linux-x86_64
```

### 2. Build from Source (Any Platform)
Prerequisites: Rust toolchain (`cargo` and `rustc`).

```bash
git clone https://github.com/AMANKUMAR7777/pi-birthday-finder.git
cd pi-birthday-finder

# Run in development mode
cargo run

# Build optimized release binary
cargo build --release
# Executable will be at ./target/release/pi-birthday-finder
```

---

## 🧮 Mathematical Trivia: Is Your Birthday Guaranteed to Be in $\pi$?

$\pi$ is an irrational number, which means its decimal representation never ends and never repeats.

Mathematicians strongly conjecture that $\pi$ is a **normal number**, meaning that every finite sequence of digits occurs infinitely often with uniform distribution:
- **4-Digit Sequence** (e.g. `2505`): $10^4 = 10,000$ combinations. On average occurs in the first ~10,000 digits.
- **6-Digit Sequence** (e.g. `250509`): $10^6 = 1,000,000$ combinations. Usually found within the first ~2-3 million digits.
- **8-Digit Sequence** (e.g. `25052009`): $10^8 = 100,000,000$ combinations. Usually found within the first 100-200 million digits (e.g. `25052009` occurs at digit **#140,930,892**).

---

## 🛠️ Tech Stack

- **GUI Framework**: [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe) & [`egui`](https://github.com/emilk/egui) (Immediate-mode native GUI)
- **Networking**: `reqwest` (non-blocking async background thread)
- **Concurrency**: `rayon` and standard thread channels (`mpsc`)
- **Date Handling**: `chrono`
- **CI/CD**: GitHub Actions cross-compiling for Linux, Windows, and macOS

---

## 📜 License
MIT License. Crafted with ❤️ in Rust.
