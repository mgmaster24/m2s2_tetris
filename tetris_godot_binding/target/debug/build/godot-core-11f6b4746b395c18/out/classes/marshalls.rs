#![doc = "Sidecar module for class [`Marshalls`][crate::classes::Marshalls].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Marshalls` enums](https://docs.godotengine.org/en/stable/classes/class_marshalls.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Marshalls.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`marshalls`][crate::classes::marshalls]: sidecar module with related enum/flag types\n* [`IMarshalls`][crate::classes::IMarshalls]: virtual methods\n\n\nSee also [Godot docs for `Marshalls`](https://docs.godotengine.org/en/stable/classes/class_marshalls.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Marshalls::singleton()`][Marshalls::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Marshalls {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Marshalls`][crate::classes::Marshalls].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Marshalls` methods](https://docs.godotengine.org/en/stable/classes/class_marshalls.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMarshalls: crate::obj::GodotClass < Base = Marshalls > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Marshalls {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"Marshalls");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn variant_to_base64_full(&mut self, variant: RefArg < Variant >, full_objects: bool,) -> GString {
            type CallSig < 'a0, > = (GString, RefArg < 'a0, Variant >, bool);
            let args = (variant, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Marshalls", "variant_to_base64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::variant_to_base64_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn variant_to_base64(&mut self, variant: &Variant,) -> GString {
            self.variant_to_base64_ex(variant,) . done()
        }
        #[inline]
        pub fn variant_to_base64_ex < 'a > (&'a mut self, variant: &'a Variant,) -> ExVariantToBase64 < 'a > {
            ExVariantToBase64::new(self, variant,)
        }
        pub(crate) fn base64_to_variant_full(&mut self, base64_str: CowArg < GString >, allow_objects: bool,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, GString >, bool);
            let args = (base64_str, allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Marshalls", "base64_to_variant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::base64_to_variant_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn base64_to_variant(&mut self, base64_str: impl AsArg < GString >,) -> Variant {
            self.base64_to_variant_ex(base64_str,) . done()
        }
        #[inline]
        pub fn base64_to_variant_ex < 'a > (&'a mut self, base64_str: impl AsArg < GString > + 'a,) -> ExBase64ToVariant < 'a > {
            ExBase64ToVariant::new(self, base64_str,)
        }
        pub fn raw_to_base64(&mut self, array: &PackedByteArray,) -> GString {
            type CallSig < 'a0, > = (GString, RefArg < 'a0, PackedByteArray >);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Marshalls", "raw_to_base64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn base64_to_raw(&mut self, base64_str: impl AsArg < GString >,) -> PackedByteArray {
            type CallSig < 'a0, > = (PackedByteArray, CowArg < 'a0, GString >);
            let args = (base64_str.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Marshalls", "base64_to_raw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn utf8_to_base64(&mut self, utf8_str: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (utf8_str.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Marshalls", "utf8_to_base64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn base64_to_utf8(&mut self, base64_str: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (base64_str.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Marshalls", "base64_to_utf8", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Marshalls {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Marshalls"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Marshalls {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Marshalls {
        
    }
    impl std::ops::Deref for Marshalls {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Marshalls {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Marshalls`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Marshalls {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Marshalls > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Marshalls::variant_to_base64_ex`][super::Marshalls::variant_to_base64_ex]."]
#[must_use]
pub struct ExVariantToBase64 < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Marshalls, variant: CowArg < 'a, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVariantToBase64 < 'a > {
    fn new(surround_object: &'a mut re_export::Marshalls, variant: &'a Variant,) -> Self {
        let full_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, variant: CowArg::Borrowed(variant), full_objects: full_objects,
        }
    }
    #[inline]
    pub fn full_objects(self, full_objects: bool) -> Self {
        Self {
            full_objects: full_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, variant, full_objects,
        }
        = self;
        re_export::Marshalls::variant_to_base64_full(surround_object, variant.cow_as_arg(), full_objects,)
    }
}
#[doc = "Default-param extender for [`Marshalls::base64_to_variant_ex`][super::Marshalls::base64_to_variant_ex]."]
#[must_use]
pub struct ExBase64ToVariant < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Marshalls, base64_str: CowArg < 'a, GString >, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBase64ToVariant < 'a > {
    fn new(surround_object: &'a mut re_export::Marshalls, base64_str: impl AsArg < GString > + 'a,) -> Self {
        let allow_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, base64_str: base64_str.into_arg(), allow_objects: allow_objects,
        }
    }
    #[inline]
    pub fn allow_objects(self, allow_objects: bool) -> Self {
        Self {
            allow_objects: allow_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, base64_str, allow_objects,
        }
        = self;
        re_export::Marshalls::base64_to_variant_full(surround_object, base64_str, allow_objects,)
    }
}