# SunshineApp

A standalone desktop client for the Sunshine Web UI (`https://localhost:47990`), NOT the Sunshine server itself. You still need Sunshine installed and running on your PC or LAN. Core source code from [Pake](https://github.com/tw93/Pake).  
Sunshine 网页管理后台（Web UI）的独立桌面客户端，并非 Sunshine 服务端本体（电脑或局域网内需先安装并运行 Sunshine）。核心源码来自 [Pake](https://github.com/tw93/Pake)。

## Features / 特性

- Auto-login without typing your username and password every time (Windows & macOS)  
  打开即自动登录后台，省去每次弹窗输入账号密码的麻烦（支持 Windows 与 macOS）
- Skips the browser "unsafe / self-signed certificate" warning automatically  
  自动跳过本地或局域网网页的证书安全警告页面
- Supports both this PC (`localhost`) and other devices on your LAN (e.g. `192.168.x.x`)  
  支持访问本机（`localhost`）或局域网内其他电脑的 Sunshine 后台
- Installed separately as `SunshineApp` so it will not overwrite Sunshine or be removed by uninstallers by mistake  
  使用独立的名称和安装目录（`SunshineApp`），不会覆盖 Sunshine 本体，也不会被卸载工具误删

## Quick Start / 使用方法

### 1. Direct Download / 直接下载（默认 `admin` / `admin`）

If your Sunshine runs at `https://localhost:47990` with username `admin` and password `admin`, download the installer directly from Releases:  
如果你的 Sunshine 地址为 `https://localhost:47990` 且账号密码为默认的 `admin` / `admin`，直接前往 Releases 下载安装即可：
- Windows: `SunshineApp-v...-Windows-x64.msi`
- macOS (M-series): `SunshineApp-v...-macOS-AppleSilicon.dmg`

### 2. Custom URL or Password / 自定义地址或账号密码

> [!WARNING]
> Do NOT enter your real password or run builds in a public repository — everyone can see public Actions inputs, logs, and files.  
> 切勿在公开仓库中填写你的真实密码或运行打包，公开仓库的输入参数、运行日志和安装包所有人都能看到。

1. Click `Use this template` → `Create a new repository` at the top right, and select Private.  
   点击右上角 `Use this template` → `Create a new repository`，并将新仓库设为 Private（私有）。
2. Open Actions → `Build Sunshine` → `Run workflow` in your private repository.  
   在你的私有仓库中打开 Actions → `Build Sunshine` → `Run workflow`。
3. Choose your system (`all`, `windows-latest`, or `macos-latest`), enter your Sunshine `url`, `username`, and `password` (and optional `icon`), then run.  
   选择系统（`all`、`windows-latest` 或 `macos-latest`），填入你的 Sunshine 地址、用户名和密码（图标可选），点击运行。
4. Wait a few minutes, download `SunshineApp-Windows` (.msi) or `SunshineApp-macOS` (.dmg), and install it.  
   等待几分钟打包完成后，下载 `SunshineApp-Windows`（.msi）或 `SunshineApp-macOS`（.dmg）安装即可。
