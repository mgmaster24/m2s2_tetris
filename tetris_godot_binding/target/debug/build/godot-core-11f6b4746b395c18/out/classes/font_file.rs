#![doc = "Sidecar module for class [`FontFile`][crate::classes::FontFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FontFile` enums](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FontFile.`\n\nInherits [`Font`][crate::classes::Font].\n\nRelated symbols:\n\n* [`IFontFile`][crate::classes::IFontFile]: virtual methods\n\n\nSee also [Godot docs for `FontFile`](https://docs.godotengine.org/en/stable/classes/class_fontfile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`FontFile::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FontFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`FontFile`][crate::classes::FontFile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FontFile` methods](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFontFile: crate::obj::GodotClass < Base = FontFile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FontFile {
        pub fn load_bitmap_font(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "load_bitmap_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_dynamic_font(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "load_dynamic_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_data(&mut self, data: &PackedByteArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedByteArray >);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data(&self,) -> PackedByteArray {
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_name(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_font_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_style_name(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_font_style_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_style(&mut self, style: crate::classes::text_server::FontStyle,) {
            type CallSig = ((), crate::classes::text_server::FontStyle);
            let args = (style,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_font_style", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_weight(&mut self, weight: i32,) {
            type CallSig = ((), i32);
            let args = (weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_font_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_stretch(&mut self, stretch: i32,) {
            type CallSig = ((), i32);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_font_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_antialiasing(&mut self, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            type CallSig = ((), crate::classes::text_server::FontAntialiasing);
            let args = (antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_antialiasing(&self,) -> crate::classes::text_server::FontAntialiasing {
            type CallSig = (crate::classes::text_server::FontAntialiasing,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_embedded_bitmaps(&mut self, disable_embedded_bitmaps: bool,) {
            type CallSig = ((), bool);
            let args = (disable_embedded_bitmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_disable_embedded_bitmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_disable_embedded_bitmaps(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_disable_embedded_bitmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_generate_mipmaps(&mut self, generate_mipmaps: bool,) {
            type CallSig = ((), bool);
            let args = (generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_generate_mipmaps(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_multichannel_signed_distance_field(&mut self, msdf: bool,) {
            type CallSig = ((), bool);
            let args = (msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_multichannel_signed_distance_field(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "is_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, msdf_pixel_range: i32,) {
            type CallSig = ((), i32);
            let args = (msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_size(&mut self, msdf_size: i32,) {
            type CallSig = ((), i32);
            let args = (msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_size(&mut self, fixed_size: i32,) {
            type CallSig = ((), i32);
            let args = (fixed_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_fixed_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_fixed_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_size_scale_mode(&mut self, fixed_size_scale_mode: crate::classes::text_server::FixedSizeScaleMode,) {
            type CallSig = ((), crate::classes::text_server::FixedSizeScaleMode);
            let args = (fixed_size_scale_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_fixed_size_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_size_scale_mode(&self,) -> crate::classes::text_server::FixedSizeScaleMode {
            type CallSig = (crate::classes::text_server::FixedSizeScaleMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_fixed_size_scale_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_system_fallback(&mut self, allow_system_fallback: bool,) {
            type CallSig = ((), bool);
            let args = (allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_allow_system_fallback(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "is_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_force_autohinter(&mut self, force_autohinter: bool,) {
            type CallSig = ((), bool);
            let args = (force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_force_autohinter(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "is_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hinting(&mut self, hinting: crate::classes::text_server::Hinting,) {
            type CallSig = ((), crate::classes::text_server::Hinting);
            let args = (hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hinting(&self,) -> crate::classes::text_server::Hinting {
            type CallSig = (crate::classes::text_server::Hinting,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subpixel_positioning(&mut self, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            type CallSig = ((), crate::classes::text_server::SubpixelPositioning);
            let args = (subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subpixel_positioning(&self,) -> crate::classes::text_server::SubpixelPositioning {
            type CallSig = (crate::classes::text_server::SubpixelPositioning,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_oversampling(&mut self, oversampling: f32,) {
            type CallSig = ((), f32);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_oversampling(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_cache(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "clear_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_cache(&mut self, cache_index: i32,) {
            type CallSig = ((), i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "remove_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size_cache_list(&self, cache_index: i32,) -> Array < Vector2i > {
            type CallSig = (Array < Vector2i >, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_size_cache_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_size_cache(&mut self, cache_index: i32,) {
            type CallSig = ((), i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "clear_size_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_size_cache(&mut self, cache_index: i32, size: Vector2i,) {
            type CallSig = ((), i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "remove_size_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_variation_coordinates(&mut self, cache_index: i32, variation_coordinates: &Dictionary,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Dictionary >);
            let args = (cache_index, RefArg::new(variation_coordinates),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_variation_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_variation_coordinates(&self, cache_index: i32,) -> Dictionary {
            type CallSig = (Dictionary, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_variation_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_embolden(&mut self, cache_index: i32, strength: f32,) {
            type CallSig = ((), i32, f32);
            let args = (cache_index, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_embolden(&self, cache_index: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform(&mut self, cache_index: i32, transform: Transform2D,) {
            type CallSig = ((), i32, Transform2D);
            let args = (cache_index, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform(&self, cache_index: i32,) -> Transform2D {
            type CallSig = (Transform2D, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_extra_spacing(&mut self, cache_index: i32, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallSig = ((), i32, crate::classes::text_server::SpacingType, i64);
            let args = (cache_index, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_extra_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_extra_spacing(&self, cache_index: i32, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallSig = (i64, i32, crate::classes::text_server::SpacingType);
            let args = (cache_index, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_extra_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_extra_baseline_offset(&mut self, cache_index: i32, baseline_offset: f32,) {
            type CallSig = ((), i32, f32);
            let args = (cache_index, baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_extra_baseline_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_extra_baseline_offset(&self, cache_index: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_extra_baseline_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_face_index(&mut self, cache_index: i32, face_index: i64,) {
            type CallSig = ((), i32, i64);
            let args = (cache_index, face_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_index(&self, cache_index: i32,) -> i64 {
            type CallSig = (i64, i32);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_ascent(&mut self, cache_index: i32, size: i32, ascent: f32,) {
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, ascent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_ascent(&self, cache_index: i32, size: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_ascent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_descent(&mut self, cache_index: i32, size: i32, descent: f32,) {
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, descent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_descent(&self, cache_index: i32, size: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_descent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_underline_position(&mut self, cache_index: i32, size: i32, underline_position: f32,) {
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, underline_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_underline_position(&self, cache_index: i32, size: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_underline_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_underline_thickness(&mut self, cache_index: i32, size: i32, underline_thickness: f32,) {
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, underline_thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_underline_thickness(&self, cache_index: i32, size: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_underline_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cache_scale(&mut self, cache_index: i32, size: i32, scale: f32,) {
            type CallSig = ((), i32, i32, f32);
            let args = (cache_index, size, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_scale(&self, cache_index: i32, size: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_count(&self, cache_index: i32, size: Vector2i,) -> i32 {
            type CallSig = (i32, i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_textures(&mut self, cache_index: i32, size: Vector2i,) {
            type CallSig = ((), i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "clear_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_texture(&mut self, cache_index: i32, size: Vector2i, texture_index: i32,) {
            type CallSig = ((), i32, Vector2i, i32);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "remove_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_image(&mut self, cache_index: i32, size: Vector2i, texture_index: i32, image: impl AsObjectArg < crate::classes::Image >,) {
            type CallSig = ((), i32, Vector2i, i32, ObjectArg < crate::classes::Image >);
            let args = (cache_index, size, texture_index, image.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_texture_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_image(&self, cache_index: i32, size: Vector2i, texture_index: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallSig = (Option < Gd < crate::classes::Image > >, i32, Vector2i, i32);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_offsets(&mut self, cache_index: i32, size: Vector2i, texture_index: i32, offset: &PackedInt32Array,) {
            type CallSig < 'a0, > = ((), i32, Vector2i, i32, RefArg < 'a0, PackedInt32Array >);
            let args = (cache_index, size, texture_index, RefArg::new(offset),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_texture_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_offsets(&self, cache_index: i32, size: Vector2i, texture_index: i32,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, i32, Vector2i, i32);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_offsets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_list(&self, cache_index: i32, size: Vector2i,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_glyphs(&mut self, cache_index: i32, size: Vector2i,) {
            type CallSig = ((), i32, Vector2i);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "clear_glyphs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_glyph(&mut self, cache_index: i32, size: Vector2i, glyph: i32,) {
            type CallSig = ((), i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "remove_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_advance(&mut self, cache_index: i32, size: i32, glyph: i32, advance: Vector2,) {
            type CallSig = ((), i32, i32, i32, Vector2);
            let args = (cache_index, size, glyph, advance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_advance(&self, cache_index: i32, size: i32, glyph: i32,) -> Vector2 {
            type CallSig = (Vector2, i32, i32, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_offset(&mut self, cache_index: i32, size: Vector2i, glyph: i32, offset: Vector2,) {
            type CallSig = ((), i32, Vector2i, i32, Vector2);
            let args = (cache_index, size, glyph, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_offset(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Vector2 {
            type CallSig = (Vector2, i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_size(&mut self, cache_index: i32, size: Vector2i, glyph: i32, gl_size: Vector2,) {
            type CallSig = ((), i32, Vector2i, i32, Vector2);
            let args = (cache_index, size, glyph, gl_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_size(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Vector2 {
            type CallSig = (Vector2, i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_uv_rect(&mut self, cache_index: i32, size: Vector2i, glyph: i32, uv_rect: Rect2,) {
            type CallSig = ((), i32, Vector2i, i32, Rect2);
            let args = (cache_index, size, glyph, uv_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_uv_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_uv_rect(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Rect2 {
            type CallSig = (Rect2, i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_uv_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_texture_idx(&mut self, cache_index: i32, size: Vector2i, glyph: i32, texture_idx: i32,) {
            type CallSig = ((), i32, Vector2i, i32, i32);
            let args = (cache_index, size, glyph, texture_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_texture_idx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_texture_idx(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> i32 {
            type CallSig = (i32, i32, Vector2i, i32);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_texture_idx", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_kerning_list(&self, cache_index: i32, size: i32,) -> Array < Vector2i > {
            type CallSig = (Array < Vector2i >, i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_kerning_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_kerning_map(&mut self, cache_index: i32, size: i32,) {
            type CallSig = ((), i32, i32);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "clear_kerning_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_kerning(&mut self, cache_index: i32, size: i32, glyph_pair: Vector2i,) {
            type CallSig = ((), i32, i32, Vector2i);
            let args = (cache_index, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "remove_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_kerning(&mut self, cache_index: i32, size: i32, glyph_pair: Vector2i, kerning: Vector2,) {
            type CallSig = ((), i32, i32, Vector2i, Vector2);
            let args = (cache_index, size, glyph_pair, kerning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_kerning(&self, cache_index: i32, size: i32, glyph_pair: Vector2i,) -> Vector2 {
            type CallSig = (Vector2, i32, i32, Vector2i);
            let args = (cache_index, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_kerning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn render_range(&mut self, cache_index: i32, size: Vector2i, start: i64, end: i64,) {
            type CallSig = ((), i32, Vector2i, i64, i64);
            let args = (cache_index, size, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "render_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn render_glyph(&mut self, cache_index: i32, size: Vector2i, index: i32,) {
            type CallSig = ((), i32, Vector2i, i32);
            let args = (cache_index, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "render_glyph", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language_support_override(&mut self, language: impl AsArg < GString >, supported: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (language.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language_support_override(&self, language: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_language_support_override(&mut self, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "remove_language_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language_support_overrides(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_language_support_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_script_support_override(&mut self, script: impl AsArg < GString >, supported: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (script.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_support_override(&self, script: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_script_support_override(&mut self, script: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "remove_script_support_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_support_overrides(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_script_support_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_opentype_feature_overrides(&mut self, overrides: &Dictionary,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(overrides),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "set_opentype_feature_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_opentype_feature_overrides(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_opentype_feature_overrides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_index(&self, size: i32, char: i64, variation_selector: i64,) -> i32 {
            type CallSig = (i32, i32, i64, i64);
            let args = (size, char, variation_selector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_char_from_glyph_index(&self, size: i32, glyph_index: i32,) -> i64 {
            type CallSig = (i64, i32, i32);
            let args = (size, glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FontFile", "get_char_from_glyph_index", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for FontFile {
        type Base = crate::classes::Font;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"FontFile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FontFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Font > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FontFile {
        
    }
    impl crate::obj::cap::GodotDefault for FontFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FontFile {
        type Target = crate::classes::Font;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FontFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FontFile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_FontFile {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FontFile > for $Class {
                
            }
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