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
pub struct InnerPackedFloat64Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedFloat64Array < 'a > {
    pub fn from_outer(outer: &PackedFloat64Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(800usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(801usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: f64,) {
        type CallSig = ((), i64, f64);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(802usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "set", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: f64,) -> bool {
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(803usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: f64,) -> bool {
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(804usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedFloat64Array,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, PackedFloat64Array >);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(805usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallSig = ((), i64);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(806usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: f64,) -> i64 {
        type CallSig = (i64, i64, f64);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(807usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: f64,) {
        type CallSig = ((), f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(808usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(809usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(810usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: f64,) -> bool {
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(811usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(812usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedFloat64Array {
        type CallSig = (PackedFloat64Array, i64, i64);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(813usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(814usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(815usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: f64, before: bool,) -> i64 {
        type CallSig = (i64, f64, bool);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(816usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedFloat64Array {
        type CallSig = (PackedFloat64Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(817usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: f64, from: i64,) -> i64 {
        type CallSig = (i64, f64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(818usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: f64, from: i64,) -> i64 {
        type CallSig = (i64, f64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(819usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: f64,) -> i64 {
        type CallSig = (i64, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(820usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedFloat64Array", "count", self.sys_ptr, args)
        }
    }
}
impl PackedFloat64Array {
    
}