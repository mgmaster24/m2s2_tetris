#![doc = "Sidecar module for class [`GDExtensionManager`][crate::classes::GDExtensionManager].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GDExtensionManager` enums](https://docs.godotengine.org/en/stable/classes/class_gdextensionmanager.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GDExtensionManager.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`gdextension_manager`][crate::classes::gdextension_manager]: sidecar module with related enum/flag types\n* [`IGDExtensionManager`][crate::classes::IGDExtensionManager]: virtual methods\n\n\nSee also [Godot docs for `GDExtensionManager`](https://docs.godotengine.org/en/stable/classes/class_gdextensionmanager.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`GDExtensionManager::singleton()`][GDExtensionManager::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GDExtensionManager {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GDExtensionManager`][crate::classes::GDExtensionManager].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GDExtensionManager` methods](https://docs.godotengine.org/en/stable/classes/class_gdextensionmanager.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGDExtensionManager: crate::obj::GodotClass < Base = GDExtensionManager > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GDExtensionManager {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"GDExtensionManager");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn load_extension(&mut self, path: impl AsArg < GString >,) -> crate::classes::gdextension_manager::LoadStatus {
            type CallSig < 'a0, > = (crate::classes::gdextension_manager::LoadStatus, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GDExtensionManager", "load_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reload_extension(&mut self, path: impl AsArg < GString >,) -> crate::classes::gdextension_manager::LoadStatus {
            type CallSig < 'a0, > = (crate::classes::gdextension_manager::LoadStatus, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GDExtensionManager", "reload_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unload_extension(&mut self, path: impl AsArg < GString >,) -> crate::classes::gdextension_manager::LoadStatus {
            type CallSig < 'a0, > = (crate::classes::gdextension_manager::LoadStatus, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GDExtensionManager", "unload_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_extension_loaded(&self, path: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GDExtensionManager", "is_extension_loaded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_loaded_extensions(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GDExtensionManager", "get_loaded_extensions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_extension(&mut self, path: impl AsArg < GString >,) -> Option < Gd < crate::classes::GDExtension > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::GDExtension > >, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GDExtensionManager", "get_extension", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GDExtensionManager {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GDExtensionManager"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GDExtensionManager {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GDExtensionManager {
        
    }
    impl std::ops::Deref for GDExtensionManager {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GDExtensionManager {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GDExtensionManager`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GDExtensionManager {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GDExtensionManager > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LoadStatus {
    ord: i32
}
impl LoadStatus {
    #[doc(alias = "LOAD_STATUS_OK")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_OK`"]
    pub const OK: LoadStatus = LoadStatus {
        ord: 0i32
    };
    #[doc(alias = "LOAD_STATUS_FAILED")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_FAILED`"]
    pub const FAILED: LoadStatus = LoadStatus {
        ord: 1i32
    };
    #[doc(alias = "LOAD_STATUS_ALREADY_LOADED")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_ALREADY_LOADED`"]
    pub const ALREADY_LOADED: LoadStatus = LoadStatus {
        ord: 2i32
    };
    #[doc(alias = "LOAD_STATUS_NOT_LOADED")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_NOT_LOADED`"]
    pub const NOT_LOADED: LoadStatus = LoadStatus {
        ord: 3i32
    };
    #[doc(alias = "LOAD_STATUS_NEEDS_RESTART")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_NEEDS_RESTART`"]
    pub const NEEDS_RESTART: LoadStatus = LoadStatus {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for LoadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LoadStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LoadStatus {
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
            Self::OK => "OK", Self::FAILED => "FAILED", Self::ALREADY_LOADED => "ALREADY_LOADED", Self::NOT_LOADED => "NOT_LOADED", Self::NEEDS_RESTART => "NEEDS_RESTART", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OK => "LOAD_STATUS_OK", Self::FAILED => "LOAD_STATUS_FAILED", Self::ALREADY_LOADED => "LOAD_STATUS_ALREADY_LOADED", Self::NOT_LOADED => "LOAD_STATUS_NOT_LOADED", Self::NEEDS_RESTART => "LOAD_STATUS_NEEDS_RESTART", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for LoadStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LoadStatus {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LoadStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}