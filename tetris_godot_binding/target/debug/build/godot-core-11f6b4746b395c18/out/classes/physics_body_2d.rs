#![doc = "Sidecar module for class [`PhysicsBody2D`][crate::classes::PhysicsBody2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsBody2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsbody2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsBody2D.`\n\nInherits [`CollisionObject2D`][crate::classes::CollisionObject2D].\n\nRelated symbols:\n\n* [`physics_body_2d`][crate::classes::physics_body_2d]: sidecar module with related enum/flag types\n* [`IPhysicsBody2D`][crate::classes::IPhysicsBody2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsBody2D`](https://docs.godotengine.org/en/stable/classes/class_physicsbody2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsBody2D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsBody2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsBody2D`][crate::classes::PhysicsBody2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsBody2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsbody2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsBody2D: crate::obj::GodotClass < Base = PhysicsBody2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsBody2D {
        pub(crate) fn move_and_collide_full(&mut self, motion: Vector2, test_only: bool, safe_margin: f32, recovery_as_collision: bool,) -> Option < Gd < crate::classes::KinematicCollision2D > > {
            type CallSig = (Option < Gd < crate::classes::KinematicCollision2D > >, Vector2, bool, f32, bool);
            let args = (motion, test_only, safe_margin, recovery_as_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "move_and_collide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::move_and_collide_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn move_and_collide(&mut self, motion: Vector2,) -> Option < Gd < crate::classes::KinematicCollision2D > > {
            self.move_and_collide_ex(motion,) . done()
        }
        #[inline]
        pub fn move_and_collide_ex < 'a > (&'a mut self, motion: Vector2,) -> ExMoveAndCollide < 'a > {
            ExMoveAndCollide::new(self, motion,)
        }
        pub(crate) fn test_move_full(&mut self, from: Transform2D, motion: Vector2, collision: ObjectArg < crate::classes::KinematicCollision2D >, safe_margin: f32, recovery_as_collision: bool,) -> bool {
            type CallSig = (bool, Transform2D, Vector2, ObjectArg < crate::classes::KinematicCollision2D >, f32, bool);
            let args = (from, motion, collision, safe_margin, recovery_as_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "test_move", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::test_move_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn test_move(&mut self, from: Transform2D, motion: Vector2,) -> bool {
            self.test_move_ex(from, motion,) . done()
        }
        #[inline]
        pub fn test_move_ex < 'a > (&'a mut self, from: Transform2D, motion: Vector2,) -> ExTestMove < 'a > {
            ExTestMove::new(self, from, motion,)
        }
        pub fn get_gravity(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_exceptions(&mut self,) -> Array < Gd < crate::classes::PhysicsBody2D > > {
            type CallSig = (Array < Gd < crate::classes::PhysicsBody2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "get_collision_exceptions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_collision_exception_with(&mut self, body: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (body.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "add_collision_exception_with", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_collision_exception_with(&mut self, body: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (body.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "remove_collision_exception_with", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsBody2D {
        type Base = crate::classes::CollisionObject2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicsBody2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsBody2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsBody2D {
        
    }
    impl std::ops::Deref for PhysicsBody2D {
        type Target = crate::classes::CollisionObject2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsBody2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsBody2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicsBody2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsBody2D > for $Class {
                
            }
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
#[doc = "Default-param extender for [`PhysicsBody2D::move_and_collide_ex`][super::PhysicsBody2D::move_and_collide_ex]."]
#[must_use]
pub struct ExMoveAndCollide < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsBody2D, motion: Vector2, test_only: bool, safe_margin: f32, recovery_as_collision: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveAndCollide < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody2D, motion: Vector2,) -> Self {
        let test_only = false;
        let safe_margin = 0.08f32;
        let recovery_as_collision = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, motion: motion, test_only: test_only, safe_margin: safe_margin, recovery_as_collision: recovery_as_collision,
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
    pub fn done(self) -> Option < Gd < crate::classes::KinematicCollision2D > > {
        let Self {
            _phantom, surround_object, motion, test_only, safe_margin, recovery_as_collision,
        }
        = self;
        re_export::PhysicsBody2D::move_and_collide_full(surround_object, motion, test_only, safe_margin, recovery_as_collision,)
    }
}
#[doc = "Default-param extender for [`PhysicsBody2D::test_move_ex`][super::PhysicsBody2D::test_move_ex]."]
#[must_use]
pub struct ExTestMove < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsBody2D, from: Transform2D, motion: Vector2, collision: ObjectCow < crate::classes::KinematicCollision2D >, safe_margin: f32, recovery_as_collision: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTestMove < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody2D, from: Transform2D, motion: Vector2,) -> Self {
        let collision = Gd::null_arg();
        let safe_margin = 0.08f32;
        let recovery_as_collision = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, motion: motion, collision: collision.consume_arg(), safe_margin: safe_margin, recovery_as_collision: recovery_as_collision,
        }
    }
    #[inline]
    pub fn collision(self, collision: impl AsObjectArg < crate::classes::KinematicCollision2D >) -> Self {
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
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, from, motion, collision, safe_margin, recovery_as_collision,
        }
        = self;
        re_export::PhysicsBody2D::test_move_full(surround_object, from, motion, collision.cow_as_object_arg(), safe_margin, recovery_as_collision,)
    }
}