#![doc = "Sidecar module for class [`TextLine`][crate::classes::TextLine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextLine` enums](https://docs.godotengine.org/en/stable/classes/class_textline.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextLine.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`text_line`][crate::classes::text_line]: sidecar module with related enum/flag types\n* [`ITextLine`][crate::classes::ITextLine]: virtual methods\n\n\nSee also [Godot docs for `TextLine`](https://docs.godotengine.org/en/stable/classes/class_textline.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TextLine::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextLine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextLine`][crate::classes::TextLine].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextLine` methods](https://docs.godotengine.org/en/stable/classes/class_textline.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextLine: crate::obj::GodotClass < Base = TextLine > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TextLine {
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_direction(&mut self, direction: crate::classes::text_server::Direction,) {
            type CallSig = ((), crate::classes::text_server::Direction);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_direction(&self,) -> crate::classes::text_server::Direction {
            type CallSig = (crate::classes::text_server::Direction,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_orientation(&mut self, orientation: crate::classes::text_server::Orientation,) {
            type CallSig = ((), crate::classes::text_server::Orientation);
            let args = (orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_orientation(&self,) -> crate::classes::text_server::Orientation {
            type CallSig = (crate::classes::text_server::Orientation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_preserve_invalid(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_preserve_invalid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_preserve_invalid(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_preserve_invalid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_preserve_control(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_preserve_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_preserve_control(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_preserve_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bidi_override(&mut self, override_: &VariantArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >);
            let args = (RefArg::new(override_),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_string_full(&mut self, text: CowArg < GString >, font: ObjectArg < crate::classes::Font >, font_size: i32, language: CowArg < GString >, meta: RefArg < Variant >,) -> bool {
            type CallSig < 'a0, 'a1, 'a2, > = (bool, CowArg < 'a0, GString >, ObjectArg < crate::classes::Font >, i32, CowArg < 'a1, GString >, RefArg < 'a2, Variant >);
            let args = (text, font, font_size, language, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "add_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_string(&mut self, text: impl AsArg < GString >, font: impl AsObjectArg < crate::classes::Font >, font_size: i32,) -> bool {
            self.add_string_ex(text, font, font_size,) . done()
        }
        #[inline]
        pub fn add_string_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a, font: impl AsObjectArg < crate::classes::Font >, font_size: i32,) -> ExAddString < 'a > {
            ExAddString::new(self, text, font, font_size,)
        }
        pub(crate) fn add_object_full(&mut self, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i32, baseline: f32,) -> bool {
            type CallSig < 'a0, > = (bool, RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, i32, f32);
            let args = (key, size, inline_align, length, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "add_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_object(&mut self, key: &Variant, size: Vector2,) -> bool {
            self.add_object_ex(key, size,) . done()
        }
        #[inline]
        pub fn add_object_ex < 'a > (&'a mut self, key: &'a Variant, size: Vector2,) -> ExAddObject < 'a > {
            ExAddObject::new(self, key, size,)
        }
        pub(crate) fn resize_object_full(&mut self, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f32,) -> bool {
            type CallSig < 'a0, > = (bool, RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, f32);
            let args = (key, size, inline_align, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "resize_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resize_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resize_object(&mut self, key: &Variant, size: Vector2,) -> bool {
            self.resize_object_ex(key, size,) . done()
        }
        #[inline]
        pub fn resize_object_ex < 'a > (&'a mut self, key: &'a Variant, size: Vector2,) -> ExResizeObject < 'a > {
            ExResizeObject::new(self, key, size,)
        }
        pub fn set_width(&mut self, width: f32,) {
            type CallSig = ((), f32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallSig = ((), crate::global::HorizontalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallSig = (crate::global::HorizontalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tab_align(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedFloat32Array >);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "tab_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flags(&mut self, flags: crate::classes::text_server::JustificationFlag,) {
            type CallSig = ((), crate::classes::text_server::JustificationFlag);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallSig = (crate::classes::text_server::JustificationFlag,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallSig = ((), crate::classes::text_server::OverrunBehavior);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallSig = (crate::classes::text_server::OverrunBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ellipsis_char(&mut self, char: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (char.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "set_ellipsis_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ellipsis_char(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_ellipsis_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_objects(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_objects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_object_rect(&self, key: &Variant,) -> Rect2 {
            type CallSig < 'a0, > = (Rect2, RefArg < 'a0, Variant >);
            let args = (RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_object_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rid(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_ascent(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_line_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_descent(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_line_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_width(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_line_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_underline_position(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_line_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_underline_thickness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "get_line_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_full(&self, canvas: Rid, pos: Vector2, color: Color,) {
            type CallSig = ((), Rid, Vector2, Color);
            let args = (canvas, pos, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw(&self, canvas: Rid, pos: Vector2,) {
            self.draw_ex(canvas, pos,) . done()
        }
        #[inline]
        pub fn draw_ex < 'a > (&'a self, canvas: Rid, pos: Vector2,) -> ExDraw < 'a > {
            ExDraw::new(self, canvas, pos,)
        }
        pub(crate) fn draw_outline_full(&self, canvas: Rid, pos: Vector2, outline_size: i32, color: Color,) {
            type CallSig = ((), Rid, Vector2, i32, Color);
            let args = (canvas, pos, outline_size, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "draw_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_outline(&self, canvas: Rid, pos: Vector2,) {
            self.draw_outline_ex(canvas, pos,) . done()
        }
        #[inline]
        pub fn draw_outline_ex < 'a > (&'a self, canvas: Rid, pos: Vector2,) -> ExDrawOutline < 'a > {
            ExDrawOutline::new(self, canvas, pos,)
        }
        pub fn hit_test(&self, coords: f32,) -> i32 {
            type CallSig = (i32, f32);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextLine", "hit_test", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextLine {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TextLine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextLine {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextLine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextLine {
        
    }
    impl crate::obj::cap::GodotDefault for TextLine {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextLine {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextLine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextLine`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TextLine {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextLine > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TextLine::add_string_ex`][super::TextLine::add_string_ex]."]
#[must_use]
pub struct ExAddString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextLine, text: CowArg < 'a, GString >, font: ObjectCow < crate::classes::Font >, font_size: i32, language: CowArg < 'a, GString >, meta: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddString < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, text: impl AsArg < GString > + 'a, font: impl AsObjectArg < crate::classes::Font >, font_size: i32,) -> Self {
        let language = GString::from("");
        let meta = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), font: font.consume_arg(), font_size: font_size, language: CowArg::Owned(language), meta: CowArg::Owned(meta),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn meta(self, meta: &'a Variant) -> Self {
        Self {
            meta: CowArg::Borrowed(meta), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, text, font, font_size, language, meta,
        }
        = self;
        re_export::TextLine::add_string_full(surround_object, text, font.cow_as_object_arg(), font_size, language, meta.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextLine::add_object_ex`][super::TextLine::add_object_ex]."]
#[must_use]
pub struct ExAddObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextLine, key: CowArg < 'a, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i32, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, key: &'a Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let length = 1i32;
        let baseline = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, length: length, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn length(self, length: i32) -> Self {
        Self {
            length: length, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f32) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, key, size, inline_align, length, baseline,
        }
        = self;
        re_export::TextLine::add_object_full(surround_object, key.cow_as_arg(), size, inline_align, length, baseline,)
    }
}
#[doc = "Default-param extender for [`TextLine::resize_object_ex`][super::TextLine::resize_object_ex]."]
#[must_use]
pub struct ExResizeObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextLine, key: CowArg < 'a, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResizeObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, key: &'a Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let baseline = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f32) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, key, size, inline_align, baseline,
        }
        = self;
        re_export::TextLine::resize_object_full(surround_object, key.cow_as_arg(), size, inline_align, baseline,)
    }
}
#[doc = "Default-param extender for [`TextLine::draw_ex`][super::TextLine::draw_ex]."]
#[must_use]
pub struct ExDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDraw < 'a > {
    fn new(surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, color: color,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, color,
        }
        = self;
        re_export::TextLine::draw_full(surround_object, canvas, pos, color,)
    }
}
#[doc = "Default-param extender for [`TextLine::draw_outline_ex`][super::TextLine::draw_outline_ex]."]
#[must_use]
pub struct ExDrawOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2, outline_size: i32, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawOutline < 'a > {
    fn new(surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2,) -> Self {
        let outline_size = 1i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, outline_size: outline_size, color: color,
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, outline_size, color,
        }
        = self;
        re_export::TextLine::draw_outline_full(surround_object, canvas, pos, outline_size, color,)
    }
}