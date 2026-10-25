//! An in-process harness for the Minori runtime provider.
//!
//! The Aether host drives the runtime through the C ABI in
//! `engine_runtime_provider.h`. This module speaks that same ABI directly, so a
//! test binary can open a real installation, tick it and read back frames
//! without any host, GPU or window system. Nothing here replaces the host path:
//! it calls the very same `Provider` callbacks the host calls.

use std::ffi::{CStr, CString, c_char, c_void};
use std::path::Path;
use std::ptr;
use std::sync::Mutex;

use crate::provider::{
    EngineCreateDesc, EngineFrameDesc, EngineInputEvent, EngineRuntimeHost, api_version, descriptor,
};

const INPUT_POINTER_DOWN: u32 = 1;
const INPUT_POINTER_UP: u32 = 2;
const INPUT_KEY_DOWN: u32 = 5;
const PIXEL_RGBA8888: u32 = 1;

pub struct Headless {
    opaque: *mut c_void,
    root: String,
    // Box keeps the callback user-data address stable when the harness moves.
    requests: Box<RequestQueue>,
}

// The provider owns a boxed Runtime and never shares it across threads; the
// harness is single-threaded by construction.
unsafe impl Send for Headless {}

/// Platform requests the runtime issued, in order. The host normally services
/// these (audio playback, message text on screen, dialogs); the harness records
/// them so an acceptance run can assert the story actually reached them.
type RequestQueue = Mutex<Vec<(String, String)>>;

unsafe extern "C" fn record_request(user_data: *mut c_void, operation: *const c_char, argument: *const c_char) {
    if user_data.is_null() || operation.is_null() {
        return;
    }
    let name = unsafe { CStr::from_ptr(operation) }
        .to_string_lossy()
        .into_owned();
    let value = if argument.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(argument) }
            .to_string_lossy()
            .into_owned()
    };
    // The owning Headless keeps this queue alive through provider destruction.
    let queue = unsafe { &*(user_data as *const RequestQueue) };
    if let Ok(mut requests) = queue.lock() {
        requests.push((name, value));
    }
}

fn host_struct(requests: &RequestQueue) -> EngineRuntimeHost {
    EngineRuntimeHost {
        struct_size: std::mem::size_of::<EngineRuntimeHost>() as u32,
        api_version: api_version(),
        user_data: requests as *const RequestQueue as *mut c_void,
        log: None,
        monotonic_time_micros: None,
        platform_request: Some(record_request),
        reserved_u64: [0; 4],
        reserved_ptr: [ptr::null_mut(); 3],
    }
}

impl Headless {
    /// Create a runtime and open a game installation at `root`.
    pub fn open(root: &Path, writable: &Path) -> Result<Self, String> {
        Self::open_with_options(root, writable, &[])
    }

    /// Set startup options before loading any script or emitting game requests.
    pub fn open_with_options(
        root: &Path,
        writable: &Path,
        options: &[(&str, &str)],
    ) -> Result<Self, String> {
        let provider = descriptor();
        let create = provider.create.ok_or("provider has no create")?;
        let open_game = provider.open_game.ok_or("provider has no open_game")?;

        let requests = Box::new(Mutex::new(Vec::new()));
        let host = host_struct(&requests);
        let writable_c = CString::new(writable.to_string_lossy().as_bytes())
            .map_err(|_| "writable path contains a NUL".to_string())?;
        let cache_c = writable_c.clone();
        let desc = EngineCreateDesc {
            struct_size: std::mem::size_of::<EngineCreateDesc>() as u32,
            api_version: api_version(),
            writable_path_utf8: writable_c.as_ptr(),
            cache_path_utf8: cache_c.as_ptr(),
            user_data: ptr::null_mut(),
            reserved_u64: [0; 4],
            reserved_ptr: [ptr::null_mut(); 4],
        };
        let mut opaque: *mut c_void = ptr::null_mut();
        let result = unsafe { create(ptr::null_mut(), &host, &desc, &mut opaque) };
        if result != 0 || opaque.is_null() {
            return Err(format!("create failed with {result}"));
        }
        let mut harness = Self {
            opaque,
            root: root.to_string_lossy().into_owned(),
            requests,
        };

        for &(key, value) in options {
            harness.set_option(key, value)?;
        }

        let root_c = CString::new(root.to_string_lossy().as_bytes())
            .map_err(|_| "game root contains a NUL".to_string())?;
        let result = unsafe { open_game(harness.opaque, root_c.as_ptr(), ptr::null()) };
        if result != 0 {
            let error = harness.last_error();
            return Err(format!("open_game failed with {result}: {error}"));
        }
        Ok(harness)
    }

