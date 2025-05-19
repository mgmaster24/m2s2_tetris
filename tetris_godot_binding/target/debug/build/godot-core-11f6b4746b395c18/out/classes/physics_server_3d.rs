#![doc = "Sidecar module for class [`PhysicsServer3D`][crate::classes::PhysicsServer3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer3D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_server_3d`][crate::classes::physics_server_3d]: sidecar module with related enum/flag types\n* [`IPhysicsServer3D`][crate::classes::IPhysicsServer3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer3D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`PhysicsServer3D::singleton()`][PhysicsServer3D::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsServer3D`][crate::classes::PhysicsServer3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsServer3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsServer3D: crate::obj::GodotClass < Base = PhysicsServer3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsServer3D {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"PhysicsServer3D");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn world_boundary_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "world_boundary_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn separation_ray_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "separation_ray_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sphere_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "sphere_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn box_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "box_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capsule_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "capsule_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cylinder_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "cylinder_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convex_polygon_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "convex_polygon_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn concave_polygon_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "concave_polygon_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn heightmap_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "heightmap_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn custom_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "custom_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_set_data(&mut self, shape: Rid, data: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Variant >);
            let args = (shape, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_set_margin(&mut self, shape: Rid, margin: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (shape, margin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_set_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_type(&self, shape: Rid,) -> crate::classes::physics_server_3d::ShapeType {
            type CallSig = (crate::classes::physics_server_3d::ShapeType, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_data(&self, shape: Rid,) -> Variant {
            type CallSig = (Variant, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_margin(&self, shape: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_get_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_set_active(&mut self, space: Rid, active: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (space, active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_is_active(&self, space: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_set_param(&mut self, space: Rid, param: crate::classes::physics_server_3d::SpaceParameter, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_3d::SpaceParameter, f32);
            let args = (space, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_get_param(&self, space: Rid, param: crate::classes::physics_server_3d::SpaceParameter,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_3d::SpaceParameter);
            let args = (space, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState3D > > {
            type CallSig = (Option < Gd < crate::classes::PhysicsDirectSpaceState3D > >, Rid);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_get_direct_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_space(&mut self, area: Rid, space: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (area, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_space(&self, area: Rid,) -> Rid {
            type CallSig = (Rid, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn area_add_shape_full(&mut self, area: Rid, shape: Rid, transform: Transform3D, disabled: bool,) {
            type CallSig = ((), Rid, Rid, Transform3D, bool);
            let args = (area, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_add_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::area_add_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn area_add_shape(&mut self, area: Rid, shape: Rid,) {
            self.area_add_shape_ex(area, shape,) . done()
        }
        #[inline]
        pub fn area_add_shape_ex < 'a > (&'a mut self, area: Rid, shape: Rid,) -> ExAreaAddShape < 'a > {
            ExAreaAddShape::new(self, area, shape,)
        }
        pub fn area_set_shape(&mut self, area: Rid, shape_idx: i32, shape: Rid,) {
            type CallSig = ((), Rid, i32, Rid);
            let args = (area, shape_idx, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform3D,) {
            type CallSig = ((), Rid, i32, Transform3D);
            let args = (area, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (area, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_shape_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape_count(&self, area: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid {
            type CallSig = (Rid, Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_clear_shapes(&mut self, area: Rid,) {
            type CallSig = ((), Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_collision_layer(&mut self, area: Rid, layer: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (area, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_collision_layer(&self, area: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_collision_mask(&mut self, area: Rid, mask: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (area, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_collision_mask(&self, area: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_param(&mut self, area: Rid, param: crate::classes::physics_server_3d::AreaParameter, value: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, crate::classes::physics_server_3d::AreaParameter, RefArg < 'a0, Variant >);
            let args = (area, param, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_transform(&mut self, area: Rid, transform: Transform3D,) {
            type CallSig = ((), Rid, Transform3D);
            let args = (area, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_param(&self, area: Rid, param: crate::classes::physics_server_3d::AreaParameter,) -> Variant {
            type CallSig = (Variant, Rid, crate::classes::physics_server_3d::AreaParameter);
            let args = (area, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_transform(&self, area: Rid,) -> Transform3D {
            type CallSig = (Transform3D, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,) {
            type CallSig = ((), Rid, u64);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_object_instance_id(&self, area: Rid,) -> u64 {
            type CallSig = (u64, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_monitor_callback(&mut self, area: Rid, callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Callable >);
            let args = (area, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_monitor_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_area_monitor_callback(&mut self, area: Rid, callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Callable >);
            let args = (area, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_area_monitor_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (area, monitorable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_monitorable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_ray_pickable(&mut self, area: Rid, enable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (area, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_ray_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_space(&mut self, body: Rid, space: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_space(&self, body: Rid,) -> Rid {
            type CallSig = (Rid, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_mode(&mut self, body: Rid, mode: crate::classes::physics_server_3d::BodyMode,) {
            type CallSig = ((), Rid, crate::classes::physics_server_3d::BodyMode);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_mode(&self, body: Rid,) -> crate::classes::physics_server_3d::BodyMode {
            type CallSig = (crate::classes::physics_server_3d::BodyMode, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (body, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_layer(&self, body: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (body, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_mask(&self, body: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_priority(&mut self, body: Rid, priority: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_priority(&self, body: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_add_shape_full(&mut self, body: Rid, shape: Rid, transform: Transform3D, disabled: bool,) {
            type CallSig = ((), Rid, Rid, Transform3D, bool);
            let args = (body, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_add_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_add_shape(&mut self, body: Rid, shape: Rid,) {
            self.body_add_shape_ex(body, shape,) . done()
        }
        #[inline]
        pub fn body_add_shape_ex < 'a > (&'a mut self, body: Rid, shape: Rid,) -> ExBodyAddShape < 'a > {
            ExBodyAddShape::new(self, body, shape,)
        }
        pub fn body_set_shape(&mut self, body: Rid, shape_idx: i32, shape: Rid,) {
            type CallSig = ((), Rid, i32, Rid);
            let args = (body, shape_idx, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform3D,) {
            type CallSig = ((), Rid, i32, Transform3D);
            let args = (body, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (body, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_shape_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape_count(&self, body: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid {
            type CallSig = (Rid, Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_clear_shapes(&mut self, body: Rid,) {
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,) {
            type CallSig = ((), Rid, u64);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_object_instance_id(&self, body: Rid,) -> u64 {
            type CallSig = (u64, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_enable_continuous_collision_detection(&mut self, body: Rid, enable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_enable_continuous_collision_detection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_is_continuous_collision_detection_enabled(&self, body: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_is_continuous_collision_detection_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_param(&mut self, body: Rid, param: crate::classes::physics_server_3d::BodyParameter, value: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, crate::classes::physics_server_3d::BodyParameter, RefArg < 'a0, Variant >);
            let args = (body, param, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_param(&self, body: Rid, param: crate::classes::physics_server_3d::BodyParameter,) -> Variant {
            type CallSig = (Variant, Rid, crate::classes::physics_server_3d::BodyParameter);
            let args = (body, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_reset_mass_properties(&mut self, body: Rid,) {
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_reset_mass_properties", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_state(&mut self, body: Rid, state: crate::classes::physics_server_3d::BodyState, value: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, crate::classes::physics_server_3d::BodyState, RefArg < 'a0, Variant >);
            let args = (body, state, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_state(&self, body: Rid, state: crate::classes::physics_server_3d::BodyState,) -> Variant {
            type CallSig = (Variant, Rid, crate::classes::physics_server_3d::BodyState);
            let args = (body, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_apply_impulse_full(&mut self, body: Rid, impulse: Vector3, position: Vector3,) {
            type CallSig = ((), Rid, Vector3, Vector3);
            let args = (body, impulse, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_apply_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_apply_impulse(&mut self, body: Rid, impulse: Vector3,) {
            self.body_apply_impulse_ex(body, impulse,) . done()
        }
        #[inline]
        pub fn body_apply_impulse_ex < 'a > (&'a mut self, body: Rid, impulse: Vector3,) -> ExBodyApplyImpulse < 'a > {
            ExBodyApplyImpulse::new(self, body, impulse,)
        }
        pub fn body_apply_torque_impulse(&mut self, body: Rid, impulse: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_torque_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_apply_central_force(&mut self, body: Rid, force: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_apply_force_full(&mut self, body: Rid, force: Vector3, position: Vector3,) {
            type CallSig = ((), Rid, Vector3, Vector3);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_apply_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_apply_force(&mut self, body: Rid, force: Vector3,) {
            self.body_apply_force_ex(body, force,) . done()
        }
        #[inline]
        pub fn body_apply_force_ex < 'a > (&'a mut self, body: Rid, force: Vector3,) -> ExBodyApplyForce < 'a > {
            ExBodyApplyForce::new(self, body, force,)
        }
        pub fn body_apply_torque(&mut self, body: Rid, torque: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_add_constant_central_force(&mut self, body: Rid, force: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_constant_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_add_constant_force_full(&mut self, body: Rid, force: Vector3, position: Vector3,) {
            type CallSig = ((), Rid, Vector3, Vector3);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_add_constant_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_add_constant_force(&mut self, body: Rid, force: Vector3,) {
            self.body_add_constant_force_ex(body, force,) . done()
        }
        #[inline]
        pub fn body_add_constant_force_ex < 'a > (&'a mut self, body: Rid, force: Vector3,) -> ExBodyAddConstantForce < 'a > {
            ExBodyAddConstantForce::new(self, body, force,)
        }
        pub fn body_add_constant_torque(&mut self, body: Rid, torque: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_constant_force(&mut self, body: Rid, force: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_constant_force(&self, body: Rid,) -> Vector3 {
            type CallSig = (Vector3, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_constant_torque(&mut self, body: Rid, torque: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_constant_torque(&self, body: Rid,) -> Vector3 {
            type CallSig = (Vector3, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (body, axis_velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_axis_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_axis_lock(&mut self, body: Rid, axis: crate::classes::physics_server_3d::BodyAxis, lock: bool,) {
            type CallSig = ((), Rid, crate::classes::physics_server_3d::BodyAxis, bool);
            let args = (body, axis, lock,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_axis_lock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_is_axis_locked(&self, body: Rid, axis: crate::classes::physics_server_3d::BodyAxis,) -> bool {
            type CallSig = (bool, Rid, crate::classes::physics_server_3d::BodyAxis);
            let args = (body, axis,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_is_axis_locked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_remove_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (body, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_max_contacts_reported(&self, body: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_omit_force_integration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_is_omitting_force_integration(&self, body: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_is_omitting_force_integration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_state_sync_callback(&mut self, body: Rid, callable: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Callable >);
            let args = (body, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_state_sync_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_set_force_integration_callback_full(&mut self, body: Rid, callable: RefArg < Callable >, userdata: RefArg < Variant >,) {
            type CallSig < 'a0, 'a1, > = ((), Rid, RefArg < 'a0, Callable >, RefArg < 'a1, Variant >);
            let args = (body, callable, userdata,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_force_integration_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_set_force_integration_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_set_force_integration_callback(&mut self, body: Rid, callable: &Callable,) {
            self.body_set_force_integration_callback_ex(body, callable,) . done()
        }
        #[inline]
        pub fn body_set_force_integration_callback_ex < 'a > (&'a mut self, body: Rid, callable: &'a Callable,) -> ExBodySetForceIntegrationCallback < 'a > {
            ExBodySetForceIntegrationCallback::new(self, body, callable,)
        }
        pub fn body_set_ray_pickable(&mut self, body: Rid, enable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_ray_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_test_motion_full(&mut self, body: Rid, parameters: ObjectArg < crate::classes::PhysicsTestMotionParameters3D >, result: ObjectArg < crate::classes::PhysicsTestMotionResult3D >,) -> bool {
            type CallSig = (bool, Rid, ObjectArg < crate::classes::PhysicsTestMotionParameters3D >, ObjectArg < crate::classes::PhysicsTestMotionResult3D >);
            let args = (body, parameters, result,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_test_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_test_motion_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_test_motion(&mut self, body: Rid, parameters: impl AsObjectArg < crate::classes::PhysicsTestMotionParameters3D >,) -> bool {
            self.body_test_motion_ex(body, parameters,) . done()
        }
        #[inline]
        pub fn body_test_motion_ex < 'a > (&'a mut self, body: Rid, parameters: impl AsObjectArg < crate::classes::PhysicsTestMotionParameters3D >,) -> ExBodyTestMotion < 'a > {
            ExBodyTestMotion::new(self, body, parameters,)
        }
        pub fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::classes::PhysicsDirectBodyState3D > > {
            type CallSig = (Option < Gd < crate::classes::PhysicsDirectBodyState3D > >, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_direct_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_update_rendering_server(&mut self, body: Rid, rendering_server_handler: impl AsObjectArg < crate::classes::PhysicsServer3DRenderingServerHandler >,) {
            type CallSig = ((), Rid, ObjectArg < crate::classes::PhysicsServer3DRenderingServerHandler >);
            let args = (body, rendering_server_handler.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_update_rendering_server", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_space(&mut self, body: Rid, space: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_space(&self, body: Rid,) -> Rid {
            type CallSig = (Rid, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_mesh(&mut self, body: Rid, mesh: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_bounds(&self, body: Rid,) -> Aabb {
            type CallSig = (Aabb, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_bounds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (body, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_collision_layer(&self, body: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (body, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_collision_mask(&self, body: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_add_collision_exception(&mut self, body: Rid, body_b: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_add_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_remove_collision_exception(&mut self, body: Rid, body_b: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_remove_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_state(&mut self, body: Rid, state: crate::classes::physics_server_3d::BodyState, variant: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, crate::classes::physics_server_3d::BodyState, RefArg < 'a0, Variant >);
            let args = (body, state, RefArg::new(variant),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_state(&self, body: Rid, state: crate::classes::physics_server_3d::BodyState,) -> Variant {
            type CallSig = (Variant, Rid, crate::classes::physics_server_3d::BodyState);
            let args = (body, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_transform(&mut self, body: Rid, transform: Transform3D,) {
            type CallSig = ((), Rid, Transform3D);
            let args = (body, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_ray_pickable(&mut self, body: Rid, enable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_ray_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_simulation_precision(&mut self, body: Rid, simulation_precision: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (body, simulation_precision,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_simulation_precision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_simulation_precision(&self, body: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_simulation_precision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_total_mass(&mut self, body: Rid, total_mass: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, total_mass,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_total_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_total_mass(&self, body: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_total_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_linear_stiffness(&mut self, body: Rid, stiffness: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, stiffness,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_linear_stiffness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_linear_stiffness(&self, body: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_linear_stiffness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_pressure_coefficient(&mut self, body: Rid, pressure_coefficient: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, pressure_coefficient,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_pressure_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_pressure_coefficient(&self, body: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_pressure_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_damping_coefficient(&mut self, body: Rid, damping_coefficient: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, damping_coefficient,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_damping_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_damping_coefficient(&self, body: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_damping_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_set_drag_coefficient(&mut self, body: Rid, drag_coefficient: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, drag_coefficient,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_drag_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_drag_coefficient(&self, body: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_drag_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_move_point(&mut self, body: Rid, point_index: i32, global_position: Vector3,) {
            type CallSig = ((), Rid, i32, Vector3);
            let args = (body, point_index, global_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_move_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_get_point_global_position(&self, body: Rid, point_index: i32,) -> Vector3 {
            type CallSig = (Vector3, Rid, i32);
            let args = (body, point_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_point_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_remove_all_pinned_points(&mut self, body: Rid,) {
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_remove_all_pinned_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_pin_point(&mut self, body: Rid, point_index: i32, pin: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (body, point_index, pin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_pin_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn soft_body_is_point_pinned(&self, body: Rid, point_index: i32,) -> bool {
            type CallSig = (bool, Rid, i32);
            let args = (body, point_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_is_point_pinned", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_clear(&mut self, joint: Rid,) {
            type CallSig = ((), Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_pin(&mut self, joint: Rid, body_A: Rid, local_A: Vector3, body_B: Rid, local_B: Vector3,) {
            type CallSig = ((), Rid, Rid, Vector3, Rid, Vector3);
            let args = (joint, body_A, local_A, body_B, local_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_pin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_3d::PinJointParam, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_3d::PinJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_3d::PinJointParam,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_3d::PinJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_set_local_a(&mut self, joint: Rid, local_A: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (joint, local_A,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_set_local_a", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_local_a(&self, joint: Rid,) -> Vector3 {
            type CallSig = (Vector3, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_get_local_a", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_set_local_b(&mut self, joint: Rid, local_B: Vector3,) {
            type CallSig = ((), Rid, Vector3);
            let args = (joint, local_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_set_local_b", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_local_b(&self, joint: Rid,) -> Vector3 {
            type CallSig = (Vector3, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_get_local_b", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_hinge(&mut self, joint: Rid, body_A: Rid, hinge_A: Transform3D, body_B: Rid, hinge_B: Transform3D,) {
            type CallSig = ((), Rid, Rid, Transform3D, Rid, Transform3D);
            let args = (joint, body_A, hinge_A, body_B, hinge_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_hinge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hinge_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_3d::HingeJointParam, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_3d::HingeJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "hinge_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hinge_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_3d::HingeJointParam,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_3d::HingeJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "hinge_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hinge_joint_set_flag(&mut self, joint: Rid, flag: crate::classes::physics_server_3d::HingeJointFlag, enabled: bool,) {
            type CallSig = ((), Rid, crate::classes::physics_server_3d::HingeJointFlag, bool);
            let args = (joint, flag, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "hinge_joint_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hinge_joint_get_flag(&self, joint: Rid, flag: crate::classes::physics_server_3d::HingeJointFlag,) -> bool {
            type CallSig = (bool, Rid, crate::classes::physics_server_3d::HingeJointFlag);
            let args = (joint, flag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "hinge_joint_get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_slider(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type CallSig = ((), Rid, Rid, Transform3D, Rid, Transform3D);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_slider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn slider_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_3d::SliderJointParam, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_3d::SliderJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "slider_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn slider_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_3d::SliderJointParam,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_3d::SliderJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "slider_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_cone_twist(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type CallSig = ((), Rid, Rid, Transform3D, Rid, Transform3D);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_cone_twist", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cone_twist_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_3d::ConeTwistJointParam, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_3d::ConeTwistJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "cone_twist_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cone_twist_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_3d::ConeTwistJointParam,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_3d::ConeTwistJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "cone_twist_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_get_type(&self, joint: Rid,) -> crate::classes::physics_server_3d::JointType {
            type CallSig = (crate::classes::physics_server_3d::JointType, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_set_solver_priority(&mut self, joint: Rid, priority: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (joint, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_set_solver_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_get_solver_priority(&self, joint: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_get_solver_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (joint, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_disable_collisions_between_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_is_disabled_collisions_between_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_make_generic_6dof(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type CallSig = ((), Rid, Rid, Transform3D, Rid, Transform3D);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_generic_6dof", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generic_6dof_joint_set_param(&mut self, joint: Rid, axis: Vector3Axis, param: crate::classes::physics_server_3d::G6dofJointAxisParam, value: f32,) {
            type CallSig = ((), Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisParam, f32);
            let args = (joint, axis, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "generic_6dof_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generic_6dof_joint_get_param(&self, joint: Rid, axis: Vector3Axis, param: crate::classes::physics_server_3d::G6dofJointAxisParam,) -> f32 {
            type CallSig = (f32, Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisParam);
            let args = (joint, axis, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "generic_6dof_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generic_6dof_joint_set_flag(&mut self, joint: Rid, axis: Vector3Axis, flag: crate::classes::physics_server_3d::G6dofJointAxisFlag, enable: bool,) {
            type CallSig = ((), Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisFlag, bool);
            let args = (joint, axis, flag, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "generic_6dof_joint_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generic_6dof_joint_get_flag(&self, joint: Rid, axis: Vector3Axis, flag: crate::classes::physics_server_3d::G6dofJointAxisFlag,) -> bool {
            type CallSig = (bool, Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisFlag);
            let args = (joint, axis, flag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "generic_6dof_joint_get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_info(&mut self, process_info: crate::classes::physics_server_3d::ProcessInfo,) -> i32 {
            type CallSig = (i32, crate::classes::physics_server_3d::ProcessInfo);
            let args = (process_info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "get_process_info", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsServer3D {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicsServer3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsServer3D {
        
    }
    impl std::ops::Deref for PhysicsServer3D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsServer3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicsServer3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsServer3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::area_add_shape_ex`][super::PhysicsServer3D::area_add_shape_ex]."]
#[must_use]
pub struct ExAreaAddShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer3D, area: Rid, shape: Rid, transform: Transform3D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAreaAddShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, area: Rid, shape: Rid,) -> Self {
        let transform = Transform3D::__internal_codegen(1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _);
        let disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, area: area, shape: shape, transform: transform, disabled: disabled,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform3D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, area, shape, transform, disabled,
        }
        = self;
        re_export::PhysicsServer3D::area_add_shape_full(surround_object, area, shape, transform, disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_add_shape_ex`][super::PhysicsServer3D::body_add_shape_ex]."]
#[must_use]
pub struct ExBodyAddShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, shape: Rid, transform: Transform3D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyAddShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, shape: Rid,) -> Self {
        let transform = Transform3D::__internal_codegen(1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _);
        let disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, shape: shape, transform: transform, disabled: disabled,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform3D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, shape, transform, disabled,
        }
        = self;
        re_export::PhysicsServer3D::body_add_shape_full(surround_object, body, shape, transform, disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_apply_impulse_ex`][super::PhysicsServer3D::body_apply_impulse_ex]."]
#[must_use]
pub struct ExBodyApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, impulse: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, impulse, position,
        }
        = self;
        re_export::PhysicsServer3D::body_apply_impulse_full(surround_object, body, impulse, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_apply_force_ex`][super::PhysicsServer3D::body_apply_force_ex]."]
#[must_use]
pub struct ExBodyApplyForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, force: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, force, position,
        }
        = self;
        re_export::PhysicsServer3D::body_apply_force_full(surround_object, body, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_add_constant_force_ex`][super::PhysicsServer3D::body_add_constant_force_ex]."]
#[must_use]
pub struct ExBodyAddConstantForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, force: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, force, position,
        }
        = self;
        re_export::PhysicsServer3D::body_add_constant_force_full(surround_object, body, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_set_force_integration_callback_ex`][super::PhysicsServer3D::body_set_force_integration_callback_ex]."]
#[must_use]
pub struct ExBodySetForceIntegrationCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, callable: CowArg < 'a, Callable >, userdata: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodySetForceIntegrationCallback < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, callable: &'a Callable,) -> Self {
        let userdata = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, callable: CowArg::Borrowed(callable), userdata: CowArg::Owned(userdata),
        }
    }
    #[inline]
    pub fn userdata(self, userdata: &'a Variant) -> Self {
        Self {
            userdata: CowArg::Borrowed(userdata), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, callable, userdata,
        }
        = self;
        re_export::PhysicsServer3D::body_set_force_integration_callback_full(surround_object, body, callable.cow_as_arg(), userdata.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_test_motion_ex`][super::PhysicsServer3D::body_test_motion_ex]."]
#[must_use]
pub struct ExBodyTestMotion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, parameters: ObjectCow < crate::classes::PhysicsTestMotionParameters3D >, result: ObjectCow < crate::classes::PhysicsTestMotionResult3D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyTestMotion < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer3D, body: Rid, parameters: impl AsObjectArg < crate::classes::PhysicsTestMotionParameters3D >,) -> Self {
        let result = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, parameters: parameters.consume_arg(), result: result.consume_arg(),
        }
    }
    #[inline]
    pub fn result(self, result: impl AsObjectArg < crate::classes::PhysicsTestMotionResult3D >) -> Self {
        Self {
            result: result.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, body, parameters, result,
        }
        = self;
        re_export::PhysicsServer3D::body_test_motion_full(surround_object, body, parameters.cow_as_object_arg(), result.cow_as_object_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct JointType {
    ord: i32
}
impl JointType {
    #[doc(alias = "JOINT_TYPE_PIN")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_PIN`"]
    pub const PIN: JointType = JointType {
        ord: 0i32
    };
    #[doc(alias = "JOINT_TYPE_HINGE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_HINGE`"]
    pub const HINGE: JointType = JointType {
        ord: 1i32
    };
    #[doc(alias = "JOINT_TYPE_SLIDER")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_SLIDER`"]
    pub const SLIDER: JointType = JointType {
        ord: 2i32
    };
    #[doc(alias = "JOINT_TYPE_CONE_TWIST")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_CONE_TWIST`"]
    pub const CONE_TWIST: JointType = JointType {
        ord: 3i32
    };
    #[doc(alias = "JOINT_TYPE_6DOF")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_6DOF`"]
    pub const TYPE_6DOF: JointType = JointType {
        ord: 4i32
    };
    #[doc(alias = "JOINT_TYPE_MAX")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_MAX`"]
    pub const MAX: JointType = JointType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for JointType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("JointType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for JointType {
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
            Self::PIN => "PIN", Self::HINGE => "HINGE", Self::SLIDER => "SLIDER", Self::CONE_TWIST => "CONE_TWIST", Self::TYPE_6DOF => "TYPE_6DOF", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PIN => "JOINT_TYPE_PIN", Self::HINGE => "JOINT_TYPE_HINGE", Self::SLIDER => "JOINT_TYPE_SLIDER", Self::CONE_TWIST => "JOINT_TYPE_CONE_TWIST", Self::TYPE_6DOF => "JOINT_TYPE_6DOF", Self::MAX => "JOINT_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for JointType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for JointType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for JointType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JointType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PinJointParam {
    ord: i32
}
impl PinJointParam {
    #[doc(alias = "PIN_JOINT_BIAS")]
    #[doc = "Godot enumerator name: `PIN_JOINT_BIAS`"]
    pub const BIAS: PinJointParam = PinJointParam {
        ord: 0i32
    };
    #[doc(alias = "PIN_JOINT_DAMPING")]
    #[doc = "Godot enumerator name: `PIN_JOINT_DAMPING`"]
    pub const DAMPING: PinJointParam = PinJointParam {
        ord: 1i32
    };
    #[doc(alias = "PIN_JOINT_IMPULSE_CLAMP")]
    #[doc = "Godot enumerator name: `PIN_JOINT_IMPULSE_CLAMP`"]
    pub const IMPULSE_CLAMP: PinJointParam = PinJointParam {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PinJointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PinJointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PinJointParam {
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
            Self::BIAS => "BIAS", Self::DAMPING => "DAMPING", Self::IMPULSE_CLAMP => "IMPULSE_CLAMP", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BIAS => "PIN_JOINT_BIAS", Self::DAMPING => "PIN_JOINT_DAMPING", Self::IMPULSE_CLAMP => "PIN_JOINT_IMPULSE_CLAMP", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PinJointParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PinJointParam {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PinJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HingeJointParam {
    ord: i32
}
impl HingeJointParam {
    #[doc(alias = "HINGE_JOINT_BIAS")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_BIAS`"]
    pub const BIAS: HingeJointParam = HingeJointParam {
        ord: 0i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_UPPER`"]
    pub const LIMIT_UPPER: HingeJointParam = HingeJointParam {
        ord: 1i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_LOWER`"]
    pub const LIMIT_LOWER: HingeJointParam = HingeJointParam {
        ord: 2i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_BIAS")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_BIAS`"]
    pub const LIMIT_BIAS: HingeJointParam = HingeJointParam {
        ord: 3i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_SOFTNESS`"]
    pub const LIMIT_SOFTNESS: HingeJointParam = HingeJointParam {
        ord: 4i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_RELAXATION")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_RELAXATION`"]
    pub const LIMIT_RELAXATION: HingeJointParam = HingeJointParam {
        ord: 5i32
    };
    #[doc(alias = "HINGE_JOINT_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_MOTOR_TARGET_VELOCITY`"]
    pub const MOTOR_TARGET_VELOCITY: HingeJointParam = HingeJointParam {
        ord: 6i32
    };
    #[doc(alias = "HINGE_JOINT_MOTOR_MAX_IMPULSE")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_MOTOR_MAX_IMPULSE`"]
    pub const MOTOR_MAX_IMPULSE: HingeJointParam = HingeJointParam {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for HingeJointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HingeJointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HingeJointParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::BIAS => "BIAS", Self::LIMIT_UPPER => "LIMIT_UPPER", Self::LIMIT_LOWER => "LIMIT_LOWER", Self::LIMIT_BIAS => "LIMIT_BIAS", Self::LIMIT_SOFTNESS => "LIMIT_SOFTNESS", Self::LIMIT_RELAXATION => "LIMIT_RELAXATION", Self::MOTOR_TARGET_VELOCITY => "MOTOR_TARGET_VELOCITY", Self::MOTOR_MAX_IMPULSE => "MOTOR_MAX_IMPULSE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BIAS => "HINGE_JOINT_BIAS", Self::LIMIT_UPPER => "HINGE_JOINT_LIMIT_UPPER", Self::LIMIT_LOWER => "HINGE_JOINT_LIMIT_LOWER", Self::LIMIT_BIAS => "HINGE_JOINT_LIMIT_BIAS", Self::LIMIT_SOFTNESS => "HINGE_JOINT_LIMIT_SOFTNESS", Self::LIMIT_RELAXATION => "HINGE_JOINT_LIMIT_RELAXATION", Self::MOTOR_TARGET_VELOCITY => "HINGE_JOINT_MOTOR_TARGET_VELOCITY", Self::MOTOR_MAX_IMPULSE => "HINGE_JOINT_MOTOR_MAX_IMPULSE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for HingeJointParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HingeJointParam {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HingeJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HingeJointFlag {
    ord: i32
}
impl HingeJointFlag {
    #[doc(alias = "HINGE_JOINT_FLAG_USE_LIMIT")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_FLAG_USE_LIMIT`"]
    pub const USE_LIMIT: HingeJointFlag = HingeJointFlag {
        ord: 0i32
    };
    #[doc(alias = "HINGE_JOINT_FLAG_ENABLE_MOTOR")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_FLAG_ENABLE_MOTOR`"]
    pub const ENABLE_MOTOR: HingeJointFlag = HingeJointFlag {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for HingeJointFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HingeJointFlag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HingeJointFlag {
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
            Self::USE_LIMIT => "USE_LIMIT", Self::ENABLE_MOTOR => "ENABLE_MOTOR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::USE_LIMIT => "HINGE_JOINT_FLAG_USE_LIMIT", Self::ENABLE_MOTOR => "HINGE_JOINT_FLAG_ENABLE_MOTOR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for HingeJointFlag {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HingeJointFlag {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HingeJointFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SliderJointParam {
    ord: i32
}
impl SliderJointParam {
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_UPPER`"]
    pub const LINEAR_LIMIT_UPPER: SliderJointParam = SliderJointParam {
        ord: 0i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_LOWER`"]
    pub const LINEAR_LIMIT_LOWER: SliderJointParam = SliderJointParam {
        ord: 1i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_SOFTNESS`"]
    pub const LINEAR_LIMIT_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 2i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_RESTITUTION`"]
    pub const LINEAR_LIMIT_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 3i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_DAMPING`"]
    pub const LINEAR_LIMIT_DAMPING: SliderJointParam = SliderJointParam {
        ord: 4i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_MOTION_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_MOTION_SOFTNESS`"]
    pub const LINEAR_MOTION_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 5i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_MOTION_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_MOTION_RESTITUTION`"]
    pub const LINEAR_MOTION_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 6i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_MOTION_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_MOTION_DAMPING`"]
    pub const LINEAR_MOTION_DAMPING: SliderJointParam = SliderJointParam {
        ord: 7i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_ORTHOGONAL_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_ORTHOGONAL_SOFTNESS`"]
    pub const LINEAR_ORTHOGONAL_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 8i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_ORTHOGONAL_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_ORTHOGONAL_RESTITUTION`"]
    pub const LINEAR_ORTHOGONAL_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 9i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_ORTHOGONAL_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_ORTHOGONAL_DAMPING`"]
    pub const LINEAR_ORTHOGONAL_DAMPING: SliderJointParam = SliderJointParam {
        ord: 10i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_UPPER`"]
    pub const ANGULAR_LIMIT_UPPER: SliderJointParam = SliderJointParam {
        ord: 11i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_LOWER`"]
    pub const ANGULAR_LIMIT_LOWER: SliderJointParam = SliderJointParam {
        ord: 12i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_SOFTNESS`"]
    pub const ANGULAR_LIMIT_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 13i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_RESTITUTION`"]
    pub const ANGULAR_LIMIT_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 14i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_DAMPING`"]
    pub const ANGULAR_LIMIT_DAMPING: SliderJointParam = SliderJointParam {
        ord: 15i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_MOTION_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_MOTION_SOFTNESS`"]
    pub const ANGULAR_MOTION_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 16i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_MOTION_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_MOTION_RESTITUTION`"]
    pub const ANGULAR_MOTION_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 17i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_MOTION_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_MOTION_DAMPING`"]
    pub const ANGULAR_MOTION_DAMPING: SliderJointParam = SliderJointParam {
        ord: 18i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_ORTHOGONAL_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_ORTHOGONAL_SOFTNESS`"]
    pub const ANGULAR_ORTHOGONAL_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 19i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_ORTHOGONAL_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_ORTHOGONAL_RESTITUTION`"]
    pub const ANGULAR_ORTHOGONAL_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 20i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_ORTHOGONAL_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_ORTHOGONAL_DAMPING`"]
    pub const ANGULAR_ORTHOGONAL_DAMPING: SliderJointParam = SliderJointParam {
        ord: 21i32
    };
    #[doc(alias = "SLIDER_JOINT_MAX")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_MAX`"]
    pub const MAX: SliderJointParam = SliderJointParam {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for SliderJointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SliderJointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SliderJointParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::LINEAR_LIMIT_UPPER => "LINEAR_LIMIT_UPPER", Self::LINEAR_LIMIT_LOWER => "LINEAR_LIMIT_LOWER", Self::LINEAR_LIMIT_SOFTNESS => "LINEAR_LIMIT_SOFTNESS", Self::LINEAR_LIMIT_RESTITUTION => "LINEAR_LIMIT_RESTITUTION", Self::LINEAR_LIMIT_DAMPING => "LINEAR_LIMIT_DAMPING", Self::LINEAR_MOTION_SOFTNESS => "LINEAR_MOTION_SOFTNESS", Self::LINEAR_MOTION_RESTITUTION => "LINEAR_MOTION_RESTITUTION", Self::LINEAR_MOTION_DAMPING => "LINEAR_MOTION_DAMPING", Self::LINEAR_ORTHOGONAL_SOFTNESS => "LINEAR_ORTHOGONAL_SOFTNESS", Self::LINEAR_ORTHOGONAL_RESTITUTION => "LINEAR_ORTHOGONAL_RESTITUTION", Self::LINEAR_ORTHOGONAL_DAMPING => "LINEAR_ORTHOGONAL_DAMPING", Self::ANGULAR_LIMIT_UPPER => "ANGULAR_LIMIT_UPPER", Self::ANGULAR_LIMIT_LOWER => "ANGULAR_LIMIT_LOWER", Self::ANGULAR_LIMIT_SOFTNESS => "ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_LIMIT_RESTITUTION => "ANGULAR_LIMIT_RESTITUTION", Self::ANGULAR_LIMIT_DAMPING => "ANGULAR_LIMIT_DAMPING", Self::ANGULAR_MOTION_SOFTNESS => "ANGULAR_MOTION_SOFTNESS", Self::ANGULAR_MOTION_RESTITUTION => "ANGULAR_MOTION_RESTITUTION", Self::ANGULAR_MOTION_DAMPING => "ANGULAR_MOTION_DAMPING", Self::ANGULAR_ORTHOGONAL_SOFTNESS => "ANGULAR_ORTHOGONAL_SOFTNESS", Self::ANGULAR_ORTHOGONAL_RESTITUTION => "ANGULAR_ORTHOGONAL_RESTITUTION", Self::ANGULAR_ORTHOGONAL_DAMPING => "ANGULAR_ORTHOGONAL_DAMPING", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LINEAR_LIMIT_UPPER => "SLIDER_JOINT_LINEAR_LIMIT_UPPER", Self::LINEAR_LIMIT_LOWER => "SLIDER_JOINT_LINEAR_LIMIT_LOWER", Self::LINEAR_LIMIT_SOFTNESS => "SLIDER_JOINT_LINEAR_LIMIT_SOFTNESS", Self::LINEAR_LIMIT_RESTITUTION => "SLIDER_JOINT_LINEAR_LIMIT_RESTITUTION", Self::LINEAR_LIMIT_DAMPING => "SLIDER_JOINT_LINEAR_LIMIT_DAMPING", Self::LINEAR_MOTION_SOFTNESS => "SLIDER_JOINT_LINEAR_MOTION_SOFTNESS", Self::LINEAR_MOTION_RESTITUTION => "SLIDER_JOINT_LINEAR_MOTION_RESTITUTION", Self::LINEAR_MOTION_DAMPING => "SLIDER_JOINT_LINEAR_MOTION_DAMPING", Self::LINEAR_ORTHOGONAL_SOFTNESS => "SLIDER_JOINT_LINEAR_ORTHOGONAL_SOFTNESS", Self::LINEAR_ORTHOGONAL_RESTITUTION => "SLIDER_JOINT_LINEAR_ORTHOGONAL_RESTITUTION", Self::LINEAR_ORTHOGONAL_DAMPING => "SLIDER_JOINT_LINEAR_ORTHOGONAL_DAMPING", Self::ANGULAR_LIMIT_UPPER => "SLIDER_JOINT_ANGULAR_LIMIT_UPPER", Self::ANGULAR_LIMIT_LOWER => "SLIDER_JOINT_ANGULAR_LIMIT_LOWER", Self::ANGULAR_LIMIT_SOFTNESS => "SLIDER_JOINT_ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_LIMIT_RESTITUTION => "SLIDER_JOINT_ANGULAR_LIMIT_RESTITUTION", Self::ANGULAR_LIMIT_DAMPING => "SLIDER_JOINT_ANGULAR_LIMIT_DAMPING", Self::ANGULAR_MOTION_SOFTNESS => "SLIDER_JOINT_ANGULAR_MOTION_SOFTNESS", Self::ANGULAR_MOTION_RESTITUTION => "SLIDER_JOINT_ANGULAR_MOTION_RESTITUTION", Self::ANGULAR_MOTION_DAMPING => "SLIDER_JOINT_ANGULAR_MOTION_DAMPING", Self::ANGULAR_ORTHOGONAL_SOFTNESS => "SLIDER_JOINT_ANGULAR_ORTHOGONAL_SOFTNESS", Self::ANGULAR_ORTHOGONAL_RESTITUTION => "SLIDER_JOINT_ANGULAR_ORTHOGONAL_RESTITUTION", Self::ANGULAR_ORTHOGONAL_DAMPING => "SLIDER_JOINT_ANGULAR_ORTHOGONAL_DAMPING", Self::MAX => "SLIDER_JOINT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for SliderJointParam {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::meta::GodotConvert for SliderJointParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SliderJointParam {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SliderJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConeTwistJointParam {
    ord: i32
}
impl ConeTwistJointParam {
    #[doc(alias = "CONE_TWIST_JOINT_SWING_SPAN")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_SWING_SPAN`"]
    pub const SWING_SPAN: ConeTwistJointParam = ConeTwistJointParam {
        ord: 0i32
    };
    #[doc(alias = "CONE_TWIST_JOINT_TWIST_SPAN")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_TWIST_SPAN`"]
    pub const TWIST_SPAN: ConeTwistJointParam = ConeTwistJointParam {
        ord: 1i32
    };
    #[doc(alias = "CONE_TWIST_JOINT_BIAS")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_BIAS`"]
    pub const BIAS: ConeTwistJointParam = ConeTwistJointParam {
        ord: 2i32
    };
    #[doc(alias = "CONE_TWIST_JOINT_SOFTNESS")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_SOFTNESS`"]
    pub const SOFTNESS: ConeTwistJointParam = ConeTwistJointParam {
        ord: 3i32
    };
    #[doc(alias = "CONE_TWIST_JOINT_RELAXATION")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_RELAXATION`"]
    pub const RELAXATION: ConeTwistJointParam = ConeTwistJointParam {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ConeTwistJointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ConeTwistJointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ConeTwistJointParam {
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
            Self::SWING_SPAN => "SWING_SPAN", Self::TWIST_SPAN => "TWIST_SPAN", Self::BIAS => "BIAS", Self::SOFTNESS => "SOFTNESS", Self::RELAXATION => "RELAXATION", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SWING_SPAN => "CONE_TWIST_JOINT_SWING_SPAN", Self::TWIST_SPAN => "CONE_TWIST_JOINT_TWIST_SPAN", Self::BIAS => "CONE_TWIST_JOINT_BIAS", Self::SOFTNESS => "CONE_TWIST_JOINT_SOFTNESS", Self::RELAXATION => "CONE_TWIST_JOINT_RELAXATION", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ConeTwistJointParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ConeTwistJointParam {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConeTwistJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `G6DOFJointAxisParam`."]
pub struct G6dofJointAxisParam {
    ord: i32
}
impl G6dofJointAxisParam {
    #[doc(alias = "G6DOF_JOINT_LINEAR_LOWER_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_LOWER_LIMIT`"]
    pub const LINEAR_LOWER_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 0i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_UPPER_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_UPPER_LIMIT`"]
    pub const LINEAR_UPPER_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 1i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_LIMIT_SOFTNESS`"]
    pub const LINEAR_LIMIT_SOFTNESS: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 2i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_RESTITUTION")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_RESTITUTION`"]
    pub const LINEAR_RESTITUTION: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 3i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_DAMPING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_DAMPING`"]
    pub const LINEAR_DAMPING: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 4i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_MOTOR_TARGET_VELOCITY`"]
    pub const LINEAR_MOTOR_TARGET_VELOCITY: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 5i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_MOTOR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_MOTOR_FORCE_LIMIT`"]
    pub const LINEAR_MOTOR_FORCE_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 6i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_SPRING_STIFFNESS")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_SPRING_STIFFNESS`"]
    pub const LINEAR_SPRING_STIFFNESS: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 7i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_SPRING_DAMPING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_SPRING_DAMPING`"]
    pub const LINEAR_SPRING_DAMPING: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 8i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_SPRING_EQUILIBRIUM_POINT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_SPRING_EQUILIBRIUM_POINT`"]
    pub const LINEAR_SPRING_EQUILIBRIUM_POINT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 9i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_LOWER_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_LOWER_LIMIT`"]
    pub const ANGULAR_LOWER_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 10i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_UPPER_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_UPPER_LIMIT`"]
    pub const ANGULAR_UPPER_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 11i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_LIMIT_SOFTNESS`"]
    pub const ANGULAR_LIMIT_SOFTNESS: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 12i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_DAMPING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_DAMPING`"]
    pub const ANGULAR_DAMPING: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 13i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_RESTITUTION")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_RESTITUTION`"]
    pub const ANGULAR_RESTITUTION: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 14i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_FORCE_LIMIT`"]
    pub const ANGULAR_FORCE_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 15i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_ERP")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_ERP`"]
    pub const ANGULAR_ERP: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 16i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_MOTOR_TARGET_VELOCITY`"]
    pub const ANGULAR_MOTOR_TARGET_VELOCITY: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 17i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_MOTOR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_MOTOR_FORCE_LIMIT`"]
    pub const ANGULAR_MOTOR_FORCE_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 18i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_SPRING_STIFFNESS")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_SPRING_STIFFNESS`"]
    pub const ANGULAR_SPRING_STIFFNESS: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 19i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_SPRING_DAMPING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_SPRING_DAMPING`"]
    pub const ANGULAR_SPRING_DAMPING: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 20i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_SPRING_EQUILIBRIUM_POINT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_SPRING_EQUILIBRIUM_POINT`"]
    pub const ANGULAR_SPRING_EQUILIBRIUM_POINT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 21i32
    };
    #[doc(alias = "G6DOF_JOINT_MAX")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_MAX`"]
    pub const MAX: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for G6dofJointAxisParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("G6dofJointAxisParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for G6dofJointAxisParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::LINEAR_LOWER_LIMIT => "LINEAR_LOWER_LIMIT", Self::LINEAR_UPPER_LIMIT => "LINEAR_UPPER_LIMIT", Self::LINEAR_LIMIT_SOFTNESS => "LINEAR_LIMIT_SOFTNESS", Self::LINEAR_RESTITUTION => "LINEAR_RESTITUTION", Self::LINEAR_DAMPING => "LINEAR_DAMPING", Self::LINEAR_MOTOR_TARGET_VELOCITY => "LINEAR_MOTOR_TARGET_VELOCITY", Self::LINEAR_MOTOR_FORCE_LIMIT => "LINEAR_MOTOR_FORCE_LIMIT", Self::LINEAR_SPRING_STIFFNESS => "LINEAR_SPRING_STIFFNESS", Self::LINEAR_SPRING_DAMPING => "LINEAR_SPRING_DAMPING", Self::LINEAR_SPRING_EQUILIBRIUM_POINT => "LINEAR_SPRING_EQUILIBRIUM_POINT", Self::ANGULAR_LOWER_LIMIT => "ANGULAR_LOWER_LIMIT", Self::ANGULAR_UPPER_LIMIT => "ANGULAR_UPPER_LIMIT", Self::ANGULAR_LIMIT_SOFTNESS => "ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_DAMPING => "ANGULAR_DAMPING", Self::ANGULAR_RESTITUTION => "ANGULAR_RESTITUTION", Self::ANGULAR_FORCE_LIMIT => "ANGULAR_FORCE_LIMIT", Self::ANGULAR_ERP => "ANGULAR_ERP", Self::ANGULAR_MOTOR_TARGET_VELOCITY => "ANGULAR_MOTOR_TARGET_VELOCITY", Self::ANGULAR_MOTOR_FORCE_LIMIT => "ANGULAR_MOTOR_FORCE_LIMIT", Self::ANGULAR_SPRING_STIFFNESS => "ANGULAR_SPRING_STIFFNESS", Self::ANGULAR_SPRING_DAMPING => "ANGULAR_SPRING_DAMPING", Self::ANGULAR_SPRING_EQUILIBRIUM_POINT => "ANGULAR_SPRING_EQUILIBRIUM_POINT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LINEAR_LOWER_LIMIT => "G6DOF_JOINT_LINEAR_LOWER_LIMIT", Self::LINEAR_UPPER_LIMIT => "G6DOF_JOINT_LINEAR_UPPER_LIMIT", Self::LINEAR_LIMIT_SOFTNESS => "G6DOF_JOINT_LINEAR_LIMIT_SOFTNESS", Self::LINEAR_RESTITUTION => "G6DOF_JOINT_LINEAR_RESTITUTION", Self::LINEAR_DAMPING => "G6DOF_JOINT_LINEAR_DAMPING", Self::LINEAR_MOTOR_TARGET_VELOCITY => "G6DOF_JOINT_LINEAR_MOTOR_TARGET_VELOCITY", Self::LINEAR_MOTOR_FORCE_LIMIT => "G6DOF_JOINT_LINEAR_MOTOR_FORCE_LIMIT", Self::LINEAR_SPRING_STIFFNESS => "G6DOF_JOINT_LINEAR_SPRING_STIFFNESS", Self::LINEAR_SPRING_DAMPING => "G6DOF_JOINT_LINEAR_SPRING_DAMPING", Self::LINEAR_SPRING_EQUILIBRIUM_POINT => "G6DOF_JOINT_LINEAR_SPRING_EQUILIBRIUM_POINT", Self::ANGULAR_LOWER_LIMIT => "G6DOF_JOINT_ANGULAR_LOWER_LIMIT", Self::ANGULAR_UPPER_LIMIT => "G6DOF_JOINT_ANGULAR_UPPER_LIMIT", Self::ANGULAR_LIMIT_SOFTNESS => "G6DOF_JOINT_ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_DAMPING => "G6DOF_JOINT_ANGULAR_DAMPING", Self::ANGULAR_RESTITUTION => "G6DOF_JOINT_ANGULAR_RESTITUTION", Self::ANGULAR_FORCE_LIMIT => "G6DOF_JOINT_ANGULAR_FORCE_LIMIT", Self::ANGULAR_ERP => "G6DOF_JOINT_ANGULAR_ERP", Self::ANGULAR_MOTOR_TARGET_VELOCITY => "G6DOF_JOINT_ANGULAR_MOTOR_TARGET_VELOCITY", Self::ANGULAR_MOTOR_FORCE_LIMIT => "G6DOF_JOINT_ANGULAR_MOTOR_FORCE_LIMIT", Self::ANGULAR_SPRING_STIFFNESS => "G6DOF_JOINT_ANGULAR_SPRING_STIFFNESS", Self::ANGULAR_SPRING_DAMPING => "G6DOF_JOINT_ANGULAR_SPRING_DAMPING", Self::ANGULAR_SPRING_EQUILIBRIUM_POINT => "G6DOF_JOINT_ANGULAR_SPRING_EQUILIBRIUM_POINT", Self::MAX => "G6DOF_JOINT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for G6dofJointAxisParam {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::meta::GodotConvert for G6dofJointAxisParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for G6dofJointAxisParam {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for G6dofJointAxisParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `G6DOFJointAxisFlag`."]
pub struct G6dofJointAxisFlag {
    ord: i32
}
impl G6dofJointAxisFlag {
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_LINEAR_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_LINEAR_LIMIT`"]
    pub const ENABLE_LINEAR_LIMIT: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 0i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_ANGULAR_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_ANGULAR_LIMIT`"]
    pub const ENABLE_ANGULAR_LIMIT: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 1i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_ANGULAR_SPRING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_ANGULAR_SPRING`"]
    pub const ENABLE_ANGULAR_SPRING: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 2i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_LINEAR_SPRING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_LINEAR_SPRING`"]
    pub const ENABLE_LINEAR_SPRING: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 3i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_MOTOR")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_MOTOR`"]
    pub const ENABLE_MOTOR: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 4i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_LINEAR_MOTOR")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_LINEAR_MOTOR`"]
    pub const ENABLE_LINEAR_MOTOR: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 5i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_MAX")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_MAX`"]
    pub const MAX: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for G6dofJointAxisFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("G6dofJointAxisFlag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for G6dofJointAxisFlag {
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
            Self::ENABLE_LINEAR_LIMIT => "ENABLE_LINEAR_LIMIT", Self::ENABLE_ANGULAR_LIMIT => "ENABLE_ANGULAR_LIMIT", Self::ENABLE_ANGULAR_SPRING => "ENABLE_ANGULAR_SPRING", Self::ENABLE_LINEAR_SPRING => "ENABLE_LINEAR_SPRING", Self::ENABLE_MOTOR => "ENABLE_MOTOR", Self::ENABLE_LINEAR_MOTOR => "ENABLE_LINEAR_MOTOR", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ENABLE_LINEAR_LIMIT => "G6DOF_JOINT_FLAG_ENABLE_LINEAR_LIMIT", Self::ENABLE_ANGULAR_LIMIT => "G6DOF_JOINT_FLAG_ENABLE_ANGULAR_LIMIT", Self::ENABLE_ANGULAR_SPRING => "G6DOF_JOINT_FLAG_ENABLE_ANGULAR_SPRING", Self::ENABLE_LINEAR_SPRING => "G6DOF_JOINT_FLAG_ENABLE_LINEAR_SPRING", Self::ENABLE_MOTOR => "G6DOF_JOINT_FLAG_ENABLE_MOTOR", Self::ENABLE_LINEAR_MOTOR => "G6DOF_JOINT_FLAG_ENABLE_LINEAR_MOTOR", Self::MAX => "G6DOF_JOINT_FLAG_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for G6dofJointAxisFlag {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for G6dofJointAxisFlag {
    type Via = i32;
    
}
impl crate::meta::ToGodot for G6dofJointAxisFlag {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for G6dofJointAxisFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShapeType {
    ord: i32
}
impl ShapeType {
    #[doc(alias = "SHAPE_WORLD_BOUNDARY")]
    #[doc = "Godot enumerator name: `SHAPE_WORLD_BOUNDARY`"]
    pub const WORLD_BOUNDARY: ShapeType = ShapeType {
        ord: 0i32
    };
    #[doc(alias = "SHAPE_SEPARATION_RAY")]
    #[doc = "Godot enumerator name: `SHAPE_SEPARATION_RAY`"]
    pub const SEPARATION_RAY: ShapeType = ShapeType {
        ord: 1i32
    };
    #[doc(alias = "SHAPE_SPHERE")]
    #[doc = "Godot enumerator name: `SHAPE_SPHERE`"]
    pub const SPHERE: ShapeType = ShapeType {
        ord: 2i32
    };
    #[doc(alias = "SHAPE_BOX")]
    #[doc = "Godot enumerator name: `SHAPE_BOX`"]
    pub const BOX: ShapeType = ShapeType {
        ord: 3i32
    };
    #[doc(alias = "SHAPE_CAPSULE")]
    #[doc = "Godot enumerator name: `SHAPE_CAPSULE`"]
    pub const CAPSULE: ShapeType = ShapeType {
        ord: 4i32
    };
    #[doc(alias = "SHAPE_CYLINDER")]
    #[doc = "Godot enumerator name: `SHAPE_CYLINDER`"]
    pub const CYLINDER: ShapeType = ShapeType {
        ord: 5i32
    };
    #[doc(alias = "SHAPE_CONVEX_POLYGON")]
    #[doc = "Godot enumerator name: `SHAPE_CONVEX_POLYGON`"]
    pub const CONVEX_POLYGON: ShapeType = ShapeType {
        ord: 6i32
    };
    #[doc(alias = "SHAPE_CONCAVE_POLYGON")]
    #[doc = "Godot enumerator name: `SHAPE_CONCAVE_POLYGON`"]
    pub const CONCAVE_POLYGON: ShapeType = ShapeType {
        ord: 7i32
    };
    #[doc(alias = "SHAPE_HEIGHTMAP")]
    #[doc = "Godot enumerator name: `SHAPE_HEIGHTMAP`"]
    pub const HEIGHTMAP: ShapeType = ShapeType {
        ord: 8i32
    };
    #[doc(alias = "SHAPE_SOFT_BODY")]
    #[doc = "Godot enumerator name: `SHAPE_SOFT_BODY`"]
    pub const SOFT_BODY: ShapeType = ShapeType {
        ord: 9i32
    };
    #[doc(alias = "SHAPE_CUSTOM")]
    #[doc = "Godot enumerator name: `SHAPE_CUSTOM`"]
    pub const CUSTOM: ShapeType = ShapeType {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for ShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShapeType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShapeType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::WORLD_BOUNDARY => "WORLD_BOUNDARY", Self::SEPARATION_RAY => "SEPARATION_RAY", Self::SPHERE => "SPHERE", Self::BOX => "BOX", Self::CAPSULE => "CAPSULE", Self::CYLINDER => "CYLINDER", Self::CONVEX_POLYGON => "CONVEX_POLYGON", Self::CONCAVE_POLYGON => "CONCAVE_POLYGON", Self::HEIGHTMAP => "HEIGHTMAP", Self::SOFT_BODY => "SOFT_BODY", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::WORLD_BOUNDARY => "SHAPE_WORLD_BOUNDARY", Self::SEPARATION_RAY => "SHAPE_SEPARATION_RAY", Self::SPHERE => "SHAPE_SPHERE", Self::BOX => "SHAPE_BOX", Self::CAPSULE => "SHAPE_CAPSULE", Self::CYLINDER => "SHAPE_CYLINDER", Self::CONVEX_POLYGON => "SHAPE_CONVEX_POLYGON", Self::CONCAVE_POLYGON => "SHAPE_CONCAVE_POLYGON", Self::HEIGHTMAP => "SHAPE_HEIGHTMAP", Self::SOFT_BODY => "SHAPE_SOFT_BODY", Self::CUSTOM => "SHAPE_CUSTOM", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ShapeType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShapeType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShapeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AreaParameter {
    ord: i32
}
impl AreaParameter {
    #[doc(alias = "AREA_PARAM_GRAVITY_OVERRIDE_MODE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY_OVERRIDE_MODE`"]
    pub const GRAVITY_OVERRIDE_MODE: AreaParameter = AreaParameter {
        ord: 0i32
    };
    #[doc(alias = "AREA_PARAM_GRAVITY")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY`"]
    pub const GRAVITY: AreaParameter = AreaParameter {
        ord: 1i32
    };
    #[doc(alias = "AREA_PARAM_GRAVITY_VECTOR")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY_VECTOR`"]
    pub const GRAVITY_VECTOR: AreaParameter = AreaParameter {
        ord: 2i32
    };
    #[doc(alias = "AREA_PARAM_GRAVITY_IS_POINT")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY_IS_POINT`"]
    pub const GRAVITY_IS_POINT: AreaParameter = AreaParameter {
        ord: 3i32
    };
    #[doc(alias = "AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE`"]
    pub const GRAVITY_POINT_UNIT_DISTANCE: AreaParameter = AreaParameter {
        ord: 4i32
    };
    #[doc(alias = "AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE`"]
    pub const LINEAR_DAMP_OVERRIDE_MODE: AreaParameter = AreaParameter {
        ord: 5i32
    };
    #[doc(alias = "AREA_PARAM_LINEAR_DAMP")]
    #[doc = "Godot enumerator name: `AREA_PARAM_LINEAR_DAMP`"]
    pub const LINEAR_DAMP: AreaParameter = AreaParameter {
        ord: 6i32
    };
    #[doc(alias = "AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE`"]
    pub const ANGULAR_DAMP_OVERRIDE_MODE: AreaParameter = AreaParameter {
        ord: 7i32
    };
    #[doc(alias = "AREA_PARAM_ANGULAR_DAMP")]
    #[doc = "Godot enumerator name: `AREA_PARAM_ANGULAR_DAMP`"]
    pub const ANGULAR_DAMP: AreaParameter = AreaParameter {
        ord: 8i32
    };
    #[doc(alias = "AREA_PARAM_PRIORITY")]
    #[doc = "Godot enumerator name: `AREA_PARAM_PRIORITY`"]
    pub const PRIORITY: AreaParameter = AreaParameter {
        ord: 9i32
    };
    #[doc(alias = "AREA_PARAM_WIND_FORCE_MAGNITUDE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_WIND_FORCE_MAGNITUDE`"]
    pub const WIND_FORCE_MAGNITUDE: AreaParameter = AreaParameter {
        ord: 10i32
    };
    #[doc(alias = "AREA_PARAM_WIND_SOURCE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_WIND_SOURCE`"]
    pub const WIND_SOURCE: AreaParameter = AreaParameter {
        ord: 11i32
    };
    #[doc(alias = "AREA_PARAM_WIND_DIRECTION")]
    #[doc = "Godot enumerator name: `AREA_PARAM_WIND_DIRECTION`"]
    pub const WIND_DIRECTION: AreaParameter = AreaParameter {
        ord: 12i32
    };
    #[doc(alias = "AREA_PARAM_WIND_ATTENUATION_FACTOR")]
    #[doc = "Godot enumerator name: `AREA_PARAM_WIND_ATTENUATION_FACTOR`"]
    pub const WIND_ATTENUATION_FACTOR: AreaParameter = AreaParameter {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for AreaParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AreaParameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AreaParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::GRAVITY_OVERRIDE_MODE => "GRAVITY_OVERRIDE_MODE", Self::GRAVITY => "GRAVITY", Self::GRAVITY_VECTOR => "GRAVITY_VECTOR", Self::GRAVITY_IS_POINT => "GRAVITY_IS_POINT", Self::GRAVITY_POINT_UNIT_DISTANCE => "GRAVITY_POINT_UNIT_DISTANCE", Self::LINEAR_DAMP_OVERRIDE_MODE => "LINEAR_DAMP_OVERRIDE_MODE", Self::LINEAR_DAMP => "LINEAR_DAMP", Self::ANGULAR_DAMP_OVERRIDE_MODE => "ANGULAR_DAMP_OVERRIDE_MODE", Self::ANGULAR_DAMP => "ANGULAR_DAMP", Self::PRIORITY => "PRIORITY", Self::WIND_FORCE_MAGNITUDE => "WIND_FORCE_MAGNITUDE", Self::WIND_SOURCE => "WIND_SOURCE", Self::WIND_DIRECTION => "WIND_DIRECTION", Self::WIND_ATTENUATION_FACTOR => "WIND_ATTENUATION_FACTOR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GRAVITY_OVERRIDE_MODE => "AREA_PARAM_GRAVITY_OVERRIDE_MODE", Self::GRAVITY => "AREA_PARAM_GRAVITY", Self::GRAVITY_VECTOR => "AREA_PARAM_GRAVITY_VECTOR", Self::GRAVITY_IS_POINT => "AREA_PARAM_GRAVITY_IS_POINT", Self::GRAVITY_POINT_UNIT_DISTANCE => "AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE", Self::LINEAR_DAMP_OVERRIDE_MODE => "AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE", Self::LINEAR_DAMP => "AREA_PARAM_LINEAR_DAMP", Self::ANGULAR_DAMP_OVERRIDE_MODE => "AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE", Self::ANGULAR_DAMP => "AREA_PARAM_ANGULAR_DAMP", Self::PRIORITY => "AREA_PARAM_PRIORITY", Self::WIND_FORCE_MAGNITUDE => "AREA_PARAM_WIND_FORCE_MAGNITUDE", Self::WIND_SOURCE => "AREA_PARAM_WIND_SOURCE", Self::WIND_DIRECTION => "AREA_PARAM_WIND_DIRECTION", Self::WIND_ATTENUATION_FACTOR => "AREA_PARAM_WIND_ATTENUATION_FACTOR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AreaParameter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AreaParameter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AreaParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AreaSpaceOverrideMode {
    ord: i32
}
impl AreaSpaceOverrideMode {
    #[doc(alias = "AREA_SPACE_OVERRIDE_DISABLED")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_DISABLED`"]
    pub const DISABLED: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 0i32
    };
    #[doc(alias = "AREA_SPACE_OVERRIDE_COMBINE")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_COMBINE`"]
    pub const COMBINE: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 1i32
    };
    #[doc(alias = "AREA_SPACE_OVERRIDE_COMBINE_REPLACE")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_COMBINE_REPLACE`"]
    pub const COMBINE_REPLACE: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 2i32
    };
    #[doc(alias = "AREA_SPACE_OVERRIDE_REPLACE")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_REPLACE`"]
    pub const REPLACE: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 3i32
    };
    #[doc(alias = "AREA_SPACE_OVERRIDE_REPLACE_COMBINE")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_REPLACE_COMBINE`"]
    pub const REPLACE_COMBINE: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for AreaSpaceOverrideMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AreaSpaceOverrideMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AreaSpaceOverrideMode {
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
            Self::DISABLED => "DISABLED", Self::COMBINE => "COMBINE", Self::COMBINE_REPLACE => "COMBINE_REPLACE", Self::REPLACE => "REPLACE", Self::REPLACE_COMBINE => "REPLACE_COMBINE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "AREA_SPACE_OVERRIDE_DISABLED", Self::COMBINE => "AREA_SPACE_OVERRIDE_COMBINE", Self::COMBINE_REPLACE => "AREA_SPACE_OVERRIDE_COMBINE_REPLACE", Self::REPLACE => "AREA_SPACE_OVERRIDE_REPLACE", Self::REPLACE_COMBINE => "AREA_SPACE_OVERRIDE_REPLACE_COMBINE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AreaSpaceOverrideMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AreaSpaceOverrideMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AreaSpaceOverrideMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyMode {
    ord: i32
}
impl BodyMode {
    #[doc(alias = "BODY_MODE_STATIC")]
    #[doc = "Godot enumerator name: `BODY_MODE_STATIC`"]
    pub const STATIC: BodyMode = BodyMode {
        ord: 0i32
    };
    #[doc(alias = "BODY_MODE_KINEMATIC")]
    #[doc = "Godot enumerator name: `BODY_MODE_KINEMATIC`"]
    pub const KINEMATIC: BodyMode = BodyMode {
        ord: 1i32
    };
    #[doc(alias = "BODY_MODE_RIGID")]
    #[doc = "Godot enumerator name: `BODY_MODE_RIGID`"]
    pub const RIGID: BodyMode = BodyMode {
        ord: 2i32
    };
    #[doc(alias = "BODY_MODE_RIGID_LINEAR")]
    #[doc = "Godot enumerator name: `BODY_MODE_RIGID_LINEAR`"]
    pub const RIGID_LINEAR: BodyMode = BodyMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for BodyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyMode {
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
            Self::STATIC => "STATIC", Self::KINEMATIC => "KINEMATIC", Self::RIGID => "RIGID", Self::RIGID_LINEAR => "RIGID_LINEAR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STATIC => "BODY_MODE_STATIC", Self::KINEMATIC => "BODY_MODE_KINEMATIC", Self::RIGID => "BODY_MODE_RIGID", Self::RIGID_LINEAR => "BODY_MODE_RIGID_LINEAR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BodyMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BodyMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyParameter {
    ord: i32
}
impl BodyParameter {
    #[doc(alias = "BODY_PARAM_BOUNCE")]
    #[doc = "Godot enumerator name: `BODY_PARAM_BOUNCE`"]
    pub const BOUNCE: BodyParameter = BodyParameter {
        ord: 0i32
    };
    #[doc(alias = "BODY_PARAM_FRICTION")]
    #[doc = "Godot enumerator name: `BODY_PARAM_FRICTION`"]
    pub const FRICTION: BodyParameter = BodyParameter {
        ord: 1i32
    };
    #[doc(alias = "BODY_PARAM_MASS")]
    #[doc = "Godot enumerator name: `BODY_PARAM_MASS`"]
    pub const MASS: BodyParameter = BodyParameter {
        ord: 2i32
    };
    #[doc(alias = "BODY_PARAM_INERTIA")]
    #[doc = "Godot enumerator name: `BODY_PARAM_INERTIA`"]
    pub const INERTIA: BodyParameter = BodyParameter {
        ord: 3i32
    };
    #[doc(alias = "BODY_PARAM_CENTER_OF_MASS")]
    #[doc = "Godot enumerator name: `BODY_PARAM_CENTER_OF_MASS`"]
    pub const CENTER_OF_MASS: BodyParameter = BodyParameter {
        ord: 4i32
    };
    #[doc(alias = "BODY_PARAM_GRAVITY_SCALE")]
    #[doc = "Godot enumerator name: `BODY_PARAM_GRAVITY_SCALE`"]
    pub const GRAVITY_SCALE: BodyParameter = BodyParameter {
        ord: 5i32
    };
    #[doc(alias = "BODY_PARAM_LINEAR_DAMP_MODE")]
    #[doc = "Godot enumerator name: `BODY_PARAM_LINEAR_DAMP_MODE`"]
    pub const LINEAR_DAMP_MODE: BodyParameter = BodyParameter {
        ord: 6i32
    };
    #[doc(alias = "BODY_PARAM_ANGULAR_DAMP_MODE")]
    #[doc = "Godot enumerator name: `BODY_PARAM_ANGULAR_DAMP_MODE`"]
    pub const ANGULAR_DAMP_MODE: BodyParameter = BodyParameter {
        ord: 7i32
    };
    #[doc(alias = "BODY_PARAM_LINEAR_DAMP")]
    #[doc = "Godot enumerator name: `BODY_PARAM_LINEAR_DAMP`"]
    pub const LINEAR_DAMP: BodyParameter = BodyParameter {
        ord: 8i32
    };
    #[doc(alias = "BODY_PARAM_ANGULAR_DAMP")]
    #[doc = "Godot enumerator name: `BODY_PARAM_ANGULAR_DAMP`"]
    pub const ANGULAR_DAMP: BodyParameter = BodyParameter {
        ord: 9i32
    };
    #[doc(alias = "BODY_PARAM_MAX")]
    #[doc = "Godot enumerator name: `BODY_PARAM_MAX`"]
    pub const MAX: BodyParameter = BodyParameter {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for BodyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyParameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::BOUNCE => "BOUNCE", Self::FRICTION => "FRICTION", Self::MASS => "MASS", Self::INERTIA => "INERTIA", Self::CENTER_OF_MASS => "CENTER_OF_MASS", Self::GRAVITY_SCALE => "GRAVITY_SCALE", Self::LINEAR_DAMP_MODE => "LINEAR_DAMP_MODE", Self::ANGULAR_DAMP_MODE => "ANGULAR_DAMP_MODE", Self::LINEAR_DAMP => "LINEAR_DAMP", Self::ANGULAR_DAMP => "ANGULAR_DAMP", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BOUNCE => "BODY_PARAM_BOUNCE", Self::FRICTION => "BODY_PARAM_FRICTION", Self::MASS => "BODY_PARAM_MASS", Self::INERTIA => "BODY_PARAM_INERTIA", Self::CENTER_OF_MASS => "BODY_PARAM_CENTER_OF_MASS", Self::GRAVITY_SCALE => "BODY_PARAM_GRAVITY_SCALE", Self::LINEAR_DAMP_MODE => "BODY_PARAM_LINEAR_DAMP_MODE", Self::ANGULAR_DAMP_MODE => "BODY_PARAM_ANGULAR_DAMP_MODE", Self::LINEAR_DAMP => "BODY_PARAM_LINEAR_DAMP", Self::ANGULAR_DAMP => "BODY_PARAM_ANGULAR_DAMP", Self::MAX => "BODY_PARAM_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for BodyParameter {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::meta::GodotConvert for BodyParameter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BodyParameter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyDampMode {
    ord: i32
}
impl BodyDampMode {
    #[doc(alias = "BODY_DAMP_MODE_COMBINE")]
    #[doc = "Godot enumerator name: `BODY_DAMP_MODE_COMBINE`"]
    pub const COMBINE: BodyDampMode = BodyDampMode {
        ord: 0i32
    };
    #[doc(alias = "BODY_DAMP_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `BODY_DAMP_MODE_REPLACE`"]
    pub const REPLACE: BodyDampMode = BodyDampMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for BodyDampMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyDampMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyDampMode {
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
            Self::COMBINE => "COMBINE", Self::REPLACE => "REPLACE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::COMBINE => "BODY_DAMP_MODE_COMBINE", Self::REPLACE => "BODY_DAMP_MODE_REPLACE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BodyDampMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BodyDampMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyDampMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyState {
    ord: i32
}
impl BodyState {
    #[doc(alias = "BODY_STATE_TRANSFORM")]
    #[doc = "Godot enumerator name: `BODY_STATE_TRANSFORM`"]
    pub const TRANSFORM: BodyState = BodyState {
        ord: 0i32
    };
    #[doc(alias = "BODY_STATE_LINEAR_VELOCITY")]
    #[doc = "Godot enumerator name: `BODY_STATE_LINEAR_VELOCITY`"]
    pub const LINEAR_VELOCITY: BodyState = BodyState {
        ord: 1i32
    };
    #[doc(alias = "BODY_STATE_ANGULAR_VELOCITY")]
    #[doc = "Godot enumerator name: `BODY_STATE_ANGULAR_VELOCITY`"]
    pub const ANGULAR_VELOCITY: BodyState = BodyState {
        ord: 2i32
    };
    #[doc(alias = "BODY_STATE_SLEEPING")]
    #[doc = "Godot enumerator name: `BODY_STATE_SLEEPING`"]
    pub const SLEEPING: BodyState = BodyState {
        ord: 3i32
    };
    #[doc(alias = "BODY_STATE_CAN_SLEEP")]
    #[doc = "Godot enumerator name: `BODY_STATE_CAN_SLEEP`"]
    pub const CAN_SLEEP: BodyState = BodyState {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for BodyState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyState {
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
            Self::TRANSFORM => "TRANSFORM", Self::LINEAR_VELOCITY => "LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "ANGULAR_VELOCITY", Self::SLEEPING => "SLEEPING", Self::CAN_SLEEP => "CAN_SLEEP", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TRANSFORM => "BODY_STATE_TRANSFORM", Self::LINEAR_VELOCITY => "BODY_STATE_LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "BODY_STATE_ANGULAR_VELOCITY", Self::SLEEPING => "BODY_STATE_SLEEPING", Self::CAN_SLEEP => "BODY_STATE_CAN_SLEEP", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BodyState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BodyState {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AreaBodyStatus {
    ord: i32
}
impl AreaBodyStatus {
    #[doc(alias = "AREA_BODY_ADDED")]
    #[doc = "Godot enumerator name: `AREA_BODY_ADDED`"]
    pub const ADDED: AreaBodyStatus = AreaBodyStatus {
        ord: 0i32
    };
    #[doc(alias = "AREA_BODY_REMOVED")]
    #[doc = "Godot enumerator name: `AREA_BODY_REMOVED`"]
    pub const REMOVED: AreaBodyStatus = AreaBodyStatus {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AreaBodyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AreaBodyStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AreaBodyStatus {
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
            Self::ADDED => "ADDED", Self::REMOVED => "REMOVED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ADDED => "AREA_BODY_ADDED", Self::REMOVED => "AREA_BODY_REMOVED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AreaBodyStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AreaBodyStatus {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AreaBodyStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessInfo {
    ord: i32
}
impl ProcessInfo {
    #[doc(alias = "INFO_ACTIVE_OBJECTS")]
    #[doc = "Godot enumerator name: `INFO_ACTIVE_OBJECTS`"]
    pub const ACTIVE_OBJECTS: ProcessInfo = ProcessInfo {
        ord: 0i32
    };
    #[doc(alias = "INFO_COLLISION_PAIRS")]
    #[doc = "Godot enumerator name: `INFO_COLLISION_PAIRS`"]
    pub const COLLISION_PAIRS: ProcessInfo = ProcessInfo {
        ord: 1i32
    };
    #[doc(alias = "INFO_ISLAND_COUNT")]
    #[doc = "Godot enumerator name: `INFO_ISLAND_COUNT`"]
    pub const ISLAND_COUNT: ProcessInfo = ProcessInfo {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessInfo") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessInfo {
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
            Self::ACTIVE_OBJECTS => "ACTIVE_OBJECTS", Self::COLLISION_PAIRS => "COLLISION_PAIRS", Self::ISLAND_COUNT => "ISLAND_COUNT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ACTIVE_OBJECTS => "INFO_ACTIVE_OBJECTS", Self::COLLISION_PAIRS => "INFO_COLLISION_PAIRS", Self::ISLAND_COUNT => "INFO_ISLAND_COUNT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ProcessInfo {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ProcessInfo {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpaceParameter {
    ord: i32
}
impl SpaceParameter {
    #[doc(alias = "SPACE_PARAM_CONTACT_RECYCLE_RADIUS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONTACT_RECYCLE_RADIUS`"]
    pub const CONTACT_RECYCLE_RADIUS: SpaceParameter = SpaceParameter {
        ord: 0i32
    };
    #[doc(alias = "SPACE_PARAM_CONTACT_MAX_SEPARATION")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONTACT_MAX_SEPARATION`"]
    pub const CONTACT_MAX_SEPARATION: SpaceParameter = SpaceParameter {
        ord: 1i32
    };
    #[doc(alias = "SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION`"]
    pub const CONTACT_MAX_ALLOWED_PENETRATION: SpaceParameter = SpaceParameter {
        ord: 2i32
    };
    #[doc(alias = "SPACE_PARAM_CONTACT_DEFAULT_BIAS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONTACT_DEFAULT_BIAS`"]
    pub const CONTACT_DEFAULT_BIAS: SpaceParameter = SpaceParameter {
        ord: 3i32
    };
    #[doc(alias = "SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD`"]
    pub const BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD: SpaceParameter = SpaceParameter {
        ord: 4i32
    };
    #[doc(alias = "SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD`"]
    pub const BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD: SpaceParameter = SpaceParameter {
        ord: 5i32
    };
    #[doc(alias = "SPACE_PARAM_BODY_TIME_TO_SLEEP")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_BODY_TIME_TO_SLEEP`"]
    pub const BODY_TIME_TO_SLEEP: SpaceParameter = SpaceParameter {
        ord: 6i32
    };
    #[doc(alias = "SPACE_PARAM_SOLVER_ITERATIONS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_SOLVER_ITERATIONS`"]
    pub const SOLVER_ITERATIONS: SpaceParameter = SpaceParameter {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for SpaceParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpaceParameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpaceParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::CONTACT_RECYCLE_RADIUS => "CONTACT_RECYCLE_RADIUS", Self::CONTACT_MAX_SEPARATION => "CONTACT_MAX_SEPARATION", Self::CONTACT_MAX_ALLOWED_PENETRATION => "CONTACT_MAX_ALLOWED_PENETRATION", Self::CONTACT_DEFAULT_BIAS => "CONTACT_DEFAULT_BIAS", Self::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD => "BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD => "BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_TIME_TO_SLEEP => "BODY_TIME_TO_SLEEP", Self::SOLVER_ITERATIONS => "SOLVER_ITERATIONS", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CONTACT_RECYCLE_RADIUS => "SPACE_PARAM_CONTACT_RECYCLE_RADIUS", Self::CONTACT_MAX_SEPARATION => "SPACE_PARAM_CONTACT_MAX_SEPARATION", Self::CONTACT_MAX_ALLOWED_PENETRATION => "SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION", Self::CONTACT_DEFAULT_BIAS => "SPACE_PARAM_CONTACT_DEFAULT_BIAS", Self::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD => "SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD => "SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_TIME_TO_SLEEP => "SPACE_PARAM_BODY_TIME_TO_SLEEP", Self::SOLVER_ITERATIONS => "SPACE_PARAM_SOLVER_ITERATIONS", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SpaceParameter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpaceParameter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpaceParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyAxis {
    ord: i32
}
impl BodyAxis {
    #[doc(alias = "BODY_AXIS_LINEAR_X")]
    #[doc = "Godot enumerator name: `BODY_AXIS_LINEAR_X`"]
    pub const LINEAR_X: BodyAxis = BodyAxis {
        ord: 1i32
    };
    #[doc(alias = "BODY_AXIS_LINEAR_Y")]
    #[doc = "Godot enumerator name: `BODY_AXIS_LINEAR_Y`"]
    pub const LINEAR_Y: BodyAxis = BodyAxis {
        ord: 2i32
    };
    #[doc(alias = "BODY_AXIS_LINEAR_Z")]
    #[doc = "Godot enumerator name: `BODY_AXIS_LINEAR_Z`"]
    pub const LINEAR_Z: BodyAxis = BodyAxis {
        ord: 4i32
    };
    #[doc(alias = "BODY_AXIS_ANGULAR_X")]
    #[doc = "Godot enumerator name: `BODY_AXIS_ANGULAR_X`"]
    pub const ANGULAR_X: BodyAxis = BodyAxis {
        ord: 8i32
    };
    #[doc(alias = "BODY_AXIS_ANGULAR_Y")]
    #[doc = "Godot enumerator name: `BODY_AXIS_ANGULAR_Y`"]
    pub const ANGULAR_Y: BodyAxis = BodyAxis {
        ord: 16i32
    };
    #[doc(alias = "BODY_AXIS_ANGULAR_Z")]
    #[doc = "Godot enumerator name: `BODY_AXIS_ANGULAR_Z`"]
    pub const ANGULAR_Z: BodyAxis = BodyAxis {
        ord: 32i32
    };
    
}
impl std::fmt::Debug for BodyAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyAxis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyAxis {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 => Some(Self {
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
            Self::LINEAR_X => "LINEAR_X", Self::LINEAR_Y => "LINEAR_Y", Self::LINEAR_Z => "LINEAR_Z", Self::ANGULAR_X => "ANGULAR_X", Self::ANGULAR_Y => "ANGULAR_Y", Self::ANGULAR_Z => "ANGULAR_Z", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LINEAR_X => "BODY_AXIS_LINEAR_X", Self::LINEAR_Y => "BODY_AXIS_LINEAR_Y", Self::LINEAR_Z => "BODY_AXIS_LINEAR_Z", Self::ANGULAR_X => "BODY_AXIS_ANGULAR_X", Self::ANGULAR_Y => "BODY_AXIS_ANGULAR_Y", Self::ANGULAR_Z => "BODY_AXIS_ANGULAR_Z", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BodyAxis {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BodyAxis {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}