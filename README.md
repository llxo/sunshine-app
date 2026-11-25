# SunshineApp

Minimal Tauri 2 desktop client for Sunshine Web UI (`https://localhost:47990`), core source code from [Pake](https://github.com/tw93/Pake).  
极简的 Sunshine Web UI（`https://localhost:47990`）Tauri 2 桌面客户端，核心源码来自 [Pake](https://github.com/tw93/Pake)。

## Features / 特性

- Native auto-login for HTTP Basic Auth on Windows (WebView2) and macOS (WKWebView)  
  Windows (WebView2) 与 macOS (WKWebView) 原生自动完成 HTTP Basic Auth 登录，无需弹窗输入
- Automatic self-signed HTTPS certificate trust for the configured local or LAN host  
  自动信任所配置的本机或局域网主机自签名 HTTPS 证书
- Isolated app name and install path (`SunshineApp`) to avoid conflicts with Sunshine itself  
  独立的应用名称与安装目录（`SunshineApp`），避免与 Sunshine 本体目录或卸载器冲突

## Custom Build / 自定义构建

> [!WARNING]
> Never enter your real password or run builds in a public repository — public Actions inputs, logs, and artifacts are visible to everyone.  
> 切勿在公开仓库中输入真实密码或运行构建，公开仓库的 Actions 输入参数、日志和产物对所有人可见。

1. Click `Use this template` → `Create a new repository` at the top right, and set visibility to Private.  
   点击右上角 `Use this template` → `Create a new repository`，并将仓库设为 Private（私有）。
2. Open Actions → `Build Sunshine` → `Run workflow` in your private repository.  
   在你的私有仓库中进入 Actions → `Build Sunshine` → `Run workflow`。
3. Choose the target platform (`all`, `windows-latest`, or `macos-latest`), fill in `username` and `password` (and optional `url` / `icon`), then run.  
   选择目标平台（`all`、`windows-latest` 或 `macos-latest`），填写 `username` 与 `password`（及可选 `url` / `icon`）后运行。
4. Download `SunshineApp-Windows` (.msi) or `SunshineApp-macOS` (.dmg) from the workflow artifacts and install.  
   在构建产物中下载 `SunshineApp-Windows`（.msi）或 `SunshineApp-macOS`（.dmg）安装即可。

## Release / 预设发布

- You can download prebuilt installers directly from Releases (`https://localhost:47990`, default `admin` / `admin`), or trigger `Release Sunshine` manually to publish your own tag.  
  可直接在 Releases 下载预设安装包（默认地址 `https://localhost:47990`，默认账密 `admin` / `admin`），也可手动触发 `Release Sunshine` 发布新版本。
