#![doc = "Sidecar module for class [`CpuParticles2D`][crate::classes::CpuParticles2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CPUParticles2D` enums](https://docs.godotengine.org/en/stable/classes/class_cpuparticles2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CPUParticles2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`cpu_particles_2d`][crate::classes::cpu_particles_2d]: sidecar module with related enum/flag types\n* [`ICpuParticles2D`][crate::classes::ICpuParticles2D]: virtual methods\n\n\nSee also [Godot docs for `CPUParticles2D`](https://docs.godotengine.org/en/stable/classes/class_cpuparticles2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CpuParticles2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CpuParticles2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CpuParticles2D`][crate::classes::CpuParticles2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CPUParticles2D` methods](https://docs.godotengine.org/en/stable/classes/class_cpuparticles2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICpuParticles2D: crate::obj::GodotClass < Base = CpuParticles2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CpuParticles2D {
        pub fn set_emitting(&mut self, emitting: bool,) {
            type CallSig = ((), bool);
            let args = (emitting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime(&mut self, secs: f64,) {
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64,) {
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime_randomness(&mut self, random: f64,) {
            type CallSig = ((), f64);
            let args = (random,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i32,) {
            type CallSig = ((), i32);
            let args = (fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64,) {
            type CallSig = ((), f64);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emitting(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "is_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pre_process_time(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_explosiveness_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_randomness_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime_randomness(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_local_coordinates(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_fps(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractional_delta(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_order(&mut self, order: crate::classes::cpu_particles_2d::DrawOrder,) {
            type CallSig = ((), crate::classes::cpu_particles_2d::DrawOrder);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_order(&self,) -> crate::classes::cpu_particles_2d::DrawOrder {
            type CallSig = (crate::classes::cpu_particles_2d::DrawOrder,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn restart(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "restart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_direction(&mut self, direction: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_direction(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spread(&mut self, spread: f32,) {
            type CallSig = ((), f32);
            let args = (spread,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spread(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_min(&mut self, param: crate::classes::cpu_particles_2d::Parameter, value: f32,) {
            type CallSig = ((), crate::classes::cpu_particles_2d::Parameter, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_min(&self, param: crate::classes::cpu_particles_2d::Parameter,) -> f32 {
            type CallSig = (f32, crate::classes::cpu_particles_2d::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_max(&mut self, param: crate::classes::cpu_particles_2d::Parameter, value: f32,) {
            type CallSig = ((), crate::classes::cpu_particles_2d::Parameter, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_max(&self, param: crate::classes::cpu_particles_2d::Parameter,) -> f32 {
            type CallSig = (f32, crate::classes::cpu_particles_2d::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_curve(&mut self, param: crate::classes::cpu_particles_2d::Parameter, curve: impl AsObjectArg < crate::classes::Curve >,) {
            type CallSig = ((), crate::classes::cpu_particles_2d::Parameter, ObjectArg < crate::classes::Curve >);
            let args = (param, curve.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_param_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_curve(&self, param: crate::classes::cpu_particles_2d::Parameter,) -> Option < Gd < crate::classes::Curve > > {
            type CallSig = (Option < Gd < crate::classes::Curve > >, crate::classes::cpu_particles_2d::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_param_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_ramp(&mut self, ramp: impl AsObjectArg < crate::classes::Gradient >,) {
            type CallSig = ((), ObjectArg < crate::classes::Gradient >);
            let args = (ramp.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallSig = (Option < Gd < crate::classes::Gradient > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_initial_ramp(&mut self, ramp: impl AsObjectArg < crate::classes::Gradient >,) {
            type CallSig = ((), ObjectArg < crate::classes::Gradient >);
            let args = (ramp.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_initial_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallSig = (Option < Gd < crate::classes::Gradient > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particle_flag(&mut self, particle_flag: crate::classes::cpu_particles_2d::ParticleFlags, enable: bool,) {
            type CallSig = ((), crate::classes::cpu_particles_2d::ParticleFlags, bool);
            let args = (particle_flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particle_flag(&self, particle_flag: crate::classes::cpu_particles_2d::ParticleFlags,) -> bool {
            type CallSig = (bool, crate::classes::cpu_particles_2d::ParticleFlags);
            let args = (particle_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape(&mut self, shape: crate::classes::cpu_particles_2d::EmissionShape,) {
            type CallSig = ((), crate::classes::cpu_particles_2d::EmissionShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape(&self,) -> crate::classes::cpu_particles_2d::EmissionShape {
            type CallSig = (crate::classes::cpu_particles_2d::EmissionShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_sphere_radius(&mut self, radius: f32,) {
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_sphere_radius(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_rect_extents(&mut self, extents: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_rect_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_rect_extents(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_rect_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_points(&mut self, array: &PackedVector2Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_points(&self,) -> PackedVector2Array {
            type CallSig = (PackedVector2Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_normals(&mut self, array: &PackedVector2Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedVector2Array >);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_normals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_normals(&self,) -> PackedVector2Array {
            type CallSig = (PackedVector2Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_normals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_colors(&mut self, array: &PackedColorArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedColorArray >);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_colors(&self,) -> PackedColorArray {
            type CallSig = (PackedColorArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, accel_vec: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (accel_vec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_split_scale(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_split_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_split_scale(&mut self, split_scale: bool,) {
            type CallSig = ((), bool);
            let args = (split_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_split_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_curve_x(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallSig = (Option < Gd < crate::classes::Curve > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_scale_curve_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_curve_x(&mut self, scale_curve: impl AsObjectArg < crate::classes::Curve >,) {
            type CallSig = ((), ObjectArg < crate::classes::Curve >);
            let args = (scale_curve.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_scale_curve_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_curve_y(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallSig = (Option < Gd < crate::classes::Curve > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_scale_curve_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_curve_y(&mut self, scale_curve: impl AsObjectArg < crate::classes::Curve >,) {
            type CallSig = ((), ObjectArg < crate::classes::Curve >);
            let args = (scale_curve.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_scale_curve_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert_from_particles(&mut self, particles: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (particles.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CpuParticles2D", "convert_from_particles", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CpuParticles2D {
        type Base = crate::classes::Node2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"CPUParticles2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CpuParticles2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for CpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for CpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CpuParticles2D {
        
    }
    impl crate::obj::cap::GodotDefault for CpuParticles2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CpuParticles2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CpuParticles2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CpuParticles2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_CpuParticles2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CpuParticles2D > for $Class {
                
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
pub struct DrawOrder {
    ord: i32
}
impl DrawOrder {
    #[doc(alias = "DRAW_ORDER_INDEX")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_INDEX`"]
    pub const INDEX: DrawOrder = DrawOrder {
        ord: 0i32
    };
    #[doc(alias = "DRAW_ORDER_LIFETIME")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_LIFETIME`"]
    pub const LIFETIME: DrawOrder = DrawOrder {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DrawOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawOrder") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawOrder {
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
            Self::INDEX => "INDEX", Self::LIFETIME => "LIFETIME", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INDEX => "DRAW_ORDER_INDEX", Self::LIFETIME => "DRAW_ORDER_LIFETIME", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DrawOrder {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DrawOrder {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawOrder {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Parameter {
    ord: i32
}
impl Parameter {
    #[doc(alias = "PARAM_INITIAL_LINEAR_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_INITIAL_LINEAR_VELOCITY`"]
    pub const INITIAL_LINEAR_VELOCITY: Parameter = Parameter {
        ord: 0i32
    };
    #[doc(alias = "PARAM_ANGULAR_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_VELOCITY`"]
    pub const ANGULAR_VELOCITY: Parameter = Parameter {
        ord: 1i32
    };
    #[doc(alias = "PARAM_ORBIT_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ORBIT_VELOCITY`"]
    pub const ORBIT_VELOCITY: Parameter = Parameter {
        ord: 2i32
    };
    #[doc(alias = "PARAM_LINEAR_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_ACCEL`"]
    pub const LINEAR_ACCEL: Parameter = Parameter {
        ord: 3i32
    };
    #[doc(alias = "PARAM_RADIAL_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_RADIAL_ACCEL`"]
    pub const RADIAL_ACCEL: Parameter = Parameter {
        ord: 4i32
    };
    #[doc(alias = "PARAM_TANGENTIAL_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_TANGENTIAL_ACCEL`"]
    pub const TANGENTIAL_ACCEL: Parameter = Parameter {
        ord: 5i32
    };
    #[doc(alias = "PARAM_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_DAMPING`"]
    pub const DAMPING: Parameter = Parameter {
        ord: 6i32
    };
    #[doc(alias = "PARAM_ANGLE")]
    #[doc = "Godot enumerator name: `PARAM_ANGLE`"]
    pub const ANGLE: Parameter = Parameter {
        ord: 7i32
    };
    #[doc(alias = "PARAM_SCALE")]
    #[doc = "Godot enumerator name: `PARAM_SCALE`"]
    pub const SCALE: Parameter = Parameter {
        ord: 8i32
    };
    #[doc(alias = "PARAM_HUE_VARIATION")]
    #[doc = "Godot enumerator name: `PARAM_HUE_VARIATION`"]
    pub const HUE_VARIATION: Parameter = Parameter {
        ord: 9i32
    };
    #[doc(alias = "PARAM_ANIM_SPEED")]
    #[doc = "Godot enumerator name: `PARAM_ANIM_SPEED`"]
    pub const ANIM_SPEED: Parameter = Parameter {
        ord: 10i32
    };
    #[doc(alias = "PARAM_ANIM_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_ANIM_OFFSET`"]
    pub const ANIM_OFFSET: Parameter = Parameter {
        ord: 11i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Parameter = Parameter {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Parameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Parameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
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
            Self::INITIAL_LINEAR_VELOCITY => "INITIAL_LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "ANGULAR_VELOCITY", Self::ORBIT_VELOCITY => "ORBIT_VELOCITY", Self::LINEAR_ACCEL => "LINEAR_ACCEL", Self::RADIAL_ACCEL => "RADIAL_ACCEL", Self::TANGENTIAL_ACCEL => "TANGENTIAL_ACCEL", Self::DAMPING => "DAMPING", Self::ANGLE => "ANGLE", Self::SCALE => "SCALE", Self::HUE_VARIATION => "HUE_VARIATION", Self::ANIM_SPEED => "ANIM_SPEED", Self::ANIM_OFFSET => "ANIM_OFFSET", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INITIAL_LINEAR_VELOCITY => "PARAM_INITIAL_LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "PARAM_ANGULAR_VELOCITY", Self::ORBIT_VELOCITY => "PARAM_ORBIT_VELOCITY", Self::LINEAR_ACCEL => "PARAM_LINEAR_ACCEL", Self::RADIAL_ACCEL => "PARAM_RADIAL_ACCEL", Self::TANGENTIAL_ACCEL => "PARAM_TANGENTIAL_ACCEL", Self::DAMPING => "PARAM_DAMPING", Self::ANGLE => "PARAM_ANGLE", Self::SCALE => "PARAM_SCALE", Self::HUE_VARIATION => "PARAM_HUE_VARIATION", Self::ANIM_SPEED => "PARAM_ANIM_SPEED", Self::ANIM_OFFSET => "PARAM_ANIM_OFFSET", Self::MAX => "PARAM_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Parameter {
    const ENUMERATOR_COUNT: usize = 12usize;
    
}
impl crate::meta::GodotConvert for Parameter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Parameter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Parameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticleFlags {
    ord: i32
}
impl ParticleFlags {
    #[doc(alias = "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY`"]
    pub const ALIGN_Y_TO_VELOCITY: ParticleFlags = ParticleFlags {
        ord: 0i32
    };
    #[doc(alias = "PARTICLE_FLAG_ROTATE_Y")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_ROTATE_Y`"]
    pub const ROTATE_Y: ParticleFlags = ParticleFlags {
        ord: 1i32
    };
    #[doc(alias = "PARTICLE_FLAG_DISABLE_Z")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_DISABLE_Z`"]
    pub const DISABLE_Z: ParticleFlags = ParticleFlags {
        ord: 2i32
    };
    #[doc(alias = "PARTICLE_FLAG_MAX")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_MAX`"]
    pub const MAX: ParticleFlags = ParticleFlags {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ParticleFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticleFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticleFlags {
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
            Self::ALIGN_Y_TO_VELOCITY => "ALIGN_Y_TO_VELOCITY", Self::ROTATE_Y => "ROTATE_Y", Self::DISABLE_Z => "DISABLE_Z", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ALIGN_Y_TO_VELOCITY => "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY", Self::ROTATE_Y => "PARTICLE_FLAG_ROTATE_Y", Self::DISABLE_Z => "PARTICLE_FLAG_DISABLE_Z", Self::MAX => "PARTICLE_FLAG_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for ParticleFlags {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ParticleFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ParticleFlags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticleFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EmissionShape {
    ord: i32
}
impl EmissionShape {
    #[doc(alias = "EMISSION_SHAPE_POINT")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_POINT`"]
    pub const POINT: EmissionShape = EmissionShape {
        ord: 0i32
    };
    #[doc(alias = "EMISSION_SHAPE_SPHERE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_SPHERE`"]
    pub const SPHERE: EmissionShape = EmissionShape {
        ord: 1i32
    };
    #[doc(alias = "EMISSION_SHAPE_SPHERE_SURFACE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_SPHERE_SURFACE`"]
    pub const SPHERE_SURFACE: EmissionShape = EmissionShape {
        ord: 2i32
    };
    #[doc(alias = "EMISSION_SHAPE_RECTANGLE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_RECTANGLE`"]
    pub const RECTANGLE: EmissionShape = EmissionShape {
        ord: 3i32
    };
    #[doc(alias = "EMISSION_SHAPE_POINTS")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_POINTS`"]
    pub const POINTS: EmissionShape = EmissionShape {
        ord: 4i32
    };
    #[doc(alias = "EMISSION_SHAPE_DIRECTED_POINTS")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_DIRECTED_POINTS`"]
    pub const DIRECTED_POINTS: EmissionShape = EmissionShape {
        ord: 5i32
    };
    #[doc(alias = "EMISSION_SHAPE_MAX")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_MAX`"]
    pub const MAX: EmissionShape = EmissionShape {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for EmissionShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EmissionShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EmissionShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::POINT => "POINT", Self::SPHERE => "SPHERE", Self::SPHERE_SURFACE => "SPHERE_SURFACE", Self::RECTANGLE => "RECTANGLE", Self::POINTS => "POINTS", Self::DIRECTED_POINTS => "DIRECTED_POINTS", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::POINT => "EMISSION_SHAPE_POINT", Self::SPHERE => "EMISSION_SHAPE_SPHERE", Self::SPHERE_SURFACE => "EMISSION_SHAPE_SPHERE_SURFACE", Self::RECTANGLE => "EMISSION_SHAPE_RECTANGLE", Self::POINTS => "EMISSION_SHAPE_POINTS", Self::DIRECTED_POINTS => "EMISSION_SHAPE_DIRECTED_POINTS", Self::MAX => "EMISSION_SHAPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for EmissionShape {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for EmissionShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EmissionShape {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EmissionShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}