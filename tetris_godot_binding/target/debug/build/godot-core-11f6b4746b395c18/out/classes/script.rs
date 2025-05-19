#![doc = "Sidecar module for class [`Script`][crate::classes::Script].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Script` enums](https://docs.godotengine.org/en/stable/classes/class_script.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Script.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`script`][crate::classes::script]: sidecar module with related enum/flag types\n* [`IScript`][crate::classes::IScript]: virtual methods\n\n\nSee also [Godot docs for `Script`](https://docs.godotengine.org/en/stable/classes/class_script.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Script>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Script {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Script`][crate::classes::Script].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Script` methods](https://docs.godotengine.org/en/stable/classes/class_script.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScript: crate::obj::GodotClass < Base = Script > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Script {
        pub fn can_instantiate(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "can_instantiate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instance_has(&self, base_object: impl AsObjectArg < crate::classes::Object >,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::Object >);
            let args = (base_object.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "instance_has", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_source_code(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "has_source_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source_code(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_source_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_source_code(&mut self, source: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "set_source_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn reload_full(&mut self, keep_state: bool,) -> crate::global::Error {
            type CallSig = (crate::global::Error, bool);
            let args = (keep_state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "reload", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::reload_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn reload(&mut self,) -> crate::global::Error {
            self.reload_ex() . done()
        }
        #[inline]
        pub fn reload_ex < 'a > (&'a mut self,) -> ExReload < 'a > {
            ExReload::new(self,)
        }
        pub fn get_base_script(&self,) -> Option < Gd < crate::classes::Script > > {
            type CallSig = (Option < Gd < crate::classes::Script > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_base_script", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_base_type(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_instance_base_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_name(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_global_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_script_signal(&self, signal_name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (signal_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "has_script_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_property_list(&mut self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_script_property_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_method_list(&mut self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_script_method_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_signal_list(&mut self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_script_signal_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_constant_map(&mut self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_script_constant_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_property_default_value(&mut self, property: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "get_property_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tool(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "is_tool", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_abstract(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Script", "is_abstract", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Script {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Script"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Script {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Script {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Script {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Script {
        
    }
    impl std::ops::Deref for Script {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Script {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Script`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Script {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Script > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Script::reload_ex`][super::Script::reload_ex]."]
#[must_use]
pub struct ExReload < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Script, keep_state: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReload < 'a > {
    fn new(surround_object: &'a mut re_export::Script,) -> Self {
        let keep_state = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, keep_state: keep_state,
        }
    }
    #[inline]
    pub fn keep_state(self, keep_state: bool) -> Self {
        Self {
            keep_state: keep_state, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, keep_state,
        }
        = self;
        re_export::Script::reload_full(surround_object, keep_state,)
    }
}