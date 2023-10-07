use std::mem::{size_of, align_of, MaybeUninit};

pub struct PackedDyn {
    data: usize,
    vtable: *mut (),
}

impl PackedDyn {

    pub fn new<T, F: FnOnce(T) -> *mut T>(val: T, alloc: F, vtable: *mut ()) -> Self {
        if size_of::<T>() <= size_of::<usize>() && align_of::<T>() <= align_of::<usize>() {
            let mut finish = MaybeUninit::zeroed();

            unsafe { finish.as_mut_ptr().cast::<T>().write(val); }

            Self {
                data: unsafe { finish.assume_init() },
                vtable,
            }
        } else {
            Self {
                data: alloc(val) as usize,
                vtable,
            }
        }
    }

    #[inline]
    pub fn new_external<T, F: FnOnce(T) -> *mut T, VT: FnOnce(&T) -> *mut ()>(val: T, alloc: F, vtable: VT) -> Self {
        let vtable = vtable(&val);
        Self::new(val, alloc, vtable)
    }

    #[inline]
    pub fn new_predefined<T: GetVtable, F: FnOnce(T) -> *mut T>(val: T, alloc: F) -> Self {
        let vtable = val.get_vtable();
        Self::new(val, alloc, vtable)
    }

}

pub trait GetVtable {

    fn get_vtable(&self) -> *mut ();

}
