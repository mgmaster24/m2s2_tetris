#![doc = "Sidecar module for class [`Light2D`][crate::classes::Light2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Light2D` enums](https://docs.godotengine.org/en/stable/classes/class_light2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Light2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`light_2d`][crate::classes::light_2d]: sidecar module with related enum/flag types\n* [`ILight2D`][crate::classes::ILight2D]: virtual methods\n\n\nSee also [Godot docs for `Light2D`](https://docs.godotengine.org/en/stable/classes/class_light2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Light2D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Light2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Light2D`][crate::classes::Light2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Light2D` methods](https://docs.godotengine.org/en/stable/classes/class_light2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILight2D: crate::obj::GodotClass < Base = Light2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
        fn draw(&mut self,) {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
    }
    impl Light2D {
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editor_only(&mut self, editor_only: bool,) {
            type CallSig = ((), bool);
            let args = (editor_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_editor_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editor_only(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "is_editor_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_energy(&mut self, energy: f32,) {
            type CallSig = ((), f32);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_energy(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_z_range_min(&mut self, z: i32,) {
            type CallSig = ((), i32);
            let args = (z,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_z_range_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_z_range_min(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_z_range_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_z_range_max(&mut self, z: i32,) {
            type CallSig = ((), i32);
            let args = (z,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_z_range_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_z_range_max(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_z_range_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_range_min(&mut self, layer: i32,) {
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_layer_range_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_range_min(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_layer_range_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_range_max(&mut self, layer: i32,) {
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_layer_range_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_range_max(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_layer_range_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_cull_mask(&mut self, item_cull_mask: i32,) {
            type CallSig = ((), i32);
            let args = (item_cull_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_item_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_cull_mask(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_item_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_shadow_cull_mask(&mut self, item_shadow_cull_mask: i32,) {
            type CallSig = ((), i32);
            let args = (item_shadow_cull_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_item_shadow_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_shadow_cull_mask(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_item_shadow_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_shadow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shadow_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "is_shadow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_smooth(&mut self, smooth: f32,) {
            type CallSig = ((), f32);
            let args = (smooth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_shadow_smooth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_smooth(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_shadow_smooth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_filter(&mut self, filter: crate::classes::light_2d::ShadowFilter,) {
            type CallSig = ((), crate::classes::light_2d::ShadowFilter);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_shadow_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_filter(&self,) -> crate::classes::light_2d::ShadowFilter {
            type CallSig = (crate::classes::light_2d::ShadowFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_shadow_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_color(&mut self, shadow_color: Color,) {
            type CallSig = ((), Color);
            let args = (shadow_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_shadow_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_shadow_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_mode(&mut self, mode: crate::classes::light_2d::BlendMode,) {
            type CallSig = ((), crate::classes::light_2d::BlendMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::classes::light_2d::BlendMode {
            type CallSig = (crate::classes::light_2d::BlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, height: f32,) {
            type CallSig = ((), f32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light2D", "get_height", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Light2D {
        type Base = crate::classes::Node2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Light2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Light2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Light2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Light2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Light2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Light2D {
        
    }
    impl std::ops::Deref for Light2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Light2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Light2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Light2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Light2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadowFilter {
    ord: i32
}
impl ShadowFilter {
    #[doc(alias = "SHADOW_FILTER_NONE")]
    #[doc = "Godot enumerator name: `SHADOW_FILTER_NONE`"]
    pub const NONE: ShadowFilter = ShadowFilter {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_FILTER_PCF5")]
    #[doc = "Godot enumerator name: `SHADOW_FILTER_PCF5`"]
    pub const PCF5: ShadowFilter = ShadowFilter {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_FILTER_PCF13")]
    #[doc = "Godot enumerator name: `SHADOW_FILTER_PCF13`"]
    pub const PCF13: ShadowFilter = ShadowFilter {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ShadowFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowFilter {
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
            Self::NONE => "NONE", Self::PCF5 => "PCF5", Self::PCF13 => "PCF13", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "SHADOW_FILTER_NONE", Self::PCF5 => "SHADOW_FILTER_PCF5", Self::PCF13 => "SHADOW_FILTER_PCF13", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ShadowFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadowFilter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendMode {
    ord: i32
}
impl BlendMode {
    #[doc(alias = "BLEND_MODE_ADD")]
    #[doc = "Godot enumerator name: `BLEND_MODE_ADD`"]
    pub const ADD: BlendMode = BlendMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_MODE_SUB")]
    #[doc = "Godot enumerator name: `BLEND_MODE_SUB`"]
    pub const SUB: BlendMode = BlendMode {
        ord: 1i32
    };
    #[doc(alias = "BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `BLEND_MODE_MIX`"]
    pub const MIX: BlendMode = BlendMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendMode {
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
            Self::ADD => "ADD", Self::SUB => "SUB", Self::MIX => "MIX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ADD => "BLEND_MODE_ADD", Self::SUB => "BLEND_MODE_SUB", Self::MIX => "BLEND_MODE_MIX", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}