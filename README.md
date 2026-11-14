# 🥧 Pi Cosmic Observatory (Modern Desktop GUI)

An interactive, modern cross-platform desktop application to discover where your date of birth is permanently etched into the infinite decimal expansion of **$\pi$**.

---

## 🌟 Key Features

* **🎨 Bespoke Modern Desktop UI**:
  * Designed using modern component architecture with a rich midnight-charcoal aesthetic, subtle glowing borders, and high contrast.
  * Segmented date inputs (Day, Month, Year) with live target preview and multiple international formats (`DD/MM/YYYY`, `MM/DD/YYYY`, `YYYY/MM/DD`, `DD/MM/YY`, `DD/MM`).
  * Historical landmark presets (Albert Einstein on Pi Day 1879, Moon Landing 1969, World Wide Web 1989, Ada Lovelace 1815).
* **⚡ Live Matrix Streamer**:
  * Real-time cascading display of digits streaming from $\pi$ with an odometer counter and lock-on target highlights.
* **🔍 Deep Pi Search Engine**:
  * Fast local offline cache for immediate lookups.
  * Asynchronous non-blocking search querying a **200,000,000-digit $\pi$ index** with zero UI freezing.
  * Visualizes the surrounding context tape (`...before [ YOUR DATE ] after...`).
* **📜 Official Pi Birthday Certificate**:
  * Clean, formatted box-drawn certificate copyable to your clipboard with one click.
* **🚀 Multi-Engine Support**:
  * **Modern Python Desktop GUI**: Fast, responsive web-grade UI via `pywebview`.
  * **Native CustomTkinter GUI**: Alternative native widget desktop interface.
  * **High-Performance Rust Core**: Native compiled binary via `eframe` / `wgpu`.

---

## 💻 Quick Start & Running

### Option 1: Modern Python Desktop GUI (Recommended)
```bash
cd pi-birthday-finder

# Install dependencies (if not already installed)
pip install -r requirements.txt

# Run the modern desktop application
python3 run_app.py
```

### Option 2: Native CustomTkinter GUI
```bash
python3 run_customtkinter.py
```

### Option 3: Rust Native Binary
```bash
chmod +x bin/pi-birthday-finder-linux-x86_64
./bin/pi-birthday-finder-linux-x86_64

# Or build from source
cargo run --release
```

---

## 🧮 Mathematical Trivia: Is Your Date in $\pi$?

$\pi$ is an irrational number whose digits never terminate and never repeat.

Mathematicians strongly conjecture that $\pi$ is a **normal number**, meaning every finite sequence of digits occurs infinitely often with uniform distribution:
- **4-Digit Sequence** (e.g. `1403` - Pi Day): $10^4 = 10,000$ combinations. Found in the first ~10,000 digits.
- **6-Digit Sequence** (e.g. `140379`): $10^6 = 1,000,000$ combinations. Found in the first ~1-3 million digits.
- **8-Digit Sequence** (e.g. `14031879` - Einstein's Birthday): $10^8 = 100,000,000$ combinations. Found at decimal position **#11,885,022**.

---

## 📜 License
MIT License.
