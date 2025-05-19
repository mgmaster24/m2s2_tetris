#![doc = "Sidecar module for class [`Light3D`][crate::classes::Light3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Light3D` enums](https://docs.godotengine.org/en/stable/classes/class_light3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Light3D.`\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`light_3d`][crate::classes::light_3d]: sidecar module with related enum/flag types\n* [`ILight3D`][crate::classes::ILight3D]: virtual methods\n\n\nSee also [Godot docs for `Light3D`](https://docs.godotengine.org/en/stable/classes/class_light3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Light3D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Light3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Light3D`][crate::classes::Light3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Light3D` methods](https://docs.godotengine.org/en/stable/classes/class_light3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILight3D: crate::obj::GodotClass < Base = Light3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl Light3D {
        pub fn set_editor_only(&mut self, editor_only: bool,) {
            type CallSig = ((), bool);
            let args = (editor_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_editor_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editor_only(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "is_editor_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param(&mut self, param: crate::classes::light_3d::Param, value: f32,) {
            type CallSig = ((), crate::classes::light_3d::Param, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param(&self, param: crate::classes::light_3d::Param,) -> f32 {
            type CallSig = (f32, crate::classes::light_3d::Param);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_shadow(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "has_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_negative(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_negative", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_negative(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "is_negative", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cull_mask(&mut self, cull_mask: u32,) {
            type CallSig = ((), u32);
            let args = (cull_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_distance_fade(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_enable_distance_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_distance_fade_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "is_distance_fade_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_begin(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_distance_fade_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_begin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_distance_fade_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_shadow(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_distance_fade_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_shadow(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_distance_fade_shadow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_length(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_distance_fade_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_length(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_distance_fade_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_reverse_cull_face(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_shadow_reverse_cull_face", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_reverse_cull_face(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_shadow_reverse_cull_face", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_mode(&mut self, bake_mode: crate::classes::light_3d::BakeMode,) {
            type CallSig = ((), crate::classes::light_3d::BakeMode);
            let args = (bake_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_bake_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_mode(&self,) -> crate::classes::light_3d::BakeMode {
            type CallSig = (crate::classes::light_3d::BakeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_bake_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_projector(&mut self, projector: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (projector.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_projector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_projector(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_projector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_temperature(&mut self, temperature: f32,) {
            type CallSig = ((), f32);
            let args = (temperature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "set_temperature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_temperature(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_temperature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_correlated_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Light3D", "get_correlated_color", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Light3D {
        type Base = crate::classes::VisualInstance3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Light3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Light3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for Light3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Light3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Light3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Light3D {
        
    }
    impl std::ops::Deref for Light3D {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Light3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Light3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Light3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Light3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
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
pub struct Param {
    ord: i32
}
impl Param {
    #[doc(alias = "PARAM_ENERGY")]
    #[doc = "Godot enumerator name: `PARAM_ENERGY`"]
    pub const ENERGY: Param = Param {
        ord: 0i32
    };
    #[doc(alias = "PARAM_INDIRECT_ENERGY")]
    #[doc = "Godot enumerator name: `PARAM_INDIRECT_ENERGY`"]
    pub const INDIRECT_ENERGY: Param = Param {
        ord: 1i32
    };
    #[doc(alias = "PARAM_VOLUMETRIC_FOG_ENERGY")]
    #[doc = "Godot enumerator name: `PARAM_VOLUMETRIC_FOG_ENERGY`"]
    pub const VOLUMETRIC_FOG_ENERGY: Param = Param {
        ord: 2i32
    };
    #[doc(alias = "PARAM_SPECULAR")]
    #[doc = "Godot enumerator name: `PARAM_SPECULAR`"]
    pub const SPECULAR: Param = Param {
        ord: 3i32
    };
    #[doc(alias = "PARAM_RANGE")]
    #[doc = "Godot enumerator name: `PARAM_RANGE`"]
    pub const RANGE: Param = Param {
        ord: 4i32
    };
    #[doc(alias = "PARAM_SIZE")]
    #[doc = "Godot enumerator name: `PARAM_SIZE`"]
    pub const SIZE: Param = Param {
        ord: 5i32
    };
    #[doc(alias = "PARAM_ATTENUATION")]
    #[doc = "Godot enumerator name: `PARAM_ATTENUATION`"]
    pub const ATTENUATION: Param = Param {
        ord: 6i32
    };
    #[doc(alias = "PARAM_SPOT_ANGLE")]
    #[doc = "Godot enumerator name: `PARAM_SPOT_ANGLE`"]
    pub const SPOT_ANGLE: Param = Param {
        ord: 7i32
    };
    #[doc(alias = "PARAM_SPOT_ATTENUATION")]
    #[doc = "Godot enumerator name: `PARAM_SPOT_ATTENUATION`"]
    pub const SPOT_ATTENUATION: Param = Param {
        ord: 8i32
    };
    #[doc(alias = "PARAM_SHADOW_MAX_DISTANCE")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_MAX_DISTANCE`"]
    pub const SHADOW_MAX_DISTANCE: Param = Param {
        ord: 9i32
    };
    #[doc(alias = "PARAM_SHADOW_SPLIT_1_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_SPLIT_1_OFFSET`"]
    pub const SHADOW_SPLIT_1_OFFSET: Param = Param {
        ord: 10i32
    };
    #[doc(alias = "PARAM_SHADOW_SPLIT_2_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_SPLIT_2_OFFSET`"]
    pub const SHADOW_SPLIT_2_OFFSET: Param = Param {
        ord: 11i32
    };
    #[doc(alias = "PARAM_SHADOW_SPLIT_3_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_SPLIT_3_OFFSET`"]
    pub const SHADOW_SPLIT_3_OFFSET: Param = Param {
        ord: 12i32
    };
    #[doc(alias = "PARAM_SHADOW_FADE_START")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_FADE_START`"]
    pub const SHADOW_FADE_START: Param = Param {
        ord: 13i32
    };
    #[doc(alias = "PARAM_SHADOW_NORMAL_BIAS")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_NORMAL_BIAS`"]
    pub const SHADOW_NORMAL_BIAS: Param = Param {
        ord: 14i32
    };
    #[doc(alias = "PARAM_SHADOW_BIAS")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_BIAS`"]
    pub const SHADOW_BIAS: Param = Param {
        ord: 15i32
    };
    #[doc(alias = "PARAM_SHADOW_PANCAKE_SIZE")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_PANCAKE_SIZE`"]
    pub const SHADOW_PANCAKE_SIZE: Param = Param {
        ord: 16i32
    };
    #[doc(alias = "PARAM_SHADOW_OPACITY")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_OPACITY`"]
    pub const SHADOW_OPACITY: Param = Param {
        ord: 17i32
    };
    #[doc(alias = "PARAM_SHADOW_BLUR")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_BLUR`"]
    pub const SHADOW_BLUR: Param = Param {
        ord: 18i32
    };
    #[doc(alias = "PARAM_TRANSMITTANCE_BIAS")]
    #[doc = "Godot enumerator name: `PARAM_TRANSMITTANCE_BIAS`"]
    pub const TRANSMITTANCE_BIAS: Param = Param {
        ord: 19i32
    };
    #[doc(alias = "PARAM_INTENSITY")]
    #[doc = "Godot enumerator name: `PARAM_INTENSITY`"]
    pub const INTENSITY: Param = Param {
        ord: 20i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Param = Param {
        ord: 21i32
    };
    
}
impl std::fmt::Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Param") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Param {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 => Some(Self {
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
            Self::ENERGY => "ENERGY", Self::INDIRECT_ENERGY => "INDIRECT_ENERGY", Self::VOLUMETRIC_FOG_ENERGY => "VOLUMETRIC_FOG_ENERGY", Self::SPECULAR => "SPECULAR", Self::RANGE => "RANGE", Self::SIZE => "SIZE", Self::ATTENUATION => "ATTENUATION", Self::SPOT_ANGLE => "SPOT_ANGLE", Self::SPOT_ATTENUATION => "SPOT_ATTENUATION", Self::SHADOW_MAX_DISTANCE => "SHADOW_MAX_DISTANCE", Self::SHADOW_SPLIT_1_OFFSET => "SHADOW_SPLIT_1_OFFSET", Self::SHADOW_SPLIT_2_OFFSET => "SHADOW_SPLIT_2_OFFSET", Self::SHADOW_SPLIT_3_OFFSET => "SHADOW_SPLIT_3_OFFSET", Self::SHADOW_FADE_START => "SHADOW_FADE_START", Self::SHADOW_NORMAL_BIAS => "SHADOW_NORMAL_BIAS", Self::SHADOW_BIAS => "SHADOW_BIAS", Self::SHADOW_PANCAKE_SIZE => "SHADOW_PANCAKE_SIZE", Self::SHADOW_OPACITY => "SHADOW_OPACITY", Self::SHADOW_BLUR => "SHADOW_BLUR", Self::TRANSMITTANCE_BIAS => "TRANSMITTANCE_BIAS", Self::INTENSITY => "INTENSITY", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ENERGY => "PARAM_ENERGY", Self::INDIRECT_ENERGY => "PARAM_INDIRECT_ENERGY", Self::VOLUMETRIC_FOG_ENERGY => "PARAM_VOLUMETRIC_FOG_ENERGY", Self::SPECULAR => "PARAM_SPECULAR", Self::RANGE => "PARAM_RANGE", Self::SIZE => "PARAM_SIZE", Self::ATTENUATION => "PARAM_ATTENUATION", Self::SPOT_ANGLE => "PARAM_SPOT_ANGLE", Self::SPOT_ATTENUATION => "PARAM_SPOT_ATTENUATION", Self::SHADOW_MAX_DISTANCE => "PARAM_SHADOW_MAX_DISTANCE", Self::SHADOW_SPLIT_1_OFFSET => "PARAM_SHADOW_SPLIT_1_OFFSET", Self::SHADOW_SPLIT_2_OFFSET => "PARAM_SHADOW_SPLIT_2_OFFSET", Self::SHADOW_SPLIT_3_OFFSET => "PARAM_SHADOW_SPLIT_3_OFFSET", Self::SHADOW_FADE_START => "PARAM_SHADOW_FADE_START", Self::SHADOW_NORMAL_BIAS => "PARAM_SHADOW_NORMAL_BIAS", Self::SHADOW_BIAS => "PARAM_SHADOW_BIAS", Self::SHADOW_PANCAKE_SIZE => "PARAM_SHADOW_PANCAKE_SIZE", Self::SHADOW_OPACITY => "PARAM_SHADOW_OPACITY", Self::SHADOW_BLUR => "PARAM_SHADOW_BLUR", Self::TRANSMITTANCE_BIAS => "PARAM_TRANSMITTANCE_BIAS", Self::INTENSITY => "PARAM_INTENSITY", Self::MAX => "PARAM_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Param {
    const ENUMERATOR_COUNT: usize = 21usize;
    
}
impl crate::meta::GodotConvert for Param {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Param {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Param {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BakeMode {
    ord: i32
}
impl BakeMode {
    #[doc(alias = "BAKE_DISABLED")]
    #[doc = "Godot enumerator name: `BAKE_DISABLED`"]
    pub const DISABLED: BakeMode = BakeMode {
        ord: 0i32
    };
    #[doc(alias = "BAKE_STATIC")]
    #[doc = "Godot enumerator name: `BAKE_STATIC`"]
    pub const STATIC: BakeMode = BakeMode {
        ord: 1i32
    };
    #[doc(alias = "BAKE_DYNAMIC")]
    #[doc = "Godot enumerator name: `BAKE_DYNAMIC`"]
    pub const DYNAMIC: BakeMode = BakeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for BakeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BakeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BakeMode {
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
            Self::DISABLED => "DISABLED", Self::STATIC => "STATIC", Self::DYNAMIC => "DYNAMIC", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "BAKE_DISABLED", Self::STATIC => "BAKE_STATIC", Self::DYNAMIC => "BAKE_DYNAMIC", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BakeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BakeMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BakeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}