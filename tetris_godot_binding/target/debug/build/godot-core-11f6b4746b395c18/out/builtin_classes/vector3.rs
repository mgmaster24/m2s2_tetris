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
pub struct InnerVector3 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector3 < 'a > {
    pub fn from_outer(outer: &Vector3) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(200usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(201usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn angle_to(&self, to: Vector3,) -> f64 {
        type CallSig = (f64, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(202usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "angle_to", self.sys_ptr, args)
        }
    }
    pub fn signed_angle_to(&self, to: Vector3, axis: Vector3,) -> f64 {
        type CallSig = (f64, Vector3, Vector3);
        let args = (to, axis,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(203usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "signed_angle_to", self.sys_ptr, args)
        }
    }
    pub fn direction_to(&self, to: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(204usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "direction_to", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, to: Vector3,) -> f64 {
        type CallSig = (f64, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(205usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "distance_to", self.sys_ptr, args)
        }
    }
    pub fn distance_squared_to(&self, to: Vector3,) -> f64 {
        type CallSig = (f64, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(206usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "distance_squared_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(207usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(208usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "length_squared", self.sys_ptr, args)
        }
    }
    pub fn limit_length(&self, length: f64,) -> Vector3 {
        type CallSig = (Vector3, f64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(209usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "limit_length", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(210usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(211usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Vector3,) -> bool {
        type CallSig = (bool, Vector3);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(212usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_zero_approx(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(213usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "is_zero_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(214usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(215usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "inverse", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector3, max: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3, Vector3);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(216usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "clamp", self.sys_ptr, args)
        }
    }
    pub fn clampf(&self, min: f64, max: f64,) -> Vector3 {
        type CallSig = (Vector3, f64, f64);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(217usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "clampf", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(218usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "snapped", self.sys_ptr, args)
        }
    }
    pub fn snappedf(&self, step: f64,) -> Vector3 {
        type CallSig = (Vector3, f64);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(219usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "snappedf", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Vector3 {
        type CallSig = (Vector3, Vector3, f64);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(220usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "rotated", self.sys_ptr, args)
        }
    }
    pub fn lerp(&self, to: Vector3, weight: f64,) -> Vector3 {
        type CallSig = (Vector3, Vector3, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(221usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "lerp", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Vector3, weight: f64,) -> Vector3 {
        type CallSig = (Vector3, Vector3, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(222usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "slerp", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate(&self, b: Vector3, pre_a: Vector3, post_b: Vector3, weight: f64,) -> Vector3 {
        type CallSig = (Vector3, Vector3, Vector3, Vector3, f64);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(223usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate_in_time(&self, b: Vector3, pre_a: Vector3, post_b: Vector3, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector3 {
        type CallSig = (Vector3, Vector3, Vector3, Vector3, f64, f64, f64, f64);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(224usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn bezier_interpolate(&self, control_1: Vector3, control_2: Vector3, end: Vector3, t: f64,) -> Vector3 {
        type CallSig = (Vector3, Vector3, Vector3, Vector3, f64);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(225usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "bezier_interpolate", self.sys_ptr, args)
        }
    }
    pub fn bezier_derivative(&self, control_1: Vector3, control_2: Vector3, end: Vector3, t: f64,) -> Vector3 {
        type CallSig = (Vector3, Vector3, Vector3, Vector3, f64);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(226usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "bezier_derivative", self.sys_ptr, args)
        }
    }
    pub fn move_toward(&self, to: Vector3, delta: f64,) -> Vector3 {
        type CallSig = (Vector3, Vector3, f64);
        let args = (to, delta,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(227usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "move_toward", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Vector3,) -> f64 {
        type CallSig = (f64, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(228usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "dot", self.sys_ptr, args)
        }
    }
    pub fn cross(&self, with: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(229usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "cross", self.sys_ptr, args)
        }
    }
    pub fn outer(&self, with: Vector3,) -> Basis {
        type CallSig = (Basis, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(230usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "outer", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(231usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "abs", self.sys_ptr, args)
        }
    }
    pub fn floor(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(232usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "floor", self.sys_ptr, args)
        }
    }
    pub fn ceil(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(233usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "ceil", self.sys_ptr, args)
        }
    }
    pub fn round(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(234usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "round", self.sys_ptr, args)
        }
    }
    pub fn posmod(&self, mod_: f64,) -> Vector3 {
        type CallSig = (Vector3, f64);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(235usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "posmod", self.sys_ptr, args)
        }
    }
    pub fn posmodv(&self, modv: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(236usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "posmodv", self.sys_ptr, args)
        }
    }
    pub fn project(&self, b: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(237usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "project", self.sys_ptr, args)
        }
    }
    pub fn slide(&self, n: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(238usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "slide", self.sys_ptr, args)
        }
    }
    pub fn bounce(&self, n: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(239usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "bounce", self.sys_ptr, args)
        }
    }
    pub fn reflect(&self, n: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(240usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "reflect", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(241usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "sign", self.sys_ptr, args)
        }
    }
    pub fn octahedron_encode(&self,) -> Vector2 {
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(242usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "octahedron_encode", self.sys_ptr, args)
        }
    }
    pub fn min(&self, with: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(243usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "min", self.sys_ptr, args)
        }
    }
    pub fn minf(&self, with: f64,) -> Vector3 {
        type CallSig = (Vector3, f64);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(244usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "minf", self.sys_ptr, args)
        }
    }
    pub fn max(&self, with: Vector3,) -> Vector3 {
        type CallSig = (Vector3, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(245usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "max", self.sys_ptr, args)
        }
    }
    pub fn maxf(&self, with: f64,) -> Vector3 {
        type CallSig = (Vector3, f64);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(246usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "maxf", self.sys_ptr, args)
        }
    }
    pub fn octahedron_decode(uv: Vector2,) -> Vector3 {
        type CallSig = (Vector3, Vector2);
        let args = (uv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(247usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Vector3", "octahedron_decode", std::ptr::null_mut(), args)
        }
    }
}
impl Vector3 {
    
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