#![doc = "Sidecar module for class [`AnimationMixer`][crate::classes::AnimationMixer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationMixer` enums](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationMixer.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`animation_mixer`][crate::classes::animation_mixer]: sidecar module with related enum/flag types\n* [`IAnimationMixer`][crate::classes::IAnimationMixer]: virtual methods\n\n\nSee also [Godot docs for `AnimationMixer`](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AnimationMixer>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationMixer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationMixer`][crate::classes::AnimationMixer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationMixer` methods](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationMixer: crate::obj::GodotClass < Base = AnimationMixer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
        fn post_process_key_value(&self, animation: Option < Gd < crate::classes::Animation > >, track: i32, value: Variant, object_id: u64, object_sub_idx: i32,) -> Variant {
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
    impl AnimationMixer {
        pub fn add_animation_library(&mut self, name: impl AsArg < StringName >, library: impl AsObjectArg < crate::classes::AnimationLibrary >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, StringName >, ObjectArg < crate::classes::AnimationLibrary >);
            let args = (name.into_arg(), library.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "add_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_animation_library(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "remove_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_animation_library(&mut self, name: impl AsArg < StringName >, newname: impl AsArg < StringName >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name.into_arg(), newname.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "rename_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_animation_library(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "has_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_library(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::AnimationLibrary > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::AnimationLibrary > >, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_library_list(&self,) -> Array < StringName > {
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_library_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_animation(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "has_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Animation > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Animation > >, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_list(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deterministic(&mut self, deterministic: bool,) {
            type CallSig = ((), bool);
            let args = (deterministic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_deterministic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deterministic(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_deterministic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root_node(&mut self, path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_root_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_node(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_callback_mode_process(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeProcess,) {
            type CallSig = ((), crate::classes::animation_mixer::AnimationCallbackModeProcess);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_callback_mode_process(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeProcess {
            type CallSig = (crate::classes::animation_mixer::AnimationCallbackModeProcess,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_callback_mode_method(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeMethod,) {
            type CallSig = ((), crate::classes::animation_mixer::AnimationCallbackModeMethod);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_callback_mode_method(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeMethod {
            type CallSig = (crate::classes::animation_mixer::AnimationCallbackModeMethod,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_callback_mode_discrete(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeDiscrete,) {
            type CallSig = ((), crate::classes::animation_mixer::AnimationCallbackModeDiscrete);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_discrete", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_callback_mode_discrete(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeDiscrete {
            type CallSig = (crate::classes::animation_mixer::AnimationCallbackModeDiscrete,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_discrete", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_audio_max_polyphony(&mut self, max_polyphony: i32,) {
            type CallSig = ((), i32);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_audio_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_audio_max_polyphony(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_audio_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root_motion_track(&mut self, path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_root_motion_track", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_track(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_track", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_position(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_rotation(&self,) -> Quaternion {
            type CallSig = (Quaternion,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_scale(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_position_accumulator(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_position_accumulator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_rotation_accumulator(&self,) -> Quaternion {
            type CallSig = (Quaternion,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_rotation_accumulator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_scale_accumulator(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_scale_accumulator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_caches(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "clear_caches", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn advance(&mut self, delta: f64,) {
            type CallSig = ((), f64);
            let args = (delta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn capture_full(&mut self, name: CowArg < StringName >, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, f64, crate::classes::tween::TransitionType, crate::classes::tween::EaseType);
            let args = (name, duration, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::capture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn capture(&mut self, name: impl AsArg < StringName >, duration: f64,) {
            self.capture_ex(name, duration,) . done()
        }
        #[inline]
        pub fn capture_ex < 'a > (&'a mut self, name: impl AsArg < StringName > + 'a, duration: f64,) -> ExCapture < 'a > {
            ExCapture::new(self, name, duration,)
        }
        pub fn set_reset_on_save_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_reset_on_save_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_reset_on_save_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_reset_on_save_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_animation(&self, animation: impl AsObjectArg < crate::classes::Animation >,) -> StringName {
            type CallSig = (StringName, ObjectArg < crate::classes::Animation >);
            let args = (animation.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "find_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_animation_library(&self, animation: impl AsObjectArg < crate::classes::Animation >,) -> StringName {
            type CallSig = (StringName, ObjectArg < crate::classes::Animation >);
            let args = (animation.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationMixer", "find_animation_library", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationMixer {
        type Base = crate::classes::Node;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AnimationMixer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationMixer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AnimationMixer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationMixer {
        
    }
    impl std::ops::Deref for AnimationMixer {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationMixer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationMixer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AnimationMixer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationMixer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AnimationMixer::capture_ex`][super::AnimationMixer::capture_ex]."]
#[must_use]
pub struct ExCapture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationMixer, name: CowArg < 'a, StringName >, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCapture < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationMixer, name: impl AsArg < StringName > + 'a, duration: f64,) -> Self {
        let trans_type = crate::obj::EngineEnum::from_ord(0);
        let ease_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), duration: duration, trans_type: trans_type, ease_type: ease_type,
        }
    }
    #[inline]
    pub fn trans_type(self, trans_type: crate::classes::tween::TransitionType) -> Self {
        Self {
            trans_type: trans_type, .. self
        }
    }
    #[inline]
    pub fn ease_type(self, ease_type: crate::classes::tween::EaseType) -> Self {
        Self {
            ease_type: ease_type, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, duration, trans_type, ease_type,
        }
        = self;
        re_export::AnimationMixer::capture_full(surround_object, name, duration, trans_type, ease_type,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeProcess {
    ord: i32
}
impl AnimationCallbackModeProcess {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS`"]
    pub const PHYSICS: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_IDLE`"]
    pub const IDLE: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_MANUAL")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_MANUAL`"]
    pub const MANUAL: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeProcess") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeProcess {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", Self::MANUAL => "MANUAL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PHYSICS => "ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS", Self::IDLE => "ANIMATION_CALLBACK_MODE_PROCESS_IDLE", Self::MANUAL => "ANIMATION_CALLBACK_MODE_PROCESS_MANUAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeProcess {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationCallbackModeProcess {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeProcess {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeMethod {
    ord: i32
}
impl AnimationCallbackModeMethod {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_METHOD_DEFERRED")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_METHOD_DEFERRED`"]
    pub const DEFERRED: AnimationCallbackModeMethod = AnimationCallbackModeMethod {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE`"]
    pub const IMMEDIATE: AnimationCallbackModeMethod = AnimationCallbackModeMethod {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeMethod") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeMethod {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::DEFERRED => "DEFERRED", Self::IMMEDIATE => "IMMEDIATE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFERRED => "ANIMATION_CALLBACK_MODE_METHOD_DEFERRED", Self::IMMEDIATE => "ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeMethod {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationCallbackModeMethod {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeMethod {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeDiscrete {
    ord: i32
}
impl AnimationCallbackModeDiscrete {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT`"]
    pub const DOMINANT: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE`"]
    pub const RECESSIVE: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS`"]
    pub const FORCE_CONTINUOUS: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeDiscrete {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeDiscrete") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeDiscrete {
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
            Self::DOMINANT => "DOMINANT", Self::RECESSIVE => "RECESSIVE", Self::FORCE_CONTINUOUS => "FORCE_CONTINUOUS", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DOMINANT => "ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT", Self::RECESSIVE => "ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE", Self::FORCE_CONTINUOUS => "ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeDiscrete {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationCallbackModeDiscrete {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeDiscrete {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}