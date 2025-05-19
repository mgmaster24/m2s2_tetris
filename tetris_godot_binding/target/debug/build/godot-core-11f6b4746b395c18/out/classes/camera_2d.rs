#![doc = "Sidecar module for class [`Camera2D`][crate::classes::Camera2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Camera2D` enums](https://docs.godotengine.org/en/stable/classes/class_camera2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Camera2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`camera_2d`][crate::classes::camera_2d]: sidecar module with related enum/flag types\n* [`ICamera2D`][crate::classes::ICamera2D]: virtual methods\n\n\nSee also [Godot docs for `Camera2D`](https://docs.godotengine.org/en/stable/classes/class_camera2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Camera2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Camera2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Camera2D`][crate::classes::Camera2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Camera2D` methods](https://docs.godotengine.org/en/stable/classes/class_camera2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICamera2D: crate::obj::GodotClass < Base = Camera2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Camera2D {
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_anchor_mode(&mut self, anchor_mode: crate::classes::camera_2d::AnchorMode,) {
            type CallSig = ((), crate::classes::camera_2d::AnchorMode);
            let args = (anchor_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_anchor_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_anchor_mode(&self,) -> crate::classes::camera_2d::AnchorMode {
            type CallSig = (crate::classes::camera_2d::AnchorMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_anchor_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ignore_rotation(&mut self, ignore: bool,) {
            type CallSig = ((), bool);
            let args = (ignore,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_ignore_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ignoring_rotation(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_ignoring_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_callback(&mut self, mode: crate::classes::camera_2d::Camera2DProcessCallback,) {
            type CallSig = ((), crate::classes::camera_2d::Camera2DProcessCallback);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_process_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_callback(&self,) -> crate::classes::camera_2d::Camera2DProcessCallback {
            type CallSig = (crate::classes::camera_2d::Camera2DProcessCallback,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_process_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_current(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "make_current", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_current(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_current", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_limit(&mut self, margin: crate::global::Side, limit: i32,) {
            type CallSig = ((), crate::global::Side, i32);
            let args = (margin, limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_limit(&self, margin: crate::global::Side,) -> i32 {
            type CallSig = (i32, crate::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_limit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_limit_smoothing_enabled(&mut self, limit_smoothing_enabled: bool,) {
            type CallSig = ((), bool);
            let args = (limit_smoothing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit_smoothing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_limit_smoothing_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_limit_smoothing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_vertical_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_vertical_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drag_vertical_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_drag_vertical_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_horizontal_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_horizontal_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drag_horizontal_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_drag_horizontal_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_vertical_offset(&mut self, offset: f32,) {
            type CallSig = ((), f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_vertical_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drag_vertical_offset(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_vertical_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_horizontal_offset(&mut self, offset: f32,) {
            type CallSig = ((), f32);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_horizontal_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drag_horizontal_offset(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_horizontal_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_margin(&mut self, margin: crate::global::Side, drag_margin: f32,) {
            type CallSig = ((), crate::global::Side, f32);
            let args = (margin, drag_margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drag_margin(&self, margin: crate::global::Side,) -> f32 {
            type CallSig = (f32, crate::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_target_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_center_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_screen_center_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_zoom(&mut self, zoom: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (zoom,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_zoom", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_zoom(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_zoom", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_viewport(&mut self, viewport: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (viewport.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_custom_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_viewport(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_custom_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_position_smoothing_speed(&mut self, position_smoothing_speed: f32,) {
            type CallSig = ((), f32);
            let args = (position_smoothing_speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_position_smoothing_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position_smoothing_speed(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_position_smoothing_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_position_smoothing_enabled(&mut self, position_smoothing_speed: bool,) {
            type CallSig = ((), bool);
            let args = (position_smoothing_speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_position_smoothing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_position_smoothing_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_position_smoothing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_smoothing_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_rotation_smoothing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_rotation_smoothing_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_rotation_smoothing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_smoothing_speed(&mut self, speed: f32,) {
            type CallSig = ((), f32);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_rotation_smoothing_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_smoothing_speed(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "get_rotation_smoothing_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_scroll(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "force_update_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset_smoothing(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "reset_smoothing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn align(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_screen_drawing_enabled(&mut self, screen_drawing_enabled: bool,) {
            type CallSig = ((), bool);
            let args = (screen_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_screen_drawing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_screen_drawing_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_screen_drawing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_limit_drawing_enabled(&mut self, limit_drawing_enabled: bool,) {
            type CallSig = ((), bool);
            let args = (limit_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit_drawing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_limit_drawing_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_limit_drawing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_margin_drawing_enabled(&mut self, margin_drawing_enabled: bool,) {
            type CallSig = ((), bool);
            let args = (margin_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "set_margin_drawing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_margin_drawing_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Camera2D", "is_margin_drawing_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Camera2D {
        type Base = crate::classes::Node2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Camera2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Camera2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Camera2D {
        
    }
    impl crate::obj::cap::GodotDefault for Camera2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Camera2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Camera2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Camera2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Camera2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Camera2D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnchorMode {
    ord: i32
}
impl AnchorMode {
    #[doc(alias = "ANCHOR_MODE_FIXED_TOP_LEFT")]
    #[doc = "Godot enumerator name: `ANCHOR_MODE_FIXED_TOP_LEFT`"]
    pub const FIXED_TOP_LEFT: AnchorMode = AnchorMode {
        ord: 0i32
    };
    #[doc(alias = "ANCHOR_MODE_DRAG_CENTER")]
    #[doc = "Godot enumerator name: `ANCHOR_MODE_DRAG_CENTER`"]
    pub const DRAG_CENTER: AnchorMode = AnchorMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AnchorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnchorMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnchorMode {
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
            Self::FIXED_TOP_LEFT => "FIXED_TOP_LEFT", Self::DRAG_CENTER => "DRAG_CENTER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::FIXED_TOP_LEFT => "ANCHOR_MODE_FIXED_TOP_LEFT", Self::DRAG_CENTER => "ANCHOR_MODE_DRAG_CENTER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AnchorMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnchorMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnchorMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Camera2DProcessCallback {
    ord: i32
}
impl Camera2DProcessCallback {
    #[doc(alias = "CAMERA2D_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `CAMERA2D_PROCESS_PHYSICS`"]
    pub const PHYSICS: Camera2DProcessCallback = Camera2DProcessCallback {
        ord: 0i32
    };
    #[doc(alias = "CAMERA2D_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `CAMERA2D_PROCESS_IDLE`"]
    pub const IDLE: Camera2DProcessCallback = Camera2DProcessCallback {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Camera2DProcessCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Camera2DProcessCallback") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Camera2DProcessCallback {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PHYSICS => "CAMERA2D_PROCESS_PHYSICS", Self::IDLE => "CAMERA2D_PROCESS_IDLE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Camera2DProcessCallback {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Camera2DProcessCallback {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Camera2DProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}