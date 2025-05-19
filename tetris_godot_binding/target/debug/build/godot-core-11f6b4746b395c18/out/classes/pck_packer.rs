#![doc = "Sidecar module for class [`PckPacker`][crate::classes::PckPacker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PCKPacker` enums](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PCKPacker.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`pck_packer`][crate::classes::pck_packer]: sidecar module with related enum/flag types\n* [`IPckPacker`][crate::classes::IPckPacker]: virtual methods\n\n\nSee also [Godot docs for `PCKPacker`](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PckPacker::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PckPacker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PckPacker`][crate::classes::PckPacker].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PCKPacker` methods](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPckPacker: crate::obj::GodotClass < Base = PckPacker > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PckPacker {
        pub(crate) fn pck_start_full(&mut self, pck_name: CowArg < GString >, alignment: i32, key: CowArg < GString >, encrypt_directory: bool,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >, bool);
            let args = (pck_name, alignment, key, encrypt_directory,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PckPacker", "pck_start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::pck_start_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn pck_start(&mut self, pck_name: impl AsArg < GString >,) -> crate::global::Error {
            self.pck_start_ex(pck_name,) . done()
        }
        #[inline]
        pub fn pck_start_ex < 'a > (&'a mut self, pck_name: impl AsArg < GString > + 'a,) -> ExPckStart < 'a > {
            ExPckStart::new(self, pck_name,)
        }
        pub(crate) fn add_file_full(&mut self, pck_path: CowArg < GString >, source_path: CowArg < GString >, encrypt: bool,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool);
            let args = (pck_path, source_path, encrypt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PckPacker", "add_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_file_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_file(&mut self, pck_path: impl AsArg < GString >, source_path: impl AsArg < GString >,) -> crate::global::Error {
            self.add_file_ex(pck_path, source_path,) . done()
        }
        #[inline]
        pub fn add_file_ex < 'a > (&'a mut self, pck_path: impl AsArg < GString > + 'a, source_path: impl AsArg < GString > + 'a,) -> ExAddFile < 'a > {
            ExAddFile::new(self, pck_path, source_path,)
        }
        pub(crate) fn flush_full(&mut self, verbose: bool,) -> crate::global::Error {
            type CallSig = (crate::global::Error, bool);
            let args = (verbose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PckPacker", "flush", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::flush_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn flush(&mut self,) -> crate::global::Error {
            self.flush_ex() . done()
        }
        #[inline]
        pub fn flush_ex < 'a > (&'a mut self,) -> ExFlush < 'a > {
            ExFlush::new(self,)
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
    impl crate::obj::GodotClass for PckPacker {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PCKPacker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PckPacker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PckPacker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PckPacker {
        
    }
    impl crate::obj::cap::GodotDefault for PckPacker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PckPacker {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PckPacker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PckPacker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PckPacker {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PckPacker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PckPacker::pck_start_ex`][super::PckPacker::pck_start_ex]."]
#[must_use]
pub struct ExPckStart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PckPacker, pck_name: CowArg < 'a, GString >, alignment: i32, key: CowArg < 'a, GString >, encrypt_directory: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPckStart < 'a > {
    fn new(surround_object: &'a mut re_export::PckPacker, pck_name: impl AsArg < GString > + 'a,) -> Self {
        let alignment = 32i32;
        let key = GString::from("0000000000000000000000000000000000000000000000000000000000000000");
        let encrypt_directory = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pck_name: pck_name.into_arg(), alignment: alignment, key: CowArg::Owned(key), encrypt_directory: encrypt_directory,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: i32) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn key(self, key: impl AsArg < GString > + 'a) -> Self {
        Self {
            key: key.into_arg(), .. self
        }
    }
    #[inline]
    pub fn encrypt_directory(self, encrypt_directory: bool) -> Self {
        Self {
            encrypt_directory: encrypt_directory, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, pck_name, alignment, key, encrypt_directory,
        }
        = self;
        re_export::PckPacker::pck_start_full(surround_object, pck_name, alignment, key, encrypt_directory,)
    }
}
#[doc = "Default-param extender for [`PckPacker::add_file_ex`][super::PckPacker::add_file_ex]."]
#[must_use]
pub struct ExAddFile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PckPacker, pck_path: CowArg < 'a, GString >, source_path: CowArg < 'a, GString >, encrypt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddFile < 'a > {
    fn new(surround_object: &'a mut re_export::PckPacker, pck_path: impl AsArg < GString > + 'a, source_path: impl AsArg < GString > + 'a,) -> Self {
        let encrypt = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pck_path: pck_path.into_arg(), source_path: source_path.into_arg(), encrypt: encrypt,
        }
    }
    #[inline]
    pub fn encrypt(self, encrypt: bool) -> Self {
        Self {
            encrypt: encrypt, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, pck_path, source_path, encrypt,
        }
        = self;
        re_export::PckPacker::add_file_full(surround_object, pck_path, source_path, encrypt,)
    }
}
#[doc = "Default-param extender for [`PckPacker::flush_ex`][super::PckPacker::flush_ex]."]
#[must_use]
pub struct ExFlush < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PckPacker, verbose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFlush < 'a > {
    fn new(surround_object: &'a mut re_export::PckPacker,) -> Self {
        let verbose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, verbose: verbose,
        }
    }
    #[inline]
    pub fn verbose(self, verbose: bool) -> Self {
        Self {
            verbose: verbose, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, verbose,
        }
        = self;
        re_export::PckPacker::flush_full(surround_object, verbose,)
    }
}