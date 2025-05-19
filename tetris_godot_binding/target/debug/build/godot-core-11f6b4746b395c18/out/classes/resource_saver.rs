#![doc = "Sidecar module for class [`ResourceSaver`][crate::classes::ResourceSaver].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceSaver` enums](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceSaver.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`resource_saver`][crate::classes::resource_saver]: sidecar module with related enum/flag types\n* [`IResourceSaver`][crate::classes::IResourceSaver]: virtual methods\n\n\nSee also [Godot docs for `ResourceSaver`](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`ResourceSaver::singleton()`][ResourceSaver::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceSaver {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ResourceSaver`][crate::classes::ResourceSaver].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ResourceSaver` methods](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IResourceSaver: crate::obj::GodotClass < Base = ResourceSaver > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ResourceSaver {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"ResourceSaver");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn save_full(&mut self, resource: ObjectArg < crate::classes::Resource >, path: CowArg < GString >, flags: crate::classes::resource_saver::SaverFlags,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, ObjectArg < crate::classes::Resource >, CowArg < 'a0, GString >, crate::classes::resource_saver::SaverFlags);
            let args = (resource, path, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceSaver", "save", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save(&mut self, resource: impl AsObjectArg < crate::classes::Resource >,) -> crate::global::Error {
            self.save_ex(resource,) . done()
        }
        #[inline]
        pub fn save_ex < 'a > (&'a mut self, resource: impl AsObjectArg < crate::classes::Resource >,) -> ExSave < 'a > {
            ExSave::new(self, resource,)
        }
        pub fn get_recognized_extensions(&mut self, type_: impl AsObjectArg < crate::classes::Resource >,) -> PackedStringArray {
            type CallSig = (PackedStringArray, ObjectArg < crate::classes::Resource >);
            let args = (type_.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceSaver", "get_recognized_extensions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_resource_format_saver_full(&mut self, format_saver: ObjectArg < crate::classes::ResourceFormatSaver >, at_front: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::ResourceFormatSaver >, bool);
            let args = (format_saver, at_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceSaver", "add_resource_format_saver", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_resource_format_saver_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_resource_format_saver(&mut self, format_saver: impl AsObjectArg < crate::classes::ResourceFormatSaver >,) {
            self.add_resource_format_saver_ex(format_saver,) . done()
        }
        #[inline]
        pub fn add_resource_format_saver_ex < 'a > (&'a mut self, format_saver: impl AsObjectArg < crate::classes::ResourceFormatSaver >,) -> ExAddResourceFormatSaver < 'a > {
            ExAddResourceFormatSaver::new(self, format_saver,)
        }
        pub fn remove_resource_format_saver(&mut self, format_saver: impl AsObjectArg < crate::classes::ResourceFormatSaver >,) {
            type CallSig = ((), ObjectArg < crate::classes::ResourceFormatSaver >);
            let args = (format_saver.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ResourceSaver", "remove_resource_format_saver", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ResourceSaver {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ResourceSaver"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceSaver {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ResourceSaver {
        
    }
    impl std::ops::Deref for ResourceSaver {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceSaver {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ResourceSaver`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ResourceSaver {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ResourceSaver > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ResourceSaver::save_ex`][super::ResourceSaver::save_ex]."]
#[must_use]
pub struct ExSave < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceSaver, resource: ObjectCow < crate::classes::Resource >, path: CowArg < 'a, GString >, flags: crate::classes::resource_saver::SaverFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSave < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceSaver, resource: impl AsObjectArg < crate::classes::Resource >,) -> Self {
        let path = GString::from("");
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, resource: resource.consume_arg(), path: CowArg::Owned(path), flags: flags,
        }
    }
    #[inline]
    pub fn path(self, path: impl AsArg < GString > + 'a) -> Self {
        Self {
            path: path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::resource_saver::SaverFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, resource, path, flags,
        }
        = self;
        re_export::ResourceSaver::save_full(surround_object, resource.cow_as_object_arg(), path, flags,)
    }
}
#[doc = "Default-param extender for [`ResourceSaver::add_resource_format_saver_ex`][super::ResourceSaver::add_resource_format_saver_ex]."]
#[must_use]
pub struct ExAddResourceFormatSaver < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceSaver, format_saver: ObjectCow < crate::classes::ResourceFormatSaver >, at_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddResourceFormatSaver < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceSaver, format_saver: impl AsObjectArg < crate::classes::ResourceFormatSaver >,) -> Self {
        let at_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format_saver: format_saver.consume_arg(), at_front: at_front,
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
            _phantom, surround_object, format_saver, at_front,
        }
        = self;
        re_export::ResourceSaver::add_resource_format_saver_full(surround_object, format_saver.cow_as_object_arg(), at_front,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct SaverFlags {
    ord: u64
}
impl SaverFlags {
    #[doc(alias = "FLAG_NONE")]
    #[doc = "Godot enumerator name: `FLAG_NONE`"]
    pub const NONE: SaverFlags = SaverFlags {
        ord: 0u64
    };
    #[doc(alias = "FLAG_RELATIVE_PATHS")]
    #[doc = "Godot enumerator name: `FLAG_RELATIVE_PATHS`"]
    pub const RELATIVE_PATHS: SaverFlags = SaverFlags {
        ord: 1u64
    };
    #[doc(alias = "FLAG_BUNDLE_RESOURCES")]
    #[doc = "Godot enumerator name: `FLAG_BUNDLE_RESOURCES`"]
    pub const BUNDLE_RESOURCES: SaverFlags = SaverFlags {
        ord: 2u64
    };
    #[doc(alias = "FLAG_CHANGE_PATH")]
    #[doc = "Godot enumerator name: `FLAG_CHANGE_PATH`"]
    pub const CHANGE_PATH: SaverFlags = SaverFlags {
        ord: 4u64
    };
    #[doc(alias = "FLAG_OMIT_EDITOR_PROPERTIES")]
    #[doc = "Godot enumerator name: `FLAG_OMIT_EDITOR_PROPERTIES`"]
    pub const OMIT_EDITOR_PROPERTIES: SaverFlags = SaverFlags {
        ord: 8u64
    };
    #[doc(alias = "FLAG_SAVE_BIG_ENDIAN")]
    #[doc = "Godot enumerator name: `FLAG_SAVE_BIG_ENDIAN`"]
    pub const SAVE_BIG_ENDIAN: SaverFlags = SaverFlags {
        ord: 16u64
    };
    #[doc(alias = "FLAG_COMPRESS")]
    #[doc = "Godot enumerator name: `FLAG_COMPRESS`"]
    pub const COMPRESS: SaverFlags = SaverFlags {
        ord: 32u64
    };
    #[doc(alias = "FLAG_REPLACE_SUBRESOURCE_PATHS")]
    #[doc = "Godot enumerator name: `FLAG_REPLACE_SUBRESOURCE_PATHS`"]
    pub const REPLACE_SUBRESOURCE_PATHS: SaverFlags = SaverFlags {
        ord: 64u64
    };
    
}
impl std::fmt::Debug for SaverFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::RELATIVE_PATHS => "RELATIVE_PATHS", Self::BUNDLE_RESOURCES => "BUNDLE_RESOURCES", Self::CHANGE_PATH => "CHANGE_PATH", Self::OMIT_EDITOR_PROPERTIES => "OMIT_EDITOR_PROPERTIES", Self::SAVE_BIG_ENDIAN => "SAVE_BIG_ENDIAN", Self::COMPRESS => "COMPRESS", Self::REPLACE_SUBRESOURCE_PATHS => "REPLACE_SUBRESOURCE_PATHS", _ => {
                f.debug_struct("SaverFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for SaverFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for SaverFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for SaverFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for SaverFlags {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SaverFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}