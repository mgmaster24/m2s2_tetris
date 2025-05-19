#![doc = "Sidecar module for class [`TextMesh`][crate::classes::TextMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextMesh` enums](https://docs.godotengine.org/en/stable/classes/class_textmesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextMesh.`\n\nInherits [`PrimitiveMesh`][crate::classes::PrimitiveMesh].\n\nRelated symbols:\n\n* [`ITextMesh`][crate::classes::ITextMesh]: virtual methods\n\n\nSee also [Godot docs for `TextMesh`](https://docs.godotengine.org/en/stable/classes/class_textmesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TextMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextMesh`][crate::classes::TextMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextMesh` methods](https://docs.godotengine.org/en/stable/classes/class_textmesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextMesh: crate::obj::GodotClass < Base = TextMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn create_mesh_array(&self,) -> VariantArray {
            unimplemented !()
        }
        fn get_surface_count(&self,) -> i32;
        fn surface_get_array_len(&self, index: i32,) -> i32;
        fn surface_get_array_index_len(&self, index: i32,) -> i32;
        fn surface_get_arrays(&self, index: i32,) -> VariantArray;
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VariantArray >;
        fn surface_get_lods(&self, index: i32,) -> Dictionary;
        fn surface_get_format(&self, index: i32,) -> u32;
        fn surface_get_primitive_type(&self, index: i32,) -> u32;
        fn surface_set_material(&mut self, index: i32, material: Option < Gd < crate::classes::Material > >,);
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::classes::Material > >;
        fn get_blend_shape_count(&self,) -> i32;
        fn get_blend_shape_name(&self, index: i32,) -> StringName;
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,);
        fn get_aabb(&self,) -> Aabb;
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl TextMesh {
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallSig = ((), crate::global::HorizontalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallSig = (crate::global::HorizontalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_horizontal_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::global::VerticalAlignment,) {
            type CallSig = ((), crate::global::VerticalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_vertical_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallSig = (crate::global::VerticalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_vertical_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font(&mut self, font: impl AsObjectArg < crate::classes::Font >,) {
            type CallSig = ((), ObjectArg < crate::classes::Font >);
            let args = (font.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallSig = (Option < Gd < crate::classes::Font > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_size(&mut self, font_size: i32,) {
            type CallSig = ((), i32);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_spacing(&mut self, line_spacing: f32,) {
            type CallSig = ((), f32);
            let args = (line_spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_line_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_spacing(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_line_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallSig = ((), crate::classes::text_server::AutowrapMode);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallSig = (crate::classes::text_server::AutowrapMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::classes::text_server::JustificationFlag,) {
            type CallSig = ((), crate::classes::text_server::JustificationFlag);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_justification_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallSig = (crate::classes::text_server::JustificationFlag,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_justification_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth(&mut self, depth: f32,) {
            type CallSig = ((), f32);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_width(&mut self, width: f32,) {
            type CallSig = ((), f32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pixel_size(&mut self, pixel_size: f32,) {
            type CallSig = ((), f32);
            let args = (pixel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_pixel_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pixel_size(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_pixel_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_curve_step(&mut self, curve_step: f32,) {
            type CallSig = ((), f32);
            let args = (curve_step,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_curve_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_curve_step(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_curve_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::text_server::Direction,) {
            type CallSig = ((), crate::classes::text_server::Direction);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::text_server::Direction {
            type CallSig = (crate::classes::text_server::Direction,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallSig = ((), crate::classes::text_server::StructuredTextParser);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallSig = (crate::classes::text_server::StructuredTextParser,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VariantArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uppercase(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "set_uppercase", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_uppercase(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextMesh", "is_uppercase", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextMesh {
        type Base = crate::classes::PrimitiveMesh;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TextMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PrimitiveMesh > for TextMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for TextMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TextMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextMesh {
        
    }
    impl crate::obj::cap::GodotDefault for TextMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextMesh {
        type Target = crate::classes::PrimitiveMesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TextMesh {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PrimitiveMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Mesh > for $Class {
                
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