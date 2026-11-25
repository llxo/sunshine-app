# sunshine-app

Minimal Tauri 2 desktop wrapper for Sunshine Web UI (`https://localhost:47990/config`), inspired by and derived from [Pake](https://github.com/tw93/Pake).  
极简的 Tauri 2 Sunshine Web UI（`https://localhost:47990/config`）桌面封装，核心源码参考自 [Pake](https://github.com/tw93/Pake)。

## Features / 特性

- Automatic HTTP Basic Authentication on Windows WebView2 (`BasicAuthenticationRequested`)  
  Windows WebView2 原生自动完成 HTTP Basic Auth 登录（跳过登录弹窗）
- Self-signed HTTPS certificate bypass (`--ignore-certificate-errors`)  
  自动放行本地自签名 HTTPS 证书警告

## Usage / 使用方式

> [!WARNING]
> Do NOT enter your real password or run builds in a public repository — public Actions inputs, logs, and artifacts are visible to everyone.  
> 切勿在公开仓库中输入真实密码或运行构建，公开仓库的 Actions 输入参数、日志和产物对所有人可见。

1. Click **`Use this template` → `Create a new repository`** at the top right, and select **Private**.  
   点击右上角 **`Use this template` → `Create a new repository`**，并将新仓库设为 **Private（私有）**。
2. In your private repository, go to **Actions → Build Sunshine (Windows) → Run workflow**, fill in `username` / `password` (and optional `url` / `icon`), and run.  
   在你的私有仓库中进入 **Actions → Build Sunshine (Windows) → Run workflow**，填写 `username` / `password`（及可选 `url` / `icon`）即可构建。
3. Download the `Sunshine-Windows` MSI artifact and install it.  
   下载生成的 `Sunshine-Windows` MSI 安装包并安装使用。

## Release Build / 发布构建

- **`Release Sunshine (Windows)`** is a manual workflow that builds an MSI with default `https://localhost:47990` and `admin` / `admin` credentials and publishes it to GitHub Releases.  
  **`Release Sunshine (Windows)`** 为手动触发的发布工作流，默认使用 `https://localhost:47990` 与 `admin` / `admin` 打包并自动发布到 GitHub Releases。