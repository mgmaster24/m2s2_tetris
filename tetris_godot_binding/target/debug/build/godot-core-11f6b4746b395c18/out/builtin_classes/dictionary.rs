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
pub struct InnerDictionary < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerDictionary < 'a > {
    pub fn from_outer(outer: &Dictionary) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(614usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(615usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(616usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "clear", self.sys_ptr, args)
        }
    }
    pub fn merge(&mut self, dictionary: &Dictionary, overwrite: bool,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, Dictionary >, bool);
        let args = (RefArg::new(dictionary), overwrite,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(617usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "merge", self.sys_ptr, args)
        }
    }
    pub fn merged(&self, dictionary: &Dictionary, overwrite: bool,) -> Dictionary {
        type CallSig < 'a0, > = (Dictionary, RefArg < 'a0, Dictionary >, bool);
        let args = (RefArg::new(dictionary), overwrite,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(618usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "merged", self.sys_ptr, args)
        }
    }
    pub fn has(&self, key: &Variant,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, Variant >);
        let args = (RefArg::new(key),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(619usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "has", self.sys_ptr, args)
        }
    }
    pub fn has_all(&self, keys: &VariantArray,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, VariantArray >);
        let args = (RefArg::new(keys),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(620usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "has_all", self.sys_ptr, args)
        }
    }
    pub fn find_key(&self, value: &Variant,) -> Variant {
        type CallSig < 'a0, > = (Variant, RefArg < 'a0, Variant >);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(621usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "find_key", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, key: &Variant,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, Variant >);
        let args = (RefArg::new(key),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(622usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "erase", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(623usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "hash", self.sys_ptr, args)
        }
    }
    pub fn keys(&self,) -> VariantArray {
        type CallSig = (VariantArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(624usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "keys", self.sys_ptr, args)
        }
    }
    pub fn values(&self,) -> VariantArray {
        type CallSig = (VariantArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(625usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "values", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&self, deep: bool,) -> Dictionary {
        type CallSig = (Dictionary, bool);
        let args = (deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(626usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn get(&self, key: &Variant, default: &Variant,) -> Variant {
        type CallSig < 'a0, 'a1, > = (Variant, RefArg < 'a0, Variant >, RefArg < 'a1, Variant >);
        let args = (RefArg::new(key), RefArg::new(default),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(627usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "get", self.sys_ptr, args)
        }
    }
    pub fn get_or_add(&mut self, key: &Variant, default: &Variant,) -> Variant {
        type CallSig < 'a0, 'a1, > = (Variant, RefArg < 'a0, Variant >, RefArg < 'a1, Variant >);
        let args = (RefArg::new(key), RefArg::new(default),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(628usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_or_add", self.sys_ptr, args)
        }
    }
    pub fn make_read_only(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(629usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "make_read_only", self.sys_ptr, args)
        }
    }
    pub fn is_read_only(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(630usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_read_only", self.sys_ptr, args)
        }
    }
    pub fn recursive_equal(&self, dictionary: &Dictionary, recursion_count: i64,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, Dictionary >, i64);
        let args = (RefArg::new(dictionary), recursion_count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(631usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Dictionary", "recursive_equal", self.sys_ptr, args)
        }
    }
}
impl Dictionary {
    
}