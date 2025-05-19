#![doc = "Sidecar module for class [`OpenXrInterface`][crate::classes::OpenXrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRInterface.`\n\nInherits [`XrInterface`][crate::classes::XrInterface].\n\nRelated symbols:\n\n* [`open_xr_interface`][crate::classes::open_xr_interface]: sidecar module with related enum/flag types\n* [`IOpenXrInterface`][crate::classes::IOpenXrInterface]: virtual methods\n\n\nSee also [Godot docs for `OpenXRInterface`](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrInterface::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrInterface`][crate::classes::OpenXrInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRInterface` methods](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrInterface: crate::obj::GodotClass < Base = OpenXrInterface > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrInterface {
        pub fn get_display_refresh_rate(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_display_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_display_refresh_rate(&mut self, refresh_rate: f32,) {
            type CallSig = ((), f32);
            let args = (refresh_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_display_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_target_size_multiplier(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_render_target_size_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_render_target_size_multiplier(&mut self, multiplier: f64,) {
            type CallSig = ((), f64);
            let args = (multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_render_target_size_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_foveation_supported(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_foveation_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_foveation_level(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_foveation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_foveation_level(&mut self, foveation_level: i32,) {
            type CallSig = ((), i32);
            let args = (foveation_level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_foveation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_foveation_dynamic(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_foveation_dynamic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_foveation_dynamic(&mut self, foveation_dynamic: bool,) {
            type CallSig = ((), bool);
            let args = (foveation_dynamic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_foveation_dynamic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_action_set_active(&self, name: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_action_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_action_set_active(&mut self, name: impl AsArg < GString >, active: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (name.into_arg(), active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_action_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action_sets(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_action_sets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_available_display_refresh_rates(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_available_display_refresh_rates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_range(&mut self, hand: crate::classes::open_xr_interface::Hand, motion_range: crate::classes::open_xr_interface::HandMotionRange,) {
            type CallSig = ((), crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandMotionRange);
            let args = (hand, motion_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_motion_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_range(&self, hand: crate::classes::open_xr_interface::Hand,) -> crate::classes::open_xr_interface::HandMotionRange {
            type CallSig = (crate::classes::open_xr_interface::HandMotionRange, crate::classes::open_xr_interface::Hand);
            let args = (hand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_motion_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_tracking_source(&self, hand: crate::classes::open_xr_interface::Hand,) -> crate::classes::open_xr_interface::HandTrackedSource {
            type CallSig = (crate::classes::open_xr_interface::HandTrackedSource, crate::classes::open_xr_interface::Hand);
            let args = (hand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_tracking_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_flags(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> crate::classes::open_xr_interface::HandJointFlags {
            type CallSig = (crate::classes::open_xr_interface::HandJointFlags, crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_rotation(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> Quaternion {
            type CallSig = (Quaternion, crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_position(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> Vector3 {
            type CallSig = (Vector3, crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_radius(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> f32 {
            type CallSig = (f32, crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_linear_velocity(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> Vector3 {
            type CallSig = (Vector3, crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_angular_velocity(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> Vector3 {
            type CallSig = (Vector3, crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hand_tracking_supported(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_hand_tracking_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hand_interaction_supported(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_hand_interaction_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_eye_gaze_interaction_supported(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_eye_gaze_interaction_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vrs_min_radius(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_vrs_min_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vrs_min_radius(&mut self, radius: f32,) {
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_vrs_min_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vrs_strength(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_vrs_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vrs_strength(&mut self, strength: f32,) {
            type CallSig = ((), f32);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_vrs_strength", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrInterface {
        type Base = crate::classes::XrInterface;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"OpenXRInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrInterface > for OpenXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrInterface {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrInterface {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrInterface {
        type Target = crate::classes::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrInterface`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_OpenXrInterface {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrInterface > for $Class {
                
            }
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
pub struct Hand {
    ord: i32
}
impl Hand {
    #[doc(alias = "HAND_LEFT")]
    #[doc = "Godot enumerator name: `HAND_LEFT`"]
    pub const LEFT: Hand = Hand {
        ord: 0i32
    };
    #[doc(alias = "HAND_RIGHT")]
    #[doc = "Godot enumerator name: `HAND_RIGHT`"]
    pub const RIGHT: Hand = Hand {
        ord: 1i32
    };
    #[doc(alias = "HAND_MAX")]
    #[doc = "Godot enumerator name: `HAND_MAX`"]
    pub const MAX: Hand = Hand {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Hand") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Hand {
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
            Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LEFT => "HAND_LEFT", Self::RIGHT => "HAND_RIGHT", Self::MAX => "HAND_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Hand {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for Hand {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Hand {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Hand {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandMotionRange {
    ord: i32
}
impl HandMotionRange {
    #[doc(alias = "HAND_MOTION_RANGE_UNOBSTRUCTED")]
    #[doc = "Godot enumerator name: `HAND_MOTION_RANGE_UNOBSTRUCTED`"]
    pub const UNOBSTRUCTED: HandMotionRange = HandMotionRange {
        ord: 0i32
    };
    #[doc(alias = "HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER")]
    #[doc = "Godot enumerator name: `HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER`"]
    pub const CONFORM_TO_CONTROLLER: HandMotionRange = HandMotionRange {
        ord: 1i32
    };
    #[doc(alias = "HAND_MOTION_RANGE_MAX")]
    #[doc = "Godot enumerator name: `HAND_MOTION_RANGE_MAX`"]
    pub const MAX: HandMotionRange = HandMotionRange {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for HandMotionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandMotionRange") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandMotionRange {
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
            Self::UNOBSTRUCTED => "UNOBSTRUCTED", Self::CONFORM_TO_CONTROLLER => "CONFORM_TO_CONTROLLER", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UNOBSTRUCTED => "HAND_MOTION_RANGE_UNOBSTRUCTED", Self::CONFORM_TO_CONTROLLER => "HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER", Self::MAX => "HAND_MOTION_RANGE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for HandMotionRange {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for HandMotionRange {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandMotionRange {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandMotionRange {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandTrackedSource {
    ord: i32
}
impl HandTrackedSource {
    #[doc(alias = "HAND_TRACKED_SOURCE_UNKNOWN")]
    #[doc = "Godot enumerator name: `HAND_TRACKED_SOURCE_UNKNOWN`"]
    pub const UNKNOWN: HandTrackedSource = HandTrackedSource {
        ord: 0i32
    };
    #[doc(alias = "HAND_TRACKED_SOURCE_UNOBSTRUCTED")]
    #[doc = "Godot enumerator name: `HAND_TRACKED_SOURCE_UNOBSTRUCTED`"]
    pub const UNOBSTRUCTED: HandTrackedSource = HandTrackedSource {
        ord: 1i32
    };
    #[doc(alias = "HAND_TRACKED_SOURCE_CONTROLLER")]
    #[doc = "Godot enumerator name: `HAND_TRACKED_SOURCE_CONTROLLER`"]
    pub const CONTROLLER: HandTrackedSource = HandTrackedSource {
        ord: 2i32
    };
    #[doc(alias = "HAND_TRACKED_SOURCE_MAX")]
    #[doc = "Godot enumerator name: `HAND_TRACKED_SOURCE_MAX`"]
    pub const MAX: HandTrackedSource = HandTrackedSource {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for HandTrackedSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandTrackedSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandTrackedSource {
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
            Self::UNKNOWN => "UNKNOWN", Self::UNOBSTRUCTED => "UNOBSTRUCTED", Self::CONTROLLER => "CONTROLLER", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UNKNOWN => "HAND_TRACKED_SOURCE_UNKNOWN", Self::UNOBSTRUCTED => "HAND_TRACKED_SOURCE_UNOBSTRUCTED", Self::CONTROLLER => "HAND_TRACKED_SOURCE_CONTROLLER", Self::MAX => "HAND_TRACKED_SOURCE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for HandTrackedSource {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for HandTrackedSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandTrackedSource {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandTrackedSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandJoints {
    ord: i32
}
impl HandJoints {
    #[doc(alias = "HAND_JOINT_PALM")]
    #[doc = "Godot enumerator name: `HAND_JOINT_PALM`"]
    pub const PALM: HandJoints = HandJoints {
        ord: 0i32
    };
    #[doc(alias = "HAND_JOINT_WRIST")]
    #[doc = "Godot enumerator name: `HAND_JOINT_WRIST`"]
    pub const WRIST: HandJoints = HandJoints {
        ord: 1i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_METACARPAL`"]
    pub const THUMB_METACARPAL: HandJoints = HandJoints {
        ord: 2i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_PROXIMAL`"]
    pub const THUMB_PROXIMAL: HandJoints = HandJoints {
        ord: 3i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_DISTAL`"]
    pub const THUMB_DISTAL: HandJoints = HandJoints {
        ord: 4i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_TIP`"]
    pub const THUMB_TIP: HandJoints = HandJoints {
        ord: 5i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_METACARPAL`"]
    pub const INDEX_METACARPAL: HandJoints = HandJoints {
        ord: 6i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_PROXIMAL`"]
    pub const INDEX_PROXIMAL: HandJoints = HandJoints {
        ord: 7i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_INTERMEDIATE`"]
    pub const INDEX_INTERMEDIATE: HandJoints = HandJoints {
        ord: 8i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_DISTAL`"]
    pub const INDEX_DISTAL: HandJoints = HandJoints {
        ord: 9i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_TIP`"]
    pub const INDEX_TIP: HandJoints = HandJoints {
        ord: 10i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_METACARPAL`"]
    pub const MIDDLE_METACARPAL: HandJoints = HandJoints {
        ord: 11i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_PROXIMAL`"]
    pub const MIDDLE_PROXIMAL: HandJoints = HandJoints {
        ord: 12i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_INTERMEDIATE`"]
    pub const MIDDLE_INTERMEDIATE: HandJoints = HandJoints {
        ord: 13i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_DISTAL`"]
    pub const MIDDLE_DISTAL: HandJoints = HandJoints {
        ord: 14i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_TIP`"]
    pub const MIDDLE_TIP: HandJoints = HandJoints {
        ord: 15i32
    };
    #[doc(alias = "HAND_JOINT_RING_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_METACARPAL`"]
    pub const RING_METACARPAL: HandJoints = HandJoints {
        ord: 16i32
    };
    #[doc(alias = "HAND_JOINT_RING_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_PROXIMAL`"]
    pub const RING_PROXIMAL: HandJoints = HandJoints {
        ord: 17i32
    };
    #[doc(alias = "HAND_JOINT_RING_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_INTERMEDIATE`"]
    pub const RING_INTERMEDIATE: HandJoints = HandJoints {
        ord: 18i32
    };
    #[doc(alias = "HAND_JOINT_RING_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_DISTAL`"]
    pub const RING_DISTAL: HandJoints = HandJoints {
        ord: 19i32
    };
    #[doc(alias = "HAND_JOINT_RING_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_TIP`"]
    pub const RING_TIP: HandJoints = HandJoints {
        ord: 20i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_METACARPAL`"]
    pub const LITTLE_METACARPAL: HandJoints = HandJoints {
        ord: 21i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_PROXIMAL`"]
    pub const LITTLE_PROXIMAL: HandJoints = HandJoints {
        ord: 22i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_INTERMEDIATE`"]
    pub const LITTLE_INTERMEDIATE: HandJoints = HandJoints {
        ord: 23i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_DISTAL`"]
    pub const LITTLE_DISTAL: HandJoints = HandJoints {
        ord: 24i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_TIP`"]
    pub const LITTLE_TIP: HandJoints = HandJoints {
        ord: 25i32
    };
    #[doc(alias = "HAND_JOINT_MAX")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MAX`"]
    pub const MAX: HandJoints = HandJoints {
        ord: 26i32
    };
    
}
impl std::fmt::Debug for HandJoints {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandJoints") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandJoints {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
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
            Self::PALM => "PALM", Self::WRIST => "WRIST", Self::THUMB_METACARPAL => "THUMB_METACARPAL", Self::THUMB_PROXIMAL => "THUMB_PROXIMAL", Self::THUMB_DISTAL => "THUMB_DISTAL", Self::THUMB_TIP => "THUMB_TIP", Self::INDEX_METACARPAL => "INDEX_METACARPAL", Self::INDEX_PROXIMAL => "INDEX_PROXIMAL", Self::INDEX_INTERMEDIATE => "INDEX_INTERMEDIATE", Self::INDEX_DISTAL => "INDEX_DISTAL", Self::INDEX_TIP => "INDEX_TIP", Self::MIDDLE_METACARPAL => "MIDDLE_METACARPAL", Self::MIDDLE_PROXIMAL => "MIDDLE_PROXIMAL", Self::MIDDLE_INTERMEDIATE => "MIDDLE_INTERMEDIATE", Self::MIDDLE_DISTAL => "MIDDLE_DISTAL", Self::MIDDLE_TIP => "MIDDLE_TIP", Self::RING_METACARPAL => "RING_METACARPAL", Self::RING_PROXIMAL => "RING_PROXIMAL", Self::RING_INTERMEDIATE => "RING_INTERMEDIATE", Self::RING_DISTAL => "RING_DISTAL", Self::RING_TIP => "RING_TIP", Self::LITTLE_METACARPAL => "LITTLE_METACARPAL", Self::LITTLE_PROXIMAL => "LITTLE_PROXIMAL", Self::LITTLE_INTERMEDIATE => "LITTLE_INTERMEDIATE", Self::LITTLE_DISTAL => "LITTLE_DISTAL", Self::LITTLE_TIP => "LITTLE_TIP", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PALM => "HAND_JOINT_PALM", Self::WRIST => "HAND_JOINT_WRIST", Self::THUMB_METACARPAL => "HAND_JOINT_THUMB_METACARPAL", Self::THUMB_PROXIMAL => "HAND_JOINT_THUMB_PROXIMAL", Self::THUMB_DISTAL => "HAND_JOINT_THUMB_DISTAL", Self::THUMB_TIP => "HAND_JOINT_THUMB_TIP", Self::INDEX_METACARPAL => "HAND_JOINT_INDEX_METACARPAL", Self::INDEX_PROXIMAL => "HAND_JOINT_INDEX_PROXIMAL", Self::INDEX_INTERMEDIATE => "HAND_JOINT_INDEX_INTERMEDIATE", Self::INDEX_DISTAL => "HAND_JOINT_INDEX_DISTAL", Self::INDEX_TIP => "HAND_JOINT_INDEX_TIP", Self::MIDDLE_METACARPAL => "HAND_JOINT_MIDDLE_METACARPAL", Self::MIDDLE_PROXIMAL => "HAND_JOINT_MIDDLE_PROXIMAL", Self::MIDDLE_INTERMEDIATE => "HAND_JOINT_MIDDLE_INTERMEDIATE", Self::MIDDLE_DISTAL => "HAND_JOINT_MIDDLE_DISTAL", Self::MIDDLE_TIP => "HAND_JOINT_MIDDLE_TIP", Self::RING_METACARPAL => "HAND_JOINT_RING_METACARPAL", Self::RING_PROXIMAL => "HAND_JOINT_RING_PROXIMAL", Self::RING_INTERMEDIATE => "HAND_JOINT_RING_INTERMEDIATE", Self::RING_DISTAL => "HAND_JOINT_RING_DISTAL", Self::RING_TIP => "HAND_JOINT_RING_TIP", Self::LITTLE_METACARPAL => "HAND_JOINT_LITTLE_METACARPAL", Self::LITTLE_PROXIMAL => "HAND_JOINT_LITTLE_PROXIMAL", Self::LITTLE_INTERMEDIATE => "HAND_JOINT_LITTLE_INTERMEDIATE", Self::LITTLE_DISTAL => "HAND_JOINT_LITTLE_DISTAL", Self::LITTLE_TIP => "HAND_JOINT_LITTLE_TIP", Self::MAX => "HAND_JOINT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for HandJoints {
    const ENUMERATOR_COUNT: usize = 26usize;
    
}
impl crate::meta::GodotConvert for HandJoints {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandJoints {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandJoints {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct HandJointFlags {
    ord: u64
}
impl HandJointFlags {
    #[doc(alias = "HAND_JOINT_NONE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_NONE`"]
    pub const NONE: HandJointFlags = HandJointFlags {
        ord: 0u64
    };
    #[doc(alias = "HAND_JOINT_ORIENTATION_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_ORIENTATION_VALID`"]
    pub const ORIENTATION_VALID: HandJointFlags = HandJointFlags {
        ord: 1u64
    };
    #[doc(alias = "HAND_JOINT_ORIENTATION_TRACKED")]
    #[doc = "Godot enumerator name: `HAND_JOINT_ORIENTATION_TRACKED`"]
    pub const ORIENTATION_TRACKED: HandJointFlags = HandJointFlags {
        ord: 2u64
    };
    #[doc(alias = "HAND_JOINT_POSITION_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_POSITION_VALID`"]
    pub const POSITION_VALID: HandJointFlags = HandJointFlags {
        ord: 4u64
    };
    #[doc(alias = "HAND_JOINT_POSITION_TRACKED")]
    #[doc = "Godot enumerator name: `HAND_JOINT_POSITION_TRACKED`"]
    pub const POSITION_TRACKED: HandJointFlags = HandJointFlags {
        ord: 8u64
    };
    #[doc(alias = "HAND_JOINT_LINEAR_VELOCITY_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LINEAR_VELOCITY_VALID`"]
    pub const LINEAR_VELOCITY_VALID: HandJointFlags = HandJointFlags {
        ord: 16u64
    };
    #[doc(alias = "HAND_JOINT_ANGULAR_VELOCITY_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_ANGULAR_VELOCITY_VALID`"]
    pub const ANGULAR_VELOCITY_VALID: HandJointFlags = HandJointFlags {
        ord: 32u64
    };
    
}
impl std::fmt::Debug for HandJointFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::ORIENTATION_VALID => "ORIENTATION_VALID", Self::ORIENTATION_TRACKED => "ORIENTATION_TRACKED", Self::POSITION_VALID => "POSITION_VALID", Self::POSITION_TRACKED => "POSITION_TRACKED", Self::LINEAR_VELOCITY_VALID => "LINEAR_VELOCITY_VALID", Self::ANGULAR_VELOCITY_VALID => "ANGULAR_VELOCITY_VALID", _ => {
                f.debug_struct("HandJointFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for HandJointFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for HandJointFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for HandJointFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for HandJointFlags {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandJointFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}