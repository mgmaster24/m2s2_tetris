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
pub struct InnerRect2i < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerRect2i < 'a > {
    pub fn from_outer(outer: &Rect2i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get_center(&self,) -> Vector2i {
        type CallSig = (Vector2i,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(187usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "get_center", self.sys_ptr, args)
        }
    }
    pub fn get_area(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(188usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "get_area", self.sys_ptr, args)
        }
    }
    pub fn has_area(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(189usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "has_area", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector2i,) -> bool {
        type CallSig = (bool, Vector2i);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(190usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "has_point", self.sys_ptr, args)
        }
    }
    pub fn intersects(&self, b: Rect2i,) -> bool {
        type CallSig = (bool, Rect2i);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(191usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "intersects", self.sys_ptr, args)
        }
    }
    pub fn encloses(&self, b: Rect2i,) -> bool {
        type CallSig = (bool, Rect2i);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(192usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "encloses", self.sys_ptr, args)
        }
    }
    pub fn intersection(&self, b: Rect2i,) -> Rect2i {
        type CallSig = (Rect2i, Rect2i);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(193usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "intersection", self.sys_ptr, args)
        }
    }
    pub fn merge(&self, b: Rect2i,) -> Rect2i {
        type CallSig = (Rect2i, Rect2i);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(194usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "merge", self.sys_ptr, args)
        }
    }
    pub fn expand(&self, to: Vector2i,) -> Rect2i {
        type CallSig = (Rect2i, Vector2i);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(195usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "expand", self.sys_ptr, args)
        }
    }
    pub fn grow(&self, amount: i64,) -> Rect2i {
        type CallSig = (Rect2i, i64);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(196usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow", self.sys_ptr, args)
        }
    }
    pub fn grow_side(&self, side: i64, amount: i64,) -> Rect2i {
        type CallSig = (Rect2i, i64, i64);
        let args = (side, amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(197usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow_side", self.sys_ptr, args)
        }
    }
    pub fn grow_individual(&self, left: i64, top: i64, right: i64, bottom: i64,) -> Rect2i {
        type CallSig = (Rect2i, i64, i64, i64, i64);
        let args = (left, top, right, bottom,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(198usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow_individual", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Rect2i {
        type CallSig = (Rect2i,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(199usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Rect2i", "abs", self.sys_ptr, args)
        }
    }
}
impl Rect2i {
    
}