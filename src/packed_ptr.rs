use std::{mem::{MaybeUninit, size_of, align_of, ManuallyDrop}, ptr::NonNull};

pub union PackedPtr<T> {
    inline: ManuallyDrop<T>,
    ptr: *mut T,
}

impl<T> PackedPtr<T> {

    #[inline]
    pub fn new<F: FnOnce(T) -> NonNull<T>>(val: T, alloc: F) -> Self {
        if size_of::<T>() <= size_of::<usize>() && align_of::<T>() <= align_of::<usize>() {
            Self {
                inline: ManuallyDrop::new(val),
            }
        } else {
            Self {
                ptr: alloc(val).as_ptr(),
            }
        }
    }

    #[inline]
    pub fn as_ref_mut<'a>(&'a mut self) -> &'a mut T {
        if size_of::<T>() <= size_of::<usize>() && align_of::<T>() <= align_of::<usize>() {
            unsafe { (&mut self.inline as *mut ManuallyDrop<T>).cast::<T>().as_mut().unwrap_unchecked() }
        } else {
            unsafe { self.ptr.as_mut().unwrap_unchecked() }
        }
    }

    #[inline]
    pub fn as_ref<'a>(&'a self) -> &'a T {
        if size_of::<T>() <= size_of::<usize>() && align_of::<T>() <= align_of::<usize>() {
            unsafe { (&self.inline as *const ManuallyDrop<T>).cast::<T>().as_ref().unwrap_unchecked() }
        } else {
            unsafe { self.ptr.as_ref().unwrap_unchecked() }
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> NonNull<T> {
        if size_of::<T>() <= size_of::<usize>() && align_of::<T>() <= align_of::<usize>() {
            unsafe { NonNull::new_unchecked((&self.inline as *const ManuallyDrop<T>).cast::<T>().cast_mut().cast::<T>()) }
        } else {
            unsafe { NonNull::new_unchecked(self.ptr) }
        }
    }

    pub fn destroy<F: FnOnce(*mut T)>(mut self, dealloc: F) {
        if size_of::<T>() <= size_of::<usize>() && align_of::<T>() <= align_of::<usize>() {
            let mut val = MaybeUninit::uninit();
            val.write(ManuallyDrop::into_inner(self.inline));
            unsafe { val.as_mut_ptr().cast::<T>().drop_in_place(); }
        } else {
            dealloc(self.ptr);
        }
    }

}