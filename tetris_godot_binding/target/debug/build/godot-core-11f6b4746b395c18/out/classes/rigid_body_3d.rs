#![doc = "Sidecar module for class [`RigidBody3D`][crate::classes::RigidBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RigidBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_rigidbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RigidBody3D.`\n\nInherits [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nRelated symbols:\n\n* [`rigid_body_3d`][crate::classes::rigid_body_3d]: sidecar module with related enum/flag types\n* [`IRigidBody3D`][crate::classes::IRigidBody3D]: virtual methods\n\n\nSee also [Godot docs for `RigidBody3D`](https://docs.godotengine.org/en/stable/classes/class_rigidbody3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`RigidBody3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RigidBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RigidBody3D`][crate::classes::RigidBody3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RigidBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_rigidbody3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRigidBody3D: crate::obj::GodotClass < Base = RigidBody3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn integrate_forces(&mut self, state: Option < Gd < crate::classes::PhysicsDirectBodyState3D > >,) {
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
    impl RigidBody3D {
        pub fn set_mass(&mut self, mass: f32,) {
            type CallSig = ((), f32);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mass(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inertia(&mut self, inertia: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (inertia,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_inertia", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inertia(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_inertia", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_of_mass_mode(&mut self, mode: crate::classes::rigid_body_3d::CenterOfMassMode,) {
            type CallSig = ((), crate::classes::rigid_body_3d::CenterOfMassMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_center_of_mass_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass_mode(&self,) -> crate::classes::rigid_body_3d::CenterOfMassMode {
            type CallSig = (crate::classes::rigid_body_3d::CenterOfMassMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_center_of_mass_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_of_mass(&mut self, center_of_mass: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (center_of_mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_center_of_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_center_of_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_material_override(&mut self, physics_material_override: impl AsObjectArg < crate::classes::PhysicsMaterial >,) {
            type CallSig = ((), ObjectArg < crate::classes::PhysicsMaterial >);
            let args = (physics_material_override.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_physics_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_material_override(&self,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallSig = (Option < Gd < crate::classes::PhysicsMaterial > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_physics_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, linear_velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, angular_velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inverse_inertia_tensor(&self,) -> Basis {
            type CallSig = (Basis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_inverse_inertia_tensor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_scale(&mut self, gravity_scale: f32,) {
            type CallSig = ((), f32);
            let args = (gravity_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_gravity_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_gravity_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp_mode(&mut self, linear_damp_mode: crate::classes::rigid_body_3d::DampMode,) {
            type CallSig = ((), crate::classes::rigid_body_3d::DampMode);
            let args = (linear_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_linear_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp_mode(&self,) -> crate::classes::rigid_body_3d::DampMode {
            type CallSig = (crate::classes::rigid_body_3d::DampMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_linear_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp_mode(&mut self, angular_damp_mode: crate::classes::rigid_body_3d::DampMode,) {
            type CallSig = ((), crate::classes::rigid_body_3d::DampMode);
            let args = (angular_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_angular_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp_mode(&self,) -> crate::classes::rigid_body_3d::DampMode {
            type CallSig = (crate::classes::rigid_body_3d::DampMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_angular_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallSig = ((), f32);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallSig = ((), f32);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_contacts_reported(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_contacts_reported(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_contact_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_custom_integrator(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_use_custom_integrator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_custom_integrator(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "is_using_custom_integrator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_contact_monitor(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_contact_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_contact_monitor_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "is_contact_monitor_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_continuous_collision_detection(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_use_continuous_collision_detection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_continuous_collision_detection(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "is_using_continuous_collision_detection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_axis_velocity(&mut self, axis_velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (axis_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_axis_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_central_impulse(&mut self, impulse: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector3, position: Vector3,) {
            type CallSig = ((), Vector3, Vector3);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "apply_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector3,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[inline]
        pub fn apply_impulse_ex < 'a > (&'a mut self, impulse: Vector3,) -> ExApplyImpulse < 'a > {
            ExApplyImpulse::new(self, impulse,)
        }
        pub fn apply_torque_impulse(&mut self, impulse: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "apply_torque_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_central_force(&mut self, force: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "apply_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn apply_force_full(&mut self, force: Vector3, position: Vector3,) {
            type CallSig = ((), Vector3, Vector3);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "apply_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_force(&mut self, force: Vector3,) {
            self.apply_force_ex(force,) . done()
        }
        #[inline]
        pub fn apply_force_ex < 'a > (&'a mut self, force: Vector3,) -> ExApplyForce < 'a > {
            ExApplyForce::new(self, force,)
        }
        pub fn apply_torque(&mut self, torque: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "apply_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_constant_central_force(&mut self, force: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "add_constant_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_constant_force_full(&mut self, force: Vector3, position: Vector3,) {
            type CallSig = ((), Vector3, Vector3);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "add_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_constant_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_constant_force(&mut self, force: Vector3,) {
            self.add_constant_force_ex(force,) . done()
        }
        #[inline]
        pub fn add_constant_force_ex < 'a > (&'a mut self, force: Vector3,) -> ExAddConstantForce < 'a > {
            ExAddConstantForce::new(self, force,)
        }
        pub fn add_constant_torque(&mut self, torque: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "add_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_force(&mut self, force: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_force(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_torque(&mut self, torque: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_torque(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sleeping(&mut self, sleeping: bool,) {
            type CallSig = ((), bool);
            let args = (sleeping,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_sleeping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "is_sleeping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_can_sleep(&mut self, able_to_sleep: bool,) {
            type CallSig = ((), bool);
            let args = (able_to_sleep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_can_sleep", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_able_to_sleep(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "is_able_to_sleep", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lock_rotation_enabled(&mut self, lock_rotation: bool,) {
            type CallSig = ((), bool);
            let args = (lock_rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_lock_rotation_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_lock_rotation_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "is_lock_rotation_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_freeze_enabled(&mut self, freeze_mode: bool,) {
            type CallSig = ((), bool);
            let args = (freeze_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_freeze_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_freeze_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "is_freeze_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_freeze_mode(&mut self, freeze_mode: crate::classes::rigid_body_3d::FreezeMode,) {
            type CallSig = ((), crate::classes::rigid_body_3d::FreezeMode);
            let args = (freeze_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "set_freeze_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_freeze_mode(&self,) -> crate::classes::rigid_body_3d::FreezeMode {
            type CallSig = (crate::classes::rigid_body_3d::FreezeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_freeze_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_colliding_bodies(&self,) -> Array < Gd < crate::classes::Node3D > > {
            type CallSig = (Array < Gd < crate::classes::Node3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RigidBody3D", "get_colliding_bodies", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RigidBody3D {
        type Base = crate::classes::PhysicsBody3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"RigidBody3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RigidBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody3D > for RigidBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for RigidBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for RigidBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for RigidBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RigidBody3D {
        
    }
    impl crate::obj::cap::GodotDefault for RigidBody3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RigidBody3D {
        type Target = crate::classes::PhysicsBody3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RigidBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RigidBody3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_RigidBody3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RigidBody3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsBody3D > for $Class {
                
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
#[doc = "Default-param extender for [`RigidBody3D::apply_impulse_ex`][super::RigidBody3D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RigidBody3D, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody3D, impulse: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
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
            _phantom, surround_object, impulse, position,
        }
        = self;
        re_export::RigidBody3D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[doc = "Default-param extender for [`RigidBody3D::apply_force_ex`][super::RigidBody3D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RigidBody3D, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody3D, force: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
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
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::RigidBody3D::apply_force_full(surround_object, force, position,)
    }
}
#[doc = "Default-param extender for [`RigidBody3D::add_constant_force_ex`][super::RigidBody3D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RigidBody3D, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody3D, force: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
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
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::RigidBody3D::add_constant_force_full(surround_object, force, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FreezeMode {
    ord: i32
}
impl FreezeMode {
    #[doc(alias = "FREEZE_MODE_STATIC")]
    #[doc = "Godot enumerator name: `FREEZE_MODE_STATIC`"]
    pub const STATIC: FreezeMode = FreezeMode {
        ord: 0i32
    };
    #[doc(alias = "FREEZE_MODE_KINEMATIC")]
    #[doc = "Godot enumerator name: `FREEZE_MODE_KINEMATIC`"]
    pub const KINEMATIC: FreezeMode = FreezeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for FreezeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FreezeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FreezeMode {
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
            Self::STATIC => "STATIC", Self::KINEMATIC => "KINEMATIC", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STATIC => "FREEZE_MODE_STATIC", Self::KINEMATIC => "FREEZE_MODE_KINEMATIC", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FreezeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FreezeMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FreezeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CenterOfMassMode {
    ord: i32
}
impl CenterOfMassMode {
    #[doc(alias = "CENTER_OF_MASS_MODE_AUTO")]
    #[doc = "Godot enumerator name: `CENTER_OF_MASS_MODE_AUTO`"]
    pub const AUTO: CenterOfMassMode = CenterOfMassMode {
        ord: 0i32
    };
    #[doc(alias = "CENTER_OF_MASS_MODE_CUSTOM")]
    #[doc = "Godot enumerator name: `CENTER_OF_MASS_MODE_CUSTOM`"]
    pub const CUSTOM: CenterOfMassMode = CenterOfMassMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for CenterOfMassMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CenterOfMassMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CenterOfMassMode {
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
            Self::AUTO => "AUTO", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::AUTO => "CENTER_OF_MASS_MODE_AUTO", Self::CUSTOM => "CENTER_OF_MASS_MODE_CUSTOM", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CenterOfMassMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CenterOfMassMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CenterOfMassMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DampMode {
    ord: i32
}
impl DampMode {
    #[doc(alias = "DAMP_MODE_COMBINE")]
    #[doc = "Godot enumerator name: `DAMP_MODE_COMBINE`"]
    pub const COMBINE: DampMode = DampMode {
        ord: 0i32
    };
    #[doc(alias = "DAMP_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `DAMP_MODE_REPLACE`"]
    pub const REPLACE: DampMode = DampMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DampMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DampMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DampMode {
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
            Self::COMBINE => "DAMP_MODE_COMBINE", Self::REPLACE => "DAMP_MODE_REPLACE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DampMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DampMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DampMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}