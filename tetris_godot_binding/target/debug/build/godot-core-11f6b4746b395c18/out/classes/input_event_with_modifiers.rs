#![doc = "Sidecar module for class [`InputEventWithModifiers`][crate::classes::InputEventWithModifiers].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventWithModifiers` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventwithmodifiers.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventWithModifiers.`\n\nInherits [`InputEventFromWindow`][crate::classes::InputEventFromWindow].\n\nRelated symbols:\n\n* [`IInputEventWithModifiers`][crate::classes::IInputEventWithModifiers]: virtual methods\n\n\nSee also [Godot docs for `InputEventWithModifiers`](https://docs.godotengine.org/en/stable/classes/class_inputeventwithmodifiers.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InputEventWithModifiers>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventWithModifiers {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InputEventWithModifiers`][crate::classes::InputEventWithModifiers].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InputEventWithModifiers` methods](https://docs.godotengine.org/en/stable/classes/class_inputeventwithmodifiers.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputEventWithModifiers: crate::obj::GodotClass < Base = InputEventWithModifiers > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InputEventWithModifiers {
        pub fn set_command_or_control_autoremap(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_command_or_control_autoremap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_command_or_control_autoremap(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_command_or_control_autoremap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_command_or_control_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_command_or_control_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alt_pressed(&mut self, pressed: bool,) {
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_alt_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_alt_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_alt_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shift_pressed(&mut self, pressed: bool,) {
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_shift_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shift_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_shift_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ctrl_pressed(&mut self, pressed: bool,) {
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_ctrl_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ctrl_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_ctrl_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_meta_pressed(&mut self, pressed: bool,) {
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_meta_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_meta_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_meta_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modifiers_mask(&self,) -> crate::global::KeyModifierMask {
            type CallSig = (crate::global::KeyModifierMask,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "get_modifiers_mask", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for InputEventWithModifiers {
        type Base = crate::classes::InputEventFromWindow;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"InputEventWithModifiers"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventWithModifiers {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEventFromWindow > for InputEventWithModifiers {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEvent > for InputEventWithModifiers {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEventWithModifiers {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEventWithModifiers {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEventWithModifiers {
        
    }
    impl std::ops::Deref for InputEventWithModifiers {
        type Target = crate::classes::InputEventFromWindow;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventWithModifiers {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`InputEventWithModifiers`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_InputEventWithModifiers {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEventWithModifiers > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEventFromWindow > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEvent > for $Class {
                
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