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
pub struct InnerPlane < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPlane < 'a > {
    pub fn from_outer(outer: &Plane) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn normalized(&self,) -> Plane {
        type CallSig = (Plane,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(333usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "normalized", self.sys_ptr, args)
        }
    }
    pub fn get_center(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(334usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "get_center", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to_plane: Plane,) -> bool {
        type CallSig = (bool, Plane);
        let args = (to_plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(335usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(336usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn is_point_over(&self, point: Vector3,) -> bool {
        type CallSig = (bool, Vector3);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(337usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "is_point_over", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, point: Vector3,) -> f64 {
        type CallSig = (f64, Vector3);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(338usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "distance_to", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector3, tolerance: f64,) -> bool {
        type CallSig = (bool, Vector3, f64);
        let args = (point, tolerance,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(339usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "has_point", self.sys_ptr, args)
        }
    }
    pub fn project(&self, point: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(340usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "project", self.sys_ptr, args)
        }
    }
    pub fn intersect_3(&self, b: Plane, c: Plane,) -> Variant {
        type CallSig = (Variant, Plane, Plane);
        let args = (b, c,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(341usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "intersect_3", self.sys_ptr, args)
        }
    }
    pub fn intersects_ray(&self, from: Vector3, dir: Vector3,) -> Variant {
        type CallSig = (Variant, Vector3, Vector3);
        let args = (from, dir,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(342usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "intersects_ray", self.sys_ptr, args)
        }
    }
    pub fn intersects_segment(&self, from: Vector3, to: Vector3,) -> Variant {
        type CallSig = (Variant, Vector3, Vector3);
        let args = (from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(343usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Plane", "intersects_segment", self.sys_ptr, args)
        }
    }
}
impl Plane {
    
}