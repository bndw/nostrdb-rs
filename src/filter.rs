use crate::bindings;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;

#[derive(Debug)]
pub struct Filter {
    data: bindings::ndb_filter,
}

impl bindings::ndb_filter {
    fn as_ptr(&self) -> *const bindings::ndb_filter {
        self as *const bindings::ndb_filter
    }
}

impl Filter {
    pub fn new() -> Filter {
        let null = std::ptr::null_mut();
        let mut filter_data = bindings::ndb_filter {
            elem_buf: bindings::cursor {
                start: null,
                p: null,
                end: null,
            },
            data_buf: bindings::cursor {
                start: null,
                p: null,
                end: null,
            },
            num_elements: 0,
            current: std::ptr::null_mut(),
            elements: [
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            ],
        };

        unsafe {
            bindings::ndb_filter_init(&mut filter_data as *mut bindings::ndb_filter);
        };

        Self { data: filter_data }
    }

    pub fn as_ptr(&self) -> *const bindings::ndb_filter {
        return self.data.as_ptr();
    }

    pub fn as_mut_ptr(&self) -> *mut bindings::ndb_filter {
        return self.data.as_ptr() as *mut bindings::ndb_filter;
    }

    fn add_int_element(&self, i: u64) {
        unsafe { bindings::ndb_filter_add_int_element(self.as_mut_ptr(), i) };
    }

    fn add_str_element(&self, s: &str) {
        let c_str = CString::new(s).expect("string to cstring conversion failed");
        unsafe {
            bindings::ndb_filter_add_str_element(self.as_mut_ptr(), c_str.as_ptr());
        };
    }

    fn add_id_element(&self, id: &[u8; 32]) {
        let ptr: *const ::std::os::raw::c_uchar = id.as_ptr() as *const ::std::os::raw::c_uchar;
        unsafe {
            bindings::ndb_filter_add_id_element(self.as_mut_ptr(), ptr);
        };
    }

    fn start_kind_field(&self) {
        unsafe {
            bindings::ndb_filter_start_field(
                self.as_mut_ptr(),
                bindings::ndb_filter_fieldtype_NDB_FILTER_KINDS,
            )
        };
    }

    fn end_field(&self) {
        unsafe { bindings::ndb_filter_end_field(self.as_mut_ptr()) }
    }

    pub fn authors(self, authors: Vec<[u8; 32]>) -> Filter {
        unsafe {
            bindings::ndb_filter_start_field(
                self.as_mut_ptr(),
                bindings::ndb_filter_fieldtype_NDB_FILTER_AUTHORS,
            );
        };

        for author in authors {
            self.add_id_element(&author);
        }

        self.end_field();

        self
    }

    fn start_tag_field(&self, tag: char) {
        unsafe {
            bindings::ndb_filter_start_tag_field(self.as_mut_ptr(), tag as u8 as c_char);
        }
    }

    pub fn kinds(self, kinds: Vec<u64>) -> Filter {
        self.start_kind_field();
        for kind in kinds {
            self.add_int_element(kind);
        }
        self.end_field();
        self
    }

    pub fn hashtags(self, tags: Vec<String>) -> Filter {
        self.start_tag_field('t');
        for tag in tags {
            self.add_str_element(&tag);
        }
        self.end_field();
        self
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe { bindings::ndb_filter_destroy(self.as_mut_ptr()) };
    }
}