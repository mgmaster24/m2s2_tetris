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
pub struct InnerAabb < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerAabb < 'a > {
    pub fn from_outer(outer: &Aabb) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn abs(&self,) -> Aabb {
        type CallSig = (Aabb,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(363usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "abs", self.sys_ptr, args)
        }
    }
    pub fn get_center(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(364usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_center", self.sys_ptr, args)
        }
    }
    pub fn get_volume(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(365usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_volume", self.sys_ptr, args)
        }
    }
    pub fn has_volume(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(366usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "has_volume", self.sys_ptr, args)
        }
    }
    pub fn has_surface(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(367usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "has_surface", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector3,) -> bool {
        type CallSig = (bool, Vector3);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(368usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "has_point", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, aabb: Aabb,) -> bool {
        type CallSig = (bool, Aabb);
        let args = (aabb,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(369usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(370usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn intersects(&self, with: Aabb,) -> bool {
        type CallSig = (bool, Aabb);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(371usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "intersects", self.sys_ptr, args)
        }
    }
    pub fn encloses(&self, with: Aabb,) -> bool {
        type CallSig = (bool, Aabb);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(372usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "encloses", self.sys_ptr, args)
        }
    }
    pub fn intersects_plane(&self, plane: Plane,) -> bool {
        type CallSig = (bool, Plane);
        let args = (plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(373usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "intersects_plane", self.sys_ptr, args)
        }
    }
    pub fn intersection(&self, with: Aabb,) -> Aabb {
        type CallSig = (Aabb, Aabb);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(374usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "intersection", self.sys_ptr, args)
        }
    }
    pub fn merge(&self, with: Aabb,) -> Aabb {
        type CallSig = (Aabb, Aabb);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(375usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "merge", self.sys_ptr, args)
        }
    }
    pub fn expand(&self, to_point: Vector3,) -> Aabb {
        type CallSig = (Aabb, Vector3);
        let args = (to_point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(376usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "expand", self.sys_ptr, args)
        }
    }
    pub fn grow(&self, by: f64,) -> Aabb {
        type CallSig = (Aabb, f64);
        let args = (by,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(377usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "grow", self.sys_ptr, args)
        }
    }
    pub fn get_support(&self, dir: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (dir,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(378usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_support", self.sys_ptr, args)
        }
    }
    pub fn get_longest_axis(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(379usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_longest_axis", self.sys_ptr, args)
        }
    }
    pub fn get_longest_axis_index(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(380usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_longest_axis_index", self.sys_ptr, args)
        }
    }
    pub fn get_longest_axis_size(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(381usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_longest_axis_size", self.sys_ptr, args)
        }
    }
    pub fn get_shortest_axis(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(382usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_shortest_axis", self.sys_ptr, args)
        }
    }
    pub fn get_shortest_axis_index(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(383usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_shortest_axis_index", self.sys_ptr, args)
        }
    }
    pub fn get_shortest_axis_size(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(384usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_shortest_axis_size", self.sys_ptr, args)
        }
    }
    pub fn get_endpoint(&self, idx: i64,) -> Vector3 {
        type CallSig = (Vector3, i64);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(385usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "get_endpoint", self.sys_ptr, args)
        }
    }
    pub fn intersects_segment(&self, from: Vector3, to: Vector3,) -> Variant {
        type CallSig = (Variant, Vector3, Vector3);
        let args = (from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(386usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "intersects_segment", self.sys_ptr, args)
        }
    }
    pub fn intersects_ray(&self, from: Vector3, dir: Vector3,) -> Variant {
        type CallSig = (Variant, Vector3, Vector3);
        let args = (from, dir,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(387usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Aabb", "intersects_ray", self.sys_ptr, args)
        }
    }
}
impl Aabb {
    
}