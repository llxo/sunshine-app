#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};

fn host_matches(host: &str, allowed_hosts: &[String]) -> bool {
    let normalized = host.trim().trim_end_matches('.');
    !normalized.is_empty()
        && allowed_hosts
            .iter()
            .any(|h| h.trim().trim_end_matches('.').eq_ignore_ascii_case(normalized))
}

fn build_allowed_hosts(target_url: &url::Url) -> Vec<String> {
    let mut hosts: Vec<String> = option_env!("SUNSHINE_HOSTS")
        .unwrap_or("localhost,127.0.0.1")
        .split(',')
        .map(str::trim)
        .filter(|h| !h.is_empty())
        .map(str::to_owned)
        .collect();

    if let Some(target_host) = target_url.host_str() {
        if !host_matches(target_host, &hosts) {
            hosts.push(target_host.to_owned());
        }
    }
    hosts
}

#[cfg(target_os = "windows")]
mod windows_auth {
    use super::host_matches;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    use webview2_com::{
        take_pwstr, BasicAuthenticationRequestedEventHandler,
        Microsoft::Web::WebView2::Win32::ICoreWebView2_10,
    };
    use windows_core::{Interface, HSTRING, PWSTR};

    const MAX_RAPID_ATTEMPTS: usize = 3;
    const ATTEMPT_WINDOW: Duration = Duration::from_secs(5);

    pub fn install_and_navigate(
        webview: &tauri::webview::PlatformWebview,
        target_url: &str,
        allowed_hosts: Vec<String>,
    ) {
        let user = option_env!("SUNSHINE_USER").filter(|s| !s.is_empty());
        let pass = option_env!("SUNSHINE_PASS").filter(|s| !s.is_empty());

        let result = unsafe {
            (|| -> windows_core::Result<()> {
                let core = webview.controller().CoreWebView2()?;

                if let (Some(user), Some(pass)) = (user, pass) {
                    let webview10 = core.cast::<ICoreWebView2_10>()?;
                    let mut token = 0;
                    let attempt_state = Arc::new(Mutex::new((Instant::now() - ATTEMPT_WINDOW, 0usize)));

                    webview10.add_BasicAuthenticationRequested(
                        &BasicAuthenticationRequestedEventHandler::create(Box::new(move |_, args| {
                            let Some(args) = args else {
                                return Ok(());
                            };
                            let mut uri_ptr = PWSTR::null();
                            args.Uri(&mut uri_ptr)?;
                            let uri = take_pwstr(uri_ptr);

                            let Ok(parsed) = url::Url::parse(&uri) else {
                                return Ok(());
                            };
                            let Some(host) = parsed.host_str() else {
                                return Ok(());
                            };
                            if !host_matches(host, &allowed_hosts) {
                                return Ok(());
                            }

                            // Guard against infinite auth loops on wrong credentials while
                            // allowing normal re-authentication after ATTEMPT_WINDOW elapses.
                            {
                                let mut state = attempt_state.lock().unwrap_or_else(|e| e.into_inner());
                                let now = Instant::now();
                                if now.duration_since(state.0) > ATTEMPT_WINDOW {
                                    state.0 = now;
                                    state.1 = 1;
                                } else {
                                    state.1 += 1;
                                    if state.1 > MAX_RAPID_ATTEMPTS {
                                        return Ok(());
                                    }
                                }
                            }

                            let response = args.Response()?;
                            response.SetUserName(&HSTRING::from(user))?;
                            response.SetPassword(&HSTRING::from(pass))?;
                            args.SetCancel(false)?;
                            Ok(())
                        })),
                        &mut token,
                    )?;
                }

                core.Navigate(&HSTRING::from(target_url))?;
                Ok(())
            })()
        };

        if let Err(error) = result {
            eprintln!("[sunshine-app] Failed to initialize WebView2 auth/navigation: {error}");
        }
    }
}

#[cfg(target_os = "macos")]
mod macos_auth {
    use super::host_matches;
    use objc2::rc::{Retained, Weak};
    use objc2::runtime::{AnyObject, Bool, NSObject, NSObjectProtocol, Sel};
    use objc2::{class, define_class, msg_send, DefinedClass, MainThreadMarker};
    use objc2_foundation::NSString;
    use std::ffi::c_void;

    const USE_CREDENTIAL: isize = 0;
    const PERFORM_DEFAULT_HANDLING: isize = 1;
    const CANCEL_AUTHENTICATION_CHALLENGE: isize = 2;
    const CREDENTIAL_PERSISTENCE_FOR_SESSION: usize = 1;
    const KVO_OPTION_NEW: usize = 1;
    const MAX_AUTO_ATTEMPTS: usize = 3;

    const HTTP_BASIC_METHOD: &str = "NSURLAuthenticationMethodHTTPBasic";
    const SERVER_TRUST_METHOD: &str = "NSURLAuthenticationMethodServerTrust";

    static AUTH_DELEGATE_ASSOCIATION_KEY: u8 = 0;

