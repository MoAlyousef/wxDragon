use crate::dialogs::Dialog;
use crate::widgets::{Frame, Panel};
use crate::window::WxWidget;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use wxdragon_sys as ffi;

/// Represents the global wxXmlResource object.
#[derive(Clone)] // Cloning just copies the pointer to the singleton.
pub struct XmlResource {
    ptr: *mut ffi::wxd_XmlResource_t,
    // Ensures that XmlResource is !Send and !Sync if wxXmlResource is not thread-safe,
    // which is typical for GUI singletons.
    _phantom: PhantomData<*mut ()>,
}

impl XmlResource {
    /// Get the global wxXmlResource instance
    pub fn get() -> Self {
        Self {
            ptr: unsafe { ffi::wxd_XmlResource_Get() },
            _phantom: PhantomData,
        }
    }

    /// Initialize all standard handlers (idempotent)
    pub fn init_all_handlers(&self) {
        unsafe {
            ffi::wxd_XmlResource_InitAllHandlers(self.ptr);
        }
    }

    /// Load XRC from file
    pub fn load_from_file(&self, filename: &str) -> Result<(), String> {
        let c_filename = CString::new(filename).map_err(|_| "Invalid filename")?;
        let success = unsafe { ffi::wxd_XmlResource_LoadFromFile(self.ptr, c_filename.as_ptr()) };
        if success {
            Ok(())
        } else {
            Err(format!("Failed to load XRC file: {}", filename))
        }
    }

    /// Load XRC from string data
    pub fn load_from_string(&self, xrc_data: &str) -> Result<(), String> {
        let c_data = CString::new(xrc_data).map_err(|_| "Invalid XRC data")?;
        let success = unsafe { ffi::wxd_XmlResource_LoadFromString(self.ptr, c_data.as_ptr()) };
        if success {
            Ok(())
        } else {
            Err("Failed to load XRC from string".to_string())
        }
    }

    /// Load a dialog from XRC
    pub fn load_dialog(&self, parent: Option<&dyn WxWidget>, name: &str) -> Option<Dialog> {
        let c_name = CString::new(name).ok()?;
        let parent_ptr = parent.map_or(ptr::null_mut(), |p| p.handle_ptr());

        let dialog_ptr =
            unsafe { ffi::wxd_XmlResource_LoadDialog(self.ptr, parent_ptr, c_name.as_ptr()) };

        if dialog_ptr.is_null() {
            None
        } else {
            Some(Dialog::from_xrc_ptr(dialog_ptr))
        }
    }

    /// Load a frame from XRC
    pub fn load_frame(&self, parent: Option<&dyn WxWidget>, name: &str) -> Option<Frame> {
        let c_name = CString::new(name).ok()?;
        let parent_ptr = parent.map_or(ptr::null_mut(), |p| p.handle_ptr());

        let frame_ptr =
            unsafe { ffi::wxd_XmlResource_LoadFrame(self.ptr, parent_ptr, c_name.as_ptr()) };

        if frame_ptr.is_null() {
            None
        } else {
            Some(<Frame as FromXrcPtr>::from_xrc_ptr(
                frame_ptr as *mut ffi::wxd_Window_t,
            ))
        }
    }

    /// Load a panel from XRC
    pub fn load_panel(&self, parent: Option<&dyn WxWidget>, name: &str) -> Option<Panel> {
        let c_name = CString::new(name).ok()?;
        let parent_ptr = parent.map_or(ptr::null_mut(), |p| p.handle_ptr());

        let panel_ptr =
            unsafe { ffi::wxd_XmlResource_LoadPanel(self.ptr, parent_ptr, c_name.as_ptr()) };

        if panel_ptr.is_null() {
            None
        } else {
            Some(<Panel as FromXrcPtr>::from_xrc_ptr(
                panel_ptr as *mut ffi::wxd_Window_t,
            ))
        }
    }

    /// Get XRC ID for a control name
    pub fn get_xrc_id(name: &str) -> i32 {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::wxd_XmlResource_GetXRCID(c_name.as_ptr()) }
    }

    /// Returns the raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is used correctly.
    pub(crate) unsafe fn as_ptr(&self) -> *mut ffi::wxd_XmlResource_t {
        self.ptr
    }
}

/// Trait for creating widgets from XRC-managed pointers
pub trait FromXrcPtr {
    type RawFfiType;

    /// Create a widget wrapper from an XRC-managed pointer
    /// The widget does not own the pointer - XRC manages its lifetime
    fn from_xrc_ptr(ptr: Self::RawFfiType) -> Self;
}

/// Trait for widgets that support XRC loading
/// Widgets implement this trait to provide XRC support with their specific structure
pub trait XrcSupport: WxWidget + Sized {
    /// Creates a widget wrapper for an XRC-managed object.
    /// This widget will not be destroyed when dropped as it's managed by XRC.
    /// Each widget implements this with their specific field structure.
    fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self;
}

/// Automatically implement FromXrcPtr for any widget that implements XrcSupport
impl<T: XrcSupport> FromXrcPtr for T {
    type RawFfiType = *mut ffi::wxd_Window_t;

    fn from_xrc_ptr(ptr: Self::RawFfiType) -> Self {
        T::from_xrc_ptr(ptr)
    }
}

/// Trait for XRC-specific window methods
pub trait WindowXrcMethods: WxWidget + Sized {
    /// Find a child window by XRC name
    fn find_child_by_xrc_name<T: FromXrcPtr<RawFfiType = *mut ffi::wxd_Window_t> + WxWidget>(
        &self,
        name: &str,
    ) -> Option<T> {
        let c_name = CString::new(name).ok()?;
        let child_ptr =
            unsafe { ffi::wxd_Window_FindWindowByXRCName(self.handle_ptr(), c_name.as_ptr()) };

        if child_ptr.is_null() {
            None
        } else {
            Some(T::from_xrc_ptr(child_ptr))
        }
    }
}

// Implement for all WxWidget types
impl<W: WxWidget> WindowXrcMethods for W {}

// FromXrcPtr implementations are now handled automatically by the implement_widget_traits_with_target! macro
// TODO: Add more FromXrcPtr implementations for other widget types as needed
