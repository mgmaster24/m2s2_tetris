#![doc = "Sidecar module for class [`GltfAccessor`][crate::classes::GltfAccessor].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFAccessor` enums](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFAccessor.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gltf_accessor`][crate::classes::gltf_accessor]: sidecar module with related enum/flag types\n* [`IGltfAccessor`][crate::classes::IGltfAccessor]: virtual methods\n\n\nSee also [Godot docs for `GLTFAccessor`](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfAccessor::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfAccessor {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfAccessor`][crate::classes::GltfAccessor].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFAccessor` methods](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfAccessor: crate::obj::GodotClass < Base = GltfAccessor > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfAccessor {
        pub fn get_buffer_view(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_buffer_view(&mut self, buffer_view: i32,) {
            type CallSig = ((), i32);
            let args = (buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_byte_offset(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_byte_offset(&mut self, byte_offset: i32,) {
            type CallSig = ((), i32);
            let args = (byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_component_type(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_component_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_component_type(&mut self, component_type: i32,) {
            type CallSig = ((), i32);
            let args = (component_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_component_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_normalized(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_normalized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normalized(&mut self, normalized: bool,) {
            type CallSig = ((), bool);
            let args = (normalized,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_normalized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_count(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_accessor_type(&mut self,) -> crate::classes::gltf_accessor::GltfAccessorType {
            type CallSig = (crate::classes::gltf_accessor::GltfAccessorType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_accessor_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_accessor_type(&mut self, accessor_type: crate::classes::gltf_accessor::GltfAccessorType,) {
            type CallSig = ((), crate::classes::gltf_accessor::GltfAccessorType);
            let args = (accessor_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_accessor_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_type(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_type(&mut self, type_: i32,) {
            type CallSig = ((), i32);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min(&mut self,) -> PackedFloat64Array {
            type CallSig = (PackedFloat64Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min(&mut self, min: &PackedFloat64Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedFloat64Array >);
            let args = (RefArg::new(min),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max(&mut self,) -> PackedFloat64Array {
            type CallSig = (PackedFloat64Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max(&mut self, max: &PackedFloat64Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedFloat64Array >);
            let args = (RefArg::new(max),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_count(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_count(&mut self, sparse_count: i32,) {
            type CallSig = ((), i32);
            let args = (sparse_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_indices_buffer_view(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_indices_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_indices_buffer_view(&mut self, sparse_indices_buffer_view: i32,) {
            type CallSig = ((), i32);
            let args = (sparse_indices_buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_indices_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_indices_byte_offset(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_indices_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_indices_byte_offset(&mut self, sparse_indices_byte_offset: i32,) {
            type CallSig = ((), i32);
            let args = (sparse_indices_byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_indices_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_indices_component_type(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_indices_component_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_indices_component_type(&mut self, sparse_indices_component_type: i32,) {
            type CallSig = ((), i32);
            let args = (sparse_indices_component_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_indices_component_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_values_buffer_view(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_values_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_values_buffer_view(&mut self, sparse_values_buffer_view: i32,) {
            type CallSig = ((), i32);
            let args = (sparse_values_buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_values_buffer_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sparse_values_byte_offset(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_values_byte_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sparse_values_byte_offset(&mut self, sparse_values_byte_offset: i32,) {
            type CallSig = ((), i32);
            let args = (sparse_values_byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_values_byte_offset", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GltfAccessor {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GLTFAccessor"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfAccessor {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfAccessor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfAccessor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfAccessor {
        
    }
    impl crate::obj::cap::GodotDefault for GltfAccessor {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfAccessor {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfAccessor {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfAccessor`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GltfAccessor {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfAccessor > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `GLTFAccessorType`."]
pub struct GltfAccessorType {
    ord: i32
}
impl GltfAccessorType {
    #[doc(alias = "TYPE_SCALAR")]
    #[doc = "Godot enumerator name: `TYPE_SCALAR`"]
    pub const SCALAR: GltfAccessorType = GltfAccessorType {
        ord: 0i32
    };
    #[doc(alias = "TYPE_VEC2")]
    #[doc = "Godot enumerator name: `TYPE_VEC2`"]
    pub const VEC2: GltfAccessorType = GltfAccessorType {
        ord: 1i32
    };
    #[doc(alias = "TYPE_VEC3")]
    #[doc = "Godot enumerator name: `TYPE_VEC3`"]
    pub const VEC3: GltfAccessorType = GltfAccessorType {
        ord: 2i32
    };
    #[doc(alias = "TYPE_VEC4")]
    #[doc = "Godot enumerator name: `TYPE_VEC4`"]
    pub const VEC4: GltfAccessorType = GltfAccessorType {
        ord: 3i32
    };
    #[doc(alias = "TYPE_MAT2")]
    #[doc = "Godot enumerator name: `TYPE_MAT2`"]
    pub const MAT2: GltfAccessorType = GltfAccessorType {
        ord: 4i32
    };
    #[doc(alias = "TYPE_MAT3")]
    #[doc = "Godot enumerator name: `TYPE_MAT3`"]
    pub const MAT3: GltfAccessorType = GltfAccessorType {
        ord: 5i32
    };
    #[doc(alias = "TYPE_MAT4")]
    #[doc = "Godot enumerator name: `TYPE_MAT4`"]
    pub const MAT4: GltfAccessorType = GltfAccessorType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for GltfAccessorType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GltfAccessorType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GltfAccessorType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::SCALAR => "SCALAR", Self::VEC2 => "VEC2", Self::VEC3 => "VEC3", Self::VEC4 => "VEC4", Self::MAT2 => "MAT2", Self::MAT3 => "MAT3", Self::MAT4 => "MAT4", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SCALAR => "TYPE_SCALAR", Self::VEC2 => "TYPE_VEC2", Self::VEC3 => "TYPE_VEC3", Self::VEC4 => "TYPE_VEC4", Self::MAT2 => "TYPE_MAT2", Self::MAT3 => "TYPE_MAT3", Self::MAT4 => "TYPE_MAT4", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for GltfAccessorType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GltfAccessorType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GltfAccessorType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}