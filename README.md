# sunshine-app

Minimal Tauri 2 desktop wrapper for Sunshine Web UI (`https://localhost:47990/config`) with:
- Automatic HTTP Basic Authentication on Windows WebView2 (`BasicAuthenticationRequested`)
- Self-signed HTTPS certificate bypass (`--ignore-certificate-errors`)

## Usage (Important)

> [!WARNING]
> **Do NOT run the workflow or enter your password in a public repository.** Public GitHub Actions inputs, logs, and artifacts are visible to everyone.

1. Click **`Use this template` → `Create a new repository`** at the top right of this page, and choose **Private** as the visibility.
2. In your new private repository, go to **Actions → Build Sunshine (Windows) → Run workflow**.
3. Fill in `username` and `password` (and adjust `url` / `icon` if needed), then run the workflow.
4. Download the `Sunshine-Windows` MSI artifact and install it.