    pub struct SunshineAuthDelegateIvars {
        inner: Weak<AnyObject>,
        webview: Weak<AnyObject>,
        allowed_hosts: Vec<String>,
        user: Option<String>,
        pass: Option<String>,
    }

    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "SunshineAuthenticationDelegate"]
        #[thread_kind = objc2::MainThreadOnly]
        #[ivars = SunshineAuthDelegateIvars]
        struct SunshineAuthDelegate;

        unsafe impl NSObjectProtocol for SunshineAuthDelegate {}

        impl SunshineAuthDelegate {
            #[unsafe(method(webView:didReceiveAuthenticationChallenge:completionHandler:))]
            fn did_receive_challenge(
                &self,
                _webview: &AnyObject,
                challenge: &AnyObject,
                handler: &block2::Block<dyn Fn(isize, *mut AnyObject)>,
            ) {
                unsafe {
                    let space: *mut AnyObject = msg_send![challenge, protectionSpace];
                    if space.is_null() {
                        handler.call((PERFORM_DEFAULT_HANDLING, core::ptr::null_mut()));
                        return;
                    }

                    let host: *mut NSString = msg_send![space, host];
                    let method: *mut NSString = msg_send![space, authenticationMethod];
                    if host.is_null() || method.is_null() {
                        handler.call((PERFORM_DEFAULT_HANDLING, core::ptr::null_mut()));
                        return;
                    }

                    let challenge_host = (&*host).to_string();
                    if !host_matches(&challenge_host, &self.ivars().allowed_hosts) {
                        handler.call((PERFORM_DEFAULT_HANDLING, core::ptr::null_mut()));
                        return;
                    }

                    let authentication_method = (&*method).to_string();
                    if authentication_method == SERVER_TRUST_METHOD {
                        let server_trust: *mut AnyObject = msg_send![space, serverTrust];
                        if server_trust.is_null() {
                            handler.call((PERFORM_DEFAULT_HANDLING, core::ptr::null_mut()));
                            return;
                        }
                        let credential: *mut AnyObject = msg_send![
                            class!(NSURLCredential),
                            credentialForTrust: server_trust
                        ];
                        if credential.is_null() {
                            handler.call((PERFORM_DEFAULT_HANDLING, core::ptr::null_mut()));
                        } else {
                            handler.call((USE_CREDENTIAL, credential));
                        }
                        return;
                    }

                    if authentication_method == HTTP_BASIC_METHOD {
                        let previous_failures: usize = msg_send![challenge, previousFailureCount];
                        if previous_failures >= MAX_AUTO_ATTEMPTS {
                            handler.call((CANCEL_AUTHENTICATION_CHALLENGE, core::ptr::null_mut()));
                            return;
                        }
                        if let (Some(user), Some(pass)) = (&self.ivars().user, &self.ivars().pass) {
                            let credential: *mut AnyObject = msg_send![
                                class!(NSURLCredential),
                                credentialWithUser: &*NSString::from_str(user),
                                password: &*NSString::from_str(pass),
                                persistence: CREDENTIAL_PERSISTENCE_FOR_SESSION
                            ];
                            if credential.is_null() {
                                handler.call((PERFORM_DEFAULT_HANDLING, core::ptr::null_mut()));
                            } else {
                                handler.call((USE_CREDENTIAL, credential));
                            }
                            return;
                        }
                    }

                    handler.call((PERFORM_DEFAULT_HANDLING, core::ptr::null_mut()));
                }
            }

            #[unsafe(method(respondsToSelector:))]
            fn responds_to_selector(&self, selector: Sel) -> Bool {
                let responds: Bool = unsafe { msg_send![super(self), respondsToSelector: selector] };
                if responds.as_bool() {
                    return Bool::YES;
                }
                self.ivars()
                    .inner
                    .load()
                    .map(|inner| unsafe { msg_send![&*inner, respondsToSelector: selector] })
                    .unwrap_or(Bool::NO)
            }

            #[unsafe(method(forwardingTargetForSelector:))]
            fn forwarding_target(&self, _selector: Sel) -> *mut AnyObject {
                self.ivars()
                    .inner
                    .load()
                    .map(|inner| Retained::as_ptr(&inner) as *mut AnyObject)
                    .unwrap_or(core::ptr::null_mut())
            }

            #[unsafe(method(observeValueForKeyPath:ofObject:change:context:))]
            fn observe_value(
                &self,
                key_path: &NSString,
                object: &AnyObject,
                change: *mut AnyObject,
                context: *mut c_void,
            ) {
                unsafe {
                    if key_path.to_string() != "navigationDelegate" {
                        let _: () = msg_send![
                            super(self),
                            observeValueForKeyPath: key_path,
                            ofObject: object,
                            change: change,
                            context: context
                        ];
                        return;
                    }
                    let current: *mut AnyObject = msg_send![object, navigationDelegate];
                    let self_ptr = self as *const Self as *mut AnyObject;
                    if current != self_ptr {
                        let _: () = msg_send![object, setNavigationDelegate: self];
                    }
                }
            }
        }
    );

    impl Drop for SunshineAuthDelegate {
        fn drop(&mut self) {
            if let Some(webview) = self.ivars().webview.load() {
                unsafe {
                    let key_path = NSString::from_str("navigationDelegate");
                    let self_ptr = self as *const Self as *mut AnyObject;
                    let _: () = msg_send![
                        &*webview,
                        removeObserver: self_ptr,
                        forKeyPath: &*key_path
                    ];
                }
            }
        }
    }

    impl SunshineAuthDelegate {
        fn new(
            webview: &Retained<AnyObject>,
            inner: &Retained<AnyObject>,
            allowed_hosts: Vec<String>,
            user: Option<String>,
            pass: Option<String>,
            mtm: MainThreadMarker,
        ) -> Retained<Self> {
            let this = mtm
                .alloc::<SunshineAuthDelegate>()
                .set_ivars(SunshineAuthDelegateIvars {
                    inner: Weak::from_retained(inner),
                    webview: Weak::from_retained(webview),
                    allowed_hosts,
                    user,
                    pass,
                });
            unsafe { msg_send![super(this), init] }
        }
    }

    pub fn install_and_navigate(
        webview: &tauri::webview::PlatformWebview,
        target_url: &str,
        allowed_hosts: Vec<String>,
    ) -> bool {
        let webview_ptr = webview.inner() as *mut c_void;
        if webview_ptr.is_null() {
            return false;
        }
        let Some(mtm) = MainThreadMarker::new() else {
            return false;
        };

        let user = option_env!("SUNSHINE_USER")
            .filter(|s| !s.is_empty())
            .map(str::to_owned);
        let pass = option_env!("SUNSHINE_PASS")
            .filter(|s| !s.is_empty())
            .map(str::to_owned);

        unsafe {
            let Some(wk_webview) = Retained::retain(webview_ptr as *mut AnyObject) else {
                return false;
            };
            let existing: *mut AnyObject = msg_send![&*wk_webview, navigationDelegate];
            if existing.is_null() {
                return false;
            }
            let Some(inner) = Retained::retain(existing) else {
                return false;
            };

            let proxy = SunshineAuthDelegate::new(&wk_webview, &inner, allowed_hosts, user, pass, mtm);
            objc2::ffi::objc_setAssociatedObject(
                &*wk_webview as *const AnyObject as *mut AnyObject,
                std::ptr::addr_of!(AUTH_DELEGATE_ASSOCIATION_KEY).cast(),
                Retained::as_ptr(&proxy) as *mut AnyObject,
                objc2::ffi::OBJC_ASSOCIATION_RETAIN_NONATOMIC,
            );

            let key_path = NSString::from_str("navigationDelegate");
            let observer_ptr = &*proxy as *const SunshineAuthDelegate as *mut AnyObject;
            let _: () = msg_send![
                &*wk_webview,
                addObserver: observer_ptr,
                forKeyPath: &*key_path,
                options: KVO_OPTION_NEW,
                context: core::ptr::null_mut::<c_void>()
            ];
            let _: () = msg_send![&*wk_webview, setNavigationDelegate: &*proxy];

            let ns_url: *mut AnyObject = msg_send![
                class!(NSURL),
                URLWithString: &*NSString::from_str(target_url)
            ];
            if ns_url.is_null() {
                return false;
            }
            let request: *mut AnyObject = msg_send![class!(NSURLRequest), requestWithURL: ns_url];
            if request.is_null() {
                return false;
            }
            let _: *mut AnyObject = msg_send![&*wk_webview, loadRequest: request];
            true
        }
    }
}

