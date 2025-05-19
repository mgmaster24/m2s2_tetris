#![doc = "Sidecar module for class [`ResourceLoader`][crate::classes::ResourceLoader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceLoader` enums](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceLoader.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`resource_loader`][crate::classes::resource_loader]: sidecar module with related enum/flag types\n* [`IResourceLoader`][crate::classes::IResourceLoader]: virtual methods\n\n\nSee also [Godot docs for `ResourceLoader`](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`ResourceLoader::singleton()`][ResourceLoader::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceLoader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ResourceLoader`][crate::classes::ResourceLoader].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ResourceLoader` methods](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IResourceLoader: crate::obj::GodotClass < Base = ResourceLoader > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ResourceLoader {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"ResourceLoader");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn load_full(&mut self, path: CowArg < GString >, type_hint: CowArg < GString >, cache_mode: crate::classes::resource_loader::CacheMode,) -> Option < Gd < crate::classes::Resource > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::Resource > >, CowArg < 'a0, GString >, CowArg < 'a1, GString >, crate::classes::resource_loader::CacheMode);
            let args = (path, type_hint, cache_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "load", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::load_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn load(&mut self, path: impl AsArg < GString >,) -> Option < Gd < crate::classes::Resource > > {
            self.load_ex(path,) . done()
        }
        #[inline]
        pub fn load_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExLoad < 'a > {
            ExLoad::new(self, path,)
        }
        pub fn get_recognized_extensions_for_type(&mut self, type_: impl AsArg < GString >,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_recognized_extensions_for_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_resource_format_loader_full(&mut self, format_loader: ObjectArg < crate::classes::ResourceFormatLoader >, at_front: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::ResourceFormatLoader >, bool);
            let args = (format_loader, at_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "add_resource_format_loader", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_resource_format_loader_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_resource_format_loader(&mut self, format_loader: impl AsObjectArg < crate::classes::ResourceFormatLoader >,) {
            self.add_resource_format_loader_ex(format_loader,) . done()
        }
        #[inline]
        pub fn add_resource_format_loader_ex < 'a > (&'a mut self, format_loader: impl AsObjectArg < crate::classes::ResourceFormatLoader >,) -> ExAddResourceFormatLoader < 'a > {
            ExAddResourceFormatLoader::new(self, format_loader,)
        }
        pub fn remove_resource_format_loader(&mut self, format_loader: impl AsObjectArg < crate::classes::ResourceFormatLoader >,) {
            type CallSig = ((), ObjectArg < crate::classes::ResourceFormatLoader >);
            let args = (format_loader.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "remove_resource_format_loader", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_abort_on_missing_resources(&mut self, abort: bool,) {
            type CallSig = ((), bool);
            let args = (abort,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "set_abort_on_missing_resources", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dependencies(&mut self, path: impl AsArg < GString >,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_dependencies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_cached(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "has_cached", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn exists_full(&mut self, path: CowArg < GString >, type_hint: CowArg < GString >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (path, type_hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::exists_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn exists(&mut self, path: impl AsArg < GString >,) -> bool {
            self.exists_ex(path,) . done()
        }
        #[inline]
        pub fn exists_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExExists < 'a > {
            ExExists::new(self, path,)
        }
        pub fn get_resource_uid(&mut self, path: impl AsArg < GString >,) -> i64 {
            type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_resource_uid", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ResourceLoader {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ResourceLoader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceLoader {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ResourceLoader {
        
    }
    impl std::ops::Deref for ResourceLoader {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceLoader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ResourceLoader`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ResourceLoader {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ResourceLoader > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ResourceLoader::load_ex`][super::ResourceLoader::load_ex]."]
#[must_use]
pub struct ExLoad < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceLoader, path: CowArg < 'a, GString >, type_hint: CowArg < 'a, GString >, cache_mode: crate::classes::resource_loader::CacheMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoad < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, path: impl AsArg < GString > + 'a,) -> Self {
        let type_hint = GString::from("");
        let cache_mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), type_hint: CowArg::Owned(type_hint), cache_mode: cache_mode,
        }
    }
    #[inline]
    pub fn type_hint(self, type_hint: impl AsArg < GString > + 'a) -> Self {
        Self {
            type_hint: type_hint.into_arg(), .. self
        }
    }
    #[inline]
    pub fn cache_mode(self, cache_mode: crate::classes::resource_loader::CacheMode) -> Self {
        Self {
            cache_mode: cache_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Resource > > {
        let Self {
            _phantom, surround_object, path, type_hint, cache_mode,
        }
        = self;
        re_export::ResourceLoader::load_full(surround_object, path, type_hint, cache_mode,)
    }
}
#[doc = "Default-param extender for [`ResourceLoader::add_resource_format_loader_ex`][super::ResourceLoader::add_resource_format_loader_ex]."]
#[must_use]
pub struct ExAddResourceFormatLoader < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceLoader, format_loader: ObjectCow < crate::classes::ResourceFormatLoader >, at_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddResourceFormatLoader < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, format_loader: impl AsObjectArg < crate::classes::ResourceFormatLoader >,) -> Self {
        let at_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format_loader: format_loader.consume_arg(), at_front: at_front,
        }
    }
    #[inline]
    pub fn at_front(self, at_front: bool) -> Self {
        Self {
            at_front: at_front, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, format_loader, at_front,
        }
        = self;
        re_export::ResourceLoader::add_resource_format_loader_full(surround_object, format_loader.cow_as_object_arg(), at_front,)
    }
}
#[doc = "Default-param extender for [`ResourceLoader::exists_ex`][super::ResourceLoader::exists_ex]."]
#[must_use]
pub struct ExExists < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceLoader, path: CowArg < 'a, GString >, type_hint: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExists < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, path: impl AsArg < GString > + 'a,) -> Self {
        let type_hint = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), type_hint: CowArg::Owned(type_hint),
        }
    }
    #[inline]
    pub fn type_hint(self, type_hint: impl AsArg < GString > + 'a) -> Self {
        Self {
            type_hint: type_hint.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, path, type_hint,
        }
        = self;
        re_export::ResourceLoader::exists_full(surround_object, path, type_hint,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ThreadLoadStatus {
    ord: i32
}
impl ThreadLoadStatus {
    #[doc(alias = "THREAD_LOAD_INVALID_RESOURCE")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_INVALID_RESOURCE`"]
    pub const INVALID_RESOURCE: ThreadLoadStatus = ThreadLoadStatus {
        ord: 0i32
    };
    #[doc(alias = "THREAD_LOAD_IN_PROGRESS")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_IN_PROGRESS`"]
    pub const IN_PROGRESS: ThreadLoadStatus = ThreadLoadStatus {
        ord: 1i32
    };
    #[doc(alias = "THREAD_LOAD_FAILED")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_FAILED`"]
    pub const FAILED: ThreadLoadStatus = ThreadLoadStatus {
        ord: 2i32
    };
    #[doc(alias = "THREAD_LOAD_LOADED")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_LOADED`"]
    pub const LOADED: ThreadLoadStatus = ThreadLoadStatus {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ThreadLoadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ThreadLoadStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ThreadLoadStatus {
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
            Self::INVALID_RESOURCE => "INVALID_RESOURCE", Self::IN_PROGRESS => "IN_PROGRESS", Self::FAILED => "FAILED", Self::LOADED => "LOADED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INVALID_RESOURCE => "THREAD_LOAD_INVALID_RESOURCE", Self::IN_PROGRESS => "THREAD_LOAD_IN_PROGRESS", Self::FAILED => "THREAD_LOAD_FAILED", Self::LOADED => "THREAD_LOAD_LOADED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ThreadLoadStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ThreadLoadStatus {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ThreadLoadStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CacheMode {
    ord: i32
}
impl CacheMode {
    #[doc(alias = "CACHE_MODE_IGNORE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_IGNORE`"]
    pub const IGNORE: CacheMode = CacheMode {
        ord: 0i32
    };
    #[doc(alias = "CACHE_MODE_REUSE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REUSE`"]
    pub const REUSE: CacheMode = CacheMode {
        ord: 1i32
    };
    #[doc(alias = "CACHE_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REPLACE`"]
    pub const REPLACE: CacheMode = CacheMode {
        ord: 2i32
    };
    #[doc(alias = "CACHE_MODE_IGNORE_DEEP")]
    #[doc = "Godot enumerator name: `CACHE_MODE_IGNORE_DEEP`"]
    pub const IGNORE_DEEP: CacheMode = CacheMode {
        ord: 3i32
    };
    #[doc(alias = "CACHE_MODE_REPLACE_DEEP")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REPLACE_DEEP`"]
    pub const REPLACE_DEEP: CacheMode = CacheMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CacheMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CacheMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CacheMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::IGNORE => "IGNORE", Self::REUSE => "REUSE", Self::REPLACE => "REPLACE", Self::IGNORE_DEEP => "IGNORE_DEEP", Self::REPLACE_DEEP => "REPLACE_DEEP", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::IGNORE => "CACHE_MODE_IGNORE", Self::REUSE => "CACHE_MODE_REUSE", Self::REPLACE => "CACHE_MODE_REPLACE", Self::IGNORE_DEEP => "CACHE_MODE_IGNORE_DEEP", Self::REPLACE_DEEP => "CACHE_MODE_REPLACE_DEEP", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CacheMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CacheMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CacheMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}