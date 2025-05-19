#![doc = "Sidecar module for class [`PhysicsServer2D`][crate::classes::PhysicsServer2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer2D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_server_2d`][crate::classes::physics_server_2d]: sidecar module with related enum/flag types\n* [`IPhysicsServer2D`][crate::classes::IPhysicsServer2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer2D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`PhysicsServer2D::singleton()`][PhysicsServer2D::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsServer2D`][crate::classes::PhysicsServer2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsServer2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsServer2D: crate::obj::GodotClass < Base = PhysicsServer2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsServer2D {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"PhysicsServer2D");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn world_boundary_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "world_boundary_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn separation_ray_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "separation_ray_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "segment_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn circle_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "circle_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rectangle_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "rectangle_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capsule_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "capsule_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convex_polygon_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "convex_polygon_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn concave_polygon_shape_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "concave_polygon_shape_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_set_data(&mut self, shape: Rid, data: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Variant >);
            let args = (shape, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "shape_set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_type(&self, shape: Rid,) -> crate::classes::physics_server_2d::ShapeType {
            type CallSig = (crate::classes::physics_server_2d::ShapeType, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "shape_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shape_get_data(&self, shape: Rid,) -> Variant {
            type CallSig = (Variant, Rid);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "shape_get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_set_active(&mut self, space: Rid, active: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (space, active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_is_active(&self, space: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_set_param(&mut self, space: Rid, param: crate::classes::physics_server_2d::SpaceParameter, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_2d::SpaceParameter, f32);
            let args = (space, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_get_param(&self, space: Rid, param: crate::classes::physics_server_2d::SpaceParameter,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_2d::SpaceParameter);
            let args = (space, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState2D > > {
            type CallSig = (Option < Gd < crate::classes::PhysicsDirectSpaceState2D > >, Rid);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_get_direct_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_space(&mut self, area: Rid, space: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (area, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_space(&self, area: Rid,) -> Rid {
            type CallSig = (Rid, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn area_add_shape_full(&mut self, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,) {
            type CallSig = ((), Rid, Rid, Transform2D, bool);
            let args = (area, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_add_shape", self.object_ptr, self.__checked_id(), args,)
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
                let method_bind = sys::class_servers_api() . fptr_by_index(226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform2D,) {
            type CallSig = ((), Rid, i32, Transform2D);
            let args = (area, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (area, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_shape_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape_count(&self, area: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid {
            type CallSig = (Rid, Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform2D {
            type CallSig = (Transform2D, Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_clear_shapes(&mut self, area: Rid,) {
            type CallSig = ((), Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_collision_layer(&mut self, area: Rid, layer: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (area, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_collision_layer(&self, area: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_collision_mask(&mut self, area: Rid, mask: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (area, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_collision_mask(&self, area: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_param(&mut self, area: Rid, param: crate::classes::physics_server_2d::AreaParameter, value: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, crate::classes::physics_server_2d::AreaParameter, RefArg < 'a0, Variant >);
            let args = (area, param, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_transform(&mut self, area: Rid, transform: Transform2D,) {
            type CallSig = ((), Rid, Transform2D);
            let args = (area, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_param(&self, area: Rid, param: crate::classes::physics_server_2d::AreaParameter,) -> Variant {
            type CallSig = (Variant, Rid, crate::classes::physics_server_2d::AreaParameter);
            let args = (area, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_transform(&self, area: Rid,) -> Transform2D {
            type CallSig = (Transform2D, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,) {
            type CallSig = ((), Rid, u64);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_object_instance_id(&self, area: Rid,) -> u64 {
            type CallSig = (u64, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_attach_canvas_instance_id(&mut self, area: Rid, id: u64,) {
            type CallSig = ((), Rid, u64);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_attach_canvas_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_get_canvas_instance_id(&self, area: Rid,) -> u64 {
            type CallSig = (u64, Rid);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_canvas_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_monitor_callback(&mut self, area: Rid, callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Callable >);
            let args = (area, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_monitor_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_area_monitor_callback(&mut self, area: Rid, callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Callable >);
            let args = (area, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_area_monitor_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (area, monitorable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_monitorable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_space(&mut self, body: Rid, space: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_space(&self, body: Rid,) -> Rid {
            type CallSig = (Rid, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_mode(&mut self, body: Rid, mode: crate::classes::physics_server_2d::BodyMode,) {
            type CallSig = ((), Rid, crate::classes::physics_server_2d::BodyMode);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_mode(&self, body: Rid,) -> crate::classes::physics_server_2d::BodyMode {
            type CallSig = (crate::classes::physics_server_2d::BodyMode, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_add_shape_full(&mut self, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,) {
            type CallSig = ((), Rid, Rid, Transform2D, bool);
            let args = (body, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_shape", self.object_ptr, self.__checked_id(), args,)
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
                let method_bind = sys::class_servers_api() . fptr_by_index(255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform2D,) {
            type CallSig = ((), Rid, i32, Transform2D);
            let args = (body, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape_count(&self, body: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid {
            type CallSig = (Rid, Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform2D {
            type CallSig = (Transform2D, Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_shape_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_remove_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_clear_shapes(&mut self, body: Rid,) {
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_clear_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (body, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_shape_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_shape_as_one_way_collision(&mut self, body: Rid, shape_idx: i32, enable: bool, margin: f32,) {
            type CallSig = ((), Rid, i32, bool, f32);
            let args = (body, shape_idx, enable, margin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_shape_as_one_way_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,) {
            type CallSig = ((), Rid, u64);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_attach_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_object_instance_id(&self, body: Rid,) -> u64 {
            type CallSig = (u64, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_object_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_attach_canvas_instance_id(&mut self, body: Rid, id: u64,) {
            type CallSig = ((), Rid, u64);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_attach_canvas_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_canvas_instance_id(&self, body: Rid,) -> u64 {
            type CallSig = (u64, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_canvas_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_continuous_collision_detection_mode(&mut self, body: Rid, mode: crate::classes::physics_server_2d::CcdMode,) {
            type CallSig = ((), Rid, crate::classes::physics_server_2d::CcdMode);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_continuous_collision_detection_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_continuous_collision_detection_mode(&self, body: Rid,) -> crate::classes::physics_server_2d::CcdMode {
            type CallSig = (crate::classes::physics_server_2d::CcdMode, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_continuous_collision_detection_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (body, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_layer(&self, body: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            type CallSig = ((), Rid, u32);
            let args = (body, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_mask(&self, body: Rid,) -> u32 {
            type CallSig = (u32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_collision_priority(&mut self, body: Rid, priority: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_collision_priority(&self, body: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_param(&mut self, body: Rid, param: crate::classes::physics_server_2d::BodyParameter, value: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, crate::classes::physics_server_2d::BodyParameter, RefArg < 'a0, Variant >);
            let args = (body, param, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_param(&self, body: Rid, param: crate::classes::physics_server_2d::BodyParameter,) -> Variant {
            type CallSig = (Variant, Rid, crate::classes::physics_server_2d::BodyParameter);
            let args = (body, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_reset_mass_properties(&mut self, body: Rid,) {
            type CallSig = ((), Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_reset_mass_properties", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_state(&mut self, body: Rid, state: crate::classes::physics_server_2d::BodyState, value: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, crate::classes::physics_server_2d::BodyState, RefArg < 'a0, Variant >);
            let args = (body, state, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_state(&self, body: Rid, state: crate::classes::physics_server_2d::BodyState,) -> Variant {
            type CallSig = (Variant, Rid, crate::classes::physics_server_2d::BodyState);
            let args = (body, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector2,) {
            type CallSig = ((), Rid, Vector2);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_apply_torque_impulse(&mut self, body: Rid, impulse: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_torque_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_apply_impulse_full(&mut self, body: Rid, impulse: Vector2, position: Vector2,) {
            type CallSig = ((), Rid, Vector2, Vector2);
            let args = (body, impulse, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_apply_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_apply_impulse(&mut self, body: Rid, impulse: Vector2,) {
            self.body_apply_impulse_ex(body, impulse,) . done()
        }
        #[inline]
        pub fn body_apply_impulse_ex < 'a > (&'a mut self, body: Rid, impulse: Vector2,) -> ExBodyApplyImpulse < 'a > {
            ExBodyApplyImpulse::new(self, body, impulse,)
        }
        pub fn body_apply_central_force(&mut self, body: Rid, force: Vector2,) {
            type CallSig = ((), Rid, Vector2);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_apply_force_full(&mut self, body: Rid, force: Vector2, position: Vector2,) {
            type CallSig = ((), Rid, Vector2, Vector2);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_apply_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_apply_force(&mut self, body: Rid, force: Vector2,) {
            self.body_apply_force_ex(body, force,) . done()
        }
        #[inline]
        pub fn body_apply_force_ex < 'a > (&'a mut self, body: Rid, force: Vector2,) -> ExBodyApplyForce < 'a > {
            ExBodyApplyForce::new(self, body, force,)
        }
        pub fn body_apply_torque(&mut self, body: Rid, torque: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_add_constant_central_force(&mut self, body: Rid, force: Vector2,) {
            type CallSig = ((), Rid, Vector2);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_constant_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_add_constant_force_full(&mut self, body: Rid, force: Vector2, position: Vector2,) {
            type CallSig = ((), Rid, Vector2, Vector2);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_add_constant_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_add_constant_force(&mut self, body: Rid, force: Vector2,) {
            self.body_add_constant_force_ex(body, force,) . done()
        }
        #[inline]
        pub fn body_add_constant_force_ex < 'a > (&'a mut self, body: Rid, force: Vector2,) -> ExBodyAddConstantForce < 'a > {
            ExBodyAddConstantForce::new(self, body, force,)
        }
        pub fn body_add_constant_torque(&mut self, body: Rid, torque: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_constant_force(&mut self, body: Rid, force: Vector2,) {
            type CallSig = ((), Rid, Vector2);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_constant_force(&self, body: Rid,) -> Vector2 {
            type CallSig = (Vector2, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_constant_torque(&mut self, body: Rid, torque: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_constant_torque(&self, body: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector2,) {
            type CallSig = ((), Rid, Vector2);
            let args = (body, axis_velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_axis_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type CallSig = ((), Rid, Rid);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_remove_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (body, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_get_max_contacts_reported(&self, body: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_omit_force_integration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_is_omitting_force_integration(&self, body: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_is_omitting_force_integration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn body_set_state_sync_callback(&mut self, body: Rid, callable: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Callable >);
            let args = (body, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_state_sync_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn body_set_force_integration_callback_full(&mut self, body: Rid, callable: RefArg < Callable >, userdata: RefArg < Variant >,) {
            type CallSig < 'a0, 'a1, > = ((), Rid, RefArg < 'a0, Callable >, RefArg < 'a1, Variant >);
            let args = (body, callable, userdata,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_force_integration_callback", self.object_ptr, self.__checked_id(), args,)
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
        pub(crate) fn body_test_motion_full(&mut self, body: Rid, parameters: ObjectArg < crate::classes::PhysicsTestMotionParameters2D >, result: ObjectArg < crate::classes::PhysicsTestMotionResult2D >,) -> bool {
            type CallSig = (bool, Rid, ObjectArg < crate::classes::PhysicsTestMotionParameters2D >, ObjectArg < crate::classes::PhysicsTestMotionResult2D >);
            let args = (body, parameters, result,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_test_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::body_test_motion_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn body_test_motion(&mut self, body: Rid, parameters: impl AsObjectArg < crate::classes::PhysicsTestMotionParameters2D >,) -> bool {
            self.body_test_motion_ex(body, parameters,) . done()
        }
        #[inline]
        pub fn body_test_motion_ex < 'a > (&'a mut self, body: Rid, parameters: impl AsObjectArg < crate::classes::PhysicsTestMotionParameters2D >,) -> ExBodyTestMotion < 'a > {
            ExBodyTestMotion::new(self, body, parameters,)
        }
        pub fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::classes::PhysicsDirectBodyState2D > > {
            type CallSig = (Option < Gd < crate::classes::PhysicsDirectBodyState2D > >, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_direct_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_create(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_clear(&mut self, joint: Rid,) {
            type CallSig = ((), Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::JointParam, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_2d::JointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::JointParam,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_2d::JointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (joint, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_disable_collisions_between_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_is_disabled_collisions_between_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn joint_make_pin_full(&mut self, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,) {
            type CallSig = ((), Rid, Vector2, Rid, Rid);
            let args = (joint, anchor, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_make_pin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::joint_make_pin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn joint_make_pin(&mut self, joint: Rid, anchor: Vector2, body_a: Rid,) {
            self.joint_make_pin_ex(joint, anchor, body_a,) . done()
        }
        #[inline]
        pub fn joint_make_pin_ex < 'a > (&'a mut self, joint: Rid, anchor: Vector2, body_a: Rid,) -> ExJointMakePin < 'a > {
            ExJointMakePin::new(self, joint, anchor, body_a,)
        }
        pub(crate) fn joint_make_groove_full(&mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,) {
            type CallSig = ((), Rid, Vector2, Vector2, Vector2, Rid, Rid);
            let args = (joint, groove1_a, groove2_a, anchor_b, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_make_groove", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::joint_make_groove_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn joint_make_groove(&mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) {
            self.joint_make_groove_ex(joint, groove1_a, groove2_a, anchor_b,) . done()
        }
        #[inline]
        pub fn joint_make_groove_ex < 'a > (&'a mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) -> ExJointMakeGroove < 'a > {
            ExJointMakeGroove::new(self, joint, groove1_a, groove2_a, anchor_b,)
        }
        pub(crate) fn joint_make_damped_spring_full(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,) {
            type CallSig = ((), Rid, Vector2, Vector2, Rid, Rid);
            let args = (joint, anchor_a, anchor_b, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_make_damped_spring", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::joint_make_damped_spring_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn joint_make_damped_spring(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) {
            self.joint_make_damped_spring_ex(joint, anchor_a, anchor_b, body_a,) . done()
        }
        #[inline]
        pub fn joint_make_damped_spring_ex < 'a > (&'a mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) -> ExJointMakeDampedSpring < 'a > {
            ExJointMakeDampedSpring::new(self, joint, anchor_a, anchor_b, body_a,)
        }
        pub fn pin_joint_set_flag(&mut self, joint: Rid, flag: crate::classes::physics_server_2d::PinJointFlag, enabled: bool,) {
            type CallSig = ((), Rid, crate::classes::physics_server_2d::PinJointFlag, bool);
            let args = (joint, flag, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "pin_joint_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_flag(&self, joint: Rid, flag: crate::classes::physics_server_2d::PinJointFlag,) -> bool {
            type CallSig = (bool, Rid, crate::classes::physics_server_2d::PinJointFlag);
            let args = (joint, flag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "pin_joint_get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::PinJointParam, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_2d::PinJointParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "pin_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pin_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::PinJointParam,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_2d::PinJointParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "pin_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn damped_spring_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::DampedSpringParam, value: f32,) {
            type CallSig = ((), Rid, crate::classes::physics_server_2d::DampedSpringParam, f32);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "damped_spring_joint_set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn damped_spring_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::DampedSpringParam,) -> f32 {
            type CallSig = (f32, Rid, crate::classes::physics_server_2d::DampedSpringParam);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "damped_spring_joint_get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn joint_get_type(&self, joint: Rid,) -> crate::classes::physics_server_2d::JointType {
            type CallSig = (crate::classes::physics_server_2d::JointType, Rid);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_get_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_info(&mut self, process_info: crate::classes::physics_server_2d::ProcessInfo,) -> i32 {
            type CallSig = (i32, crate::classes::physics_server_2d::ProcessInfo);
            let args = (process_info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "get_process_info", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsServer2D {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicsServer2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsServer2D {
        
    }
    impl std::ops::Deref for PhysicsServer2D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsServer2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicsServer2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsServer2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::area_add_shape_ex`][super::PhysicsServer2D::area_add_shape_ex]."]
#[must_use]
pub struct ExAreaAddShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAreaAddShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, area: Rid, shape: Rid,) -> Self {
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, area: area, shape: shape, transform: transform, disabled: disabled,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
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
        re_export::PhysicsServer2D::area_add_shape_full(surround_object, area, shape, transform, disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_add_shape_ex`][super::PhysicsServer2D::body_add_shape_ex]."]
#[must_use]
pub struct ExBodyAddShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyAddShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, shape: Rid,) -> Self {
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, shape: shape, transform: transform, disabled: disabled,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
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
        re_export::PhysicsServer2D::body_add_shape_full(surround_object, body, shape, transform, disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_apply_impulse_ex`][super::PhysicsServer2D::body_apply_impulse_ex]."]
#[must_use]
pub struct ExBodyApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, impulse: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
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
        re_export::PhysicsServer2D::body_apply_impulse_full(surround_object, body, impulse, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_apply_force_ex`][super::PhysicsServer2D::body_apply_force_ex]."]
#[must_use]
pub struct ExBodyApplyForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
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
        re_export::PhysicsServer2D::body_apply_force_full(surround_object, body, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_add_constant_force_ex`][super::PhysicsServer2D::body_add_constant_force_ex]."]
#[must_use]
pub struct ExBodyAddConstantForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
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
        re_export::PhysicsServer2D::body_add_constant_force_full(surround_object, body, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_set_force_integration_callback_ex`][super::PhysicsServer2D::body_set_force_integration_callback_ex]."]
#[must_use]
pub struct ExBodySetForceIntegrationCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, callable: CowArg < 'a, Callable >, userdata: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodySetForceIntegrationCallback < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, callable: &'a Callable,) -> Self {
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
        re_export::PhysicsServer2D::body_set_force_integration_callback_full(surround_object, body, callable.cow_as_arg(), userdata.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_test_motion_ex`][super::PhysicsServer2D::body_test_motion_ex]."]
#[must_use]
pub struct ExBodyTestMotion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, parameters: ObjectCow < crate::classes::PhysicsTestMotionParameters2D >, result: ObjectCow < crate::classes::PhysicsTestMotionResult2D >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBodyTestMotion < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, body: Rid, parameters: impl AsObjectArg < crate::classes::PhysicsTestMotionParameters2D >,) -> Self {
        let result = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, parameters: parameters.consume_arg(), result: result.consume_arg(),
        }
    }
    #[inline]
    pub fn result(self, result: impl AsObjectArg < crate::classes::PhysicsTestMotionResult2D >) -> Self {
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
        re_export::PhysicsServer2D::body_test_motion_full(surround_object, body, parameters.cow_as_object_arg(), result.cow_as_object_arg(),)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_pin_ex`][super::PhysicsServer2D::joint_make_pin_ex]."]
#[must_use]
pub struct ExJointMakePin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExJointMakePin < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, anchor: Vector2, body_a: Rid,) -> Self {
        let body_b = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, joint: joint, anchor: anchor, body_a: body_a, body_b: body_b,
        }
    }
    #[inline]
    pub fn body_b(self, body_b: Rid) -> Self {
        Self {
            body_b: body_b, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, joint, anchor, body_a, body_b,
        }
        = self;
        re_export::PhysicsServer2D::joint_make_pin_full(surround_object, joint, anchor, body_a, body_b,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_groove_ex`][super::PhysicsServer2D::joint_make_groove_ex]."]
#[must_use]
pub struct ExJointMakeGroove < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExJointMakeGroove < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) -> Self {
        let body_a = Rid::Invalid;
        let body_b = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, joint: joint, groove1_a: groove1_a, groove2_a: groove2_a, anchor_b: anchor_b, body_a: body_a, body_b: body_b,
        }
    }
    #[inline]
    pub fn body_a(self, body_a: Rid) -> Self {
        Self {
            body_a: body_a, .. self
        }
    }
    #[inline]
    pub fn body_b(self, body_b: Rid) -> Self {
        Self {
            body_b: body_b, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, joint, groove1_a, groove2_a, anchor_b, body_a, body_b,
        }
        = self;
        re_export::PhysicsServer2D::joint_make_groove_full(surround_object, joint, groove1_a, groove2_a, anchor_b, body_a, body_b,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_damped_spring_ex`][super::PhysicsServer2D::joint_make_damped_spring_ex]."]
#[must_use]
pub struct ExJointMakeDampedSpring < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExJointMakeDampedSpring < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsServer2D, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) -> Self {
        let body_b = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, joint: joint, anchor_a: anchor_a, anchor_b: anchor_b, body_a: body_a, body_b: body_b,
        }
    }
    #[inline]
    pub fn body_b(self, body_b: Rid) -> Self {
        Self {
            body_b: body_b, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, joint, anchor_a, anchor_b, body_a, body_b,
        }
        = self;
        re_export::PhysicsServer2D::joint_make_damped_spring_full(surround_object, joint, anchor_a, anchor_b, body_a, body_b,)
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
    #[doc(alias = "SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS`"]
    pub const CONSTRAINT_DEFAULT_BIAS: SpaceParameter = SpaceParameter {
        ord: 7i32
    };
    #[doc(alias = "SPACE_PARAM_SOLVER_ITERATIONS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_SOLVER_ITERATIONS`"]
    pub const SOLVER_ITERATIONS: SpaceParameter = SpaceParameter {
        ord: 8i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::CONTACT_RECYCLE_RADIUS => "CONTACT_RECYCLE_RADIUS", Self::CONTACT_MAX_SEPARATION => "CONTACT_MAX_SEPARATION", Self::CONTACT_MAX_ALLOWED_PENETRATION => "CONTACT_MAX_ALLOWED_PENETRATION", Self::CONTACT_DEFAULT_BIAS => "CONTACT_DEFAULT_BIAS", Self::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD => "BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD => "BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_TIME_TO_SLEEP => "BODY_TIME_TO_SLEEP", Self::CONSTRAINT_DEFAULT_BIAS => "CONSTRAINT_DEFAULT_BIAS", Self::SOLVER_ITERATIONS => "SOLVER_ITERATIONS", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CONTACT_RECYCLE_RADIUS => "SPACE_PARAM_CONTACT_RECYCLE_RADIUS", Self::CONTACT_MAX_SEPARATION => "SPACE_PARAM_CONTACT_MAX_SEPARATION", Self::CONTACT_MAX_ALLOWED_PENETRATION => "SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION", Self::CONTACT_DEFAULT_BIAS => "SPACE_PARAM_CONTACT_DEFAULT_BIAS", Self::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD => "SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD => "SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_TIME_TO_SLEEP => "SPACE_PARAM_BODY_TIME_TO_SLEEP", Self::CONSTRAINT_DEFAULT_BIAS => "SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS", Self::SOLVER_ITERATIONS => "SPACE_PARAM_SOLVER_ITERATIONS", _ => self.as_str(),
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
    #[doc(alias = "SHAPE_SEGMENT")]
    #[doc = "Godot enumerator name: `SHAPE_SEGMENT`"]
    pub const SEGMENT: ShapeType = ShapeType {
        ord: 2i32
    };
    #[doc(alias = "SHAPE_CIRCLE")]
    #[doc = "Godot enumerator name: `SHAPE_CIRCLE`"]
    pub const CIRCLE: ShapeType = ShapeType {
        ord: 3i32
    };
    #[doc(alias = "SHAPE_RECTANGLE")]
    #[doc = "Godot enumerator name: `SHAPE_RECTANGLE`"]
    pub const RECTANGLE: ShapeType = ShapeType {
        ord: 4i32
    };
    #[doc(alias = "SHAPE_CAPSULE")]
    #[doc = "Godot enumerator name: `SHAPE_CAPSULE`"]
    pub const CAPSULE: ShapeType = ShapeType {
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
    #[doc(alias = "SHAPE_CUSTOM")]
    #[doc = "Godot enumerator name: `SHAPE_CUSTOM`"]
    pub const CUSTOM: ShapeType = ShapeType {
        ord: 8i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::WORLD_BOUNDARY => "WORLD_BOUNDARY", Self::SEPARATION_RAY => "SEPARATION_RAY", Self::SEGMENT => "SEGMENT", Self::CIRCLE => "CIRCLE", Self::RECTANGLE => "RECTANGLE", Self::CAPSULE => "CAPSULE", Self::CONVEX_POLYGON => "CONVEX_POLYGON", Self::CONCAVE_POLYGON => "CONCAVE_POLYGON", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::WORLD_BOUNDARY => "SHAPE_WORLD_BOUNDARY", Self::SEPARATION_RAY => "SHAPE_SEPARATION_RAY", Self::SEGMENT => "SHAPE_SEGMENT", Self::CIRCLE => "SHAPE_CIRCLE", Self::RECTANGLE => "SHAPE_RECTANGLE", Self::CAPSULE => "SHAPE_CAPSULE", Self::CONVEX_POLYGON => "SHAPE_CONVEX_POLYGON", Self::CONCAVE_POLYGON => "SHAPE_CONCAVE_POLYGON", Self::CUSTOM => "SHAPE_CUSTOM", _ => self.as_str(),
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::GRAVITY_OVERRIDE_MODE => "GRAVITY_OVERRIDE_MODE", Self::GRAVITY => "GRAVITY", Self::GRAVITY_VECTOR => "GRAVITY_VECTOR", Self::GRAVITY_IS_POINT => "GRAVITY_IS_POINT", Self::GRAVITY_POINT_UNIT_DISTANCE => "GRAVITY_POINT_UNIT_DISTANCE", Self::LINEAR_DAMP_OVERRIDE_MODE => "LINEAR_DAMP_OVERRIDE_MODE", Self::LINEAR_DAMP => "LINEAR_DAMP", Self::ANGULAR_DAMP_OVERRIDE_MODE => "ANGULAR_DAMP_OVERRIDE_MODE", Self::ANGULAR_DAMP => "ANGULAR_DAMP", Self::PRIORITY => "PRIORITY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GRAVITY_OVERRIDE_MODE => "AREA_PARAM_GRAVITY_OVERRIDE_MODE", Self::GRAVITY => "AREA_PARAM_GRAVITY", Self::GRAVITY_VECTOR => "AREA_PARAM_GRAVITY_VECTOR", Self::GRAVITY_IS_POINT => "AREA_PARAM_GRAVITY_IS_POINT", Self::GRAVITY_POINT_UNIT_DISTANCE => "AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE", Self::LINEAR_DAMP_OVERRIDE_MODE => "AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE", Self::LINEAR_DAMP => "AREA_PARAM_LINEAR_DAMP", Self::ANGULAR_DAMP_OVERRIDE_MODE => "AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE", Self::ANGULAR_DAMP => "AREA_PARAM_ANGULAR_DAMP", Self::PRIORITY => "AREA_PARAM_PRIORITY", _ => self.as_str(),
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
pub struct JointType {
    ord: i32
}
impl JointType {
    #[doc(alias = "JOINT_TYPE_PIN")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_PIN`"]
    pub const PIN: JointType = JointType {
        ord: 0i32
    };
    #[doc(alias = "JOINT_TYPE_GROOVE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_GROOVE`"]
    pub const GROOVE: JointType = JointType {
        ord: 1i32
    };
    #[doc(alias = "JOINT_TYPE_DAMPED_SPRING")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_DAMPED_SPRING`"]
    pub const DAMPED_SPRING: JointType = JointType {
        ord: 2i32
    };
    #[doc(alias = "JOINT_TYPE_MAX")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_MAX`"]
    pub const MAX: JointType = JointType {
        ord: 3i32
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
            Self::PIN => "PIN", Self::GROOVE => "GROOVE", Self::DAMPED_SPRING => "DAMPED_SPRING", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PIN => "JOINT_TYPE_PIN", Self::GROOVE => "JOINT_TYPE_GROOVE", Self::DAMPED_SPRING => "JOINT_TYPE_DAMPED_SPRING", Self::MAX => "JOINT_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for JointType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
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
pub struct JointParam {
    ord: i32
}
impl JointParam {
    #[doc(alias = "JOINT_PARAM_BIAS")]
    #[doc = "Godot enumerator name: `JOINT_PARAM_BIAS`"]
    pub const BIAS: JointParam = JointParam {
        ord: 0i32
    };
    #[doc(alias = "JOINT_PARAM_MAX_BIAS")]
    #[doc = "Godot enumerator name: `JOINT_PARAM_MAX_BIAS`"]
    pub const MAX_BIAS: JointParam = JointParam {
        ord: 1i32
    };
    #[doc(alias = "JOINT_PARAM_MAX_FORCE")]
    #[doc = "Godot enumerator name: `JOINT_PARAM_MAX_FORCE`"]
    pub const MAX_FORCE: JointParam = JointParam {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for JointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("JointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for JointParam {
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
            Self::BIAS => "BIAS", Self::MAX_BIAS => "MAX_BIAS", Self::MAX_FORCE => "MAX_FORCE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BIAS => "JOINT_PARAM_BIAS", Self::MAX_BIAS => "JOINT_PARAM_MAX_BIAS", Self::MAX_FORCE => "JOINT_PARAM_MAX_FORCE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for JointParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for JointParam {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JointParam {
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
    #[doc(alias = "PIN_JOINT_SOFTNESS")]
    #[doc = "Godot enumerator name: `PIN_JOINT_SOFTNESS`"]
    pub const SOFTNESS: PinJointParam = PinJointParam {
        ord: 0i32
    };
    #[doc(alias = "PIN_JOINT_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `PIN_JOINT_LIMIT_UPPER`"]
    pub const LIMIT_UPPER: PinJointParam = PinJointParam {
        ord: 1i32
    };
    #[doc(alias = "PIN_JOINT_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `PIN_JOINT_LIMIT_LOWER`"]
    pub const LIMIT_LOWER: PinJointParam = PinJointParam {
        ord: 2i32
    };
    #[doc(alias = "PIN_JOINT_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `PIN_JOINT_MOTOR_TARGET_VELOCITY`"]
    pub const MOTOR_TARGET_VELOCITY: PinJointParam = PinJointParam {
        ord: 3i32
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
            Self::SOFTNESS => "SOFTNESS", Self::LIMIT_UPPER => "LIMIT_UPPER", Self::LIMIT_LOWER => "LIMIT_LOWER", Self::MOTOR_TARGET_VELOCITY => "MOTOR_TARGET_VELOCITY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SOFTNESS => "PIN_JOINT_SOFTNESS", Self::LIMIT_UPPER => "PIN_JOINT_LIMIT_UPPER", Self::LIMIT_LOWER => "PIN_JOINT_LIMIT_LOWER", Self::MOTOR_TARGET_VELOCITY => "PIN_JOINT_MOTOR_TARGET_VELOCITY", _ => self.as_str(),
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
pub struct PinJointFlag {
    ord: i32
}
impl PinJointFlag {
    #[doc(alias = "PIN_JOINT_FLAG_ANGULAR_LIMIT_ENABLED")]
    #[doc = "Godot enumerator name: `PIN_JOINT_FLAG_ANGULAR_LIMIT_ENABLED`"]
    pub const ANGULAR_LIMIT_ENABLED: PinJointFlag = PinJointFlag {
        ord: 0i32
    };
    #[doc(alias = "PIN_JOINT_FLAG_MOTOR_ENABLED")]
    #[doc = "Godot enumerator name: `PIN_JOINT_FLAG_MOTOR_ENABLED`"]
    pub const MOTOR_ENABLED: PinJointFlag = PinJointFlag {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for PinJointFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PinJointFlag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PinJointFlag {
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
            Self::ANGULAR_LIMIT_ENABLED => "ANGULAR_LIMIT_ENABLED", Self::MOTOR_ENABLED => "MOTOR_ENABLED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ANGULAR_LIMIT_ENABLED => "PIN_JOINT_FLAG_ANGULAR_LIMIT_ENABLED", Self::MOTOR_ENABLED => "PIN_JOINT_FLAG_MOTOR_ENABLED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PinJointFlag {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PinJointFlag {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PinJointFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DampedSpringParam {
    ord: i32
}
impl DampedSpringParam {
    #[doc(alias = "DAMPED_SPRING_REST_LENGTH")]
    #[doc = "Godot enumerator name: `DAMPED_SPRING_REST_LENGTH`"]
    pub const REST_LENGTH: DampedSpringParam = DampedSpringParam {
        ord: 0i32
    };
    #[doc(alias = "DAMPED_SPRING_STIFFNESS")]
    #[doc = "Godot enumerator name: `DAMPED_SPRING_STIFFNESS`"]
    pub const STIFFNESS: DampedSpringParam = DampedSpringParam {
        ord: 1i32
    };
    #[doc(alias = "DAMPED_SPRING_DAMPING")]
    #[doc = "Godot enumerator name: `DAMPED_SPRING_DAMPING`"]
    pub const DAMPING: DampedSpringParam = DampedSpringParam {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DampedSpringParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DampedSpringParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DampedSpringParam {
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
            Self::REST_LENGTH => "REST_LENGTH", Self::STIFFNESS => "STIFFNESS", Self::DAMPING => "DAMPING", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::REST_LENGTH => "DAMPED_SPRING_REST_LENGTH", Self::STIFFNESS => "DAMPED_SPRING_STIFFNESS", Self::DAMPING => "DAMPED_SPRING_DAMPING", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DampedSpringParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DampedSpringParam {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DampedSpringParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `CCDMode`."]
pub struct CcdMode {
    ord: i32
}
impl CcdMode {
    #[doc(alias = "CCD_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `CCD_MODE_DISABLED`"]
    pub const DISABLED: CcdMode = CcdMode {
        ord: 0i32
    };
    #[doc(alias = "CCD_MODE_CAST_RAY")]
    #[doc = "Godot enumerator name: `CCD_MODE_CAST_RAY`"]
    pub const CAST_RAY: CcdMode = CcdMode {
        ord: 1i32
    };
    #[doc(alias = "CCD_MODE_CAST_SHAPE")]
    #[doc = "Godot enumerator name: `CCD_MODE_CAST_SHAPE`"]
    pub const CAST_SHAPE: CcdMode = CcdMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CcdMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CcdMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CcdMode {
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
            Self::DISABLED => "DISABLED", Self::CAST_RAY => "CAST_RAY", Self::CAST_SHAPE => "CAST_SHAPE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "CCD_MODE_DISABLED", Self::CAST_RAY => "CCD_MODE_CAST_RAY", Self::CAST_SHAPE => "CCD_MODE_CAST_SHAPE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CcdMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CcdMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CcdMode {
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