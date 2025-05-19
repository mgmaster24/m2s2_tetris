#![doc = "Sidecar module for class [`OpenXrApiExtension`][crate::classes::OpenXrApiExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRAPIExtension` enums](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRAPIExtension.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`open_xr_api_extension`][crate::classes::open_xr_api_extension]: sidecar module with related enum/flag types\n* [`IOpenXrApiExtension`][crate::classes::IOpenXrApiExtension]: virtual methods\n\n\nSee also [Godot docs for `OpenXRAPIExtension`](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrApiExtension::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrApiExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrApiExtension`][crate::classes::OpenXrApiExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRAPIExtension` methods](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrApiExtension: crate::obj::GodotClass < Base = OpenXrApiExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrApiExtension {
        pub fn get_instance(&mut self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_id(&mut self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_system_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_session(&mut self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_session", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        pub unsafe fn transform_from_pose(&mut self, pose: * const c_void,) -> Transform3D {
            type CallSig = (Transform3D, * const c_void);
            let args = (pose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "transform_from_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn xr_result(&mut self, result: u64, format: impl AsArg < GString >, args: &VariantArray,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, u64, CowArg < 'a0, GString >, RefArg < 'a1, VariantArray >);
            let args = (result, format.into_arg(), RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "xr_result", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn openxr_is_enabled(check_run_in_editor: bool,) -> bool {
            type CallSig = (bool, bool);
            let args = (check_run_in_editor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_is_enabled", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_instance_proc_addr(&mut self, name: impl AsArg < GString >,) -> u64 {
            type CallSig < 'a0, > = (u64, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_instance_proc_addr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_error_string(&mut self, result: u64,) -> GString {
            type CallSig = (GString, u64);
            let args = (result,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_error_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_swapchain_format_name(&mut self, swapchain_format: i64,) -> GString {
            type CallSig = (GString, i64);
            let args = (swapchain_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_swapchain_format_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_initialized(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_initialized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_running(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_running", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_play_space(&mut self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_play_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_predicted_display_time(&mut self,) -> i64 {
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_predicted_display_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next_frame_time(&mut self,) -> i64 {
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_next_frame_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_render(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "can_render", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_tracker(&mut self, hand_index: i32,) -> u64 {
            type CallSig = (u64, i32);
            let args = (hand_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_hand_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_composition_layer_provider(&mut self, extension: impl AsObjectArg < crate::classes::OpenXrExtensionWrapperExtension >,) {
            type CallSig = ((), ObjectArg < crate::classes::OpenXrExtensionWrapperExtension >);
            let args = (extension.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "register_composition_layer_provider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_composition_layer_provider(&mut self, extension: impl AsObjectArg < crate::classes::OpenXrExtensionWrapperExtension >,) {
            type CallSig = ((), ObjectArg < crate::classes::OpenXrExtensionWrapperExtension >);
            let args = (extension.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "unregister_composition_layer_provider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emulate_environment_blend_mode_alpha_blend(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_emulate_environment_blend_mode_alpha_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_environment_blend_mode_alpha_supported(&mut self,) -> crate::classes::open_xr_api_extension::OpenXrAlphaBlendModeSupport {
            type CallSig = (crate::classes::open_xr_api_extension::OpenXrAlphaBlendModeSupport,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_environment_blend_mode_alpha_supported", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrApiExtension {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"OpenXRAPIExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrApiExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrApiExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrApiExtension {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrApiExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrApiExtension {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrApiExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrApiExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_OpenXrApiExtension {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrApiExtension > for $Class {
                
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
#[doc = "Godot enum name: `OpenXRAlphaBlendModeSupport`."]
pub struct OpenXrAlphaBlendModeSupport {
    ord: i32
}
impl OpenXrAlphaBlendModeSupport {
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE`"]
    pub const NONE: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 0i32
    };
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL`"]
    pub const REAL: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 1i32
    };
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING`"]
    pub const EMULATING: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for OpenXrAlphaBlendModeSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OpenXrAlphaBlendModeSupport") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OpenXrAlphaBlendModeSupport {
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
            Self::NONE => "NONE", Self::REAL => "REAL", Self::EMULATING => "EMULATING", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE", Self::REAL => "OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL", Self::EMULATING => "OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for OpenXrAlphaBlendModeSupport {
    type Via = i32;
    
}
impl crate::meta::ToGodot for OpenXrAlphaBlendModeSupport {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OpenXrAlphaBlendModeSupport {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}