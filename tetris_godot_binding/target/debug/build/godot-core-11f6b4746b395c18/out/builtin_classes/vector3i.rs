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
pub struct InnerVector3i < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector3i < 'a > {
    pub fn from_outer(outer: &Vector3i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(248usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(249usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, to: Vector3i,) -> f64 {
        type CallSig = (f64, Vector3i);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(250usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "distance_to", self.sys_ptr, args)
        }
    }
    pub fn distance_squared_to(&self, to: Vector3i,) -> i64 {
        type CallSig = (i64, Vector3i);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(251usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "distance_squared_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(252usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(253usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "length_squared", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector3i {
        type CallSig = (Vector3i,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(254usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "sign", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector3i {
        type CallSig = (Vector3i,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(255usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "abs", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector3i, max: Vector3i,) -> Vector3i {
        type CallSig = (Vector3i, Vector3i, Vector3i);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(256usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "clamp", self.sys_ptr, args)
        }
    }
    pub fn clampi(&self, min: i64, max: i64,) -> Vector3i {
        type CallSig = (Vector3i, i64, i64);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(257usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "clampi", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector3i,) -> Vector3i {
        type CallSig = (Vector3i, Vector3i);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(258usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "snapped", self.sys_ptr, args)
        }
    }
    pub fn snappedi(&self, step: i64,) -> Vector3i {
        type CallSig = (Vector3i, i64);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(259usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "snappedi", self.sys_ptr, args)
        }
    }
    pub fn min(&self, with: Vector3i,) -> Vector3i {
        type CallSig = (Vector3i, Vector3i);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(260usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "min", self.sys_ptr, args)
        }
    }
    pub fn mini(&self, with: i64,) -> Vector3i {
        type CallSig = (Vector3i, i64);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(261usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "mini", self.sys_ptr, args)
        }
    }
    pub fn max(&self, with: Vector3i,) -> Vector3i {
        type CallSig = (Vector3i, Vector3i);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(262usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "max", self.sys_ptr, args)
        }
    }
    pub fn maxi(&self, with: i64,) -> Vector3i {
        type CallSig = (Vector3i, i64);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(263usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3i", "maxi", self.sys_ptr, args)
        }
    }
}
impl Vector3i {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Axis {
    ord: i32
}
impl Axis {
    #[doc(alias = "AXIS_X")]
    #[doc = "Godot enumerator name: `AXIS_X`"]
    pub const X: Axis = Axis {
        ord: 0i32
    };
    #[doc(alias = "AXIS_Y")]
    #[doc = "Godot enumerator name: `AXIS_Y`"]
    pub const Y: Axis = Axis {
        ord: 1i32
    };
    #[doc(alias = "AXIS_Z")]
    #[doc = "Godot enumerator name: `AXIS_Z`"]
    pub const Z: Axis = Axis {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Axis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Axis {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::X => "X", Self::Y => "Y", Self::Z => "Z", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::X => "AXIS_X", Self::Y => "AXIS_Y", Self::Z => "AXIS_Z", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Axis {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Axis {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Axis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}