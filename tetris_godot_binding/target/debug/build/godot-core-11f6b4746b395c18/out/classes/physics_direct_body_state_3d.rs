#![doc = "Sidecar module for class [`PhysicsDirectBodyState3D`][crate::classes::PhysicsDirectBodyState3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectBodyState3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectBodyState3D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_direct_body_state_3d`][crate::classes::physics_direct_body_state_3d]: sidecar module with related enum/flag types\n* [`IPhysicsDirectBodyState3D`][crate::classes::IPhysicsDirectBodyState3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsDirectBodyState3D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsDirectBodyState3D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectBodyState3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsDirectBodyState3D`][crate::classes::PhysicsDirectBodyState3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsDirectBodyState3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsDirectBodyState3D: crate::obj::GodotClass < Base = PhysicsDirectBodyState3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsDirectBodyState3D {
        pub fn get_total_gravity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_total_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_linear_damp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_total_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_angular_damp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_total_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_center_of_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass_local(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_center_of_mass_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_principal_inertia_axes(&self,) -> Basis {
            type CallSig = (Basis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_principal_inertia_axes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inverse_mass(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_inverse_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inverse_inertia(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_inverse_inertia", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inverse_inertia_tensor(&self,) -> Basis {
            type CallSig = (Basis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_inverse_inertia_tensor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform3D,) {
            type CallSig = ((), Transform3D);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_at_local_position(&self, local_position: Vector3,) -> Vector3 {
            type CallSig = (Vector3, Vector3);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_velocity_at_local_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn apply_central_impulse_full(&mut self, impulse: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_central_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_central_impulse(&mut self,) {
            self.apply_central_impulse_ex() . done()
        }
        #[inline]
        pub fn apply_central_impulse_ex < 'a > (&'a mut self,) -> ExApplyCentralImpulse < 'a > {
            ExApplyCentralImpulse::new(self,)
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector3, position: Vector3,) {
            type CallSig = ((), Vector3, Vector3);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_impulse", self.object_ptr, self.__checked_id(), args,)
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
                let method_bind = sys::class_servers_api() . fptr_by_index(163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_torque_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn apply_central_force_full(&mut self, force: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_central_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_central_force(&mut self,) {
            self.apply_central_force_ex() . done()
        }
        #[inline]
        pub fn apply_central_force_ex < 'a > (&'a mut self,) -> ExApplyCentralForce < 'a > {
            ExApplyCentralForce::new(self,)
        }
        pub(crate) fn apply_force_full(&mut self, force: Vector3, position: Vector3,) {
            type CallSig = ((), Vector3, Vector3);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_force", self.object_ptr, self.__checked_id(), args,)
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
                let method_bind = sys::class_servers_api() . fptr_by_index(166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_constant_central_force_full(&mut self, force: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "add_constant_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_constant_central_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_constant_central_force(&mut self,) {
            self.add_constant_central_force_ex() . done()
        }
        #[inline]
        pub fn add_constant_central_force_ex < 'a > (&'a mut self,) -> ExAddConstantCentralForce < 'a > {
            ExAddConstantCentralForce::new(self,)
        }
        pub(crate) fn add_constant_force_full(&mut self, force: Vector3, position: Vector3,) {
            type CallSig = ((), Vector3, Vector3);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "add_constant_force", self.object_ptr, self.__checked_id(), args,)
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
                let method_bind = sys::class_servers_api() . fptr_by_index(169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "add_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_force(&mut self, force: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_force(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_torque(&mut self, torque: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_torque(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sleep_state(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_sleep_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "is_sleeping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_local_position(&self, contact_idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_local_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_local_normal(&self, contact_idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_local_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_impulse(&self, contact_idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_local_shape(&self, contact_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_local_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_local_velocity_at_position(&self, contact_idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_local_velocity_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider(&self, contact_idx: i32,) -> Rid {
            type CallSig = (Rid, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_position(&self, contact_idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_id(&self, contact_idx: i32,) -> u64 {
            type CallSig = (u64, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_object(&self, contact_idx: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallSig = (Option < Gd < crate::classes::Object > >, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_shape(&self, contact_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_velocity_at_position(&self, contact_idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_velocity_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_step(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn integrate_forces(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "integrate_forces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_space_state(&mut self,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState3D > > {
            type CallSig = (Option < Gd < crate::classes::PhysicsDirectSpaceState3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_space_state", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectBodyState3D {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicsDirectBodyState3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectBodyState3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsDirectBodyState3D {
        
    }
    impl std::ops::Deref for PhysicsDirectBodyState3D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectBodyState3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsDirectBodyState3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicsDirectBodyState3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsDirectBodyState3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::apply_central_impulse_ex`][super::PhysicsDirectBodyState3D::apply_central_impulse_ex]."]
#[must_use]
pub struct ExApplyCentralImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, impulse: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyCentralImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D,) -> Self {
        let impulse = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse,
        }
    }
    #[inline]
    pub fn impulse(self, impulse: Vector3) -> Self {
        Self {
            impulse: impulse, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse,
        }
        = self;
        re_export::PhysicsDirectBodyState3D::apply_central_impulse_full(surround_object, impulse,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::apply_impulse_ex`][super::PhysicsDirectBodyState3D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D, impulse: Vector3,) -> Self {
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
        re_export::PhysicsDirectBodyState3D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::apply_central_force_ex`][super::PhysicsDirectBodyState3D::apply_central_force_ex]."]
#[must_use]
pub struct ExApplyCentralForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyCentralForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D,) -> Self {
        let force = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: Vector3) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::PhysicsDirectBodyState3D::apply_central_force_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::apply_force_ex`][super::PhysicsDirectBodyState3D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3,) -> Self {
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
        re_export::PhysicsDirectBodyState3D::apply_force_full(surround_object, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::add_constant_central_force_ex`][super::PhysicsDirectBodyState3D::add_constant_central_force_ex]."]
#[must_use]
pub struct ExAddConstantCentralForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantCentralForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D,) -> Self {
        let force = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: Vector3) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::PhysicsDirectBodyState3D::add_constant_central_force_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::add_constant_force_ex`][super::PhysicsDirectBodyState3D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3,) -> Self {
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
        re_export::PhysicsDirectBodyState3D::add_constant_force_full(surround_object, force, position,)
    }
}