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
pub struct InnerCallable < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerCallable < 'a > {
    pub fn from_outer(outer: &Callable) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn create(variant: &Variant, method: impl AsArg < StringName >,) -> Callable {
        type CallSig < 'a0, 'a1, > = (Callable, RefArg < 'a0, Variant >, CowArg < 'a1, StringName >);
        let args = (RefArg::new(variant), method.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(585usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "create", std::ptr::null_mut(), args)
        }
    }
    pub fn callv(&self, arguments: &VariantArray,) -> Variant {
        type CallSig < 'a0, > = (Variant, RefArg < 'a0, VariantArray >);
        let args = (RefArg::new(arguments),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(586usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "callv", self.sys_ptr, args)
        }
    }
    pub fn is_null(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(587usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "is_null", self.sys_ptr, args)
        }
    }
    pub fn is_custom(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(588usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "is_custom", self.sys_ptr, args)
        }
    }
    pub fn is_standard(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(589usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "is_standard", self.sys_ptr, args)
        }
    }
    pub fn is_valid(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(590usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "is_valid", self.sys_ptr, args)
        }
    }
    pub fn get_object(&self,) -> Option < Gd < crate::classes::Object > > {
        type CallSig = (Option < Gd < crate::classes::Object > >,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(591usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "get_object", self.sys_ptr, args)
        }
    }
    pub fn get_object_id(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(592usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "get_object_id", self.sys_ptr, args)
        }
    }
    pub fn get_method(&self,) -> StringName {
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(593usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "get_method", self.sys_ptr, args)
        }
    }
    pub fn get_argument_count(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(594usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "get_argument_count", self.sys_ptr, args)
        }
    }
    pub fn get_bound_arguments_count(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(595usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "get_bound_arguments_count", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(597usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "hash", self.sys_ptr, args)
        }
    }
    pub fn bindv(&mut self, arguments: &VariantArray,) -> Callable {
        type CallSig < 'a0, > = (Callable, RefArg < 'a0, VariantArray >);
        let args = (RefArg::new(arguments),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(598usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "bindv", self.sys_ptr, args)
        }
    }
    pub fn unbind(&self, argcount: i64,) -> Callable {
        type CallSig = (Callable, i64);
        let args = (argcount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(599usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "unbind", self.sys_ptr, args)
        }
    }
}
impl Callable {
    pub fn get_bound_arguments(&self,) -> VariantArray {
        type CallSig = (VariantArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(596usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Callable", "get_bound_arguments", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn call(&self, varargs: &[Variant]) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(600usize);
            < CallSig as VarcallSignatureTuple > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "call", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    pub fn call_deferred(&self, varargs: &[Variant]) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(601usize);
            < CallSig as VarcallSignatureTuple > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "call_deferred", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    pub fn rpc(&self, varargs: &[Variant]) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(602usize);
            < CallSig as VarcallSignatureTuple > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "rpc", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    pub fn rpc_id(&self, peer_id: i64, varargs: &[Variant]) {
        type CallSig = ((), i64);
        let args = (peer_id,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(603usize);
            < CallSig as VarcallSignatureTuple > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "rpc_id", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    pub fn bind(&self, varargs: &[Variant]) -> Callable {
        type CallSig = (Callable,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(604usize);
            < CallSig as VarcallSignatureTuple > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "bind", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
}