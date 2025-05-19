#![doc = "Sidecar module for class [`StyleBoxFlat`][crate::classes::StyleBoxFlat].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StyleBoxFlat` enums](https://docs.godotengine.org/en/stable/classes/class_styleboxflat.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StyleBoxFlat.`\n\nInherits [`StyleBox`][crate::classes::StyleBox].\n\nRelated symbols:\n\n* [`IStyleBoxFlat`][crate::classes::IStyleBoxFlat]: virtual methods\n\n\nSee also [Godot docs for `StyleBoxFlat`](https://docs.godotengine.org/en/stable/classes/class_styleboxflat.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`StyleBoxFlat::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StyleBoxFlat {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`StyleBoxFlat`][crate::classes::StyleBoxFlat].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `StyleBoxFlat` methods](https://docs.godotengine.org/en/stable/classes/class_styleboxflat.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStyleBoxFlat: crate::obj::GodotClass < Base = StyleBoxFlat > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn draw(&self, to_canvas_item: Rid, rect: Rect2,);
        fn get_draw_rect(&self, rect: Rect2,) -> Rect2 {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn test_mask(&self, point: Vector2, rect: Rect2,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl StyleBoxFlat {
        pub fn set_bg_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bg_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_border_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_border_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_border_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_border_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_border_width_all(&mut self, width: i32,) {
            type CallSig = ((), i32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_border_width_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_border_width_min(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_border_width_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_border_width(&mut self, margin: crate::global::Side, width: i32,) {
            type CallSig = ((), crate::global::Side, i32);
            let args = (margin, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_border_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_border_width(&self, margin: crate::global::Side,) -> i32 {
            type CallSig = (i32, crate::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_border_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_border_blend(&mut self, blend: bool,) {
            type CallSig = ((), bool);
            let args = (blend,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_border_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_border_blend(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_border_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_corner_radius_all(&mut self, radius: i32,) {
            type CallSig = ((), i32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_corner_radius_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_corner_radius(&mut self, corner: crate::global::Corner, radius: i32,) {
            type CallSig = ((), crate::global::Corner, i32);
            let args = (corner, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_corner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_corner_radius(&self, corner: crate::global::Corner,) -> i32 {
            type CallSig = (i32, crate::global::Corner);
            let args = (corner,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_corner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_margin(&mut self, margin: crate::global::Side, size: f32,) {
            type CallSig = ((), crate::global::Side, f32);
            let args = (margin, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_expand_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_margin_all(&mut self, size: f32,) {
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_expand_margin_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_expand_margin(&self, margin: crate::global::Side,) -> f32 {
            type CallSig = (f32, crate::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_expand_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_center(&mut self, draw_center: bool,) {
            type CallSig = ((), bool);
            let args = (draw_center,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_draw_center", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_draw_center_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "is_draw_center_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skew(&mut self, skew: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (skew,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skew(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_shadow_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_shadow_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_size(&mut self, size: i32,) {
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_shadow_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_shadow_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_offset(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_shadow_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_offset(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_shadow_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_anti_aliased(&mut self, anti_aliased: bool,) {
            type CallSig = ((), bool);
            let args = (anti_aliased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_anti_aliased", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_anti_aliased(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "is_anti_aliased", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_aa_size(&mut self, size: f32,) {
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_aa_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_aa_size(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_aa_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_corner_detail(&mut self, detail: i32,) {
            type CallSig = ((), i32);
            let args = (detail,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_corner_detail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_corner_detail(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_corner_detail", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for StyleBoxFlat {
        type Base = crate::classes::StyleBox;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"StyleBoxFlat"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StyleBoxFlat {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::StyleBox > for StyleBoxFlat {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for StyleBoxFlat {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StyleBoxFlat {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StyleBoxFlat {
        
    }
    impl crate::obj::cap::GodotDefault for StyleBoxFlat {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StyleBoxFlat {
        type Target = crate::classes::StyleBox;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StyleBoxFlat {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`StyleBoxFlat`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_StyleBoxFlat {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::StyleBoxFlat > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::StyleBox > for $Class {
                
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