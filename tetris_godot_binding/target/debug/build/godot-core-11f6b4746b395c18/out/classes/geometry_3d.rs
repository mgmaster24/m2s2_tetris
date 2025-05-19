#![doc = "Sidecar module for class [`Geometry3D`][crate::classes::Geometry3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Geometry3D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`geometry_3d`][crate::classes::geometry_3d]: sidecar module with related enum/flag types\n* [`IGeometry3D`][crate::classes::IGeometry3D]: virtual methods\n\n\nSee also [Godot docs for `Geometry3D`](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Geometry3D::singleton()`][Geometry3D::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Geometry3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Geometry3D`][crate::classes::Geometry3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Geometry3D` methods](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeometry3D: crate::obj::GodotClass < Base = Geometry3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Geometry3D {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"Geometry3D");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn compute_convex_mesh_points(&mut self, planes: &Array < Plane >,) -> PackedVector3Array {
            type CallSig < 'a0, > = (PackedVector3Array, RefArg < 'a0, Array < Plane > >);
            let args = (RefArg::new(planes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "compute_convex_mesh_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn build_box_planes(&mut self, extents: Vector3,) -> Array < Plane > {
            type CallSig = (Array < Plane >, Vector3);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "build_box_planes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn build_cylinder_planes_full(&mut self, radius: f32, height: f32, sides: i32, axis: Vector3Axis,) -> Array < Plane > {
            type CallSig = (Array < Plane >, f32, f32, i32, Vector3Axis);
            let args = (radius, height, sides, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "build_cylinder_planes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::build_cylinder_planes_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn build_cylinder_planes(&mut self, radius: f32, height: f32, sides: i32,) -> Array < Plane > {
            self.build_cylinder_planes_ex(radius, height, sides,) . done()
        }
        #[inline]
        pub fn build_cylinder_planes_ex < 'a > (&'a mut self, radius: f32, height: f32, sides: i32,) -> ExBuildCylinderPlanes < 'a > {
            ExBuildCylinderPlanes::new(self, radius, height, sides,)
        }
        pub(crate) fn build_capsule_planes_full(&mut self, radius: f32, height: f32, sides: i32, lats: i32, axis: Vector3Axis,) -> Array < Plane > {
            type CallSig = (Array < Plane >, f32, f32, i32, i32, Vector3Axis);
            let args = (radius, height, sides, lats, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "build_capsule_planes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::build_capsule_planes_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn build_capsule_planes(&mut self, radius: f32, height: f32, sides: i32, lats: i32,) -> Array < Plane > {
            self.build_capsule_planes_ex(radius, height, sides, lats,) . done()
        }
        #[inline]
        pub fn build_capsule_planes_ex < 'a > (&'a mut self, radius: f32, height: f32, sides: i32, lats: i32,) -> ExBuildCapsulePlanes < 'a > {
            ExBuildCapsulePlanes::new(self, radius, height, sides, lats,)
        }
        pub fn get_closest_points_between_segments(&mut self, p1: Vector3, p2: Vector3, q1: Vector3, q2: Vector3,) -> PackedVector3Array {
            type CallSig = (PackedVector3Array, Vector3, Vector3, Vector3, Vector3);
            let args = (p1, p2, q1, q2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_points_between_segments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point_to_segment(&mut self, point: Vector3, s1: Vector3, s2: Vector3,) -> Vector3 {
            type CallSig = (Vector3, Vector3, Vector3, Vector3);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_point_to_segment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point_to_segment_uncapped(&mut self, point: Vector3, s1: Vector3, s2: Vector3,) -> Vector3 {
            type CallSig = (Vector3, Vector3, Vector3, Vector3);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_point_to_segment_uncapped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_triangle_barycentric_coords(&mut self, point: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Vector3 {
            type CallSig = (Vector3, Vector3, Vector3, Vector3, Vector3);
            let args = (point, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "get_triangle_barycentric_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ray_intersects_triangle(&mut self, from: Vector3, dir: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Variant {
            type CallSig = (Variant, Vector3, Vector3, Vector3, Vector3, Vector3);
            let args = (from, dir, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "ray_intersects_triangle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_triangle(&mut self, from: Vector3, to: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Variant {
            type CallSig = (Variant, Vector3, Vector3, Vector3, Vector3, Vector3);
            let args = (from, to, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_triangle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_sphere(&mut self, from: Vector3, to: Vector3, sphere_position: Vector3, sphere_radius: f32,) -> PackedVector3Array {
            type CallSig = (PackedVector3Array, Vector3, Vector3, Vector3, f32);
            let args = (from, to, sphere_position, sphere_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_sphere", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_cylinder(&mut self, from: Vector3, to: Vector3, height: f32, radius: f32,) -> PackedVector3Array {
            type CallSig = (PackedVector3Array, Vector3, Vector3, f32, f32);
            let args = (from, to, height, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_cylinder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_convex(&mut self, from: Vector3, to: Vector3, planes: &Array < Plane >,) -> PackedVector3Array {
            type CallSig < 'a0, > = (PackedVector3Array, Vector3, Vector3, RefArg < 'a0, Array < Plane > >);
            let args = (from, to, RefArg::new(planes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_convex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clip_polygon(&mut self, points: &PackedVector3Array, plane: Plane,) -> PackedVector3Array {
            type CallSig < 'a0, > = (PackedVector3Array, RefArg < 'a0, PackedVector3Array >, Plane);
            let args = (RefArg::new(points), plane,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "clip_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tetrahedralize_delaunay(&mut self, points: &PackedVector3Array,) -> PackedInt32Array {
            type CallSig < 'a0, > = (PackedInt32Array, RefArg < 'a0, PackedVector3Array >);
            let args = (RefArg::new(points),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry3D", "tetrahedralize_delaunay", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Geometry3D {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Geometry3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Geometry3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Geometry3D {
        
    }
    impl std::ops::Deref for Geometry3D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Geometry3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Geometry3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Geometry3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Geometry3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Geometry3D::build_cylinder_planes_ex`][super::Geometry3D::build_cylinder_planes_ex]."]
#[must_use]
pub struct ExBuildCylinderPlanes < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, axis: Vector3Axis,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBuildCylinderPlanes < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32,) -> Self {
        let axis = crate::obj::EngineEnum::from_ord(2);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, radius: radius, height: height, sides: sides, axis: axis,
        }
    }
    #[inline]
    pub fn axis(self, axis: Vector3Axis) -> Self {
        Self {
            axis: axis, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Plane > {
        let Self {
            _phantom, surround_object, radius, height, sides, axis,
        }
        = self;
        re_export::Geometry3D::build_cylinder_planes_full(surround_object, radius, height, sides, axis,)
    }
}
#[doc = "Default-param extender for [`Geometry3D::build_capsule_planes_ex`][super::Geometry3D::build_capsule_planes_ex]."]
#[must_use]
pub struct ExBuildCapsulePlanes < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, lats: i32, axis: Vector3Axis,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBuildCapsulePlanes < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, lats: i32,) -> Self {
        let axis = crate::obj::EngineEnum::from_ord(2);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, radius: radius, height: height, sides: sides, lats: lats, axis: axis,
        }
    }
    #[inline]
    pub fn axis(self, axis: Vector3Axis) -> Self {
        Self {
            axis: axis, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Plane > {
        let Self {
            _phantom, surround_object, radius, height, sides, lats, axis,
        }
        = self;
        re_export::Geometry3D::build_capsule_planes_full(surround_object, radius, height, sides, lats, axis,)
    }
}