#![doc = "Sidecar module for class [`GltfPhysicsBody`][crate::classes::GltfPhysicsBody].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFPhysicsBody` enums](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsbody.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFPhysicsBody.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IGltfPhysicsBody`][crate::classes::IGltfPhysicsBody]: virtual methods\n\n\nSee also [Godot docs for `GLTFPhysicsBody`](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsbody.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfPhysicsBody::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfPhysicsBody {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfPhysicsBody`][crate::classes::GltfPhysicsBody].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFPhysicsBody` methods](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsbody.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfPhysicsBody: crate::obj::GodotClass < Base = GltfPhysicsBody > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl GltfPhysicsBody {
        pub fn from_node(body_node: impl AsObjectArg < crate::classes::CollisionObject3D >,) -> Option < Gd < crate::classes::GltfPhysicsBody > > {
            type CallSig = (Option < Gd < crate::classes::GltfPhysicsBody > >, ObjectArg < crate::classes::CollisionObject3D >);
            let args = (body_node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "from_node", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn to_node(&self,) -> Option < Gd < crate::classes::CollisionObject3D > > {
            type CallSig = (Option < Gd < crate::classes::CollisionObject3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "to_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn from_dictionary(dictionary: &Dictionary,) -> Option < Gd < crate::classes::GltfPhysicsBody > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::GltfPhysicsBody > >, RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(dictionary),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "from_dictionary", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn to_dictionary(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "to_dictionary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_body_type(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "get_body_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_body_type(&mut self, body_type: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (body_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "set_body_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mass(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "get_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mass(&mut self, mass: f32,) {
            type CallSig = ((), f32);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "set_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "get_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, linear_velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "set_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "get_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, angular_velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "set_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "get_center_of_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_of_mass(&mut self, center_of_mass: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (center_of_mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "set_center_of_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inertia_diagonal(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "get_inertia_diagonal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inertia_diagonal(&mut self, inertia_diagonal: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (inertia_diagonal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "set_inertia_diagonal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inertia_orientation(&self,) -> Quaternion {
            type CallSig = (Quaternion,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "get_inertia_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inertia_orientation(&mut self, inertia_orientation: Quaternion,) {
            type CallSig = ((), Quaternion);
            let args = (inertia_orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "set_inertia_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inertia_tensor(&self,) -> Basis {
            type CallSig = (Basis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "get_inertia_tensor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inertia_tensor(&mut self, inertia_tensor: Basis,) {
            type CallSig = ((), Basis);
            let args = (inertia_tensor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsBody", "set_inertia_tensor", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GltfPhysicsBody {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GLTFPhysicsBody"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfPhysicsBody {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfPhysicsBody {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfPhysicsBody {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfPhysicsBody {
        
    }
    impl crate::obj::cap::GodotDefault for GltfPhysicsBody {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfPhysicsBody {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfPhysicsBody {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfPhysicsBody`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GltfPhysicsBody {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfPhysicsBody > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}