fn main() {
    let target_url = option_env!("SUNSHINE_URL")
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("https://localhost:47990");

    let parsed_url: url::Url = target_url.parse().expect("Invalid SUNSHINE_URL");
    let allowed_hosts = build_allowed_hosts(&parsed_url);

    tauri::Builder::default()
        .setup(move |app| {
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            let initial_url = WebviewUrl::External("about:blank".parse().unwrap());
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            let initial_url = WebviewUrl::External(parsed_url.clone());

            let builder = WebviewWindowBuilder::new(app, "main", initial_url)
                .title("SunshineApp")
                .inner_size(1200.0, 780.0)
                .center();

            #[cfg(target_os = "windows")]
            let builder = builder.additional_browser_args("--ignore-certificate-errors");

            let window = builder.build()?;

            #[cfg(target_os = "windows")]
            {
                let nav_url = parsed_url.as_str().to_owned();
                let hosts = allowed_hosts.clone();
                if let Err(error) = window.with_webview(move |webview| {
                    windows_auth::install_and_navigate(&webview, &nav_url, hosts);
                }) {
                    eprintln!("[sunshine-app] Failed to access WebView2: {error}");
                }
            }

            #[cfg(target_os = "macos")]
            {
                let nav_url = parsed_url.as_str().to_owned();
                let hosts = allowed_hosts.clone();
                if let Err(error) = window.with_webview(move |webview| {
                    if !macos_auth::install_and_navigate(&webview, &nav_url, hosts) {
                        eprintln!("[sunshine-app] Failed to install macOS WKWebView auth delegate");
                    }
                }) {
                    eprintln!("[sunshine-app] Failed to access WKWebView: {error}");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running sunshine-app");
}
