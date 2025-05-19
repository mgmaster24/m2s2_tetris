#![doc = "Sidecar module for class [`AudioStreamPlayer3D`][crate::classes::AudioStreamPlayer3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlayer3D` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlayer3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`audio_stream_player_3d`][crate::classes::audio_stream_player_3d]: sidecar module with related enum/flag types\n* [`IAudioStreamPlayer3D`][crate::classes::IAudioStreamPlayer3D]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamPlayer3D`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AudioStreamPlayer3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlayer3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamPlayer3D`][crate::classes::AudioStreamPlayer3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamPlayer3D` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlayer3D: crate::obj::GodotClass < Base = AudioStreamPlayer3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioStreamPlayer3D {
        pub fn set_stream(&mut self, stream: impl AsObjectArg < crate::classes::AudioStream >,) {
            type CallSig = ((), ObjectArg < crate::classes::AudioStream >);
            let args = (stream.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::classes::AudioStream > > {
            type CallSig = (Option < Gd < crate::classes::AudioStream > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volume_db(&mut self, volume_db: f32,) {
            type CallSig = ((), f32);
            let args = (volume_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volume_db(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unit_size(&mut self, unit_size: f32,) {
            type CallSig = ((), f32);
            let args = (unit_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_unit_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unit_size(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_unit_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_db(&mut self, max_db: f32,) {
            type CallSig = ((), f32);
            let args = (max_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_max_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_db(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_max_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pitch_scale(&mut self, pitch_scale: f32,) {
            type CallSig = ((), f32);
            let args = (pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_pitch_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pitch_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_pitch_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn play_full(&mut self, from_position: f32,) {
            type CallSig = ((), f32);
            let args = (from_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "play", self.object_ptr, self.__checked_id(), args,)
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
        pub fn seek(&mut self, to_position: f32,) {
            type CallSig = ((), f32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "seek", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "is_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playback_position(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_playback_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus(&mut self, bus: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (bus.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autoplay(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_autoplay_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "is_autoplay_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_distance(&mut self, meters: f32,) {
            type CallSig = ((), f32);
            let args = (meters,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_distance(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_area_mask(&mut self, mask: u32,) {
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_area_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_area_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_area_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_angle(&mut self, degrees: f32,) {
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_emission_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_angle(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_emission_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_angle_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_emission_angle_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emission_angle_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "is_emission_angle_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_angle_filter_attenuation_db(&mut self, db: f32,) {
            type CallSig = ((), f32);
            let args = (db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_emission_angle_filter_attenuation_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_angle_filter_attenuation_db(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_emission_angle_filter_attenuation_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attenuation_filter_cutoff_hz(&mut self, degrees: f32,) {
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_attenuation_filter_cutoff_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attenuation_filter_cutoff_hz(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_attenuation_filter_cutoff_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attenuation_filter_db(&mut self, db: f32,) {
            type CallSig = ((), f32);
            let args = (db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_attenuation_filter_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attenuation_filter_db(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_attenuation_filter_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attenuation_model(&mut self, model: crate::classes::audio_stream_player_3d::AttenuationModel,) {
            type CallSig = ((), crate::classes::audio_stream_player_3d::AttenuationModel);
            let args = (model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_attenuation_model", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attenuation_model(&self,) -> crate::classes::audio_stream_player_3d::AttenuationModel {
            type CallSig = (crate::classes::audio_stream_player_3d::AttenuationModel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_attenuation_model", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_doppler_tracking(&mut self, mode: crate::classes::audio_stream_player_3d::DopplerTracking,) {
            type CallSig = ((), crate::classes::audio_stream_player_3d::DopplerTracking);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_doppler_tracking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_doppler_tracking(&self,) -> crate::classes::audio_stream_player_3d::DopplerTracking {
            type CallSig = (crate::classes::audio_stream_player_3d::DopplerTracking,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_doppler_tracking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream_paused(&mut self, pause: bool,) {
            type CallSig = ((), bool);
            let args = (pause,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_stream_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_paused(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_stream_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_polyphony(&mut self, max_polyphony: i32,) {
            type CallSig = ((), i32);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_polyphony(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_panning_strength(&mut self, panning_strength: f32,) {
            type CallSig = ((), f32);
            let args = (panning_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_panning_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_panning_strength(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_panning_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_stream_playback(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "has_stream_playback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_playback(&mut self,) -> Option < Gd < crate::classes::AudioStreamPlayback > > {
            type CallSig = (Option < Gd < crate::classes::AudioStreamPlayback > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_stream_playback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_playback_type(&mut self, playback_type: crate::classes::audio_server::PlaybackType,) {
            type CallSig = ((), crate::classes::audio_server::PlaybackType);
            let args = (playback_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_playback_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playback_type(&self,) -> crate::classes::audio_server::PlaybackType {
            type CallSig = (crate::classes::audio_server::PlaybackType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_playback_type", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioStreamPlayer3D {
        type Base = crate::classes::Node3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioStreamPlayer3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlayer3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for AudioStreamPlayer3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AudioStreamPlayer3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamPlayer3D {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamPlayer3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamPlayer3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlayer3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamPlayer3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioStreamPlayer3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamPlayer3D > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamPlayer3D::play_ex`][super::AudioStreamPlayer3D::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamPlayer3D, from_position: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlayer3D,) -> Self {
        let from_position = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_position: from_position,
        }
    }
    #[inline]
    pub fn from_position(self, from_position: f32) -> Self {
        Self {
            from_position: from_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_position,
        }
        = self;
        re_export::AudioStreamPlayer3D::play_full(surround_object, from_position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AttenuationModel {
    ord: i32
}
impl AttenuationModel {
    #[doc(alias = "ATTENUATION_INVERSE_DISTANCE")]
    #[doc = "Godot enumerator name: `ATTENUATION_INVERSE_DISTANCE`"]
    pub const INVERSE_DISTANCE: AttenuationModel = AttenuationModel {
        ord: 0i32
    };
    #[doc(alias = "ATTENUATION_INVERSE_SQUARE_DISTANCE")]
    #[doc = "Godot enumerator name: `ATTENUATION_INVERSE_SQUARE_DISTANCE`"]
    pub const INVERSE_SQUARE_DISTANCE: AttenuationModel = AttenuationModel {
        ord: 1i32
    };
    #[doc(alias = "ATTENUATION_LOGARITHMIC")]
    #[doc = "Godot enumerator name: `ATTENUATION_LOGARITHMIC`"]
    pub const LOGARITHMIC: AttenuationModel = AttenuationModel {
        ord: 2i32
    };
    #[doc(alias = "ATTENUATION_DISABLED")]
    #[doc = "Godot enumerator name: `ATTENUATION_DISABLED`"]
    pub const DISABLED: AttenuationModel = AttenuationModel {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AttenuationModel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AttenuationModel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AttenuationModel {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::INVERSE_DISTANCE => "INVERSE_DISTANCE", Self::INVERSE_SQUARE_DISTANCE => "INVERSE_SQUARE_DISTANCE", Self::LOGARITHMIC => "LOGARITHMIC", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INVERSE_DISTANCE => "ATTENUATION_INVERSE_DISTANCE", Self::INVERSE_SQUARE_DISTANCE => "ATTENUATION_INVERSE_SQUARE_DISTANCE", Self::LOGARITHMIC => "ATTENUATION_LOGARITHMIC", Self::DISABLED => "ATTENUATION_DISABLED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AttenuationModel {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AttenuationModel {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AttenuationModel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DopplerTracking {
    ord: i32
}
impl DopplerTracking {
    #[doc(alias = "DOPPLER_TRACKING_DISABLED")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_DISABLED`"]
    pub const DISABLED: DopplerTracking = DopplerTracking {
        ord: 0i32
    };
    #[doc(alias = "DOPPLER_TRACKING_IDLE_STEP")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_IDLE_STEP`"]
    pub const IDLE_STEP: DopplerTracking = DopplerTracking {
        ord: 1i32
    };
    #[doc(alias = "DOPPLER_TRACKING_PHYSICS_STEP")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_PHYSICS_STEP`"]
    pub const PHYSICS_STEP: DopplerTracking = DopplerTracking {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DopplerTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DopplerTracking") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DopplerTracking {
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
            Self::DISABLED => "DISABLED", Self::IDLE_STEP => "IDLE_STEP", Self::PHYSICS_STEP => "PHYSICS_STEP", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DOPPLER_TRACKING_DISABLED", Self::IDLE_STEP => "DOPPLER_TRACKING_IDLE_STEP", Self::PHYSICS_STEP => "DOPPLER_TRACKING_PHYSICS_STEP", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DopplerTracking {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DopplerTracking {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DopplerTracking {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}