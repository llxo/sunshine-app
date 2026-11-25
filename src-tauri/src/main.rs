#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};

#[cfg(target_os = "windows")]
mod windows_auth {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use webview2_com::{
        take_pwstr, BasicAuthenticationRequestedEventHandler,
        Microsoft::Web::WebView2::Win32::ICoreWebView2_10,
    };
    use windows_core::{Interface, HSTRING, PWSTR};

    const MAX_AUTO_ATTEMPTS: usize = 3;

    fn host_allowed(uri: &str, allowed_hosts: &str) -> bool {
        let Ok(parsed) = url::Url::parse(uri) else {
            return false;
        };
        let Some(host) = parsed.host_str() else {
            return false;
        };
        allowed_hosts
            .split(',')
            .map(str::trim)
            .filter(|h| !h.is_empty())
            .any(|h| h.eq_ignore_ascii_case(host))
    }

    pub fn install(webview: &tauri::webview::PlatformWebview) {
        let Some(user) = option_env!("SUNSHINE_USER").filter(|s| !s.is_empty()) else {
            return;
        };
        let Some(pass) = option_env!("SUNSHINE_PASS").filter(|s| !s.is_empty()) else {
            return;
        };
        let allowed_hosts = option_env!("SUNSHINE_HOSTS")
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("localhost,127.0.0.1");

        let attempts = Arc::new(AtomicUsize::new(0));
        let result = unsafe {
            (|| -> windows_core::Result<()> {
                let core = webview.controller().CoreWebView2()?;
                let webview10 = core.cast::<ICoreWebView2_10>()?;
                let mut token = 0;
                let attempts_ref = Arc::clone(&attempts);
                webview10.add_BasicAuthenticationRequested(
                    &BasicAuthenticationRequestedEventHandler::create(Box::new(move |_, args| {
                        let Some(args) = args else {
                            return Ok(());
                        };
                        let mut uri_ptr = PWSTR::null();
                        args.Uri(&mut uri_ptr)?;
                        let uri = take_pwstr(uri_ptr);

                        if !host_allowed(&uri, allowed_hosts) {
                            return Ok(());
                        }

                        if attempts_ref.fetch_add(1, Ordering::Relaxed) >= MAX_AUTO_ATTEMPTS {
                            return Ok(());
                        }

                        let response = args.Response()?;
                        response.SetUserName(&HSTRING::from(user))?;
                        response.SetPassword(&HSTRING::from(pass))?;
                        args.SetCancel(false)?;
                        Ok(())
                    })),
                    &mut token,
                )?;
                Ok(())
            })()
        };

        if let Err(error) = result {
            eprintln!("[sunshine-app] Failed to install Basic Auth handler: {error}");
        }
    }
}

fn main() {
    let target_url = option_env!("SUNSHINE_URL")
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("https://localhost:47990/config");

    let parsed_url: url::Url = target_url.parse().expect("Invalid SUNSHINE_URL");

    tauri::Builder::default()
        .setup(move |app| {
            let builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(parsed_url.clone()))
                .title("Sunshine")
                .inner_size(1200.0, 780.0);

            #[cfg(target_os = "windows")]
            let builder = builder.additional_browser_args("--ignore-certificate-errors");

            let window = builder.build()?;

            #[cfg(target_os = "windows")]
            if let Err(error) = window.with_webview(|webview| {
                windows_auth::install(&webview);
            }) {
                eprintln!("[sunshine-app] Failed to access WebView2: {error}");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running sunshine-app");
}
