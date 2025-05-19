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
pub struct InnerPackedFloat32Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedFloat32Array < 'a > {
    pub fn from_outer(outer: &PackedFloat32Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(779usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(780usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: f64,) {
        type CallSig = ((), i64, f64);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(781usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "set", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: f64,) -> bool {
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(782usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: f64,) -> bool {
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(783usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedFloat32Array,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, PackedFloat32Array >);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(784usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallSig = ((), i64);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(785usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: f64,) -> i64 {
        type CallSig = (i64, i64, f64);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(786usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: f64,) {
        type CallSig = ((), f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(787usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(788usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(789usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: f64,) -> bool {
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(790usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(791usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedFloat32Array {
        type CallSig = (PackedFloat32Array, i64, i64);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(792usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(793usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(794usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: f64, before: bool,) -> i64 {
        type CallSig = (i64, f64, bool);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(795usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedFloat32Array {
        type CallSig = (PackedFloat32Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(796usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: f64, from: i64,) -> i64 {
        type CallSig = (i64, f64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(797usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: f64, from: i64,) -> i64 {
        type CallSig = (i64, f64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(798usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: f64,) -> i64 {
        type CallSig = (i64, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(799usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat32Array", "count", self.sys_ptr, args)
        }
    }
}
impl PackedFloat32Array {
    
}