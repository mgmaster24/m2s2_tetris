#![doc = "Sidecar module for class [`InputMap`][crate::classes::InputMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputMap` enums](https://docs.godotengine.org/en/stable/classes/class_inputmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputMap.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`input_map`][crate::classes::input_map]: sidecar module with related enum/flag types\n* [`IInputMap`][crate::classes::IInputMap]: virtual methods\n\n\nSee also [Godot docs for `InputMap`](https://docs.godotengine.org/en/stable/classes/class_inputmap.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`InputMap::singleton()`][InputMap::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InputMap`][crate::classes::InputMap].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InputMap` methods](https://docs.godotengine.org/en/stable/classes/class_inputmap.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputMap: crate::obj::GodotClass < Base = InputMap > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InputMap {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"InputMap");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn has_action(&self, action: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "has_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_actions(&mut self,) -> Array < StringName > {
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "get_actions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_action_full(&mut self, action: CowArg < StringName >, deadzone: f32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, f32);
            let args = (action, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "add_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_action(&mut self, action: impl AsArg < StringName >,) {
            self.add_action_ex(action,) . done()
        }
        #[inline]
        pub fn add_action_ex < 'a > (&'a mut self, action: impl AsArg < StringName > + 'a,) -> ExAddAction < 'a > {
            ExAddAction::new(self, action,)
        }
        pub fn erase_action(&mut self, action: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "erase_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_set_deadzone(&mut self, action: impl AsArg < StringName >, deadzone: f32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, f32);
            let args = (action.into_arg(), deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "action_set_deadzone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_get_deadzone(&mut self, action: impl AsArg < StringName >,) -> f32 {
            type CallSig < 'a0, > = (f32, CowArg < 'a0, StringName >);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "action_get_deadzone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_add_event(&mut self, action: impl AsArg < StringName >, event: impl AsObjectArg < crate::classes::InputEvent >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, ObjectArg < crate::classes::InputEvent >);
            let args = (action.into_arg(), event.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "action_add_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_has_event(&mut self, action: impl AsArg < StringName >, event: impl AsObjectArg < crate::classes::InputEvent >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >, ObjectArg < crate::classes::InputEvent >);
            let args = (action.into_arg(), event.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "action_has_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_erase_event(&mut self, action: impl AsArg < StringName >, event: impl AsObjectArg < crate::classes::InputEvent >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, ObjectArg < crate::classes::InputEvent >);
            let args = (action.into_arg(), event.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "action_erase_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_erase_events(&mut self, action: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "action_erase_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn action_get_events(&mut self, action: impl AsArg < StringName >,) -> Array < Gd < crate::classes::InputEvent > > {
            type CallSig < 'a0, > = (Array < Gd < crate::classes::InputEvent > >, CowArg < 'a0, StringName >);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "action_get_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn event_is_action_full(&self, event: ObjectArg < crate::classes::InputEvent >, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallSig < 'a0, > = (bool, ObjectArg < crate::classes::InputEvent >, CowArg < 'a0, StringName >, bool);
            let args = (event, action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "event_is_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::event_is_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn event_is_action(&self, event: impl AsObjectArg < crate::classes::InputEvent >, action: impl AsArg < StringName >,) -> bool {
            self.event_is_action_ex(event, action,) . done()
        }
        #[inline]
        pub fn event_is_action_ex < 'a > (&'a self, event: impl AsObjectArg < crate::classes::InputEvent >, action: impl AsArg < StringName > + 'a,) -> ExEventIsAction < 'a > {
            ExEventIsAction::new(self, event, action,)
        }
        pub fn load_from_project_settings(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputMap", "load_from_project_settings", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for InputMap {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"InputMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputMap {
        
    }
    impl std::ops::Deref for InputMap {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`InputMap`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_InputMap {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputMap > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`InputMap::add_action_ex`][super::InputMap::add_action_ex]."]
#[must_use]
pub struct ExAddAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::InputMap, action: CowArg < 'a, StringName >, deadzone: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddAction < 'a > {
    fn new(surround_object: &'a mut re_export::InputMap, action: impl AsArg < StringName > + 'a,) -> Self {
        let deadzone = 0.5f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), deadzone: deadzone,
        }
    }
    #[inline]
    pub fn deadzone(self, deadzone: f32) -> Self {
        Self {
            deadzone: deadzone, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, action, deadzone,
        }
        = self;
        re_export::InputMap::add_action_full(surround_object, action, deadzone,)
    }
}
#[doc = "Default-param extender for [`InputMap::event_is_action_ex`][super::InputMap::event_is_action_ex]."]
#[must_use]
pub struct ExEventIsAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputMap, event: ObjectCow < crate::classes::InputEvent >, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEventIsAction < 'a > {
    fn new(surround_object: &'a re_export::InputMap, event: impl AsObjectArg < crate::classes::InputEvent >, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.consume_arg(), action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, event, action, exact_match,
        }
        = self;
        re_export::InputMap::event_is_action_full(surround_object, event.cow_as_object_arg(), action, exact_match,)
    }
}