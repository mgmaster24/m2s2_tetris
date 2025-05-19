#![doc = "Sidecar module for class [`Label`][crate::classes::Label].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Label` enums](https://docs.godotengine.org/en/stable/classes/class_label.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Label.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`label`][crate::classes::label]: sidecar module with related enum/flag types\n* [`ILabel`][crate::classes::ILabel]: virtual methods\n\n\nSee also [Godot docs for `Label`](https://docs.godotengine.org/en/stable/classes/class_label.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Label::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Label {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Label`][crate::classes::Label].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Label` methods](https://docs.godotengine.org/en/stable/classes/class_label.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILabel: crate::obj::GodotClass < Base = Label > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
    }
    impl Label {
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallSig = ((), crate::global::HorizontalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallSig = (crate::global::HorizontalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::global::VerticalAlignment,) {
            type CallSig = ((), crate::global::VerticalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_vertical_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallSig = (crate::global::VerticalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_vertical_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_label_settings(&mut self, settings: impl AsObjectArg < crate::classes::LabelSettings >,) {
            type CallSig = ((), ObjectArg < crate::classes::LabelSettings >);
            let args = (settings.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_label_settings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_label_settings(&self,) -> Option < Gd < crate::classes::LabelSettings > > {
            type CallSig = (Option < Gd < crate::classes::LabelSettings > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_label_settings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallSig = ((), crate::classes::control::TextDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallSig = (crate::classes::control::TextDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallSig = ((), crate::classes::text_server::AutowrapMode);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallSig = (crate::classes::text_server::AutowrapMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::classes::text_server::JustificationFlag,) {
            type CallSig = ((), crate::classes::text_server::JustificationFlag);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_justification_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallSig = (crate::classes::text_server::JustificationFlag,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_justification_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_text(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_clip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_clipping_text(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "is_clipping_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_stops(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedFloat32Array >);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_tab_stops", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_stops(&self,) -> PackedFloat32Array {
            type CallSig = (PackedFloat32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_tab_stops", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallSig = ((), crate::classes::text_server::OverrunBehavior);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallSig = (crate::classes::text_server::OverrunBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ellipsis_char(&mut self, char: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (char.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_ellipsis_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ellipsis_char(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_ellipsis_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uppercase(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_uppercase", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_uppercase(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "is_uppercase", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_line_height_full(&self, line: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_line_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_line_height_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_line_height(&self,) -> i32 {
            self.get_line_height_ex() . done()
        }
        #[inline]
        pub fn get_line_height_ex < 'a > (&'a self,) -> ExGetLineHeight < 'a > {
            ExGetLineHeight::new(self,)
        }
        pub fn get_line_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_line_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_visible_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_character_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_total_character_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_characters(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_visible_characters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_characters(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_visible_characters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_characters_behavior(&self,) -> crate::classes::text_server::VisibleCharactersBehavior {
            type CallSig = (crate::classes::text_server::VisibleCharactersBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_visible_characters_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_characters_behavior(&mut self, behavior: crate::classes::text_server::VisibleCharactersBehavior,) {
            type CallSig = ((), crate::classes::text_server::VisibleCharactersBehavior);
            let args = (behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_visible_characters_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_visible_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_visible_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lines_skipped(&mut self, lines_skipped: i32,) {
            type CallSig = ((), i32);
            let args = (lines_skipped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_lines_skipped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lines_skipped(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_lines_skipped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_lines_visible(&mut self, lines_visible: i32,) {
            type CallSig = ((), i32);
            let args = (lines_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_max_lines_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_lines_visible(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_max_lines_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallSig = ((), crate::classes::text_server::StructuredTextParser);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallSig = (crate::classes::text_server::StructuredTextParser,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VariantArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_character_bounds(&self, pos: i32,) -> Rect2 {
            type CallSig = (Rect2, i32);
            let args = (pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Label", "get_character_bounds", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Label {
        type Base = crate::classes::Control;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Label"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Label {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Label {
        
    }
    impl crate::obj::cap::GodotDefault for Label {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Label {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Label {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Label`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Label {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Label > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Label::get_line_height_ex`][super::Label::get_line_height_ex]."]
#[must_use]
pub struct ExGetLineHeight < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Label, line: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetLineHeight < 'a > {
    fn new(surround_object: &'a re_export::Label,) -> Self {
        let line = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line,
        }
    }
    #[inline]
    pub fn line(self, line: i32) -> Self {
        Self {
            line: line, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line,
        }
        = self;
        re_export::Label::get_line_height_full(surround_object, line,)
    }
}