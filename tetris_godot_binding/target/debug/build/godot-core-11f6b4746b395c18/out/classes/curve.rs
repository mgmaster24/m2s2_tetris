#![doc = "Sidecar module for class [`Curve`][crate::classes::Curve].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Curve` enums](https://docs.godotengine.org/en/stable/classes/class_curve.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, AsObjectArg, ClassName, CowArg, ObjectArg, ObjectCow, PtrcallSignatureTuple, RefArg, VarcallSignatureTuple
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Curve.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`curve`][crate::classes::curve]: sidecar module with related enum/flag types\n* [`ICurve`][crate::classes::ICurve]: virtual methods\n\n\nSee also [Godot docs for `Curve`](https://docs.godotengine.org/en/stable/classes/class_curve.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Curve::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Curve {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Curve`][crate::classes::Curve].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Curve` methods](https://docs.godotengine.org/en/stable/classes/class_curve.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICurve: crate::obj::GodotClass < Base = Curve > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn get_property(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn set_property(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        fn get_property_list(&mut self) -> Vec < crate::meta::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        #[cfg(since_api = "4.2")]
        fn validate_property(&self, property: &mut crate::meta::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Curve {
        pub fn get_point_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_point_full(&mut self, position: Vector2, left_tangent: f32, right_tangent: f32, left_mode: crate::classes::curve::TangentMode, right_mode: crate::classes::curve::TangentMode,) -> i32 {
            type CallSig = (i32, Vector2, f32, f32, crate::classes::curve::TangentMode, crate::classes::curve::TangentMode);
            let args = (position, left_tangent, right_tangent, left_mode, right_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "add_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_point(&mut self, position: Vector2,) -> i32 {
            self.add_point_ex(position,) . done()
        }
        #[inline]
        pub fn add_point_ex < 'a > (&'a mut self, position: Vector2,) -> ExAddPoint < 'a > {
            ExAddPoint::new(self, position,)
        }
        pub fn remove_point(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "remove_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_points(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "clear_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_position(&self, index: i32,) -> Vector2 {
            type CallSig = (Vector2, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_value(&mut self, index: i32, y: f32,) {
            type CallSig = ((), i32, f32);
            let args = (index, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_point_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_offset(&mut self, index: i32, offset: f32,) -> i32 {
            type CallSig = (i32, i32, f32);
            let args = (index, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_point_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sample(&self, offset: f32,) -> f32 {
            type CallSig = (f32, f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "sample", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sample_baked(&self, offset: f32,) -> f32 {
            type CallSig = (f32, f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "sample_baked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_left_tangent(&self, index: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_point_left_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_right_tangent(&self, index: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_point_right_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_left_mode(&self, index: i32,) -> crate::classes::curve::TangentMode {
            type CallSig = (crate::classes::curve::TangentMode, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_point_left_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_right_mode(&self, index: i32,) -> crate::classes::curve::TangentMode {
            type CallSig = (crate::classes::curve::TangentMode, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_point_right_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_left_tangent(&mut self, index: i32, tangent: f32,) {
            type CallSig = ((), i32, f32);
            let args = (index, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_point_left_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_right_tangent(&mut self, index: i32, tangent: f32,) {
            type CallSig = ((), i32, f32);
            let args = (index, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_point_right_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_left_mode(&mut self, index: i32, mode: crate::classes::curve::TangentMode,) {
            type CallSig = ((), i32, crate::classes::curve::TangentMode);
            let args = (index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_point_left_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_right_mode(&mut self, index: i32, mode: crate::classes::curve::TangentMode,) {
            type CallSig = ((), i32, crate::classes::curve::TangentMode);
            let args = (index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_point_right_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_value(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_min_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_value(&mut self, min: f32,) {
            type CallSig = ((), f32);
            let args = (min,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_min_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_value(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_max_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_value(&mut self, max: f32,) {
            type CallSig = ((), f32);
            let args = (max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_max_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clean_dupes(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "clean_dupes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn bake(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "bake", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_resolution(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "get_bake_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_resolution(&mut self, resolution: i32,) {
            type CallSig = ((), i32);
            let args = (resolution,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Curve", "set_bake_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for Curve {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Curve"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Curve {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Curve {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Curve {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Curve {
        
    }
    impl crate::obj::cap::GodotDefault for Curve {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Curve {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Curve {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Curve`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Curve {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Curve > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Curve::add_point_ex`][super::Curve::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Curve, position: Vector2, left_tangent: f32, right_tangent: f32, left_mode: crate::classes::curve::TangentMode, right_mode: crate::classes::curve::TangentMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPoint < 'a > {
    fn new(surround_object: &'a mut re_export::Curve, position: Vector2,) -> Self {
        let left_tangent = 0f32;
        let right_tangent = 0f32;
        let left_mode = crate::obj::EngineEnum::from_ord(0);
        let right_mode = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, left_tangent: left_tangent, right_tangent: right_tangent, left_mode: left_mode, right_mode: right_mode,
        }
    }
    #[inline]
    pub fn left_tangent(self, left_tangent: f32) -> Self {
        Self {
            left_tangent: left_tangent, .. self
        }
    }
    #[inline]
    pub fn right_tangent(self, right_tangent: f32) -> Self {
        Self {
            right_tangent: right_tangent, .. self
        }
    }
    #[inline]
    pub fn left_mode(self, left_mode: crate::classes::curve::TangentMode) -> Self {
        Self {
            left_mode: left_mode, .. self
        }
    }
    #[inline]
    pub fn right_mode(self, right_mode: crate::classes::curve::TangentMode) -> Self {
        Self {
            right_mode: right_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, position, left_tangent, right_tangent, left_mode, right_mode,
        }
        = self;
        re_export::Curve::add_point_full(surround_object, position, left_tangent, right_tangent, left_mode, right_mode,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TangentMode {
    ord: i32
}
impl TangentMode {
    #[doc(alias = "TANGENT_FREE")]
    #[doc = "Godot enumerator name: `TANGENT_FREE`"]
    pub const FREE: TangentMode = TangentMode {
        ord: 0i32
    };
    #[doc(alias = "TANGENT_LINEAR")]
    #[doc = "Godot enumerator name: `TANGENT_LINEAR`"]
    pub const LINEAR: TangentMode = TangentMode {
        ord: 1i32
    };
    #[doc(alias = "TANGENT_MODE_COUNT")]
    #[doc = "Godot enumerator name: `TANGENT_MODE_COUNT`"]
    pub const MODE_COUNT: TangentMode = TangentMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TangentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TangentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TangentMode {
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
            Self::FREE => "FREE", Self::LINEAR => "LINEAR", Self::MODE_COUNT => "MODE_COUNT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::FREE => "TANGENT_FREE", Self::LINEAR => "TANGENT_LINEAR", Self::MODE_COUNT => "TANGENT_MODE_COUNT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TangentMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TangentMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TangentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}