    /// Drain only this runtime's platform requests, in callback order.
    pub fn take_requests(&mut self) -> Vec<(String, String)> {
        self.requests
            .lock()
            .map(|mut requests| std::mem::take(&mut *requests))
            .unwrap_or_default()
    }

    pub fn set_option(&mut self, key: &str, value: &str) -> Result<(), String> {
        let provider = descriptor();
        let set_option = provider.set_option.ok_or("provider has no set_option")?;
        let key_c = CString::new(key).map_err(|_| "option key contains a NUL".to_string())?;
        let value_c = CString::new(value).map_err(|_| "option value contains a NUL".to_string())?;
        let option = crate::provider::EngineOption {
            key_utf8: key_c.as_ptr(),
            value_utf8: value_c.as_ptr(),
            reserved_u64: [0; 2],
            reserved_ptr: [ptr::null_mut(); 2],
        };
        let result = unsafe { set_option(self.opaque, &option as *const _ as *const c_void) };
        if result != 0 {
            return Err(format!("set_option({key}) failed with {result}: {}", self.last_error()));
        }
        Ok(())
    }

    pub fn tick(&mut self, delta_ms: u32) -> Result<(), String> {
        let provider = descriptor();
        let tick = provider.tick.ok_or("provider has no tick")?;
        let result = unsafe { tick(self.opaque, delta_ms) };
        if result != 0 {
            return Err(format!("tick failed with {result}: {}", self.last_error()));
        }
        Ok(())
    }

    /// Frame geometry plus RGBA pixels, if a scene is live.
    pub fn frame(&mut self) -> Option<(u32, u32, Vec<u8>)> {
        let provider = descriptor();
        let get_frame_desc = provider.get_frame_desc?;
        let read_frame = provider.read_frame_rgba?;
        let mut desc = EngineFrameDesc {
            struct_size: std::mem::size_of::<EngineFrameDesc>() as u32,
            width: 0,
            height: 0,
            stride_bytes: 0,
            pixel_format: 0,
            frame_serial: 0,
            reserved_u64: [0; 4],
            reserved_ptr: [ptr::null_mut(); 4],
        };
        if unsafe { get_frame_desc(self.opaque, &mut desc) } != 0 {
            return None;
        }
        if desc.pixel_format != PIXEL_RGBA8888 {
            return None;
        }
        let mut rgba = vec![0u8; desc.width as usize * desc.height as usize * 4];
        let result =
            unsafe { read_frame(self.opaque, rgba.as_mut_ptr() as *mut c_void, rgba.len()) };
        if result != 0 {
            return None;
        }
        Some((desc.width, desc.height, rgba))
    }

    fn send(&mut self, event_type: u32, x: f64, y: f64) -> Result<(), String> {
        let provider = descriptor();
        let send_input = provider.send_input.ok_or("provider has no send_input")?;
        let event = EngineInputEvent {
            struct_size: std::mem::size_of::<EngineInputEvent>() as u32,
            event_type,
            timestamp_micros: 0,
            x,
            y,
            delta_x: 0.0,
            delta_y: 0.0,
            pointer_id: 0,
            button: 0,
            key_code: 0,
            modifiers: 0,
            unicode_codepoint: 0,
            reserved_u32: 0,
            reserved_u64: [0; 2],
            reserved_ptr: [ptr::null_mut(); 2],
        };
        let result = unsafe { send_input(self.opaque, &event as *const _ as *const c_void) };
        if result != 0 {
            return Err(format!("send_input failed with {result}: {}", self.last_error()));
        }
        Ok(())
    }

