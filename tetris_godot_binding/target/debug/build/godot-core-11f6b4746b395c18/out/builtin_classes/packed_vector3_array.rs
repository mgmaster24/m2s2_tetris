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
pub struct InnerPackedVector3Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedVector3Array < 'a > {
    pub fn from_outer(outer: &PackedVector3Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(863usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(864usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: Vector3,) {
        type CallSig = ((), i64, Vector3);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(865usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "set", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: Vector3,) -> bool {
        type CallSig = (bool, Vector3);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(866usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: Vector3,) -> bool {
        type CallSig = (bool, Vector3);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(867usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedVector3Array,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, PackedVector3Array >);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(868usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallSig = ((), i64);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(869usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: Vector3,) -> i64 {
        type CallSig = (i64, i64, Vector3);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(870usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: Vector3,) {
        type CallSig = ((), Vector3);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(871usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(872usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(873usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: Vector3,) -> bool {
        type CallSig = (bool, Vector3);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(874usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(875usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedVector3Array {
        type CallSig = (PackedVector3Array, i64, i64);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(876usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(877usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(878usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: Vector3, before: bool,) -> i64 {
        type CallSig = (i64, Vector3, bool);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(879usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedVector3Array {
        type CallSig = (PackedVector3Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(880usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: Vector3, from: i64,) -> i64 {
        type CallSig = (i64, Vector3, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(881usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: Vector3, from: i64,) -> i64 {
        type CallSig = (i64, Vector3, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(882usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: Vector3,) -> i64 {
        type CallSig = (i64, Vector3);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(883usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "count", self.sys_ptr, args)
        }
    }
}
impl PackedVector3Array {
    
}