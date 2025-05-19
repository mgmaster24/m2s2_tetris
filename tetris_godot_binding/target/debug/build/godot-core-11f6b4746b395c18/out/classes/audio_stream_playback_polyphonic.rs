#![doc = "Sidecar module for class [`AudioStreamPlaybackPolyphonic`][crate::classes::AudioStreamPlaybackPolyphonic].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlaybackPolyphonic` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackpolyphonic.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlaybackPolyphonic.`\n\nInherits [`AudioStreamPlayback`][crate::classes::AudioStreamPlayback].\n\nRelated symbols:\n\n* [`audio_stream_playback_polyphonic`][crate::classes::audio_stream_playback_polyphonic]: sidecar module with related enum/flag types\n* [`IAudioStreamPlaybackPolyphonic`][crate::classes::IAudioStreamPlaybackPolyphonic]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamPlaybackPolyphonic`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackpolyphonic.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AudioStreamPlaybackPolyphonic>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlaybackPolyphonic {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamPlaybackPolyphonic`][crate::classes::AudioStreamPlaybackPolyphonic].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamPlaybackPolyphonic` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackpolyphonic.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlaybackPolyphonic: crate::obj::GodotClass < Base = AudioStreamPlaybackPolyphonic > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn start(&mut self, from_pos: f64,) {
            unimplemented !()
        }
        fn stop(&mut self,) {
            unimplemented !()
        }
        fn is_playing(&self,) -> bool {
            unimplemented !()
        }
        fn get_loop_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_playback_position(&self,) -> f64 {
            unimplemented !()
        }
        fn seek(&mut self, position: f64,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn mix(&mut self, buffer: * mut AudioFrame, rate_scale: f32, frames: i32,) -> i32;
        fn tag_used_streams(&mut self,) {
            unimplemented !()
        }
        fn set_parameter(&mut self, name: StringName, value: Variant,) {
            unimplemented !()
        }
        fn get_parameter(&self, name: StringName,) -> Variant {
            unimplemented !()
        }
    }
    impl AudioStreamPlaybackPolyphonic {
        pub(crate) fn play_stream_full(&mut self, stream: ObjectArg < crate::classes::AudioStream >, from_offset: f32, volume_db: f32, pitch_scale: f32, playback_type: crate::classes::audio_server::PlaybackType, bus: CowArg < StringName >,) -> i64 {
            type CallSig < 'a0, > = (i64, ObjectArg < crate::classes::AudioStream >, f32, f32, f32, crate::classes::audio_server::PlaybackType, CowArg < 'a0, StringName >);
            let args = (stream, from_offset, volume_db, pitch_scale, playback_type, bus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "play_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_stream_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play_stream(&mut self, stream: impl AsObjectArg < crate::classes::AudioStream >,) -> i64 {
            self.play_stream_ex(stream,) . done()
        }
        #[inline]
        pub fn play_stream_ex < 'a > (&'a mut self, stream: impl AsObjectArg < crate::classes::AudioStream >,) -> ExPlayStream < 'a > {
            ExPlayStream::new(self, stream,)
        }
        pub fn set_stream_volume(&mut self, stream: i64, volume_db: f32,) {
            type CallSig = ((), i64, f32);
            let args = (stream, volume_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "set_stream_volume", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream_pitch_scale(&mut self, stream: i64, pitch_scale: f32,) {
            type CallSig = ((), i64, f32);
            let args = (stream, pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "set_stream_pitch_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_stream_playing(&self, stream: i64,) -> bool {
            type CallSig = (bool, i64);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "is_stream_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop_stream(&mut self, stream: i64,) {
            type CallSig = ((), i64);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "stop_stream", self.object_ptr, self.__checked_id(), args,)
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
        pub const INVALID_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for AudioStreamPlaybackPolyphonic {
        type Base = crate::classes::AudioStreamPlayback;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioStreamPlaybackPolyphonic"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlaybackPolyphonic {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStreamPlayback > for AudioStreamPlaybackPolyphonic {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamPlaybackPolyphonic {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamPlaybackPolyphonic {
        
    }
    impl std::ops::Deref for AudioStreamPlaybackPolyphonic {
        type Target = crate::classes::AudioStreamPlayback;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlaybackPolyphonic {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamPlaybackPolyphonic`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioStreamPlaybackPolyphonic {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamPlaybackPolyphonic > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamPlayback > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioStreamPlaybackPolyphonic::play_stream_ex`][super::AudioStreamPlaybackPolyphonic::play_stream_ex]."]
#[must_use]
pub struct ExPlayStream < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamPlaybackPolyphonic, stream: ObjectCow < crate::classes::AudioStream >, from_offset: f32, volume_db: f32, pitch_scale: f32, playback_type: crate::classes::audio_server::PlaybackType, bus: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlayStream < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlaybackPolyphonic, stream: impl AsObjectArg < crate::classes::AudioStream >,) -> Self {
        let from_offset = 0f32;
        let volume_db = 0f32;
        let pitch_scale = 1f32;
        let playback_type = crate::obj::EngineEnum::from_ord(0);
        let bus = StringName::from("Master");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, stream: stream.consume_arg(), from_offset: from_offset, volume_db: volume_db, pitch_scale: pitch_scale, playback_type: playback_type, bus: CowArg::Owned(bus),
        }
    }
    #[inline]
    pub fn from_offset(self, from_offset: f32) -> Self {
        Self {
            from_offset: from_offset, .. self
        }
    }
    #[inline]
    pub fn volume_db(self, volume_db: f32) -> Self {
        Self {
            volume_db: volume_db, .. self
        }
    }
    #[inline]
    pub fn pitch_scale(self, pitch_scale: f32) -> Self {
        Self {
            pitch_scale: pitch_scale, .. self
        }
    }
    #[inline]
    pub fn playback_type(self, playback_type: crate::classes::audio_server::PlaybackType) -> Self {
        Self {
            playback_type: playback_type, .. self
        }
    }
    #[inline]
    pub fn bus(self, bus: impl AsArg < StringName > + 'a) -> Self {
        Self {
            bus: bus.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, stream, from_offset, volume_db, pitch_scale, playback_type, bus,
        }
        = self;
        re_export::AudioStreamPlaybackPolyphonic::play_stream_full(surround_object, stream.cow_as_object_arg(), from_offset, volume_db, pitch_scale, playback_type, bus,)
    }
}