use super::*;

/// Sets the global application state.
///
/// # Arguments
///
/// - `Arc<AppState>` - The shared application state to store globally.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure of the initialization.
pub(crate) fn set_global_state(state: Arc<AppState>) -> Result<(), EuvError> {
    APP_STATE.set(state).map_err(|_: Arc<AppState>| {
        EuvError::Message(String::from("Global state already initialized"))
    })
}

/// Retrieves the global application state.
///
/// # Returns
///
/// - `Option<Arc<AppState>>` - The global state if initialized.
pub(crate) fn get_global_state() -> Option<Arc<AppState>> {
    APP_STATE.get().cloned()
}

/// Generates `index.html` based on the build profile.
///
/// Uses `INDEX_HTML_RELEASE` when `is_release` is `true` (no live-reload script),
/// otherwise uses `INDEX_HTML_DEV` (includes live-reload instrumentation).
///
/// Then writes the template with the import path placeholder replaced to disk.
///
/// # Arguments
///
/// - `&HtmlConfig` - The HTML generation configuration.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The generated HTML content written to disk.
pub(crate) async fn generate_html(config: &HtmlConfig) -> Result<String, EuvError> {
    let template_content: String = if let Some(custom_path) = config.try_get_custom_index_html() {
        let bytes: Vec<u8> =
            read(custom_path)
                .await
                .map_err(|error: io::Error| EuvError::IoPath {
                    message: String::from("Failed to read custom index.html"),
                    path: custom_path.to_path_buf(),
                    error,
                })?;
        String::from_utf8(bytes).map_err(|error: FromUtf8Error| EuvError::Utf8 {
            message: String::from("Custom index.html is not valid UTF-8"),
            error,
        })?
    } else if config.get_is_release() {
        INDEX_HTML_RELEASE.to_string()
    } else {
        INDEX_HTML_DEV.to_string()
    };
    let html: String = minify_html_template(
        &template_content
            .replace(IMPORT_PATH_PLACEHOLDER, config.get_import_path())
            .replace(RELOAD_ROUTE_PLACEHOLDER, RELOAD_ROUTE),
    );
    let index_path: PathBuf = config.get_serving_root().join(INDEX_HTML_FILE_NAME);
    create_dir_all(config.get_serving_root())
        .await
        .map_err(|error: io::Error| EuvError::Io {
            message: String::from("Failed to create static directory"),
            error,
        })?;
    write(&index_path, &html)
        .await
        .map_err(|error: io::Error| EuvError::Io {
            message: String::from("Failed to write index.html"),
            error,
        })?;
    Ok(html)
}

/// Resolves the effective www directory, handling wasm-pack nested output.
///
/// # Arguments
///
/// - `&Path` - The candidate www directory path.
///
/// # Returns
///
/// - `PathBuf` - The resolved www directory containing `index.html`.
pub async fn resolve_www_dir(www_dir: &Path) -> PathBuf {
    if metadata(www_dir.join(INDEX_HTML_FILE_NAME)).await.is_ok() {
        return www_dir.to_path_buf();
    }
    let parent_name: Option<&str> = www_dir
        .file_name()
        .and_then(|file_name_os_str: &ffi::OsStr| file_name_os_str.to_str());
    if let Some(name) = parent_name {
        let nested: PathBuf = www_dir.join(name);
        if metadata(nested.join(INDEX_HTML_FILE_NAME)).await.is_ok() {
            return nested;
        }
    }
    www_dir.to_path_buf()
}

/// Resolves the pkg directory for serving WASM artifacts.
///
/// Delegates to `resolve_out_dir` which respects `--out-dir`
/// from wasm-pack args or defaults to `{www_dir}/pkg`.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments for resolving out_dir.
///
/// # Returns
///
/// - `PathBuf` - The resolved pkg directory containing WASM build artifacts.
pub fn resolve_pkg_dir(args: &ModeArgs) -> PathBuf {
    resolve_out_dir(args)
}

/// Resolves a file path within a base directory with path-traversal protection.
///
/// Canonicalizes both the file path and the base directory, then verifies
/// that the canonical file path starts with the canonical base directory.
///
/// # Arguments
///
/// - `&Path` - The base directory to resolve within.
/// - `&str` - The relative path to the file.
///
/// # Returns
///
/// - `Option<PathBuf>` - The resolved file path if valid, `None` otherwise.
pub async fn resolve_file_in_base(base: &Path, path: &str) -> Option<PathBuf> {
    let file_path: PathBuf = base.join(path);
    let canonical_path: PathBuf = canonicalize(&file_path).await.ok()?;
    let base_canonical: PathBuf = canonicalize(base).await.ok()?;
    canonical_path
        .starts_with(&base_canonical)
        .then_some(file_path)
}
