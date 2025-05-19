#![doc = "Sidecar module for class [`Time`][crate::classes::Time].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Time` enums](https://docs.godotengine.org/en/stable/classes/class_time.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Time.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`time`][crate::classes::time]: sidecar module with related enum/flag types\n* [`ITime`][crate::classes::ITime]: virtual methods\n\n\nSee also [Godot docs for `Time`](https://docs.godotengine.org/en/stable/classes/class_time.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Time::singleton()`][Time::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Time {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Time`][crate::classes::Time].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Time` methods](https://docs.godotengine.org/en/stable/classes/class_time.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITime: crate::obj::GodotClass < Base = Time > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Time {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"Time");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_datetime_dict_from_unix_time(&self, unix_time_val: i64,) -> Dictionary {
            type CallSig = (Dictionary, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_date_dict_from_unix_time(&self, unix_time_val: i64,) -> Dictionary {
            type CallSig = (Dictionary, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_date_dict_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_dict_from_unix_time(&self, unix_time_val: i64,) -> Dictionary {
            type CallSig = (Dictionary, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_time_dict_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_datetime_string_from_unix_time_full(&self, unix_time_val: i64, use_space: bool,) -> GString {
            type CallSig = (GString, i64, bool);
            let args = (unix_time_val, use_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_datetime_string_from_unix_time_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_datetime_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            self.get_datetime_string_from_unix_time_ex(unix_time_val,) . done()
        }
        #[inline]
        pub fn get_datetime_string_from_unix_time_ex < 'a > (&'a self, unix_time_val: i64,) -> ExGetDatetimeStringFromUnixTime < 'a > {
            ExGetDatetimeStringFromUnixTime::new(self, unix_time_val,)
        }
        pub fn get_date_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            type CallSig = (GString, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_date_string_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            type CallSig = (GString, i64);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_time_string_from_unix_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_datetime_dict_from_datetime_string(&self, datetime: impl AsArg < GString >, weekday: bool,) -> Dictionary {
            type CallSig < 'a0, > = (Dictionary, CowArg < 'a0, GString >, bool);
            let args = (datetime.into_arg(), weekday,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_datetime_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_datetime_string_from_datetime_dict(&self, datetime: &Dictionary, use_space: bool,) -> GString {
            type CallSig < 'a0, > = (GString, RefArg < 'a0, Dictionary >, bool);
            let args = (RefArg::new(datetime), use_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_datetime_dict", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unix_time_from_datetime_dict(&self, datetime: &Dictionary,) -> i64 {
            type CallSig < 'a0, > = (i64, RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(datetime),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_datetime_dict", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unix_time_from_datetime_string(&self, datetime: impl AsArg < GString >,) -> i64 {
            type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
            let args = (datetime.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_datetime_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset_string_from_offset_minutes(&self, offset_minutes: i64,) -> GString {
            type CallSig = (GString, i64);
            let args = (offset_minutes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_offset_string_from_offset_minutes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_datetime_dict_from_system_full(&self, utc: bool,) -> Dictionary {
            type CallSig = (Dictionary, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_datetime_dict_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_datetime_dict_from_system(&self,) -> Dictionary {
            self.get_datetime_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_datetime_dict_from_system_ex < 'a > (&'a self,) -> ExGetDatetimeDictFromSystem < 'a > {
            ExGetDatetimeDictFromSystem::new(self,)
        }
        pub(crate) fn get_date_dict_from_system_full(&self, utc: bool,) -> Dictionary {
            type CallSig = (Dictionary, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_date_dict_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_date_dict_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_date_dict_from_system(&self,) -> Dictionary {
            self.get_date_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_date_dict_from_system_ex < 'a > (&'a self,) -> ExGetDateDictFromSystem < 'a > {
            ExGetDateDictFromSystem::new(self,)
        }
        pub(crate) fn get_time_dict_from_system_full(&self, utc: bool,) -> Dictionary {
            type CallSig = (Dictionary, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_time_dict_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_time_dict_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_time_dict_from_system(&self,) -> Dictionary {
            self.get_time_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_time_dict_from_system_ex < 'a > (&'a self,) -> ExGetTimeDictFromSystem < 'a > {
            ExGetTimeDictFromSystem::new(self,)
        }
        pub(crate) fn get_datetime_string_from_system_full(&self, utc: bool, use_space: bool,) -> GString {
            type CallSig = (GString, bool, bool);
            let args = (utc, use_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_datetime_string_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_datetime_string_from_system(&self,) -> GString {
            self.get_datetime_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_datetime_string_from_system_ex < 'a > (&'a self,) -> ExGetDatetimeStringFromSystem < 'a > {
            ExGetDatetimeStringFromSystem::new(self,)
        }
        pub(crate) fn get_date_string_from_system_full(&self, utc: bool,) -> GString {
            type CallSig = (GString, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_date_string_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_date_string_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_date_string_from_system(&self,) -> GString {
            self.get_date_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_date_string_from_system_ex < 'a > (&'a self,) -> ExGetDateStringFromSystem < 'a > {
            ExGetDateStringFromSystem::new(self,)
        }
        pub(crate) fn get_time_string_from_system_full(&self, utc: bool,) -> GString {
            type CallSig = (GString, bool);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_time_string_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_time_string_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_time_string_from_system(&self,) -> GString {
            self.get_time_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_time_string_from_system_ex < 'a > (&'a self,) -> ExGetTimeStringFromSystem < 'a > {
            ExGetTimeStringFromSystem::new(self,)
        }
        pub fn get_time_zone_from_system(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_time_zone_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unix_time_from_system(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ticks_msec(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_ticks_msec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ticks_usec(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Time", "get_ticks_usec", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Time {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Time"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Time {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Time {
        
    }
    impl std::ops::Deref for Time {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Time {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Time`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Time {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Time > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_string_from_unix_time_ex`][super::Time::get_datetime_string_from_unix_time_ex]."]
#[must_use]
pub struct ExGetDatetimeStringFromUnixTime < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, unix_time_val: i64, use_space: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeStringFromUnixTime < 'a > {
    fn new(surround_object: &'a re_export::Time, unix_time_val: i64,) -> Self {
        let use_space = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, unix_time_val: unix_time_val, use_space: use_space,
        }
    }
    #[inline]
    pub fn use_space(self, use_space: bool) -> Self {
        Self {
            use_space: use_space, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, unix_time_val, use_space,
        }
        = self;
        re_export::Time::get_datetime_string_from_unix_time_full(surround_object, unix_time_val, use_space,)
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_dict_from_system_ex`][super::Time::get_datetime_dict_from_system_ex]."]
#[must_use]
pub struct ExGetDatetimeDictFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_datetime_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_date_dict_from_system_ex`][super::Time::get_date_dict_from_system_ex]."]
#[must_use]
pub struct ExGetDateDictFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDateDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_date_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_time_dict_from_system_ex`][super::Time::get_time_dict_from_system_ex]."]
#[must_use]
pub struct ExGetTimeDictFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTimeDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_time_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_string_from_system_ex`][super::Time::get_datetime_string_from_system_ex]."]
#[must_use]
pub struct ExGetDatetimeStringFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool, use_space: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        let use_space = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc, use_space: use_space,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn use_space(self, use_space: bool) -> Self {
        Self {
            use_space: use_space, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc, use_space,
        }
        = self;
        re_export::Time::get_datetime_string_from_system_full(surround_object, utc, use_space,)
    }
}
#[doc = "Default-param extender for [`Time::get_date_string_from_system_ex`][super::Time::get_date_string_from_system_ex]."]
#[must_use]
pub struct ExGetDateStringFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDateStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_date_string_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_time_string_from_system_ex`][super::Time::get_time_string_from_system_ex]."]
#[must_use]
pub struct ExGetTimeStringFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTimeStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_time_string_from_system_full(surround_object, utc,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Month {
    ord: i32
}
impl Month {
    #[doc(alias = "MONTH_JANUARY")]
    #[doc = "Godot enumerator name: `MONTH_JANUARY`"]
    pub const JANUARY: Month = Month {
        ord: 1i32
    };
    #[doc(alias = "MONTH_FEBRUARY")]
    #[doc = "Godot enumerator name: `MONTH_FEBRUARY`"]
    pub const FEBRUARY: Month = Month {
        ord: 2i32
    };
    #[doc(alias = "MONTH_MARCH")]
    #[doc = "Godot enumerator name: `MONTH_MARCH`"]
    pub const MARCH: Month = Month {
        ord: 3i32
    };
    #[doc(alias = "MONTH_APRIL")]
    #[doc = "Godot enumerator name: `MONTH_APRIL`"]
    pub const APRIL: Month = Month {
        ord: 4i32
    };
    #[doc(alias = "MONTH_MAY")]
    #[doc = "Godot enumerator name: `MONTH_MAY`"]
    pub const MAY: Month = Month {
        ord: 5i32
    };
    #[doc(alias = "MONTH_JUNE")]
    #[doc = "Godot enumerator name: `MONTH_JUNE`"]
    pub const JUNE: Month = Month {
        ord: 6i32
    };
    #[doc(alias = "MONTH_JULY")]
    #[doc = "Godot enumerator name: `MONTH_JULY`"]
    pub const JULY: Month = Month {
        ord: 7i32
    };
    #[doc(alias = "MONTH_AUGUST")]
    #[doc = "Godot enumerator name: `MONTH_AUGUST`"]
    pub const AUGUST: Month = Month {
        ord: 8i32
    };
    #[doc(alias = "MONTH_SEPTEMBER")]
    #[doc = "Godot enumerator name: `MONTH_SEPTEMBER`"]
    pub const SEPTEMBER: Month = Month {
        ord: 9i32
    };
    #[doc(alias = "MONTH_OCTOBER")]
    #[doc = "Godot enumerator name: `MONTH_OCTOBER`"]
    pub const OCTOBER: Month = Month {
        ord: 10i32
    };
    #[doc(alias = "MONTH_NOVEMBER")]
    #[doc = "Godot enumerator name: `MONTH_NOVEMBER`"]
    pub const NOVEMBER: Month = Month {
        ord: 11i32
    };
    #[doc(alias = "MONTH_DECEMBER")]
    #[doc = "Godot enumerator name: `MONTH_DECEMBER`"]
    pub const DECEMBER: Month = Month {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Month") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Month {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
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
            Self::JANUARY => "JANUARY", Self::FEBRUARY => "FEBRUARY", Self::MARCH => "MARCH", Self::APRIL => "APRIL", Self::MAY => "MAY", Self::JUNE => "JUNE", Self::JULY => "JULY", Self::AUGUST => "AUGUST", Self::SEPTEMBER => "SEPTEMBER", Self::OCTOBER => "OCTOBER", Self::NOVEMBER => "NOVEMBER", Self::DECEMBER => "DECEMBER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::JANUARY => "MONTH_JANUARY", Self::FEBRUARY => "MONTH_FEBRUARY", Self::MARCH => "MONTH_MARCH", Self::APRIL => "MONTH_APRIL", Self::MAY => "MONTH_MAY", Self::JUNE => "MONTH_JUNE", Self::JULY => "MONTH_JULY", Self::AUGUST => "MONTH_AUGUST", Self::SEPTEMBER => "MONTH_SEPTEMBER", Self::OCTOBER => "MONTH_OCTOBER", Self::NOVEMBER => "MONTH_NOVEMBER", Self::DECEMBER => "MONTH_DECEMBER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Month {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Month {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Month {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Weekday {
    ord: i32
}
impl Weekday {
    #[doc(alias = "WEEKDAY_SUNDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_SUNDAY`"]
    pub const SUNDAY: Weekday = Weekday {
        ord: 0i32
    };
    #[doc(alias = "WEEKDAY_MONDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_MONDAY`"]
    pub const MONDAY: Weekday = Weekday {
        ord: 1i32
    };
    #[doc(alias = "WEEKDAY_TUESDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_TUESDAY`"]
    pub const TUESDAY: Weekday = Weekday {
        ord: 2i32
    };
    #[doc(alias = "WEEKDAY_WEDNESDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_WEDNESDAY`"]
    pub const WEDNESDAY: Weekday = Weekday {
        ord: 3i32
    };
    #[doc(alias = "WEEKDAY_THURSDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_THURSDAY`"]
    pub const THURSDAY: Weekday = Weekday {
        ord: 4i32
    };
    #[doc(alias = "WEEKDAY_FRIDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_FRIDAY`"]
    pub const FRIDAY: Weekday = Weekday {
        ord: 5i32
    };
    #[doc(alias = "WEEKDAY_SATURDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_SATURDAY`"]
    pub const SATURDAY: Weekday = Weekday {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Weekday") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Weekday {
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
            Self::SUNDAY => "SUNDAY", Self::MONDAY => "MONDAY", Self::TUESDAY => "TUESDAY", Self::WEDNESDAY => "WEDNESDAY", Self::THURSDAY => "THURSDAY", Self::FRIDAY => "FRIDAY", Self::SATURDAY => "SATURDAY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SUNDAY => "WEEKDAY_SUNDAY", Self::MONDAY => "WEEKDAY_MONDAY", Self::TUESDAY => "WEEKDAY_TUESDAY", Self::WEDNESDAY => "WEEKDAY_WEDNESDAY", Self::THURSDAY => "WEEKDAY_THURSDAY", Self::FRIDAY => "WEEKDAY_FRIDAY", Self::SATURDAY => "WEEKDAY_SATURDAY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Weekday {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Weekday {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Weekday {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}