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
pub struct InnerPackedByteArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedByteArray < 'a > {
    pub fn from_outer(outer: &PackedByteArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(678usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(679usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(680usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "set", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: i64,) -> bool {
        type CallSig = (bool, i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(681usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: i64,) -> bool {
        type CallSig = (bool, i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(682usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedByteArray,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, PackedByteArray >);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(683usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallSig = ((), i64);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(684usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: i64,) -> i64 {
        type CallSig = (i64, i64, i64);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(685usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: i64,) {
        type CallSig = ((), i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(686usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(687usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(688usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: i64,) -> bool {
        type CallSig = (bool, i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(689usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(690usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedByteArray {
        type CallSig = (PackedByteArray, i64, i64);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(691usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "slice", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(692usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: i64, before: bool,) -> i64 {
        type CallSig = (i64, i64, bool);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(693usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(694usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: i64, from: i64,) -> i64 {
        type CallSig = (i64, i64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(695usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: i64, from: i64,) -> i64 {
        type CallSig = (i64, i64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(696usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(697usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "count", self.sys_ptr, args)
        }
    }
    pub fn compress(&self, compression_mode: i64,) -> PackedByteArray {
        type CallSig = (PackedByteArray, i64);
        let args = (compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(704usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "compress", self.sys_ptr, args)
        }
    }
    pub fn decompress(&self, buffer_size: i64, compression_mode: i64,) -> PackedByteArray {
        type CallSig = (PackedByteArray, i64, i64);
        let args = (buffer_size, compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(705usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decompress", self.sys_ptr, args)
        }
    }
    pub fn decompress_dynamic(&self, max_output_size: i64, compression_mode: i64,) -> PackedByteArray {
        type CallSig = (PackedByteArray, i64, i64);
        let args = (max_output_size, compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(706usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decompress_dynamic", self.sys_ptr, args)
        }
    }
    pub fn decode_u8(&self, byte_offset: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(707usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u8", self.sys_ptr, args)
        }
    }
    pub fn decode_s8(&self, byte_offset: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(708usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s8", self.sys_ptr, args)
        }
    }
    pub fn decode_u16(&self, byte_offset: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(709usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u16", self.sys_ptr, args)
        }
    }
    pub fn decode_s16(&self, byte_offset: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(710usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s16", self.sys_ptr, args)
        }
    }
    pub fn decode_u32(&self, byte_offset: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(711usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u32", self.sys_ptr, args)
        }
    }
    pub fn decode_s32(&self, byte_offset: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(712usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s32", self.sys_ptr, args)
        }
    }
    pub fn decode_u64(&self, byte_offset: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(713usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u64", self.sys_ptr, args)
        }
    }
    pub fn decode_s64(&self, byte_offset: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(714usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s64", self.sys_ptr, args)
        }
    }
    pub fn decode_half(&self, byte_offset: i64,) -> f64 {
        type CallSig = (f64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(715usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_half", self.sys_ptr, args)
        }
    }
    pub fn decode_float(&self, byte_offset: i64,) -> f64 {
        type CallSig = (f64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(716usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_float", self.sys_ptr, args)
        }
    }
    pub fn decode_double(&self, byte_offset: i64,) -> f64 {
        type CallSig = (f64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(717usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_double", self.sys_ptr, args)
        }
    }
    pub fn has_encoded_var(&self, byte_offset: i64, allow_objects: bool,) -> bool {
        type CallSig = (bool, i64, bool);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(718usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "has_encoded_var", self.sys_ptr, args)
        }
    }
    pub fn decode_var(&self, byte_offset: i64, allow_objects: bool,) -> Variant {
        type CallSig = (Variant, i64, bool);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(719usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_var", self.sys_ptr, args)
        }
    }
    pub fn decode_var_size(&self, byte_offset: i64, allow_objects: bool,) -> i64 {
        type CallSig = (i64, i64, bool);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(720usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_var_size", self.sys_ptr, args)
        }
    }
    pub fn to_int32_array(&self,) -> PackedInt32Array {
        type CallSig = (PackedInt32Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(721usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_int32_array", self.sys_ptr, args)
        }
    }
    pub fn to_int64_array(&self,) -> PackedInt64Array {
        type CallSig = (PackedInt64Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(722usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_int64_array", self.sys_ptr, args)
        }
    }
    pub fn to_float32_array(&self,) -> PackedFloat32Array {
        type CallSig = (PackedFloat32Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(723usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_float32_array", self.sys_ptr, args)
        }
    }
    pub fn to_float64_array(&self,) -> PackedFloat64Array {
        type CallSig = (PackedFloat64Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(724usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_float64_array", self.sys_ptr, args)
        }
    }
    pub fn encode_u8(&mut self, byte_offset: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(725usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u8", self.sys_ptr, args)
        }
    }
    pub fn encode_s8(&mut self, byte_offset: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(726usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s8", self.sys_ptr, args)
        }
    }
    pub fn encode_u16(&mut self, byte_offset: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(727usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u16", self.sys_ptr, args)
        }
    }
    pub fn encode_s16(&mut self, byte_offset: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(728usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s16", self.sys_ptr, args)
        }
    }
    pub fn encode_u32(&mut self, byte_offset: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(729usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u32", self.sys_ptr, args)
        }
    }
    pub fn encode_s32(&mut self, byte_offset: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(730usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s32", self.sys_ptr, args)
        }
    }
    pub fn encode_u64(&mut self, byte_offset: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(731usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u64", self.sys_ptr, args)
        }
    }
    pub fn encode_s64(&mut self, byte_offset: i64, value: i64,) {
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(732usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s64", self.sys_ptr, args)
        }
    }
    pub fn encode_half(&mut self, byte_offset: i64, value: f64,) {
        type CallSig = ((), i64, f64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(733usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_half", self.sys_ptr, args)
        }
    }
    pub fn encode_float(&mut self, byte_offset: i64, value: f64,) {
        type CallSig = ((), i64, f64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(734usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_float", self.sys_ptr, args)
        }
    }
    pub fn encode_double(&mut self, byte_offset: i64, value: f64,) {
        type CallSig = ((), i64, f64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(735usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_double", self.sys_ptr, args)
        }
    }
    pub fn encode_var(&mut self, byte_offset: i64, value: &Variant, allow_objects: bool,) -> i64 {
        type CallSig < 'a0, > = (i64, i64, RefArg < 'a0, Variant >, bool);
        let args = (byte_offset, RefArg::new(value), allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(736usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_var", self.sys_ptr, args)
        }
    }
}
impl PackedByteArray {
    pub fn get_string_from_ascii(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(698usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_ascii", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_string_from_utf8(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(699usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf8", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_string_from_utf16(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(700usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf16", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_string_from_utf32(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(701usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf32", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_string_from_wchar(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(702usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_wchar", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn hex_encode(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(703usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "hex_encode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
}