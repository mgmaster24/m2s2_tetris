#![doc = "Sidecar module for class [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsBody3D.`\n\nInherits [`CollisionObject3D`][crate::classes::CollisionObject3D].\n\nRelated symbols:\n\n* [`physics_body_3d`][crate::classes::physics_body_3d]: sidecar module with related enum/flag types\n* [`IPhysicsBody3D`][crate::classes::IPhysicsBody3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsBody3D`](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsBody3D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsBody3D: crate::obj::GodotClass < Base = PhysicsBody3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsBody3D {
        pub(crate) fn move_and_collide_full(&mut self, motion: Vector3, test_only: bool, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,) -> Option < Gd < crate::classes::KinematicCollision3D > > {
            type CallSig = (Option < Gd < crate::classes::KinematicCollision3D > >, Vector3, bool, f32, bool, i32);
            let args = (motion, test_only, safe_margin, recovery_as_collision, max_collisions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "move_and_collide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::move_and_collide_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn move_and_collide(&mut self, motion: Vector3,) -> Option < Gd < crate::classes::KinematicCollision3D > > {
            self.move_and_collide_ex(motion,) . done()
        }
        #[inline]
        pub fn move_and_collide_ex < 'a > (&'a mut self, motion: Vector3,) -> ExMoveAndCollide < 'a > {
            ExMoveAndCollide::new(self, motion,)
        }
        pub(crate) fn test_move_full(&mut self, from: Transform3D, motion: Vector3, collision: ObjectArg < crate::classes::KinematicCollision3D >, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,) -> bool {
            type CallSig = (bool, Transform3D, Vector3, ObjectArg < crate::classes::KinematicCollision3D >, f32, bool, i32);
            let args = (from, motion, collision, safe_margin, recovery_as_collision, max_collisions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "test_move", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::test_move_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn test_move(&mut self, from: Transform3D, motion: Vector3,) -> bool {
            self.test_move_ex(from, motion,) . done()
        }
        #[inline]
        pub fn test_move_ex < 'a > (&'a mut self, from: Transform3D, motion: Vector3,) -> ExTestMove < 'a > {
            ExTestMove::new(self, from, motion,)
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_axis_lock(&mut self, axis: crate::classes::physics_server_3d::BodyAxis, lock: bool,) {
            type CallSig = ((), crate::classes::physics_server_3d::BodyAxis, bool);
            let args = (axis, lock,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "set_axis_lock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_axis_lock(&self, axis: crate::classes::physics_server_3d::BodyAxis,) -> bool {
            type CallSig = (bool, crate::classes::physics_server_3d::BodyAxis);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "get_axis_lock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_exceptions(&mut self,) -> Array < Gd < crate::classes::PhysicsBody3D > > {
            type CallSig = (Array < Gd < crate::classes::PhysicsBody3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "get_collision_exceptions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_collision_exception_with(&mut self, body: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (body.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "add_collision_exception_with", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_collision_exception_with(&mut self, body: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (body.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "remove_collision_exception_with", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsBody3D {
        type Base = crate::classes::CollisionObject3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicsBody3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for PhysicsBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for PhysicsBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicsBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsBody3D {
        
    }
    impl std::ops::Deref for PhysicsBody3D {
        type Target = crate::classes::CollisionObject3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsBody3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicsBody3D {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`PhysicsBody3D::move_and_collide_ex`][super::PhysicsBody3D::move_and_collide_ex]."]
#[must_use]
pub struct ExMoveAndCollide < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsBody3D, motion: Vector3, test_only: bool, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveAndCollide < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody3D, motion: Vector3,) -> Self {
        let test_only = false;
        let safe_margin = 0.001f32;
        let recovery_as_collision = false;
        let max_collisions = 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, motion: motion, test_only: test_only, safe_margin: safe_margin, recovery_as_collision: recovery_as_collision, max_collisions: max_collisions,
        }
    }
    #[inline]
    pub fn test_only(self, test_only: bool) -> Self {
        Self {
            test_only: test_only, .. self
        }
    }
    #[inline]
    pub fn safe_margin(self, safe_margin: f32) -> Self {
        Self {
            safe_margin: safe_margin, .. self
        }
    }
    #[inline]
    pub fn recovery_as_collision(self, recovery_as_collision: bool) -> Self {
        Self {
            recovery_as_collision: recovery_as_collision, .. self
        }
    }
    #[inline]
    pub fn max_collisions(self, max_collisions: i32) -> Self {
        Self {
            max_collisions: max_collisions, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::KinematicCollision3D > > {
        let Self {
            _phantom, surround_object, motion, test_only, safe_margin, recovery_as_collision, max_collisions,
        }
        = self;
        re_export::PhysicsBody3D::move_and_collide_full(surround_object, motion, test_only, safe_margin, recovery_as_collision, max_collisions,)
    }
}
#[doc = "Default-param extender for [`PhysicsBody3D::test_move_ex`][super::PhysicsBody3D::test_move_ex]."]
#[must_use]
pub struct ExTestMove < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsBody3D, from: Transform3D, motion: Vector3, collision: ObjectCow < crate::classes::KinematicCollision3D >, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTestMove < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody3D, from: Transform3D, motion: Vector3,) -> Self {
        let collision = Gd::null_arg();
        let safe_margin = 0.001f32;
        let recovery_as_collision = false;
        let max_collisions = 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, motion: motion, collision: collision.consume_arg(), safe_margin: safe_margin, recovery_as_collision: recovery_as_collision, max_collisions: max_collisions,
        }
    }
    #[inline]
    pub fn collision(self, collision: impl AsObjectArg < crate::classes::KinematicCollision3D >) -> Self {
        Self {
            collision: collision.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn safe_margin(self, safe_margin: f32) -> Self {
        Self {
            safe_margin: safe_margin, .. self
        }
    }
    #[inline]
    pub fn recovery_as_collision(self, recovery_as_collision: bool) -> Self {
        Self {
            recovery_as_collision: recovery_as_collision, .. self
        }
    }
    #[inline]
    pub fn max_collisions(self, max_collisions: i32) -> Self {
        Self {
            max_collisions: max_collisions, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, from, motion, collision, safe_margin, recovery_as_collision, max_collisions,
        }
        = self;
        re_export::PhysicsBody3D::test_move_full(surround_object, from, motion, collision.cow_as_object_arg(), safe_margin, recovery_as_collision, max_collisions,)
    }
}