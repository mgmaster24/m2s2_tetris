#![doc = "Sidecar module for class [`ProjectSettings`][crate::classes::ProjectSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ProjectSettings` enums](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ProjectSettings.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`project_settings`][crate::classes::project_settings]: sidecar module with related enum/flag types\n* [`IProjectSettings`][crate::classes::IProjectSettings]: virtual methods\n\n\nSee also [Godot docs for `ProjectSettings`](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`ProjectSettings::singleton()`][ProjectSettings::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ProjectSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ProjectSettings`][crate::classes::ProjectSettings].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ProjectSettings` methods](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IProjectSettings: crate::obj::GodotClass < Base = ProjectSettings > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ProjectSettings {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"ProjectSettings");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn has_setting(&self, name: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "has_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_setting(&mut self, name: impl AsArg < GString >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, Variant >);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_setting_full(&self, name: CowArg < GString >, default_value: RefArg < Variant >,) -> Variant {
            type CallSig < 'a0, 'a1, > = (Variant, CowArg < 'a0, GString >, RefArg < 'a1, Variant >);
            let args = (name, default_value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_setting_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_setting(&self, name: impl AsArg < GString >,) -> Variant {
            self.get_setting_ex(name,) . done()
        }
        #[inline]
        pub fn get_setting_ex < 'a > (&'a self, name: impl AsArg < GString > + 'a,) -> ExGetSetting < 'a > {
            ExGetSetting::new(self, name,)
        }
        pub fn get_setting_with_override(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_setting_with_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_class_list(&mut self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_global_class_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_order(&mut self, name: impl AsArg < GString >, position: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (name.into_arg(), position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_order(&self, name: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_initial_value(&mut self, name: impl AsArg < GString >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, Variant >);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_initial_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_basic(&mut self, name: impl AsArg < GString >, basic: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (name.into_arg(), basic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_as_basic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_internal(&mut self, name: impl AsArg < GString >, internal: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (name.into_arg(), internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_as_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_property_info(&mut self, hint: &Dictionary,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(hint),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "add_property_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_restart_if_changed(&mut self, name: impl AsArg < GString >, restart: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (name.into_arg(), restart,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_restart_if_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn localize_path(&self, path: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "localize_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn globalize_path(&self, path: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "globalize_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "save", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn load_resource_pack_full(&mut self, pack: CowArg < GString >, replace_files: bool, offset: i32,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >, bool, i32);
            let args = (pack, replace_files, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "load_resource_pack", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::load_resource_pack_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn load_resource_pack(&mut self, pack: impl AsArg < GString >,) -> bool {
            self.load_resource_pack_ex(pack,) . done()
        }
        #[inline]
        pub fn load_resource_pack_ex < 'a > (&'a mut self, pack: impl AsArg < GString > + 'a,) -> ExLoadResourcePack < 'a > {
            ExLoadResourcePack::new(self, pack,)
        }
        pub fn save_custom(&mut self, file: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ProjectSettings", "save_custom", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ProjectSettings {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ProjectSettings"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ProjectSettings {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ProjectSettings {
        
    }
    impl std::ops::Deref for ProjectSettings {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ProjectSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ProjectSettings`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ProjectSettings {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ProjectSettings > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ProjectSettings::get_setting_ex`][super::ProjectSettings::get_setting_ex]."]
#[must_use]
pub struct ExGetSetting < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ProjectSettings, name: CowArg < 'a, GString >, default_value: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSetting < 'a > {
    fn new(surround_object: &'a re_export::ProjectSettings, name: impl AsArg < GString > + 'a,) -> Self {
        let default_value = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), default_value: CowArg::Owned(default_value),
        }
    }
    #[inline]
    pub fn default_value(self, default_value: &'a Variant) -> Self {
        Self {
            default_value: CowArg::Borrowed(default_value), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, name, default_value,
        }
        = self;
        re_export::ProjectSettings::get_setting_full(surround_object, name, default_value.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`ProjectSettings::load_resource_pack_ex`][super::ProjectSettings::load_resource_pack_ex]."]
#[must_use]
pub struct ExLoadResourcePack < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ProjectSettings, pack: CowArg < 'a, GString >, replace_files: bool, offset: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadResourcePack < 'a > {
    fn new(surround_object: &'a mut re_export::ProjectSettings, pack: impl AsArg < GString > + 'a,) -> Self {
        let replace_files = true;
        let offset = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pack: pack.into_arg(), replace_files: replace_files, offset: offset,
        }
    }
    #[inline]
    pub fn replace_files(self, replace_files: bool) -> Self {
        Self {
            replace_files: replace_files, .. self
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, pack, replace_files, offset,
        }
        = self;
        re_export::ProjectSettings::load_resource_pack_full(surround_object, pack, replace_files, offset,)
    }
}