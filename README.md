# sunshine-app

Sunshine Web UI (`https://localhost:47990`) desktop wrapper, source code from [Pake](https://github.com/tw93/Pake).  
Sunshine Web UI（`https://localhost:47990`）桌面封装，核心源码来自 [Pake](https://github.com/tw93/Pake)。

## Features / 特性

- Automatic HTTP Basic Authentication on Windows (WebView2) and macOS (WKWebView)  
  Windows (WebView2) 与 macOS (WKWebView) 原生自动完成 HTTP Basic Auth 登录（跳过登录弹窗）
- Self-signed HTTPS certificate bypass for target local/LAN hosts  
  自动放行目标本机或局域网主机的自签名 HTTPS 证书警告

## Usage / 使用方式

> [!WARNING]
> Do NOT enter your real password or run builds in a public repository — public Actions inputs, logs, and artifacts are visible to everyone.  
> 切勿在公开仓库中输入真实密码或运行构建，公开仓库的 Actions 输入参数、日志和产物对所有人可见。

1. Click `Use this template` → `Create a new repository` at the top right, and select Private.  
   点击右上角 `Use this template` → `Create a new repository`，并将新仓库设为 Private（私有）。
2. In your private repository, go to Actions → Build Sunshine → Run workflow, select platform (`all`, `windows-latest`, or `macos-latest`), fill in `username` / `password` (and optional `url` / `icon`), and run.  
   在你的私有仓库中进入 Actions → Build Sunshine → Run workflow，选择平台（`all`、`windows-latest` 或 `macos-latest`），填写 `username` / `password`（及可选 `url` / `icon`）即可构建。
3. Download the `Sunshine-Windows` (.msi) or `Sunshine-macOS` (.dmg) artifact and install it.  
   下载生成的 `Sunshine-Windows` (.msi) 或 `Sunshine-macOS` (.dmg) 安装包并安装使用。

## Release Build / 发布构建

- `Release Sunshine` is a manual workflow that builds Windows MSI and macOS DMG with default `https://localhost:47990` and `admin` / `admin` credentials and publishes them to GitHub Releases.  
  `Release Sunshine` 为手动触发的发布工作流，默认使用 `https://localhost:47990` 与 `admin` / `admin` 同时打包 Windows MSI 与 macOS DMG 并发布到 GitHub Releases。
