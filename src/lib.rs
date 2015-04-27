use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;

pub type NewtComponentPtr = u32;

pub trait Component {
    fn get_ptr(&self) -> NewtComponentPtr;
}

#[link(name = "newt")]
extern {
    fn newtInit();
    fn newtCls();
    fn newtWaitForKey();
    fn newtDrawRootText(col: i32, row: i32, text: *const i8);
    fn newtFinished();
    fn newtCenteredWindow(width: i32, height: i32, text: *const i8) -> i32;
    fn newtForm(vertBar: NewtComponentPtr, help: *const i8, flags: i32) -> NewtComponentPtr;
    fn newtFormAddComponent(form: NewtComponentPtr, co: NewtComponentPtr);
    fn newtFormDestroy(form: NewtComponentPtr);
    fn newtRunForm(form: NewtComponentPtr) -> NewtComponentPtr;
    fn newtButton(left: i32, top: i32, text: *const i8) -> NewtComponentPtr;
    fn newtLabel(left: i32, top: i32, text: *const i8) -> NewtComponentPtr;
    fn newtEntry(left: i32, top: i32, initialValue: *const i8, width: i32, resultPtr: *mut *mut i8, flags: i32) -> NewtComponentPtr;
    fn newtEntryGetValue(co: NewtComponentPtr) -> *const i8;
}

// Wonder if this could be made nicer by returning a var that Rust could clean up for us
// by implementing the Drop trait
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

pub fn label(width: i32, height: i32, text: &str) {
    unsafe {
        newtLabel(width, height, CString::new(text).unwrap().as_ptr());
    }
}

pub struct Form {
    pub ptr: NewtComponentPtr
}
impl Form {
    // TODO: support flags
    pub fn new(vertical_bar: Option<Scrollbar>, help: Option<&str>, flags: i32) -> Form {
        let vert_bar_ptr = match vertical_bar {
            Some(s) => s.ptr,
            None => 0
        };
        let help_ptr = match help {
            Some(s) => CString::new(s).unwrap().as_ptr(),
            None => ptr::null()
        };
        Form { ptr: unsafe { newtForm(vert_bar_ptr, help_ptr, flags) } }
    }

    pub fn run(&self) -> NewtComponentPtr { // TODO: with the object model returning a ptr isn't useful
        unsafe {
            newtRunForm(self.ptr)
        }
    }

    pub fn add_component(&self, component: &Component) {
        unsafe {
            newtFormAddComponent(self.ptr, component.get_ptr())
        }
    }
}
impl Component for Form {
    fn get_ptr(&self) -> NewtComponentPtr { self.ptr }
}
impl Drop for Form {
    fn drop(&mut self) {
        unsafe {
            newtFormDestroy(self.ptr);
        }
    }
}

pub struct Button {
    pub ptr: NewtComponentPtr
}
impl Button {
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        Button { ptr: unsafe { newtButton(left, top, CString::new(text).unwrap().as_ptr()) } }
    }
}
impl Component for Button {
    fn get_ptr(&self) -> NewtComponentPtr { self.ptr }
}

pub struct Label {
    pub ptr: NewtComponentPtr
}
impl Label {
    pub fn new(left: i32, top: i32, text: &str) -> Label {
        Label { ptr: unsafe { newtLabel(left, top, CString::new(text).unwrap().as_ptr()) } }
    }
}
impl Component for Label {
    fn get_ptr(&self) -> NewtComponentPtr { self.ptr }
}

pub struct Entry {
    pub ptr: NewtComponentPtr
}
impl Entry {
    pub fn new(left: i32, top: i32, initial_value: Option<&str>, width: i32, flags: i32) -> Entry {
        let initial_value_ptr = match initial_value {
            Some(v) => CString::new(v).unwrap().as_ptr(),
            None => ptr::null()
        };
        Entry { ptr: unsafe { newtEntry(left, top, initial_value_ptr, width, 0 as *mut *mut i8, flags) } }
    }

    // TODO: set

    pub fn get_value(&self) -> String {
        let ptr = unsafe { newtEntryGetValue(self.ptr) };
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let buf = cstr.to_bytes();
        String::from_utf8(buf.to_vec()).unwrap()
    }

    // TODO: set filter
}
impl Component for Entry {
    fn get_ptr(&self) -> NewtComponentPtr { self.ptr }
}

pub struct Scrollbar {
    pub ptr: NewtComponentPtr
}
impl Component for Scrollbar {
    fn get_ptr(&self) -> NewtComponentPtr { self.ptr }
}
