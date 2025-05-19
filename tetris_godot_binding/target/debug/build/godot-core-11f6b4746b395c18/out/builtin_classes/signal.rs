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
pub struct InnerSignal < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerSignal < 'a > {
    pub fn from_outer(outer: &Signal) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn is_null(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(605usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Signal", "is_null", self.sys_ptr, args)
        }
    }
    pub fn get_object(&self,) -> Option < Gd < crate::classes::Object > > {
        type CallSig = (Option < Gd < crate::classes::Object > >,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(606usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Signal", "get_object", self.sys_ptr, args)
        }
    }
    pub fn get_object_id(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(607usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Signal", "get_object_id", self.sys_ptr, args)
        }
    }
    pub fn get_name(&self,) -> StringName {
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(608usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Signal", "get_name", self.sys_ptr, args)
        }
    }
    pub fn connect(&mut self, callable: &Callable, flags: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, RefArg < 'a0, Callable >, i64);
        let args = (RefArg::new(callable), flags,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(609usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Signal", "connect", self.sys_ptr, args)
        }
    }
    pub fn disconnect(&mut self, callable: &Callable,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >);
        let args = (RefArg::new(callable),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(610usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Signal", "disconnect", self.sys_ptr, args)
        }
    }
    pub fn is_connected(&self, callable: &Callable,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, Callable >);
        let args = (RefArg::new(callable),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(611usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Signal", "is_connected", self.sys_ptr, args)
        }
    }
    pub fn get_connections(&self,) -> VariantArray {
        type CallSig = (VariantArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(612usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Signal", "get_connections", self.sys_ptr, args)
        }
    }
    pub fn emit(&self, varargs: &[Variant]) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(613usize);
            < CallSig as VarcallSignatureTuple > ::out_builtin_ptrcall_varargs(method_bind, "Signal", "emit", self.sys_ptr, args, varargs)
        }
    }
}
impl Signal {
    
}