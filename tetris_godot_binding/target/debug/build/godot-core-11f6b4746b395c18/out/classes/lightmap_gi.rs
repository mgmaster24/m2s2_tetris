#![doc = "Sidecar module for class [`LightmapGi`][crate::classes::LightmapGi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LightmapGI` enums](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LightmapGI.`\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`lightmap_gi`][crate::classes::lightmap_gi]: sidecar module with related enum/flag types\n* [`ILightmapGi`][crate::classes::ILightmapGi]: virtual methods\n\n\nSee also [Godot docs for `LightmapGI`](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`LightmapGi::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LightmapGi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`LightmapGi`][crate::classes::LightmapGi].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `LightmapGI` methods](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILightmapGi: crate::obj::GodotClass < Base = LightmapGi > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl LightmapGi {
        pub fn set_light_data(&mut self, data: impl AsObjectArg < crate::classes::LightmapGiData >,) {
            type CallSig = ((), ObjectArg < crate::classes::LightmapGiData >);
            let args = (data.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_light_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_light_data(&self,) -> Option < Gd < crate::classes::LightmapGiData > > {
            type CallSig = (Option < Gd < crate::classes::LightmapGiData > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_light_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_quality(&mut self, bake_quality: crate::classes::lightmap_gi::BakeQuality,) {
            type CallSig = ((), crate::classes::lightmap_gi::BakeQuality);
            let args = (bake_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_bake_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_quality(&self,) -> crate::classes::lightmap_gi::BakeQuality {
            type CallSig = (crate::classes::lightmap_gi::BakeQuality,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_bake_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bounces(&mut self, bounces: i32,) {
            type CallSig = ((), i32);
            let args = (bounces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bounces(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bounce_indirect_energy(&mut self, bounce_indirect_energy: f32,) {
            type CallSig = ((), f32);
            let args = (bounce_indirect_energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_bounce_indirect_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bounce_indirect_energy(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_bounce_indirect_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_generate_probes(&mut self, subdivision: crate::classes::lightmap_gi::GenerateProbes,) {
            type CallSig = ((), crate::classes::lightmap_gi::GenerateProbes);
            let args = (subdivision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_generate_probes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_generate_probes(&self,) -> crate::classes::lightmap_gi::GenerateProbes {
            type CallSig = (crate::classes::lightmap_gi::GenerateProbes,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_generate_probes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bias(&mut self, bias: f32,) {
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bias(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_mode(&mut self, mode: crate::classes::lightmap_gi::EnvironmentMode,) {
            type CallSig = ((), crate::classes::lightmap_gi::EnvironmentMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_environment_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_mode(&self,) -> crate::classes::lightmap_gi::EnvironmentMode {
            type CallSig = (crate::classes::lightmap_gi::EnvironmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_environment_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_custom_sky(&mut self, sky: impl AsObjectArg < crate::classes::Sky >,) {
            type CallSig = ((), ObjectArg < crate::classes::Sky >);
            let args = (sky.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_environment_custom_sky", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_custom_sky(&self,) -> Option < Gd < crate::classes::Sky > > {
            type CallSig = (Option < Gd < crate::classes::Sky > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_environment_custom_sky", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_custom_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_environment_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_custom_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_environment_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_custom_energy(&mut self, energy: f32,) {
            type CallSig = ((), f32);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_environment_custom_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_custom_energy(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_environment_custom_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texel_scale(&mut self, texel_scale: f32,) {
            type CallSig = ((), f32);
            let args = (texel_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_texel_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texel_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_texel_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_texture_size(&mut self, max_texture_size: i32,) {
            type CallSig = ((), i32);
            let args = (max_texture_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_max_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_texture_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_max_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_denoiser(&mut self, use_denoiser: bool,) {
            type CallSig = ((), bool);
            let args = (use_denoiser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_use_denoiser", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_denoiser(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "is_using_denoiser", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_denoiser_strength(&mut self, denoiser_strength: f32,) {
            type CallSig = ((), f32);
            let args = (denoiser_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_denoiser_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_denoiser_strength(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_denoiser_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_denoiser_range(&mut self, denoiser_range: i32,) {
            type CallSig = ((), i32);
            let args = (denoiser_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_denoiser_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_denoiser_range(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_denoiser_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interior(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_interior(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "is_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_directional(&mut self, directional: bool,) {
            type CallSig = ((), bool);
            let args = (directional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_directional", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_directional(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "is_directional", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_texture_for_bounces(&mut self, use_texture_for_bounces: bool,) {
            type CallSig = ((), bool);
            let args = (use_texture_for_bounces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_use_texture_for_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_texture_for_bounces(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "is_using_texture_for_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_camera_attributes(&mut self, camera_attributes: impl AsObjectArg < crate::classes::CameraAttributes >,) {
            type CallSig = ((), ObjectArg < crate::classes::CameraAttributes >);
            let args = (camera_attributes.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "set_camera_attributes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_attributes(&self,) -> Option < Gd < crate::classes::CameraAttributes > > {
            type CallSig = (Option < Gd < crate::classes::CameraAttributes > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "LightmapGi", "get_camera_attributes", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for LightmapGi {
        type Base = crate::classes::VisualInstance3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"LightmapGI"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LightmapGi {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for LightmapGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for LightmapGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for LightmapGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for LightmapGi {
        
    }
    impl crate::obj::cap::GodotDefault for LightmapGi {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LightmapGi {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LightmapGi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`LightmapGi`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_LightmapGi {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::LightmapGi > for $Class {
                
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
pub struct BakeQuality {
    ord: i32
}
impl BakeQuality {
    #[doc(alias = "BAKE_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `BAKE_QUALITY_LOW`"]
    pub const LOW: BakeQuality = BakeQuality {
        ord: 0i32
    };
    #[doc(alias = "BAKE_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `BAKE_QUALITY_MEDIUM`"]
    pub const MEDIUM: BakeQuality = BakeQuality {
        ord: 1i32
    };
    #[doc(alias = "BAKE_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `BAKE_QUALITY_HIGH`"]
    pub const HIGH: BakeQuality = BakeQuality {
        ord: 2i32
    };
    #[doc(alias = "BAKE_QUALITY_ULTRA")]
    #[doc = "Godot enumerator name: `BAKE_QUALITY_ULTRA`"]
    pub const ULTRA: BakeQuality = BakeQuality {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for BakeQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BakeQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BakeQuality {
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
            Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", Self::ULTRA => "ULTRA", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LOW => "BAKE_QUALITY_LOW", Self::MEDIUM => "BAKE_QUALITY_MEDIUM", Self::HIGH => "BAKE_QUALITY_HIGH", Self::ULTRA => "BAKE_QUALITY_ULTRA", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BakeQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BakeQuality {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BakeQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GenerateProbes {
    ord: i32
}
impl GenerateProbes {
    #[doc(alias = "GENERATE_PROBES_DISABLED")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_DISABLED`"]
    pub const DISABLED: GenerateProbes = GenerateProbes {
        ord: 0i32
    };
    #[doc(alias = "GENERATE_PROBES_SUBDIV_4")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_SUBDIV_4`"]
    pub const SUBDIV_4: GenerateProbes = GenerateProbes {
        ord: 1i32
    };
    #[doc(alias = "GENERATE_PROBES_SUBDIV_8")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_SUBDIV_8`"]
    pub const SUBDIV_8: GenerateProbes = GenerateProbes {
        ord: 2i32
    };
    #[doc(alias = "GENERATE_PROBES_SUBDIV_16")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_SUBDIV_16`"]
    pub const SUBDIV_16: GenerateProbes = GenerateProbes {
        ord: 3i32
    };
    #[doc(alias = "GENERATE_PROBES_SUBDIV_32")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_SUBDIV_32`"]
    pub const SUBDIV_32: GenerateProbes = GenerateProbes {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for GenerateProbes {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GenerateProbes") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GenerateProbes {
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
            Self::DISABLED => "DISABLED", Self::SUBDIV_4 => "SUBDIV_4", Self::SUBDIV_8 => "SUBDIV_8", Self::SUBDIV_16 => "SUBDIV_16", Self::SUBDIV_32 => "SUBDIV_32", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "GENERATE_PROBES_DISABLED", Self::SUBDIV_4 => "GENERATE_PROBES_SUBDIV_4", Self::SUBDIV_8 => "GENERATE_PROBES_SUBDIV_8", Self::SUBDIV_16 => "GENERATE_PROBES_SUBDIV_16", Self::SUBDIV_32 => "GENERATE_PROBES_SUBDIV_32", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for GenerateProbes {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GenerateProbes {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GenerateProbes {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BakeError {
    ord: i32
}
impl BakeError {
    #[doc(alias = "BAKE_ERROR_OK")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_OK`"]
    pub const OK: BakeError = BakeError {
        ord: 0i32
    };
    #[doc(alias = "BAKE_ERROR_NO_SCENE_ROOT")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_NO_SCENE_ROOT`"]
    pub const NO_SCENE_ROOT: BakeError = BakeError {
        ord: 1i32
    };
    #[doc(alias = "BAKE_ERROR_FOREIGN_DATA")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_FOREIGN_DATA`"]
    pub const FOREIGN_DATA: BakeError = BakeError {
        ord: 2i32
    };
    #[doc(alias = "BAKE_ERROR_NO_LIGHTMAPPER")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_NO_LIGHTMAPPER`"]
    pub const NO_LIGHTMAPPER: BakeError = BakeError {
        ord: 3i32
    };
    #[doc(alias = "BAKE_ERROR_NO_SAVE_PATH")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_NO_SAVE_PATH`"]
    pub const NO_SAVE_PATH: BakeError = BakeError {
        ord: 4i32
    };
    #[doc(alias = "BAKE_ERROR_NO_MESHES")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_NO_MESHES`"]
    pub const NO_MESHES: BakeError = BakeError {
        ord: 5i32
    };
    #[doc(alias = "BAKE_ERROR_MESHES_INVALID")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_MESHES_INVALID`"]
    pub const MESHES_INVALID: BakeError = BakeError {
        ord: 6i32
    };
    #[doc(alias = "BAKE_ERROR_CANT_CREATE_IMAGE")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_CANT_CREATE_IMAGE`"]
    pub const CANT_CREATE_IMAGE: BakeError = BakeError {
        ord: 7i32
    };
    #[doc(alias = "BAKE_ERROR_USER_ABORTED")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_USER_ABORTED`"]
    pub const USER_ABORTED: BakeError = BakeError {
        ord: 8i32
    };
    #[doc(alias = "BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL`"]
    pub const TEXTURE_SIZE_TOO_SMALL: BakeError = BakeError {
        ord: 9i32
    };
    #[doc(alias = "BAKE_ERROR_LIGHTMAP_TOO_SMALL")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_LIGHTMAP_TOO_SMALL`"]
    pub const LIGHTMAP_TOO_SMALL: BakeError = BakeError {
        ord: 10i32
    };
    #[doc(alias = "BAKE_ERROR_ATLAS_TOO_SMALL")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_ATLAS_TOO_SMALL`"]
    pub const ATLAS_TOO_SMALL: BakeError = BakeError {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for BakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BakeError") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BakeError {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::OK => "OK", Self::NO_SCENE_ROOT => "NO_SCENE_ROOT", Self::FOREIGN_DATA => "FOREIGN_DATA", Self::NO_LIGHTMAPPER => "NO_LIGHTMAPPER", Self::NO_SAVE_PATH => "NO_SAVE_PATH", Self::NO_MESHES => "NO_MESHES", Self::MESHES_INVALID => "MESHES_INVALID", Self::CANT_CREATE_IMAGE => "CANT_CREATE_IMAGE", Self::USER_ABORTED => "USER_ABORTED", Self::TEXTURE_SIZE_TOO_SMALL => "TEXTURE_SIZE_TOO_SMALL", Self::LIGHTMAP_TOO_SMALL => "LIGHTMAP_TOO_SMALL", Self::ATLAS_TOO_SMALL => "ATLAS_TOO_SMALL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OK => "BAKE_ERROR_OK", Self::NO_SCENE_ROOT => "BAKE_ERROR_NO_SCENE_ROOT", Self::FOREIGN_DATA => "BAKE_ERROR_FOREIGN_DATA", Self::NO_LIGHTMAPPER => "BAKE_ERROR_NO_LIGHTMAPPER", Self::NO_SAVE_PATH => "BAKE_ERROR_NO_SAVE_PATH", Self::NO_MESHES => "BAKE_ERROR_NO_MESHES", Self::MESHES_INVALID => "BAKE_ERROR_MESHES_INVALID", Self::CANT_CREATE_IMAGE => "BAKE_ERROR_CANT_CREATE_IMAGE", Self::USER_ABORTED => "BAKE_ERROR_USER_ABORTED", Self::TEXTURE_SIZE_TOO_SMALL => "BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL", Self::LIGHTMAP_TOO_SMALL => "BAKE_ERROR_LIGHTMAP_TOO_SMALL", Self::ATLAS_TOO_SMALL => "BAKE_ERROR_ATLAS_TOO_SMALL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BakeError {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BakeError {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BakeError {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentMode {
    ord: i32
}
impl EnvironmentMode {
    #[doc(alias = "ENVIRONMENT_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `ENVIRONMENT_MODE_DISABLED`"]
    pub const DISABLED: EnvironmentMode = EnvironmentMode {
        ord: 0i32
    };
    #[doc(alias = "ENVIRONMENT_MODE_SCENE")]
    #[doc = "Godot enumerator name: `ENVIRONMENT_MODE_SCENE`"]
    pub const SCENE: EnvironmentMode = EnvironmentMode {
        ord: 1i32
    };
    #[doc(alias = "ENVIRONMENT_MODE_CUSTOM_SKY")]
    #[doc = "Godot enumerator name: `ENVIRONMENT_MODE_CUSTOM_SKY`"]
    pub const CUSTOM_SKY: EnvironmentMode = EnvironmentMode {
        ord: 2i32
    };
    #[doc(alias = "ENVIRONMENT_MODE_CUSTOM_COLOR")]
    #[doc = "Godot enumerator name: `ENVIRONMENT_MODE_CUSTOM_COLOR`"]
    pub const CUSTOM_COLOR: EnvironmentMode = EnvironmentMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EnvironmentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentMode {
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
            Self::DISABLED => "DISABLED", Self::SCENE => "SCENE", Self::CUSTOM_SKY => "CUSTOM_SKY", Self::CUSTOM_COLOR => "CUSTOM_COLOR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "ENVIRONMENT_MODE_DISABLED", Self::SCENE => "ENVIRONMENT_MODE_SCENE", Self::CUSTOM_SKY => "ENVIRONMENT_MODE_CUSTOM_SKY", Self::CUSTOM_COLOR => "ENVIRONMENT_MODE_CUSTOM_COLOR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}