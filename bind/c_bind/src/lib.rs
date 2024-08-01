use std::ffi::CString;
use std::os::raw::c_void;

use libloading::{os::unix::Symbol as RawSymbol, Library, Symbol};

#[repr(C)]
struct Object {
    _private: [u8; 0],
}

type Create = extern "C" fn () -> *mut Object;
type Load = extern "C" fn (*mut Object, *const i8, *const i8) -> bool;
type Close = extern "C" fn (*mut Object) -> bool;

type Read = extern "C" fn (*mut Object, *const i8, *mut c_void, isize) -> bool;
type Write = extern "C" fn (*mut Object, *const i8, *mut c_void, isize) -> bool;
type Cleanup = extern "C" fn (*mut Object, *const i8) -> bool;

pub struct PluginManager {
    object: *mut Object,
    #[allow(dead_code)]
    library: Library,
    on_close: RawSymbol<Close>,
    on_load: RawSymbol <Load>,
    on_read: RawSymbol <Read>,
    on_write: RawSymbol <Write>,
    on_cleanup: RawSymbol <Cleanup>,
}

impl PluginManager {
    pub fn new () -> Self {

        unsafe {
            let path = "plugins/libmanager.so";
            let library = Library::new (path).unwrap();

            let plugin_manager_create: Symbol<Create> = library.get (b"plugin_manager_create\0").unwrap();

            let plugin_manager_close: Symbol<Close> = library.get(b"plugin_manager_close\0").unwrap();
            let plugin_manager_close = plugin_manager_close.into_raw();

            let plugin_manager_load: Symbol<Load> = library.get(b"plugin_manager_load\0").unwrap();
            let plugin_manager_load = plugin_manager_load.into_raw();

            let plugin_manager_read: Symbol<Read> = library.get(b"plugin_manager_read\0").unwrap();
            let plugin_manager_read = plugin_manager_read.into_raw();

            let plugin_manager_write: Symbol<Write> = library.get(b"plugin_manager_write\0").unwrap();
            let plugin_manager_write = plugin_manager_write.into_raw();

            let plugin_manager_cleanup: Symbol<Cleanup> = library.get(b"plugin_manager_cleanup\0").unwrap();
            let plugin_manager_cleanup = plugin_manager_cleanup.into_raw();

            let pm: *mut Object = plugin_manager_create ();

            PluginManager {
                object: pm,
                library: library,
                on_close: plugin_manager_close,
                on_load: plugin_manager_load,
                on_read: plugin_manager_read,
                on_write: plugin_manager_write,
                on_cleanup: plugin_manager_cleanup,
            }
        }
    }

    pub fn close (&mut self) -> bool {
        (self.on_close) (self.object)
    }

    pub fn load (&mut self, name: String, path: String) -> bool {
        let c_name = CString::new(name).expect ("Failed to create CString");
        let c_path = CString::new(path).expect ("Failed to create CString");

        (self.on_load) (self.object, c_name.as_ptr (), c_path.as_ptr())
    }

    pub fn read (&mut self, device: String, data: &[u8]) -> bool {
        let c_device = CString::new (device).expect ("Failed to create CString");
        let c_data: *mut c_void = &data as *const _ as *mut c_void;

        (self.on_read) (self.object, c_device.as_ptr (), c_data, data.len() as isize)
    }

    pub fn write (&mut self, device: String, data: &[u8]) -> bool {
        let c_device = CString::new (device).expect ("Failed to create CString");
        let c_data: *mut c_void = &data as *const _ as *mut c_void;

        (self.on_write) (self.object, c_device.as_ptr (), c_data, data.len() as isize)
    }

    pub fn cleanup (&mut self, device: String) -> bool {
        let c_device = CString::new (device).expect ("Failed to create CString");

        (self.on_cleanup) (self.object, c_device.as_ptr ())
    }
}