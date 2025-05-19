#![doc = "Sidecar module for class [`GpuParticles2D`][crate::classes::GpuParticles2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GPUParticles2D` enums](https://docs.godotengine.org/en/stable/classes/class_gpuparticles2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GPUParticles2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`gpu_particles_2d`][crate::classes::gpu_particles_2d]: sidecar module with related enum/flag types\n* [`IGpuParticles2D`][crate::classes::IGpuParticles2D]: virtual methods\n\n\nSee also [Godot docs for `GPUParticles2D`](https://docs.godotengine.org/en/stable/classes/class_gpuparticles2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GpuParticles2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GpuParticles2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GpuParticles2D`][crate::classes::GpuParticles2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GPUParticles2D` methods](https://docs.godotengine.org/en/stable/classes/class_gpuparticles2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGpuParticles2D: crate::obj::GodotClass < Base = GpuParticles2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GpuParticles2D {
        pub fn set_emitting(&mut self, emitting: bool,) {
            type CallSig = ((), bool);
            let args = (emitting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime(&mut self, secs: f64,) {
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_one_shot(&mut self, secs: bool,) {
            type CallSig = ((), bool);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64,) {
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_rect(&mut self, visibility_rect: Rect2,) {
            type CallSig = ((), Rect2);
            let args = (visibility_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_visibility_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i32,) {
            type CallSig = ((), i32);
            let args = (fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interpolate(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_material(&mut self, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), ObjectArg < crate::classes::Material >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_process_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64,) {
            type CallSig = ((), f64);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_base_size(&mut self, size: f32,) {
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_collision_base_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interp_to_end(&mut self, interp: f32,) {
            type CallSig = ((), f32);
            let args = (interp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_interp_to_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emitting(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "is_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pre_process_time(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_explosiveness_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_randomness_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_rect(&self,) -> Rect2 {
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_visibility_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_local_coordinates(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_fps(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractional_delta(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interpolate(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_process_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_base_size(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_collision_base_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interp_to_end(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_interp_to_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_order(&mut self, order: crate::classes::gpu_particles_2d::DrawOrder,) {
            type CallSig = ((), crate::classes::gpu_particles_2d::DrawOrder);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_order(&self,) -> crate::classes::gpu_particles_2d::DrawOrder {
            type CallSig = (crate::classes::gpu_particles_2d::DrawOrder,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capture_rect(&self,) -> Rect2 {
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "capture_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn restart(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "restart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter(&mut self, path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_sub_emitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_sub_emitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn emit_particle(&mut self, xform: Transform2D, velocity: Vector2, color: Color, custom: Color, flags: u32,) {
            type CallSig = ((), Transform2D, Vector2, Color, Color, u32);
            let args = (xform, velocity, color, custom, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "emit_particle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trail_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_trail_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trail_lifetime(&mut self, secs: f64,) {
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_trail_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_trail_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "is_trail_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_trail_lifetime(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_trail_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trail_sections(&mut self, sections: i32,) {
            type CallSig = ((), i32);
            let args = (sections,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_trail_sections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_trail_sections(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_trail_sections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trail_section_subdivisions(&mut self, subdivisions: i32,) {
            type CallSig = ((), i32);
            let args = (subdivisions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_trail_section_subdivisions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_trail_section_subdivisions(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_trail_section_subdivisions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert_from_particles(&mut self, particles: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (particles.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "convert_from_particles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "set_amount_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles2D", "get_amount_ratio", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GpuParticles2D {
        type Base = crate::classes::Node2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GPUParticles2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GpuParticles2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for GpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for GpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GpuParticles2D {
        
    }
    impl crate::obj::cap::GodotDefault for GpuParticles2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GpuParticles2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GpuParticles2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GpuParticles2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GpuParticles2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GpuParticles2D > for $Class {
                
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
    #[doc(alias = "DRAW_ORDER_REVERSE_LIFETIME")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_REVERSE_LIFETIME`"]
    pub const REVERSE_LIFETIME: DrawOrder = DrawOrder {
        ord: 2i32
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
            Self::INDEX => "INDEX", Self::LIFETIME => "LIFETIME", Self::REVERSE_LIFETIME => "REVERSE_LIFETIME", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INDEX => "DRAW_ORDER_INDEX", Self::LIFETIME => "DRAW_ORDER_LIFETIME", Self::REVERSE_LIFETIME => "DRAW_ORDER_REVERSE_LIFETIME", _ => self.as_str(),
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
pub struct EmitFlags {
    ord: i32
}
impl EmitFlags {
    #[doc(alias = "EMIT_FLAG_POSITION")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_POSITION`"]
    pub const POSITION: EmitFlags = EmitFlags {
        ord: 1i32
    };
    #[doc(alias = "EMIT_FLAG_ROTATION_SCALE")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_ROTATION_SCALE`"]
    pub const ROTATION_SCALE: EmitFlags = EmitFlags {
        ord: 2i32
    };
    #[doc(alias = "EMIT_FLAG_VELOCITY")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_VELOCITY`"]
    pub const VELOCITY: EmitFlags = EmitFlags {
        ord: 4i32
    };
    #[doc(alias = "EMIT_FLAG_COLOR")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_COLOR`"]
    pub const COLOR: EmitFlags = EmitFlags {
        ord: 8i32
    };
    #[doc(alias = "EMIT_FLAG_CUSTOM")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_CUSTOM`"]
    pub const CUSTOM: EmitFlags = EmitFlags {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for EmitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EmitFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EmitFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 => Some(Self {
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
            Self::POSITION => "POSITION", Self::ROTATION_SCALE => "ROTATION_SCALE", Self::VELOCITY => "VELOCITY", Self::COLOR => "COLOR", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::POSITION => "EMIT_FLAG_POSITION", Self::ROTATION_SCALE => "EMIT_FLAG_ROTATION_SCALE", Self::VELOCITY => "EMIT_FLAG_VELOCITY", Self::COLOR => "EMIT_FLAG_COLOR", Self::CUSTOM => "EMIT_FLAG_CUSTOM", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for EmitFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EmitFlags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EmitFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}