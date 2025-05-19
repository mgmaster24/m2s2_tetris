#![doc = "Sidecar module for class [`ShapeCast3D`][crate::classes::ShapeCast3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ShapeCast3D` enums](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ShapeCast3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`IShapeCast3D`][crate::classes::IShapeCast3D]: virtual methods\n\n\nSee also [Godot docs for `ShapeCast3D`](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ShapeCast3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ShapeCast3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ShapeCast3D`][crate::classes::ShapeCast3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ShapeCast3D` methods](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IShapeCast3D: crate::obj::GodotClass < Base = ShapeCast3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ShapeCast3D {
        pub fn resource_changed(&mut self, resource: impl AsObjectArg < crate::classes::Resource >,) {
            type CallSig = ((), ObjectArg < crate::classes::Resource >);
            let args = (resource.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "resource_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape(&mut self, shape: impl AsObjectArg < crate::classes::Shape3D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Shape3D >);
            let args = (shape.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape(&self,) -> Option < Gd < crate::classes::Shape3D > > {
            type CallSig = (Option < Gd < crate::classes::Shape3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_position(&mut self, local_point: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_target_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_position(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_target_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_margin(&mut self, margin: f32,) {
            type CallSig = ((), f32);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_margin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_results(&mut self, max_results: i32,) {
            type CallSig = ((), i32);
            let args = (max_results,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_max_results", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_results(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_max_results", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_colliding(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "is_colliding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_shapecast_update(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "force_shapecast_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider(&self, index: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallSig = (Option < Gd < crate::classes::Object > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_rid(&self, index: i32,) -> Rid {
            type CallSig = (Rid, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collider_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_shape(&self, index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collider_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_point(&self, index: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_normal(&self, index: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_collision_safe_fraction(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_closest_collision_safe_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_collision_unsafe_fraction(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_closest_collision_unsafe_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_exception_rid(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "add_exception_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_exception(&mut self, node: impl AsObjectArg < crate::classes::CollisionObject3D >,) {
            type CallSig = ((), ObjectArg < crate::classes::CollisionObject3D >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "add_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_exception_rid(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "remove_exception_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_exception(&mut self, node: impl AsObjectArg < crate::classes::CollisionObject3D >,) {
            type CallSig = ((), ObjectArg < crate::classes::CollisionObject3D >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "remove_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_exceptions(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "clear_exceptions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exclude_parent_body(&mut self, mask: bool,) {
            type CallSig = ((), bool);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_exclude_parent_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_exclude_parent_body(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_exclude_parent_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_areas(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_collide_with_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_areas_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "is_collide_with_areas_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_bodies(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_collide_with_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_bodies_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "is_collide_with_bodies_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_debug_shape_custom_color(&mut self, debug_shape_custom_color: Color,) {
            type CallSig = ((), Color);
            let args = (debug_shape_custom_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_debug_shape_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_debug_shape_custom_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_debug_shape_custom_color", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ShapeCast3D {
        type Base = crate::classes::Node3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ShapeCast3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ShapeCast3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for ShapeCast3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ShapeCast3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ShapeCast3D {
        
    }
    impl crate::obj::cap::GodotDefault for ShapeCast3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ShapeCast3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ShapeCast3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ShapeCast3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ShapeCast3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ShapeCast3D > for $Class {
                
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