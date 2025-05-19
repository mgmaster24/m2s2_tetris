#![doc = "Sidecar module for class [`RichTextLabel`][crate::classes::RichTextLabel].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RichTextLabel` enums](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RichTextLabel.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`rich_text_label`][crate::classes::rich_text_label]: sidecar module with related enum/flag types\n* [`IRichTextLabel`][crate::classes::IRichTextLabel]: virtual methods\n\n\nSee also [Godot docs for `RichTextLabel`](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`RichTextLabel::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RichTextLabel {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RichTextLabel`][crate::classes::RichTextLabel].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RichTextLabel` methods](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRichTextLabel: crate::obj::GodotClass < Base = RichTextLabel > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RichTextLabel {
        pub fn get_parsed_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_parsed_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_text(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "add_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_image_full(&mut self, image: ObjectArg < crate::classes::Texture2D >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, key: RefArg < Variant >, pad: bool, tooltip: CowArg < GString >, size_in_percent: bool,) {
            type CallSig < 'a0, 'a1, > = ((), ObjectArg < crate::classes::Texture2D >, i32, i32, Color, crate::global::InlineAlignment, Rect2, RefArg < 'a0, Variant >, bool, CowArg < 'a1, GString >, bool);
            let args = (image, width, height, color, inline_align, region, key, pad, tooltip, size_in_percent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "add_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_image(&mut self, image: impl AsObjectArg < crate::classes::Texture2D >,) {
            self.add_image_ex(image,) . done()
        }
        #[inline]
        pub fn add_image_ex < 'a > (&'a mut self, image: impl AsObjectArg < crate::classes::Texture2D >,) -> ExAddImage < 'a > {
            ExAddImage::new(self, image,)
        }
        pub(crate) fn update_image_full(&mut self, key: RefArg < Variant >, mask: crate::classes::rich_text_label::ImageUpdateMask, image: ObjectArg < crate::classes::Texture2D >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, pad: bool, tooltip: CowArg < GString >, size_in_percent: bool,) {
            type CallSig < 'a0, 'a1, > = ((), RefArg < 'a0, Variant >, crate::classes::rich_text_label::ImageUpdateMask, ObjectArg < crate::classes::Texture2D >, i32, i32, Color, crate::global::InlineAlignment, Rect2, bool, CowArg < 'a1, GString >, bool);
            let args = (key, mask, image, width, height, color, inline_align, region, pad, tooltip, size_in_percent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "update_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::update_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn update_image(&mut self, key: &Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsObjectArg < crate::classes::Texture2D >,) {
            self.update_image_ex(key, mask, image,) . done()
        }
        #[inline]
        pub fn update_image_ex < 'a > (&'a mut self, key: &'a Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsObjectArg < crate::classes::Texture2D >,) -> ExUpdateImage < 'a > {
            ExUpdateImage::new(self, key, mask, image,)
        }
        pub fn newline(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "newline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn remove_paragraph_full(&mut self, paragraph: i32, no_invalidate: bool,) -> bool {
            type CallSig = (bool, i32, bool);
            let args = (paragraph, no_invalidate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "remove_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::remove_paragraph_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn remove_paragraph(&mut self, paragraph: i32,) -> bool {
            self.remove_paragraph_ex(paragraph,) . done()
        }
        #[inline]
        pub fn remove_paragraph_ex < 'a > (&'a mut self, paragraph: i32,) -> ExRemoveParagraph < 'a > {
            ExRemoveParagraph::new(self, paragraph,)
        }
        pub fn invalidate_paragraph(&mut self, paragraph: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "invalidate_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_font_full(&mut self, font: ObjectArg < crate::classes::Font >, font_size: i32,) {
            type CallSig = ((), ObjectArg < crate::classes::Font >, i32);
            let args = (font, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_font_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_font(&mut self, font: impl AsObjectArg < crate::classes::Font >,) {
            self.push_font_ex(font,) . done()
        }
        #[inline]
        pub fn push_font_ex < 'a > (&'a mut self, font: impl AsObjectArg < crate::classes::Font >,) -> ExPushFont < 'a > {
            ExPushFont::new(self, font,)
        }
        pub fn push_font_size(&mut self, font_size: i32,) {
            type CallSig = ((), i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_normal(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_bold(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_bold_italics(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bold_italics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_italics(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_italics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_mono(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_mono", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_outline_size(&mut self, outline_size: i32,) {
            type CallSig = ((), i32);
            let args = (outline_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_outline_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_outline_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_outline_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_paragraph_full(&mut self, alignment: crate::global::HorizontalAlignment, base_direction: crate::classes::control::TextDirection, language: CowArg < GString >, st_parser: crate::classes::text_server::StructuredTextParser, justification_flags: crate::classes::text_server::JustificationFlag, tab_stops: RefArg < PackedFloat32Array >,) {
            type CallSig < 'a0, 'a1, > = ((), crate::global::HorizontalAlignment, crate::classes::control::TextDirection, CowArg < 'a0, GString >, crate::classes::text_server::StructuredTextParser, crate::classes::text_server::JustificationFlag, RefArg < 'a1, PackedFloat32Array >);
            let args = (alignment, base_direction, language, st_parser, justification_flags, tab_stops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_paragraph_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_paragraph(&mut self, alignment: crate::global::HorizontalAlignment,) {
            self.push_paragraph_ex(alignment,) . done()
        }
        #[inline]
        pub fn push_paragraph_ex < 'a > (&'a mut self, alignment: crate::global::HorizontalAlignment,) -> ExPushParagraph < 'a > {
            ExPushParagraph::new(self, alignment,)
        }
        pub fn push_indent(&mut self, level: i32,) {
            type CallSig = ((), i32);
            let args = (level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_list_full(&mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool, bullet: CowArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, crate::classes::rich_text_label::ListType, bool, CowArg < 'a0, GString >);
            let args = (level, type_, capitalize, bullet,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_list(&mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) {
            self.push_list_ex(level, type_, capitalize,) . done()
        }
        #[inline]
        pub fn push_list_ex < 'a > (&'a mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) -> ExPushList < 'a > {
            ExPushList::new(self, level, type_, capitalize,)
        }
        pub(crate) fn push_meta_full(&mut self, data: RefArg < Variant >, underline_mode: crate::classes::rich_text_label::MetaUnderline,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >, crate::classes::rich_text_label::MetaUnderline);
            let args = (data, underline_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_meta_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_meta(&mut self, data: &Variant,) {
            self.push_meta_ex(data,) . done()
        }
        #[inline]
        pub fn push_meta_ex < 'a > (&'a mut self, data: &'a Variant,) -> ExPushMeta < 'a > {
            ExPushMeta::new(self, data,)
        }
        pub fn push_hint(&mut self, description: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_language(&mut self, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_underline(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_underline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_strikethrough(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_strikethrough", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_table_full(&mut self, columns: i32, inline_align: crate::global::InlineAlignment, align_to_row: i32,) {
            type CallSig = ((), i32, crate::global::InlineAlignment, i32);
            let args = (columns, inline_align, align_to_row,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_table", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_table_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_table(&mut self, columns: i32,) {
            self.push_table_ex(columns,) . done()
        }
        #[inline]
        pub fn push_table_ex < 'a > (&'a mut self, columns: i32,) -> ExPushTable < 'a > {
            ExPushTable::new(self, columns,)
        }
        pub(crate) fn push_dropcap_full(&mut self, string: CowArg < GString >, font: ObjectArg < crate::classes::Font >, size: i32, dropcap_margins: Rect2, color: Color, outline_size: i32, outline_color: Color,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, ObjectArg < crate::classes::Font >, i32, Rect2, Color, i32, Color);
            let args = (string, font, size, dropcap_margins, color, outline_size, outline_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_dropcap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_dropcap_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_dropcap(&mut self, string: impl AsArg < GString >, font: impl AsObjectArg < crate::classes::Font >, size: i32,) {
            self.push_dropcap_ex(string, font, size,) . done()
        }
        #[inline]
        pub fn push_dropcap_ex < 'a > (&'a mut self, string: impl AsArg < GString > + 'a, font: impl AsObjectArg < crate::classes::Font >, size: i32,) -> ExPushDropcap < 'a > {
            ExPushDropcap::new(self, string, font, size,)
        }
        pub(crate) fn set_table_column_expand_full(&mut self, column: i32, expand: bool, ratio: i32,) {
            type CallSig = ((), i32, bool, i32);
            let args = (column, expand, ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_table_column_expand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_table_column_expand_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_table_column_expand(&mut self, column: i32, expand: bool,) {
            self.set_table_column_expand_ex(column, expand,) . done()
        }
        #[inline]
        pub fn set_table_column_expand_ex < 'a > (&'a mut self, column: i32, expand: bool,) -> ExSetTableColumnExpand < 'a > {
            ExSetTableColumnExpand::new(self, column, expand,)
        }
        pub fn set_cell_row_background_color(&mut self, odd_row_bg: Color, even_row_bg: Color,) {
            type CallSig = ((), Color, Color);
            let args = (odd_row_bg, even_row_bg,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_row_background_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_border_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_border_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_size_override(&mut self, min_size: Vector2, max_size: Vector2,) {
            type CallSig = ((), Vector2, Vector2);
            let args = (min_size, max_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_padding(&mut self, padding: Rect2,) {
            type CallSig = ((), Rect2);
            let args = (padding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_padding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_cell(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_fgcolor(&mut self, fgcolor: Color,) {
            type CallSig = ((), Color);
            let args = (fgcolor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_fgcolor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_bgcolor(&mut self, bgcolor: Color,) {
            type CallSig = ((), Color);
            let args = (bgcolor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bgcolor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_customfx(&mut self, effect: impl AsObjectArg < crate::classes::RichTextEffect >, env: &Dictionary,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::RichTextEffect >, RefArg < 'a0, Dictionary >);
            let args = (effect.as_object_arg(), RefArg::new(env),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_customfx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_context(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pop_context(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pop(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pop_all(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallSig = ((), crate::classes::text_server::StructuredTextParser);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallSig = (crate::classes::text_server::StructuredTextParser,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VariantArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallSig = ((), crate::classes::control::TextDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallSig = (crate::classes::control::TextDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallSig = ((), crate::classes::text_server::AutowrapMode);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallSig = (crate::classes::text_server::AutowrapMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_meta_underline(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_meta_underline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_meta_underlined(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_meta_underlined", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hint_underline(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_hint_underline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hint_underlined(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_hint_underlined", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scroll_active(&mut self, active: bool,) {
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_scroll_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_scroll_active(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_scroll_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scroll_follow(&mut self, follow: bool,) {
            type CallSig = ((), bool);
            let args = (follow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_scroll_follow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_scroll_following(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_scroll_following", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll_bar(&mut self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallSig = (Option < Gd < crate::classes::VScrollBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_v_scroll_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scroll_to_line(&mut self, line: i32,) {
            type CallSig = ((), i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scroll_to_paragraph(&mut self, paragraph: i32,) {
            type CallSig = ((), i32);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scroll_to_selection(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_size(&mut self, spaces: i32,) {
            type CallSig = ((), i32);
            let args = (spaces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_tab_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_tab_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fit_content(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_fit_content", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_fit_content_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_fit_content_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_selection_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selection_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_context_menu_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_context_menu_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_context_menu_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_shortcut_keys_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_shortcut_keys_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_deselect_on_focus_loss_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_deselect_on_focus_loss_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_drag_and_drop_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_drag_and_drop_selection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selection_from(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selection_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selection_to(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selection_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_all(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "select_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selected_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deselect(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "deselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn parse_bbcode(&mut self, bbcode: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (bbcode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "parse_bbcode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn append_text(&mut self, bbcode: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (bbcode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "append_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ready(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_ready", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_threaded(&mut self, threaded: bool,) {
            type CallSig = ((), bool);
            let args = (threaded,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_threaded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_threaded(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_threaded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_progress_bar_delay(&mut self, delay_ms: i32,) {
            type CallSig = ((), i32);
            let args = (delay_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_progress_bar_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_progress_bar_delay(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_progress_bar_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_characters(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_characters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_characters(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_characters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_characters_behavior(&self,) -> crate::classes::text_server::VisibleCharactersBehavior {
            type CallSig = (crate::classes::text_server::VisibleCharactersBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_characters_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_characters_behavior(&mut self, behavior: crate::classes::text_server::VisibleCharactersBehavior,) {
            type CallSig = ((), crate::classes::text_server::VisibleCharactersBehavior);
            let args = (behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_characters_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_character_line(&mut self, character: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (character,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_character_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_character_paragraph(&mut self, character: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (character,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_character_paragraph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_character_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_total_character_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_bbcode(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_use_bbcode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_bbcode(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_using_bbcode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_line_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_line_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_paragraph_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_paragraph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_paragraph_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_paragraph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_height(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_content_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_content_width(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_content_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_offset(&mut self, line: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_paragraph_offset(&mut self, paragraph: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_paragraph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn parse_expressions_for_values(&mut self, expressions: &PackedStringArray,) -> Dictionary {
            type CallSig < 'a0, > = (Dictionary, RefArg < 'a0, PackedStringArray >);
            let args = (RefArg::new(expressions),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "parse_expressions_for_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_effects(&mut self, effects: &VariantArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >);
            let args = (RefArg::new(effects),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_effects(&mut self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn install_effect(&mut self, effect: &Variant,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >);
            let args = (RefArg::new(effect),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "install_effect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_menu(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallSig = (Option < Gd < crate::classes::PopupMenu > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_menu_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_menu_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn menu_option(&mut self, option: i32,) {
            type CallSig = ((), i32);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RichTextLabel", "menu_option", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RichTextLabel {
        type Base = crate::classes::Control;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"RichTextLabel"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RichTextLabel {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RichTextLabel {
        
    }
    impl crate::obj::cap::GodotDefault for RichTextLabel {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RichTextLabel {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RichTextLabel {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RichTextLabel`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_RichTextLabel {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RichTextLabel > for $Class {
                
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
#[doc = "Default-param extender for [`RichTextLabel::add_image_ex`][super::RichTextLabel::add_image_ex]."]
#[must_use]
pub struct ExAddImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, image: ObjectCow < crate::classes::Texture2D >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, key: CowArg < 'a, Variant >, pad: bool, tooltip: CowArg < 'a, GString >, size_in_percent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddImage < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, image: impl AsObjectArg < crate::classes::Texture2D >,) -> Self {
        let width = 0i32;
        let height = 0i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let key = Variant::nil();
        let pad = false;
        let tooltip = GString::from("");
        let size_in_percent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.consume_arg(), width: width, height: height, color: color, inline_align: inline_align, region: region, key: CowArg::Owned(key), pad: pad, tooltip: CowArg::Owned(tooltip), size_in_percent: size_in_percent,
        }
    }
    #[inline]
    pub fn width(self, width: i32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn height(self, height: i32) -> Self {
        Self {
            height: height, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn key(self, key: &'a Variant) -> Self {
        Self {
            key: CowArg::Borrowed(key), .. self
        }
    }
    #[inline]
    pub fn pad(self, pad: bool) -> Self {
        Self {
            pad: pad, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'a) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn size_in_percent(self, size_in_percent: bool) -> Self {
        Self {
            size_in_percent: size_in_percent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, width, height, color, inline_align, region, key, pad, tooltip, size_in_percent,
        }
        = self;
        re_export::RichTextLabel::add_image_full(surround_object, image.cow_as_object_arg(), width, height, color, inline_align, region, key.cow_as_arg(), pad, tooltip, size_in_percent,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::update_image_ex`][super::RichTextLabel::update_image_ex]."]
#[must_use]
pub struct ExUpdateImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, key: CowArg < 'a, Variant >, mask: crate::classes::rich_text_label::ImageUpdateMask, image: ObjectCow < crate::classes::Texture2D >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, pad: bool, tooltip: CowArg < 'a, GString >, size_in_percent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExUpdateImage < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, key: &'a Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsObjectArg < crate::classes::Texture2D >,) -> Self {
        let width = 0i32;
        let height = 0i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let pad = false;
        let tooltip = GString::from("");
        let size_in_percent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), mask: mask, image: image.consume_arg(), width: width, height: height, color: color, inline_align: inline_align, region: region, pad: pad, tooltip: CowArg::Owned(tooltip), size_in_percent: size_in_percent,
        }
    }
    #[inline]
    pub fn width(self, width: i32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn height(self, height: i32) -> Self {
        Self {
            height: height, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn pad(self, pad: bool) -> Self {
        Self {
            pad: pad, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'a) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn size_in_percent(self, size_in_percent: bool) -> Self {
        Self {
            size_in_percent: size_in_percent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, key, mask, image, width, height, color, inline_align, region, pad, tooltip, size_in_percent,
        }
        = self;
        re_export::RichTextLabel::update_image_full(surround_object, key.cow_as_arg(), mask, image.cow_as_object_arg(), width, height, color, inline_align, region, pad, tooltip, size_in_percent,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::remove_paragraph_ex`][super::RichTextLabel::remove_paragraph_ex]."]
#[must_use]
pub struct ExRemoveParagraph < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, paragraph: i32, no_invalidate: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRemoveParagraph < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, paragraph: i32,) -> Self {
        let no_invalidate = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, paragraph: paragraph, no_invalidate: no_invalidate,
        }
    }
    #[inline]
    pub fn no_invalidate(self, no_invalidate: bool) -> Self {
        Self {
            no_invalidate: no_invalidate, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, paragraph, no_invalidate,
        }
        = self;
        re_export::RichTextLabel::remove_paragraph_full(surround_object, paragraph, no_invalidate,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_font_ex`][super::RichTextLabel::push_font_ex]."]
#[must_use]
pub struct ExPushFont < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, font: ObjectCow < crate::classes::Font >, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushFont < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, font: impl AsObjectArg < crate::classes::Font >,) -> Self {
        let font_size = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.consume_arg(), font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, font_size,
        }
        = self;
        re_export::RichTextLabel::push_font_full(surround_object, font.cow_as_object_arg(), font_size,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_paragraph_ex`][super::RichTextLabel::push_paragraph_ex]."]
#[must_use]
pub struct ExPushParagraph < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, alignment: crate::global::HorizontalAlignment, base_direction: crate::classes::control::TextDirection, language: CowArg < 'a, GString >, st_parser: crate::classes::text_server::StructuredTextParser, justification_flags: crate::classes::text_server::JustificationFlag, tab_stops: CowArg < 'a, PackedFloat32Array >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushParagraph < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, alignment: crate::global::HorizontalAlignment,) -> Self {
        let base_direction = crate::obj::EngineEnum::from_ord(0);
        let language = GString::from("");
        let st_parser = crate::obj::EngineEnum::from_ord(0);
        let justification_flags = crate::obj::EngineBitfield::from_ord(163);
        let tab_stops = PackedFloat32Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, alignment: alignment, base_direction: base_direction, language: CowArg::Owned(language), st_parser: st_parser, justification_flags: justification_flags, tab_stops: CowArg::Owned(tab_stops),
        }
    }
    #[inline]
    pub fn base_direction(self, base_direction: crate::classes::control::TextDirection) -> Self {
        Self {
            base_direction: base_direction, .. self
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn st_parser(self, st_parser: crate::classes::text_server::StructuredTextParser) -> Self {
        Self {
            st_parser: st_parser, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn tab_stops(self, tab_stops: &'a PackedFloat32Array) -> Self {
        Self {
            tab_stops: CowArg::Borrowed(tab_stops), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, alignment, base_direction, language, st_parser, justification_flags, tab_stops,
        }
        = self;
        re_export::RichTextLabel::push_paragraph_full(surround_object, alignment, base_direction, language, st_parser, justification_flags, tab_stops.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_list_ex`][super::RichTextLabel::push_list_ex]."]
#[must_use]
pub struct ExPushList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool, bullet: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushList < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) -> Self {
        let bullet = GString::from("•");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, level: level, type_: type_, capitalize: capitalize, bullet: CowArg::Owned(bullet),
        }
    }
    #[inline]
    pub fn bullet(self, bullet: impl AsArg < GString > + 'a) -> Self {
        Self {
            bullet: bullet.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, level, type_, capitalize, bullet,
        }
        = self;
        re_export::RichTextLabel::push_list_full(surround_object, level, type_, capitalize, bullet,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_meta_ex`][super::RichTextLabel::push_meta_ex]."]
#[must_use]
pub struct ExPushMeta < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, data: CowArg < 'a, Variant >, underline_mode: crate::classes::rich_text_label::MetaUnderline,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushMeta < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, data: &'a Variant,) -> Self {
        let underline_mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: CowArg::Borrowed(data), underline_mode: underline_mode,
        }
    }
    #[inline]
    pub fn underline_mode(self, underline_mode: crate::classes::rich_text_label::MetaUnderline) -> Self {
        Self {
            underline_mode: underline_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data, underline_mode,
        }
        = self;
        re_export::RichTextLabel::push_meta_full(surround_object, data.cow_as_arg(), underline_mode,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_table_ex`][super::RichTextLabel::push_table_ex]."]
#[must_use]
pub struct ExPushTable < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, columns: i32, inline_align: crate::global::InlineAlignment, align_to_row: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushTable < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, columns: i32,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(0);
        let align_to_row = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, columns: columns, inline_align: inline_align, align_to_row: align_to_row,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn align_to_row(self, align_to_row: i32) -> Self {
        Self {
            align_to_row: align_to_row, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, columns, inline_align, align_to_row,
        }
        = self;
        re_export::RichTextLabel::push_table_full(surround_object, columns, inline_align, align_to_row,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_dropcap_ex`][super::RichTextLabel::push_dropcap_ex]."]
#[must_use]
pub struct ExPushDropcap < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, string: CowArg < 'a, GString >, font: ObjectCow < crate::classes::Font >, size: i32, dropcap_margins: Rect2, color: Color, outline_size: i32, outline_color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushDropcap < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, string: impl AsArg < GString > + 'a, font: impl AsObjectArg < crate::classes::Font >, size: i32,) -> Self {
        let dropcap_margins = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let outline_size = 0i32;
        let outline_color = Color::from_rgba(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), font: font.consume_arg(), size: size, dropcap_margins: dropcap_margins, color: color, outline_size: outline_size, outline_color: outline_color,
        }
    }
    #[inline]
    pub fn dropcap_margins(self, dropcap_margins: Rect2) -> Self {
        Self {
            dropcap_margins: dropcap_margins, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn outline_color(self, outline_color: Color) -> Self {
        Self {
            outline_color: outline_color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, string, font, size, dropcap_margins, color, outline_size, outline_color,
        }
        = self;
        re_export::RichTextLabel::push_dropcap_full(surround_object, string, font.cow_as_object_arg(), size, dropcap_margins, color, outline_size, outline_color,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::set_table_column_expand_ex`][super::RichTextLabel::set_table_column_expand_ex]."]
#[must_use]
pub struct ExSetTableColumnExpand < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, column: i32, expand: bool, ratio: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetTableColumnExpand < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, column: i32, expand: bool,) -> Self {
        let ratio = 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, expand: expand, ratio: ratio,
        }
    }
    #[inline]
    pub fn ratio(self, ratio: i32) -> Self {
        Self {
            ratio: ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, expand, ratio,
        }
        = self;
        re_export::RichTextLabel::set_table_column_expand_full(surround_object, column, expand, ratio,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ListType {
    ord: i32
}
impl ListType {
    #[doc(alias = "LIST_NUMBERS")]
    #[doc = "Godot enumerator name: `LIST_NUMBERS`"]
    pub const NUMBERS: ListType = ListType {
        ord: 0i32
    };
    #[doc(alias = "LIST_LETTERS")]
    #[doc = "Godot enumerator name: `LIST_LETTERS`"]
    pub const LETTERS: ListType = ListType {
        ord: 1i32
    };
    #[doc(alias = "LIST_ROMAN")]
    #[doc = "Godot enumerator name: `LIST_ROMAN`"]
    pub const ROMAN: ListType = ListType {
        ord: 2i32
    };
    #[doc(alias = "LIST_DOTS")]
    #[doc = "Godot enumerator name: `LIST_DOTS`"]
    pub const DOTS: ListType = ListType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ListType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ListType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::NUMBERS => "NUMBERS", Self::LETTERS => "LETTERS", Self::ROMAN => "ROMAN", Self::DOTS => "DOTS", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NUMBERS => "LIST_NUMBERS", Self::LETTERS => "LIST_LETTERS", Self::ROMAN => "LIST_ROMAN", Self::DOTS => "LIST_DOTS", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ListType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ListType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ListType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MenuItems {
    ord: i32
}
impl MenuItems {
    #[doc(alias = "MENU_COPY")]
    #[doc = "Godot enumerator name: `MENU_COPY`"]
    pub const COPY: MenuItems = MenuItems {
        ord: 0i32
    };
    #[doc(alias = "MENU_SELECT_ALL")]
    #[doc = "Godot enumerator name: `MENU_SELECT_ALL`"]
    pub const SELECT_ALL: MenuItems = MenuItems {
        ord: 1i32
    };
    #[doc(alias = "MENU_MAX")]
    #[doc = "Godot enumerator name: `MENU_MAX`"]
    pub const MAX: MenuItems = MenuItems {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MenuItems {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MenuItems") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MenuItems {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::COPY => "COPY", Self::SELECT_ALL => "SELECT_ALL", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::COPY => "MENU_COPY", Self::SELECT_ALL => "MENU_SELECT_ALL", Self::MAX => "MENU_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for MenuItems {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for MenuItems {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MenuItems {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MenuItems {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MetaUnderline {
    ord: i32
}
impl MetaUnderline {
    #[doc(alias = "META_UNDERLINE_NEVER")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_NEVER`"]
    pub const NEVER: MetaUnderline = MetaUnderline {
        ord: 0i32
    };
    #[doc(alias = "META_UNDERLINE_ALWAYS")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_ALWAYS`"]
    pub const ALWAYS: MetaUnderline = MetaUnderline {
        ord: 1i32
    };
    #[doc(alias = "META_UNDERLINE_ON_HOVER")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_ON_HOVER`"]
    pub const ON_HOVER: MetaUnderline = MetaUnderline {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MetaUnderline {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MetaUnderline") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MetaUnderline {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::NEVER => "NEVER", Self::ALWAYS => "ALWAYS", Self::ON_HOVER => "ON_HOVER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NEVER => "META_UNDERLINE_NEVER", Self::ALWAYS => "META_UNDERLINE_ALWAYS", Self::ON_HOVER => "META_UNDERLINE_ON_HOVER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for MetaUnderline {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MetaUnderline {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MetaUnderline {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ImageUpdateMask {
    ord: u64
}
impl ImageUpdateMask {
    #[doc(alias = "UPDATE_TEXTURE")]
    #[doc = "Godot enumerator name: `UPDATE_TEXTURE`"]
    pub const TEXTURE: ImageUpdateMask = ImageUpdateMask {
        ord: 1u64
    };
    #[doc(alias = "UPDATE_SIZE")]
    #[doc = "Godot enumerator name: `UPDATE_SIZE`"]
    pub const SIZE: ImageUpdateMask = ImageUpdateMask {
        ord: 2u64
    };
    #[doc(alias = "UPDATE_COLOR")]
    #[doc = "Godot enumerator name: `UPDATE_COLOR`"]
    pub const COLOR: ImageUpdateMask = ImageUpdateMask {
        ord: 4u64
    };
    #[doc(alias = "UPDATE_ALIGNMENT")]
    #[doc = "Godot enumerator name: `UPDATE_ALIGNMENT`"]
    pub const ALIGNMENT: ImageUpdateMask = ImageUpdateMask {
        ord: 8u64
    };
    #[doc(alias = "UPDATE_REGION")]
    #[doc = "Godot enumerator name: `UPDATE_REGION`"]
    pub const REGION: ImageUpdateMask = ImageUpdateMask {
        ord: 16u64
    };
    #[doc(alias = "UPDATE_PAD")]
    #[doc = "Godot enumerator name: `UPDATE_PAD`"]
    pub const PAD: ImageUpdateMask = ImageUpdateMask {
        ord: 32u64
    };
    #[doc(alias = "UPDATE_TOOLTIP")]
    #[doc = "Godot enumerator name: `UPDATE_TOOLTIP`"]
    pub const TOOLTIP: ImageUpdateMask = ImageUpdateMask {
        ord: 64u64
    };
    #[doc(alias = "UPDATE_WIDTH_IN_PERCENT")]
    #[doc = "Godot enumerator name: `UPDATE_WIDTH_IN_PERCENT`"]
    pub const WIDTH_IN_PERCENT: ImageUpdateMask = ImageUpdateMask {
        ord: 128u64
    };
    
}
impl std::fmt::Debug for ImageUpdateMask {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::TEXTURE => "TEXTURE", Self::SIZE => "SIZE", Self::COLOR => "COLOR", Self::ALIGNMENT => "ALIGNMENT", Self::REGION => "REGION", Self::PAD => "PAD", Self::TOOLTIP => "TOOLTIP", Self::WIDTH_IN_PERCENT => "WIDTH_IN_PERCENT", _ => {
                f.debug_struct("ImageUpdateMask") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for ImageUpdateMask {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for ImageUpdateMask {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for ImageUpdateMask {
    type Via = u64;
    
}
impl crate::meta::ToGodot for ImageUpdateMask {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ImageUpdateMask {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}