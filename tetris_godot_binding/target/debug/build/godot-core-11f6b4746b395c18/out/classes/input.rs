#![doc = "Sidecar module for class [`Input`][crate::classes::Input].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Input` enums](https://docs.godotengine.org/en/stable/classes/class_input.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Input.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`input`][crate::classes::input]: sidecar module with related enum/flag types\n* [`IInput`][crate::classes::IInput]: virtual methods\n\n\nSee also [Godot docs for `Input`](https://docs.godotengine.org/en/stable/classes/class_input.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Input::singleton()`][Input::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Input {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Input`][crate::classes::Input].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Input` methods](https://docs.godotengine.org/en/stable/classes/class_input.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInput: crate::obj::GodotClass < Base = Input > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Input {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"Input");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn is_anything_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_anything_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_key_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallSig = (bool, crate::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_key_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_physical_key_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallSig = (bool, crate::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_physical_key_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_key_label_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallSig = (bool, crate::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_key_label_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_mouse_button_pressed(&self, button: crate::global::MouseButton,) -> bool {
            type CallSig = (bool, crate::global::MouseButton);
            let args = (button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_mouse_button_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_joy_button_pressed(&self, device: i32, button: crate::global::JoyButton,) -> bool {
            type CallSig = (bool, i32, crate::global::JoyButton);
            let args = (device, button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_joy_button_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_action_pressed_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_action_pressed", self.object_ptr, self.__checked_id(), args,)
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
        pub(crate) fn is_action_just_pressed_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_action_just_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_just_pressed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_just_pressed(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_just_pressed_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_just_pressed_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionJustPressed < 'a > {
            ExIsActionJustPressed::new(self, action,)
        }
        pub(crate) fn is_action_just_released_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_action_just_released", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_just_released_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_just_released(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_just_released_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_just_released_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionJustReleased < 'a > {
            ExIsActionJustReleased::new(self, action,)
        }
        pub(crate) fn get_action_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallSig < 'a0, > = (f32, CowArg < 'a0, StringName >, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_action_strength", self.object_ptr, self.__checked_id(), args,)
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
        pub(crate) fn get_action_raw_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallSig < 'a0, > = (f32, CowArg < 'a0, StringName >, bool);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_action_raw_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_action_raw_strength_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_action_raw_strength(&self, action: impl AsArg < StringName >,) -> f32 {
            self.get_action_raw_strength_ex(action,) . done()
        }
        #[inline]
        pub fn get_action_raw_strength_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExGetActionRawStrength < 'a > {
            ExGetActionRawStrength::new(self, action,)
        }
        pub fn get_axis(&self, negative_action: impl AsArg < StringName >, positive_action: impl AsArg < StringName >,) -> f32 {
            type CallSig < 'a0, 'a1, > = (f32, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (negative_action.into_arg(), positive_action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_vector_full(&self, negative_x: CowArg < StringName >, positive_x: CowArg < StringName >, negative_y: CowArg < StringName >, positive_y: CowArg < StringName >, deadzone: f32,) -> Vector2 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (Vector2, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >, CowArg < 'a3, StringName >, f32);
            let args = (negative_x, positive_x, negative_y, positive_y, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_vector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_vector_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_vector(&self, negative_x: impl AsArg < StringName >, positive_x: impl AsArg < StringName >, negative_y: impl AsArg < StringName >, positive_y: impl AsArg < StringName >,) -> Vector2 {
            self.get_vector_ex(negative_x, positive_x, negative_y, positive_y,) . done()
        }
        #[inline]
        pub fn get_vector_ex < 'a > (&'a self, negative_x: impl AsArg < StringName > + 'a, positive_x: impl AsArg < StringName > + 'a, negative_y: impl AsArg < StringName > + 'a, positive_y: impl AsArg < StringName > + 'a,) -> ExGetVector < 'a > {
            ExGetVector::new(self, negative_x, positive_x, negative_y, positive_y,)
        }
        pub(crate) fn add_joy_mapping_full(&mut self, mapping: CowArg < GString >, update_existing: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (mapping, update_existing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "add_joy_mapping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_joy_mapping_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_joy_mapping(&mut self, mapping: impl AsArg < GString >,) {
            self.add_joy_mapping_ex(mapping,) . done()
        }
        #[inline]
        pub fn add_joy_mapping_ex < 'a > (&'a mut self, mapping: impl AsArg < GString > + 'a,) -> ExAddJoyMapping < 'a > {
            ExAddJoyMapping::new(self, mapping,)
        }
        pub fn remove_joy_mapping(&mut self, guid: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (guid.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "remove_joy_mapping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_joy_known(&mut self, device: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_joy_known", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_axis(&self, device: i32, axis: crate::global::JoyAxis,) -> f32 {
            type CallSig = (f32, i32, crate::global::JoyAxis);
            let args = (device, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_joy_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_name(&mut self, device: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_joy_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_guid(&self, device: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_joy_guid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_info(&self, device: i32,) -> Dictionary {
            type CallSig = (Dictionary, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_joy_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn should_ignore_device(&self, vendor_id: i32, product_id: i32,) -> bool {
            type CallSig = (bool, i32, i32);
            let args = (vendor_id, product_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "should_ignore_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connected_joypads(&mut self,) -> Array < i64 > {
            type CallSig = (Array < i64 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_connected_joypads", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_vibration_strength(&mut self, device: i32,) -> Vector2 {
            type CallSig = (Vector2, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_joy_vibration_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joy_vibration_duration(&mut self, device: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_joy_vibration_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn start_joy_vibration_full(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32, duration: f32,) {
            type CallSig = ((), i32, f32, f32, f32);
            let args = (device, weak_magnitude, strong_magnitude, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "start_joy_vibration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::start_joy_vibration_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn start_joy_vibration(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32,) {
            self.start_joy_vibration_ex(device, weak_magnitude, strong_magnitude,) . done()
        }
        #[inline]
        pub fn start_joy_vibration_ex < 'a > (&'a mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32,) -> ExStartJoyVibration < 'a > {
            ExStartJoyVibration::new(self, device, weak_magnitude, strong_magnitude,)
        }
        pub fn stop_joy_vibration(&mut self, device: i32,) {
            type CallSig = ((), i32);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "stop_joy_vibration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn vibrate_handheld_full(&mut self, duration_ms: i32, amplitude: f32,) {
            type CallSig = ((), i32, f32);
            let args = (duration_ms, amplitude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "vibrate_handheld", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::vibrate_handheld_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn vibrate_handheld(&mut self,) {
            self.vibrate_handheld_ex() . done()
        }
        #[inline]
        pub fn vibrate_handheld_ex < 'a > (&'a mut self,) -> ExVibrateHandheld < 'a > {
            ExVibrateHandheld::new(self,)
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_accelerometer(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_accelerometer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_magnetometer(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_magnetometer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gyroscope(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_gyroscope", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, value: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_accelerometer(&mut self, value: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_accelerometer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_magnetometer(&mut self, value: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_magnetometer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gyroscope(&mut self, value: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_gyroscope", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_mouse_velocity(&mut self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_last_mouse_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_mouse_screen_velocity(&mut self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_last_mouse_screen_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mouse_button_mask(&self,) -> crate::global::MouseButtonMask {
            type CallSig = (crate::global::MouseButtonMask,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_mouse_button_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mouse_mode(&mut self, mode: crate::classes::input::MouseMode,) {
            type CallSig = ((), crate::classes::input::MouseMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_mouse_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mouse_mode(&self,) -> crate::classes::input::MouseMode {
            type CallSig = (crate::classes::input::MouseMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_mouse_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "warp_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn action_press_full(&mut self, action: CowArg < StringName >, strength: f32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, f32);
            let args = (action, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "action_press", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::action_press_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn action_press(&mut self, action: impl AsArg < StringName >,) {
            self.action_press_ex(action,) . done()
        }
        #[inline]
        pub fn action_press_ex < 'a > (&'a mut self, action: impl AsArg < StringName > + 'a,) -> ExActionPress < 'a > {
            ExActionPress::new(self, action,)
        }
        pub fn action_release(&mut self, action: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "action_release", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_default_cursor_shape_full(&mut self, shape: crate::classes::input::CursorShape,) {
            type CallSig = ((), crate::classes::input::CursorShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_default_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_default_cursor_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_default_cursor_shape(&mut self,) {
            self.set_default_cursor_shape_ex() . done()
        }
        #[inline]
        pub fn set_default_cursor_shape_ex < 'a > (&'a mut self,) -> ExSetDefaultCursorShape < 'a > {
            ExSetDefaultCursorShape::new(self,)
        }
        pub fn get_current_cursor_shape(&self,) -> crate::classes::input::CursorShape {
            type CallSig = (crate::classes::input::CursorShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "get_current_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_custom_mouse_cursor_full(&mut self, image: ObjectArg < crate::classes::Resource >, shape: crate::classes::input::CursorShape, hotspot: Vector2,) {
            type CallSig = ((), ObjectArg < crate::classes::Resource >, crate::classes::input::CursorShape, Vector2);
            let args = (image, shape, hotspot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_custom_mouse_cursor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_custom_mouse_cursor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_custom_mouse_cursor(&mut self, image: impl AsObjectArg < crate::classes::Resource >,) {
            self.set_custom_mouse_cursor_ex(image,) . done()
        }
        #[inline]
        pub fn set_custom_mouse_cursor_ex < 'a > (&'a mut self, image: impl AsObjectArg < crate::classes::Resource >,) -> ExSetCustomMouseCursor < 'a > {
            ExSetCustomMouseCursor::new(self, image,)
        }
        pub fn parse_input_event(&mut self, event: impl AsObjectArg < crate::classes::InputEvent >,) {
            type CallSig = ((), ObjectArg < crate::classes::InputEvent >);
            let args = (event.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "parse_input_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_accumulated_input(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_use_accumulated_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_accumulated_input(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_using_accumulated_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn flush_buffered_events(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "flush_buffered_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emulate_mouse_from_touch(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_emulate_mouse_from_touch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emulating_mouse_from_touch(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_emulating_mouse_from_touch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emulate_touch_from_mouse(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "set_emulate_touch_from_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emulating_touch_from_mouse(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Input", "is_emulating_touch_from_mouse", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Input {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Input"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Input {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Input {
        
    }
    impl std::ops::Deref for Input {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Input {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Input`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Input {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Input > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Input::is_action_pressed_ex`][super::Input::is_action_pressed_ex]."]
#[must_use]
pub struct ExIsActionPressed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionPressed < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
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
        re_export::Input::is_action_pressed_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_pressed_ex`][super::Input::is_action_just_pressed_ex]."]
#[must_use]
pub struct ExIsActionJustPressed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionJustPressed < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
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
        re_export::Input::is_action_just_pressed_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_released_ex`][super::Input::is_action_just_released_ex]."]
#[must_use]
pub struct ExIsActionJustReleased < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionJustReleased < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
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
        re_export::Input::is_action_just_released_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_action_strength_ex`][super::Input::get_action_strength_ex]."]
#[must_use]
pub struct ExGetActionStrength < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionStrength < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
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
        re_export::Input::get_action_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_action_raw_strength_ex`][super::Input::get_action_raw_strength_ex]."]
#[must_use]
pub struct ExGetActionRawStrength < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionRawStrength < 'a > {
    fn new(surround_object: &'a re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
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
        re_export::Input::get_action_raw_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_vector_ex`][super::Input::get_vector_ex]."]
#[must_use]
pub struct ExGetVector < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Input, negative_x: CowArg < 'a, StringName >, positive_x: CowArg < 'a, StringName >, negative_y: CowArg < 'a, StringName >, positive_y: CowArg < 'a, StringName >, deadzone: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVector < 'a > {
    fn new(surround_object: &'a re_export::Input, negative_x: impl AsArg < StringName > + 'a, positive_x: impl AsArg < StringName > + 'a, negative_y: impl AsArg < StringName > + 'a, positive_y: impl AsArg < StringName > + 'a,) -> Self {
        let deadzone = - 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, negative_x: negative_x.into_arg(), positive_x: positive_x.into_arg(), negative_y: negative_y.into_arg(), positive_y: positive_y.into_arg(), deadzone: deadzone,
        }
    }
    #[inline]
    pub fn deadzone(self, deadzone: f32) -> Self {
        Self {
            deadzone: deadzone, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, negative_x, positive_x, negative_y, positive_y, deadzone,
        }
        = self;
        re_export::Input::get_vector_full(surround_object, negative_x, positive_x, negative_y, positive_y, deadzone,)
    }
}
#[doc = "Default-param extender for [`Input::add_joy_mapping_ex`][super::Input::add_joy_mapping_ex]."]
#[must_use]
pub struct ExAddJoyMapping < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, mapping: CowArg < 'a, GString >, update_existing: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddJoyMapping < 'a > {
    fn new(surround_object: &'a mut re_export::Input, mapping: impl AsArg < GString > + 'a,) -> Self {
        let update_existing = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mapping: mapping.into_arg(), update_existing: update_existing,
        }
    }
    #[inline]
    pub fn update_existing(self, update_existing: bool) -> Self {
        Self {
            update_existing: update_existing, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mapping, update_existing,
        }
        = self;
        re_export::Input::add_joy_mapping_full(surround_object, mapping, update_existing,)
    }
}
#[doc = "Default-param extender for [`Input::start_joy_vibration_ex`][super::Input::start_joy_vibration_ex]."]
#[must_use]
pub struct ExStartJoyVibration < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, device: i32, weak_magnitude: f32, strong_magnitude: f32, duration: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStartJoyVibration < 'a > {
    fn new(surround_object: &'a mut re_export::Input, device: i32, weak_magnitude: f32, strong_magnitude: f32,) -> Self {
        let duration = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, device: device, weak_magnitude: weak_magnitude, strong_magnitude: strong_magnitude, duration: duration,
        }
    }
    #[inline]
    pub fn duration(self, duration: f32) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, device, weak_magnitude, strong_magnitude, duration,
        }
        = self;
        re_export::Input::start_joy_vibration_full(surround_object, device, weak_magnitude, strong_magnitude, duration,)
    }
}
#[doc = "Default-param extender for [`Input::vibrate_handheld_ex`][super::Input::vibrate_handheld_ex]."]
#[must_use]
pub struct ExVibrateHandheld < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, duration_ms: i32, amplitude: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVibrateHandheld < 'a > {
    fn new(surround_object: &'a mut re_export::Input,) -> Self {
        let duration_ms = 500i32;
        let amplitude = - 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, duration_ms: duration_ms, amplitude: amplitude,
        }
    }
    #[inline]
    pub fn duration_ms(self, duration_ms: i32) -> Self {
        Self {
            duration_ms: duration_ms, .. self
        }
    }
    #[inline]
    pub fn amplitude(self, amplitude: f32) -> Self {
        Self {
            amplitude: amplitude, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, duration_ms, amplitude,
        }
        = self;
        re_export::Input::vibrate_handheld_full(surround_object, duration_ms, amplitude,)
    }
}
#[doc = "Default-param extender for [`Input::action_press_ex`][super::Input::action_press_ex]."]
#[must_use]
pub struct ExActionPress < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, action: CowArg < 'a, StringName >, strength: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExActionPress < 'a > {
    fn new(surround_object: &'a mut re_export::Input, action: impl AsArg < StringName > + 'a,) -> Self {
        let strength = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), strength: strength,
        }
    }
    #[inline]
    pub fn strength(self, strength: f32) -> Self {
        Self {
            strength: strength, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, action, strength,
        }
        = self;
        re_export::Input::action_press_full(surround_object, action, strength,)
    }
}
#[doc = "Default-param extender for [`Input::set_default_cursor_shape_ex`][super::Input::set_default_cursor_shape_ex]."]
#[must_use]
pub struct ExSetDefaultCursorShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, shape: crate::classes::input::CursorShape,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetDefaultCursorShape < 'a > {
    fn new(surround_object: &'a mut re_export::Input,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shape: shape,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::input::CursorShape) -> Self {
        Self {
            shape: shape, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shape,
        }
        = self;
        re_export::Input::set_default_cursor_shape_full(surround_object, shape,)
    }
}
#[doc = "Default-param extender for [`Input::set_custom_mouse_cursor_ex`][super::Input::set_custom_mouse_cursor_ex]."]
#[must_use]
pub struct ExSetCustomMouseCursor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Input, image: ObjectCow < crate::classes::Resource >, shape: crate::classes::input::CursorShape, hotspot: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCustomMouseCursor < 'a > {
    fn new(surround_object: &'a mut re_export::Input, image: impl AsObjectArg < crate::classes::Resource >,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        let hotspot = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.consume_arg(), shape: shape, hotspot: hotspot,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::input::CursorShape) -> Self {
        Self {
            shape: shape, .. self
        }
    }
    #[inline]
    pub fn hotspot(self, hotspot: Vector2) -> Self {
        Self {
            hotspot: hotspot, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, shape, hotspot,
        }
        = self;
        re_export::Input::set_custom_mouse_cursor_full(surround_object, image.cow_as_object_arg(), shape, hotspot,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseMode {
    ord: i32
}
impl MouseMode {
    #[doc(alias = "MOUSE_MODE_VISIBLE")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_VISIBLE`"]
    pub const VISIBLE: MouseMode = MouseMode {
        ord: 0i32
    };
    #[doc(alias = "MOUSE_MODE_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_HIDDEN`"]
    pub const HIDDEN: MouseMode = MouseMode {
        ord: 1i32
    };
    #[doc(alias = "MOUSE_MODE_CAPTURED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CAPTURED`"]
    pub const CAPTURED: MouseMode = MouseMode {
        ord: 2i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED`"]
    pub const CONFINED: MouseMode = MouseMode {
        ord: 3i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED_HIDDEN`"]
    pub const CONFINED_HIDDEN: MouseMode = MouseMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for MouseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MouseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MouseMode {
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
            Self::VISIBLE => "VISIBLE", Self::HIDDEN => "HIDDEN", Self::CAPTURED => "CAPTURED", Self::CONFINED => "CONFINED", Self::CONFINED_HIDDEN => "CONFINED_HIDDEN", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VISIBLE => "MOUSE_MODE_VISIBLE", Self::HIDDEN => "MOUSE_MODE_HIDDEN", Self::CAPTURED => "MOUSE_MODE_CAPTURED", Self::CONFINED => "MOUSE_MODE_CONFINED", Self::CONFINED_HIDDEN => "MOUSE_MODE_CONFINED_HIDDEN", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for MouseMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MouseMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MouseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CursorShape {
    ord: i32
}
impl CursorShape {
    #[doc(alias = "CURSOR_ARROW")]
    #[doc = "Godot enumerator name: `CURSOR_ARROW`"]
    pub const ARROW: CursorShape = CursorShape {
        ord: 0i32
    };
    #[doc(alias = "CURSOR_IBEAM")]
    #[doc = "Godot enumerator name: `CURSOR_IBEAM`"]
    pub const IBEAM: CursorShape = CursorShape {
        ord: 1i32
    };
    #[doc(alias = "CURSOR_POINTING_HAND")]
    #[doc = "Godot enumerator name: `CURSOR_POINTING_HAND`"]
    pub const POINTING_HAND: CursorShape = CursorShape {
        ord: 2i32
    };
    #[doc(alias = "CURSOR_CROSS")]
    #[doc = "Godot enumerator name: `CURSOR_CROSS`"]
    pub const CROSS: CursorShape = CursorShape {
        ord: 3i32
    };
    #[doc(alias = "CURSOR_WAIT")]
    #[doc = "Godot enumerator name: `CURSOR_WAIT`"]
    pub const WAIT: CursorShape = CursorShape {
        ord: 4i32
    };
    #[doc(alias = "CURSOR_BUSY")]
    #[doc = "Godot enumerator name: `CURSOR_BUSY`"]
    pub const BUSY: CursorShape = CursorShape {
        ord: 5i32
    };
    #[doc(alias = "CURSOR_DRAG")]
    #[doc = "Godot enumerator name: `CURSOR_DRAG`"]
    pub const DRAG: CursorShape = CursorShape {
        ord: 6i32
    };
    #[doc(alias = "CURSOR_CAN_DROP")]
    #[doc = "Godot enumerator name: `CURSOR_CAN_DROP`"]
    pub const CAN_DROP: CursorShape = CursorShape {
        ord: 7i32
    };
    #[doc(alias = "CURSOR_FORBIDDEN")]
    #[doc = "Godot enumerator name: `CURSOR_FORBIDDEN`"]
    pub const FORBIDDEN: CursorShape = CursorShape {
        ord: 8i32
    };
    #[doc(alias = "CURSOR_VSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_VSIZE`"]
    pub const VSIZE: CursorShape = CursorShape {
        ord: 9i32
    };
    #[doc(alias = "CURSOR_HSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_HSIZE`"]
    pub const HSIZE: CursorShape = CursorShape {
        ord: 10i32
    };
    #[doc(alias = "CURSOR_BDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_BDIAGSIZE`"]
    pub const BDIAGSIZE: CursorShape = CursorShape {
        ord: 11i32
    };
    #[doc(alias = "CURSOR_FDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_FDIAGSIZE`"]
    pub const FDIAGSIZE: CursorShape = CursorShape {
        ord: 12i32
    };
    #[doc(alias = "CURSOR_MOVE")]
    #[doc = "Godot enumerator name: `CURSOR_MOVE`"]
    pub const MOVE: CursorShape = CursorShape {
        ord: 13i32
    };
    #[doc(alias = "CURSOR_VSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_VSPLIT`"]
    pub const VSPLIT: CursorShape = CursorShape {
        ord: 14i32
    };
    #[doc(alias = "CURSOR_HSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_HSPLIT`"]
    pub const HSPLIT: CursorShape = CursorShape {
        ord: 15i32
    };
    #[doc(alias = "CURSOR_HELP")]
    #[doc = "Godot enumerator name: `CURSOR_HELP`"]
    pub const HELP: CursorShape = CursorShape {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for CursorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CursorShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CursorShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 => Some(Self {
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
            Self::ARROW => "ARROW", Self::IBEAM => "IBEAM", Self::POINTING_HAND => "POINTING_HAND", Self::CROSS => "CROSS", Self::WAIT => "WAIT", Self::BUSY => "BUSY", Self::DRAG => "DRAG", Self::CAN_DROP => "CAN_DROP", Self::FORBIDDEN => "FORBIDDEN", Self::VSIZE => "VSIZE", Self::HSIZE => "HSIZE", Self::BDIAGSIZE => "BDIAGSIZE", Self::FDIAGSIZE => "FDIAGSIZE", Self::MOVE => "MOVE", Self::VSPLIT => "VSPLIT", Self::HSPLIT => "HSPLIT", Self::HELP => "HELP", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ARROW => "CURSOR_ARROW", Self::IBEAM => "CURSOR_IBEAM", Self::POINTING_HAND => "CURSOR_POINTING_HAND", Self::CROSS => "CURSOR_CROSS", Self::WAIT => "CURSOR_WAIT", Self::BUSY => "CURSOR_BUSY", Self::DRAG => "CURSOR_DRAG", Self::CAN_DROP => "CURSOR_CAN_DROP", Self::FORBIDDEN => "CURSOR_FORBIDDEN", Self::VSIZE => "CURSOR_VSIZE", Self::HSIZE => "CURSOR_HSIZE", Self::BDIAGSIZE => "CURSOR_BDIAGSIZE", Self::FDIAGSIZE => "CURSOR_FDIAGSIZE", Self::MOVE => "CURSOR_MOVE", Self::VSPLIT => "CURSOR_VSPLIT", Self::HSPLIT => "CURSOR_HSPLIT", Self::HELP => "CURSOR_HELP", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CursorShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CursorShape {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CursorShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}