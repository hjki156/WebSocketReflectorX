pub mod bridges;
pub mod daemon;
pub mod launcher;
pub mod logging;
use slint::ComponentHandle;
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub mod ui {
    slint::include_modules!();
}

// Android 专用模块
#[cfg(target_os = "android")]
mod android {
    use android_logger;
    use log::Level;

    pub fn init_logger() {
        android_logger::init_once(
            android_logger::Config::default()
                .with_tag("WebSocketReflectorX")
                .with_min_level(Level::Info),
        );
        log::info!("Android logger initialized");
    }
}

// 桌面平台初始化（保持原样）
#[cfg(not(target_os = "android"))]
fn init_platform() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(target_os = "macos"))]
    {
        slint::platform::set_platform(Box::new(
            i_slint_backend_winit::Backend::new()
                .map_err(|e| format!("Failed to initialize winit backend: {}", e))?,
        ))?;
    }

    #[cfg(target_os = "macos")]
    {
        use winit::platform::macos::WindowAttributesExtMacOS;
        let mut backend = i_slint_backend_winit::Backend::new()
            .map_err(|e| format!("Failed to create winit backend: {}", e))?;
        backend.window_attributes_hook = Some(Box::new(|attr| {
            attr.with_fullsize_content_view(true)
                .with_title_hidden(true)
                .with_titlebar_transparent(true)
        }));
        slint::platform::set_platform(Box::new(backend))?;
    }

    Ok(())
}

// Android 平台初始化（无须手动设置 backend）
#[cfg(target_os = "android")]
fn init_platform() -> Result<(), Box<dyn std::error::Error>> {
    // Slint Android backend 会自动初始化
    // 我们只需要初始化日志等
    android::init_logger();
    Ok(())
}

/// 启动 UI 应用的核心逻辑
pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化平台后端
    init_platform()?;

    // 启动 launcher（你的主 UI）
    let ui = crate::launcher::setup()?;
    ui.run()?;

    Ok(())
}

/// Android 入口点（必须导出）
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main() {
    // 在 Android 上运行应用
    if let Err(e) = run_app() {
        log::error!("Application error: {}", e);
    }
}