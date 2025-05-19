#![doc = "Sidecar module for class [`XrInterface`][crate::classes::XrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_xrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRInterface.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`xr_interface`][crate::classes::xr_interface]: sidecar module with related enum/flag types\n* [`IXrInterface`][crate::classes::IXrInterface]: virtual methods\n\n\nSee also [Godot docs for `XRInterface`](https://docs.godotengine.org/en/stable/classes/class_xrinterface.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<XrInterface>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`XrInterface`][crate::classes::XrInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `XRInterface` methods](https://docs.godotengine.org/en/stable/classes/class_xrinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrInterface: crate::obj::GodotClass < Base = XrInterface > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl XrInterface {
        pub fn get_name(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_capabilities(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_capabilities", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_primary(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "is_primary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_primary(&mut self, primary: bool,) {
            type CallSig = ((), bool);
            let args = (primary,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "set_primary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_initialized(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "is_initialized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn initialize(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "initialize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn uninitialize(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "uninitialize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_info(&mut self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_system_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tracking_status(&self,) -> crate::classes::xr_interface::TrackingStatus {
            type CallSig = (crate::classes::xr_interface::TrackingStatus,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_tracking_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_target_size(&mut self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_render_target_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_view_count(&mut self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_view_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn trigger_haptic_pulse(&mut self, action_name: impl AsArg < GString >, tracker_name: impl AsArg < StringName >, frequency: f64, amplitude: f64, duration_sec: f64, delay_sec: f64,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, StringName >, f64, f64, f64, f64);
            let args = (action_name.into_arg(), tracker_name.into_arg(), frequency, amplitude, duration_sec, delay_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "trigger_haptic_pulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn supports_play_area_mode(&mut self, mode: crate::classes::xr_interface::PlayAreaMode,) -> bool {
            type CallSig = (bool, crate::classes::xr_interface::PlayAreaMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "supports_play_area_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_play_area_mode(&self,) -> crate::classes::xr_interface::PlayAreaMode {
            type CallSig = (crate::classes::xr_interface::PlayAreaMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_play_area_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_play_area_mode(&mut self, mode: crate::classes::xr_interface::PlayAreaMode,) -> bool {
            type CallSig = (bool, crate::classes::xr_interface::PlayAreaMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "set_play_area_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_play_area(&self,) -> PackedVector3Array {
            type CallSig = (PackedVector3Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_play_area", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_anchor_detection_is_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_anchor_detection_is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_anchor_detection_is_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "set_anchor_detection_is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_feed_id(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_camera_feed_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_passthrough_supported(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "is_passthrough_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_passthrough_enabled(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "is_passthrough_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn start_passthrough(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "start_passthrough", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop_passthrough(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "stop_passthrough", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform_for_view(&mut self, view: u32, cam_transform: Transform3D,) -> Transform3D {
            type CallSig = (Transform3D, u32, Transform3D);
            let args = (view, cam_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_transform_for_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_projection_for_view(&mut self, view: u32, aspect: f64, near: f64, far: f64,) -> Projection {
            type CallSig = (Projection, u32, f64, f64, f64);
            let args = (view, aspect, near, far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_projection_for_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_environment_blend_modes(&mut self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_supported_environment_blend_modes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_blend_mode(&mut self, mode: crate::classes::xr_interface::EnvironmentBlendMode,) -> bool {
            type CallSig = (bool, crate::classes::xr_interface::EnvironmentBlendMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "set_environment_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_blend_mode(&self,) -> crate::classes::xr_interface::EnvironmentBlendMode {
            type CallSig = (crate::classes::xr_interface::EnvironmentBlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterface", "get_environment_blend_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for XrInterface {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"XRInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for XrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrInterface {
        
    }
    impl std::ops::Deref for XrInterface {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`XrInterface`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_XrInterface {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrInterface > for $Class {
                
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
pub struct Capabilities {
    ord: i32
}
impl Capabilities {
    #[doc(alias = "XR_NONE")]
    #[doc = "Godot enumerator name: `XR_NONE`"]
    pub const NONE: Capabilities = Capabilities {
        ord: 0i32
    };
    #[doc(alias = "XR_MONO")]
    #[doc = "Godot enumerator name: `XR_MONO`"]
    pub const MONO: Capabilities = Capabilities {
        ord: 1i32
    };
    #[doc(alias = "XR_STEREO")]
    #[doc = "Godot enumerator name: `XR_STEREO`"]
    pub const STEREO: Capabilities = Capabilities {
        ord: 2i32
    };
    #[doc(alias = "XR_QUAD")]
    #[doc = "Godot enumerator name: `XR_QUAD`"]
    pub const QUAD: Capabilities = Capabilities {
        ord: 4i32
    };
    #[doc(alias = "XR_VR")]
    #[doc = "Godot enumerator name: `XR_VR`"]
    pub const VR: Capabilities = Capabilities {
        ord: 8i32
    };
    #[doc(alias = "XR_AR")]
    #[doc = "Godot enumerator name: `XR_AR`"]
    pub const AR: Capabilities = Capabilities {
        ord: 16i32
    };
    #[doc(alias = "XR_EXTERNAL")]
    #[doc = "Godot enumerator name: `XR_EXTERNAL`"]
    pub const EXTERNAL: Capabilities = Capabilities {
        ord: 32i32
    };
    
}
impl std::fmt::Debug for Capabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Capabilities") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Capabilities {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 => Some(Self {
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
            Self::NONE => "NONE", Self::MONO => "MONO", Self::STEREO => "STEREO", Self::QUAD => "QUAD", Self::VR => "VR", Self::AR => "AR", Self::EXTERNAL => "EXTERNAL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "XR_NONE", Self::MONO => "XR_MONO", Self::STEREO => "XR_STEREO", Self::QUAD => "XR_QUAD", Self::VR => "XR_VR", Self::AR => "XR_AR", Self::EXTERNAL => "XR_EXTERNAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Capabilities {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Capabilities {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Capabilities {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TrackingStatus {
    ord: i32
}
impl TrackingStatus {
    #[doc(alias = "XR_NORMAL_TRACKING")]
    #[doc = "Godot enumerator name: `XR_NORMAL_TRACKING`"]
    pub const NORMAL_TRACKING: TrackingStatus = TrackingStatus {
        ord: 0i32
    };
    #[doc(alias = "XR_EXCESSIVE_MOTION")]
    #[doc = "Godot enumerator name: `XR_EXCESSIVE_MOTION`"]
    pub const EXCESSIVE_MOTION: TrackingStatus = TrackingStatus {
        ord: 1i32
    };
    #[doc(alias = "XR_INSUFFICIENT_FEATURES")]
    #[doc = "Godot enumerator name: `XR_INSUFFICIENT_FEATURES`"]
    pub const INSUFFICIENT_FEATURES: TrackingStatus = TrackingStatus {
        ord: 2i32
    };
    #[doc(alias = "XR_UNKNOWN_TRACKING")]
    #[doc = "Godot enumerator name: `XR_UNKNOWN_TRACKING`"]
    pub const UNKNOWN_TRACKING: TrackingStatus = TrackingStatus {
        ord: 3i32
    };
    #[doc(alias = "XR_NOT_TRACKING")]
    #[doc = "Godot enumerator name: `XR_NOT_TRACKING`"]
    pub const NOT_TRACKING: TrackingStatus = TrackingStatus {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TrackingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TrackingStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TrackingStatus {
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
            Self::NORMAL_TRACKING => "NORMAL_TRACKING", Self::EXCESSIVE_MOTION => "EXCESSIVE_MOTION", Self::INSUFFICIENT_FEATURES => "INSUFFICIENT_FEATURES", Self::UNKNOWN_TRACKING => "UNKNOWN_TRACKING", Self::NOT_TRACKING => "NOT_TRACKING", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NORMAL_TRACKING => "XR_NORMAL_TRACKING", Self::EXCESSIVE_MOTION => "XR_EXCESSIVE_MOTION", Self::INSUFFICIENT_FEATURES => "XR_INSUFFICIENT_FEATURES", Self::UNKNOWN_TRACKING => "XR_UNKNOWN_TRACKING", Self::NOT_TRACKING => "XR_NOT_TRACKING", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TrackingStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TrackingStatus {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TrackingStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlayAreaMode {
    ord: i32
}
impl PlayAreaMode {
    #[doc(alias = "XR_PLAY_AREA_UNKNOWN")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_UNKNOWN`"]
    pub const UNKNOWN: PlayAreaMode = PlayAreaMode {
        ord: 0i32
    };
    #[doc(alias = "XR_PLAY_AREA_3DOF")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_3DOF`"]
    pub const AREA_3DOF: PlayAreaMode = PlayAreaMode {
        ord: 1i32
    };
    #[doc(alias = "XR_PLAY_AREA_SITTING")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_SITTING`"]
    pub const SITTING: PlayAreaMode = PlayAreaMode {
        ord: 2i32
    };
    #[doc(alias = "XR_PLAY_AREA_ROOMSCALE")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_ROOMSCALE`"]
    pub const ROOMSCALE: PlayAreaMode = PlayAreaMode {
        ord: 3i32
    };
    #[doc(alias = "XR_PLAY_AREA_STAGE")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_STAGE`"]
    pub const STAGE: PlayAreaMode = PlayAreaMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for PlayAreaMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlayAreaMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlayAreaMode {
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
            Self::UNKNOWN => "UNKNOWN", Self::AREA_3DOF => "AREA_3DOF", Self::SITTING => "SITTING", Self::ROOMSCALE => "ROOMSCALE", Self::STAGE => "STAGE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UNKNOWN => "XR_PLAY_AREA_UNKNOWN", Self::AREA_3DOF => "XR_PLAY_AREA_3DOF", Self::SITTING => "XR_PLAY_AREA_SITTING", Self::ROOMSCALE => "XR_PLAY_AREA_ROOMSCALE", Self::STAGE => "XR_PLAY_AREA_STAGE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PlayAreaMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PlayAreaMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlayAreaMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentBlendMode {
    ord: i32
}
impl EnvironmentBlendMode {
    #[doc(alias = "XR_ENV_BLEND_MODE_OPAQUE")]
    #[doc = "Godot enumerator name: `XR_ENV_BLEND_MODE_OPAQUE`"]
    pub const OPAQUE: EnvironmentBlendMode = EnvironmentBlendMode {
        ord: 0i32
    };
    #[doc(alias = "XR_ENV_BLEND_MODE_ADDITIVE")]
    #[doc = "Godot enumerator name: `XR_ENV_BLEND_MODE_ADDITIVE`"]
    pub const ADDITIVE: EnvironmentBlendMode = EnvironmentBlendMode {
        ord: 1i32
    };
    #[doc(alias = "XR_ENV_BLEND_MODE_ALPHA_BLEND")]
    #[doc = "Godot enumerator name: `XR_ENV_BLEND_MODE_ALPHA_BLEND`"]
    pub const ALPHA_BLEND: EnvironmentBlendMode = EnvironmentBlendMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for EnvironmentBlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentBlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentBlendMode {
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
            Self::OPAQUE => "OPAQUE", Self::ADDITIVE => "ADDITIVE", Self::ALPHA_BLEND => "ALPHA_BLEND", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OPAQUE => "XR_ENV_BLEND_MODE_OPAQUE", Self::ADDITIVE => "XR_ENV_BLEND_MODE_ADDITIVE", Self::ALPHA_BLEND => "XR_ENV_BLEND_MODE_ALPHA_BLEND", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentBlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentBlendMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}