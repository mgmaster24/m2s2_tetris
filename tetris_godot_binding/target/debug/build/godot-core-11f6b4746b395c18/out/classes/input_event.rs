#![doc = "Sidecar module for class [`InputEvent`][crate::classes::InputEvent].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEvent` enums](https://docs.godotengine.org/en/stable/classes/class_inputevent.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEvent.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`input_event`][crate::classes::input_event]: sidecar module with related enum/flag types\n* [`IInputEvent`][crate::classes::IInputEvent]: virtual methods\n\n\nSee also [Godot docs for `InputEvent`](https://docs.godotengine.org/en/stable/classes/class_inputevent.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InputEvent>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEvent {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InputEvent`][crate::classes::InputEvent].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InputEvent` methods](https://docs.godotengine.org/en/stable/classes/class_inputevent.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputEvent: crate::obj::GodotClass < Base = InputEvent > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InputEvent {
        pub fn set_device(&mut self, device: i32,) {
            type CallSig = ((), i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "set_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "get_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_action_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsAction < 'a > {
            ExIsAction::new(self, action,)
        }
        pub(crate) fn is_action_pressed_full(&self, action: CowArg < StringName >, allow_echo: bool, exact_match: bool,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >, bool, bool);
            let args = (action, allow_echo, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_pressed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_pressed(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_pressed_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_pressed_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionPressed < 'a > {
            ExIsActionPressed::new(self, action,)
        }
        pub(crate) fn is_action_released_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_released", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_released_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_released(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_released_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_released_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionReleased < 'a > {
            ExIsActionReleased::new(self, action,)
        }
        pub(crate) fn get_action_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallSig < 'a0, > = (f32, CowArg < 'a0, StringName >, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "get_action_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_action_strength_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_action_strength(&self, action: impl AsArg < StringName >,) -> f32 {
            self.get_action_strength_ex(action,) . done()
        }
        #[inline]
        pub fn get_action_strength_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExGetActionStrength < 'a > {
            ExGetActionStrength::new(self, action,)
        }
        pub fn is_canceled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_canceled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_released(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_released", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_echo(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_echo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn as_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "as_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_match_full(&self, event: ObjectArg < crate::classes::InputEvent >, exact_match: bool,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::InputEvent >, bool);
            let args = (event, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_match", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_match_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_match(&self, event: impl AsObjectArg < crate::classes::InputEvent >,) -> bool {
            self.is_match_ex(event,) . done()
        }
        #[inline]
        pub fn is_match_ex < 'a > (&'a self, event: impl AsObjectArg < crate::classes::InputEvent >,) -> ExIsMatch < 'a > {
            ExIsMatch::new(self, event,)
        }
        pub fn is_action_type(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn accumulate(&mut self, with_event: impl AsObjectArg < crate::classes::InputEvent >,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::InputEvent >);
            let args = (with_event.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "accumulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn xformed_by_full(&self, xform: Transform2D, local_ofs: Vector2,) -> Option < Gd < crate::classes::InputEvent > > {
            type CallSig = (Option < Gd < crate::classes::InputEvent > >, Transform2D, Vector2);
            let args = (xform, local_ofs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InputEvent", "xformed_by", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::xformed_by_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn xformed_by(&self, xform: Transform2D,) -> Option < Gd < crate::classes::InputEvent > > {
            self.xformed_by_ex(xform,) . done()
        }
        #[inline]
        pub fn xformed_by_ex < 'a > (&'a self, xform: Transform2D,) -> ExXformedBy < 'a > {
            ExXformedBy::new(self, xform,)
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
        pub const DEVICE_ID_EMULATION: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for InputEvent {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"InputEvent"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEvent {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEvent {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEvent {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEvent {
        
    }
    impl std::ops::Deref for InputEvent {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEvent {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`InputEvent`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_InputEvent {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`InputEvent::is_action_ex`][super::InputEvent::is_action_ex]."]
#[must_use]
pub struct ExIsAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsAction < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
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
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_pressed_ex`][super::InputEvent::is_action_pressed_ex]."]
#[must_use]
pub struct ExIsActionPressed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, action: CowArg < 'a, StringName >, allow_echo: bool, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionPressed < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: impl AsArg < StringName > + 'a,) -> Self {
        let allow_echo = false;
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), allow_echo: allow_echo, exact_match: exact_match,
        }
    }
    #[inline]
    pub fn allow_echo(self, allow_echo: bool) -> Self {
        Self {
            allow_echo: allow_echo, .. self
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
            _phantom, surround_object, action, allow_echo, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_pressed_full(surround_object, action, allow_echo, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_released_ex`][super::InputEvent::is_action_released_ex]."]
#[must_use]
pub struct ExIsActionReleased < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionReleased < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
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
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_released_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::get_action_strength_ex`][super::InputEvent::get_action_strength_ex]."]
#[must_use]
pub struct ExGetActionStrength < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionStrength < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::get_action_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_match_ex`][super::InputEvent::is_match_ex]."]
#[must_use]
pub struct ExIsMatch < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, event: ObjectCow < crate::classes::InputEvent >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsMatch < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, event: impl AsObjectArg < crate::classes::InputEvent >,) -> Self {
        let exact_match = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.consume_arg(), exact_match: exact_match,
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
            _phantom, surround_object, event, exact_match,
        }
        = self;
        re_export::InputEvent::is_match_full(surround_object, event.cow_as_object_arg(), exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::xformed_by_ex`][super::InputEvent::xformed_by_ex]."]
#[must_use]
pub struct ExXformedBy < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, xform: Transform2D, local_ofs: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExXformedBy < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, xform: Transform2D,) -> Self {
        let local_ofs = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, xform: xform, local_ofs: local_ofs,
        }
    }
    #[inline]
    pub fn local_ofs(self, local_ofs: Vector2) -> Self {
        Self {
            local_ofs: local_ofs, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::InputEvent > > {
        let Self {
            _phantom, surround_object, xform, local_ofs,
        }
        = self;
        re_export::InputEvent::xformed_by_full(surround_object, xform, local_ofs,)
    }
}