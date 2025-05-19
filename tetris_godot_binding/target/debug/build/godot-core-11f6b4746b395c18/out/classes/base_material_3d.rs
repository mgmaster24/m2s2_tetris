#![doc = "Sidecar module for class [`BaseMaterial3D`][crate::classes::BaseMaterial3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BaseMaterial3D` enums](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `BaseMaterial3D.`\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`base_material_3d`][crate::classes::base_material_3d]: sidecar module with related enum/flag types\n* [`IBaseMaterial3D`][crate::classes::IBaseMaterial3D]: virtual methods\n\n\nSee also [Godot docs for `BaseMaterial3D`](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<BaseMaterial3D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct BaseMaterial3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`BaseMaterial3D`][crate::classes::BaseMaterial3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `BaseMaterial3D` methods](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IBaseMaterial3D: crate::obj::GodotClass < Base = BaseMaterial3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl BaseMaterial3D {
        pub fn set_albedo(&mut self, albedo: Color,) {
            type CallSig = ((), Color);
            let args = (albedo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_albedo(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transparency(&mut self, transparency: crate::classes::base_material_3d::Transparency,) {
            type CallSig = ((), crate::classes::base_material_3d::Transparency);
            let args = (transparency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transparency(&self,) -> crate::classes::base_material_3d::Transparency {
            type CallSig = (crate::classes::base_material_3d::Transparency,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: crate::classes::base_material_3d::AlphaAntiAliasing,) {
            type CallSig = ((), crate::classes::base_material_3d::AlphaAntiAliasing);
            let args = (alpha_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing(&self,) -> crate::classes::base_material_3d::AlphaAntiAliasing {
            type CallSig = (crate::classes::base_material_3d::AlphaAntiAliasing,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_alpha_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f32,) {
            type CallSig = ((), f32);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_antialiasing_edge(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_alpha_antialiasing_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shading_mode(&mut self, shading_mode: crate::classes::base_material_3d::ShadingMode,) {
            type CallSig = ((), crate::classes::base_material_3d::ShadingMode);
            let args = (shading_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_shading_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shading_mode(&self,) -> crate::classes::base_material_3d::ShadingMode {
            type CallSig = (crate::classes::base_material_3d::ShadingMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_shading_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_specular(&mut self, specular: f32,) {
            type CallSig = ((), f32);
            let args = (specular,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_specular", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_specular(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_specular", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_metallic(&mut self, metallic: f32,) {
            type CallSig = ((), f32);
            let args = (metallic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_metallic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_metallic(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_metallic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_roughness(&mut self, roughness: f32,) {
            type CallSig = ((), f32);
            let args = (roughness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_roughness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_roughness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_roughness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission(&mut self, emission: Color,) {
            type CallSig = ((), Color);
            let args = (emission,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_energy_multiplier(&mut self, emission_energy_multiplier: f32,) {
            type CallSig = ((), f32);
            let args = (emission_energy_multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_emission_energy_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_energy_multiplier(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_emission_energy_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_intensity(&mut self, emission_energy_multiplier: f32,) {
            type CallSig = ((), f32);
            let args = (emission_energy_multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_emission_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_intensity(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_emission_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normal_scale(&mut self, normal_scale: f32,) {
            type CallSig = ((), f32);
            let args = (normal_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_normal_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_normal_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_normal_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rim(&mut self, rim: f32,) {
            type CallSig = ((), f32);
            let args = (rim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_rim", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rim(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_rim", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rim_tint(&mut self, rim_tint: f32,) {
            type CallSig = ((), f32);
            let args = (rim_tint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_rim_tint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rim_tint(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_rim_tint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clearcoat(&mut self, clearcoat: f32,) {
            type CallSig = ((), f32);
            let args = (clearcoat,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_clearcoat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clearcoat(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_clearcoat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clearcoat_roughness(&mut self, clearcoat_roughness: f32,) {
            type CallSig = ((), f32);
            let args = (clearcoat_roughness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_clearcoat_roughness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clearcoat_roughness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_clearcoat_roughness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_anisotropy(&mut self, anisotropy: f32,) {
            type CallSig = ((), f32);
            let args = (anisotropy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_anisotropy(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_scale(&mut self, heightmap_scale: f32,) {
            type CallSig = ((), f32);
            let args = (heightmap_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subsurface_scattering_strength(&mut self, strength: f32,) {
            type CallSig = ((), f32);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_subsurface_scattering_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subsurface_scattering_strength(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_subsurface_scattering_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transmittance_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_transmittance_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transmittance_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_transmittance_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transmittance_depth(&mut self, depth: f32,) {
            type CallSig = ((), f32);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_transmittance_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transmittance_depth(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_transmittance_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transmittance_boost(&mut self, boost: f32,) {
            type CallSig = ((), f32);
            let args = (boost,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_transmittance_boost", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transmittance_boost(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_transmittance_boost", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_backlight(&mut self, backlight: Color,) {
            type CallSig = ((), Color);
            let args = (backlight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_backlight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_backlight(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_backlight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_refraction(&mut self, refraction: f32,) {
            type CallSig = ((), f32);
            let args = (refraction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_refraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_refraction(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_refraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_size(&mut self, point_size: f32,) {
            type CallSig = ((), f32);
            let args = (point_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_point_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_size(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_point_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_detail_uv(&mut self, detail_uv: crate::classes::base_material_3d::DetailUv,) {
            type CallSig = ((), crate::classes::base_material_3d::DetailUv);
            let args = (detail_uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_detail_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_detail_uv(&self,) -> crate::classes::base_material_3d::DetailUv {
            type CallSig = (crate::classes::base_material_3d::DetailUv,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_detail_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_mode(&mut self, blend_mode: crate::classes::base_material_3d::BlendMode,) {
            type CallSig = ((), crate::classes::base_material_3d::BlendMode);
            let args = (blend_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::classes::base_material_3d::BlendMode {
            type CallSig = (crate::classes::base_material_3d::BlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_draw_mode(&mut self, depth_draw_mode: crate::classes::base_material_3d::DepthDrawMode,) {
            type CallSig = ((), crate::classes::base_material_3d::DepthDrawMode);
            let args = (depth_draw_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_depth_draw_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_draw_mode(&self,) -> crate::classes::base_material_3d::DepthDrawMode {
            type CallSig = (crate::classes::base_material_3d::DepthDrawMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_depth_draw_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cull_mode(&mut self, cull_mode: crate::classes::base_material_3d::CullMode,) {
            type CallSig = ((), crate::classes::base_material_3d::CullMode);
            let args = (cull_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cull_mode(&self,) -> crate::classes::base_material_3d::CullMode {
            type CallSig = (crate::classes::base_material_3d::CullMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_diffuse_mode(&mut self, diffuse_mode: crate::classes::base_material_3d::DiffuseMode,) {
            type CallSig = ((), crate::classes::base_material_3d::DiffuseMode);
            let args = (diffuse_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_diffuse_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_diffuse_mode(&self,) -> crate::classes::base_material_3d::DiffuseMode {
            type CallSig = (crate::classes::base_material_3d::DiffuseMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_diffuse_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_specular_mode(&mut self, specular_mode: crate::classes::base_material_3d::SpecularMode,) {
            type CallSig = ((), crate::classes::base_material_3d::SpecularMode);
            let args = (specular_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_specular_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_specular_mode(&self,) -> crate::classes::base_material_3d::SpecularMode {
            type CallSig = (crate::classes::base_material_3d::SpecularMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_specular_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flag(&mut self, flag: crate::classes::base_material_3d::Flags, enable: bool,) {
            type CallSig = ((), crate::classes::base_material_3d::Flags, bool);
            let args = (flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flag(&self, flag: crate::classes::base_material_3d::Flags,) -> bool {
            type CallSig = (bool, crate::classes::base_material_3d::Flags);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::classes::base_material_3d::TextureFilter,) {
            type CallSig = ((), crate::classes::base_material_3d::TextureFilter);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::base_material_3d::TextureFilter {
            type CallSig = (crate::classes::base_material_3d::TextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_feature(&mut self, feature: crate::classes::base_material_3d::Feature, enable: bool,) {
            type CallSig = ((), crate::classes::base_material_3d::Feature, bool);
            let args = (feature, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_feature(&self, feature: crate::classes::base_material_3d::Feature,) -> bool {
            type CallSig = (bool, crate::classes::base_material_3d::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture(&mut self, param: crate::classes::base_material_3d::TextureParam, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), crate::classes::base_material_3d::TextureParam, ObjectArg < crate::classes::Texture2D >);
            let args = (param, texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self, param: crate::classes::base_material_3d::TextureParam,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, crate::classes::base_material_3d::TextureParam);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_detail_blend_mode(&mut self, detail_blend_mode: crate::classes::base_material_3d::BlendMode,) {
            type CallSig = ((), crate::classes::base_material_3d::BlendMode);
            let args = (detail_blend_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_detail_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_detail_blend_mode(&self,) -> crate::classes::base_material_3d::BlendMode {
            type CallSig = (crate::classes::base_material_3d::BlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_detail_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv1_scale(&mut self, scale: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv1_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv1_scale(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv1_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv1_offset(&mut self, offset: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv1_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv1_offset(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv1_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv1_triplanar_blend_sharpness(&mut self, sharpness: f32,) {
            type CallSig = ((), f32);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv1_triplanar_blend_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv1_triplanar_blend_sharpness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv1_triplanar_blend_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv2_scale(&mut self, scale: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv2_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv2_scale(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv2_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv2_offset(&mut self, offset: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv2_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv2_offset(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv2_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv2_triplanar_blend_sharpness(&mut self, sharpness: f32,) {
            type CallSig = ((), f32);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv2_triplanar_blend_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_uv2_triplanar_blend_sharpness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv2_triplanar_blend_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_billboard_mode(&mut self, mode: crate::classes::base_material_3d::BillboardMode,) {
            type CallSig = ((), crate::classes::base_material_3d::BillboardMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_billboard_mode(&self,) -> crate::classes::base_material_3d::BillboardMode {
            type CallSig = (crate::classes::base_material_3d::BillboardMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_billboard_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_h_frames(&mut self, frames: i32,) {
            type CallSig = ((), i32);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_particles_anim_h_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_h_frames(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_particles_anim_h_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_v_frames(&mut self, frames: i32,) {
            type CallSig = ((), i32);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_particles_anim_v_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_v_frames(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_particles_anim_v_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particles_anim_loop(&mut self, loop_: bool,) {
            type CallSig = ((), bool);
            let args = (loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_particles_anim_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particles_anim_loop(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_particles_anim_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_heightmap_deep_parallax_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "is_heightmap_deep_parallax_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_min_layers(&mut self, layer: i32,) {
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax_min_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_min_layers(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_deep_parallax_min_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_max_layers(&mut self, layer: i32,) {
            type CallSig = ((), i32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax_max_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_max_layers(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_deep_parallax_max_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_flip_tangent(&mut self, flip: bool,) {
            type CallSig = ((), bool);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax_flip_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_flip_tangent(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_deep_parallax_flip_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_flip_binormal(&mut self, flip: bool,) {
            type CallSig = ((), bool);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax_flip_binormal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_flip_binormal(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_deep_parallax_flip_binormal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_grow(&mut self, amount: f32,) {
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_grow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_grow(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_grow", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_operator(&mut self, operator: crate::classes::base_material_3d::EmissionOperator,) {
            type CallSig = ((), crate::classes::base_material_3d::EmissionOperator);
            let args = (operator,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_emission_operator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_operator(&self,) -> crate::classes::base_material_3d::EmissionOperator {
            type CallSig = (crate::classes::base_material_3d::EmissionOperator,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_emission_operator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ao_light_affect(&mut self, amount: f32,) {
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_ao_light_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ao_light_affect(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_ao_light_affect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f32,) {
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_scissor_threshold(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_alpha_scissor_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f32,) {
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_hash_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_alpha_hash_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_grow_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_grow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_grow_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "is_grow_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_metallic_texture_channel(&mut self, channel: crate::classes::base_material_3d::TextureChannel,) {
            type CallSig = ((), crate::classes::base_material_3d::TextureChannel);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_metallic_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_metallic_texture_channel(&self,) -> crate::classes::base_material_3d::TextureChannel {
            type CallSig = (crate::classes::base_material_3d::TextureChannel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_metallic_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_roughness_texture_channel(&mut self, channel: crate::classes::base_material_3d::TextureChannel,) {
            type CallSig = ((), crate::classes::base_material_3d::TextureChannel);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_roughness_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_roughness_texture_channel(&self,) -> crate::classes::base_material_3d::TextureChannel {
            type CallSig = (crate::classes::base_material_3d::TextureChannel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_roughness_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ao_texture_channel(&mut self, channel: crate::classes::base_material_3d::TextureChannel,) {
            type CallSig = ((), crate::classes::base_material_3d::TextureChannel);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_ao_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ao_texture_channel(&self,) -> crate::classes::base_material_3d::TextureChannel {
            type CallSig = (crate::classes::base_material_3d::TextureChannel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_ao_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_refraction_texture_channel(&mut self, channel: crate::classes::base_material_3d::TextureChannel,) {
            type CallSig = ((), crate::classes::base_material_3d::TextureChannel);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_refraction_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_refraction_texture_channel(&self,) -> crate::classes::base_material_3d::TextureChannel {
            type CallSig = (crate::classes::base_material_3d::TextureChannel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_refraction_texture_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_proximity_fade_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_proximity_fade_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_proximity_fade_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "is_proximity_fade_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_proximity_fade_distance(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_proximity_fade_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_proximity_fade_distance(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_proximity_fade_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, range: f32,) {
            type CallSig = ((), f32);
            let args = (range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_outline_size(&mut self, size: f32,) {
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_msdf_outline_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_outline_size(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_msdf_outline_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade(&mut self, mode: crate::classes::base_material_3d::DistanceFadeMode,) {
            type CallSig = ((), crate::classes::base_material_3d::DistanceFadeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_distance_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade(&self,) -> crate::classes::base_material_3d::DistanceFadeMode {
            type CallSig = (crate::classes::base_material_3d::DistanceFadeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_distance_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_max_distance(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_distance_fade_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_max_distance(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_distance_fade_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distance_fade_min_distance(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_distance_fade_min_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distance_fade_min_distance(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_distance_fade_min_distance", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for BaseMaterial3D {
        type Base = crate::classes::Material;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"BaseMaterial3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for BaseMaterial3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for BaseMaterial3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for BaseMaterial3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for BaseMaterial3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for BaseMaterial3D {
        
    }
    impl std::ops::Deref for BaseMaterial3D {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for BaseMaterial3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`BaseMaterial3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_BaseMaterial3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::BaseMaterial3D > for $Class {
                
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
pub struct TextureParam {
    ord: i32
}
impl TextureParam {
    #[doc(alias = "TEXTURE_ALBEDO")]
    #[doc = "Godot enumerator name: `TEXTURE_ALBEDO`"]
    pub const ALBEDO: TextureParam = TextureParam {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_METALLIC")]
    #[doc = "Godot enumerator name: `TEXTURE_METALLIC`"]
    pub const METALLIC: TextureParam = TextureParam {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_ROUGHNESS")]
    #[doc = "Godot enumerator name: `TEXTURE_ROUGHNESS`"]
    pub const ROUGHNESS: TextureParam = TextureParam {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_EMISSION")]
    #[doc = "Godot enumerator name: `TEXTURE_EMISSION`"]
    pub const EMISSION: TextureParam = TextureParam {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_NORMAL")]
    #[doc = "Godot enumerator name: `TEXTURE_NORMAL`"]
    pub const NORMAL: TextureParam = TextureParam {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_RIM")]
    #[doc = "Godot enumerator name: `TEXTURE_RIM`"]
    pub const RIM: TextureParam = TextureParam {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_CLEARCOAT")]
    #[doc = "Godot enumerator name: `TEXTURE_CLEARCOAT`"]
    pub const CLEARCOAT: TextureParam = TextureParam {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_FLOWMAP")]
    #[doc = "Godot enumerator name: `TEXTURE_FLOWMAP`"]
    pub const FLOWMAP: TextureParam = TextureParam {
        ord: 7i32
    };
    #[doc(alias = "TEXTURE_AMBIENT_OCCLUSION")]
    #[doc = "Godot enumerator name: `TEXTURE_AMBIENT_OCCLUSION`"]
    pub const AMBIENT_OCCLUSION: TextureParam = TextureParam {
        ord: 8i32
    };
    #[doc(alias = "TEXTURE_HEIGHTMAP")]
    #[doc = "Godot enumerator name: `TEXTURE_HEIGHTMAP`"]
    pub const HEIGHTMAP: TextureParam = TextureParam {
        ord: 9i32
    };
    #[doc(alias = "TEXTURE_SUBSURFACE_SCATTERING")]
    #[doc = "Godot enumerator name: `TEXTURE_SUBSURFACE_SCATTERING`"]
    pub const SUBSURFACE_SCATTERING: TextureParam = TextureParam {
        ord: 10i32
    };
    #[doc(alias = "TEXTURE_SUBSURFACE_TRANSMITTANCE")]
    #[doc = "Godot enumerator name: `TEXTURE_SUBSURFACE_TRANSMITTANCE`"]
    pub const SUBSURFACE_TRANSMITTANCE: TextureParam = TextureParam {
        ord: 11i32
    };
    #[doc(alias = "TEXTURE_BACKLIGHT")]
    #[doc = "Godot enumerator name: `TEXTURE_BACKLIGHT`"]
    pub const BACKLIGHT: TextureParam = TextureParam {
        ord: 12i32
    };
    #[doc(alias = "TEXTURE_REFRACTION")]
    #[doc = "Godot enumerator name: `TEXTURE_REFRACTION`"]
    pub const REFRACTION: TextureParam = TextureParam {
        ord: 13i32
    };
    #[doc(alias = "TEXTURE_DETAIL_MASK")]
    #[doc = "Godot enumerator name: `TEXTURE_DETAIL_MASK`"]
    pub const DETAIL_MASK: TextureParam = TextureParam {
        ord: 14i32
    };
    #[doc(alias = "TEXTURE_DETAIL_ALBEDO")]
    #[doc = "Godot enumerator name: `TEXTURE_DETAIL_ALBEDO`"]
    pub const DETAIL_ALBEDO: TextureParam = TextureParam {
        ord: 15i32
    };
    #[doc(alias = "TEXTURE_DETAIL_NORMAL")]
    #[doc = "Godot enumerator name: `TEXTURE_DETAIL_NORMAL`"]
    pub const DETAIL_NORMAL: TextureParam = TextureParam {
        ord: 16i32
    };
    #[doc(alias = "TEXTURE_ORM")]
    #[doc = "Godot enumerator name: `TEXTURE_ORM`"]
    pub const ORM: TextureParam = TextureParam {
        ord: 17i32
    };
    #[doc(alias = "TEXTURE_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_MAX`"]
    pub const MAX: TextureParam = TextureParam {
        ord: 18i32
    };
    
}
impl std::fmt::Debug for TextureParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureParam {
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
            Self::ALBEDO => "ALBEDO", Self::METALLIC => "METALLIC", Self::ROUGHNESS => "ROUGHNESS", Self::EMISSION => "EMISSION", Self::NORMAL => "NORMAL", Self::RIM => "RIM", Self::CLEARCOAT => "CLEARCOAT", Self::FLOWMAP => "FLOWMAP", Self::AMBIENT_OCCLUSION => "AMBIENT_OCCLUSION", Self::HEIGHTMAP => "HEIGHTMAP", Self::SUBSURFACE_SCATTERING => "SUBSURFACE_SCATTERING", Self::SUBSURFACE_TRANSMITTANCE => "SUBSURFACE_TRANSMITTANCE", Self::BACKLIGHT => "BACKLIGHT", Self::REFRACTION => "REFRACTION", Self::DETAIL_MASK => "DETAIL_MASK", Self::DETAIL_ALBEDO => "DETAIL_ALBEDO", Self::DETAIL_NORMAL => "DETAIL_NORMAL", Self::ORM => "ORM", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ALBEDO => "TEXTURE_ALBEDO", Self::METALLIC => "TEXTURE_METALLIC", Self::ROUGHNESS => "TEXTURE_ROUGHNESS", Self::EMISSION => "TEXTURE_EMISSION", Self::NORMAL => "TEXTURE_NORMAL", Self::RIM => "TEXTURE_RIM", Self::CLEARCOAT => "TEXTURE_CLEARCOAT", Self::FLOWMAP => "TEXTURE_FLOWMAP", Self::AMBIENT_OCCLUSION => "TEXTURE_AMBIENT_OCCLUSION", Self::HEIGHTMAP => "TEXTURE_HEIGHTMAP", Self::SUBSURFACE_SCATTERING => "TEXTURE_SUBSURFACE_SCATTERING", Self::SUBSURFACE_TRANSMITTANCE => "TEXTURE_SUBSURFACE_TRANSMITTANCE", Self::BACKLIGHT => "TEXTURE_BACKLIGHT", Self::REFRACTION => "TEXTURE_REFRACTION", Self::DETAIL_MASK => "TEXTURE_DETAIL_MASK", Self::DETAIL_ALBEDO => "TEXTURE_DETAIL_ALBEDO", Self::DETAIL_NORMAL => "TEXTURE_DETAIL_NORMAL", Self::ORM => "TEXTURE_ORM", Self::MAX => "TEXTURE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureParam {
    const ENUMERATOR_COUNT: usize = 18usize;
    
}
impl crate::meta::GodotConvert for TextureParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureParam {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureFilter {
    ord: i32
}
impl TextureFilter {
    #[doc(alias = "TEXTURE_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST`"]
    pub const NEAREST: TextureFilter = TextureFilter {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR`"]
    pub const LINEAR: TextureFilter = TextureFilter {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST_WITH_MIPMAPS`"]
    pub const NEAREST_WITH_MIPMAPS: TextureFilter = TextureFilter {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR_WITH_MIPMAPS`"]
    pub const LINEAR_WITH_MIPMAPS: TextureFilter = TextureFilter {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_WITH_MIPMAPS_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_WITH_MIPMAPS_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_FILTER_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_MAX`"]
    pub const MAX: TextureFilter = TextureFilter {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for TextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_WITH_MIPMAPS => "NEAREST_WITH_MIPMAPS", Self::LINEAR_WITH_MIPMAPS => "LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS_ANISOTROPIC => "NEAREST_WITH_MIPMAPS_ANISOTROPIC", Self::LINEAR_WITH_MIPMAPS_ANISOTROPIC => "LINEAR_WITH_MIPMAPS_ANISOTROPIC", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NEAREST => "TEXTURE_FILTER_NEAREST", Self::LINEAR => "TEXTURE_FILTER_LINEAR", Self::NEAREST_WITH_MIPMAPS => "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS", Self::LINEAR_WITH_MIPMAPS => "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS_ANISOTROPIC => "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC", Self::LINEAR_WITH_MIPMAPS_ANISOTROPIC => "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC", Self::MAX => "TEXTURE_FILTER_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for TextureFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureFilter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `DetailUV`."]
pub struct DetailUv {
    ord: i32
}
impl DetailUv {
    #[doc(alias = "DETAIL_UV_1")]
    #[doc = "Godot enumerator name: `DETAIL_UV_1`"]
    pub const UV_1: DetailUv = DetailUv {
        ord: 0i32
    };
    #[doc(alias = "DETAIL_UV_2")]
    #[doc = "Godot enumerator name: `DETAIL_UV_2`"]
    pub const UV_2: DetailUv = DetailUv {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DetailUv {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DetailUv") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DetailUv {
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
            Self::UV_1 => "UV_1", Self::UV_2 => "UV_2", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UV_1 => "DETAIL_UV_1", Self::UV_2 => "DETAIL_UV_2", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DetailUv {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DetailUv {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DetailUv {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Transparency {
    ord: i32
}
impl Transparency {
    #[doc(alias = "TRANSPARENCY_DISABLED")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_DISABLED`"]
    pub const DISABLED: Transparency = Transparency {
        ord: 0i32
    };
    #[doc(alias = "TRANSPARENCY_ALPHA")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_ALPHA`"]
    pub const ALPHA: Transparency = Transparency {
        ord: 1i32
    };
    #[doc(alias = "TRANSPARENCY_ALPHA_SCISSOR")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_ALPHA_SCISSOR`"]
    pub const ALPHA_SCISSOR: Transparency = Transparency {
        ord: 2i32
    };
    #[doc(alias = "TRANSPARENCY_ALPHA_HASH")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_ALPHA_HASH`"]
    pub const ALPHA_HASH: Transparency = Transparency {
        ord: 3i32
    };
    #[doc(alias = "TRANSPARENCY_ALPHA_DEPTH_PRE_PASS")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_ALPHA_DEPTH_PRE_PASS`"]
    pub const ALPHA_DEPTH_PRE_PASS: Transparency = Transparency {
        ord: 4i32
    };
    #[doc(alias = "TRANSPARENCY_MAX")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_MAX`"]
    pub const MAX: Transparency = Transparency {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for Transparency {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Transparency") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Transparency {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::ALPHA => "ALPHA", Self::ALPHA_SCISSOR => "ALPHA_SCISSOR", Self::ALPHA_HASH => "ALPHA_HASH", Self::ALPHA_DEPTH_PRE_PASS => "ALPHA_DEPTH_PRE_PASS", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "TRANSPARENCY_DISABLED", Self::ALPHA => "TRANSPARENCY_ALPHA", Self::ALPHA_SCISSOR => "TRANSPARENCY_ALPHA_SCISSOR", Self::ALPHA_HASH => "TRANSPARENCY_ALPHA_HASH", Self::ALPHA_DEPTH_PRE_PASS => "TRANSPARENCY_ALPHA_DEPTH_PRE_PASS", Self::MAX => "TRANSPARENCY_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Transparency {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for Transparency {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Transparency {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Transparency {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadingMode {
    ord: i32
}
impl ShadingMode {
    #[doc(alias = "SHADING_MODE_UNSHADED")]
    #[doc = "Godot enumerator name: `SHADING_MODE_UNSHADED`"]
    pub const UNSHADED: ShadingMode = ShadingMode {
        ord: 0i32
    };
    #[doc(alias = "SHADING_MODE_PER_PIXEL")]
    #[doc = "Godot enumerator name: `SHADING_MODE_PER_PIXEL`"]
    pub const PER_PIXEL: ShadingMode = ShadingMode {
        ord: 1i32
    };
    #[doc(alias = "SHADING_MODE_PER_VERTEX")]
    #[doc = "Godot enumerator name: `SHADING_MODE_PER_VERTEX`"]
    pub const PER_VERTEX: ShadingMode = ShadingMode {
        ord: 2i32
    };
    #[doc(alias = "SHADING_MODE_MAX")]
    #[doc = "Godot enumerator name: `SHADING_MODE_MAX`"]
    pub const MAX: ShadingMode = ShadingMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ShadingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadingMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadingMode {
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
            Self::UNSHADED => "UNSHADED", Self::PER_PIXEL => "PER_PIXEL", Self::PER_VERTEX => "PER_VERTEX", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UNSHADED => "SHADING_MODE_UNSHADED", Self::PER_PIXEL => "SHADING_MODE_PER_PIXEL", Self::PER_VERTEX => "SHADING_MODE_PER_VERTEX", Self::MAX => "SHADING_MODE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for ShadingMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ShadingMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadingMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    #[doc(alias = "FEATURE_EMISSION")]
    #[doc = "Godot enumerator name: `FEATURE_EMISSION`"]
    pub const EMISSION: Feature = Feature {
        ord: 0i32
    };
    #[doc(alias = "FEATURE_NORMAL_MAPPING")]
    #[doc = "Godot enumerator name: `FEATURE_NORMAL_MAPPING`"]
    pub const NORMAL_MAPPING: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_RIM")]
    #[doc = "Godot enumerator name: `FEATURE_RIM`"]
    pub const RIM: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_CLEARCOAT")]
    #[doc = "Godot enumerator name: `FEATURE_CLEARCOAT`"]
    pub const CLEARCOAT: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_ANISOTROPY")]
    #[doc = "Godot enumerator name: `FEATURE_ANISOTROPY`"]
    pub const ANISOTROPY: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_AMBIENT_OCCLUSION")]
    #[doc = "Godot enumerator name: `FEATURE_AMBIENT_OCCLUSION`"]
    pub const AMBIENT_OCCLUSION: Feature = Feature {
        ord: 5i32
    };
    #[doc(alias = "FEATURE_HEIGHT_MAPPING")]
    #[doc = "Godot enumerator name: `FEATURE_HEIGHT_MAPPING`"]
    pub const HEIGHT_MAPPING: Feature = Feature {
        ord: 6i32
    };
    #[doc(alias = "FEATURE_SUBSURFACE_SCATTERING")]
    #[doc = "Godot enumerator name: `FEATURE_SUBSURFACE_SCATTERING`"]
    pub const SUBSURFACE_SCATTERING: Feature = Feature {
        ord: 7i32
    };
    #[doc(alias = "FEATURE_SUBSURFACE_TRANSMITTANCE")]
    #[doc = "Godot enumerator name: `FEATURE_SUBSURFACE_TRANSMITTANCE`"]
    pub const SUBSURFACE_TRANSMITTANCE: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_BACKLIGHT")]
    #[doc = "Godot enumerator name: `FEATURE_BACKLIGHT`"]
    pub const BACKLIGHT: Feature = Feature {
        ord: 9i32
    };
    #[doc(alias = "FEATURE_REFRACTION")]
    #[doc = "Godot enumerator name: `FEATURE_REFRACTION`"]
    pub const REFRACTION: Feature = Feature {
        ord: 10i32
    };
    #[doc(alias = "FEATURE_DETAIL")]
    #[doc = "Godot enumerator name: `FEATURE_DETAIL`"]
    pub const DETAIL: Feature = Feature {
        ord: 11i32
    };
    #[doc(alias = "FEATURE_MAX")]
    #[doc = "Godot enumerator name: `FEATURE_MAX`"]
    pub const MAX: Feature = Feature {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
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
            Self::EMISSION => "EMISSION", Self::NORMAL_MAPPING => "NORMAL_MAPPING", Self::RIM => "RIM", Self::CLEARCOAT => "CLEARCOAT", Self::ANISOTROPY => "ANISOTROPY", Self::AMBIENT_OCCLUSION => "AMBIENT_OCCLUSION", Self::HEIGHT_MAPPING => "HEIGHT_MAPPING", Self::SUBSURFACE_SCATTERING => "SUBSURFACE_SCATTERING", Self::SUBSURFACE_TRANSMITTANCE => "SUBSURFACE_TRANSMITTANCE", Self::BACKLIGHT => "BACKLIGHT", Self::REFRACTION => "REFRACTION", Self::DETAIL => "DETAIL", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::EMISSION => "FEATURE_EMISSION", Self::NORMAL_MAPPING => "FEATURE_NORMAL_MAPPING", Self::RIM => "FEATURE_RIM", Self::CLEARCOAT => "FEATURE_CLEARCOAT", Self::ANISOTROPY => "FEATURE_ANISOTROPY", Self::AMBIENT_OCCLUSION => "FEATURE_AMBIENT_OCCLUSION", Self::HEIGHT_MAPPING => "FEATURE_HEIGHT_MAPPING", Self::SUBSURFACE_SCATTERING => "FEATURE_SUBSURFACE_SCATTERING", Self::SUBSURFACE_TRANSMITTANCE => "FEATURE_SUBSURFACE_TRANSMITTANCE", Self::BACKLIGHT => "FEATURE_BACKLIGHT", Self::REFRACTION => "FEATURE_REFRACTION", Self::DETAIL => "FEATURE_DETAIL", Self::MAX => "FEATURE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Feature {
    const ENUMERATOR_COUNT: usize = 12usize;
    
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Feature {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendMode {
    ord: i32
}
impl BlendMode {
    #[doc(alias = "BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `BLEND_MODE_MIX`"]
    pub const MIX: BlendMode = BlendMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_MODE_ADD")]
    #[doc = "Godot enumerator name: `BLEND_MODE_ADD`"]
    pub const ADD: BlendMode = BlendMode {
        ord: 1i32
    };
    #[doc(alias = "BLEND_MODE_SUB")]
    #[doc = "Godot enumerator name: `BLEND_MODE_SUB`"]
    pub const SUB: BlendMode = BlendMode {
        ord: 2i32
    };
    #[doc(alias = "BLEND_MODE_MUL")]
    #[doc = "Godot enumerator name: `BLEND_MODE_MUL`"]
    pub const MUL: BlendMode = BlendMode {
        ord: 3i32
    };
    #[doc(alias = "BLEND_MODE_PREMULT_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_MODE_PREMULT_ALPHA`"]
    pub const PREMULT_ALPHA: BlendMode = BlendMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendMode {
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
            Self::MIX => "MIX", Self::ADD => "ADD", Self::SUB => "SUB", Self::MUL => "MUL", Self::PREMULT_ALPHA => "PREMULT_ALPHA", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::MIX => "BLEND_MODE_MIX", Self::ADD => "BLEND_MODE_ADD", Self::SUB => "BLEND_MODE_SUB", Self::MUL => "BLEND_MODE_MUL", Self::PREMULT_ALPHA => "BLEND_MODE_PREMULT_ALPHA", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlphaAntiAliasing {
    ord: i32
}
impl AlphaAntiAliasing {
    #[doc(alias = "ALPHA_ANTIALIASING_OFF")]
    #[doc = "Godot enumerator name: `ALPHA_ANTIALIASING_OFF`"]
    pub const OFF: AlphaAntiAliasing = AlphaAntiAliasing {
        ord: 0i32
    };
    #[doc(alias = "ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE")]
    #[doc = "Godot enumerator name: `ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE`"]
    pub const ALPHA_TO_COVERAGE: AlphaAntiAliasing = AlphaAntiAliasing {
        ord: 1i32
    };
    #[doc(alias = "ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE_AND_TO_ONE")]
    #[doc = "Godot enumerator name: `ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE_AND_TO_ONE`"]
    pub const ALPHA_TO_COVERAGE_AND_TO_ONE: AlphaAntiAliasing = AlphaAntiAliasing {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AlphaAntiAliasing {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlphaAntiAliasing") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlphaAntiAliasing {
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
            Self::OFF => "OFF", Self::ALPHA_TO_COVERAGE => "ALPHA_TO_COVERAGE", Self::ALPHA_TO_COVERAGE_AND_TO_ONE => "ALPHA_TO_COVERAGE_AND_TO_ONE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OFF => "ALPHA_ANTIALIASING_OFF", Self::ALPHA_TO_COVERAGE => "ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE", Self::ALPHA_TO_COVERAGE_AND_TO_ONE => "ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE_AND_TO_ONE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AlphaAntiAliasing {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AlphaAntiAliasing {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlphaAntiAliasing {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DepthDrawMode {
    ord: i32
}
impl DepthDrawMode {
    #[doc(alias = "DEPTH_DRAW_OPAQUE_ONLY")]
    #[doc = "Godot enumerator name: `DEPTH_DRAW_OPAQUE_ONLY`"]
    pub const OPAQUE_ONLY: DepthDrawMode = DepthDrawMode {
        ord: 0i32
    };
    #[doc(alias = "DEPTH_DRAW_ALWAYS")]
    #[doc = "Godot enumerator name: `DEPTH_DRAW_ALWAYS`"]
    pub const ALWAYS: DepthDrawMode = DepthDrawMode {
        ord: 1i32
    };
    #[doc(alias = "DEPTH_DRAW_DISABLED")]
    #[doc = "Godot enumerator name: `DEPTH_DRAW_DISABLED`"]
    pub const DISABLED: DepthDrawMode = DepthDrawMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DepthDrawMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DepthDrawMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DepthDrawMode {
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
            Self::OPAQUE_ONLY => "OPAQUE_ONLY", Self::ALWAYS => "ALWAYS", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OPAQUE_ONLY => "DEPTH_DRAW_OPAQUE_ONLY", Self::ALWAYS => "DEPTH_DRAW_ALWAYS", Self::DISABLED => "DEPTH_DRAW_DISABLED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DepthDrawMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DepthDrawMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DepthDrawMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CullMode {
    ord: i32
}
impl CullMode {
    #[doc(alias = "CULL_BACK")]
    #[doc = "Godot enumerator name: `CULL_BACK`"]
    pub const BACK: CullMode = CullMode {
        ord: 0i32
    };
    #[doc(alias = "CULL_FRONT")]
    #[doc = "Godot enumerator name: `CULL_FRONT`"]
    pub const FRONT: CullMode = CullMode {
        ord: 1i32
    };
    #[doc(alias = "CULL_DISABLED")]
    #[doc = "Godot enumerator name: `CULL_DISABLED`"]
    pub const DISABLED: CullMode = CullMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CullMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CullMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CullMode {
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
            Self::BACK => "BACK", Self::FRONT => "FRONT", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BACK => "CULL_BACK", Self::FRONT => "CULL_FRONT", Self::DISABLED => "CULL_DISABLED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CullMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CullMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Flags {
    ord: i32
}
impl Flags {
    #[doc(alias = "FLAG_DISABLE_DEPTH_TEST")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_DEPTH_TEST`"]
    pub const DISABLE_DEPTH_TEST: Flags = Flags {
        ord: 0i32
    };
    #[doc(alias = "FLAG_ALBEDO_FROM_VERTEX_COLOR")]
    #[doc = "Godot enumerator name: `FLAG_ALBEDO_FROM_VERTEX_COLOR`"]
    pub const ALBEDO_FROM_VERTEX_COLOR: Flags = Flags {
        ord: 1i32
    };
    #[doc(alias = "FLAG_SRGB_VERTEX_COLOR")]
    #[doc = "Godot enumerator name: `FLAG_SRGB_VERTEX_COLOR`"]
    pub const SRGB_VERTEX_COLOR: Flags = Flags {
        ord: 2i32
    };
    #[doc(alias = "FLAG_USE_POINT_SIZE")]
    #[doc = "Godot enumerator name: `FLAG_USE_POINT_SIZE`"]
    pub const USE_POINT_SIZE: Flags = Flags {
        ord: 3i32
    };
    #[doc(alias = "FLAG_FIXED_SIZE")]
    #[doc = "Godot enumerator name: `FLAG_FIXED_SIZE`"]
    pub const FIXED_SIZE: Flags = Flags {
        ord: 4i32
    };
    #[doc(alias = "FLAG_BILLBOARD_KEEP_SCALE")]
    #[doc = "Godot enumerator name: `FLAG_BILLBOARD_KEEP_SCALE`"]
    pub const BILLBOARD_KEEP_SCALE: Flags = Flags {
        ord: 5i32
    };
    #[doc(alias = "FLAG_UV1_USE_TRIPLANAR")]
    #[doc = "Godot enumerator name: `FLAG_UV1_USE_TRIPLANAR`"]
    pub const UV1_USE_TRIPLANAR: Flags = Flags {
        ord: 6i32
    };
    #[doc(alias = "FLAG_UV2_USE_TRIPLANAR")]
    #[doc = "Godot enumerator name: `FLAG_UV2_USE_TRIPLANAR`"]
    pub const UV2_USE_TRIPLANAR: Flags = Flags {
        ord: 7i32
    };
    #[doc(alias = "FLAG_UV1_USE_WORLD_TRIPLANAR")]
    #[doc = "Godot enumerator name: `FLAG_UV1_USE_WORLD_TRIPLANAR`"]
    pub const UV1_USE_WORLD_TRIPLANAR: Flags = Flags {
        ord: 8i32
    };
    #[doc(alias = "FLAG_UV2_USE_WORLD_TRIPLANAR")]
    #[doc = "Godot enumerator name: `FLAG_UV2_USE_WORLD_TRIPLANAR`"]
    pub const UV2_USE_WORLD_TRIPLANAR: Flags = Flags {
        ord: 9i32
    };
    #[doc(alias = "FLAG_AO_ON_UV2")]
    #[doc = "Godot enumerator name: `FLAG_AO_ON_UV2`"]
    pub const AO_ON_UV2: Flags = Flags {
        ord: 10i32
    };
    #[doc(alias = "FLAG_EMISSION_ON_UV2")]
    #[doc = "Godot enumerator name: `FLAG_EMISSION_ON_UV2`"]
    pub const EMISSION_ON_UV2: Flags = Flags {
        ord: 11i32
    };
    #[doc(alias = "FLAG_ALBEDO_TEXTURE_FORCE_SRGB")]
    #[doc = "Godot enumerator name: `FLAG_ALBEDO_TEXTURE_FORCE_SRGB`"]
    pub const ALBEDO_TEXTURE_FORCE_SRGB: Flags = Flags {
        ord: 12i32
    };
    #[doc(alias = "FLAG_DONT_RECEIVE_SHADOWS")]
    #[doc = "Godot enumerator name: `FLAG_DONT_RECEIVE_SHADOWS`"]
    pub const DONT_RECEIVE_SHADOWS: Flags = Flags {
        ord: 13i32
    };
    #[doc(alias = "FLAG_DISABLE_AMBIENT_LIGHT")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_AMBIENT_LIGHT`"]
    pub const DISABLE_AMBIENT_LIGHT: Flags = Flags {
        ord: 14i32
    };
    #[doc(alias = "FLAG_USE_SHADOW_TO_OPACITY")]
    #[doc = "Godot enumerator name: `FLAG_USE_SHADOW_TO_OPACITY`"]
    pub const USE_SHADOW_TO_OPACITY: Flags = Flags {
        ord: 15i32
    };
    #[doc(alias = "FLAG_USE_TEXTURE_REPEAT")]
    #[doc = "Godot enumerator name: `FLAG_USE_TEXTURE_REPEAT`"]
    pub const USE_TEXTURE_REPEAT: Flags = Flags {
        ord: 16i32
    };
    #[doc(alias = "FLAG_INVERT_HEIGHTMAP")]
    #[doc = "Godot enumerator name: `FLAG_INVERT_HEIGHTMAP`"]
    pub const INVERT_HEIGHTMAP: Flags = Flags {
        ord: 17i32
    };
    #[doc(alias = "FLAG_SUBSURFACE_MODE_SKIN")]
    #[doc = "Godot enumerator name: `FLAG_SUBSURFACE_MODE_SKIN`"]
    pub const SUBSURFACE_MODE_SKIN: Flags = Flags {
        ord: 18i32
    };
    #[doc(alias = "FLAG_PARTICLE_TRAILS_MODE")]
    #[doc = "Godot enumerator name: `FLAG_PARTICLE_TRAILS_MODE`"]
    pub const PARTICLE_TRAILS_MODE: Flags = Flags {
        ord: 19i32
    };
    #[doc(alias = "FLAG_ALBEDO_TEXTURE_MSDF")]
    #[doc = "Godot enumerator name: `FLAG_ALBEDO_TEXTURE_MSDF`"]
    pub const ALBEDO_TEXTURE_MSDF: Flags = Flags {
        ord: 20i32
    };
    #[doc(alias = "FLAG_DISABLE_FOG")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_FOG`"]
    pub const DISABLE_FOG: Flags = Flags {
        ord: 21i32
    };
    #[doc(alias = "FLAG_MAX")]
    #[doc = "Godot enumerator name: `FLAG_MAX`"]
    pub const MAX: Flags = Flags {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Flags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Flags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::DISABLE_DEPTH_TEST => "DISABLE_DEPTH_TEST", Self::ALBEDO_FROM_VERTEX_COLOR => "ALBEDO_FROM_VERTEX_COLOR", Self::SRGB_VERTEX_COLOR => "SRGB_VERTEX_COLOR", Self::USE_POINT_SIZE => "USE_POINT_SIZE", Self::FIXED_SIZE => "FIXED_SIZE", Self::BILLBOARD_KEEP_SCALE => "BILLBOARD_KEEP_SCALE", Self::UV1_USE_TRIPLANAR => "UV1_USE_TRIPLANAR", Self::UV2_USE_TRIPLANAR => "UV2_USE_TRIPLANAR", Self::UV1_USE_WORLD_TRIPLANAR => "UV1_USE_WORLD_TRIPLANAR", Self::UV2_USE_WORLD_TRIPLANAR => "UV2_USE_WORLD_TRIPLANAR", Self::AO_ON_UV2 => "AO_ON_UV2", Self::EMISSION_ON_UV2 => "EMISSION_ON_UV2", Self::ALBEDO_TEXTURE_FORCE_SRGB => "ALBEDO_TEXTURE_FORCE_SRGB", Self::DONT_RECEIVE_SHADOWS => "DONT_RECEIVE_SHADOWS", Self::DISABLE_AMBIENT_LIGHT => "DISABLE_AMBIENT_LIGHT", Self::USE_SHADOW_TO_OPACITY => "USE_SHADOW_TO_OPACITY", Self::USE_TEXTURE_REPEAT => "USE_TEXTURE_REPEAT", Self::INVERT_HEIGHTMAP => "INVERT_HEIGHTMAP", Self::SUBSURFACE_MODE_SKIN => "SUBSURFACE_MODE_SKIN", Self::PARTICLE_TRAILS_MODE => "PARTICLE_TRAILS_MODE", Self::ALBEDO_TEXTURE_MSDF => "ALBEDO_TEXTURE_MSDF", Self::DISABLE_FOG => "DISABLE_FOG", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLE_DEPTH_TEST => "FLAG_DISABLE_DEPTH_TEST", Self::ALBEDO_FROM_VERTEX_COLOR => "FLAG_ALBEDO_FROM_VERTEX_COLOR", Self::SRGB_VERTEX_COLOR => "FLAG_SRGB_VERTEX_COLOR", Self::USE_POINT_SIZE => "FLAG_USE_POINT_SIZE", Self::FIXED_SIZE => "FLAG_FIXED_SIZE", Self::BILLBOARD_KEEP_SCALE => "FLAG_BILLBOARD_KEEP_SCALE", Self::UV1_USE_TRIPLANAR => "FLAG_UV1_USE_TRIPLANAR", Self::UV2_USE_TRIPLANAR => "FLAG_UV2_USE_TRIPLANAR", Self::UV1_USE_WORLD_TRIPLANAR => "FLAG_UV1_USE_WORLD_TRIPLANAR", Self::UV2_USE_WORLD_TRIPLANAR => "FLAG_UV2_USE_WORLD_TRIPLANAR", Self::AO_ON_UV2 => "FLAG_AO_ON_UV2", Self::EMISSION_ON_UV2 => "FLAG_EMISSION_ON_UV2", Self::ALBEDO_TEXTURE_FORCE_SRGB => "FLAG_ALBEDO_TEXTURE_FORCE_SRGB", Self::DONT_RECEIVE_SHADOWS => "FLAG_DONT_RECEIVE_SHADOWS", Self::DISABLE_AMBIENT_LIGHT => "FLAG_DISABLE_AMBIENT_LIGHT", Self::USE_SHADOW_TO_OPACITY => "FLAG_USE_SHADOW_TO_OPACITY", Self::USE_TEXTURE_REPEAT => "FLAG_USE_TEXTURE_REPEAT", Self::INVERT_HEIGHTMAP => "FLAG_INVERT_HEIGHTMAP", Self::SUBSURFACE_MODE_SKIN => "FLAG_SUBSURFACE_MODE_SKIN", Self::PARTICLE_TRAILS_MODE => "FLAG_PARTICLE_TRAILS_MODE", Self::ALBEDO_TEXTURE_MSDF => "FLAG_ALBEDO_TEXTURE_MSDF", Self::DISABLE_FOG => "FLAG_DISABLE_FOG", Self::MAX => "FLAG_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Flags {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::meta::GodotConvert for Flags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Flags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Flags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DiffuseMode {
    ord: i32
}
impl DiffuseMode {
    #[doc(alias = "DIFFUSE_BURLEY")]
    #[doc = "Godot enumerator name: `DIFFUSE_BURLEY`"]
    pub const BURLEY: DiffuseMode = DiffuseMode {
        ord: 0i32
    };
    #[doc(alias = "DIFFUSE_LAMBERT")]
    #[doc = "Godot enumerator name: `DIFFUSE_LAMBERT`"]
    pub const LAMBERT: DiffuseMode = DiffuseMode {
        ord: 1i32
    };
    #[doc(alias = "DIFFUSE_LAMBERT_WRAP")]
    #[doc = "Godot enumerator name: `DIFFUSE_LAMBERT_WRAP`"]
    pub const LAMBERT_WRAP: DiffuseMode = DiffuseMode {
        ord: 2i32
    };
    #[doc(alias = "DIFFUSE_TOON")]
    #[doc = "Godot enumerator name: `DIFFUSE_TOON`"]
    pub const TOON: DiffuseMode = DiffuseMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DiffuseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DiffuseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DiffuseMode {
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
            Self::BURLEY => "BURLEY", Self::LAMBERT => "LAMBERT", Self::LAMBERT_WRAP => "LAMBERT_WRAP", Self::TOON => "TOON", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BURLEY => "DIFFUSE_BURLEY", Self::LAMBERT => "DIFFUSE_LAMBERT", Self::LAMBERT_WRAP => "DIFFUSE_LAMBERT_WRAP", Self::TOON => "DIFFUSE_TOON", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DiffuseMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DiffuseMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DiffuseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpecularMode {
    ord: i32
}
impl SpecularMode {
    #[doc(alias = "SPECULAR_SCHLICK_GGX")]
    #[doc = "Godot enumerator name: `SPECULAR_SCHLICK_GGX`"]
    pub const SCHLICK_GGX: SpecularMode = SpecularMode {
        ord: 0i32
    };
    #[doc(alias = "SPECULAR_TOON")]
    #[doc = "Godot enumerator name: `SPECULAR_TOON`"]
    pub const TOON: SpecularMode = SpecularMode {
        ord: 1i32
    };
    #[doc(alias = "SPECULAR_DISABLED")]
    #[doc = "Godot enumerator name: `SPECULAR_DISABLED`"]
    pub const DISABLED: SpecularMode = SpecularMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SpecularMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpecularMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpecularMode {
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
            Self::SCHLICK_GGX => "SCHLICK_GGX", Self::TOON => "TOON", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SCHLICK_GGX => "SPECULAR_SCHLICK_GGX", Self::TOON => "SPECULAR_TOON", Self::DISABLED => "SPECULAR_DISABLED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SpecularMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpecularMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpecularMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BillboardMode {
    ord: i32
}
impl BillboardMode {
    #[doc(alias = "BILLBOARD_DISABLED")]
    #[doc = "Godot enumerator name: `BILLBOARD_DISABLED`"]
    pub const DISABLED: BillboardMode = BillboardMode {
        ord: 0i32
    };
    #[doc(alias = "BILLBOARD_ENABLED")]
    #[doc = "Godot enumerator name: `BILLBOARD_ENABLED`"]
    pub const ENABLED: BillboardMode = BillboardMode {
        ord: 1i32
    };
    #[doc(alias = "BILLBOARD_FIXED_Y")]
    #[doc = "Godot enumerator name: `BILLBOARD_FIXED_Y`"]
    pub const FIXED_Y: BillboardMode = BillboardMode {
        ord: 2i32
    };
    #[doc(alias = "BILLBOARD_PARTICLES")]
    #[doc = "Godot enumerator name: `BILLBOARD_PARTICLES`"]
    pub const PARTICLES: BillboardMode = BillboardMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for BillboardMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BillboardMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BillboardMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::FIXED_Y => "FIXED_Y", Self::PARTICLES => "PARTICLES", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "BILLBOARD_DISABLED", Self::ENABLED => "BILLBOARD_ENABLED", Self::FIXED_Y => "BILLBOARD_FIXED_Y", Self::PARTICLES => "BILLBOARD_PARTICLES", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BillboardMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BillboardMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BillboardMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureChannel {
    ord: i32
}
impl TextureChannel {
    #[doc(alias = "TEXTURE_CHANNEL_RED")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_RED`"]
    pub const RED: TextureChannel = TextureChannel {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_CHANNEL_GREEN")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_GREEN`"]
    pub const GREEN: TextureChannel = TextureChannel {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_CHANNEL_BLUE")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_BLUE`"]
    pub const BLUE: TextureChannel = TextureChannel {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_CHANNEL_ALPHA")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_ALPHA`"]
    pub const ALPHA: TextureChannel = TextureChannel {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_CHANNEL_GRAYSCALE")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_GRAYSCALE`"]
    pub const GRAYSCALE: TextureChannel = TextureChannel {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TextureChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureChannel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureChannel {
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
            Self::RED => "RED", Self::GREEN => "GREEN", Self::BLUE => "BLUE", Self::ALPHA => "ALPHA", Self::GRAYSCALE => "GRAYSCALE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::RED => "TEXTURE_CHANNEL_RED", Self::GREEN => "TEXTURE_CHANNEL_GREEN", Self::BLUE => "TEXTURE_CHANNEL_BLUE", Self::ALPHA => "TEXTURE_CHANNEL_ALPHA", Self::GRAYSCALE => "TEXTURE_CHANNEL_GRAYSCALE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TextureChannel {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureChannel {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureChannel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EmissionOperator {
    ord: i32
}
impl EmissionOperator {
    #[doc(alias = "EMISSION_OP_ADD")]
    #[doc = "Godot enumerator name: `EMISSION_OP_ADD`"]
    pub const ADD: EmissionOperator = EmissionOperator {
        ord: 0i32
    };
    #[doc(alias = "EMISSION_OP_MULTIPLY")]
    #[doc = "Godot enumerator name: `EMISSION_OP_MULTIPLY`"]
    pub const MULTIPLY: EmissionOperator = EmissionOperator {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for EmissionOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EmissionOperator") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EmissionOperator {
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
            Self::ADD => "ADD", Self::MULTIPLY => "MULTIPLY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ADD => "EMISSION_OP_ADD", Self::MULTIPLY => "EMISSION_OP_MULTIPLY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for EmissionOperator {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EmissionOperator {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EmissionOperator {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DistanceFadeMode {
    ord: i32
}
impl DistanceFadeMode {
    #[doc(alias = "DISTANCE_FADE_DISABLED")]
    #[doc = "Godot enumerator name: `DISTANCE_FADE_DISABLED`"]
    pub const DISABLED: DistanceFadeMode = DistanceFadeMode {
        ord: 0i32
    };
    #[doc(alias = "DISTANCE_FADE_PIXEL_ALPHA")]
    #[doc = "Godot enumerator name: `DISTANCE_FADE_PIXEL_ALPHA`"]
    pub const PIXEL_ALPHA: DistanceFadeMode = DistanceFadeMode {
        ord: 1i32
    };
    #[doc(alias = "DISTANCE_FADE_PIXEL_DITHER")]
    #[doc = "Godot enumerator name: `DISTANCE_FADE_PIXEL_DITHER`"]
    pub const PIXEL_DITHER: DistanceFadeMode = DistanceFadeMode {
        ord: 2i32
    };
    #[doc(alias = "DISTANCE_FADE_OBJECT_DITHER")]
    #[doc = "Godot enumerator name: `DISTANCE_FADE_OBJECT_DITHER`"]
    pub const OBJECT_DITHER: DistanceFadeMode = DistanceFadeMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DistanceFadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DistanceFadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DistanceFadeMode {
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
            Self::DISABLED => "DISABLED", Self::PIXEL_ALPHA => "PIXEL_ALPHA", Self::PIXEL_DITHER => "PIXEL_DITHER", Self::OBJECT_DITHER => "OBJECT_DITHER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DISTANCE_FADE_DISABLED", Self::PIXEL_ALPHA => "DISTANCE_FADE_PIXEL_ALPHA", Self::PIXEL_DITHER => "DISTANCE_FADE_PIXEL_DITHER", Self::OBJECT_DITHER => "DISTANCE_FADE_OBJECT_DITHER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DistanceFadeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DistanceFadeMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DistanceFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}