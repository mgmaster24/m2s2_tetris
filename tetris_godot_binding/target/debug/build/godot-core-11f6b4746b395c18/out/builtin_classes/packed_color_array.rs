use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, AsObjectArg, ClassName, CowArg, ObjectArg, ObjectCow, PtrcallSignatureTuple, RefArg, VarcallSignatureTuple
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
#[repr(transparent)]
pub struct InnerPackedColorArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedColorArray < 'a > {
    pub fn from_outer(outer: &PackedColorArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(884usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(885usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: Color,) {
        type CallSig = ((), i64, Color);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(886usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "set", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: Color,) -> bool {
        type CallSig = (bool, Color);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(887usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: Color,) -> bool {
        type CallSig = (bool, Color);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(888usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedColorArray,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, PackedColorArray >);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(889usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallSig = ((), i64);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(890usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: Color,) -> i64 {
        type CallSig = (i64, i64, Color);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(891usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: Color,) {
        type CallSig = ((), Color);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(892usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(893usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(894usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: Color,) -> bool {
        type CallSig = (bool, Color);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(895usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(896usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedColorArray {
        type CallSig = (PackedColorArray, i64, i64);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(897usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(898usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(899usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: Color, before: bool,) -> i64 {
        type CallSig = (i64, Color, bool);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(900usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedColorArray {
        type CallSig = (PackedColorArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(901usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: Color, from: i64,) -> i64 {
        type CallSig = (i64, Color, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(902usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: Color, from: i64,) -> i64 {
        type CallSig = (i64, Color, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(903usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: Color,) -> i64 {
        type CallSig = (i64, Color);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(904usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "count", self.sys_ptr, args)
        }
    }
}
impl PackedColorArray {
    
}