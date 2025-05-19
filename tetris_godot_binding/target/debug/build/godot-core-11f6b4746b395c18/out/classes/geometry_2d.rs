#![doc = "Sidecar module for class [`Geometry2D`][crate::classes::Geometry2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry2D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Geometry2D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`geometry_2d`][crate::classes::geometry_2d]: sidecar module with related enum/flag types\n* [`IGeometry2D`][crate::classes::IGeometry2D]: virtual methods\n\n\nSee also [Godot docs for `Geometry2D`](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Geometry2D::singleton()`][Geometry2D::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Geometry2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Geometry2D`][crate::classes::Geometry2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Geometry2D` methods](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeometry2D: crate::obj::GodotClass < Base = Geometry2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Geometry2D {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"Geometry2D");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn is_point_in_circle(&mut self, point: Vector2, circle_position: Vector2, circle_radius: f32,) -> bool {
            type CallSig = (bool, Vector2, Vector2, f32);
            let args = (point, circle_position, circle_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "is_point_in_circle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_circle(&mut self, segment_from: Vector2, segment_to: Vector2, circle_position: Vector2, circle_radius: f32,) -> f32 {
            type CallSig = (f32, Vector2, Vector2, Vector2, f32);
            let args = (segment_from, segment_to, circle_position, circle_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "segment_intersects_circle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_segment(&mut self, from_a: Vector2, to_a: Vector2, from_b: Vector2, to_b: Vector2,) -> Variant {
            type CallSig = (Variant, Vector2, Vector2, Vector2, Vector2);
            let args = (from_a, to_a, from_b, to_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "segment_intersects_segment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn line_intersects_line(&mut self, from_a: Vector2, dir_a: Vector2, from_b: Vector2, dir_b: Vector2,) -> Variant {
            type CallSig = (Variant, Vector2, Vector2, Vector2, Vector2);
            let args = (from_a, dir_a, from_b, dir_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "line_intersects_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_points_between_segments(&mut self, p1: Vector2, q1: Vector2, p2: Vector2, q2: Vector2,) -> PackedVector2Array {
            type CallSig = (PackedVector2Array, Vector2, Vector2, Vector2, Vector2);
            let args = (p1, q1, p2, q2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "get_closest_points_between_segments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point_to_segment(&mut self, point: Vector2, s1: Vector2, s2: Vector2,) -> Vector2 {
            type CallSig = (Vector2, Vector2, Vector2, Vector2);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "get_closest_point_to_segment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point_to_segment_uncapped(&mut self, point: Vector2, s1: Vector2, s2: Vector2,) -> Vector2 {
            type CallSig = (Vector2, Vector2, Vector2, Vector2);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "get_closest_point_to_segment_uncapped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn point_is_inside_triangle(&self, point: Vector2, a: Vector2, b: Vector2, c: Vector2,) -> bool {
            type CallSig = (bool, Vector2, Vector2, Vector2, Vector2);
            let args = (point, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "point_is_inside_triangle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_polygon_clockwise(&mut self, polygon: &PackedVector2Array,) -> bool {
            type CallSig < 'a0, > = (bool, RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "is_polygon_clockwise", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_point_in_polygon(&mut self, point: Vector2, polygon: &PackedVector2Array,) -> bool {
            type CallSig < 'a0, > = (bool, Vector2, RefArg < 'a0, PackedVector2Array >);
            let args = (point, RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "is_point_in_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn triangulate_polygon(&mut self, polygon: &PackedVector2Array,) -> PackedInt32Array {
            type CallSig < 'a0, > = (PackedInt32Array, RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "triangulate_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn triangulate_delaunay(&mut self, points: &PackedVector2Array,) -> PackedInt32Array {
            type CallSig < 'a0, > = (PackedInt32Array, RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(points),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "triangulate_delaunay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convex_hull(&mut self, points: &PackedVector2Array,) -> PackedVector2Array {
            type CallSig < 'a0, > = (PackedVector2Array, RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(points),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "convex_hull", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decompose_polygon_in_convex(&mut self, polygon: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "decompose_polygon_in_convex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn merge_polygons(&mut self, polygon_a: &PackedVector2Array, polygon_b: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, 'a1, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >);
            let args = (RefArg::new(polygon_a), RefArg::new(polygon_b),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "merge_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clip_polygons(&mut self, polygon_a: &PackedVector2Array, polygon_b: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, 'a1, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >);
            let args = (RefArg::new(polygon_a), RefArg::new(polygon_b),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "clip_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn intersect_polygons(&mut self, polygon_a: &PackedVector2Array, polygon_b: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, 'a1, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >);
            let args = (RefArg::new(polygon_a), RefArg::new(polygon_b),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "intersect_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn exclude_polygons(&mut self, polygon_a: &PackedVector2Array, polygon_b: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, 'a1, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >);
            let args = (RefArg::new(polygon_a), RefArg::new(polygon_b),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "exclude_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clip_polyline_with_polygon(&mut self, polyline: &PackedVector2Array, polygon: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, 'a1, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >);
            let args = (RefArg::new(polyline), RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "clip_polyline_with_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn intersect_polyline_with_polygon(&mut self, polyline: &PackedVector2Array, polygon: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, 'a1, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >);
            let args = (RefArg::new(polyline), RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "intersect_polyline_with_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn offset_polygon_full(&mut self, polygon: RefArg < PackedVector2Array >, delta: f32, join_type: crate::classes::geometry_2d::PolyJoinType,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >, f32, crate::classes::geometry_2d::PolyJoinType);
            let args = (polygon, delta, join_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "offset_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::offset_polygon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn offset_polygon(&mut self, polygon: &PackedVector2Array, delta: f32,) -> Array < PackedVector2Array > {
            self.offset_polygon_ex(polygon, delta,) . done()
        }
        #[inline]
        pub fn offset_polygon_ex < 'a > (&'a mut self, polygon: &'a PackedVector2Array, delta: f32,) -> ExOffsetPolygon < 'a > {
            ExOffsetPolygon::new(self, polygon, delta,)
        }
        pub(crate) fn offset_polyline_full(&mut self, polyline: RefArg < PackedVector2Array >, delta: f32, join_type: crate::classes::geometry_2d::PolyJoinType, end_type: crate::classes::geometry_2d::PolyEndType,) -> Array < PackedVector2Array > {
            type CallSig < 'a0, > = (Array < PackedVector2Array >, RefArg < 'a0, PackedVector2Array >, f32, crate::classes::geometry_2d::PolyJoinType, crate::classes::geometry_2d::PolyEndType);
            let args = (polyline, delta, join_type, end_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "offset_polyline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::offset_polyline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn offset_polyline(&mut self, polyline: &PackedVector2Array, delta: f32,) -> Array < PackedVector2Array > {
            self.offset_polyline_ex(polyline, delta,) . done()
        }
        #[inline]
        pub fn offset_polyline_ex < 'a > (&'a mut self, polyline: &'a PackedVector2Array, delta: f32,) -> ExOffsetPolyline < 'a > {
            ExOffsetPolyline::new(self, polyline, delta,)
        }
        pub fn make_atlas(&mut self, sizes: &PackedVector2Array,) -> Dictionary {
            type CallSig < 'a0, > = (Dictionary, RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(sizes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Geometry2D", "make_atlas", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Geometry2D {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Geometry2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Geometry2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Geometry2D {
        
    }
    impl std::ops::Deref for Geometry2D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Geometry2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Geometry2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Geometry2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Geometry2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Geometry2D::offset_polygon_ex`][super::Geometry2D::offset_polygon_ex]."]
#[must_use]
pub struct ExOffsetPolygon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Geometry2D, polygon: CowArg < 'a, PackedVector2Array >, delta: f32, join_type: crate::classes::geometry_2d::PolyJoinType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOffsetPolygon < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry2D, polygon: &'a PackedVector2Array, delta: f32,) -> Self {
        let join_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, polygon: CowArg::Borrowed(polygon), delta: delta, join_type: join_type,
        }
    }
    #[inline]
    pub fn join_type(self, join_type: crate::classes::geometry_2d::PolyJoinType) -> Self {
        Self {
            join_type: join_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < PackedVector2Array > {
        let Self {
            _phantom, surround_object, polygon, delta, join_type,
        }
        = self;
        re_export::Geometry2D::offset_polygon_full(surround_object, polygon.cow_as_arg(), delta, join_type,)
    }
}
#[doc = "Default-param extender for [`Geometry2D::offset_polyline_ex`][super::Geometry2D::offset_polyline_ex]."]
#[must_use]
pub struct ExOffsetPolyline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Geometry2D, polyline: CowArg < 'a, PackedVector2Array >, delta: f32, join_type: crate::classes::geometry_2d::PolyJoinType, end_type: crate::classes::geometry_2d::PolyEndType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOffsetPolyline < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry2D, polyline: &'a PackedVector2Array, delta: f32,) -> Self {
        let join_type = crate::obj::EngineEnum::from_ord(0);
        let end_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, polyline: CowArg::Borrowed(polyline), delta: delta, join_type: join_type, end_type: end_type,
        }
    }
    #[inline]
    pub fn join_type(self, join_type: crate::classes::geometry_2d::PolyJoinType) -> Self {
        Self {
            join_type: join_type, .. self
        }
    }
    #[inline]
    pub fn end_type(self, end_type: crate::classes::geometry_2d::PolyEndType) -> Self {
        Self {
            end_type: end_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < PackedVector2Array > {
        let Self {
            _phantom, surround_object, polyline, delta, join_type, end_type,
        }
        = self;
        re_export::Geometry2D::offset_polyline_full(surround_object, polyline.cow_as_arg(), delta, join_type, end_type,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolyBooleanOperation {
    ord: i32
}
impl PolyBooleanOperation {
    #[doc(alias = "OPERATION_UNION")]
    #[doc = "Godot enumerator name: `OPERATION_UNION`"]
    pub const UNION: PolyBooleanOperation = PolyBooleanOperation {
        ord: 0i32
    };
    #[doc(alias = "OPERATION_DIFFERENCE")]
    #[doc = "Godot enumerator name: `OPERATION_DIFFERENCE`"]
    pub const DIFFERENCE: PolyBooleanOperation = PolyBooleanOperation {
        ord: 1i32
    };
    #[doc(alias = "OPERATION_INTERSECTION")]
    #[doc = "Godot enumerator name: `OPERATION_INTERSECTION`"]
    pub const INTERSECTION: PolyBooleanOperation = PolyBooleanOperation {
        ord: 2i32
    };
    #[doc(alias = "OPERATION_XOR")]
    #[doc = "Godot enumerator name: `OPERATION_XOR`"]
    pub const XOR: PolyBooleanOperation = PolyBooleanOperation {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for PolyBooleanOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolyBooleanOperation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolyBooleanOperation {
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
            Self::UNION => "UNION", Self::DIFFERENCE => "DIFFERENCE", Self::INTERSECTION => "INTERSECTION", Self::XOR => "XOR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UNION => "OPERATION_UNION", Self::DIFFERENCE => "OPERATION_DIFFERENCE", Self::INTERSECTION => "OPERATION_INTERSECTION", Self::XOR => "OPERATION_XOR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PolyBooleanOperation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PolyBooleanOperation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolyBooleanOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolyJoinType {
    ord: i32
}
impl PolyJoinType {
    #[doc(alias = "JOIN_SQUARE")]
    #[doc = "Godot enumerator name: `JOIN_SQUARE`"]
    pub const SQUARE: PolyJoinType = PolyJoinType {
        ord: 0i32
    };
    #[doc(alias = "JOIN_ROUND")]
    #[doc = "Godot enumerator name: `JOIN_ROUND`"]
    pub const ROUND: PolyJoinType = PolyJoinType {
        ord: 1i32
    };
    #[doc(alias = "JOIN_MITER")]
    #[doc = "Godot enumerator name: `JOIN_MITER`"]
    pub const MITER: PolyJoinType = PolyJoinType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PolyJoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolyJoinType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolyJoinType {
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
            Self::SQUARE => "SQUARE", Self::ROUND => "ROUND", Self::MITER => "MITER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SQUARE => "JOIN_SQUARE", Self::ROUND => "JOIN_ROUND", Self::MITER => "JOIN_MITER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PolyJoinType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PolyJoinType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolyJoinType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolyEndType {
    ord: i32
}
impl PolyEndType {
    #[doc(alias = "END_POLYGON")]
    #[doc = "Godot enumerator name: `END_POLYGON`"]
    pub const POLYGON: PolyEndType = PolyEndType {
        ord: 0i32
    };
    #[doc(alias = "END_JOINED")]
    #[doc = "Godot enumerator name: `END_JOINED`"]
    pub const JOINED: PolyEndType = PolyEndType {
        ord: 1i32
    };
    #[doc(alias = "END_BUTT")]
    #[doc = "Godot enumerator name: `END_BUTT`"]
    pub const BUTT: PolyEndType = PolyEndType {
        ord: 2i32
    };
    #[doc(alias = "END_SQUARE")]
    #[doc = "Godot enumerator name: `END_SQUARE`"]
    pub const SQUARE: PolyEndType = PolyEndType {
        ord: 3i32
    };
    #[doc(alias = "END_ROUND")]
    #[doc = "Godot enumerator name: `END_ROUND`"]
    pub const ROUND: PolyEndType = PolyEndType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for PolyEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolyEndType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolyEndType {
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
            Self::POLYGON => "POLYGON", Self::JOINED => "JOINED", Self::BUTT => "BUTT", Self::SQUARE => "SQUARE", Self::ROUND => "ROUND", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::POLYGON => "END_POLYGON", Self::JOINED => "END_JOINED", Self::BUTT => "END_BUTT", Self::SQUARE => "END_SQUARE", Self::ROUND => "END_ROUND", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PolyEndType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PolyEndType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolyEndType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}