#![doc = "Sidecar module for class [`AnimationPlayer`][crate::classes::AnimationPlayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationPlayer` enums](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationPlayer.`\n\nInherits [`AnimationMixer`][crate::classes::AnimationMixer].\n\nRelated symbols:\n\n* [`animation_player`][crate::classes::animation_player]: sidecar module with related enum/flag types\n* [`IAnimationPlayer`][crate::classes::IAnimationPlayer]: virtual methods\n\n\nSee also [Godot docs for `AnimationPlayer`](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AnimationPlayer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationPlayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationPlayer`][crate::classes::AnimationPlayer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationPlayer` methods](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationPlayer: crate::obj::GodotClass < Base = AnimationPlayer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationPlayer {
        pub fn animation_set_next(&mut self, animation_from: impl AsArg < StringName >, animation_to: impl AsArg < StringName >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (animation_from.into_arg(), animation_to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "animation_set_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn animation_get_next(&self, animation_from: impl AsArg < StringName >,) -> StringName {
            type CallSig < 'a0, > = (StringName, CowArg < 'a0, StringName >);
            let args = (animation_from.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "animation_get_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_time(&mut self, animation_from: impl AsArg < StringName >, animation_to: impl AsArg < StringName >, sec: f64,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, f64);
            let args = (animation_from.into_arg(), animation_to.into_arg(), sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_blend_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_time(&self, animation_from: impl AsArg < StringName >, animation_to: impl AsArg < StringName >,) -> f64 {
            type CallSig < 'a0, 'a1, > = (f64, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (animation_from.into_arg(), animation_to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_blend_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_blend_time(&mut self, sec: f64,) {
            type CallSig = ((), f64);
            let args = (sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_default_blend_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_blend_time(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_default_blend_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_capture(&mut self, auto_capture: bool,) {
            type CallSig = ((), bool);
            let args = (auto_capture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_auto_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_capture(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "is_auto_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_capture_duration(&mut self, auto_capture_duration: f64,) {
            type CallSig = ((), f64);
            let args = (auto_capture_duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_auto_capture_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_capture_duration(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_auto_capture_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_capture_transition_type(&mut self, auto_capture_transition_type: crate::classes::tween::TransitionType,) {
            type CallSig = ((), crate::classes::tween::TransitionType);
            let args = (auto_capture_transition_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_auto_capture_transition_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_capture_transition_type(&self,) -> crate::classes::tween::TransitionType {
            type CallSig = (crate::classes::tween::TransitionType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_auto_capture_transition_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_capture_ease_type(&mut self, auto_capture_ease_type: crate::classes::tween::EaseType,) {
            type CallSig = ((), crate::classes::tween::EaseType);
            let args = (auto_capture_ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_auto_capture_ease_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_capture_ease_type(&self,) -> crate::classes::tween::EaseType {
            type CallSig = (crate::classes::tween::EaseType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_auto_capture_ease_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn play_full(&mut self, name: CowArg < StringName >, custom_blend: f64, custom_speed: f32, from_end: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, f64, f32, bool);
            let args = (name, custom_blend, custom_speed, from_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play(&mut self,) {
            self.play_ex() . done()
        }
        #[inline]
        pub fn play_ex < 'a > (&'a mut self,) -> ExPlay < 'a > {
            ExPlay::new(self,)
        }
        pub(crate) fn play_backwards_full(&mut self, name: CowArg < StringName >, custom_blend: f64,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, f64);
            let args = (name, custom_blend,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play_backwards", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_backwards_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play_backwards(&mut self,) {
            self.play_backwards_ex() . done()
        }
        #[inline]
        pub fn play_backwards_ex < 'a > (&'a mut self,) -> ExPlayBackwards < 'a > {
            ExPlayBackwards::new(self,)
        }
        pub(crate) fn play_with_capture_full(&mut self, name: CowArg < StringName >, duration: f64, custom_blend: f64, custom_speed: f32, from_end: bool, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, f64, f64, f32, bool, crate::classes::tween::TransitionType, crate::classes::tween::EaseType);
            let args = (name, duration, custom_blend, custom_speed, from_end, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play_with_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_with_capture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play_with_capture(&mut self,) {
            self.play_with_capture_ex() . done()
        }
        #[inline]
        pub fn play_with_capture_ex < 'a > (&'a mut self,) -> ExPlayWithCapture < 'a > {
            ExPlayWithCapture::new(self,)
        }
        pub fn pause(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn stop_full(&mut self, keep_state: bool,) {
            type CallSig = ((), bool);
            let args = (keep_state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::stop_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn stop(&mut self,) {
            self.stop_ex() . done()
        }
        #[inline]
        pub fn stop_ex < 'a > (&'a mut self,) -> ExStop < 'a > {
            ExStop::new(self,)
        }
        pub fn is_playing(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "is_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_animation(&mut self, animation: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_current_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_animation(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_current_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_assigned_animation(&mut self, animation: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_assigned_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_assigned_animation(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_assigned_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn queue(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "queue", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_queue(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_queue", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_queue(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "clear_queue", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed: f32,) {
            type CallSig = ((), f32);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playing_speed(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_playing_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autoplay(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autoplay(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_movie_quit_on_finish_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_movie_quit_on_finish_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_movie_quit_on_finish_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "is_movie_quit_on_finish_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_animation_position(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_current_animation_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_animation_length(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_current_animation_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn seek_full(&mut self, seconds: f64, update: bool, update_only: bool,) {
            type CallSig = ((), f64, bool, bool);
            let args = (seconds, update, update_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "seek", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::seek_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn seek(&mut self, seconds: f64,) {
            self.seek_ex(seconds,) . done()
        }
        #[inline]
        pub fn seek_ex < 'a > (&'a mut self, seconds: f64,) -> ExSeek < 'a > {
            ExSeek::new(self, seconds,)
        }
        pub fn set_process_callback(&mut self, mode: crate::classes::animation_player::AnimationProcessCallback,) {
            type CallSig = ((), crate::classes::animation_player::AnimationProcessCallback);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_process_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_callback(&self,) -> crate::classes::animation_player::AnimationProcessCallback {
            type CallSig = (crate::classes::animation_player::AnimationProcessCallback,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_process_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_method_call_mode(&mut self, mode: crate::classes::animation_player::AnimationMethodCallMode,) {
            type CallSig = ((), crate::classes::animation_player::AnimationMethodCallMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_method_call_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_method_call_mode(&self,) -> crate::classes::animation_player::AnimationMethodCallMode {
            type CallSig = (crate::classes::animation_player::AnimationMethodCallMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_method_call_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root(&mut self, path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_root", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationPlayer {
        type Base = crate::classes::AnimationMixer;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AnimationPlayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationPlayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationMixer > for AnimationPlayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AnimationPlayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationPlayer {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationPlayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationPlayer {
        type Target = crate::classes::AnimationMixer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationPlayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationPlayer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AnimationPlayer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationPlayer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationMixer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_ex`][super::AnimationPlayer::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationPlayer, name: CowArg < 'a, StringName >, custom_blend: f64, custom_speed: f32, from_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let custom_blend = - 1f64;
        let custom_speed = 1f32;
        let from_end = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), custom_blend: custom_blend, custom_speed: custom_speed, from_end: from_end,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, custom_speed: f32) -> Self {
        Self {
            custom_speed: custom_speed, .. self
        }
    }
    #[inline]
    pub fn from_end(self, from_end: bool) -> Self {
        Self {
            from_end: from_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, custom_blend, custom_speed, from_end,
        }
        = self;
        re_export::AnimationPlayer::play_full(surround_object, name, custom_blend, custom_speed, from_end,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_backwards_ex`][super::AnimationPlayer::play_backwards_ex]."]
#[must_use]
pub struct ExPlayBackwards < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationPlayer, name: CowArg < 'a, StringName >, custom_blend: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlayBackwards < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let custom_blend = - 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), custom_blend: custom_blend,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, custom_blend,
        }
        = self;
        re_export::AnimationPlayer::play_backwards_full(surround_object, name, custom_blend,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_with_capture_ex`][super::AnimationPlayer::play_with_capture_ex]."]
#[must_use]
pub struct ExPlayWithCapture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationPlayer, name: CowArg < 'a, StringName >, duration: f64, custom_blend: f64, custom_speed: f32, from_end: bool, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlayWithCapture < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let duration = - 1f64;
        let custom_blend = - 1f64;
        let custom_speed = 1f32;
        let from_end = false;
        let trans_type = crate::obj::EngineEnum::from_ord(0);
        let ease_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), duration: duration, custom_blend: custom_blend, custom_speed: custom_speed, from_end: from_end, trans_type: trans_type, ease_type: ease_type,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn duration(self, duration: f64) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, custom_speed: f32) -> Self {
        Self {
            custom_speed: custom_speed, .. self
        }
    }
    #[inline]
    pub fn from_end(self, from_end: bool) -> Self {
        Self {
            from_end: from_end, .. self
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
            _phantom, surround_object, name, duration, custom_blend, custom_speed, from_end, trans_type, ease_type,
        }
        = self;
        re_export::AnimationPlayer::play_with_capture_full(surround_object, name, duration, custom_blend, custom_speed, from_end, trans_type, ease_type,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::stop_ex`][super::AnimationPlayer::stop_ex]."]
#[must_use]
pub struct ExStop < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationPlayer, keep_state: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStop < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer,) -> Self {
        let keep_state = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, keep_state: keep_state,
        }
    }
    #[inline]
    pub fn keep_state(self, keep_state: bool) -> Self {
        Self {
            keep_state: keep_state, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, keep_state,
        }
        = self;
        re_export::AnimationPlayer::stop_full(surround_object, keep_state,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::seek_ex`][super::AnimationPlayer::seek_ex]."]
#[must_use]
pub struct ExSeek < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationPlayer, seconds: f64, update: bool, update_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSeek < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer, seconds: f64,) -> Self {
        let update = false;
        let update_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, seconds: seconds, update: update, update_only: update_only,
        }
    }
    #[inline]
    pub fn update(self, update: bool) -> Self {
        Self {
            update: update, .. self
        }
    }
    #[inline]
    pub fn update_only(self, update_only: bool) -> Self {
        Self {
            update_only: update_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, seconds, update, update_only,
        }
        = self;
        re_export::AnimationPlayer::seek_full(surround_object, seconds, update, update_only,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationProcessCallback {
    ord: i32
}
impl AnimationProcessCallback {
    #[doc(alias = "ANIMATION_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_PHYSICS`"]
    pub const PHYSICS: AnimationProcessCallback = AnimationProcessCallback {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_IDLE`"]
    pub const IDLE: AnimationProcessCallback = AnimationProcessCallback {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_PROCESS_MANUAL")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_MANUAL`"]
    pub const MANUAL: AnimationProcessCallback = AnimationProcessCallback {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationProcessCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationProcessCallback") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationProcessCallback {
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
            Self::PHYSICS => "ANIMATION_PROCESS_PHYSICS", Self::IDLE => "ANIMATION_PROCESS_IDLE", Self::MANUAL => "ANIMATION_PROCESS_MANUAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AnimationProcessCallback {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationProcessCallback {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationMethodCallMode {
    ord: i32
}
impl AnimationMethodCallMode {
    #[doc(alias = "ANIMATION_METHOD_CALL_DEFERRED")]
    #[doc = "Godot enumerator name: `ANIMATION_METHOD_CALL_DEFERRED`"]
    pub const DEFERRED: AnimationMethodCallMode = AnimationMethodCallMode {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_METHOD_CALL_IMMEDIATE")]
    #[doc = "Godot enumerator name: `ANIMATION_METHOD_CALL_IMMEDIATE`"]
    pub const IMMEDIATE: AnimationMethodCallMode = AnimationMethodCallMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AnimationMethodCallMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationMethodCallMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationMethodCallMode {
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
            Self::DEFERRED => "ANIMATION_METHOD_CALL_DEFERRED", Self::IMMEDIATE => "ANIMATION_METHOD_CALL_IMMEDIATE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AnimationMethodCallMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationMethodCallMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationMethodCallMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}