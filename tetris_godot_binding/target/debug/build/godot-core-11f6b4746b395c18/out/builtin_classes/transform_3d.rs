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
pub struct InnerTransform3D < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerTransform3D < 'a > {
    pub fn from_outer(outer: &Transform3D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn inverse(&self,) -> Transform3D {
        type CallSig = (Transform3D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(407usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "inverse", self.sys_ptr, args)
        }
    }
    pub fn affine_inverse(&self,) -> Transform3D {
        type CallSig = (Transform3D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(408usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "affine_inverse", self.sys_ptr, args)
        }
    }
    pub fn orthonormalized(&self,) -> Transform3D {
        type CallSig = (Transform3D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(409usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "orthonormalized", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Transform3D {
        type CallSig = (Transform3D, Vector3, f64);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(410usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "rotated", self.sys_ptr, args)
        }
    }
    pub fn rotated_local(&self, axis: Vector3, angle: f64,) -> Transform3D {
        type CallSig = (Transform3D, Vector3, f64);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(411usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "rotated_local", self.sys_ptr, args)
        }
    }
    pub fn scaled(&self, scale: Vector3,) -> Transform3D {
        type CallSig = (Transform3D, Vector3);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(412usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "scaled", self.sys_ptr, args)
        }
    }
    pub fn scaled_local(&self, scale: Vector3,) -> Transform3D {
        type CallSig = (Transform3D, Vector3);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(413usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "scaled_local", self.sys_ptr, args)
        }
    }
    pub fn translated(&self, offset: Vector3,) -> Transform3D {
        type CallSig = (Transform3D, Vector3);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(414usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "translated", self.sys_ptr, args)
        }
    }
    pub fn translated_local(&self, offset: Vector3,) -> Transform3D {
        type CallSig = (Transform3D, Vector3);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(415usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "translated_local", self.sys_ptr, args)
        }
    }
    pub fn looking_at(&self, target: Vector3, up: Vector3, use_model_front: bool,) -> Transform3D {
        type CallSig = (Transform3D, Vector3, Vector3, bool);
        let args = (target, up, use_model_front,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(416usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "looking_at", self.sys_ptr, args)
        }
    }
    pub fn interpolate_with(&self, xform: Transform3D, weight: f64,) -> Transform3D {
        type CallSig = (Transform3D, Transform3D, f64);
        let args = (xform, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(417usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "interpolate_with", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, xform: Transform3D,) -> bool {
        type CallSig = (bool, Transform3D);
        let args = (xform,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(418usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(419usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Transform3D", "is_finite", self.sys_ptr, args)
        }
    }
}
impl Transform3D {
    
}