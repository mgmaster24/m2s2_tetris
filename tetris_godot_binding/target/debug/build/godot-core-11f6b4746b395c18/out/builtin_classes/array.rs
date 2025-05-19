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
pub struct InnerArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerArray < 'a > {
    pub fn from_outer(outer: &VariantArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn from_outer_typed < T > (outer: &Array < T >) -> Self where T: crate::meta::ArrayElement {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(632usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(633usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(634usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(635usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "hash", self.sys_ptr, args)
        }
    }
    pub fn assign(&mut self, array: &VariantArray,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(636usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "assign", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: &Variant,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(637usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn push_front(&mut self, value: &Variant,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(638usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "push_front", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: &Variant,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(639usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &VariantArray,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(640usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, size: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(641usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, position: i64, value: &Variant,) -> i64 {
        type CallSig < 'a0, > = (i64, i64, RefArg < 'a0, Variant >);
        let args = (position, RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(642usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, position: i64,) {
        type CallSig = ((), i64);
        let args = (position,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(643usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: &Variant,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(644usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: &Variant,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(645usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "erase", self.sys_ptr, args)
        }
    }
    pub fn front(&self,) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(646usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "front", self.sys_ptr, args)
        }
    }
    pub fn back(&self,) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(647usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "back", self.sys_ptr, args)
        }
    }
    pub fn pick_random(&self,) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(648usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "pick_random", self.sys_ptr, args)
        }
    }
    pub fn find(&self, what: &Variant, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, RefArg < 'a0, Variant >, i64);
        let args = (RefArg::new(what), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(649usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, what: &Variant, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, RefArg < 'a0, Variant >, i64);
        let args = (RefArg::new(what), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(650usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: &Variant,) -> i64 {
        type CallSig < 'a0, > = (i64, RefArg < 'a0, Variant >);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(651usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "count", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: &Variant,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, Variant >);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(652usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "has", self.sys_ptr, args)
        }
    }
    pub fn pop_back(&mut self,) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(653usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "pop_back", self.sys_ptr, args)
        }
    }
    pub fn pop_front(&mut self,) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(654usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "pop_front", self.sys_ptr, args)
        }
    }
    pub fn pop_at(&mut self, position: i64,) -> Variant {
        type CallSig = (Variant, i64);
        let args = (position,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(655usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "pop_at", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(656usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn sort_custom(&mut self, func: &Callable,) {
        type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >);
        let args = (RefArg::new(func),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(657usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "sort_custom", self.sys_ptr, args)
        }
    }
    pub fn shuffle(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(658usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "shuffle", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&self, value: &Variant, before: bool,) -> i64 {
        type CallSig < 'a0, > = (i64, RefArg < 'a0, Variant >, bool);
        let args = (RefArg::new(value), before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(659usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn bsearch_custom(&self, value: &Variant, func: &Callable, before: bool,) -> i64 {
        type CallSig < 'a0, 'a1, > = (i64, RefArg < 'a0, Variant >, RefArg < 'a1, Callable >, bool);
        let args = (RefArg::new(value), RefArg::new(func), before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(660usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "bsearch_custom", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(661usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "reverse", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array)."]
    pub unsafe fn duplicate(&self, deep: bool,) -> VariantArray {
        type CallSig = (VariantArray, bool);
        let args = (deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(662usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "duplicate", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array)."]
    pub unsafe fn slice(&self, begin: i64, end: i64, step: i64, deep: bool,) -> VariantArray {
        type CallSig = (VariantArray, i64, i64, i64, bool);
        let args = (begin, end, step, deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(663usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "slice", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array)."]
    pub unsafe fn filter(&self, method: &Callable,) -> VariantArray {
        type CallSig < 'a0, > = (VariantArray, RefArg < 'a0, Callable >);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(664usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "filter", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array)."]
    pub unsafe fn map(&self, method: &Callable,) -> VariantArray {
        type CallSig < 'a0, > = (VariantArray, RefArg < 'a0, Callable >);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(665usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "map", self.sys_ptr, args)
        }
    }
    pub fn reduce(&self, method: &Callable, accum: &Variant,) -> Variant {
        type CallSig < 'a0, 'a1, > = (Variant, RefArg < 'a0, Callable >, RefArg < 'a1, Variant >);
        let args = (RefArg::new(method), RefArg::new(accum),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(666usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "reduce", self.sys_ptr, args)
        }
    }
    pub fn any(&self, method: &Callable,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, Callable >);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(667usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "any", self.sys_ptr, args)
        }
    }
    pub fn all(&self, method: &Callable,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, Callable >);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(668usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "all", self.sys_ptr, args)
        }
    }
    pub fn max(&self,) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(669usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "max", self.sys_ptr, args)
        }
    }
    pub fn min(&self,) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(670usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "min", self.sys_ptr, args)
        }
    }
    pub fn is_typed(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(671usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "is_typed", self.sys_ptr, args)
        }
    }
    pub fn is_same_typed(&self, array: &VariantArray,) -> bool {
        type CallSig < 'a0, > = (bool, RefArg < 'a0, VariantArray >);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(672usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "is_same_typed", self.sys_ptr, args)
        }
    }
    pub fn get_typed_builtin(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(673usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_builtin", self.sys_ptr, args)
        }
    }
    pub fn get_typed_class_name(&self,) -> StringName {
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(674usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_class_name", self.sys_ptr, args)
        }
    }
    pub fn get_typed_script(&self,) -> Variant {
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(675usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_script", self.sys_ptr, args)
        }
    }
    pub fn make_read_only(&mut self,) {
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(676usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "make_read_only", self.sys_ptr, args)
        }
    }
    pub fn is_read_only(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(677usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Array", "is_read_only", self.sys_ptr, args)
        }
    }
}
impl VariantArray {
    
}