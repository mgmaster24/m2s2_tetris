#![doc = "Sidecar module for class [`TextServer`][crate::classes::TextServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextServer` enums](https://docs.godotengine.org/en/stable/classes/class_textserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextServer.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`text_server`][crate::classes::text_server]: sidecar module with related enum/flag types\n* [`ITextServer`][crate::classes::ITextServer]: virtual methods\n\n\nSee also [Godot docs for `TextServer`](https://docs.godotengine.org/en/stable/classes/class_textserver.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TextServer>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextServer`][crate::classes::TextServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextServer` methods](https://docs.godotengine.org/en/stable/classes/class_textserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextServer: crate::obj::GodotClass < Base = TextServer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TextServer {
        pub fn has_feature(&self, feature: crate::classes::text_server::Feature,) -> bool {
            type CallSig = (bool, crate::classes::text_server::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "has_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_features(&self,) -> i64 {
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "get_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_support_data(&mut self, filename: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "load_support_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_support_data_filename(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "get_support_data_filename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_support_data_info(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "get_support_data_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_support_data(&self, filename: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "save_support_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_locale_right_to_left(&self, locale: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "is_locale_right_to_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn name_to_tag(&self, name: impl AsArg < GString >,) -> i64 {
            type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "name_to_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tag_to_name(&self, tag: i64,) -> GString {
            type CallSig = (GString, i64);
            let args = (tag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "tag_to_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has(&mut self, rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "has", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_font(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "create_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_font_linked_variation(&mut self, font_rid: Rid,) -> Rid {
            type CallSig = (Rid, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "create_font_linked_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_data(&mut self, font_rid: Rid, data: &PackedByteArray,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, PackedByteArray >);
            let args = (font_rid, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_face_index(&mut self, font_rid: Rid, face_index: i64,) {
            type CallSig = ((), Rid, i64);
            let args = (font_rid, face_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_face_index(&self, font_rid: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_face_count(&self, font_rid: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_face_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_style(&mut self, font_rid: Rid, style: crate::classes::text_server::FontStyle,) {
            type CallSig = ((), Rid, crate::classes::text_server::FontStyle);
            let args = (font_rid, style,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_style", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_style(&self, font_rid: Rid,) -> crate::classes::text_server::FontStyle {
            type CallSig = (crate::classes::text_server::FontStyle, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_style", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_name(&mut self, font_rid: Rid, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), Rid, CowArg < 'a0, GString >);
            let args = (font_rid, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_name(&self, font_rid: Rid,) -> GString {
            type CallSig = (GString, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_ot_name_strings(&self, font_rid: Rid,) -> Dictionary {
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_ot_name_strings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_style_name(&mut self, font_rid: Rid, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), Rid, CowArg < 'a0, GString >);
            let args = (font_rid, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_style_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_style_name(&self, font_rid: Rid,) -> GString {
            type CallSig = (GString, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_style_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_weight(&mut self, font_rid: Rid, weight: i64,) {
            type CallSig = ((), Rid, i64);
            let args = (font_rid, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_weight(&self, font_rid: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_stretch(&mut self, font_rid: Rid, weight: i64,) {
            type CallSig = ((), Rid, i64);
            let args = (font_rid, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_stretch(&self, font_rid: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_antialiasing(&mut self, font_rid: Rid, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            type CallSig = ((), Rid, crate::classes::text_server::FontAntialiasing);
            let args = (font_rid, antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_antialiasing(&self, font_rid: Rid,) -> crate::classes::text_server::FontAntialiasing {
            type CallSig = (crate::classes::text_server::FontAntialiasing, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_disable_embedded_bitmaps(&mut self, font_rid: Rid, disable_embedded_bitmaps: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (font_rid, disable_embedded_bitmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_disable_embedded_bitmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_disable_embedded_bitmaps(&self, font_rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_disable_embedded_bitmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_generate_mipmaps(&mut self, font_rid: Rid, generate_mipmaps: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (font_rid, generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_generate_mipmaps(&self, font_rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_multichannel_signed_distance_field(&mut self, font_rid: Rid, msdf: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (font_rid, msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_is_multichannel_signed_distance_field(&self, font_rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_is_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_msdf_pixel_range(&mut self, font_rid: Rid, msdf_pixel_range: i64,) {
            type CallSig = ((), Rid, i64);
            let args = (font_rid, msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_msdf_pixel_range(&self, font_rid: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_msdf_size(&mut self, font_rid: Rid, msdf_size: i64,) {
            type CallSig = ((), Rid, i64);
            let args = (font_rid, msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_msdf_size(&self, font_rid: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_fixed_size(&mut self, font_rid: Rid, fixed_size: i64,) {
            type CallSig = ((), Rid, i64);
            let args = (font_rid, fixed_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_fixed_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_fixed_size(&self, font_rid: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_fixed_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_fixed_size_scale_mode(&mut self, font_rid: Rid, fixed_size_scale_mode: crate::classes::text_server::FixedSizeScaleMode,) {
            type CallSig = ((), Rid, crate::classes::text_server::FixedSizeScaleMode);
            let args = (font_rid, fixed_size_scale_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_fixed_size_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_fixed_size_scale_mode(&self, font_rid: Rid,) -> crate::classes::text_server::FixedSizeScaleMode {
            type CallSig = (crate::classes::text_server::FixedSizeScaleMode, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_fixed_size_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_allow_system_fallback(&mut self, font_rid: Rid, allow_system_fallback: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (font_rid, allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_is_allow_system_fallback(&self, font_rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_is_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_force_autohinter(&mut self, font_rid: Rid, force_autohinter: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (font_rid, force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_is_force_autohinter(&self, font_rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_is_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_hinting(&mut self, font_rid: Rid, hinting: crate::classes::text_server::Hinting,) {
            type CallSig = ((), Rid, crate::classes::text_server::Hinting);
            let args = (font_rid, hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_hinting(&self, font_rid: Rid,) -> crate::classes::text_server::Hinting {
            type CallSig = (crate::classes::text_server::Hinting, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_subpixel_positioning(&mut self, font_rid: Rid, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            type CallSig = ((), Rid, crate::classes::text_server::SubpixelPositioning);
            let args = (font_rid, subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_subpixel_positioning(&self, font_rid: Rid,) -> crate::classes::text_server::SubpixelPositioning {
            type CallSig = (crate::classes::text_server::SubpixelPositioning, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_embolden(&mut self, font_rid: Rid, strength: f64,) {
            type CallSig = ((), Rid, f64);
            let args = (font_rid, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_embolden(&self, font_rid: Rid,) -> f64 {
            type CallSig = (f64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_spacing(&mut self, font_rid: Rid, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallSig = ((), Rid, crate::classes::text_server::SpacingType, i64);
            let args = (font_rid, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_spacing(&self, font_rid: Rid, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallSig = (i64, Rid, crate::classes::text_server::SpacingType);
            let args = (font_rid, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_baseline_offset(&mut self, font_rid: Rid, baseline_offset: f64,) {
            type CallSig = ((), Rid, f64);
            let args = (font_rid, baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_baseline_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_baseline_offset(&self, font_rid: Rid,) -> f64 {
            type CallSig = (f64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_baseline_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_transform(&mut self, font_rid: Rid, transform: Transform2D,) {
            type CallSig = ((), Rid, Transform2D);
            let args = (font_rid, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_transform(&self, font_rid: Rid,) -> Transform2D {
            type CallSig = (Transform2D, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_variation_coordinates(&mut self, font_rid: Rid, variation_coordinates: &Dictionary,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Dictionary >);
            let args = (font_rid, RefArg::new(variation_coordinates),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_variation_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_variation_coordinates(&self, font_rid: Rid,) -> Dictionary {
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_variation_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_oversampling(&mut self, font_rid: Rid, oversampling: f64,) {
            type CallSig = ((), Rid, f64);
            let args = (font_rid, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_oversampling(&self, font_rid: Rid,) -> f64 {
            type CallSig = (f64, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_size_cache_list(&self, font_rid: Rid,) -> Array < Vector2i > {
            type CallSig = (Array < Vector2i >, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_size_cache_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_clear_size_cache(&mut self, font_rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_size_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_size_cache(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallSig = ((), Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_size_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_ascent(&mut self, font_rid: Rid, size: i64, ascent: f64,) {
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, ascent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_ascent(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_descent(&mut self, font_rid: Rid, size: i64, descent: f64,) {
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, descent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_descent(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_underline_position(&mut self, font_rid: Rid, size: i64, underline_position: f64,) {
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, underline_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_underline_position(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_underline_thickness(&mut self, font_rid: Rid, size: i64, underline_thickness: f64,) {
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, underline_thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_underline_thickness(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_scale(&mut self, font_rid: Rid, size: i64, scale: f64,) {
            type CallSig = ((), Rid, i64, f64);
            let args = (font_rid, size, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_scale(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallSig = (f64, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_texture_count(&self, font_rid: Rid, size: Vector2i,) -> i64 {
            type CallSig = (i64, Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_clear_textures(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallSig = ((), Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_texture(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64,) {
            type CallSig = ((), Rid, Vector2i, i64);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_texture_image(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, image: impl AsObjectArg < crate::classes::Image >,) {
            type CallSig = ((), Rid, Vector2i, i64, ObjectArg < crate::classes::Image >);
            let args = (font_rid, size, texture_index, image.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_texture_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_texture_image(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> Option < Gd < crate::classes::Image > > {
            type CallSig = (Option < Gd < crate::classes::Image > >, Rid, Vector2i, i64);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_texture_offsets(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, offset: &PackedInt32Array,) {
            type CallSig < 'a0, > = ((), Rid, Vector2i, i64, RefArg < 'a0, PackedInt32Array >);
            let args = (font_rid, size, texture_index, RefArg::new(offset),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_texture_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_texture_offsets(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, Rid, Vector2i, i64);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_list(&self, font_rid: Rid, size: Vector2i,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_clear_glyphs(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallSig = ((), Rid, Vector2i);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_glyphs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_glyph(&mut self, font_rid: Rid, size: Vector2i, glyph: i64,) {
            type CallSig = ((), Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_advance(&self, font_rid: Rid, size: i64, glyph: i64,) -> Vector2 {
            type CallSig = (Vector2, Rid, i64, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_advance(&mut self, font_rid: Rid, size: i64, glyph: i64, advance: Vector2,) {
            type CallSig = ((), Rid, i64, i64, Vector2);
            let args = (font_rid, size, glyph, advance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_offset(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallSig = (Vector2, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_offset(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, offset: Vector2,) {
            type CallSig = ((), Rid, Vector2i, i64, Vector2);
            let args = (font_rid, size, glyph, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallSig = (Vector2, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_size(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, gl_size: Vector2,) {
            type CallSig = ((), Rid, Vector2i, i64, Vector2);
            let args = (font_rid, size, glyph, gl_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_uv_rect(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rect2 {
            type CallSig = (Rect2, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_uv_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_uv_rect(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, uv_rect: Rect2,) {
            type CallSig = ((), Rid, Vector2i, i64, Rect2);
            let args = (font_rid, size, glyph, uv_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_uv_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_texture_idx(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> i64 {
            type CallSig = (i64, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_idx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_glyph_texture_idx(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, texture_idx: i64,) {
            type CallSig = ((), Rid, Vector2i, i64, i64);
            let args = (font_rid, size, glyph, texture_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_texture_idx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_texture_rid(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rid {
            type CallSig = (Rid, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_texture_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallSig = (Vector2, Rid, Vector2i, i64);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_contours(&self, font: Rid, size: i64, index: i64,) -> Dictionary {
            type CallSig = (Dictionary, Rid, i64, i64);
            let args = (font, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_contours", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_kerning_list(&self, font_rid: Rid, size: i64,) -> Array < Vector2i > {
            type CallSig = (Array < Vector2i >, Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_kerning_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_clear_kerning_map(&mut self, font_rid: Rid, size: i64,) {
            type CallSig = ((), Rid, i64);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_kerning_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) {
            type CallSig = ((), Rid, i64, Vector2i);
            let args = (font_rid, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i, kerning: Vector2,) {
            type CallSig = ((), Rid, i64, Vector2i, Vector2);
            let args = (font_rid, size, glyph_pair, kerning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_kerning(&self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) -> Vector2 {
            type CallSig = (Vector2, Rid, i64, Vector2i);
            let args = (font_rid, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_glyph_index(&self, font_rid: Rid, size: i64, char: i64, variation_selector: i64,) -> i64 {
            type CallSig = (i64, Rid, i64, i64, i64);
            let args = (font_rid, size, char, variation_selector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_char_from_glyph_index(&self, font_rid: Rid, size: i64, glyph_index: i64,) -> i64 {
            type CallSig = (i64, Rid, i64, i64);
            let args = (font_rid, size, glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_char_from_glyph_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_has_char(&self, font_rid: Rid, char: i64,) -> bool {
            type CallSig = (bool, Rid, i64);
            let args = (font_rid, char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_has_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_supported_chars(&self, font_rid: Rid,) -> GString {
            type CallSig = (GString, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_supported_chars", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_render_range(&mut self, font_rid: Rid, size: Vector2i, start: i64, end: i64,) {
            type CallSig = ((), Rid, Vector2i, i64, i64);
            let args = (font_rid, size, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_render_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_render_glyph(&mut self, font_rid: Rid, size: Vector2i, index: i64,) {
            type CallSig = ((), Rid, Vector2i, i64);
            let args = (font_rid, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_render_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn font_draw_glyph_full(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,) {
            type CallSig = ((), Rid, Rid, i64, Vector2, i64, Color);
            let args = (font_rid, canvas, size, pos, index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_draw_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::font_draw_glyph_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn font_draw_glyph(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) {
            self.font_draw_glyph_ex(font_rid, canvas, size, pos, index,) . done()
        }
        #[inline]
        pub fn font_draw_glyph_ex < 'a > (&'a self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) -> ExFontDrawGlyph < 'a > {
            ExFontDrawGlyph::new(self, font_rid, canvas, size, pos, index,)
        }
        pub(crate) fn font_draw_glyph_outline_full(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color,) {
            type CallSig = ((), Rid, Rid, i64, i64, Vector2, i64, Color);
            let args = (font_rid, canvas, size, outline_size, pos, index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_draw_glyph_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::font_draw_glyph_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn font_draw_glyph_outline(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) {
            self.font_draw_glyph_outline_ex(font_rid, canvas, size, outline_size, pos, index,) . done()
        }
        #[inline]
        pub fn font_draw_glyph_outline_ex < 'a > (&'a self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) -> ExFontDrawGlyphOutline < 'a > {
            ExFontDrawGlyphOutline::new(self, font_rid, canvas, size, outline_size, pos, index,)
        }
        pub fn font_is_language_supported(&self, font_rid: Rid, language: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, Rid, CowArg < 'a0, GString >);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_is_language_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >, supported: bool,) {
            type CallSig < 'a0, > = ((), Rid, CowArg < 'a0, GString >, bool);
            let args = (font_rid, language.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, Rid, CowArg < 'a0, GString >);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), Rid, CowArg < 'a0, GString >);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_language_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            type CallSig = (PackedStringArray, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_language_support_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_is_script_supported(&self, font_rid: Rid, script: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, Rid, CowArg < 'a0, GString >);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_is_script_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >, supported: bool,) {
            type CallSig < 'a0, > = ((), Rid, CowArg < 'a0, GString >, bool);
            let args = (font_rid, script.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, Rid, CowArg < 'a0, GString >);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_remove_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), Rid, CowArg < 'a0, GString >);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_script_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            type CallSig = (PackedStringArray, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_script_support_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_opentype_feature_overrides(&mut self, font_rid: Rid, overrides: &Dictionary,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Dictionary >);
            let args = (font_rid, RefArg::new(overrides),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_opentype_feature_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_opentype_feature_overrides(&self, font_rid: Rid,) -> Dictionary {
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_opentype_feature_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_supported_feature_list(&self, font_rid: Rid,) -> Dictionary {
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_supported_feature_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_supported_variation_list(&self, font_rid: Rid,) -> Dictionary {
            type CallSig = (Dictionary, Rid);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_supported_variation_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_get_global_oversampling(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_get_global_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn font_set_global_oversampling(&mut self, oversampling: f64,) {
            type CallSig = ((), f64);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "font_set_global_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hex_code_box_size(&self, size: i64, index: i64,) -> Vector2 {
            type CallSig = (Vector2, i64, i64);
            let args = (size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "get_hex_code_box_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_hex_code_box(&self, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,) {
            type CallSig = ((), Rid, i64, Vector2, i64, Color);
            let args = (canvas, size, pos, index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "draw_hex_code_box", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_shaped_text_full(&mut self, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) -> Rid {
            type CallSig = (Rid, crate::classes::text_server::Direction, crate::classes::text_server::Orientation);
            let args = (direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "create_shaped_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_shaped_text_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_shaped_text(&mut self,) -> Rid {
            self.create_shaped_text_ex() . done()
        }
        #[inline]
        pub fn create_shaped_text_ex < 'a > (&'a mut self,) -> ExCreateShapedText < 'a > {
            ExCreateShapedText::new(self,)
        }
        pub fn shaped_text_clear(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_set_direction_full(&mut self, shaped: Rid, direction: crate::classes::text_server::Direction,) {
            type CallSig = ((), Rid, crate::classes::text_server::Direction);
            let args = (shaped, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_set_direction_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_set_direction(&mut self, shaped: Rid,) {
            self.shaped_text_set_direction_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_set_direction_ex < 'a > (&'a mut self, shaped: Rid,) -> ExShapedTextSetDirection < 'a > {
            ExShapedTextSetDirection::new(self, shaped,)
        }
        pub fn shaped_text_get_direction(&self, shaped: Rid,) -> crate::classes::text_server::Direction {
            type CallSig = (crate::classes::text_server::Direction, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_inferred_direction(&self, shaped: Rid,) -> crate::classes::text_server::Direction {
            type CallSig = (crate::classes::text_server::Direction, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_inferred_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_bidi_override(&mut self, shaped: Rid, override_: &VariantArray,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, VariantArray >);
            let args = (shaped, RefArg::new(override_),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_custom_punctuation(&mut self, shaped: Rid, punct: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), Rid, CowArg < 'a0, GString >);
            let args = (shaped, punct.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_custom_punctuation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_custom_punctuation(&self, shaped: Rid,) -> GString {
            type CallSig = (GString, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_custom_punctuation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_custom_ellipsis(&mut self, shaped: Rid, char: i64,) {
            type CallSig = ((), Rid, i64);
            let args = (shaped, char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_custom_ellipsis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_custom_ellipsis(&self, shaped: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_custom_ellipsis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_set_orientation_full(&mut self, shaped: Rid, orientation: crate::classes::text_server::Orientation,) {
            type CallSig = ((), Rid, crate::classes::text_server::Orientation);
            let args = (shaped, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_set_orientation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_set_orientation(&mut self, shaped: Rid,) {
            self.shaped_text_set_orientation_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_set_orientation_ex < 'a > (&'a mut self, shaped: Rid,) -> ExShapedTextSetOrientation < 'a > {
            ExShapedTextSetOrientation::new(self, shaped,)
        }
        pub fn shaped_text_get_orientation(&self, shaped: Rid,) -> crate::classes::text_server::Orientation {
            type CallSig = (crate::classes::text_server::Orientation, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_preserve_invalid(&mut self, shaped: Rid, enabled: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (shaped, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_preserve_invalid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_preserve_invalid(&self, shaped: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_preserve_invalid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_preserve_control(&mut self, shaped: Rid, enabled: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (shaped, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_preserve_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_preserve_control(&self, shaped: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_preserve_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_set_spacing(&mut self, shaped: Rid, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallSig = ((), Rid, crate::classes::text_server::SpacingType, i64);
            let args = (shaped, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_spacing(&self, shaped: Rid, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallSig = (i64, Rid, crate::classes::text_server::SpacingType);
            let args = (shaped, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_add_string_full(&mut self, shaped: Rid, text: CowArg < GString >, fonts: RefArg < Array < Rid > >, size: i64, opentype_features: RefArg < Dictionary >, language: CowArg < GString >, meta: RefArg < Variant >,) -> bool {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (bool, Rid, CowArg < 'a0, GString >, RefArg < 'a1, Array < Rid > >, i64, RefArg < 'a2, Dictionary >, CowArg < 'a3, GString >, RefArg < 'a4, Variant >);
            let args = (shaped, text, fonts, size, opentype_features, language, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_add_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_add_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_add_string(&mut self, shaped: Rid, text: impl AsArg < GString >, fonts: &Array < Rid >, size: i64,) -> bool {
            self.shaped_text_add_string_ex(shaped, text, fonts, size,) . done()
        }
        #[inline]
        pub fn shaped_text_add_string_ex < 'a > (&'a mut self, shaped: Rid, text: impl AsArg < GString > + 'a, fonts: &'a Array < Rid >, size: i64,) -> ExShapedTextAddString < 'a > {
            ExShapedTextAddString::new(self, shaped, text, fonts, size,)
        }
        pub(crate) fn shaped_text_add_object_full(&mut self, shaped: Rid, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i64, baseline: f64,) -> bool {
            type CallSig < 'a0, > = (bool, Rid, RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, i64, f64);
            let args = (shaped, key, size, inline_align, length, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_add_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_add_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_add_object(&mut self, shaped: Rid, key: &Variant, size: Vector2,) -> bool {
            self.shaped_text_add_object_ex(shaped, key, size,) . done()
        }
        #[inline]
        pub fn shaped_text_add_object_ex < 'a > (&'a mut self, shaped: Rid, key: &'a Variant, size: Vector2,) -> ExShapedTextAddObject < 'a > {
            ExShapedTextAddObject::new(self, shaped, key, size,)
        }
        pub(crate) fn shaped_text_resize_object_full(&mut self, shaped: Rid, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f64,) -> bool {
            type CallSig < 'a0, > = (bool, Rid, RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, f64);
            let args = (shaped, key, size, inline_align, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_resize_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_resize_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_resize_object(&mut self, shaped: Rid, key: &Variant, size: Vector2,) -> bool {
            self.shaped_text_resize_object_ex(shaped, key, size,) . done()
        }
        #[inline]
        pub fn shaped_text_resize_object_ex < 'a > (&'a mut self, shaped: Rid, key: &'a Variant, size: Vector2,) -> ExShapedTextResizeObject < 'a > {
            ExShapedTextResizeObject::new(self, shaped, key, size,)
        }
        pub fn shaped_get_span_count(&self, shaped: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_get_span_meta(&self, shaped: Rid, index: i64,) -> Variant {
            type CallSig = (Variant, Rid, i64);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_set_span_update_font_full(&mut self, shaped: Rid, index: i64, fonts: RefArg < Array < Rid > >, size: i64, opentype_features: RefArg < Dictionary >,) {
            type CallSig < 'a0, 'a1, > = ((), Rid, i64, RefArg < 'a0, Array < Rid > >, i64, RefArg < 'a1, Dictionary >);
            let args = (shaped, index, fonts, size, opentype_features,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_set_span_update_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_set_span_update_font_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_set_span_update_font(&mut self, shaped: Rid, index: i64, fonts: &Array < Rid >, size: i64,) {
            self.shaped_set_span_update_font_ex(shaped, index, fonts, size,) . done()
        }
        #[inline]
        pub fn shaped_set_span_update_font_ex < 'a > (&'a mut self, shaped: Rid, index: i64, fonts: &'a Array < Rid >, size: i64,) -> ExShapedSetSpanUpdateFont < 'a > {
            ExShapedSetSpanUpdateFont::new(self, shaped, index, fonts, size,)
        }
        pub fn shaped_text_substr(&self, shaped: Rid, start: i64, length: i64,) -> Rid {
            type CallSig = (Rid, Rid, i64, i64);
            let args = (shaped, start, length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_substr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_parent(&self, shaped: Rid,) -> Rid {
            type CallSig = (Rid, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_fit_to_width_full(&mut self, shaped: Rid, width: f64, justification_flags: crate::classes::text_server::JustificationFlag,) -> f64 {
            type CallSig = (f64, Rid, f64, crate::classes::text_server::JustificationFlag);
            let args = (shaped, width, justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_fit_to_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_fit_to_width_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_fit_to_width(&mut self, shaped: Rid, width: f64,) -> f64 {
            self.shaped_text_fit_to_width_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_fit_to_width_ex < 'a > (&'a mut self, shaped: Rid, width: f64,) -> ExShapedTextFitToWidth < 'a > {
            ExShapedTextFitToWidth::new(self, shaped, width,)
        }
        pub fn shaped_text_tab_align(&mut self, shaped: Rid, tab_stops: &PackedFloat32Array,) -> f64 {
            type CallSig < 'a0, > = (f64, Rid, RefArg < 'a0, PackedFloat32Array >);
            let args = (shaped, RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_tab_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_shape(&mut self, shaped: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_is_ready(&self, shaped: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_is_ready", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_has_visible_chars(&self, shaped: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_has_visible_chars", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_glyphs(&self, shaped: Rid,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_glyphs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_sort_logical(&mut self, shaped: Rid,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_sort_logical", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_glyph_count(&self, shaped: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_glyph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_range(&self, shaped: Rid,) -> Vector2i {
            type CallSig = (Vector2i, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_get_line_breaks_adv_full(&self, shaped: Rid, width: RefArg < PackedFloat32Array >, start: i64, once: bool, break_flags: crate::classes::text_server::LineBreakFlag,) -> PackedInt32Array {
            type CallSig < 'a0, > = (PackedInt32Array, Rid, RefArg < 'a0, PackedFloat32Array >, i64, bool, crate::classes::text_server::LineBreakFlag);
            let args = (shaped, width, start, once, break_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_line_breaks_adv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_get_line_breaks_adv_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_get_line_breaks_adv(&self, shaped: Rid, width: &PackedFloat32Array,) -> PackedInt32Array {
            self.shaped_text_get_line_breaks_adv_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_get_line_breaks_adv_ex < 'a > (&'a self, shaped: Rid, width: &'a PackedFloat32Array,) -> ExShapedTextGetLineBreaksAdv < 'a > {
            ExShapedTextGetLineBreaksAdv::new(self, shaped, width,)
        }
        pub(crate) fn shaped_text_get_line_breaks_full(&self, shaped: Rid, width: f64, start: i64, break_flags: crate::classes::text_server::LineBreakFlag,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, Rid, f64, i64, crate::classes::text_server::LineBreakFlag);
            let args = (shaped, width, start, break_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_line_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_get_line_breaks_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_get_line_breaks(&self, shaped: Rid, width: f64,) -> PackedInt32Array {
            self.shaped_text_get_line_breaks_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_get_line_breaks_ex < 'a > (&'a self, shaped: Rid, width: f64,) -> ExShapedTextGetLineBreaks < 'a > {
            ExShapedTextGetLineBreaks::new(self, shaped, width,)
        }
        pub(crate) fn shaped_text_get_word_breaks_full(&self, shaped: Rid, grapheme_flags: crate::classes::text_server::GraphemeFlag, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, Rid, crate::classes::text_server::GraphemeFlag, crate::classes::text_server::GraphemeFlag);
            let args = (shaped, grapheme_flags, skip_grapheme_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_word_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_get_word_breaks_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_get_word_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            self.shaped_text_get_word_breaks_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_get_word_breaks_ex < 'a > (&'a self, shaped: Rid,) -> ExShapedTextGetWordBreaks < 'a > {
            ExShapedTextGetWordBreaks::new(self, shaped,)
        }
        pub fn shaped_text_get_trim_pos(&self, shaped: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_trim_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_pos(&self, shaped: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_glyphs(&self, shaped: Rid,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_glyphs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_glyph_count(&self, shaped: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_glyph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_overrun_trim_to_width_full(&mut self, shaped: Rid, width: f64, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag,) {
            type CallSig = ((), Rid, f64, crate::classes::text_server::TextOverrunFlag);
            let args = (shaped, width, overrun_trim_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_overrun_trim_to_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_overrun_trim_to_width_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_overrun_trim_to_width(&mut self, shaped: Rid,) {
            self.shaped_text_overrun_trim_to_width_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_overrun_trim_to_width_ex < 'a > (&'a mut self, shaped: Rid,) -> ExShapedTextOverrunTrimToWidth < 'a > {
            ExShapedTextOverrunTrimToWidth::new(self, shaped,)
        }
        pub fn shaped_text_get_objects(&self, shaped: Rid,) -> VariantArray {
            type CallSig = (VariantArray, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_objects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_object_rect(&self, shaped: Rid, key: &Variant,) -> Rect2 {
            type CallSig < 'a0, > = (Rect2, Rid, RefArg < 'a0, Variant >);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_object_range(&self, shaped: Rid, key: &Variant,) -> Vector2i {
            type CallSig < 'a0, > = (Vector2i, Rid, RefArg < 'a0, Variant >);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_object_glyph(&self, shaped: Rid, key: &Variant,) -> i64 {
            type CallSig < 'a0, > = (i64, Rid, RefArg < 'a0, Variant >);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_size(&self, shaped: Rid,) -> Vector2 {
            type CallSig = (Vector2, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_ascent(&self, shaped: Rid,) -> f64 {
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_descent(&self, shaped: Rid,) -> f64 {
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_width(&self, shaped: Rid,) -> f64 {
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_underline_position(&self, shaped: Rid,) -> f64 {
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_underline_thickness(&self, shaped: Rid,) -> f64 {
            type CallSig = (f64, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_carets(&self, shaped: Rid, position: i64,) -> Dictionary {
            type CallSig = (Dictionary, Rid, i64);
            let args = (shaped, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_carets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_selection(&self, shaped: Rid, start: i64, end: i64,) -> PackedVector2Array {
            type CallSig = (PackedVector2Array, Rid, i64, i64);
            let args = (shaped, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_hit_test_grapheme(&self, shaped: Rid, coords: f64,) -> i64 {
            type CallSig = (i64, Rid, f64);
            let args = (shaped, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_hit_test_grapheme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_hit_test_position(&self, shaped: Rid, coords: f64,) -> i64 {
            type CallSig = (i64, Rid, f64);
            let args = (shaped, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_hit_test_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_grapheme_bounds(&self, shaped: Rid, pos: i64,) -> Vector2 {
            type CallSig = (Vector2, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_grapheme_bounds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_next_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_next_grapheme_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_prev_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_prev_grapheme_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_get_character_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, Rid);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_character_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_next_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_next_character_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_prev_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_prev_character_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shaped_text_closest_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallSig = (i64, Rid, i64);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_closest_character_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shaped_text_draw_full(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color,) {
            type CallSig = ((), Rid, Rid, Vector2, f64, f64, Color);
            let args = (shaped, canvas, pos, clip_l, clip_r, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_draw(&self, shaped: Rid, canvas: Rid, pos: Vector2,) {
            self.shaped_text_draw_ex(shaped, canvas, pos,) . done()
        }
        #[inline]
        pub fn shaped_text_draw_ex < 'a > (&'a self, shaped: Rid, canvas: Rid, pos: Vector2,) -> ExShapedTextDraw < 'a > {
            ExShapedTextDraw::new(self, shaped, canvas, pos,)
        }
        pub(crate) fn shaped_text_draw_outline_full(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color,) {
            type CallSig = ((), Rid, Rid, Vector2, f64, f64, i64, Color);
            let args = (shaped, canvas, pos, clip_l, clip_r, outline_size, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_draw_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_draw_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_draw_outline(&self, shaped: Rid, canvas: Rid, pos: Vector2,) {
            self.shaped_text_draw_outline_ex(shaped, canvas, pos,) . done()
        }
        #[inline]
        pub fn shaped_text_draw_outline_ex < 'a > (&'a self, shaped: Rid, canvas: Rid, pos: Vector2,) -> ExShapedTextDrawOutline < 'a > {
            ExShapedTextDrawOutline::new(self, shaped, canvas, pos,)
        }
        pub fn shaped_text_get_dominant_direction_in_range(&self, shaped: Rid, start: i64, end: i64,) -> crate::classes::text_server::Direction {
            type CallSig = (crate::classes::text_server::Direction, Rid, i64, i64);
            let args = (shaped, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_dominant_direction_in_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn format_number_full(&self, number: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (number, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "format_number", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::format_number_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn format_number(&self, number: impl AsArg < GString >,) -> GString {
            self.format_number_ex(number,) . done()
        }
        #[inline]
        pub fn format_number_ex < 'a > (&'a self, number: impl AsArg < GString > + 'a,) -> ExFormatNumber < 'a > {
            ExFormatNumber::new(self, number,)
        }
        pub(crate) fn parse_number_full(&self, number: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (number, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "parse_number", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::parse_number_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn parse_number(&self, number: impl AsArg < GString >,) -> GString {
            self.parse_number_ex(number,) . done()
        }
        #[inline]
        pub fn parse_number_ex < 'a > (&'a self, number: impl AsArg < GString > + 'a,) -> ExParseNumber < 'a > {
            ExParseNumber::new(self, number,)
        }
        pub(crate) fn percent_sign_full(&self, language: CowArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "percent_sign", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::percent_sign_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn percent_sign(&self,) -> GString {
            self.percent_sign_ex() . done()
        }
        #[inline]
        pub fn percent_sign_ex < 'a > (&'a self,) -> ExPercentSign < 'a > {
            ExPercentSign::new(self,)
        }
        pub(crate) fn string_get_word_breaks_full(&self, string: CowArg < GString >, language: CowArg < GString >, chars_per_line: i64,) -> PackedInt32Array {
            type CallSig < 'a0, 'a1, > = (PackedInt32Array, CowArg < 'a0, GString >, CowArg < 'a1, GString >, i64);
            let args = (string, language, chars_per_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "string_get_word_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_get_word_breaks_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_get_word_breaks(&self, string: impl AsArg < GString >,) -> PackedInt32Array {
            self.string_get_word_breaks_ex(string,) . done()
        }
        #[inline]
        pub fn string_get_word_breaks_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringGetWordBreaks < 'a > {
            ExStringGetWordBreaks::new(self, string,)
        }
        pub(crate) fn string_get_character_breaks_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> PackedInt32Array {
            type CallSig < 'a0, 'a1, > = (PackedInt32Array, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "string_get_character_breaks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_get_character_breaks_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_get_character_breaks(&self, string: impl AsArg < GString >,) -> PackedInt32Array {
            self.string_get_character_breaks_ex(string,) . done()
        }
        #[inline]
        pub fn string_get_character_breaks_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringGetCharacterBreaks < 'a > {
            ExStringGetCharacterBreaks::new(self, string,)
        }
        pub fn is_confusable(&self, string: impl AsArg < GString >, dict: &PackedStringArray,) -> i64 {
            type CallSig < 'a0, 'a1, > = (i64, CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >);
            let args = (string.into_arg(), RefArg::new(dict),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "is_confusable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn spoof_check(&self, string: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "spoof_check", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn strip_diacritics(&self, string: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "strip_diacritics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_valid_identifier(&self, string: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "is_valid_identifier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_valid_letter(&self, unicode: u64,) -> bool {
            type CallSig = (bool, u64);
            let args = (unicode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "is_valid_letter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn string_to_upper_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "string_to_upper", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_to_upper_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_to_upper(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_upper_ex(string,) . done()
        }
        #[inline]
        pub fn string_to_upper_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringToUpper < 'a > {
            ExStringToUpper::new(self, string,)
        }
        pub(crate) fn string_to_lower_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "string_to_lower", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_to_lower_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_to_lower(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_lower_ex(string,) . done()
        }
        #[inline]
        pub fn string_to_lower_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringToLower < 'a > {
            ExStringToLower::new(self, string,)
        }
        pub(crate) fn string_to_title_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "string_to_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_to_title_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_to_title(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_title_ex(string,) . done()
        }
        #[inline]
        pub fn string_to_title_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringToTitle < 'a > {
            ExStringToTitle::new(self, string,)
        }
        pub fn parse_structured_text(&self, parser_type: crate::classes::text_server::StructuredTextParser, args: &VariantArray, text: impl AsArg < GString >,) -> Array < Vector3i > {
            type CallSig < 'a0, 'a1, > = (Array < Vector3i >, crate::classes::text_server::StructuredTextParser, RefArg < 'a0, VariantArray >, CowArg < 'a1, GString >);
            let args = (parser_type, RefArg::new(args), text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextServer", "parse_structured_text", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextServer {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TextServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextServer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextServer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextServer {
        
    }
    impl std::ops::Deref for TextServer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextServer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TextServer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextServer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TextServer::font_draw_glyph_ex`][super::TextServer::font_draw_glyph_ex]."]
#[must_use]
pub struct ExFontDrawGlyph < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFontDrawGlyph < 'a > {
    fn new(surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_rid: font_rid, canvas: canvas, size: size, pos: pos, index: index, color: color,
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
            _phantom, surround_object, font_rid, canvas, size, pos, index, color,
        }
        = self;
        re_export::TextServer::font_draw_glyph_full(surround_object, font_rid, canvas, size, pos, index, color,)
    }
}
#[doc = "Default-param extender for [`TextServer::font_draw_glyph_outline_ex`][super::TextServer::font_draw_glyph_outline_ex]."]
#[must_use]
pub struct ExFontDrawGlyphOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFontDrawGlyphOutline < 'a > {
    fn new(surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_rid: font_rid, canvas: canvas, size: size, outline_size: outline_size, pos: pos, index: index, color: color,
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
            _phantom, surround_object, font_rid, canvas, size, outline_size, pos, index, color,
        }
        = self;
        re_export::TextServer::font_draw_glyph_outline_full(surround_object, font_rid, canvas, size, outline_size, pos, index, color,)
    }
}
#[doc = "Default-param extender for [`TextServer::create_shaped_text_ex`][super::TextServer::create_shaped_text_ex]."]
#[must_use]
pub struct ExCreateShapedText < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateShapedText < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer,) -> Self {
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, direction: direction, orientation: orientation,
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
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, direction, orientation,
        }
        = self;
        re_export::TextServer::create_shaped_text_full(surround_object, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_set_direction_ex`][super::TextServer::shaped_text_set_direction_ex]."]
#[must_use]
pub struct ExShapedTextSetDirection < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, direction: crate::classes::text_server::Direction,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextSetDirection < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        let direction = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, direction: direction,
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, direction,
        }
        = self;
        re_export::TextServer::shaped_text_set_direction_full(surround_object, shaped, direction,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_set_orientation_ex`][super::TextServer::shaped_text_set_orientation_ex]."]
#[must_use]
pub struct ExShapedTextSetOrientation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextSetOrientation < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, orientation: orientation,
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
            _phantom, surround_object, shaped, orientation,
        }
        = self;
        re_export::TextServer::shaped_text_set_orientation_full(surround_object, shaped, orientation,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_add_string_ex`][super::TextServer::shaped_text_add_string_ex]."]
#[must_use]
pub struct ExShapedTextAddString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, text: CowArg < 'a, GString >, fonts: CowArg < 'a, Array < Rid > >, size: i64, opentype_features: CowArg < 'a, Dictionary >, language: CowArg < 'a, GString >, meta: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextAddString < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, text: impl AsArg < GString > + 'a, fonts: &'a Array < Rid >, size: i64,) -> Self {
        let opentype_features = Dictionary::new();
        let language = GString::from("");
        let meta = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, text: text.into_arg(), fonts: CowArg::Borrowed(fonts), size: size, opentype_features: CowArg::Owned(opentype_features), language: CowArg::Owned(language), meta: CowArg::Owned(meta),
        }
    }
    #[inline]
    pub fn opentype_features(self, opentype_features: &'a Dictionary) -> Self {
        Self {
            opentype_features: CowArg::Borrowed(opentype_features), .. self
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
            _phantom, surround_object, shaped, text, fonts, size, opentype_features, language, meta,
        }
        = self;
        re_export::TextServer::shaped_text_add_string_full(surround_object, shaped, text, fonts.cow_as_arg(), size, opentype_features.cow_as_arg(), language, meta.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_add_object_ex`][super::TextServer::shaped_text_add_object_ex]."]
#[must_use]
pub struct ExShapedTextAddObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, key: CowArg < 'a, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i64, baseline: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextAddObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, key: &'a Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let length = 1i64;
        let baseline = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, length: length, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn length(self, length: i64) -> Self {
        Self {
            length: length, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f64) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, shaped, key, size, inline_align, length, baseline,
        }
        = self;
        re_export::TextServer::shaped_text_add_object_full(surround_object, shaped, key.cow_as_arg(), size, inline_align, length, baseline,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_resize_object_ex`][super::TextServer::shaped_text_resize_object_ex]."]
#[must_use]
pub struct ExShapedTextResizeObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, key: CowArg < 'a, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextResizeObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, key: &'a Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let baseline = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f64) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, shaped, key, size, inline_align, baseline,
        }
        = self;
        re_export::TextServer::shaped_text_resize_object_full(surround_object, shaped, key.cow_as_arg(), size, inline_align, baseline,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_set_span_update_font_ex`][super::TextServer::shaped_set_span_update_font_ex]."]
#[must_use]
pub struct ExShapedSetSpanUpdateFont < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, index: i64, fonts: CowArg < 'a, Array < Rid > >, size: i64, opentype_features: CowArg < 'a, Dictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedSetSpanUpdateFont < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, index: i64, fonts: &'a Array < Rid >, size: i64,) -> Self {
        let opentype_features = Dictionary::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, index: index, fonts: CowArg::Borrowed(fonts), size: size, opentype_features: CowArg::Owned(opentype_features),
        }
    }
    #[inline]
    pub fn opentype_features(self, opentype_features: &'a Dictionary) -> Self {
        Self {
            opentype_features: CowArg::Borrowed(opentype_features), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, index, fonts, size, opentype_features,
        }
        = self;
        re_export::TextServer::shaped_set_span_update_font_full(surround_object, shaped, index, fonts.cow_as_arg(), size, opentype_features.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_fit_to_width_ex`][super::TextServer::shaped_text_fit_to_width_ex]."]
#[must_use]
pub struct ExShapedTextFitToWidth < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64, justification_flags: crate::classes::text_server::JustificationFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextFitToWidth < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64,) -> Self {
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, justification_flags: justification_flags,
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        let Self {
            _phantom, surround_object, shaped, width, justification_flags,
        }
        = self;
        re_export::TextServer::shaped_text_fit_to_width_full(surround_object, shaped, width, justification_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_line_breaks_adv_ex`][super::TextServer::shaped_text_get_line_breaks_adv_ex]."]
#[must_use]
pub struct ExShapedTextGetLineBreaksAdv < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, width: CowArg < 'a, PackedFloat32Array >, start: i64, once: bool, break_flags: crate::classes::text_server::LineBreakFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetLineBreaksAdv < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, width: &'a PackedFloat32Array,) -> Self {
        let start = 0i64;
        let once = true;
        let break_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: CowArg::Borrowed(width), start: start, once: once, break_flags: break_flags,
        }
    }
    #[inline]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: start, .. self
        }
    }
    #[inline]
    pub fn once(self, once: bool) -> Self {
        Self {
            once: once, .. self
        }
    }
    #[inline]
    pub fn break_flags(self, break_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            break_flags: break_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, width, start, once, break_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_line_breaks_adv_full(surround_object, shaped, width.cow_as_arg(), start, once, break_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_line_breaks_ex`][super::TextServer::shaped_text_get_line_breaks_ex]."]
#[must_use]
pub struct ExShapedTextGetLineBreaks < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, width: f64, start: i64, break_flags: crate::classes::text_server::LineBreakFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetLineBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, width: f64,) -> Self {
        let start = 0i64;
        let break_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, start: start, break_flags: break_flags,
        }
    }
    #[inline]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: start, .. self
        }
    }
    #[inline]
    pub fn break_flags(self, break_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            break_flags: break_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, width, start, break_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_line_breaks_full(surround_object, shaped, width, start, break_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_word_breaks_ex`][super::TextServer::shaped_text_get_word_breaks_ex]."]
#[must_use]
pub struct ExShapedTextGetWordBreaks < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, grapheme_flags: crate::classes::text_server::GraphemeFlag, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetWordBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid,) -> Self {
        let grapheme_flags = crate::obj::EngineBitfield::from_ord(264);
        let skip_grapheme_flags = crate::obj::EngineBitfield::from_ord(4);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, grapheme_flags: grapheme_flags, skip_grapheme_flags: skip_grapheme_flags,
        }
    }
    #[inline]
    pub fn grapheme_flags(self, grapheme_flags: crate::classes::text_server::GraphemeFlag) -> Self {
        Self {
            grapheme_flags: grapheme_flags, .. self
        }
    }
    #[inline]
    pub fn skip_grapheme_flags(self, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag) -> Self {
        Self {
            skip_grapheme_flags: skip_grapheme_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, grapheme_flags, skip_grapheme_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_word_breaks_full(surround_object, shaped, grapheme_flags, skip_grapheme_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_overrun_trim_to_width_ex`][super::TextServer::shaped_text_overrun_trim_to_width_ex]."]
#[must_use]
pub struct ExShapedTextOverrunTrimToWidth < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextOverrunTrimToWidth < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        let width = 0f64;
        let overrun_trim_flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, overrun_trim_flags: overrun_trim_flags,
        }
    }
    #[inline]
    pub fn width(self, width: f64) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn overrun_trim_flags(self, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag) -> Self {
        Self {
            overrun_trim_flags: overrun_trim_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, width, overrun_trim_flags,
        }
        = self;
        re_export::TextServer::shaped_text_overrun_trim_to_width_full(surround_object, shaped, width, overrun_trim_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_draw_ex`][super::TextServer::shaped_text_draw_ex]."]
#[must_use]
pub struct ExShapedTextDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextDraw < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2,) -> Self {
        let clip_l = - 1f64;
        let clip_r = - 1f64;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, canvas: canvas, pos: pos, clip_l: clip_l, clip_r: clip_r, color: color,
        }
    }
    #[inline]
    pub fn clip_l(self, clip_l: f64) -> Self {
        Self {
            clip_l: clip_l, .. self
        }
    }
    #[inline]
    pub fn clip_r(self, clip_r: f64) -> Self {
        Self {
            clip_r: clip_r, .. self
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
            _phantom, surround_object, shaped, canvas, pos, clip_l, clip_r, color,
        }
        = self;
        re_export::TextServer::shaped_text_draw_full(surround_object, shaped, canvas, pos, clip_l, clip_r, color,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_draw_outline_ex`][super::TextServer::shaped_text_draw_outline_ex]."]
#[must_use]
pub struct ExShapedTextDrawOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextDrawOutline < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2,) -> Self {
        let clip_l = - 1f64;
        let clip_r = - 1f64;
        let outline_size = 1i64;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, canvas: canvas, pos: pos, clip_l: clip_l, clip_r: clip_r, outline_size: outline_size, color: color,
        }
    }
    #[inline]
    pub fn clip_l(self, clip_l: f64) -> Self {
        Self {
            clip_l: clip_l, .. self
        }
    }
    #[inline]
    pub fn clip_r(self, clip_r: f64) -> Self {
        Self {
            clip_r: clip_r, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i64) -> Self {
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
            _phantom, surround_object, shaped, canvas, pos, clip_l, clip_r, outline_size, color,
        }
        = self;
        re_export::TextServer::shaped_text_draw_outline_full(surround_object, shaped, canvas, pos, clip_l, clip_r, outline_size, color,)
    }
}
#[doc = "Default-param extender for [`TextServer::format_number_ex`][super::TextServer::format_number_ex]."]
#[must_use]
pub struct ExFormatNumber < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, number: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFormatNumber < 'a > {
    fn new(surround_object: &'a re_export::TextServer, number: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, number: number.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, number, language,
        }
        = self;
        re_export::TextServer::format_number_full(surround_object, number, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::parse_number_ex`][super::TextServer::parse_number_ex]."]
#[must_use]
pub struct ExParseNumber < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, number: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExParseNumber < 'a > {
    fn new(surround_object: &'a re_export::TextServer, number: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, number: number.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, number, language,
        }
        = self;
        re_export::TextServer::parse_number_full(surround_object, number, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::percent_sign_ex`][super::TextServer::percent_sign_ex]."]
#[must_use]
pub struct ExPercentSign < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPercentSign < 'a > {
    fn new(surround_object: &'a re_export::TextServer,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, language,
        }
        = self;
        re_export::TextServer::percent_sign_full(surround_object, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_get_word_breaks_ex`][super::TextServer::string_get_word_breaks_ex]."]
#[must_use]
pub struct ExStringGetWordBreaks < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >, chars_per_line: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringGetWordBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        let chars_per_line = 0i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language), chars_per_line: chars_per_line,
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn chars_per_line(self, chars_per_line: i64) -> Self {
        Self {
            chars_per_line: chars_per_line, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, string, language, chars_per_line,
        }
        = self;
        re_export::TextServer::string_get_word_breaks_full(surround_object, string, language, chars_per_line,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_get_character_breaks_ex`][super::TextServer::string_get_character_breaks_ex]."]
#[must_use]
pub struct ExStringGetCharacterBreaks < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringGetCharacterBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_get_character_breaks_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_upper_ex`][super::TextServer::string_to_upper_ex]."]
#[must_use]
pub struct ExStringToUpper < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringToUpper < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_upper_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_lower_ex`][super::TextServer::string_to_lower_ex]."]
#[must_use]
pub struct ExStringToLower < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringToLower < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_lower_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_title_ex`][super::TextServer::string_to_title_ex]."]
#[must_use]
pub struct ExStringToTitle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringToTitle < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_title_full(surround_object, string, language,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FontAntialiasing {
    ord: i32
}
impl FontAntialiasing {
    #[doc(alias = "FONT_ANTIALIASING_NONE")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_NONE`"]
    pub const NONE: FontAntialiasing = FontAntialiasing {
        ord: 0i32
    };
    #[doc(alias = "FONT_ANTIALIASING_GRAY")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_GRAY`"]
    pub const GRAY: FontAntialiasing = FontAntialiasing {
        ord: 1i32
    };
    #[doc(alias = "FONT_ANTIALIASING_LCD")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_LCD`"]
    pub const LCD: FontAntialiasing = FontAntialiasing {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FontAntialiasing {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FontAntialiasing") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FontAntialiasing {
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
            Self::NONE => "NONE", Self::GRAY => "GRAY", Self::LCD => "LCD", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "FONT_ANTIALIASING_NONE", Self::GRAY => "FONT_ANTIALIASING_GRAY", Self::LCD => "FONT_ANTIALIASING_LCD", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FontAntialiasing {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FontAntialiasing {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontAntialiasing {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `FontLCDSubpixelLayout`."]
pub struct FontLcdSubpixelLayout {
    ord: i32
}
impl FontLcdSubpixelLayout {
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_NONE")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_NONE`"]
    pub const NONE: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 0i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_HRGB")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_HRGB`"]
    pub const HRGB: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 1i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_HBGR")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_HBGR`"]
    pub const HBGR: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 2i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_VRGB")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_VRGB`"]
    pub const VRGB: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 3i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_VBGR")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_VBGR`"]
    pub const VBGR: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 4i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_MAX")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_MAX`"]
    pub const MAX: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for FontLcdSubpixelLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FontLcdSubpixelLayout") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FontLcdSubpixelLayout {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::NONE => "NONE", Self::HRGB => "HRGB", Self::HBGR => "HBGR", Self::VRGB => "VRGB", Self::VBGR => "VBGR", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "FONT_LCD_SUBPIXEL_LAYOUT_NONE", Self::HRGB => "FONT_LCD_SUBPIXEL_LAYOUT_HRGB", Self::HBGR => "FONT_LCD_SUBPIXEL_LAYOUT_HBGR", Self::VRGB => "FONT_LCD_SUBPIXEL_LAYOUT_VRGB", Self::VBGR => "FONT_LCD_SUBPIXEL_LAYOUT_VBGR", Self::MAX => "FONT_LCD_SUBPIXEL_LAYOUT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for FontLcdSubpixelLayout {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for FontLcdSubpixelLayout {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FontLcdSubpixelLayout {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontLcdSubpixelLayout {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Direction {
    ord: i32
}
impl Direction {
    #[doc(alias = "DIRECTION_AUTO")]
    #[doc = "Godot enumerator name: `DIRECTION_AUTO`"]
    pub const AUTO: Direction = Direction {
        ord: 0i32
    };
    #[doc(alias = "DIRECTION_LTR")]
    #[doc = "Godot enumerator name: `DIRECTION_LTR`"]
    pub const LTR: Direction = Direction {
        ord: 1i32
    };
    #[doc(alias = "DIRECTION_RTL")]
    #[doc = "Godot enumerator name: `DIRECTION_RTL`"]
    pub const RTL: Direction = Direction {
        ord: 2i32
    };
    #[doc(alias = "DIRECTION_INHERITED")]
    #[doc = "Godot enumerator name: `DIRECTION_INHERITED`"]
    pub const INHERITED: Direction = Direction {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Direction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Direction {
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
            Self::AUTO => "AUTO", Self::LTR => "LTR", Self::RTL => "RTL", Self::INHERITED => "INHERITED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::AUTO => "DIRECTION_AUTO", Self::LTR => "DIRECTION_LTR", Self::RTL => "DIRECTION_RTL", Self::INHERITED => "DIRECTION_INHERITED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Direction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Direction {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Direction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Orientation {
    ord: i32
}
impl Orientation {
    #[doc(alias = "ORIENTATION_HORIZONTAL")]
    #[doc = "Godot enumerator name: `ORIENTATION_HORIZONTAL`"]
    pub const HORIZONTAL: Orientation = Orientation {
        ord: 0i32
    };
    #[doc(alias = "ORIENTATION_VERTICAL")]
    #[doc = "Godot enumerator name: `ORIENTATION_VERTICAL`"]
    pub const VERTICAL: Orientation = Orientation {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Orientation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Orientation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::HORIZONTAL => "HORIZONTAL", Self::VERTICAL => "VERTICAL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::HORIZONTAL => "ORIENTATION_HORIZONTAL", Self::VERTICAL => "ORIENTATION_VERTICAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Orientation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Orientation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Orientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct JustificationFlag {
    ord: u64
}
impl JustificationFlag {
    #[doc(alias = "JUSTIFICATION_NONE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_NONE`"]
    pub const NONE: JustificationFlag = JustificationFlag {
        ord: 0u64
    };
    #[doc(alias = "JUSTIFICATION_KASHIDA")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_KASHIDA`"]
    pub const KASHIDA: JustificationFlag = JustificationFlag {
        ord: 1u64
    };
    #[doc(alias = "JUSTIFICATION_WORD_BOUND")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_WORD_BOUND`"]
    pub const WORD_BOUND: JustificationFlag = JustificationFlag {
        ord: 2u64
    };
    #[doc(alias = "JUSTIFICATION_TRIM_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_TRIM_EDGE_SPACES`"]
    pub const TRIM_EDGE_SPACES: JustificationFlag = JustificationFlag {
        ord: 4u64
    };
    #[doc(alias = "JUSTIFICATION_AFTER_LAST_TAB")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_AFTER_LAST_TAB`"]
    pub const AFTER_LAST_TAB: JustificationFlag = JustificationFlag {
        ord: 8u64
    };
    #[doc(alias = "JUSTIFICATION_CONSTRAIN_ELLIPSIS")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_CONSTRAIN_ELLIPSIS`"]
    pub const CONSTRAIN_ELLIPSIS: JustificationFlag = JustificationFlag {
        ord: 16u64
    };
    #[doc(alias = "JUSTIFICATION_SKIP_LAST_LINE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_SKIP_LAST_LINE`"]
    pub const SKIP_LAST_LINE: JustificationFlag = JustificationFlag {
        ord: 32u64
    };
    #[doc(alias = "JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS`"]
    pub const SKIP_LAST_LINE_WITH_VISIBLE_CHARS: JustificationFlag = JustificationFlag {
        ord: 64u64
    };
    #[doc(alias = "JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE`"]
    pub const DO_NOT_SKIP_SINGLE_LINE: JustificationFlag = JustificationFlag {
        ord: 128u64
    };
    
}
impl std::fmt::Debug for JustificationFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::KASHIDA => "KASHIDA", Self::WORD_BOUND => "WORD_BOUND", Self::TRIM_EDGE_SPACES => "TRIM_EDGE_SPACES", Self::AFTER_LAST_TAB => "AFTER_LAST_TAB", Self::CONSTRAIN_ELLIPSIS => "CONSTRAIN_ELLIPSIS", Self::SKIP_LAST_LINE => "SKIP_LAST_LINE", Self::SKIP_LAST_LINE_WITH_VISIBLE_CHARS => "SKIP_LAST_LINE_WITH_VISIBLE_CHARS", Self::DO_NOT_SKIP_SINGLE_LINE => "DO_NOT_SKIP_SINGLE_LINE", _ => {
                f.debug_struct("JustificationFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for JustificationFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for JustificationFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for JustificationFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for JustificationFlag {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JustificationFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AutowrapMode {
    ord: i32
}
impl AutowrapMode {
    #[doc(alias = "AUTOWRAP_OFF")]
    #[doc = "Godot enumerator name: `AUTOWRAP_OFF`"]
    pub const OFF: AutowrapMode = AutowrapMode {
        ord: 0i32
    };
    #[doc(alias = "AUTOWRAP_ARBITRARY")]
    #[doc = "Godot enumerator name: `AUTOWRAP_ARBITRARY`"]
    pub const ARBITRARY: AutowrapMode = AutowrapMode {
        ord: 1i32
    };
    #[doc(alias = "AUTOWRAP_WORD")]
    #[doc = "Godot enumerator name: `AUTOWRAP_WORD`"]
    pub const WORD: AutowrapMode = AutowrapMode {
        ord: 2i32
    };
    #[doc(alias = "AUTOWRAP_WORD_SMART")]
    #[doc = "Godot enumerator name: `AUTOWRAP_WORD_SMART`"]
    pub const WORD_SMART: AutowrapMode = AutowrapMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AutowrapMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AutowrapMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AutowrapMode {
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
            Self::OFF => "OFF", Self::ARBITRARY => "ARBITRARY", Self::WORD => "WORD", Self::WORD_SMART => "WORD_SMART", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OFF => "AUTOWRAP_OFF", Self::ARBITRARY => "AUTOWRAP_ARBITRARY", Self::WORD => "AUTOWRAP_WORD", Self::WORD_SMART => "AUTOWRAP_WORD_SMART", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AutowrapMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AutowrapMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AutowrapMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct LineBreakFlag {
    ord: u64
}
impl LineBreakFlag {
    #[doc(alias = "BREAK_NONE")]
    #[doc = "Godot enumerator name: `BREAK_NONE`"]
    pub const NONE: LineBreakFlag = LineBreakFlag {
        ord: 0u64
    };
    #[doc(alias = "BREAK_MANDATORY")]
    #[doc = "Godot enumerator name: `BREAK_MANDATORY`"]
    pub const MANDATORY: LineBreakFlag = LineBreakFlag {
        ord: 1u64
    };
    #[doc(alias = "BREAK_WORD_BOUND")]
    #[doc = "Godot enumerator name: `BREAK_WORD_BOUND`"]
    pub const WORD_BOUND: LineBreakFlag = LineBreakFlag {
        ord: 2u64
    };
    #[doc(alias = "BREAK_GRAPHEME_BOUND")]
    #[doc = "Godot enumerator name: `BREAK_GRAPHEME_BOUND`"]
    pub const GRAPHEME_BOUND: LineBreakFlag = LineBreakFlag {
        ord: 4u64
    };
    #[doc(alias = "BREAK_ADAPTIVE")]
    #[doc = "Godot enumerator name: `BREAK_ADAPTIVE`"]
    pub const ADAPTIVE: LineBreakFlag = LineBreakFlag {
        ord: 8u64
    };
    #[doc(alias = "BREAK_TRIM_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_EDGE_SPACES`"]
    pub const TRIM_EDGE_SPACES: LineBreakFlag = LineBreakFlag {
        ord: 16u64
    };
    #[doc(alias = "BREAK_TRIM_INDENT")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_INDENT`"]
    pub const TRIM_INDENT: LineBreakFlag = LineBreakFlag {
        ord: 32u64
    };
    
}
impl std::fmt::Debug for LineBreakFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::MANDATORY => "MANDATORY", Self::WORD_BOUND => "WORD_BOUND", Self::GRAPHEME_BOUND => "GRAPHEME_BOUND", Self::ADAPTIVE => "ADAPTIVE", Self::TRIM_EDGE_SPACES => "TRIM_EDGE_SPACES", Self::TRIM_INDENT => "TRIM_INDENT", _ => {
                f.debug_struct("LineBreakFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for LineBreakFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for LineBreakFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for LineBreakFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for LineBreakFlag {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LineBreakFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibleCharactersBehavior {
    ord: i32
}
impl VisibleCharactersBehavior {
    #[doc(alias = "VC_CHARS_BEFORE_SHAPING")]
    #[doc = "Godot enumerator name: `VC_CHARS_BEFORE_SHAPING`"]
    pub const CHARS_BEFORE_SHAPING: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 0i32
    };
    #[doc(alias = "VC_CHARS_AFTER_SHAPING")]
    #[doc = "Godot enumerator name: `VC_CHARS_AFTER_SHAPING`"]
    pub const CHARS_AFTER_SHAPING: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 1i32
    };
    #[doc(alias = "VC_GLYPHS_AUTO")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_AUTO`"]
    pub const GLYPHS_AUTO: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 2i32
    };
    #[doc(alias = "VC_GLYPHS_LTR")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_LTR`"]
    pub const GLYPHS_LTR: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 3i32
    };
    #[doc(alias = "VC_GLYPHS_RTL")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_RTL`"]
    pub const GLYPHS_RTL: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for VisibleCharactersBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibleCharactersBehavior") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibleCharactersBehavior {
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
            Self::CHARS_BEFORE_SHAPING => "CHARS_BEFORE_SHAPING", Self::CHARS_AFTER_SHAPING => "CHARS_AFTER_SHAPING", Self::GLYPHS_AUTO => "GLYPHS_AUTO", Self::GLYPHS_LTR => "GLYPHS_LTR", Self::GLYPHS_RTL => "GLYPHS_RTL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CHARS_BEFORE_SHAPING => "VC_CHARS_BEFORE_SHAPING", Self::CHARS_AFTER_SHAPING => "VC_CHARS_AFTER_SHAPING", Self::GLYPHS_AUTO => "VC_GLYPHS_AUTO", Self::GLYPHS_LTR => "VC_GLYPHS_LTR", Self::GLYPHS_RTL => "VC_GLYPHS_RTL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for VisibleCharactersBehavior {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VisibleCharactersBehavior {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibleCharactersBehavior {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OverrunBehavior {
    ord: i32
}
impl OverrunBehavior {
    #[doc(alias = "OVERRUN_NO_TRIMMING")]
    #[doc = "Godot enumerator name: `OVERRUN_NO_TRIMMING`"]
    pub const NO_TRIMMING: OverrunBehavior = OverrunBehavior {
        ord: 0i32
    };
    #[doc(alias = "OVERRUN_TRIM_CHAR")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_CHAR`"]
    pub const TRIM_CHAR: OverrunBehavior = OverrunBehavior {
        ord: 1i32
    };
    #[doc(alias = "OVERRUN_TRIM_WORD")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD`"]
    pub const TRIM_WORD: OverrunBehavior = OverrunBehavior {
        ord: 2i32
    };
    #[doc(alias = "OVERRUN_TRIM_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_ELLIPSIS`"]
    pub const TRIM_ELLIPSIS: OverrunBehavior = OverrunBehavior {
        ord: 3i32
    };
    #[doc(alias = "OVERRUN_TRIM_WORD_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD_ELLIPSIS`"]
    pub const TRIM_WORD_ELLIPSIS: OverrunBehavior = OverrunBehavior {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for OverrunBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OverrunBehavior") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OverrunBehavior {
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
            Self::NO_TRIMMING => "NO_TRIMMING", Self::TRIM_CHAR => "TRIM_CHAR", Self::TRIM_WORD => "TRIM_WORD", Self::TRIM_ELLIPSIS => "TRIM_ELLIPSIS", Self::TRIM_WORD_ELLIPSIS => "TRIM_WORD_ELLIPSIS", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NO_TRIMMING => "OVERRUN_NO_TRIMMING", Self::TRIM_CHAR => "OVERRUN_TRIM_CHAR", Self::TRIM_WORD => "OVERRUN_TRIM_WORD", Self::TRIM_ELLIPSIS => "OVERRUN_TRIM_ELLIPSIS", Self::TRIM_WORD_ELLIPSIS => "OVERRUN_TRIM_WORD_ELLIPSIS", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for OverrunBehavior {
    type Via = i32;
    
}
impl crate::meta::ToGodot for OverrunBehavior {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OverrunBehavior {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TextOverrunFlag {
    ord: u64
}
impl TextOverrunFlag {
    #[doc(alias = "OVERRUN_NO_TRIM")]
    #[doc = "Godot enumerator name: `OVERRUN_NO_TRIM`"]
    pub const NO_TRIM: TextOverrunFlag = TextOverrunFlag {
        ord: 0u64
    };
    #[doc(alias = "OVERRUN_TRIM")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM`"]
    pub const TRIM: TextOverrunFlag = TextOverrunFlag {
        ord: 1u64
    };
    #[doc(alias = "OVERRUN_TRIM_WORD_ONLY")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD_ONLY`"]
    pub const TRIM_WORD_ONLY: TextOverrunFlag = TextOverrunFlag {
        ord: 2u64
    };
    #[doc(alias = "OVERRUN_ADD_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_ADD_ELLIPSIS`"]
    pub const ADD_ELLIPSIS: TextOverrunFlag = TextOverrunFlag {
        ord: 4u64
    };
    #[doc(alias = "OVERRUN_ENFORCE_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_ENFORCE_ELLIPSIS`"]
    pub const ENFORCE_ELLIPSIS: TextOverrunFlag = TextOverrunFlag {
        ord: 8u64
    };
    #[doc(alias = "OVERRUN_JUSTIFICATION_AWARE")]
    #[doc = "Godot enumerator name: `OVERRUN_JUSTIFICATION_AWARE`"]
    pub const JUSTIFICATION_AWARE: TextOverrunFlag = TextOverrunFlag {
        ord: 16u64
    };
    
}
impl std::fmt::Debug for TextOverrunFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NO_TRIM => "NO_TRIM", Self::TRIM => "TRIM", Self::TRIM_WORD_ONLY => "TRIM_WORD_ONLY", Self::ADD_ELLIPSIS => "ADD_ELLIPSIS", Self::ENFORCE_ELLIPSIS => "ENFORCE_ELLIPSIS", Self::JUSTIFICATION_AWARE => "JUSTIFICATION_AWARE", _ => {
                f.debug_struct("TextOverrunFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for TextOverrunFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for TextOverrunFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for TextOverrunFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for TextOverrunFlag {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextOverrunFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct GraphemeFlag {
    ord: u64
}
impl GraphemeFlag {
    #[doc(alias = "GRAPHEME_IS_VALID")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_VALID`"]
    pub const VALID: GraphemeFlag = GraphemeFlag {
        ord: 1u64
    };
    #[doc(alias = "GRAPHEME_IS_RTL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_RTL`"]
    pub const RTL: GraphemeFlag = GraphemeFlag {
        ord: 2u64
    };
    #[doc(alias = "GRAPHEME_IS_VIRTUAL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_VIRTUAL`"]
    pub const VIRTUAL: GraphemeFlag = GraphemeFlag {
        ord: 4u64
    };
    #[doc(alias = "GRAPHEME_IS_SPACE")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SPACE`"]
    pub const SPACE: GraphemeFlag = GraphemeFlag {
        ord: 8u64
    };
    #[doc(alias = "GRAPHEME_IS_BREAK_HARD")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_BREAK_HARD`"]
    pub const BREAK_HARD: GraphemeFlag = GraphemeFlag {
        ord: 16u64
    };
    #[doc(alias = "GRAPHEME_IS_BREAK_SOFT")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_BREAK_SOFT`"]
    pub const BREAK_SOFT: GraphemeFlag = GraphemeFlag {
        ord: 32u64
    };
    #[doc(alias = "GRAPHEME_IS_TAB")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_TAB`"]
    pub const TAB: GraphemeFlag = GraphemeFlag {
        ord: 64u64
    };
    #[doc(alias = "GRAPHEME_IS_ELONGATION")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_ELONGATION`"]
    pub const ELONGATION: GraphemeFlag = GraphemeFlag {
        ord: 128u64
    };
    #[doc(alias = "GRAPHEME_IS_PUNCTUATION")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_PUNCTUATION`"]
    pub const PUNCTUATION: GraphemeFlag = GraphemeFlag {
        ord: 256u64
    };
    #[doc(alias = "GRAPHEME_IS_UNDERSCORE")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_UNDERSCORE`"]
    pub const UNDERSCORE: GraphemeFlag = GraphemeFlag {
        ord: 512u64
    };
    #[doc(alias = "GRAPHEME_IS_CONNECTED")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_CONNECTED`"]
    pub const CONNECTED: GraphemeFlag = GraphemeFlag {
        ord: 1024u64
    };
    #[doc(alias = "GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL`"]
    pub const SAFE_TO_INSERT_TATWEEL: GraphemeFlag = GraphemeFlag {
        ord: 2048u64
    };
    #[doc(alias = "GRAPHEME_IS_EMBEDDED_OBJECT")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_EMBEDDED_OBJECT`"]
    pub const EMBEDDED_OBJECT: GraphemeFlag = GraphemeFlag {
        ord: 4096u64
    };
    #[doc(alias = "GRAPHEME_IS_SOFT_HYPHEN")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SOFT_HYPHEN`"]
    pub const SOFT_HYPHEN: GraphemeFlag = GraphemeFlag {
        ord: 8192u64
    };
    
}
impl std::fmt::Debug for GraphemeFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::VALID => "VALID", Self::RTL => "RTL", Self::VIRTUAL => "VIRTUAL", Self::SPACE => "SPACE", Self::BREAK_HARD => "BREAK_HARD", Self::BREAK_SOFT => "BREAK_SOFT", Self::TAB => "TAB", Self::ELONGATION => "ELONGATION", Self::PUNCTUATION => "PUNCTUATION", Self::UNDERSCORE => "UNDERSCORE", Self::CONNECTED => "CONNECTED", Self::SAFE_TO_INSERT_TATWEEL => "SAFE_TO_INSERT_TATWEEL", Self::EMBEDDED_OBJECT => "EMBEDDED_OBJECT", Self::SOFT_HYPHEN => "SOFT_HYPHEN", _ => {
                f.debug_struct("GraphemeFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for GraphemeFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for GraphemeFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for GraphemeFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for GraphemeFlag {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GraphemeFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Hinting {
    ord: i32
}
impl Hinting {
    #[doc(alias = "HINTING_NONE")]
    #[doc = "Godot enumerator name: `HINTING_NONE`"]
    pub const NONE: Hinting = Hinting {
        ord: 0i32
    };
    #[doc(alias = "HINTING_LIGHT")]
    #[doc = "Godot enumerator name: `HINTING_LIGHT`"]
    pub const LIGHT: Hinting = Hinting {
        ord: 1i32
    };
    #[doc(alias = "HINTING_NORMAL")]
    #[doc = "Godot enumerator name: `HINTING_NORMAL`"]
    pub const NORMAL: Hinting = Hinting {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Hinting {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Hinting") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Hinting {
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
            Self::NONE => "NONE", Self::LIGHT => "LIGHT", Self::NORMAL => "NORMAL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "HINTING_NONE", Self::LIGHT => "HINTING_LIGHT", Self::NORMAL => "HINTING_NORMAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Hinting {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Hinting {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Hinting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SubpixelPositioning {
    ord: i32
}
impl SubpixelPositioning {
    #[doc(alias = "SUBPIXEL_POSITIONING_DISABLED")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_DISABLED`"]
    pub const DISABLED: SubpixelPositioning = SubpixelPositioning {
        ord: 0i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_AUTO")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_AUTO`"]
    pub const AUTO: SubpixelPositioning = SubpixelPositioning {
        ord: 1i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_HALF")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_HALF`"]
    pub const ONE_HALF: SubpixelPositioning = SubpixelPositioning {
        ord: 2i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_QUARTER")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_QUARTER`"]
    pub const ONE_QUARTER: SubpixelPositioning = SubpixelPositioning {
        ord: 3i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE`"]
    pub const ONE_HALF_MAX_SIZE: SubpixelPositioning = SubpixelPositioning {
        ord: 20i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE`"]
    pub const ONE_QUARTER_MAX_SIZE: SubpixelPositioning = SubpixelPositioning {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for SubpixelPositioning {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SubpixelPositioning") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SubpixelPositioning {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 16i32 | ord @ 20i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::AUTO => "AUTO", Self::ONE_HALF => "ONE_HALF", Self::ONE_QUARTER => "ONE_QUARTER", Self::ONE_HALF_MAX_SIZE => "ONE_HALF_MAX_SIZE", Self::ONE_QUARTER_MAX_SIZE => "ONE_QUARTER_MAX_SIZE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "SUBPIXEL_POSITIONING_DISABLED", Self::AUTO => "SUBPIXEL_POSITIONING_AUTO", Self::ONE_HALF => "SUBPIXEL_POSITIONING_ONE_HALF", Self::ONE_QUARTER => "SUBPIXEL_POSITIONING_ONE_QUARTER", Self::ONE_HALF_MAX_SIZE => "SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE", Self::ONE_QUARTER_MAX_SIZE => "SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SubpixelPositioning {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SubpixelPositioning {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SubpixelPositioning {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    #[doc(alias = "FEATURE_SIMPLE_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_SIMPLE_LAYOUT`"]
    pub const SIMPLE_LAYOUT: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_BIDI_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_BIDI_LAYOUT`"]
    pub const BIDI_LAYOUT: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_VERTICAL_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_VERTICAL_LAYOUT`"]
    pub const VERTICAL_LAYOUT: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_SHAPING")]
    #[doc = "Godot enumerator name: `FEATURE_SHAPING`"]
    pub const SHAPING: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_KASHIDA_JUSTIFICATION")]
    #[doc = "Godot enumerator name: `FEATURE_KASHIDA_JUSTIFICATION`"]
    pub const KASHIDA_JUSTIFICATION: Feature = Feature {
        ord: 16i32
    };
    #[doc(alias = "FEATURE_BREAK_ITERATORS")]
    #[doc = "Godot enumerator name: `FEATURE_BREAK_ITERATORS`"]
    pub const BREAK_ITERATORS: Feature = Feature {
        ord: 32i32
    };
    #[doc(alias = "FEATURE_FONT_BITMAP")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_BITMAP`"]
    pub const FONT_BITMAP: Feature = Feature {
        ord: 64i32
    };
    #[doc(alias = "FEATURE_FONT_DYNAMIC")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_DYNAMIC`"]
    pub const FONT_DYNAMIC: Feature = Feature {
        ord: 128i32
    };
    #[doc(alias = "FEATURE_FONT_MSDF")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_MSDF`"]
    pub const FONT_MSDF: Feature = Feature {
        ord: 256i32
    };
    #[doc(alias = "FEATURE_FONT_SYSTEM")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_SYSTEM`"]
    pub const FONT_SYSTEM: Feature = Feature {
        ord: 512i32
    };
    #[doc(alias = "FEATURE_FONT_VARIABLE")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_VARIABLE`"]
    pub const FONT_VARIABLE: Feature = Feature {
        ord: 1024i32
    };
    #[doc(alias = "FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION")]
    #[doc = "Godot enumerator name: `FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION`"]
    pub const CONTEXT_SENSITIVE_CASE_CONVERSION: Feature = Feature {
        ord: 2048i32
    };
    #[doc(alias = "FEATURE_USE_SUPPORT_DATA")]
    #[doc = "Godot enumerator name: `FEATURE_USE_SUPPORT_DATA`"]
    pub const USE_SUPPORT_DATA: Feature = Feature {
        ord: 4096i32
    };
    #[doc(alias = "FEATURE_UNICODE_IDENTIFIERS")]
    #[doc = "Godot enumerator name: `FEATURE_UNICODE_IDENTIFIERS`"]
    pub const UNICODE_IDENTIFIERS: Feature = Feature {
        ord: 8192i32
    };
    #[doc(alias = "FEATURE_UNICODE_SECURITY")]
    #[doc = "Godot enumerator name: `FEATURE_UNICODE_SECURITY`"]
    pub const UNICODE_SECURITY: Feature = Feature {
        ord: 16384i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 | ord @ 64i32 | ord @ 128i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 | ord @ 2048i32 | ord @ 4096i32 | ord @ 8192i32 | ord @ 16384i32 => Some(Self {
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
            Self::SIMPLE_LAYOUT => "SIMPLE_LAYOUT", Self::BIDI_LAYOUT => "BIDI_LAYOUT", Self::VERTICAL_LAYOUT => "VERTICAL_LAYOUT", Self::SHAPING => "SHAPING", Self::KASHIDA_JUSTIFICATION => "KASHIDA_JUSTIFICATION", Self::BREAK_ITERATORS => "BREAK_ITERATORS", Self::FONT_BITMAP => "FONT_BITMAP", Self::FONT_DYNAMIC => "FONT_DYNAMIC", Self::FONT_MSDF => "FONT_MSDF", Self::FONT_SYSTEM => "FONT_SYSTEM", Self::FONT_VARIABLE => "FONT_VARIABLE", Self::CONTEXT_SENSITIVE_CASE_CONVERSION => "CONTEXT_SENSITIVE_CASE_CONVERSION", Self::USE_SUPPORT_DATA => "USE_SUPPORT_DATA", Self::UNICODE_IDENTIFIERS => "UNICODE_IDENTIFIERS", Self::UNICODE_SECURITY => "UNICODE_SECURITY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SIMPLE_LAYOUT => "FEATURE_SIMPLE_LAYOUT", Self::BIDI_LAYOUT => "FEATURE_BIDI_LAYOUT", Self::VERTICAL_LAYOUT => "FEATURE_VERTICAL_LAYOUT", Self::SHAPING => "FEATURE_SHAPING", Self::KASHIDA_JUSTIFICATION => "FEATURE_KASHIDA_JUSTIFICATION", Self::BREAK_ITERATORS => "FEATURE_BREAK_ITERATORS", Self::FONT_BITMAP => "FEATURE_FONT_BITMAP", Self::FONT_DYNAMIC => "FEATURE_FONT_DYNAMIC", Self::FONT_MSDF => "FEATURE_FONT_MSDF", Self::FONT_SYSTEM => "FEATURE_FONT_SYSTEM", Self::FONT_VARIABLE => "FEATURE_FONT_VARIABLE", Self::CONTEXT_SENSITIVE_CASE_CONVERSION => "FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION", Self::USE_SUPPORT_DATA => "FEATURE_USE_SUPPORT_DATA", Self::UNICODE_IDENTIFIERS => "FEATURE_UNICODE_IDENTIFIERS", Self::UNICODE_SECURITY => "FEATURE_UNICODE_SECURITY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Feature {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ContourPointTag {
    ord: i32
}
impl ContourPointTag {
    #[doc(alias = "CONTOUR_CURVE_TAG_ON")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_ON`"]
    pub const ON: ContourPointTag = ContourPointTag {
        ord: 1i32
    };
    #[doc(alias = "CONTOUR_CURVE_TAG_OFF_CONIC")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_OFF_CONIC`"]
    pub const OFF_CONIC: ContourPointTag = ContourPointTag {
        ord: 0i32
    };
    #[doc(alias = "CONTOUR_CURVE_TAG_OFF_CUBIC")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_OFF_CUBIC`"]
    pub const OFF_CUBIC: ContourPointTag = ContourPointTag {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ContourPointTag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ContourPointTag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ContourPointTag {
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
            Self::ON => "ON", Self::OFF_CONIC => "OFF_CONIC", Self::OFF_CUBIC => "OFF_CUBIC", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ON => "CONTOUR_CURVE_TAG_ON", Self::OFF_CONIC => "CONTOUR_CURVE_TAG_OFF_CONIC", Self::OFF_CUBIC => "CONTOUR_CURVE_TAG_OFF_CUBIC", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ContourPointTag {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ContourPointTag {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ContourPointTag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpacingType {
    ord: i32
}
impl SpacingType {
    #[doc(alias = "SPACING_GLYPH")]
    #[doc = "Godot enumerator name: `SPACING_GLYPH`"]
    pub const GLYPH: SpacingType = SpacingType {
        ord: 0i32
    };
    #[doc(alias = "SPACING_SPACE")]
    #[doc = "Godot enumerator name: `SPACING_SPACE`"]
    pub const SPACE: SpacingType = SpacingType {
        ord: 1i32
    };
    #[doc(alias = "SPACING_TOP")]
    #[doc = "Godot enumerator name: `SPACING_TOP`"]
    pub const TOP: SpacingType = SpacingType {
        ord: 2i32
    };
    #[doc(alias = "SPACING_BOTTOM")]
    #[doc = "Godot enumerator name: `SPACING_BOTTOM`"]
    pub const BOTTOM: SpacingType = SpacingType {
        ord: 3i32
    };
    #[doc(alias = "SPACING_MAX")]
    #[doc = "Godot enumerator name: `SPACING_MAX`"]
    pub const MAX: SpacingType = SpacingType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SpacingType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpacingType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpacingType {
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
            Self::GLYPH => "GLYPH", Self::SPACE => "SPACE", Self::TOP => "TOP", Self::BOTTOM => "BOTTOM", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GLYPH => "SPACING_GLYPH", Self::SPACE => "SPACING_SPACE", Self::TOP => "SPACING_TOP", Self::BOTTOM => "SPACING_BOTTOM", Self::MAX => "SPACING_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for SpacingType {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for SpacingType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpacingType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpacingType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct FontStyle {
    ord: u64
}
impl FontStyle {
    #[doc(alias = "FONT_BOLD")]
    #[doc = "Godot enumerator name: `FONT_BOLD`"]
    pub const BOLD: FontStyle = FontStyle {
        ord: 1u64
    };
    #[doc(alias = "FONT_ITALIC")]
    #[doc = "Godot enumerator name: `FONT_ITALIC`"]
    pub const ITALIC: FontStyle = FontStyle {
        ord: 2u64
    };
    #[doc(alias = "FONT_FIXED_WIDTH")]
    #[doc = "Godot enumerator name: `FONT_FIXED_WIDTH`"]
    pub const FIXED_WIDTH: FontStyle = FontStyle {
        ord: 4u64
    };
    
}
impl std::fmt::Debug for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::BOLD => "BOLD", Self::ITALIC => "ITALIC", Self::FIXED_WIDTH => "FIXED_WIDTH", _ => {
                f.debug_struct("FontStyle") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for FontStyle {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for FontStyle {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for FontStyle {
    type Via = u64;
    
}
impl crate::meta::ToGodot for FontStyle {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontStyle {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StructuredTextParser {
    ord: i32
}
impl StructuredTextParser {
    #[doc(alias = "STRUCTURED_TEXT_DEFAULT")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_DEFAULT`"]
    pub const DEFAULT: StructuredTextParser = StructuredTextParser {
        ord: 0i32
    };
    #[doc(alias = "STRUCTURED_TEXT_URI")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_URI`"]
    pub const URI: StructuredTextParser = StructuredTextParser {
        ord: 1i32
    };
    #[doc(alias = "STRUCTURED_TEXT_FILE")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_FILE`"]
    pub const FILE: StructuredTextParser = StructuredTextParser {
        ord: 2i32
    };
    #[doc(alias = "STRUCTURED_TEXT_EMAIL")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_EMAIL`"]
    pub const EMAIL: StructuredTextParser = StructuredTextParser {
        ord: 3i32
    };
    #[doc(alias = "STRUCTURED_TEXT_LIST")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_LIST`"]
    pub const LIST: StructuredTextParser = StructuredTextParser {
        ord: 4i32
    };
    #[doc(alias = "STRUCTURED_TEXT_GDSCRIPT")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_GDSCRIPT`"]
    pub const GDSCRIPT: StructuredTextParser = StructuredTextParser {
        ord: 5i32
    };
    #[doc(alias = "STRUCTURED_TEXT_CUSTOM")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_CUSTOM`"]
    pub const CUSTOM: StructuredTextParser = StructuredTextParser {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for StructuredTextParser {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StructuredTextParser") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StructuredTextParser {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::DEFAULT => "DEFAULT", Self::URI => "URI", Self::FILE => "FILE", Self::EMAIL => "EMAIL", Self::LIST => "LIST", Self::GDSCRIPT => "GDSCRIPT", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "STRUCTURED_TEXT_DEFAULT", Self::URI => "STRUCTURED_TEXT_URI", Self::FILE => "STRUCTURED_TEXT_FILE", Self::EMAIL => "STRUCTURED_TEXT_EMAIL", Self::LIST => "STRUCTURED_TEXT_LIST", Self::GDSCRIPT => "STRUCTURED_TEXT_GDSCRIPT", Self::CUSTOM => "STRUCTURED_TEXT_CUSTOM", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for StructuredTextParser {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StructuredTextParser {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StructuredTextParser {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FixedSizeScaleMode {
    ord: i32
}
impl FixedSizeScaleMode {
    #[doc(alias = "FIXED_SIZE_SCALE_DISABLE")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_DISABLE`"]
    pub const DISABLE: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 0i32
    };
    #[doc(alias = "FIXED_SIZE_SCALE_INTEGER_ONLY")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_INTEGER_ONLY`"]
    pub const INTEGER_ONLY: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 1i32
    };
    #[doc(alias = "FIXED_SIZE_SCALE_ENABLED")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_ENABLED`"]
    pub const ENABLED: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FixedSizeScaleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FixedSizeScaleMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FixedSizeScaleMode {
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
            Self::DISABLE => "DISABLE", Self::INTEGER_ONLY => "INTEGER_ONLY", Self::ENABLED => "ENABLED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLE => "FIXED_SIZE_SCALE_DISABLE", Self::INTEGER_ONLY => "FIXED_SIZE_SCALE_INTEGER_ONLY", Self::ENABLED => "FIXED_SIZE_SCALE_ENABLED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FixedSizeScaleMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FixedSizeScaleMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FixedSizeScaleMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}