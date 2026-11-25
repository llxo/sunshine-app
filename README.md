# sunshine-app

Minimal Tauri 2 desktop wrapper for Sunshine Web UI (`https://localhost:47990/config`) with:
- Automatic HTTP Basic Authentication on Windows WebView2 (`BasicAuthenticationRequested`)
- Self-signed HTTPS certificate bypass (`--ignore-certificate-errors`)

## Build via GitHub Actions

Go to **Actions → Build Sunshine (Windows) → Run workflow**, fill in `username` and `password`, then download the `Sunshine-Windows` MSI artifact.
