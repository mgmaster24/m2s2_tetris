#![doc = "Sidecar module for class [`AudioServer`][crate::classes::AudioServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioServer` enums](https://docs.godotengine.org/en/stable/classes/class_audioserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`audio_server`][crate::classes::audio_server]: sidecar module with related enum/flag types\n* [`IAudioServer`][crate::classes::IAudioServer]: virtual methods\n\n\nSee also [Godot docs for `AudioServer`](https://docs.godotengine.org/en/stable/classes/class_audioserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`AudioServer::singleton()`][AudioServer::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioServer`][crate::classes::AudioServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioServer` methods](https://docs.godotengine.org/en/stable/classes/class_audioserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioServer: crate::obj::GodotClass < Base = AudioServer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"AudioServer");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn set_bus_count(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(0usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_bus(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(2usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "remove_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_bus_full(&mut self, at_position: i32,) {
            type CallSig = ((), i32);
            let args = (at_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(3usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "add_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_bus_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_bus(&mut self,) {
            self.add_bus_ex() . done()
        }
        #[inline]
        pub fn add_bus_ex < 'a > (&'a mut self,) -> ExAddBus < 'a > {
            ExAddBus::new(self,)
        }
        pub fn move_bus(&mut self, index: i32, to_index: i32,) {
            type CallSig = ((), i32, i32);
            let args = (index, to_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(4usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "move_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_name(&mut self, bus_idx: i32, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (bus_idx, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(5usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_name(&self, bus_idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(6usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_index(&self, bus_name: impl AsArg < StringName >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, StringName >);
            let args = (bus_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(7usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_channels(&self, bus_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(8usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_channels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_volume_db(&mut self, bus_idx: i32, volume_db: f32,) {
            type CallSig = ((), i32, f32);
            let args = (bus_idx, volume_db,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(9usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_volume_db(&self, bus_idx: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(10usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_send(&mut self, bus_idx: i32, send: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (bus_idx, send.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(11usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_send", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_send(&self, bus_idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(12usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_send", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_solo(&mut self, bus_idx: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(13usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_solo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bus_solo(&self, bus_idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(14usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "is_bus_solo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_mute(&mut self, bus_idx: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(15usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_mute", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bus_mute(&self, bus_idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(16usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "is_bus_mute", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_bypass_effects(&mut self, bus_idx: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(17usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_bypass_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bus_bypassing_effects(&self, bus_idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(18usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "is_bus_bypassing_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_bus_effect_full(&mut self, bus_idx: i32, effect: ObjectArg < crate::classes::AudioEffect >, at_position: i32,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::AudioEffect >, i32);
            let args = (bus_idx, effect, at_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(19usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "add_bus_effect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_bus_effect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_bus_effect(&mut self, bus_idx: i32, effect: impl AsObjectArg < crate::classes::AudioEffect >,) {
            self.add_bus_effect_ex(bus_idx, effect,) . done()
        }
        #[inline]
        pub fn add_bus_effect_ex < 'a > (&'a mut self, bus_idx: i32, effect: impl AsObjectArg < crate::classes::AudioEffect >,) -> ExAddBusEffect < 'a > {
            ExAddBusEffect::new(self, bus_idx, effect,)
        }
        pub fn remove_bus_effect(&mut self, bus_idx: i32, effect_idx: i32,) {
            type CallSig = ((), i32, i32);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(20usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "remove_bus_effect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_effect_count(&mut self, bus_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(21usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_effect_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_effect(&mut self, bus_idx: i32, effect_idx: i32,) -> Option < Gd < crate::classes::AudioEffect > > {
            type CallSig = (Option < Gd < crate::classes::AudioEffect > >, i32, i32);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(22usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_effect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_bus_effect_instance_full(&mut self, bus_idx: i32, effect_idx: i32, channel: i32,) -> Option < Gd < crate::classes::AudioEffectInstance > > {
            type CallSig = (Option < Gd < crate::classes::AudioEffectInstance > >, i32, i32, i32);
            let args = (bus_idx, effect_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(23usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_effect_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_bus_effect_instance_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_bus_effect_instance(&mut self, bus_idx: i32, effect_idx: i32,) -> Option < Gd < crate::classes::AudioEffectInstance > > {
            self.get_bus_effect_instance_ex(bus_idx, effect_idx,) . done()
        }
        #[inline]
        pub fn get_bus_effect_instance_ex < 'a > (&'a mut self, bus_idx: i32, effect_idx: i32,) -> ExGetBusEffectInstance < 'a > {
            ExGetBusEffectInstance::new(self, bus_idx, effect_idx,)
        }
        pub fn swap_bus_effects(&mut self, bus_idx: i32, effect_idx: i32, by_effect_idx: i32,) {
            type CallSig = ((), i32, i32, i32);
            let args = (bus_idx, effect_idx, by_effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(24usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "swap_bus_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_effect_enabled(&mut self, bus_idx: i32, effect_idx: i32, enabled: bool,) {
            type CallSig = ((), i32, i32, bool);
            let args = (bus_idx, effect_idx, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(25usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_effect_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bus_effect_enabled(&self, bus_idx: i32, effect_idx: i32,) -> bool {
            type CallSig = (bool, i32, i32);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(26usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "is_bus_effect_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_peak_volume_left_db(&self, bus_idx: i32, channel: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (bus_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(27usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_peak_volume_left_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_peak_volume_right_db(&self, bus_idx: i32, channel: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (bus_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(28usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_peak_volume_right_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_playback_speed_scale(&mut self, scale: f32,) {
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(29usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_playback_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playback_speed_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(30usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_playback_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lock(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(31usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "lock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unlock(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(32usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "unlock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speaker_mode(&self,) -> crate::classes::audio_server::SpeakerMode {
            type CallSig = (crate::classes::audio_server::SpeakerMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(33usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_speaker_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mix_rate(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(34usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_mix_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_device_list(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(35usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_output_device_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_device(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(36usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_output_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_output_device(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(37usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_output_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_to_next_mix(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(38usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_time_to_next_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_since_last_mix(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(39usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_time_since_last_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_latency(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(40usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_output_latency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_device_list(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(41usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_input_device_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_device(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(42usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "get_input_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_device(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(43usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_input_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_layout(&mut self, bus_layout: impl AsObjectArg < crate::classes::AudioBusLayout >,) {
            type CallSig = ((), ObjectArg < crate::classes::AudioBusLayout >);
            let args = (bus_layout.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(44usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_bus_layout(&self,) -> Option < Gd < crate::classes::AudioBusLayout > > {
            type CallSig = (Option < Gd < crate::classes::AudioBusLayout > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(45usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "generate_bus_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_tagging_used_audio_streams(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(46usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "set_enable_tagging_used_audio_streams", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_stream_registered_as_sample(&mut self, stream: impl AsObjectArg < crate::classes::AudioStream >,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::AudioStream >);
            let args = (stream.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(47usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "is_stream_registered_as_sample", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_stream_as_sample(&mut self, stream: impl AsObjectArg < crate::classes::AudioStream >,) {
            type CallSig = ((), ObjectArg < crate::classes::AudioStream >);
            let args = (stream.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(48usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioServer", "register_stream_as_sample", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioServer {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for AudioServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioServer {
        
    }
    impl std::ops::Deref for AudioServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioServer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioServer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioServer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioServer::add_bus_ex`][super::AudioServer::add_bus_ex]."]
#[must_use]
pub struct ExAddBus < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioServer, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddBus < 'a > {
    fn new(surround_object: &'a mut re_export::AudioServer,) -> Self {
        let at_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, at_position: at_position,
        }
    }
    #[inline]
    pub fn at_position(self, at_position: i32) -> Self {
        Self {
            at_position: at_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, at_position,
        }
        = self;
        re_export::AudioServer::add_bus_full(surround_object, at_position,)
    }
}
#[doc = "Default-param extender for [`AudioServer::add_bus_effect_ex`][super::AudioServer::add_bus_effect_ex]."]
#[must_use]
pub struct ExAddBusEffect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioServer, bus_idx: i32, effect: ObjectCow < crate::classes::AudioEffect >, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddBusEffect < 'a > {
    fn new(surround_object: &'a mut re_export::AudioServer, bus_idx: i32, effect: impl AsObjectArg < crate::classes::AudioEffect >,) -> Self {
        let at_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bus_idx: bus_idx, effect: effect.consume_arg(), at_position: at_position,
        }
    }
    #[inline]
    pub fn at_position(self, at_position: i32) -> Self {
        Self {
            at_position: at_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, bus_idx, effect, at_position,
        }
        = self;
        re_export::AudioServer::add_bus_effect_full(surround_object, bus_idx, effect.cow_as_object_arg(), at_position,)
    }
}
#[doc = "Default-param extender for [`AudioServer::get_bus_effect_instance_ex`][super::AudioServer::get_bus_effect_instance_ex]."]
#[must_use]
pub struct ExGetBusEffectInstance < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioServer, bus_idx: i32, effect_idx: i32, channel: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetBusEffectInstance < 'a > {
    fn new(surround_object: &'a mut re_export::AudioServer, bus_idx: i32, effect_idx: i32,) -> Self {
        let channel = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bus_idx: bus_idx, effect_idx: effect_idx, channel: channel,
        }
    }
    #[inline]
    pub fn channel(self, channel: i32) -> Self {
        Self {
            channel: channel, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::AudioEffectInstance > > {
        let Self {
            _phantom, surround_object, bus_idx, effect_idx, channel,
        }
        = self;
        re_export::AudioServer::get_bus_effect_instance_full(surround_object, bus_idx, effect_idx, channel,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpeakerMode {
    ord: i32
}
impl SpeakerMode {
    #[doc(alias = "SPEAKER_MODE_STEREO")]
    #[doc = "Godot enumerator name: `SPEAKER_MODE_STEREO`"]
    pub const STEREO: SpeakerMode = SpeakerMode {
        ord: 0i32
    };
    #[doc(alias = "SPEAKER_SURROUND_31")]
    #[doc = "Godot enumerator name: `SPEAKER_SURROUND_31`"]
    pub const SURROUND_31: SpeakerMode = SpeakerMode {
        ord: 1i32
    };
    #[doc(alias = "SPEAKER_SURROUND_51")]
    #[doc = "Godot enumerator name: `SPEAKER_SURROUND_51`"]
    pub const SURROUND_51: SpeakerMode = SpeakerMode {
        ord: 2i32
    };
    #[doc(alias = "SPEAKER_SURROUND_71")]
    #[doc = "Godot enumerator name: `SPEAKER_SURROUND_71`"]
    pub const SURROUND_71: SpeakerMode = SpeakerMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for SpeakerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpeakerMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpeakerMode {
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
            Self::STEREO => "STEREO", Self::SURROUND_31 => "SURROUND_31", Self::SURROUND_51 => "SURROUND_51", Self::SURROUND_71 => "SURROUND_71", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STEREO => "SPEAKER_MODE_STEREO", Self::SURROUND_31 => "SPEAKER_SURROUND_31", Self::SURROUND_51 => "SPEAKER_SURROUND_51", Self::SURROUND_71 => "SPEAKER_SURROUND_71", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SpeakerMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpeakerMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpeakerMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlaybackType {
    ord: i32
}
impl PlaybackType {
    #[doc(alias = "PLAYBACK_TYPE_DEFAULT")]
    #[doc = "Godot enumerator name: `PLAYBACK_TYPE_DEFAULT`"]
    pub const DEFAULT: PlaybackType = PlaybackType {
        ord: 0i32
    };
    #[doc(alias = "PLAYBACK_TYPE_STREAM")]
    #[doc = "Godot enumerator name: `PLAYBACK_TYPE_STREAM`"]
    pub const STREAM: PlaybackType = PlaybackType {
        ord: 1i32
    };
    #[doc(alias = "PLAYBACK_TYPE_SAMPLE")]
    #[doc = "Godot enumerator name: `PLAYBACK_TYPE_SAMPLE`"]
    pub const SAMPLE: PlaybackType = PlaybackType {
        ord: 2i32
    };
    #[doc(alias = "PLAYBACK_TYPE_MAX")]
    #[doc = "Godot enumerator name: `PLAYBACK_TYPE_MAX`"]
    pub const MAX: PlaybackType = PlaybackType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for PlaybackType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlaybackType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlaybackType {
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
            Self::DEFAULT => "DEFAULT", Self::STREAM => "STREAM", Self::SAMPLE => "SAMPLE", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "PLAYBACK_TYPE_DEFAULT", Self::STREAM => "PLAYBACK_TYPE_STREAM", Self::SAMPLE => "PLAYBACK_TYPE_SAMPLE", Self::MAX => "PLAYBACK_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for PlaybackType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for PlaybackType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PlaybackType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlaybackType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}