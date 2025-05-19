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
pub struct InnerVector4 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector4 < 'a > {
    pub fn from_outer(outer: &Vector4) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(285usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(286usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(287usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(288usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "length_squared", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector4 {
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(289usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "abs", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector4 {
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(290usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "sign", self.sys_ptr, args)
        }
    }
    pub fn floor(&self,) -> Vector4 {
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(291usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "floor", self.sys_ptr, args)
        }
    }
    pub fn ceil(&self,) -> Vector4 {
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(292usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "ceil", self.sys_ptr, args)
        }
    }
    pub fn round(&self,) -> Vector4 {
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(293usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "round", self.sys_ptr, args)
        }
    }
    pub fn lerp(&self, to: Vector4, weight: f64,) -> Vector4 {
        type CallSig = (Vector4, Vector4, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(294usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "lerp", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate(&self, b: Vector4, pre_a: Vector4, post_b: Vector4, weight: f64,) -> Vector4 {
        type CallSig = (Vector4, Vector4, Vector4, Vector4, f64);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(295usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate_in_time(&self, b: Vector4, pre_a: Vector4, post_b: Vector4, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector4 {
        type CallSig = (Vector4, Vector4, Vector4, Vector4, f64, f64, f64, f64);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(296usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn posmod(&self, mod_: f64,) -> Vector4 {
        type CallSig = (Vector4, f64);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(297usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "posmod", self.sys_ptr, args)
        }
    }
    pub fn posmodv(&self, modv: Vector4,) -> Vector4 {
        type CallSig = (Vector4, Vector4);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(298usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "posmodv", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector4,) -> Vector4 {
        type CallSig = (Vector4, Vector4);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(299usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "snapped", self.sys_ptr, args)
        }
    }
    pub fn snappedf(&self, step: f64,) -> Vector4 {
        type CallSig = (Vector4, f64);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(300usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "snappedf", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector4, max: Vector4,) -> Vector4 {
        type CallSig = (Vector4, Vector4, Vector4);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(301usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "clamp", self.sys_ptr, args)
        }
    }
    pub fn clampf(&self, min: f64, max: f64,) -> Vector4 {
        type CallSig = (Vector4, f64, f64);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(302usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "clampf", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Vector4 {
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(303usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(304usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn direction_to(&self, to: Vector4,) -> Vector4 {
        type CallSig = (Vector4, Vector4);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(305usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "direction_to", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, to: Vector4,) -> f64 {
        type CallSig = (f64, Vector4);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(306usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "distance_to", self.sys_ptr, args)
        }
    }
    pub fn distance_squared_to(&self, to: Vector4,) -> f64 {
        type CallSig = (f64, Vector4);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(307usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "distance_squared_to", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Vector4,) -> f64 {
        type CallSig = (f64, Vector4);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(308usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "dot", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Vector4 {
        type CallSig = (Vector4,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(309usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "inverse", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Vector4,) -> bool {
        type CallSig = (bool, Vector4);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(310usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_zero_approx(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(311usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "is_zero_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(312usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn min(&self, with: Vector4,) -> Vector4 {
        type CallSig = (Vector4, Vector4);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(313usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "min", self.sys_ptr, args)
        }
    }
    pub fn minf(&self, with: f64,) -> Vector4 {
        type CallSig = (Vector4, f64);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(314usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "minf", self.sys_ptr, args)
        }
    }
    pub fn max(&self, with: Vector4,) -> Vector4 {
        type CallSig = (Vector4, Vector4);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(315usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "max", self.sys_ptr, args)
        }
    }
    pub fn maxf(&self, with: f64,) -> Vector4 {
        type CallSig = (Vector4, f64);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(316usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector4", "maxf", self.sys_ptr, args)
        }
    }
}
impl Vector4 {
    
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
    #[doc(alias = "AXIS_W")]
    #[doc = "Godot enumerator name: `AXIS_W`"]
    pub const W: Axis = Axis {
        ord: 3i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::X => "X", Self::Y => "Y", Self::Z => "Z", Self::W => "W", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::X => "AXIS_X", Self::Y => "AXIS_Y", Self::Z => "AXIS_Z", Self::W => "AXIS_W", _ => self.as_str(),
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