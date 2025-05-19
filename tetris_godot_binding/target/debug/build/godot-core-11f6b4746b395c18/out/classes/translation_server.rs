#![doc = "Sidecar module for class [`TranslationServer`][crate::classes::TranslationServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TranslationServer` enums](https://docs.godotengine.org/en/stable/classes/class_translationserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TranslationServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`translation_server`][crate::classes::translation_server]: sidecar module with related enum/flag types\n* [`ITranslationServer`][crate::classes::ITranslationServer]: virtual methods\n\n\nSee also [Godot docs for `TranslationServer`](https://docs.godotengine.org/en/stable/classes/class_translationserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`TranslationServer::singleton()`][TranslationServer::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TranslationServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TranslationServer`][crate::classes::TranslationServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TranslationServer` methods](https://docs.godotengine.org/en/stable/classes/class_translationserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITranslationServer: crate::obj::GodotClass < Base = TranslationServer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TranslationServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"TranslationServer");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn set_locale(&mut self, locale: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "set_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tool_locale(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_tool_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compare_locales(&self, locale_a: impl AsArg < GString >, locale_b: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (locale_a.into_arg(), locale_b.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "compare_locales", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn standardize_locale(&self, locale: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "standardize_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_all_languages(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_languages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language_name(&self, language: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_language_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_all_scripts(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_scripts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_name(&self, script: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_script_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_all_countries(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_countries", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_country_name(&self, country: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (country.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_country_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale_name(&self, locale: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_locale_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn translate_full(&self, message: CowArg < StringName >, context: CowArg < StringName >,) -> StringName {
            type CallSig < 'a0, 'a1, > = (StringName, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::translate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn translate(&self, message: impl AsArg < StringName >,) -> StringName {
            self.translate_ex(message,) . done()
        }
        #[inline]
        pub fn translate_ex < 'a > (&'a self, message: impl AsArg < StringName > + 'a,) -> ExTranslate < 'a > {
            ExTranslate::new(self, message,)
        }
        pub(crate) fn translate_plural_full(&self, message: CowArg < StringName >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> StringName {
            type CallSig < 'a0, 'a1, 'a2, > = (StringName, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "translate_plural", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::translate_plural_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn translate_plural(&self, message: impl AsArg < StringName >, plural_message: impl AsArg < StringName >, n: i32,) -> StringName {
            self.translate_plural_ex(message, plural_message, n,) . done()
        }
        #[inline]
        pub fn translate_plural_ex < 'a > (&'a self, message: impl AsArg < StringName > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> ExTranslatePlural < 'a > {
            ExTranslatePlural::new(self, message, plural_message, n,)
        }
        pub fn add_translation(&mut self, translation: impl AsObjectArg < crate::classes::Translation >,) {
            type CallSig = ((), ObjectArg < crate::classes::Translation >);
            let args = (translation.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "add_translation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_translation(&mut self, translation: impl AsObjectArg < crate::classes::Translation >,) {
            type CallSig = ((), ObjectArg < crate::classes::Translation >);
            let args = (translation.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "remove_translation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_translation_object(&mut self, locale: impl AsArg < GString >,) -> Option < Gd < crate::classes::Translation > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Translation > >, CowArg < 'a0, GString >);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_translation_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_loaded_locales(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "get_loaded_locales", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pseudolocalization_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "is_pseudolocalization_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pseudolocalization_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "set_pseudolocalization_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reload_pseudolocalization(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "reload_pseudolocalization", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pseudolocalize(&self, message: impl AsArg < StringName >,) -> StringName {
            type CallSig < 'a0, > = (StringName, CowArg < 'a0, StringName >);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TranslationServer", "pseudolocalize", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TranslationServer {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TranslationServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for TranslationServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TranslationServer {
        
    }
    impl std::ops::Deref for TranslationServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TranslationServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TranslationServer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TranslationServer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TranslationServer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TranslationServer::translate_ex`][super::TranslationServer::translate_ex]."]
#[must_use]
pub struct ExTranslate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TranslationServer, message: CowArg < 'a, StringName >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTranslate < 'a > {
    fn new(surround_object: &'a re_export::TranslationServer, message: impl AsArg < StringName > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::TranslationServer::translate_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`TranslationServer::translate_plural_ex`][super::TranslationServer::translate_plural_ex]."]
#[must_use]
pub struct ExTranslatePlural < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TranslationServer, message: CowArg < 'a, StringName >, plural_message: CowArg < 'a, StringName >, n: i32, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTranslatePlural < 'a > {
    fn new(surround_object: &'a re_export::TranslationServer, message: impl AsArg < StringName > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::TranslationServer::translate_plural_full(surround_object, message, plural_message, n, context,)
    }
}