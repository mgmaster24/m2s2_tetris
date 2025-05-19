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
pub struct InnerRect2 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerRect2 < 'a > {
    pub fn from_outer(outer: &Rect2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get_center(&self,) -> Vector2 {
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(172usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "get_center", self.sys_ptr, args)
        }
    }
    pub fn get_area(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(173usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "get_area", self.sys_ptr, args)
        }
    }
    pub fn has_area(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(174usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "has_area", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector2,) -> bool {
        type CallSig = (bool, Vector2);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(175usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "has_point", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, rect: Rect2,) -> bool {
        type CallSig = (bool, Rect2);
        let args = (rect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(176usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(177usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn intersects(&self, b: Rect2, include_borders: bool,) -> bool {
        type CallSig = (bool, Rect2, bool);
        let args = (b, include_borders,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(178usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "intersects", self.sys_ptr, args)
        }
    }
    pub fn encloses(&self, b: Rect2,) -> bool {
        type CallSig = (bool, Rect2);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(179usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "encloses", self.sys_ptr, args)
        }
    }
    pub fn intersection(&self, b: Rect2,) -> Rect2 {
        type CallSig = (Rect2, Rect2);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(180usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "intersection", self.sys_ptr, args)
        }
    }
    pub fn merge(&self, b: Rect2,) -> Rect2 {
        type CallSig = (Rect2, Rect2);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(181usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "merge", self.sys_ptr, args)
        }
    }
    pub fn expand(&self, to: Vector2,) -> Rect2 {
        type CallSig = (Rect2, Vector2);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(182usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "expand", self.sys_ptr, args)
        }
    }
    pub fn grow(&self, amount: f64,) -> Rect2 {
        type CallSig = (Rect2, f64);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(183usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "grow", self.sys_ptr, args)
        }
    }
    pub fn grow_side(&self, side: i64, amount: f64,) -> Rect2 {
        type CallSig = (Rect2, i64, f64);
        let args = (side, amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(184usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "grow_side", self.sys_ptr, args)
        }
    }
    pub fn grow_individual(&self, left: f64, top: f64, right: f64, bottom: f64,) -> Rect2 {
        type CallSig = (Rect2, f64, f64, f64, f64);
        let args = (left, top, right, bottom,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(185usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "grow_individual", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Rect2 {
        type CallSig = (Rect2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(186usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2", "abs", self.sys_ptr, args)
        }
    }
}
impl Rect2 {
    
}