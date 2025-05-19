#![doc = "Sidecar module for class [`ParticleProcessMaterial`][crate::classes::ParticleProcessMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ParticleProcessMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ParticleProcessMaterial.`\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`particle_process_material`][crate::classes::particle_process_material]: sidecar module with related enum/flag types\n* [`IParticleProcessMaterial`][crate::classes::IParticleProcessMaterial]: virtual methods\n\n\nSee also [Godot docs for `ParticleProcessMaterial`](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ParticleProcessMaterial::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ParticleProcessMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ParticleProcessMaterial`][crate::classes::ParticleProcessMaterial].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ParticleProcessMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IParticleProcessMaterial: crate::obj::GodotClass < Base = ParticleProcessMaterial > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_shader_rid(&self,) -> Rid;
        fn get_shader_mode(&self,) -> crate::classes::shader::Mode;
        fn can_do_next_pass(&self,) -> bool {
            unimplemented !()
        }
        fn can_use_render_priority(&self,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl ParticleProcessMaterial {
        pub fn set_direction(&mut self, degrees: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_direction(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inherit_velocity_ratio(&mut self, ratio: f64,) {
            type CallSig = ((), f64);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_inherit_velocity_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inherit_velocity_ratio(&mut self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_inherit_velocity_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spread(&mut self, degrees: f32,) {
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spread(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flatness(&mut self, amount: f32,) {
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_flatness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flatness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_flatness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param(&mut self, param: crate::classes::particle_process_material::Parameter, value: Vector2,) {
            type CallSig = ((), crate::classes::particle_process_material::Parameter, Vector2);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param(&self, param: crate::classes::particle_process_material::Parameter,) -> Vector2 {
            type CallSig = (Vector2, crate::classes::particle_process_material::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_min(&mut self, param: crate::classes::particle_process_material::Parameter, value: f32,) {
            type CallSig = ((), crate::classes::particle_process_material::Parameter, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_min(&self, param: crate::classes::particle_process_material::Parameter,) -> f32 {
            type CallSig = (f32, crate::classes::particle_process_material::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_max(&mut self, param: crate::classes::particle_process_material::Parameter, value: f32,) {
            type CallSig = ((), crate::classes::particle_process_material::Parameter, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_max(&self, param: crate::classes::particle_process_material::Parameter,) -> f32 {
            type CallSig = (f32, crate::classes::particle_process_material::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_texture(&mut self, param: crate::classes::particle_process_material::Parameter, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), crate::classes::particle_process_material::Parameter, ObjectArg < crate::classes::Texture2D >);
            let args = (param, texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_param_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_texture(&self, param: crate::classes::particle_process_material::Parameter,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, crate::classes::particle_process_material::Parameter);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_param_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_ramp(&mut self, ramp: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (ramp.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_curve(&mut self, curve: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (curve.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_alpha_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_curve(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_alpha_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_curve(&mut self, curve: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (curve.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_curve(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_initial_ramp(&mut self, ramp: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (ramp.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_initial_ramp(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_velocity_limit_curve(&mut self, curve: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (curve.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_velocity_limit_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_limit_curve(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_velocity_limit_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particle_flag(&mut self, particle_flag: crate::classes::particle_process_material::ParticleFlags, enable: bool,) {
            type CallSig = ((), crate::classes::particle_process_material::ParticleFlags, bool);
            let args = (particle_flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particle_flag(&self, particle_flag: crate::classes::particle_process_material::ParticleFlags,) -> bool {
            type CallSig = (bool, crate::classes::particle_process_material::ParticleFlags);
            let args = (particle_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_velocity_pivot(&mut self, pivot: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (pivot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_velocity_pivot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_pivot(&mut self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_velocity_pivot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape(&mut self, shape: crate::classes::particle_process_material::EmissionShape,) {
            type CallSig = ((), crate::classes::particle_process_material::EmissionShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape(&self,) -> crate::classes::particle_process_material::EmissionShape {
            type CallSig = (crate::classes::particle_process_material::EmissionShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_sphere_radius(&mut self, radius: f32,) {
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_sphere_radius(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_box_extents(&mut self, extents: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_box_extents(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_point_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_point_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_point_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_point_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_normal_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_normal_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_normal_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_normal_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_color_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_color_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_color_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_color_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_point_count(&mut self, point_count: i32,) {
            type CallSig = ((), i32);
            let args = (point_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_point_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_axis(&mut self, axis: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_axis(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_height(&mut self, height: f32,) {
            type CallSig = ((), f32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_height(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_radius(&mut self, radius: f32,) {
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_radius(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_inner_radius(&mut self, inner_radius: f32,) {
            type CallSig = ((), f32);
            let args = (inner_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_inner_radius(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape_offset(&mut self, emission_shape_offset: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (emission_shape_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_shape_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape_offset(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_shape_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape_scale(&mut self, emission_shape_scale: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (emission_shape_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_shape_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape_scale(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_shape_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_enabled(&mut self, turbulence_enabled: bool,) {
            type CallSig = ((), bool);
            let args = (turbulence_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_noise_strength(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_noise_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_noise_strength(&mut self, turbulence_noise_strength: f32,) {
            type CallSig = ((), f32);
            let args = (turbulence_noise_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_noise_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_noise_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_noise_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_noise_scale(&mut self, turbulence_noise_scale: f32,) {
            type CallSig = ((), f32);
            let args = (turbulence_noise_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_noise_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_noise_speed_random(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_noise_speed_random", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_noise_speed_random(&mut self, turbulence_noise_speed_random: f32,) {
            type CallSig = ((), f32);
            let args = (turbulence_noise_speed_random,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_noise_speed_random", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbulence_noise_speed(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_noise_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbulence_noise_speed(&mut self, turbulence_noise_speed: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (turbulence_noise_speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_noise_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, accel_vec: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (accel_vec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime_randomness(&mut self, randomness: f64,) {
            type CallSig = ((), f64);
            let args = (randomness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime_randomness(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_mode(&self,) -> crate::classes::particle_process_material::SubEmitterMode {
            type CallSig = (crate::classes::particle_process_material::SubEmitterMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_mode(&mut self, mode: crate::classes::particle_process_material::SubEmitterMode,) {
            type CallSig = ((), crate::classes::particle_process_material::SubEmitterMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_frequency(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_frequency(&mut self, hz: f64,) {
            type CallSig = ((), f64);
            let args = (hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_amount_at_end(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_amount_at_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_amount_at_end(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_amount_at_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_amount_at_collision(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_amount_at_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_amount_at_collision(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_amount_at_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sub_emitter_keep_velocity(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_keep_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sub_emitter_keep_velocity(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_keep_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attractor_interaction_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_attractor_interaction_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_attractor_interaction_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "is_attractor_interaction_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mode(&mut self, mode: crate::classes::particle_process_material::CollisionMode,) {
            type CallSig = ((), crate::classes::particle_process_material::CollisionMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_collision_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mode(&self,) -> crate::classes::particle_process_material::CollisionMode {
            type CallSig = (crate::classes::particle_process_material::CollisionMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_collision_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_use_scale(&mut self, radius: bool,) {
            type CallSig = ((), bool);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_collision_use_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collision_using_scale(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "is_collision_using_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_friction(&mut self, friction: f32,) {
            type CallSig = ((), f32);
            let args = (friction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_collision_friction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_friction(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_collision_friction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_bounce(&mut self, bounce: f32,) {
            type CallSig = ((), f32);
            let args = (bounce,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_collision_bounce", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_bounce(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_collision_bounce", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ParticleProcessMaterial {
        type Base = crate::classes::Material;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ParticleProcessMaterial"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ParticleProcessMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for ParticleProcessMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ParticleProcessMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ParticleProcessMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ParticleProcessMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for ParticleProcessMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ParticleProcessMaterial {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ParticleProcessMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ParticleProcessMaterial`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ParticleProcessMaterial {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ParticleProcessMaterial > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Material > for $Class {
                
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
    #[doc(alias = "PARAM_RADIAL_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_RADIAL_VELOCITY`"]
    pub const RADIAL_VELOCITY: Parameter = Parameter {
        ord: 15i32
    };
    #[doc(alias = "PARAM_DIRECTIONAL_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_DIRECTIONAL_VELOCITY`"]
    pub const DIRECTIONAL_VELOCITY: Parameter = Parameter {
        ord: 16i32
    };
    #[doc(alias = "PARAM_SCALE_OVER_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_SCALE_OVER_VELOCITY`"]
    pub const SCALE_OVER_VELOCITY: Parameter = Parameter {
        ord: 17i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Parameter = Parameter {
        ord: 18i32
    };
    #[doc(alias = "PARAM_TURB_VEL_INFLUENCE")]
    #[doc = "Godot enumerator name: `PARAM_TURB_VEL_INFLUENCE`"]
    pub const TURB_VEL_INFLUENCE: Parameter = Parameter {
        ord: 13i32
    };
    #[doc(alias = "PARAM_TURB_INIT_DISPLACEMENT")]
    #[doc = "Godot enumerator name: `PARAM_TURB_INIT_DISPLACEMENT`"]
    pub const TURB_INIT_DISPLACEMENT: Parameter = Parameter {
        ord: 14i32
    };
    #[doc(alias = "PARAM_TURB_INFLUENCE_OVER_LIFE")]
    #[doc = "Godot enumerator name: `PARAM_TURB_INFLUENCE_OVER_LIFE`"]
    pub const TURB_INFLUENCE_OVER_LIFE: Parameter = Parameter {
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 => Some(Self {
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
            Self::INITIAL_LINEAR_VELOCITY => "INITIAL_LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "ANGULAR_VELOCITY", Self::ORBIT_VELOCITY => "ORBIT_VELOCITY", Self::LINEAR_ACCEL => "LINEAR_ACCEL", Self::RADIAL_ACCEL => "RADIAL_ACCEL", Self::TANGENTIAL_ACCEL => "TANGENTIAL_ACCEL", Self::DAMPING => "DAMPING", Self::ANGLE => "ANGLE", Self::SCALE => "SCALE", Self::HUE_VARIATION => "HUE_VARIATION", Self::ANIM_SPEED => "ANIM_SPEED", Self::ANIM_OFFSET => "ANIM_OFFSET", Self::RADIAL_VELOCITY => "RADIAL_VELOCITY", Self::DIRECTIONAL_VELOCITY => "DIRECTIONAL_VELOCITY", Self::SCALE_OVER_VELOCITY => "SCALE_OVER_VELOCITY", Self::MAX => "MAX", Self::TURB_VEL_INFLUENCE => "TURB_VEL_INFLUENCE", Self::TURB_INIT_DISPLACEMENT => "TURB_INIT_DISPLACEMENT", Self::TURB_INFLUENCE_OVER_LIFE => "TURB_INFLUENCE_OVER_LIFE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INITIAL_LINEAR_VELOCITY => "PARAM_INITIAL_LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "PARAM_ANGULAR_VELOCITY", Self::ORBIT_VELOCITY => "PARAM_ORBIT_VELOCITY", Self::LINEAR_ACCEL => "PARAM_LINEAR_ACCEL", Self::RADIAL_ACCEL => "PARAM_RADIAL_ACCEL", Self::TANGENTIAL_ACCEL => "PARAM_TANGENTIAL_ACCEL", Self::DAMPING => "PARAM_DAMPING", Self::ANGLE => "PARAM_ANGLE", Self::SCALE => "PARAM_SCALE", Self::HUE_VARIATION => "PARAM_HUE_VARIATION", Self::ANIM_SPEED => "PARAM_ANIM_SPEED", Self::ANIM_OFFSET => "PARAM_ANIM_OFFSET", Self::RADIAL_VELOCITY => "PARAM_RADIAL_VELOCITY", Self::DIRECTIONAL_VELOCITY => "PARAM_DIRECTIONAL_VELOCITY", Self::SCALE_OVER_VELOCITY => "PARAM_SCALE_OVER_VELOCITY", Self::MAX => "PARAM_MAX", Self::TURB_VEL_INFLUENCE => "PARAM_TURB_VEL_INFLUENCE", Self::TURB_INIT_DISPLACEMENT => "PARAM_TURB_INIT_DISPLACEMENT", Self::TURB_INFLUENCE_OVER_LIFE => "PARAM_TURB_INFLUENCE_OVER_LIFE", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Parameter {
    const ENUMERATOR_COUNT: usize = 18usize;
    
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
    #[doc(alias = "PARTICLE_FLAG_DAMPING_AS_FRICTION")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_DAMPING_AS_FRICTION`"]
    pub const DAMPING_AS_FRICTION: ParticleFlags = ParticleFlags {
        ord: 3i32
    };
    #[doc(alias = "PARTICLE_FLAG_MAX")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_MAX`"]
    pub const MAX: ParticleFlags = ParticleFlags {
        ord: 4i32
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
            Self::ALIGN_Y_TO_VELOCITY => "ALIGN_Y_TO_VELOCITY", Self::ROTATE_Y => "ROTATE_Y", Self::DISABLE_Z => "DISABLE_Z", Self::DAMPING_AS_FRICTION => "DAMPING_AS_FRICTION", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ALIGN_Y_TO_VELOCITY => "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY", Self::ROTATE_Y => "PARTICLE_FLAG_ROTATE_Y", Self::DISABLE_Z => "PARTICLE_FLAG_DISABLE_Z", Self::DAMPING_AS_FRICTION => "PARTICLE_FLAG_DAMPING_AS_FRICTION", Self::MAX => "PARTICLE_FLAG_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for ParticleFlags {
    const ENUMERATOR_COUNT: usize = 4usize;
    
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
    #[doc(alias = "EMISSION_SHAPE_BOX")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_BOX`"]
    pub const BOX: EmissionShape = EmissionShape {
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
    #[doc(alias = "EMISSION_SHAPE_RING")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_RING`"]
    pub const RING: EmissionShape = EmissionShape {
        ord: 6i32
    };
    #[doc(alias = "EMISSION_SHAPE_MAX")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_MAX`"]
    pub const MAX: EmissionShape = EmissionShape {
        ord: 7i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::POINT => "POINT", Self::SPHERE => "SPHERE", Self::SPHERE_SURFACE => "SPHERE_SURFACE", Self::BOX => "BOX", Self::POINTS => "POINTS", Self::DIRECTED_POINTS => "DIRECTED_POINTS", Self::RING => "RING", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::POINT => "EMISSION_SHAPE_POINT", Self::SPHERE => "EMISSION_SHAPE_SPHERE", Self::SPHERE_SURFACE => "EMISSION_SHAPE_SPHERE_SURFACE", Self::BOX => "EMISSION_SHAPE_BOX", Self::POINTS => "EMISSION_SHAPE_POINTS", Self::DIRECTED_POINTS => "EMISSION_SHAPE_DIRECTED_POINTS", Self::RING => "EMISSION_SHAPE_RING", Self::MAX => "EMISSION_SHAPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for EmissionShape {
    const ENUMERATOR_COUNT: usize = 7usize;
    
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SubEmitterMode {
    ord: i32
}
impl SubEmitterMode {
    #[doc(alias = "SUB_EMITTER_DISABLED")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_DISABLED`"]
    pub const DISABLED: SubEmitterMode = SubEmitterMode {
        ord: 0i32
    };
    #[doc(alias = "SUB_EMITTER_CONSTANT")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_CONSTANT`"]
    pub const CONSTANT: SubEmitterMode = SubEmitterMode {
        ord: 1i32
    };
    #[doc(alias = "SUB_EMITTER_AT_END")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_AT_END`"]
    pub const AT_END: SubEmitterMode = SubEmitterMode {
        ord: 2i32
    };
    #[doc(alias = "SUB_EMITTER_AT_COLLISION")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_AT_COLLISION`"]
    pub const AT_COLLISION: SubEmitterMode = SubEmitterMode {
        ord: 3i32
    };
    #[doc(alias = "SUB_EMITTER_MAX")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_MAX`"]
    pub const MAX: SubEmitterMode = SubEmitterMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SubEmitterMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SubEmitterMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SubEmitterMode {
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
            Self::DISABLED => "DISABLED", Self::CONSTANT => "CONSTANT", Self::AT_END => "AT_END", Self::AT_COLLISION => "AT_COLLISION", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "SUB_EMITTER_DISABLED", Self::CONSTANT => "SUB_EMITTER_CONSTANT", Self::AT_END => "SUB_EMITTER_AT_END", Self::AT_COLLISION => "SUB_EMITTER_AT_COLLISION", Self::MAX => "SUB_EMITTER_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for SubEmitterMode {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for SubEmitterMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SubEmitterMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SubEmitterMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CollisionMode {
    ord: i32
}
impl CollisionMode {
    #[doc(alias = "COLLISION_DISABLED")]
    #[doc = "Godot enumerator name: `COLLISION_DISABLED`"]
    pub const DISABLED: CollisionMode = CollisionMode {
        ord: 0i32
    };
    #[doc(alias = "COLLISION_RIGID")]
    #[doc = "Godot enumerator name: `COLLISION_RIGID`"]
    pub const RIGID: CollisionMode = CollisionMode {
        ord: 1i32
    };
    #[doc(alias = "COLLISION_HIDE_ON_CONTACT")]
    #[doc = "Godot enumerator name: `COLLISION_HIDE_ON_CONTACT`"]
    pub const HIDE_ON_CONTACT: CollisionMode = CollisionMode {
        ord: 2i32
    };
    #[doc(alias = "COLLISION_MAX")]
    #[doc = "Godot enumerator name: `COLLISION_MAX`"]
    pub const MAX: CollisionMode = CollisionMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CollisionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CollisionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CollisionMode {
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
            Self::DISABLED => "DISABLED", Self::RIGID => "RIGID", Self::HIDE_ON_CONTACT => "HIDE_ON_CONTACT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "COLLISION_DISABLED", Self::RIGID => "COLLISION_RIGID", Self::HIDE_ON_CONTACT => "COLLISION_HIDE_ON_CONTACT", Self::MAX => "COLLISION_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for CollisionMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for CollisionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CollisionMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CollisionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}