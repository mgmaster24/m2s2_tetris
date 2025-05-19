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
pub struct InnerNodePath < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerNodePath < 'a > {
    pub fn from_outer(outer: &NodePath) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get_name_count(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(573usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "get_name_count", self.sys_ptr, args)
        }
    }
    pub fn get_name(&self, idx: i64,) -> StringName {
        type CallSig = (StringName, i64);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(574usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "get_name", self.sys_ptr, args)
        }
    }
    pub fn get_subname_count(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(575usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "get_subname_count", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(576usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "hash", self.sys_ptr, args)
        }
    }
    pub fn get_subname(&self, idx: i64,) -> StringName {
        type CallSig = (StringName, i64);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(577usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "get_subname", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> NodePath {
        type CallSig = (NodePath, i64, i64);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(580usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "slice", self.sys_ptr, args)
        }
    }
}
impl NodePath {
    pub fn is_absolute(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(572usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "is_absolute", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_concatenated_names(&self,) -> StringName {
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(578usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "get_concatenated_names", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_concatenated_subnames(&self,) -> StringName {
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(579usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "get_concatenated_subnames", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_as_property_path(&self,) -> NodePath {
        type CallSig = (NodePath,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(581usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "get_as_property_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(582usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "NodePath", "is_empty", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
}