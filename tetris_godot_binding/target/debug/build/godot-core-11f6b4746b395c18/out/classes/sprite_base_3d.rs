#![doc = "Sidecar module for class [`SpriteBase3D`][crate::classes::SpriteBase3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SpriteBase3D` enums](https://docs.godotengine.org/en/stable/classes/class_spritebase3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SpriteBase3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`sprite_base_3d`][crate::classes::sprite_base_3d]: sidecar module with related enum/flag types\n* [`ISpriteBase3D`][crate::classes::ISpriteBase3D]: virtual methods\n\n\nSee also [Godot docs for `SpriteBase3D`](https://docs.godotengine.org/en/stable/classes/class_spritebase3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<SpriteBase3D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SpriteBase3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SpriteBase3D`][crate::classes::SpriteBase3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SpriteBase3D` methods](https://docs.godotengine.org/en/stable/classes/class_spritebase3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISpriteBase3D: crate::obj::GodotClass < Base = SpriteBase3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl SpriteBase3D {
        pub fn set_centered(&mut self, centered: bool,) {
            type CallSig = ((), bool);
            let args = (centered,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_centered(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "is_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_h(&mut self, flip_h: bool,) {
            type CallSig = ((), bool);
            let args = (flip_h,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_flip_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flipped_h(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "is_flipped_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_v(&mut self, flip_v: bool,) {
            type CallSig = ((), bool);
            let args = (flip_v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_flip_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flipped_v(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "is_flipped_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type CallSig = ((), Color);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_render_priority(&mut self, priority: i32,) {
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_priority(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_render_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pixel_size(&mut self, pixel_size: f32,) {
            type CallSig = ((), f32);
            let args = (pixel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_pixel_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pixel_size(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_pixel_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_axis(&mut self, axis: Vector3Axis,) {
            type CallSig = ((), Vector3Axis);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_axis(&self,) -> Vector3Axis {
            type CallSig = (Vector3Axis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_flag(&mut self, flag: crate::classes::sprite_base_3d::DrawFlags, enabled: bool,) {
            type CallSig = ((), crate::classes::sprite_base_3d::DrawFlags, bool);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_draw_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_flag(&self, flag: crate::classes::sprite_base_3d::DrawFlags,) -> bool {
            type CallSig = (bool, crate::classes::sprite_base_3d::DrawFlags);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_draw_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_cut_mode(&mut self, mode: crate::classes::sprite_base_3d::AlphaCutMode,) {
            type CallSig = ((), crate::classes::sprite_base_3d::AlphaCutMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_cut_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_cut_mode(&self,) -> crate::classes::sprite_base_3d::AlphaCutMode {
            type CallSig = (crate::classes::sprite_base_3d::AlphaCutMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_cut_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f32,) {
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_scissor_threshold(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f32,) {
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_hash_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: crate::classes::base_material_3d::AlphaAntiAliasing,) {
            type CallSig = ((), crate::classes::base_material_3d::AlphaAntiAliasing);
            let args = (alpha_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing(&self,) -> crate::classes::base_material_3d::AlphaAntiAliasing {
            type CallSig = (crate::classes::base_material_3d::AlphaAntiAliasing,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f32,) {
            type CallSig = ((), f32);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing_edge(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_billboard_mode(&mut self, mode: crate::classes::base_material_3d::BillboardMode,) {
            type CallSig = ((), crate::classes::base_material_3d::BillboardMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_billboard_mode(&self,) -> crate::classes::base_material_3d::BillboardMode {
            type CallSig = (crate::classes::base_material_3d::BillboardMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::classes::base_material_3d::TextureFilter,) {
            type CallSig = ((), crate::classes::base_material_3d::TextureFilter);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::base_material_3d::TextureFilter {
            type CallSig = (crate::classes::base_material_3d::TextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_rect(&self,) -> Rect2 {
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_item_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_triangle_mesh(&self,) -> Option < Gd < crate::classes::TriangleMesh > > {
            type CallSig = (Option < Gd < crate::classes::TriangleMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SpriteBase3D", "generate_triangle_mesh", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SpriteBase3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SpriteBase3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SpriteBase3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for SpriteBase3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for SpriteBase3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for SpriteBase3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SpriteBase3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SpriteBase3D {
        
    }
    impl std::ops::Deref for SpriteBase3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SpriteBase3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SpriteBase3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SpriteBase3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SpriteBase3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DrawFlags {
    ord: i32
}
impl DrawFlags {
    #[doc(alias = "FLAG_TRANSPARENT")]
    #[doc = "Godot enumerator name: `FLAG_TRANSPARENT`"]
    pub const TRANSPARENT: DrawFlags = DrawFlags {
        ord: 0i32
    };
    #[doc(alias = "FLAG_SHADED")]
    #[doc = "Godot enumerator name: `FLAG_SHADED`"]
    pub const SHADED: DrawFlags = DrawFlags {
        ord: 1i32
    };
    #[doc(alias = "FLAG_DOUBLE_SIDED")]
    #[doc = "Godot enumerator name: `FLAG_DOUBLE_SIDED`"]
    pub const DOUBLE_SIDED: DrawFlags = DrawFlags {
        ord: 2i32
    };
    #[doc(alias = "FLAG_DISABLE_DEPTH_TEST")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_DEPTH_TEST`"]
    pub const DISABLE_DEPTH_TEST: DrawFlags = DrawFlags {
        ord: 3i32
    };
    #[doc(alias = "FLAG_FIXED_SIZE")]
    #[doc = "Godot enumerator name: `FLAG_FIXED_SIZE`"]
    pub const FIXED_SIZE: DrawFlags = DrawFlags {
        ord: 4i32
    };
    #[doc(alias = "FLAG_MAX")]
    #[doc = "Godot enumerator name: `FLAG_MAX`"]
    pub const MAX: DrawFlags = DrawFlags {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for DrawFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawFlags {
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
            Self::TRANSPARENT => "TRANSPARENT", Self::SHADED => "SHADED", Self::DOUBLE_SIDED => "DOUBLE_SIDED", Self::DISABLE_DEPTH_TEST => "DISABLE_DEPTH_TEST", Self::FIXED_SIZE => "FIXED_SIZE", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TRANSPARENT => "FLAG_TRANSPARENT", Self::SHADED => "FLAG_SHADED", Self::DOUBLE_SIDED => "FLAG_DOUBLE_SIDED", Self::DISABLE_DEPTH_TEST => "FLAG_DISABLE_DEPTH_TEST", Self::FIXED_SIZE => "FLAG_FIXED_SIZE", Self::MAX => "FLAG_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for DrawFlags {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for DrawFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DrawFlags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlphaCutMode {
    ord: i32
}
impl AlphaCutMode {
    #[doc(alias = "ALPHA_CUT_DISABLED")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_DISABLED`"]
    pub const DISABLED: AlphaCutMode = AlphaCutMode {
        ord: 0i32
    };
    #[doc(alias = "ALPHA_CUT_DISCARD")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_DISCARD`"]
    pub const DISCARD: AlphaCutMode = AlphaCutMode {
        ord: 1i32
    };
    #[doc(alias = "ALPHA_CUT_OPAQUE_PREPASS")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_OPAQUE_PREPASS`"]
    pub const OPAQUE_PREPASS: AlphaCutMode = AlphaCutMode {
        ord: 2i32
    };
    #[doc(alias = "ALPHA_CUT_HASH")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_HASH`"]
    pub const HASH: AlphaCutMode = AlphaCutMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AlphaCutMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlphaCutMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlphaCutMode {
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
            Self::DISABLED => "DISABLED", Self::DISCARD => "DISCARD", Self::OPAQUE_PREPASS => "OPAQUE_PREPASS", Self::HASH => "HASH", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "ALPHA_CUT_DISABLED", Self::DISCARD => "ALPHA_CUT_DISCARD", Self::OPAQUE_PREPASS => "ALPHA_CUT_OPAQUE_PREPASS", Self::HASH => "ALPHA_CUT_HASH", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AlphaCutMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AlphaCutMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlphaCutMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}