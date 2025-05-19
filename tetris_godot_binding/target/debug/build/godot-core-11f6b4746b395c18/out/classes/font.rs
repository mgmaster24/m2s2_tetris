#![doc = "Sidecar module for class [`Font`][crate::classes::Font].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Font` enums](https://docs.godotengine.org/en/stable/classes/class_font.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Font.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`font`][crate::classes::font]: sidecar module with related enum/flag types\n* [`IFont`][crate::classes::IFont]: virtual methods\n\n\nSee also [Godot docs for `Font`](https://docs.godotengine.org/en/stable/classes/class_font.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Font>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Font {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Font`][crate::classes::Font].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Font` methods](https://docs.godotengine.org/en/stable/classes/class_font.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFont: crate::obj::GodotClass < Base = Font > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Font {
        pub fn set_fallbacks(&mut self, fallbacks: &Array < Gd < crate::classes::Font > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::Font > > >);
            let args = (RefArg::new(fallbacks),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "set_fallbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fallbacks(&self,) -> Array < Gd < crate::classes::Font > > {
            type CallSig = (Array < Gd < crate::classes::Font > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_fallbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn find_variation_full(&self, variation_coordinates: RefArg < Dictionary >, face_index: i32, strength: f32, transform: Transform2D, spacing_top: i32, spacing_bottom: i32, spacing_space: i32, spacing_glyph: i32, baseline_offset: f32,) -> Rid {
            type CallSig < 'a0, > = (Rid, RefArg < 'a0, Dictionary >, i32, f32, Transform2D, i32, i32, i32, i32, f32);
            let args = (variation_coordinates, face_index, strength, transform, spacing_top, spacing_bottom, spacing_space, spacing_glyph, baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "find_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::find_variation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn find_variation(&self, variation_coordinates: &Dictionary,) -> Rid {
            self.find_variation_ex(variation_coordinates,) . done()
        }
        #[inline]
        pub fn find_variation_ex < 'a > (&'a self, variation_coordinates: &'a Dictionary,) -> ExFindVariation < 'a > {
            ExFindVariation::new(self, variation_coordinates,)
        }
        pub fn get_rids(&self,) -> Array < Rid > {
            type CallSig = (Array < Rid >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_rids", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_height_full(&self, font_size: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_height_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_height(&self,) -> f32 {
            self.get_height_ex() . done()
        }
        #[inline]
        pub fn get_height_ex < 'a > (&'a self,) -> ExGetHeight < 'a > {
            ExGetHeight::new(self,)
        }
        pub(crate) fn get_ascent_full(&self, font_size: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_ascent_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_ascent(&self,) -> f32 {
            self.get_ascent_ex() . done()
        }
        #[inline]
        pub fn get_ascent_ex < 'a > (&'a self,) -> ExGetAscent < 'a > {
            ExGetAscent::new(self,)
        }
        pub(crate) fn get_descent_full(&self, font_size: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_descent_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_descent(&self,) -> f32 {
            self.get_descent_ex() . done()
        }
        #[inline]
        pub fn get_descent_ex < 'a > (&'a self,) -> ExGetDescent < 'a > {
            ExGetDescent::new(self,)
        }
        pub(crate) fn get_underline_position_full(&self, font_size: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_underline_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_underline_position(&self,) -> f32 {
            self.get_underline_position_ex() . done()
        }
        #[inline]
        pub fn get_underline_position_ex < 'a > (&'a self,) -> ExGetUnderlinePosition < 'a > {
            ExGetUnderlinePosition::new(self,)
        }
        pub(crate) fn get_underline_thickness_full(&self, font_size: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_underline_thickness_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_underline_thickness(&self,) -> f32 {
            self.get_underline_thickness_ex() . done()
        }
        #[inline]
        pub fn get_underline_thickness_ex < 'a > (&'a self,) -> ExGetUnderlineThickness < 'a > {
            ExGetUnderlineThickness::new(self,)
        }
        pub fn get_font_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_font_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_style_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_font_style_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ot_name_strings(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_ot_name_strings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_style(&self,) -> crate::classes::text_server::FontStyle {
            type CallSig = (crate::classes::text_server::FontStyle,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_font_style", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_weight(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_font_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_stretch(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_font_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spacing(&self, spacing: crate::classes::text_server::SpacingType,) -> i32 {
            type CallSig = (i32, crate::classes::text_server::SpacingType);
            let args = (spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_opentype_features(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_opentype_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_capacity(&mut self, single_line: i32, multi_line: i32,) {
            type CallSig = ((), i32, i32);
            let args = (single_line, multi_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "set_cache_capacity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_string_size_full(&self, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) -> Vector2 {
            type CallSig < 'a0, > = (Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation);
            let args = (text, alignment, width, font_size, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_string_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_string_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_string_size(&self, text: impl AsArg < GString >,) -> Vector2 {
            self.get_string_size_ex(text,) . done()
        }
        #[inline]
        pub fn get_string_size_ex < 'a > (&'a self, text: impl AsArg < GString > + 'a,) -> ExGetStringSize < 'a > {
            ExGetStringSize::new(self, text,)
        }
        pub(crate) fn get_multiline_string_size_full(&self, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) -> Vector2 {
            type CallSig < 'a0, > = (Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, i32, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation);
            let args = (text, alignment, width, font_size, max_lines, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_multiline_string_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_multiline_string_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_multiline_string_size(&self, text: impl AsArg < GString >,) -> Vector2 {
            self.get_multiline_string_size_ex(text,) . done()
        }
        #[inline]
        pub fn get_multiline_string_size_ex < 'a > (&'a self, text: impl AsArg < GString > + 'a,) -> ExGetMultilineStringSize < 'a > {
            ExGetMultilineStringSize::new(self, text,)
        }
        pub(crate) fn draw_string_full(&self, canvas_item: Rid, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) {
            type CallSig < 'a0, > = ((), Rid, Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, Color, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation);
            let args = (canvas_item, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "draw_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_string(&self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_string_ex(canvas_item, pos, text,) . done()
        }
        #[inline]
        pub fn draw_string_ex < 'a > (&'a self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'a,) -> ExDrawString < 'a > {
            ExDrawString::new(self, canvas_item, pos, text,)
        }
        pub(crate) fn draw_multiline_string_full(&self, canvas_item: Rid, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) {
            type CallSig < 'a0, > = ((), Rid, Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, i32, Color, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation);
            let args = (canvas_item, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "draw_multiline_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_multiline_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_multiline_string(&self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_multiline_string_ex(canvas_item, pos, text,) . done()
        }
        #[inline]
        pub fn draw_multiline_string_ex < 'a > (&'a self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'a,) -> ExDrawMultilineString < 'a > {
            ExDrawMultilineString::new(self, canvas_item, pos, text,)
        }
        pub(crate) fn draw_string_outline_full(&self, canvas_item: Rid, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) {
            type CallSig < 'a0, > = ((), Rid, Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, i32, Color, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation);
            let args = (canvas_item, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "draw_string_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_string_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_string_outline(&self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_string_outline_ex(canvas_item, pos, text,) . done()
        }
        #[inline]
        pub fn draw_string_outline_ex < 'a > (&'a self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'a,) -> ExDrawStringOutline < 'a > {
            ExDrawStringOutline::new(self, canvas_item, pos, text,)
        }
        pub(crate) fn draw_multiline_string_outline_full(&self, canvas_item: Rid, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) {
            type CallSig < 'a0, > = ((), Rid, Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, i32, i32, Color, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation);
            let args = (canvas_item, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "draw_multiline_string_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_multiline_string_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_multiline_string_outline(&self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_multiline_string_outline_ex(canvas_item, pos, text,) . done()
        }
        #[inline]
        pub fn draw_multiline_string_outline_ex < 'a > (&'a self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'a,) -> ExDrawMultilineStringOutline < 'a > {
            ExDrawMultilineStringOutline::new(self, canvas_item, pos, text,)
        }
        pub fn get_char_size(&self, char: i64, font_size: i32,) -> Vector2 {
            type CallSig = (Vector2, i64, i32);
            let args = (char, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_char_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_char_full(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32, modulate: Color,) -> f32 {
            type CallSig = (f32, Rid, Vector2, i64, i32, Color);
            let args = (canvas_item, pos, char, font_size, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "draw_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_char_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_char(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> f32 {
            self.draw_char_ex(canvas_item, pos, char, font_size,) . done()
        }
        #[inline]
        pub fn draw_char_ex < 'a > (&'a self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> ExDrawChar < 'a > {
            ExDrawChar::new(self, canvas_item, pos, char, font_size,)
        }
        pub(crate) fn draw_char_outline_full(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32, size: i32, modulate: Color,) -> f32 {
            type CallSig = (f32, Rid, Vector2, i64, i32, i32, Color);
            let args = (canvas_item, pos, char, font_size, size, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "draw_char_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_char_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_char_outline(&self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> f32 {
            self.draw_char_outline_ex(canvas_item, pos, char, font_size,) . done()
        }
        #[inline]
        pub fn draw_char_outline_ex < 'a > (&'a self, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> ExDrawCharOutline < 'a > {
            ExDrawCharOutline::new(self, canvas_item, pos, char, font_size,)
        }
        pub fn has_char(&self, char: i64,) -> bool {
            type CallSig = (bool, i64);
            let args = (char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "has_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_chars(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_supported_chars", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_language_supported(&self, language: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "is_language_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_script_supported(&self, script: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "is_script_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_feature_list(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_supported_feature_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_variation_list(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_supported_variation_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_count(&self,) -> i64 {
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Font", "get_face_count", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Font {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Font"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Font {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Font {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Font {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Font {
        
    }
    impl std::ops::Deref for Font {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Font {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Font`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Font {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Font > for $Class {
                
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
#[doc = "Default-param extender for [`Font::find_variation_ex`][super::Font::find_variation_ex]."]
#[must_use]
pub struct ExFindVariation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, variation_coordinates: CowArg < 'a, Dictionary >, face_index: i32, strength: f32, transform: Transform2D, spacing_top: i32, spacing_bottom: i32, spacing_space: i32, spacing_glyph: i32, baseline_offset: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFindVariation < 'a > {
    fn new(surround_object: &'a re_export::Font, variation_coordinates: &'a Dictionary,) -> Self {
        let face_index = 0i32;
        let strength = 0f32;
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let spacing_top = 0i32;
        let spacing_bottom = 0i32;
        let spacing_space = 0i32;
        let spacing_glyph = 0i32;
        let baseline_offset = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, variation_coordinates: CowArg::Borrowed(variation_coordinates), face_index: face_index, strength: strength, transform: transform, spacing_top: spacing_top, spacing_bottom: spacing_bottom, spacing_space: spacing_space, spacing_glyph: spacing_glyph, baseline_offset: baseline_offset,
        }
    }
    #[inline]
    pub fn face_index(self, face_index: i32) -> Self {
        Self {
            face_index: face_index, .. self
        }
    }
    #[inline]
    pub fn strength(self, strength: f32) -> Self {
        Self {
            strength: strength, .. self
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn spacing_top(self, spacing_top: i32) -> Self {
        Self {
            spacing_top: spacing_top, .. self
        }
    }
    #[inline]
    pub fn spacing_bottom(self, spacing_bottom: i32) -> Self {
        Self {
            spacing_bottom: spacing_bottom, .. self
        }
    }
    #[inline]
    pub fn spacing_space(self, spacing_space: i32) -> Self {
        Self {
            spacing_space: spacing_space, .. self
        }
    }
    #[inline]
    pub fn spacing_glyph(self, spacing_glyph: i32) -> Self {
        Self {
            spacing_glyph: spacing_glyph, .. self
        }
    }
    #[inline]
    pub fn baseline_offset(self, baseline_offset: f32) -> Self {
        Self {
            baseline_offset: baseline_offset, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, variation_coordinates, face_index, strength, transform, spacing_top, spacing_bottom, spacing_space, spacing_glyph, baseline_offset,
        }
        = self;
        re_export::Font::find_variation_full(surround_object, variation_coordinates.cow_as_arg(), face_index, strength, transform, spacing_top, spacing_bottom, spacing_space, spacing_glyph, baseline_offset,)
    }
}
#[doc = "Default-param extender for [`Font::get_height_ex`][super::Font::get_height_ex]."]
#[must_use]
pub struct ExGetHeight < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetHeight < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_height_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_ascent_ex`][super::Font::get_ascent_ex]."]
#[must_use]
pub struct ExGetAscent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetAscent < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_ascent_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_descent_ex`][super::Font::get_descent_ex]."]
#[must_use]
pub struct ExGetDescent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDescent < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_descent_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_underline_position_ex`][super::Font::get_underline_position_ex]."]
#[must_use]
pub struct ExGetUnderlinePosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUnderlinePosition < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_underline_position_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_underline_thickness_ex`][super::Font::get_underline_thickness_ex]."]
#[must_use]
pub struct ExGetUnderlineThickness < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUnderlineThickness < 'a > {
    fn new(surround_object: &'a re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_underline_thickness_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_string_size_ex`][super::Font::get_string_size_ex]."]
#[must_use]
pub struct ExGetStringSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetStringSize < 'a > {
    fn new(surround_object: &'a re_export::Font, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, justification_flags: justification_flags, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, text, alignment, width, font_size, justification_flags, direction, orientation,
        }
        = self;
        re_export::Font::get_string_size_full(surround_object, text, alignment, width, font_size, justification_flags, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`Font::get_multiline_string_size_ex`][super::Font::get_multiline_string_size_ex]."]
#[must_use]
pub struct ExGetMultilineStringSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMultilineStringSize < 'a > {
    fn new(surround_object: &'a re_export::Font, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, text, alignment, width, font_size, max_lines, brk_flags, justification_flags, direction, orientation,
        }
        = self;
        re_export::Font::get_multiline_string_size_full(surround_object, text, alignment, width, font_size, max_lines, brk_flags, justification_flags, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_string_ex`][super::Font::draw_string_ex]."]
#[must_use]
pub struct ExDrawString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawString < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, modulate: modulate, justification_flags: justification_flags, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas_item, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation,
        }
        = self;
        re_export::Font::draw_string_full(surround_object, canvas_item, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_multiline_string_ex`][super::Font::draw_multiline_string_ex]."]
#[must_use]
pub struct ExDrawMultilineString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineString < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, modulate: modulate, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas_item, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation,
        }
        = self;
        re_export::Font::draw_multiline_string_full(surround_object, canvas_item, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_string_outline_ex`][super::Font::draw_string_outline_ex]."]
#[must_use]
pub struct ExDrawStringOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawStringOutline < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let size = 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, size: size, modulate: modulate, justification_flags: justification_flags, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas_item, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation,
        }
        = self;
        re_export::Font::draw_string_outline_full(surround_object, canvas_item, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_multiline_string_outline_ex`][super::Font::draw_multiline_string_outline_ex]."]
#[must_use]
pub struct ExDrawMultilineStringOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: CowArg < 'a, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawMultilineStringOutline < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'a,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let size = 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, size: size, modulate: modulate, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas_item, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation,
        }
        = self;
        re_export::Font::draw_multiline_string_outline_full(surround_object, canvas_item, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_char_ex`][super::Font::draw_char_ex]."]
#[must_use]
pub struct ExDrawChar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawChar < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, char: char, font_size: font_size, modulate: modulate,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, canvas_item, pos, char, font_size, modulate,
        }
        = self;
        re_export::Font::draw_char_full(surround_object, canvas_item, pos, char, font_size, modulate,)
    }
}
#[doc = "Default-param extender for [`Font::draw_char_outline_ex`][super::Font::draw_char_outline_ex]."]
#[must_use]
pub struct ExDrawCharOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32, size: i32, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawCharOutline < 'a > {
    fn new(surround_object: &'a re_export::Font, canvas_item: Rid, pos: Vector2, char: i64, font_size: i32,) -> Self {
        let size = - 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, char: char, font_size: font_size, size: size, modulate: modulate,
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, canvas_item, pos, char, font_size, size, modulate,
        }
        = self;
        re_export::Font::draw_char_outline_full(surround_object, canvas_item, pos, char, font_size, size, modulate,)
    }
}