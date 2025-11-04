<div align="center">

# 🎮 Uncensor Icon

[简体中文](README_zhCN.md) | English

![title](https://raw.githubusercontent.com/wind-addons/resources/main/dist/tools/uncensor-icon/title.png)

</div>

Compare and extract different icons between Global and CN World of Warcraft versions to override censored content.

<details>
<summary>📖 Table of Contents</summary>

- [💼 Requirements](#-requirements)
- [🚚 Installation](#-installation)
  - [Option 1: Download Pre-built Binary](#option-1-download-pre-built-binary)
  - [Option 2: Build from Source](#option-2-build-from-source)
- [🎯 Quick Start](#-quick-start)
  - [Step 1: Extract Game Icons](#step-1-extract-game-icons)
  - [Step 2: Run the Tool](#step-2-run-the-tool)
- [⚙️ Command Options](#️-command-options)
  - [Blacklist Files](#blacklist-files)
- [📝 Example Usage](#-example-usage)
- [🔧 Building from Source](#-building-from-source)
- [📄 License](#-license)

</details>

## 💼 Requirements

- Rust 1.70 or later
- Any OS that supports Rust (Windows, macOS, Linux)
- World of Warcraft (Global and CN versions)

## 🚚 Installation

### Option 1: Download Pre-built Binary

1. Go to the [Releases](https://github.com/wind-addons/uncensor-icon/releases) page
2. Download the latest binary for your platform:
   - Windows: `uncensor-icon-windows.exe`
   - Linux: `uncensor-icon-linux`
   - macOS: `uncensor-icon-macos`
3. Place it in a convenient location

### Option 2: Build from Source

1. Install Rust from <https://rustup.rs/>
2. Clone this repository:

   ```bash
   git clone https://github.com/wind-addons/uncensor-icon.git
   cd uncensor-icon
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. The executable will be in `target/release/uncensor-icon.exe`

## 🎯 Quick Start

### Step 1: Extract Game Icons

You need to extract the icon files from both WoW versions first:

1. Open Battle.net, go to Game Settings for each WoW version
2. Add `-console` to "Additional command line arguments"
3. Start each WoW version, go to character selection screen
4. Press `~` to open console, type `exportInterfaceFiles art`
5. Wait for it to finish (creates `BlizzardInterfaceArt\Interface\ICONS` folders)

### Step 2: Run the Tool

```pwsh
uncensor-icon `
    --global-dir "C:\WoW\Global\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --cn-dir "C:\WoW\CN\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --output-dir "C:\WoW\CN\_retail_\Interface\ICONS"
```

## ⚙️ Command Options

```bash
uncensor-icon [OPTIONS]

Required:
  --global-dir <DIR>      Path to Global WoW icons folder
  --cn-dir <DIR>          Path to CN WoW icons folder
  --output-dir <DIR>      Where to copy the different icons

Optional:
  --blacklist <FILES>     Files to skip (default: XP_ICON.BLP,XPBonus_Icon.blp,Garr_CurrencyIcon-Xp.blp)
  --help                  Show help message
```

### Blacklist Files

By default, these files are excluded from comparison as they contain localized text that naturally differs between versions:

- `XP_ICON.BLP` - Experience bar icons with localized text
- `XPBonus_Icon.blp` - Bonus experience icons with localized text
- `Garr_CurrencyIcon-Xp.blp` - Garrison experience icons with localized text

## 📝 Example Usage

<details>
<summary>Basic Usage</summary>

```pwsh
uncensor-icon `
    --global-dir "C:\Games\WoW\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --cn-dir "C:\Games\WoW CN\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --output-dir "C:\Games\WoW CN\_retail_\Interface\ICONS"
```

</details>

<details>
<summary>With Custom Blacklist</summary>

```pwsh
uncensor-icon `
    --global-dir "C:\Games\WoW\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --cn-dir "C:\Games\WoW CN\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --output-dir "C:\Games\WoW CN\_retail_\Interface\ICONS" `
    --blacklist "XP_ICON.BLP,bad_icon.blp,temp.blp,custom_icon.blp"
```

</details>

## 🔧 Building from Source

1. Install Rust from <https://rustup.rs/>
2. Clone or download this project
3. Run: `cargo build --release`
4. The executable will be in `target/release/uncensor-icon.exe`

## 📄 License

MIT
