#![doc = "Sidecar module for class [`GpuParticles3D`][crate::classes::GpuParticles3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GPUParticles3D` enums](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GPUParticles3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`gpu_particles_3d`][crate::classes::gpu_particles_3d]: sidecar module with related enum/flag types\n* [`IGpuParticles3D`][crate::classes::IGpuParticles3D]: virtual methods\n\n\nSee also [Godot docs for `GPUParticles3D`](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GpuParticles3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GpuParticles3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GpuParticles3D`][crate::classes::GpuParticles3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GPUParticles3D` methods](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGpuParticles3D: crate::obj::GodotClass < Base = GpuParticles3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GpuParticles3D {
        pub fn set_emitting(&mut self, emitting: bool,) {
            type CallSig = ((), bool);
            let args = (emitting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime(&mut self, secs: f64,) {
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64,) {
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_aabb(&mut self, aabb: Aabb,) {
            type CallSig = ((), Aabb);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_visibility_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i32,) {
            type CallSig = ((), i32);
            let args = (fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interpolate(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_material(&mut self, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), ObjectArg < crate::classes::Material >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_process_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64,) {
            type CallSig = ((), f64);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_base_size(&mut self, size: f32,) {
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_collision_base_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interp_to_end(&mut self, interp: f32,) {
            type CallSig = ((), f32);
            let args = (interp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_interp_to_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emitting(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "is_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pre_process_time(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_explosiveness_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_randomness_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_aabb(&self,) -> Aabb {
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_visibility_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_local_coordinates(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_fps(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractional_delta(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interpolate(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_interpolate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_process_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_base_size(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_collision_base_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interp_to_end(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_interp_to_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_order(&mut self, order: crate::classes::gpu_particles_3d::DrawOrder,) {
            type CallSig = ((), crate::classes::gpu_particles_3d::DrawOrder);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_order(&self,) -> crate::classes::gpu_particles_3d::DrawOrder {
            type CallSig = (crate::classes::gpu_particles_3d::DrawOrder,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_passes(&mut self, passes: i32,) {
            type CallSig = ((), i32);
            let args = (passes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_draw_passes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_pass_mesh(&mut self, pass: i32, mesh: impl AsObjectArg < crate::classes::Mesh >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Mesh >);
            let args = (pass, mesh.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_draw_pass_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_passes(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_draw_passes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_pass_mesh(&self, pass: i32,) -> Option < Gd < crate::classes::Mesh > > {
            type CallSig = (Option < Gd < crate::classes::Mesh > >, i32);
            let args = (pass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_draw_pass_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skin(&mut self, skin: impl AsObjectArg < crate::classes::Skin >,) {
            type CallSig = ((), ObjectArg < crate::classes::Skin >);
            let args = (skin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skin(&self,) -> Option < Gd < crate::classes::Skin > > {
            type CallSig = (Option < Gd < crate::classes::Skin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn restart(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "restart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capture_aabb(&self,) -> Aabb {
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "capture_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter(&mut self, path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_sub_emitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_sub_emitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn emit_particle(&mut self, xform: Transform3D, velocity: Vector3, color: Color, custom: Color, flags: u32,) {
            type CallSig = ((), Transform3D, Vector3, Color, Color, u32);
            let args = (xform, velocity, color, custom, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "emit_particle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trail_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_trail_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trail_lifetime(&mut self, secs: f64,) {
            type CallSig = ((), f64);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_trail_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_trail_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "is_trail_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_trail_lifetime(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_trail_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform_align(&mut self, align: crate::classes::gpu_particles_3d::TransformAlign,) {
            type CallSig = ((), crate::classes::gpu_particles_3d::TransformAlign);
            let args = (align,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_transform_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform_align(&self,) -> crate::classes::gpu_particles_3d::TransformAlign {
            type CallSig = (crate::classes::gpu_particles_3d::TransformAlign,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_transform_align", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert_from_particles(&mut self, particles: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (particles.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "convert_from_particles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_amount_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_amount_ratio", self.object_ptr, self.__checked_id(), args,)
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
        pub const MAX_DRAW_PASSES: i32 = 4i32;
        
    }
    impl crate::obj::GodotClass for GpuParticles3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GPUParticles3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GpuParticles3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for GpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for GpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GpuParticles3D {
        
    }
    impl crate::obj::cap::GodotDefault for GpuParticles3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GpuParticles3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GpuParticles3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GpuParticles3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GpuParticles3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GpuParticles3D > for $Class {
                
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
    #[doc(alias = "DRAW_ORDER_VIEW_DEPTH")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_VIEW_DEPTH`"]
    pub const VIEW_DEPTH: DrawOrder = DrawOrder {
        ord: 3i32
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
            Self::INDEX => "INDEX", Self::LIFETIME => "LIFETIME", Self::REVERSE_LIFETIME => "REVERSE_LIFETIME", Self::VIEW_DEPTH => "VIEW_DEPTH", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INDEX => "DRAW_ORDER_INDEX", Self::LIFETIME => "DRAW_ORDER_LIFETIME", Self::REVERSE_LIFETIME => "DRAW_ORDER_REVERSE_LIFETIME", Self::VIEW_DEPTH => "DRAW_ORDER_VIEW_DEPTH", _ => self.as_str(),
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransformAlign {
    ord: i32
}
impl TransformAlign {
    #[doc(alias = "TRANSFORM_ALIGN_DISABLED")]
    #[doc = "Godot enumerator name: `TRANSFORM_ALIGN_DISABLED`"]
    pub const DISABLED: TransformAlign = TransformAlign {
        ord: 0i32
    };
    #[doc(alias = "TRANSFORM_ALIGN_Z_BILLBOARD")]
    #[doc = "Godot enumerator name: `TRANSFORM_ALIGN_Z_BILLBOARD`"]
    pub const Z_BILLBOARD: TransformAlign = TransformAlign {
        ord: 1i32
    };
    #[doc(alias = "TRANSFORM_ALIGN_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `TRANSFORM_ALIGN_Y_TO_VELOCITY`"]
    pub const Y_TO_VELOCITY: TransformAlign = TransformAlign {
        ord: 2i32
    };
    #[doc(alias = "TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY`"]
    pub const Z_BILLBOARD_Y_TO_VELOCITY: TransformAlign = TransformAlign {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TransformAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransformAlign") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransformAlign {
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
            Self::DISABLED => "DISABLED", Self::Z_BILLBOARD => "Z_BILLBOARD", Self::Y_TO_VELOCITY => "Y_TO_VELOCITY", Self::Z_BILLBOARD_Y_TO_VELOCITY => "Z_BILLBOARD_Y_TO_VELOCITY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "TRANSFORM_ALIGN_DISABLED", Self::Z_BILLBOARD => "TRANSFORM_ALIGN_Z_BILLBOARD", Self::Y_TO_VELOCITY => "TRANSFORM_ALIGN_Y_TO_VELOCITY", Self::Z_BILLBOARD_Y_TO_VELOCITY => "TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TransformAlign {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransformAlign {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransformAlign {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}