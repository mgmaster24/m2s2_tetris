#![doc = "Sidecar module for class [`Area3D`][crate::classes::Area3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Area3D` enums](https://docs.godotengine.org/en/stable/classes/class_area3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Area3D.`\n\nInherits [`CollisionObject3D`][crate::classes::CollisionObject3D].\n\nRelated symbols:\n\n* [`area_3d`][crate::classes::area_3d]: sidecar module with related enum/flag types\n* [`IArea3D`][crate::classes::IArea3D]: virtual methods\n\n\nSee also [Godot docs for `Area3D`](https://docs.godotengine.org/en/stable/classes/class_area3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Area3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Area3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Area3D`][crate::classes::Area3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Area3D` methods](https://docs.godotengine.org/en/stable/classes/class_area3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArea3D: crate::obj::GodotClass < Base = Area3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn input_event(&mut self, camera: Option < Gd < crate::classes::Camera3D > >, event: Option < Gd < crate::classes::InputEvent > >, event_position: Vector3, normal: Vector3, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
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
    impl Area3D {
        pub fn set_gravity_space_override_mode(&mut self, space_override_mode: crate::classes::area_3d::SpaceOverride,) {
            type CallSig = ((), crate::classes::area_3d::SpaceOverride);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_gravity_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_space_override_mode(&self,) -> crate::classes::area_3d::SpaceOverride {
            type CallSig = (crate::classes::area_3d::SpaceOverride,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_gravity_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_is_point(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_gravity_is_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_gravity_a_point(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "is_gravity_a_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_point_unit_distance(&mut self, distance_scale: f32,) {
            type CallSig = ((), f32);
            let args = (distance_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_gravity_point_unit_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_point_unit_distance(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_gravity_point_unit_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_point_center(&mut self, center: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (center,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_gravity_point_center", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_point_center(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_gravity_point_center", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_direction(&mut self, direction: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_gravity_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_direction(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_gravity_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, gravity: f32,) {
            type CallSig = ((), f32);
            let args = (gravity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp_space_override_mode(&mut self, space_override_mode: crate::classes::area_3d::SpaceOverride,) {
            type CallSig = ((), crate::classes::area_3d::SpaceOverride);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_linear_damp_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp_space_override_mode(&self,) -> crate::classes::area_3d::SpaceOverride {
            type CallSig = (crate::classes::area_3d::SpaceOverride,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_linear_damp_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp_space_override_mode(&mut self, space_override_mode: crate::classes::area_3d::SpaceOverride,) {
            type CallSig = ((), crate::classes::area_3d::SpaceOverride);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_angular_damp_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp_space_override_mode(&self,) -> crate::classes::area_3d::SpaceOverride {
            type CallSig = (crate::classes::area_3d::SpaceOverride,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_angular_damp_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallSig = ((), f32);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallSig = ((), f32);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_priority(&mut self, priority: i32,) {
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_priority(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wind_force_magnitude(&mut self, wind_force_magnitude: f32,) {
            type CallSig = ((), f32);
            let args = (wind_force_magnitude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_wind_force_magnitude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wind_force_magnitude(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_wind_force_magnitude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wind_attenuation_factor(&mut self, wind_attenuation_factor: f32,) {
            type CallSig = ((), f32);
            let args = (wind_attenuation_factor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_wind_attenuation_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wind_attenuation_factor(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_wind_attenuation_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wind_source_path(&mut self, wind_source_path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (wind_source_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_wind_source_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wind_source_path(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_wind_source_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_monitorable(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_monitorable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_monitorable(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "is_monitorable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_monitoring(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_monitoring", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_monitoring(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "is_monitoring", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_overlapping_bodies(&self,) -> Array < Gd < crate::classes::Node3D > > {
            type CallSig = (Array < Gd < crate::classes::Node3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_overlapping_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_overlapping_areas(&self,) -> Array < Gd < crate::classes::Area3D > > {
            type CallSig = (Array < Gd < crate::classes::Area3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_overlapping_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_overlapping_bodies(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "has_overlapping_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_overlapping_areas(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "has_overlapping_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn overlaps_body(&self, body: impl AsObjectArg < crate::classes::Node >,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::Node >);
            let args = (body.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "overlaps_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn overlaps_area(&self, area: impl AsObjectArg < crate::classes::Node >,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::Node >);
            let args = (area.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "overlaps_area", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_audio_bus_override(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_audio_bus_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_overriding_audio_bus(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "is_overriding_audio_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_audio_bus_name(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_audio_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_audio_bus_name(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_audio_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_reverb_bus(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_use_reverb_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_reverb_bus(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "is_using_reverb_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reverb_bus_name(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_reverb_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reverb_bus_name(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_reverb_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reverb_amount(&mut self, amount: f32,) {
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_reverb_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reverb_amount(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_reverb_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reverb_uniformity(&mut self, amount: f32,) {
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "set_reverb_uniformity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reverb_uniformity(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Area3D", "get_reverb_uniformity", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Area3D {
        type Base = crate::classes::CollisionObject3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Area3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Area3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for Area3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Area3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Area3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Area3D {
        
    }
    impl crate::obj::cap::GodotDefault for Area3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Area3D {
        type Target = crate::classes::CollisionObject3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Area3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Area3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Area3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Area3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CollisionObject3D > for $Class {
                
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
pub struct SpaceOverride {
    ord: i32
}
impl SpaceOverride {
    #[doc(alias = "SPACE_OVERRIDE_DISABLED")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_DISABLED`"]
    pub const DISABLED: SpaceOverride = SpaceOverride {
        ord: 0i32
    };
    #[doc(alias = "SPACE_OVERRIDE_COMBINE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_COMBINE`"]
    pub const COMBINE: SpaceOverride = SpaceOverride {
        ord: 1i32
    };
    #[doc(alias = "SPACE_OVERRIDE_COMBINE_REPLACE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_COMBINE_REPLACE`"]
    pub const COMBINE_REPLACE: SpaceOverride = SpaceOverride {
        ord: 2i32
    };
    #[doc(alias = "SPACE_OVERRIDE_REPLACE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_REPLACE`"]
    pub const REPLACE: SpaceOverride = SpaceOverride {
        ord: 3i32
    };
    #[doc(alias = "SPACE_OVERRIDE_REPLACE_COMBINE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_REPLACE_COMBINE`"]
    pub const REPLACE_COMBINE: SpaceOverride = SpaceOverride {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SpaceOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpaceOverride") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpaceOverride {
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
            Self::DISABLED => "SPACE_OVERRIDE_DISABLED", Self::COMBINE => "SPACE_OVERRIDE_COMBINE", Self::COMBINE_REPLACE => "SPACE_OVERRIDE_COMBINE_REPLACE", Self::REPLACE => "SPACE_OVERRIDE_REPLACE", Self::REPLACE_COMBINE => "SPACE_OVERRIDE_REPLACE_COMBINE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SpaceOverride {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpaceOverride {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpaceOverride {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}