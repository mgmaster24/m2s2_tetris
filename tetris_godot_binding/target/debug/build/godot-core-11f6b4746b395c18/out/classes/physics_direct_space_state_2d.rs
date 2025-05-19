#![doc = "Sidecar module for class [`PhysicsDirectSpaceState2D`][crate::classes::PhysicsDirectSpaceState2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectSpaceState2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectSpaceState2D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_direct_space_state_2d`][crate::classes::physics_direct_space_state_2d]: sidecar module with related enum/flag types\n* [`IPhysicsDirectSpaceState2D`][crate::classes::IPhysicsDirectSpaceState2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsDirectSpaceState2D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsDirectSpaceState2D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectSpaceState2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsDirectSpaceState2D`][crate::classes::PhysicsDirectSpaceState2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsDirectSpaceState2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsDirectSpaceState2D: crate::obj::GodotClass < Base = PhysicsDirectSpaceState2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsDirectSpaceState2D {
        pub(crate) fn intersect_point_full(&mut self, parameters: ObjectArg < crate::classes::PhysicsPointQueryParameters2D >, max_results: i32,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >, ObjectArg < crate::classes::PhysicsPointQueryParameters2D >, i32);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "intersect_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::intersect_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn intersect_point(&mut self, parameters: impl AsObjectArg < crate::classes::PhysicsPointQueryParameters2D >,) -> Array < Dictionary > {
            self.intersect_point_ex(parameters,) . done()
        }
        #[inline]
        pub fn intersect_point_ex < 'a > (&'a mut self, parameters: impl AsObjectArg < crate::classes::PhysicsPointQueryParameters2D >,) -> ExIntersectPoint < 'a > {
            ExIntersectPoint::new(self, parameters,)
        }
        pub fn intersect_ray(&mut self, parameters: impl AsObjectArg < crate::classes::PhysicsRayQueryParameters2D >,) -> Dictionary {
            type CallSig = (Dictionary, ObjectArg < crate::classes::PhysicsRayQueryParameters2D >);
            let args = (parameters.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "intersect_ray", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn intersect_shape_full(&mut self, parameters: ObjectArg < crate::classes::PhysicsShapeQueryParameters2D >, max_results: i32,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >, ObjectArg < crate::classes::PhysicsShapeQueryParameters2D >, i32);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "intersect_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::intersect_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn intersect_shape(&mut self, parameters: impl AsObjectArg < crate::classes::PhysicsShapeQueryParameters2D >,) -> Array < Dictionary > {
            self.intersect_shape_ex(parameters,) . done()
        }
        #[inline]
        pub fn intersect_shape_ex < 'a > (&'a mut self, parameters: impl AsObjectArg < crate::classes::PhysicsShapeQueryParameters2D >,) -> ExIntersectShape < 'a > {
            ExIntersectShape::new(self, parameters,)
        }
        pub fn cast_motion(&mut self, parameters: impl AsObjectArg < crate::classes::PhysicsShapeQueryParameters2D >,) -> PackedFloat32Array {
            type CallSig = (PackedFloat32Array, ObjectArg < crate::classes::PhysicsShapeQueryParameters2D >);
            let args = (parameters.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "cast_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn collide_shape_full(&mut self, parameters: ObjectArg < crate::classes::PhysicsShapeQueryParameters2D >, max_results: i32,) -> Array < Vector2 > {
            type CallSig = (Array < Vector2 >, ObjectArg < crate::classes::PhysicsShapeQueryParameters2D >, i32);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "collide_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::collide_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn collide_shape(&mut self, parameters: impl AsObjectArg < crate::classes::PhysicsShapeQueryParameters2D >,) -> Array < Vector2 > {
            self.collide_shape_ex(parameters,) . done()
        }
        #[inline]
        pub fn collide_shape_ex < 'a > (&'a mut self, parameters: impl AsObjectArg < crate::classes::PhysicsShapeQueryParameters2D >,) -> ExCollideShape < 'a > {
            ExCollideShape::new(self, parameters,)
        }
        pub fn get_rest_info(&mut self, parameters: impl AsObjectArg < crate::classes::PhysicsShapeQueryParameters2D >,) -> Dictionary {
            type CallSig = (Dictionary, ObjectArg < crate::classes::PhysicsShapeQueryParameters2D >);
            let args = (parameters.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "get_rest_info", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectSpaceState2D {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicsDirectSpaceState2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectSpaceState2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsDirectSpaceState2D {
        
    }
    impl std::ops::Deref for PhysicsDirectSpaceState2D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectSpaceState2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsDirectSpaceState2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicsDirectSpaceState2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsDirectSpaceState2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::intersect_point_ex`][super::PhysicsDirectSpaceState2D::intersect_point_ex]."]
#[must_use]
pub struct ExIntersectPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: ObjectCow < crate::classes::PhysicsPointQueryParameters2D >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIntersectPoint < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: impl AsObjectArg < crate::classes::PhysicsPointQueryParameters2D >,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.consume_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState2D::intersect_point_full(surround_object, parameters.cow_as_object_arg(), max_results,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::intersect_shape_ex`][super::PhysicsDirectSpaceState2D::intersect_shape_ex]."]
#[must_use]
pub struct ExIntersectShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: ObjectCow < crate::classes::PhysicsShapeQueryParameters2D >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIntersectShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: impl AsObjectArg < crate::classes::PhysicsShapeQueryParameters2D >,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.consume_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState2D::intersect_shape_full(surround_object, parameters.cow_as_object_arg(), max_results,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::collide_shape_ex`][super::PhysicsDirectSpaceState2D::collide_shape_ex]."]
#[must_use]
pub struct ExCollideShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: ObjectCow < crate::classes::PhysicsShapeQueryParameters2D >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCollideShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: impl AsObjectArg < crate::classes::PhysicsShapeQueryParameters2D >,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.consume_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2 > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState2D::collide_shape_full(surround_object, parameters.cow_as_object_arg(), max_results,)
    }
}