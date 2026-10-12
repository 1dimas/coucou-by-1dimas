<div align="center">

<img src="NotchBuddy/Assets.xcassets/AppIcon.appiconset/icon_256x256.png" width="96" alt="Coucou icon">

# Coucou · masdevv edition 🎧✨
### `coucou-by-1dimas`

**A customized, cyberpunk-themed desktop dynamic island AI companion & developer HUD for Linux (and Windows/macOS).**

*Forked & supercharged by [@1dimas (masdevv)](https://github.com/1dimas) from the original [Coucou by Louis-CFM](https://github.com/Louis-CFM/coucou).*

[![Linux](https://img.shields.io/badge/Linux-Hyprland%20%7C%20Wayland%20%7C%20X11-FCC624?logo=linux&logoColor=black)](https://github.com/1dimas/coucou-by-1dimas)
[![Windows 10/11](https://img.shields.io/badge/Windows-Tauri%20Ready-0078D4?logo=windows&logoColor=white)](https://github.com/1dimas/coucou-by-1dimas)
[![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-Backend-dea584?logo=rust&logoColor=black)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-Frontend-3178c6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## 🚀 What is Coucou · masdevv edition?

Coucou adalah teman AI mungil (**Mochi**) yang hidup di bagian atas layar desktop kamu (Dynamic Island) untuk memantau sesi AI coding agents, notifikasi, dan status sistem secara real-time.

Di edisi **`coucou-by-1dimas`**, tampilannya dirombak total agar ramah developer, estetik cyberpunk, dan tidak lagi terlihat seperti template bawaan:

### ✨ Fitur Unggulan Edisi masdevv:

- 🎧 **Cyberpunk Studio Headphones**: Mochi sekarang memakai headphone developer studio dengan aksen LED RGB neon menyala (cyan `#38bdf8` & violet `#a855f7`).
- 💬 **Interactive Speech Bubbles & Quotes**: Mochi bisa menyapa kamu secara interaktif (*"Halo masdevv!"*, *"Ngoding apa kita hari ini bray?"*, *"Jangan lupa git commit & push!"*) dan bereaksi saat di-klik/poke.
- ⚡ **Real-Time Developer System HUD**: Monitor CPU & RAM secara real-time langsung di bilah header notch (`🟢 19% CPU · 3.6G RAM`).
- 🛠️ **Custom Integrations System**: Bebas menambahkan shortcut aplikasi atau URL kustom (Telegram, GitHub, Discord, Web server lokal, dll.) langsung di pill island dengan tombol aksi cepat.
- 🔊 **Mechanical Keyboard "Thock" Audio**: Efek suara tactile mechanical switch saat berpindah tab atau berinteraksi dengan Mochi, disintesis langsung via Web Audio API.
- 🌌 **Tokyo Night & Neon Glassmorphism**: Tampilan pulau bernuansa gelap elegan dengan rim glow lembut dan padding presisi tanpa tabrakan teks.
- 🤖 **Multi-Agent & Tool Monitoring**: Integrasi langsung dengan Claude Code, Antigravity, VS Code, Cursor, n8n, Vercel, Resend, dll.

---

## 💻 Panduan Instalasi & Menjalankan (Linux)

Versi ini diuji dan berjalan mulus secara native di **Linux (Wayland / Hyprland / Sway / X11)**.

### 1. Prasyarat (Dependencies)

Pastikan dependensi berikut sudah terpasang di sistem kamu:

* **Node.js** (v18+) & **npm**:
  ```bash
  node -v
  ```
* **Rust & Cargo**:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
  ```
* **Paket GTK & WebKit (Ubuntu / Debian)**:
  ```bash
  sudo apt update
  sudo apt install -y libgtk-3-dev libwebkit2gtk-4.1-dev libgtk-layer-shell-dev libssl-dev build-essential pkg-config
  ```

---

### 2. Clone & Build dari Source

```bash
# 1. Clone repository ini
git clone https://github.com/1dimas/coucou-by-1dimas.git
cd coucou-by-1dimas/windows

# 2. Install dependensi Node.js
npm install

# 3. Build frontend dan compile binary Tauri
npm run build
npx tauri build --no-bundle
```

Hasil kompilasi binary akan berada di:  
`target/release/coucou`

---

### 3. Menjalankan Coucou

Kamu bisa langsung menyalin binary tersebut ke folder lokal agar bisa dijalankan dari mana saja:

```bash
mkdir -p ~/.local/bin
cp target/release/coucou ~/.local/bin/coucou
chmod +x ~/.local/bin/coucou

# Jalankan Coucou di background:
coucou &
```

> 💡 **Tips Pengguna Hyprland**: Kamu bisa menambahkan baris ini ke `~/.config/hypr/hyprland.conf` agar Coucou otomatis berjalan saat login:
> ```ini
> exec-once = ~/.local/bin/coucou
> ```

---

## 🪟 Dukungan Windows

Source code berbasis **Tauri (Rust + TS)** dan memiliki implementasi Win32 API di dalamnya (`windows/src-tauri/src/platform/windows.rs`).  
Untuk mengompilasi file `.exe` di Windows:
1. Buka PowerShell di Windows dengan Node.js dan Rust terinstall.
2. `cd windows`
3. `npm install`
4. `npm run build && npx tauri build`

Binary `.exe` akan tersedia di `target/release/coucou.exe`.

---

## ⚙️ Menambahkan Custom Integration

Kamu bisa menambahkan shortcut kustom kamu sendiri di `~/.config/coucou/settings.json`:

```json
{
  "customIntegrations": [
    {
      "id": "custom_telegram",
      "name": "Telegram",
      "target": "https://web.telegram.org/a/",
      "color": "#24A1DE",
      "description": "Buka Telegram Web"
    },
    {
      "id": "custom_github",
      "name": "GitHub",
      "target": "https://github.com",
      "color": "#A855F7",
      "description": "Buka Dashboard GitHub"
    }
  ]
}
```

---

## 👏 Credits & Attribution

- Dibuat dan dimodifikasi oleh **[@1dimas (masdevv)](https://github.com/1dimas)**.
- Original project by **[Louis Raille (Louis-CFM)](https://github.com/Louis-CFM/coucou)**.
- Licensed under the **MIT License**.
