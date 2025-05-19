#![doc = "Sidecar module for class [`Engine`][crate::classes::Engine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Engine` enums](https://docs.godotengine.org/en/stable/classes/class_engine.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Engine.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`IEngine`][crate::classes::IEngine]: virtual methods\n\n\nSee also [Godot docs for `Engine`](https://docs.godotengine.org/en/stable/classes/class_engine.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Engine::singleton()`][Engine::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Engine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Engine`][crate::classes::Engine].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Engine` methods](https://docs.godotengine.org/en/stable/classes/class_engine.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEngine: crate::obj::GodotClass < Base = Engine > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl Engine {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"Engine");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn set_physics_ticks_per_second(&mut self, physics_ticks_per_second: i32,) {
            type CallSig = ((), i32);
            let args = (physics_ticks_per_second,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "set_physics_ticks_per_second", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_ticks_per_second(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_physics_ticks_per_second", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_physics_steps_per_frame(&mut self, max_physics_steps: i32,) {
            type CallSig = ((), i32);
            let args = (max_physics_steps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "set_max_physics_steps_per_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_physics_steps_per_frame(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_max_physics_steps_per_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_jitter_fix(&mut self, physics_jitter_fix: f64,) {
            type CallSig = ((), f64);
            let args = (physics_jitter_fix,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "set_physics_jitter_fix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_jitter_fix(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_physics_jitter_fix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_interpolation_fraction(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_physics_interpolation_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_fps(&mut self, max_fps: i32,) {
            type CallSig = ((), i32);
            let args = (max_fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "set_max_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_fps(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_max_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_time_scale(&mut self, time_scale: f64,) {
            type CallSig = ((), f64);
            let args = (time_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "set_time_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_scale(&mut self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_time_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frames_drawn(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_frames_drawn", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frames_per_second(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_frames_per_second", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_frames(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_physics_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_frames(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_process_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_main_loop(&self,) -> Option < Gd < crate::classes::MainLoop > > {
            type CallSig = (Option < Gd < crate::classes::MainLoop > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_main_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version_info(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_version_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_author_info(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_author_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_copyright_info(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_copyright_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_donor_info(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_donor_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_license_info(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_license_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_license_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_license_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_architecture_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_architecture_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_physics_frame(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "is_in_physics_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_singleton(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "has_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_singleton(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Object > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Object > >, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_singleton(&mut self, name: impl AsArg < StringName >, instance: impl AsObjectArg < crate::classes::Object >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, ObjectArg < crate::classes::Object >);
            let args = (name.into_arg(), instance.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "register_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_singleton(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "unregister_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_singleton_list(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_singleton_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_script_language(&mut self, language: impl AsObjectArg < crate::classes::ScriptLanguage >,) -> crate::global::Error {
            type CallSig = (crate::global::Error, ObjectArg < crate::classes::ScriptLanguage >);
            let args = (language.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "register_script_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_script_language(&mut self, language: impl AsObjectArg < crate::classes::ScriptLanguage >,) -> crate::global::Error {
            type CallSig = (crate::global::Error, ObjectArg < crate::classes::ScriptLanguage >);
            let args = (language.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "unregister_script_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_language_count(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_script_language_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_language(&self, index: i32,) -> Option < Gd < crate::classes::ScriptLanguage > > {
            type CallSig = (Option < Gd < crate::classes::ScriptLanguage > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_script_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editor_hint(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "is_editor_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_movie_path(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "get_write_movie_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_print_error_messages(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "set_print_error_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_printing_error_messages(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Engine", "is_printing_error_messages", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Engine {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Engine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Engine {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Engine {
        
    }
    impl std::ops::Deref for Engine {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Engine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Engine`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Engine {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Engine > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}