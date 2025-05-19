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
pub struct InnerString < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerString < 'a > {
    pub fn from_outer(outer: &GString) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn casecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(0usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "casecmp_to", self.sys_ptr, args)
        }
    }
    pub fn nocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(1usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "nocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalcasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(2usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "naturalcasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalnocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(3usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "naturalnocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn filecasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(4usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "filecasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn filenocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(5usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "filenocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(6usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "length", self.sys_ptr, args)
        }
    }
    pub fn substr(&self, from: i64, len: i64,) -> GString {
        type CallSig = (GString, i64, i64);
        let args = (from, len,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(7usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "substr", self.sys_ptr, args)
        }
    }
    pub fn get_slice(&self, delimiter: impl AsArg < GString >, slice: i64,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >, i64);
        let args = (delimiter.into_arg(), slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(8usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "get_slice", self.sys_ptr, args)
        }
    }
    pub fn get_slicec(&self, delimiter: i64, slice: i64,) -> GString {
        type CallSig = (GString, i64, i64);
        let args = (delimiter, slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(9usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "get_slicec", self.sys_ptr, args)
        }
    }
    pub fn get_slice_count(&self, delimiter: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (delimiter.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(10usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "get_slice_count", self.sys_ptr, args)
        }
    }
    pub fn find(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(11usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "find", self.sys_ptr, args)
        }
    }
    pub fn findn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(12usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "findn", self.sys_ptr, args)
        }
    }
    pub fn count(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64, i64);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(13usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "count", self.sys_ptr, args)
        }
    }
    pub fn countn(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64, i64);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(14usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "countn", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(15usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "rfind", self.sys_ptr, args)
        }
    }
    pub fn rfindn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(16usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "rfindn", self.sys_ptr, args)
        }
    }
    pub fn match_(&self, expr: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(17usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "match", self.sys_ptr, args)
        }
    }
    pub fn matchn(&self, expr: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(18usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "matchn", self.sys_ptr, args)
        }
    }
    pub fn format(&self, values: &Variant, placeholder: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, 'a1, > = (GString, RefArg < 'a0, Variant >, CowArg < 'a1, GString >);
        let args = (RefArg::new(values), placeholder.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(25usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "format", self.sys_ptr, args)
        }
    }
    pub fn insert(&self, position: i64, what: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, i64, CowArg < 'a0, GString >);
        let args = (position, what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(30usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "insert", self.sys_ptr, args)
        }
    }
    pub fn erase(&self, position: i64, chars: i64,) -> GString {
        type CallSig = (GString, i64, i64);
        let args = (position, chars,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(31usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "erase", self.sys_ptr, args)
        }
    }
    pub fn split(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >, bool, i64);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(36usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "split", self.sys_ptr, args)
        }
    }
    pub fn rsplit(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >, bool, i64);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(37usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "rsplit", self.sys_ptr, args)
        }
    }
    pub fn unicode_at(&self, at: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (at,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(51usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "unicode_at", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(54usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "hash", self.sys_ptr, args)
        }
    }
    pub fn lpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, i64, CowArg < 'a0, GString >);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(89usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "lpad", self.sys_ptr, args)
        }
    }
    pub fn rpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, i64, CowArg < 'a0, GString >);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(90usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "rpad", self.sys_ptr, args)
        }
    }
    pub fn pad_decimals(&self, digits: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(91usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "pad_decimals", self.sys_ptr, args)
        }
    }
    pub fn pad_zeros(&self, digits: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(92usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "pad_zeros", self.sys_ptr, args)
        }
    }
}
impl GString {
    pub fn begins_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(19usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "begins_with", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn ends_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(20usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "ends_with", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_subsequence_of(&self, text: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(21usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_subsequence_of", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_subsequence_ofn(&self, text: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(22usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_subsequence_ofn", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn bigrams(&self,) -> PackedStringArray {
        type CallSig = (PackedStringArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(23usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "bigrams", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn similarity(&self, text: impl AsArg < GString >,) -> f64 {
        type CallSig < 'a0, > = (f64, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(24usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "similarity", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn replace(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(26usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "replace", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn replacen(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(27usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "replacen", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn repeat(&self, count: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(28usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "repeat", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn reverse(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(29usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "reverse", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn capitalize(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(32usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "capitalize", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_camel_case(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(33usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_camel_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_pascal_case(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(34usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_pascal_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_snake_case(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(35usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_snake_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn split_floats(&self, delimiter: impl AsArg < GString >, allow_empty: bool,) -> PackedFloat64Array {
        type CallSig < 'a0, > = (PackedFloat64Array, CowArg < 'a0, GString >, bool);
        let args = (delimiter.into_arg(), allow_empty,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(38usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "split_floats", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn join(&self, parts: &PackedStringArray,) -> GString {
        type CallSig < 'a0, > = (GString, RefArg < 'a0, PackedStringArray >);
        let args = (RefArg::new(parts),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(39usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "join", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_upper(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(40usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_upper", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_lower(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(41usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_lower", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn left(&self, length: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(42usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "left", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn right(&self, length: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(43usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "right", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn strip_edges(&self, left: bool, right: bool,) -> GString {
        type CallSig = (GString, bool, bool);
        let args = (left, right,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(44usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "strip_edges", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn strip_escapes(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(45usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "strip_escapes", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn lstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(46usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "lstrip", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn rstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(47usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "rstrip", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_extension(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(48usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "get_extension", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_basename(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(49usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "get_basename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn path_join(&self, file: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (file.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(50usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "path_join", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn indent(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(52usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "indent", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn dedent(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(53usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "dedent", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn md5_text(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(55usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "md5_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha1_text(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(56usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "sha1_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha256_text(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(57usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "sha256_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn md5_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(58usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "md5_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha1_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(59usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "sha1_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha256_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(60usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "sha256_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(61usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_empty", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn contains(&self, what: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(62usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "contains", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn containsn(&self, what: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(63usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "containsn", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_absolute_path(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(64usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_absolute_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_relative_path(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(65usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_relative_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn simplify_path(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(66usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "simplify_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_base_dir(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(67usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "get_base_dir", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_file(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(68usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "get_file", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn xml_escape(&self, escape_quotes: bool,) -> GString {
        type CallSig = (GString, bool);
        let args = (escape_quotes,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(69usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "xml_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn xml_unescape(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(70usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "xml_unescape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn uri_encode(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(71usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "uri_encode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn uri_decode(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(72usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "uri_decode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn c_escape(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(73usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "c_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn c_unescape(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(74usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "c_unescape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn json_escape(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(75usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "json_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn validate_node_name(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(76usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "validate_node_name", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn validate_filename(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(77usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "validate_filename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_identifier(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(78usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_valid_identifier", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_int(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(79usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_valid_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_float(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(80usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_valid_float", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_hex_number(&self, with_prefix: bool,) -> bool {
        type CallSig = (bool, bool);
        let args = (with_prefix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(81usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_valid_hex_number", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_html_color(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(82usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_valid_html_color", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_ip_address(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(83usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_valid_ip_address", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_filename(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(84usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "is_valid_filename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_int(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(85usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_float(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(86usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_float", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn hex_to_int(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(87usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "hex_to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn bin_to_int(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(88usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "bin_to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn trim_prefix(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(93usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "trim_prefix", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn trim_suffix(&self, suffix: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (suffix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(94usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "trim_suffix", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_ascii_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(95usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_ascii_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf8_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(96usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_utf8_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf16_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(97usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_utf16_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf32_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(98usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_utf32_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn hex_decode(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(99usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "hex_decode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_wchar_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(100usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "to_wchar_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn num_scientific(number: f64,) -> GString {
        type CallSig = (GString, f64);
        let args = (number,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(101usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "num_scientific", std::ptr::null_mut(), args)
        }
    }
    pub fn num(number: f64, decimals: i64,) -> GString {
        type CallSig = (GString, f64, i64);
        let args = (number, decimals,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(102usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "num", std::ptr::null_mut(), args)
        }
    }
    pub fn num_int64(number: i64, base: i64, capitalize_hex: bool,) -> GString {
        type CallSig = (GString, i64, i64, bool);
        let args = (number, base, capitalize_hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(103usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "num_int64", std::ptr::null_mut(), args)
        }
    }
    pub fn num_uint64(number: i64, base: i64, capitalize_hex: bool,) -> GString {
        type CallSig = (GString, i64, i64, bool);
        let args = (number, base, capitalize_hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(104usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "num_uint64", std::ptr::null_mut(), args)
        }
    }
    pub fn chr(char: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (char,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(105usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "chr", std::ptr::null_mut(), args)
        }
    }
    pub fn humanize_size(size: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(106usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "String", "humanize_size", std::ptr::null_mut(), args)
        }
    }
}