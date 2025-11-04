<div align="center">

# 🎮 Uncensor Icon

简体中文 | [English](README.md)

![title](https://raw.githubusercontent.com/wind-addons/resources/main/dist/tools/uncensor-icon/title.png)

</div>

比较并提取全球版和中国版《魔兽世界》之间的不同图标，以覆盖被和谐的内容。

<details>
<summary>📖 目录</summary>

- [💼 系统要求](#-系统要求)
- [🚚 安装](#-安装)
  - [选项1：下载预编译二进制文件](#选项1下载预编译二进制文件)
  - [选项2：从源代码构建](#选项2从源代码构建)
- [🎯 快速开始](#-快速开始)
  - [步骤1：提取游戏图标](#步骤1提取游戏图标)
  - [步骤2：运行工具](#步骤2运行工具)
- [⚙️ 命令选项](#️-命令选项)
  - [黑名单文件](#黑名单文件)
- [📝 使用示例](#-使用示例)
- [🔧 从源代码构建](#-从源代码构建)
- [📄 许可证](#-许可证)

</details>

## 💼 系统要求

- Rust 1.70 或更高版本
- 任何支持 Rust 的操作系统（Windows、macOS、Linux）
- 《魔兽世界》（全球版和中国版）

## 🚚 安装

### 选项1：下载预编译二进制文件

1. 访问 [Releases](https://github.com/wind-addons/uncensor-icon/releases) 页面
2. 下载适合您平台的最新二进制文件：
   - Windows: `uncensor-icon-windows.exe`
   - Linux: `uncensor-icon-linux`
   - macOS: `uncensor-icon-macos`
3. 将其放置在方便的位置

### 选项2：从源代码构建

1. 从 https://rustup.rs/ 安装 Rust
2. 克隆此仓库：

   ```bash
   git clone https://github.com/wind-addons/uncensor-icon.git
   cd uncensor-icon
   ```

3. 构建项目：

   ```bash
   cargo build --release
   ```

4. 可执行文件将位于 `target/release/uncensor-icon.exe`

## 🎯 快速开始

### 步骤1：提取游戏图标

首先需要从两个WoW版本中提取图标文件：

1. 打开战网客户端，进入每个WoW版本的游戏设置
2. 在"其他命令行参数"中添加 `-console`
3. 启动每个WoW版本，进入角色选择界面
4. 按 `~` 键打开控制台，输入 `exportInterfaceFiles art`
5. 等待完成（会创建 `BlizzardInterfaceArt\Interface\ICONS` 文件夹）

### 步骤2：运行工具

```pwsh
uncensor-icon `
    --global-dir "C:\WoW\Global\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --cn-dir "C:\WoW\CN\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --output-dir "C:\WoW\CN\_retail_\Interface\ICONS"
```

## ⚙️ 命令选项

```bash
uncensor-icon [选项]

必需参数:
  --global-dir <DIR>      全球版WoW图标文件夹路径
  --cn-dir <DIR>          中国版WoW图标文件夹路径
  --output-dir <DIR>      复制不同图标的输出位置

可选参数:
  --blacklist <FILES>     要跳过的文件（默认：XP_ICON.BLP,XPBonus_Icon.blp,Garr_CurrencyIcon-Xp.blp）
  --help                  显示帮助信息
```

### 黑名单文件

默认情况下，这些文件被排除在比较之外，因为它们包含本地化文字，在不同版本间自然存在差异：
- `XP_ICON.BLP` - 包含本地化文字的经验条图标
- `XPBonus_Icon.blp` - 包含本地化文字的奖励经验图标
- `Garr_CurrencyIcon-Xp.blp` - 包含本地化文字的要塞经验图标

## 📝 使用示例

<details>
<summary>基本用法</summary>

```bash
uncensor-icon `
    --global-dir "C:\Games\WoW\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --cn-dir "C:\Games\WoW CN\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --output-dir "C:\Games\WoW CN\_retail_\Interface\ICONS"
```

</details>

<details>
<summary>使用自定义黑名单</summary>

```bash
uncensor-icon `
    --global-dir "C:\Games\WoW\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --cn-dir "C:\Games\WoW CN\_retail_\BlizzardInterfaceArt\Interface\ICONS" `
    --output-dir "C:\Games\WoW CN\_retail_\Interface\ICONS" `
    --blacklist "XP_ICON.BLP,bad_icon.blp,temp.blp,custom_icon.blp"
```

</details>

## 🔧 从源代码构建

1. 从 https://rustup.rs/ 安装 Rust
2. 克隆或下载此项目
3. 运行：`cargo build --release`
4. 可执行文件将位于 `target/release/uncensor-icon.exe`

## 📄 许可证

MIT
