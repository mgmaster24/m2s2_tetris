#![doc = "Sidecar module for class [`GridMap`][crate::classes::GridMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GridMap` enums](https://docs.godotengine.org/en/stable/classes/class_gridmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GridMap.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`grid_map`][crate::classes::grid_map]: sidecar module with related enum/flag types\n* [`IGridMap`][crate::classes::IGridMap]: virtual methods\n\n\nSee also [Godot docs for `GridMap`](https://docs.godotengine.org/en/stable/classes/class_gridmap.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GridMap::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GridMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GridMap`][crate::classes::GridMap].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GridMap` methods](https://docs.godotengine.org/en/stable/classes/class_gridmap.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGridMap: crate::obj::GodotClass < Base = GridMap > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GridMap {
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallSig = ((), u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type CallSig = ((), f32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_material(&mut self, material: impl AsObjectArg < crate::classes::PhysicsMaterial >,) {
            type CallSig = ((), ObjectArg < crate::classes::PhysicsMaterial >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_physics_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_material(&self,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallSig = (Option < Gd < crate::classes::PhysicsMaterial > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_physics_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_navigation(&mut self, bake_navigation: bool,) {
            type CallSig = ((), bool);
            let args = (bake_navigation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_bake_navigation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_baking_navigation(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "is_baking_navigation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_map(&mut self, navigation_map: Rid,) {
            type CallSig = ((), Rid);
            let args = (navigation_map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_map(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mesh_library(&mut self, mesh_library: impl AsObjectArg < crate::classes::MeshLibrary >,) {
            type CallSig = ((), ObjectArg < crate::classes::MeshLibrary >);
            let args = (mesh_library.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_mesh_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh_library(&self,) -> Option < Gd < crate::classes::MeshLibrary > > {
            type CallSig = (Option < Gd < crate::classes::MeshLibrary > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_mesh_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_size(&mut self, size: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_size(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_scale(&mut self, scale: f32,) {
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_octant_size(&mut self, size: i32,) {
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_octant_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_octant_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_octant_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_cell_item_full(&mut self, position: Vector3i, item: i32, orientation: i32,) {
            type CallSig = ((), Vector3i, i32, i32);
            let args = (position, item, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cell_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cell_item(&mut self, position: Vector3i, item: i32,) {
            self.set_cell_item_ex(position, item,) . done()
        }
        #[inline]
        pub fn set_cell_item_ex < 'a > (&'a mut self, position: Vector3i, item: i32,) -> ExSetCellItem < 'a > {
            ExSetCellItem::new(self, position, item,)
        }
        pub fn get_cell_item(&self, position: Vector3i,) -> i32 {
            type CallSig = (i32, Vector3i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_item_orientation(&self, position: Vector3i,) -> i32 {
            type CallSig = (i32, Vector3i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_item_basis(&self, position: Vector3i,) -> Basis {
            type CallSig = (Basis, Vector3i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_basis_with_orthogonal_index(&self, index: i32,) -> Basis {
            type CallSig = (Basis, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_basis_with_orthogonal_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_orthogonal_index_from_basis(&self, basis: Basis,) -> i32 {
            type CallSig = (i32, Basis);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_orthogonal_index_from_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn local_to_map(&self, local_position: Vector3,) -> Vector3i {
            type CallSig = (Vector3i, Vector3);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "local_to_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn map_to_local(&self, map_position: Vector3i,) -> Vector3 {
            type CallSig = (Vector3, Vector3i);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "map_to_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn resource_changed(&mut self, resource: impl AsObjectArg < crate::classes::Resource >,) {
            type CallSig = ((), ObjectArg < crate::classes::Resource >);
            let args = (resource.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "resource_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_x(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_center_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_x(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_center_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_y(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_center_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_y(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_center_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_z(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "set_center_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_z(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_center_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_used_cells(&self,) -> Array < Vector3i > {
            type CallSig = (Array < Vector3i >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_used_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_used_cells_by_item(&self, item: i32,) -> Array < Vector3i > {
            type CallSig = (Array < Vector3i >, i32);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_used_cells_by_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_meshes(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_meshes(&mut self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_bake_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_mesh_instance(&mut self, idx: i32,) -> Rid {
            type CallSig = (Rid, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "get_bake_mesh_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_baked_meshes(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "clear_baked_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn make_baked_meshes_full(&mut self, gen_lightmap_uv: bool, lightmap_uv_texel_size: f32,) {
            type CallSig = ((), bool, f32);
            let args = (gen_lightmap_uv, lightmap_uv_texel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GridMap", "make_baked_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::make_baked_meshes_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn make_baked_meshes(&mut self,) {
            self.make_baked_meshes_ex() . done()
        }
        #[inline]
        pub fn make_baked_meshes_ex < 'a > (&'a mut self,) -> ExMakeBakedMeshes < 'a > {
            ExMakeBakedMeshes::new(self,)
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
        pub const INVALID_CELL_ITEM: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for GridMap {
        type Base = crate::classes::Node3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GridMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GridMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GridMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GridMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GridMap {
        
    }
    impl crate::obj::cap::GodotDefault for GridMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GridMap {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GridMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GridMap`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GridMap {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GridMap > for $Class {
                
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
#[doc = "Default-param extender for [`GridMap::set_cell_item_ex`][super::GridMap::set_cell_item_ex]."]
#[must_use]
pub struct ExSetCellItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GridMap, position: Vector3i, item: i32, orientation: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellItem < 'a > {
    fn new(surround_object: &'a mut re_export::GridMap, position: Vector3i, item: i32,) -> Self {
        let orientation = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, item: item, orientation: orientation,
        }
    }
    #[inline]
    pub fn orientation(self, orientation: i32) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, item, orientation,
        }
        = self;
        re_export::GridMap::set_cell_item_full(surround_object, position, item, orientation,)
    }
}
#[doc = "Default-param extender for [`GridMap::make_baked_meshes_ex`][super::GridMap::make_baked_meshes_ex]."]
#[must_use]
pub struct ExMakeBakedMeshes < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GridMap, gen_lightmap_uv: bool, lightmap_uv_texel_size: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMakeBakedMeshes < 'a > {
    fn new(surround_object: &'a mut re_export::GridMap,) -> Self {
        let gen_lightmap_uv = false;
        let lightmap_uv_texel_size = 0.1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, gen_lightmap_uv: gen_lightmap_uv, lightmap_uv_texel_size: lightmap_uv_texel_size,
        }
    }
    #[inline]
    pub fn gen_lightmap_uv(self, gen_lightmap_uv: bool) -> Self {
        Self {
            gen_lightmap_uv: gen_lightmap_uv, .. self
        }
    }
    #[inline]
    pub fn lightmap_uv_texel_size(self, lightmap_uv_texel_size: f32) -> Self {
        Self {
            lightmap_uv_texel_size: lightmap_uv_texel_size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, gen_lightmap_uv, lightmap_uv_texel_size,
        }
        = self;
        re_export::GridMap::make_baked_meshes_full(surround_object, gen_lightmap_uv, lightmap_uv_texel_size,)
    }
}