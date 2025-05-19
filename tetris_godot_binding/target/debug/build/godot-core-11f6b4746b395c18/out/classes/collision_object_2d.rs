#![doc = "Sidecar module for class [`CollisionObject2D`][crate::classes::CollisionObject2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CollisionObject2D` enums](https://docs.godotengine.org/en/stable/classes/class_collisionobject2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CollisionObject2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`collision_object_2d`][crate::classes::collision_object_2d]: sidecar module with related enum/flag types\n* [`ICollisionObject2D`][crate::classes::ICollisionObject2D]: virtual methods\n\n\nSee also [Godot docs for `CollisionObject2D`](https://docs.godotengine.org/en/stable/classes/class_collisionobject2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<CollisionObject2D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CollisionObject2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CollisionObject2D`][crate::classes::CollisionObject2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CollisionObject2D` methods](https://docs.godotengine.org/en/stable/classes/class_collisionobject2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICollisionObject2D: crate::obj::GodotClass < Base = CollisionObject2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
        fn input_event(&mut self, viewport: Gd < crate::classes::Viewport >, event: Gd < crate::classes::InputEvent >, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
            unimplemented !()
        }
        fn mouse_shape_enter(&mut self, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_shape_exit(&mut self, shape_idx: i32,) {
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
    impl CollisionObject2D {
        pub fn get_rid(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallSig = ((), u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "set_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type CallSig = ((), f32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_mode(&mut self, mode: crate::classes::collision_object_2d::DisableMode,) {
            type CallSig = ((), crate::classes::collision_object_2d::DisableMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "set_disable_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_disable_mode(&self,) -> crate::classes::collision_object_2d::DisableMode {
            type CallSig = (crate::classes::collision_object_2d::DisableMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_disable_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pickable(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "set_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pickable(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "is_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_shape_owner(&mut self, owner: impl AsObjectArg < crate::classes::Object >,) -> u32 {
            type CallSig = (u32, ObjectArg < crate::classes::Object >);
            let args = (owner.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "create_shape_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_shape_owner(&mut self, owner_id: u32,) {
            type CallSig = ((), u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "remove_shape_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape_owners(&mut self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_shape_owners", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_set_transform(&mut self, owner_id: u32, transform: Transform2D,) {
            type CallSig = ((), u32, Transform2D);
            let args = (owner_id, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_transform(&self, owner_id: u32,) -> Transform2D {
            type CallSig = (Transform2D, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_owner(&self, owner_id: u32,) -> Option < Gd < crate::classes::Object > > {
            type CallSig = (Option < Gd < crate::classes::Object > >, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_get_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_set_disabled(&mut self, owner_id: u32, disabled: bool,) {
            type CallSig = ((), u32, bool);
            let args = (owner_id, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_set_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shape_owner_disabled(&self, owner_id: u32,) -> bool {
            type CallSig = (bool, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "is_shape_owner_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_set_one_way_collision(&mut self, owner_id: u32, enable: bool,) {
            type CallSig = ((), u32, bool);
            let args = (owner_id, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_set_one_way_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shape_owner_one_way_collision_enabled(&self, owner_id: u32,) -> bool {
            type CallSig = (bool, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "is_shape_owner_one_way_collision_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_set_one_way_collision_margin(&mut self, owner_id: u32, margin: f32,) {
            type CallSig = ((), u32, f32);
            let args = (owner_id, margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_set_one_way_collision_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape_owner_one_way_collision_margin(&self, owner_id: u32,) -> f32 {
            type CallSig = (f32, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "get_shape_owner_one_way_collision_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_add_shape(&mut self, owner_id: u32, shape: impl AsObjectArg < crate::classes::Shape2D >,) {
            type CallSig = ((), u32, ObjectArg < crate::classes::Shape2D >);
            let args = (owner_id, shape.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_add_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_shape_count(&self, owner_id: u32,) -> i32 {
            type CallSig = (i32, u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_shape(&self, owner_id: u32, shape_id: i32,) -> Option < Gd < crate::classes::Shape2D > > {
            type CallSig = (Option < Gd < crate::classes::Shape2D > >, u32, i32);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_get_shape_index(&self, owner_id: u32, shape_id: i32,) -> i32 {
            type CallSig = (i32, u32, i32);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_get_shape_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_remove_shape(&mut self, owner_id: u32, shape_id: i32,) {
            type CallSig = ((), u32, i32);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_owner_clear_shapes(&mut self, owner_id: u32,) {
            type CallSig = ((), u32);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_owner_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_find_owner(&self, shape_index: i32,) -> u32 {
            type CallSig = (u32, i32);
            let args = (shape_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CollisionObject2D", "shape_find_owner", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CollisionObject2D {
        type Base = crate::classes::Node2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"CollisionObject2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CollisionObject2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for CollisionObject2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for CollisionObject2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CollisionObject2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CollisionObject2D {
        
    }
    impl std::ops::Deref for CollisionObject2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CollisionObject2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CollisionObject2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_CollisionObject2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CollisionObject2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisableMode {
    ord: i32
}
impl DisableMode {
    #[doc(alias = "DISABLE_MODE_REMOVE")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_REMOVE`"]
    pub const REMOVE: DisableMode = DisableMode {
        ord: 0i32
    };
    #[doc(alias = "DISABLE_MODE_MAKE_STATIC")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_MAKE_STATIC`"]
    pub const MAKE_STATIC: DisableMode = DisableMode {
        ord: 1i32
    };
    #[doc(alias = "DISABLE_MODE_KEEP_ACTIVE")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_KEEP_ACTIVE`"]
    pub const KEEP_ACTIVE: DisableMode = DisableMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DisableMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DisableMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DisableMode {
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
            Self::REMOVE => "REMOVE", Self::MAKE_STATIC => "MAKE_STATIC", Self::KEEP_ACTIVE => "KEEP_ACTIVE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::REMOVE => "DISABLE_MODE_REMOVE", Self::MAKE_STATIC => "DISABLE_MODE_MAKE_STATIC", Self::KEEP_ACTIVE => "DISABLE_MODE_KEEP_ACTIVE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DisableMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DisableMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DisableMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}