    pub fn click(&mut self, x: f64, y: f64) -> Result<(), String> {
        self.send(INPUT_POINTER_DOWN, x, y)?;
        self.send(INPUT_POINTER_UP, x, y)
    }

    /// One key press, which is how the corpus advances a message or a wait.
    pub fn press(&mut self) -> Result<(), String> {
        self.send(INPUT_KEY_DOWN, 0.0, 0.0)
    }

    /// Start the game again, picking up a changed `minori.start`.
    /// Answer a pending platform request, exactly as the Aether host does.
    ///
    /// The engine blocks in `WaitingSelect` until the host replies through
    /// `submit_platform_response`, so a headless run has to answer too --
    /// otherwise the story stops dead at the first choice.
    pub fn submit_response(&mut self, operation: &str, argument: &str) -> Result<(), String> {
        let provider = descriptor();
        let submit = provider
            .submit_platform_response
            .ok_or("provider has no submit_platform_response")?;
        let operation = CString::new(operation).map_err(|e| format!("operation: {e}"))?;
        let argument = CString::new(argument).map_err(|e| format!("argument: {e}"))?;
        let status = unsafe { submit(self.opaque, operation.as_ptr(), argument.as_ptr()) };
        if status == 0 {
            Ok(())
        } else {
            Err(format!(
                "submit_platform_response({operation:?}) failed with {status}: {}",
                self.last_error()
            ))
        }
    }

    pub fn restart(&mut self) -> Result<(), String> {
        let provider = descriptor();
        let open_game = provider.open_game.ok_or("provider has no open_game")?;
        let root = CString::new(self.root.clone()).map_err(|_| "game root has a NUL".to_string())?;
        let result = unsafe { open_game(self.opaque, root.as_ptr(), ptr::null()) };
        if result != 0 {
            return Err(format!("restart failed with {result}: {}", self.last_error()));
        }
        Ok(())
    }

    pub fn debug_info(&mut self) -> String {
        let provider = descriptor();
        let Some(get_plugin_debug_info) = provider.get_plugin_debug_info else {
            return String::new();
        };
        let mut buffer = vec![0i8; 8192];
        let mut written = 0u32;
        let result = unsafe {
            get_plugin_debug_info(
                self.opaque,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                &mut written,
            )
        };
        if result != 0 {
            return String::new();
        }
        unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn last_error(&mut self) -> String {
        let provider = descriptor();
        let Some(get_last_error) = provider.get_last_error else {
            return String::new();
        };
        let value = unsafe { get_last_error(self.opaque) };
        if value.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(value) }
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callback_queues_are_isolated_and_survive_owner_moves() {
        let first = Box::new(Mutex::new(Vec::new()));
        let second = Box::new(Mutex::new(Vec::new()));
        let host = host_struct(&first);
        let other_host = host_struct(&second);
        let moved = first;
        let operation = CString::new("minori_select").unwrap();
        let argument = CString::new("choice one").unwrap();
        unsafe {
            host.platform_request.unwrap()(host.user_data, operation.as_ptr(), argument.as_ptr());
            other_host.platform_request.unwrap()(other_host.user_data, operation.as_ptr(), ptr::null());
            record_request(ptr::null_mut(), operation.as_ptr(), argument.as_ptr());
            record_request(host.user_data, ptr::null(), argument.as_ptr());
        }
        assert_eq!(*moved.lock().unwrap(), vec![("minori_select".into(), "choice one".into())]);
        assert_eq!(*second.lock().unwrap(), vec![("minori_select".into(), String::new())]);
    }

    #[test]
    fn startup_options_are_validated_before_game_open() {
        let result = Headless::open_with_options(
            Path::new("missing-game-root"),
            &std::env::temp_dir(),
            &[("minori.language", "invalid-language")],
        );
        let error = result.err().expect("invalid language must fail");
        assert!(error.contains("set_option(minori.language)"), "{error}");
        assert!(error.contains("unknown script language"), "{error}");
    }
}

impl Drop for Headless {
    fn drop(&mut self) {
        if let Some(destroy) = descriptor().destroy {
            unsafe { destroy(self.opaque) };
        }
        self.opaque = ptr::null_mut();
    }
}
