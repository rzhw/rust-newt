use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;

pub type ComponentPtr = u32;

// Design goal: Be a light wrapper. Side effects (inherent to UI programming) are okay.
// (An alternative wrapper which attempts to make this functional is possible but detracts
// from the simplicity of newt.)

#[link(name = "newt")]
extern {
    fn newtInit();
    fn newtCls();
    fn newtWaitForKey();
    fn newtDrawRootText(col: i32, row: i32, text: *const i8);
    fn newtFinished();
    fn newtCenteredWindow(width: i32, height: i32, text: *const i8) -> i32;
    fn newtForm(vertBar: ComponentPtr, help: *const i8, flags: i32) -> ComponentPtr;
    fn newtFormAddComponent(form: ComponentPtr, co: ComponentPtr);
    fn newtFormDestroy(form: ComponentPtr);
    fn newtRunForm(form: ComponentPtr) -> ComponentPtr;
    fn newtButton(left: i32, top: i32, text: *const i8) -> ComponentPtr;
    fn newtLabel(left: i32, top: i32, text: *const i8) -> ComponentPtr;
    fn newtEntry(left: i32, top: i32, initialValue: *const i8, width: i32, resultPtr: *mut *mut i8, flags: i32) -> ComponentPtr;
    fn newtEntryGetValue(co: ComponentPtr) -> *const i8;
}

pub fn init() {
    unsafe { newtInit(); }
}

pub fn cls() {
    unsafe { newtCls(); }
}

pub fn wait_for_key() {
    unsafe { newtWaitForKey(); }
}

pub fn finished() {
    unsafe { newtFinished(); }
}

pub fn draw_root_text(col: i32, row: i32, text: &str) {
    unsafe {
        // This may not be the most efficient way to do so but it works on 1.0 beta
        newtDrawRootText(col, row, CString::new(text).unwrap().as_ptr());
    }
}

pub fn centered_window(width: i32, height: i32, text: &str) {
    unsafe {
        newtCenteredWindow(width, height, CString::new(text).unwrap().as_ptr());
    }
}

// TODO: flag enum
pub fn form(vertical_bar: Option<ComponentPtr>, help: Option<&str>, flags: i32) -> ComponentPtr {
    let vert_bar_ptr = match vertical_bar {
        Some(x) => x,
        None => 0
    };
    let help_ptr = match help {
        Some(x) => CString::new(x).unwrap().as_ptr(),
        None => ptr::null()
    };
    unsafe {
        newtForm(vert_bar_ptr, help_ptr, flags)
    }
}

pub fn run_form(form: ComponentPtr) -> ComponentPtr {
    unsafe {
        newtRunForm(form)
    }
}

pub fn form_add_component(form: ComponentPtr, component: ComponentPtr) {
    unsafe {
        newtFormAddComponent(form, component)
    }
}

pub fn form_destroy(form: ComponentPtr) {
    unsafe {
        newtFormDestroy(form)
    }
}

pub fn button(left: i32, top: i32, text: &str) -> ComponentPtr {
    unsafe {
        newtButton(left, top, CString::new(text).unwrap().as_ptr())
    }
}

pub fn label(left: i32, top: i32, text: &str) -> ComponentPtr {
    unsafe {
        newtLabel(left, top, CString::new(text).unwrap().as_ptr())
    }
}

pub fn entry(left: i32, top: i32, initial_value: Option<&str>, width: i32, flags: i32) -> ComponentPtr {
    let initial_value_ptr = match initial_value {
        Some(x) => CString::new(x).unwrap().as_ptr(),
        None => ptr::null()
    };
    unsafe {
        newtEntry(left, top, initial_value_ptr, width, 0 as *mut *mut i8, flags)
    }
}

pub fn entry_get_value(component: ComponentPtr) -> String {
    let ptr = unsafe { newtEntryGetValue(component) };
    let cstr = unsafe { CStr::from_ptr(ptr) };
    let buf = cstr.to_bytes();
    String::from_utf8(buf.to_vec()).unwrap()
}
