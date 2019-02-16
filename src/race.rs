use bwapi_sys as sys;

#[repr(i32)]
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum Race {
    Zerg = 0,
    Terran,
    Protoss,
    Other,
    Unused,
    Select,
    Random,
    None,
    Unknown,
    MAX
}

impl From<sys::Race> for Race {
    fn from(input: sys::Race) -> Race {
        unsafe {
            let id = input.id;
            assert!(id >= 0);
            assert!(id < Race::MAX as i32);
            ::std::mem::transmute(id)
        }
    }
}

impl From<Race> for sys::Race {
    fn from(input: Race) -> sys::Race {
        use std::os::raw::c_int;
        sys::Race { id: input as c_int }
    }
}
