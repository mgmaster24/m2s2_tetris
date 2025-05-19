#![doc = "Sidecar module for class [`CsgPolygon3D`][crate::classes::CsgPolygon3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CSGPolygon3D` enums](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CSGPolygon3D.`\n\nInherits [`CsgPrimitive3D`][crate::classes::CsgPrimitive3D].\n\nRelated symbols:\n\n* [`csg_polygon_3d`][crate::classes::csg_polygon_3d]: sidecar module with related enum/flag types\n* [`ICsgPolygon3D`][crate::classes::ICsgPolygon3D]: virtual methods\n\n\nSee also [Godot docs for `CSGPolygon3D`](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CsgPolygon3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CsgPolygon3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CsgPolygon3D`][crate::classes::CsgPolygon3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CSGPolygon3D` methods](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICsgPolygon3D: crate::obj::GodotClass < Base = CsgPolygon3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl CsgPolygon3D {
        pub fn set_polygon(&mut self, polygon: &PackedVector2Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_polygon(&self,) -> PackedVector2Array {
            type CallSig = (PackedVector2Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mode(&mut self, mode: crate::classes::csg_polygon_3d::Mode,) {
            type CallSig = ((), crate::classes::csg_polygon_3d::Mode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mode(&self,) -> crate::classes::csg_polygon_3d::Mode {
            type CallSig = (crate::classes::csg_polygon_3d::Mode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth(&mut self, depth: f32,) {
            type CallSig = ((), f32);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spin_degrees(&mut self, degrees: f32,) {
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_spin_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spin_degrees(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_spin_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spin_sides(&mut self, spin_sides: i32,) {
            type CallSig = ((), i32);
            let args = (spin_sides,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_spin_sides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spin_sides(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_spin_sides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_node(&mut self, path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_node(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_interval_type(&mut self, interval_type: crate::classes::csg_polygon_3d::PathIntervalType,) {
            type CallSig = ((), crate::classes::csg_polygon_3d::PathIntervalType);
            let args = (interval_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_interval_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_interval_type(&self,) -> crate::classes::csg_polygon_3d::PathIntervalType {
            type CallSig = (crate::classes::csg_polygon_3d::PathIntervalType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_interval_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_interval(&mut self, interval: f32,) {
            type CallSig = ((), f32);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_interval(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_simplify_angle(&mut self, degrees: f32,) {
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_simplify_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_simplify_angle(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_simplify_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_rotation(&mut self, path_rotation: crate::classes::csg_polygon_3d::PathRotation,) {
            type CallSig = ((), crate::classes::csg_polygon_3d::PathRotation);
            let args = (path_rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_rotation(&self,) -> crate::classes::csg_polygon_3d::PathRotation {
            type CallSig = (crate::classes::csg_polygon_3d::PathRotation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_local(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_path_local(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "is_path_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_continuous_u(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_continuous_u", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_path_continuous_u(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "is_path_continuous_u", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_u_distance(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_u_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_u_distance(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_u_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_path_joined(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_joined", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_path_joined(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "is_path_joined", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material(&mut self, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), ObjectArg < crate::classes::Material >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_smooth_faces(&mut self, smooth_faces: bool,) {
            type CallSig = ((), bool);
            let args = (smooth_faces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_smooth_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_smooth_faces(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_smooth_faces", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CsgPolygon3D {
        type Base = crate::classes::CsgPrimitive3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"CSGPolygon3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CsgPolygon3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CsgPrimitive3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CsgShape3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CsgPolygon3D {
        
    }
    impl crate::obj::cap::GodotDefault for CsgPolygon3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CsgPolygon3D {
        type Target = crate::classes::CsgPrimitive3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CsgPolygon3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CsgPolygon3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_CsgPolygon3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CsgPolygon3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CsgPrimitive3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CsgShape3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
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
pub struct Mode {
    ord: i32
}
impl Mode {
    #[doc(alias = "MODE_DEPTH")]
    #[doc = "Godot enumerator name: `MODE_DEPTH`"]
    pub const DEPTH: Mode = Mode {
        ord: 0i32
    };
    #[doc(alias = "MODE_SPIN")]
    #[doc = "Godot enumerator name: `MODE_SPIN`"]
    pub const SPIN: Mode = Mode {
        ord: 1i32
    };
    #[doc(alias = "MODE_PATH")]
    #[doc = "Godot enumerator name: `MODE_PATH`"]
    pub const PATH: Mode = Mode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Mode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Mode {
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
            Self::DEPTH => "DEPTH", Self::SPIN => "SPIN", Self::PATH => "PATH", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEPTH => "MODE_DEPTH", Self::SPIN => "MODE_SPIN", Self::PATH => "MODE_PATH", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Mode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Mode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PathRotation {
    ord: i32
}
impl PathRotation {
    #[doc(alias = "PATH_ROTATION_POLYGON")]
    #[doc = "Godot enumerator name: `PATH_ROTATION_POLYGON`"]
    pub const POLYGON: PathRotation = PathRotation {
        ord: 0i32
    };
    #[doc(alias = "PATH_ROTATION_PATH")]
    #[doc = "Godot enumerator name: `PATH_ROTATION_PATH`"]
    pub const PATH: PathRotation = PathRotation {
        ord: 1i32
    };
    #[doc(alias = "PATH_ROTATION_PATH_FOLLOW")]
    #[doc = "Godot enumerator name: `PATH_ROTATION_PATH_FOLLOW`"]
    pub const PATH_FOLLOW: PathRotation = PathRotation {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PathRotation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PathRotation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PathRotation {
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
            Self::POLYGON => "POLYGON", Self::PATH => "PATH", Self::PATH_FOLLOW => "PATH_FOLLOW", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::POLYGON => "PATH_ROTATION_POLYGON", Self::PATH => "PATH_ROTATION_PATH", Self::PATH_FOLLOW => "PATH_ROTATION_PATH_FOLLOW", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PathRotation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PathRotation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PathRotation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PathIntervalType {
    ord: i32
}
impl PathIntervalType {
    #[doc(alias = "PATH_INTERVAL_DISTANCE")]
    #[doc = "Godot enumerator name: `PATH_INTERVAL_DISTANCE`"]
    pub const DISTANCE: PathIntervalType = PathIntervalType {
        ord: 0i32
    };
    #[doc(alias = "PATH_INTERVAL_SUBDIVIDE")]
    #[doc = "Godot enumerator name: `PATH_INTERVAL_SUBDIVIDE`"]
    pub const SUBDIVIDE: PathIntervalType = PathIntervalType {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for PathIntervalType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PathIntervalType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PathIntervalType {
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
            Self::DISTANCE => "DISTANCE", Self::SUBDIVIDE => "SUBDIVIDE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISTANCE => "PATH_INTERVAL_DISTANCE", Self::SUBDIVIDE => "PATH_INTERVAL_SUBDIVIDE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PathIntervalType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PathIntervalType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PathIntervalType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}