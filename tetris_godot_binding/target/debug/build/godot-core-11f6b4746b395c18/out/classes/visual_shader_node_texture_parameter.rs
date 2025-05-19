#![doc = "Sidecar module for class [`VisualShaderNodeTextureParameter`][crate::classes::VisualShaderNodeTextureParameter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeTextureParameter.`\n\nInherits [`VisualShaderNodeParameter`][crate::classes::VisualShaderNodeParameter].\n\nRelated symbols:\n\n* [`visual_shader_node_texture_parameter`][crate::classes::visual_shader_node_texture_parameter]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeTextureParameter`][crate::classes::IVisualShaderNodeTextureParameter]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNodeTextureParameter>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeTextureParameter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeTextureParameter`][crate::classes::VisualShaderNodeTextureParameter].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeTextureParameter: crate::obj::GodotClass < Base = VisualShaderNodeTextureParameter > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl VisualShaderNodeTextureParameter {
        pub fn set_texture_type(&mut self, type_: crate::classes::visual_shader_node_texture_parameter::TextureType,) {
            type CallSig = ((), crate::classes::visual_shader_node_texture_parameter::TextureType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_texture_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_type(&self,) -> crate::classes::visual_shader_node_texture_parameter::TextureType {
            type CallSig = (crate::classes::visual_shader_node_texture_parameter::TextureType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_texture_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_default(&mut self, color: crate::classes::visual_shader_node_texture_parameter::ColorDefault,) {
            type CallSig = ((), crate::classes::visual_shader_node_texture_parameter::ColorDefault);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_color_default", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_default(&self,) -> crate::classes::visual_shader_node_texture_parameter::ColorDefault {
            type CallSig = (crate::classes::visual_shader_node_texture_parameter::ColorDefault,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_color_default", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_filter(&mut self, filter: crate::classes::visual_shader_node_texture_parameter::TextureFilter,) {
            type CallSig = ((), crate::classes::visual_shader_node_texture_parameter::TextureFilter);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::visual_shader_node_texture_parameter::TextureFilter {
            type CallSig = (crate::classes::visual_shader_node_texture_parameter::TextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_repeat(&mut self, repeat: crate::classes::visual_shader_node_texture_parameter::TextureRepeat,) {
            type CallSig = ((), crate::classes::visual_shader_node_texture_parameter::TextureRepeat);
            let args = (repeat,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_repeat(&self,) -> crate::classes::visual_shader_node_texture_parameter::TextureRepeat {
            type CallSig = (crate::classes::visual_shader_node_texture_parameter::TextureRepeat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_source(&mut self, source: crate::classes::visual_shader_node_texture_parameter::TextureSource,) {
            type CallSig = ((), crate::classes::visual_shader_node_texture_parameter::TextureSource);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_texture_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_source(&self,) -> crate::classes::visual_shader_node_texture_parameter::TextureSource {
            type CallSig = (crate::classes::visual_shader_node_texture_parameter::TextureSource,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_texture_source", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeTextureParameter {
        type Base = crate::classes::VisualShaderNodeParameter;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"VisualShaderNodeTextureParameter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeTextureParameter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNodeParameter > for VisualShaderNodeTextureParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeTextureParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeTextureParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeTextureParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeTextureParameter {
        
    }
    impl std::ops::Deref for VisualShaderNodeTextureParameter {
        type Target = crate::classes::VisualShaderNodeParameter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeTextureParameter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeTextureParameter`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_VisualShaderNodeTextureParameter {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeTextureParameter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeParameter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNode > for $Class {
                
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
pub struct TextureType {
    ord: i32
}
impl TextureType {
    #[doc(alias = "TYPE_DATA")]
    #[doc = "Godot enumerator name: `TYPE_DATA`"]
    pub const DATA: TextureType = TextureType {
        ord: 0i32
    };
    #[doc(alias = "TYPE_COLOR")]
    #[doc = "Godot enumerator name: `TYPE_COLOR`"]
    pub const COLOR: TextureType = TextureType {
        ord: 1i32
    };
    #[doc(alias = "TYPE_NORMAL_MAP")]
    #[doc = "Godot enumerator name: `TYPE_NORMAL_MAP`"]
    pub const NORMAL_MAP: TextureType = TextureType {
        ord: 2i32
    };
    #[doc(alias = "TYPE_ANISOTROPY")]
    #[doc = "Godot enumerator name: `TYPE_ANISOTROPY`"]
    pub const ANISOTROPY: TextureType = TextureType {
        ord: 3i32
    };
    #[doc(alias = "TYPE_MAX")]
    #[doc = "Godot enumerator name: `TYPE_MAX`"]
    pub const MAX: TextureType = TextureType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureType {
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
            Self::DATA => "DATA", Self::COLOR => "COLOR", Self::NORMAL_MAP => "NORMAL_MAP", Self::ANISOTROPY => "ANISOTROPY", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DATA => "TYPE_DATA", Self::COLOR => "TYPE_COLOR", Self::NORMAL_MAP => "TYPE_NORMAL_MAP", Self::ANISOTROPY => "TYPE_ANISOTROPY", Self::MAX => "TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureType {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for TextureType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ColorDefault {
    ord: i32
}
impl ColorDefault {
    #[doc(alias = "COLOR_DEFAULT_WHITE")]
    #[doc = "Godot enumerator name: `COLOR_DEFAULT_WHITE`"]
    pub const WHITE: ColorDefault = ColorDefault {
        ord: 0i32
    };
    #[doc(alias = "COLOR_DEFAULT_BLACK")]
    #[doc = "Godot enumerator name: `COLOR_DEFAULT_BLACK`"]
    pub const BLACK: ColorDefault = ColorDefault {
        ord: 1i32
    };
    #[doc(alias = "COLOR_DEFAULT_TRANSPARENT")]
    #[doc = "Godot enumerator name: `COLOR_DEFAULT_TRANSPARENT`"]
    pub const TRANSPARENT: ColorDefault = ColorDefault {
        ord: 2i32
    };
    #[doc(alias = "COLOR_DEFAULT_MAX")]
    #[doc = "Godot enumerator name: `COLOR_DEFAULT_MAX`"]
    pub const MAX: ColorDefault = ColorDefault {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ColorDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ColorDefault") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ColorDefault {
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
            Self::WHITE => "WHITE", Self::BLACK => "BLACK", Self::TRANSPARENT => "TRANSPARENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::WHITE => "COLOR_DEFAULT_WHITE", Self::BLACK => "COLOR_DEFAULT_BLACK", Self::TRANSPARENT => "COLOR_DEFAULT_TRANSPARENT", Self::MAX => "COLOR_DEFAULT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for ColorDefault {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ColorDefault {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ColorDefault {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ColorDefault {
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
    #[doc(alias = "FILTER_DEFAULT")]
    #[doc = "Godot enumerator name: `FILTER_DEFAULT`"]
    pub const DEFAULT: TextureFilter = TextureFilter {
        ord: 0i32
    };
    #[doc(alias = "FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `FILTER_NEAREST`"]
    pub const NEAREST: TextureFilter = TextureFilter {
        ord: 1i32
    };
    #[doc(alias = "FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `FILTER_LINEAR`"]
    pub const LINEAR: TextureFilter = TextureFilter {
        ord: 2i32
    };
    #[doc(alias = "FILTER_NEAREST_MIPMAP")]
    #[doc = "Godot enumerator name: `FILTER_NEAREST_MIPMAP`"]
    pub const NEAREST_MIPMAP: TextureFilter = TextureFilter {
        ord: 3i32
    };
    #[doc(alias = "FILTER_LINEAR_MIPMAP")]
    #[doc = "Godot enumerator name: `FILTER_LINEAR_MIPMAP`"]
    pub const LINEAR_MIPMAP: TextureFilter = TextureFilter {
        ord: 4i32
    };
    #[doc(alias = "FILTER_NEAREST_MIPMAP_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `FILTER_NEAREST_MIPMAP_ANISOTROPIC`"]
    pub const NEAREST_MIPMAP_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 5i32
    };
    #[doc(alias = "FILTER_LINEAR_MIPMAP_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `FILTER_LINEAR_MIPMAP_ANISOTROPIC`"]
    pub const LINEAR_MIPMAP_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 6i32
    };
    #[doc(alias = "FILTER_MAX")]
    #[doc = "Godot enumerator name: `FILTER_MAX`"]
    pub const MAX: TextureFilter = TextureFilter {
        ord: 7i32
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
            Self::DEFAULT => "DEFAULT", Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_MIPMAP => "NEAREST_MIPMAP", Self::LINEAR_MIPMAP => "LINEAR_MIPMAP", Self::NEAREST_MIPMAP_ANISOTROPIC => "NEAREST_MIPMAP_ANISOTROPIC", Self::LINEAR_MIPMAP_ANISOTROPIC => "LINEAR_MIPMAP_ANISOTROPIC", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "FILTER_DEFAULT", Self::NEAREST => "FILTER_NEAREST", Self::LINEAR => "FILTER_LINEAR", Self::NEAREST_MIPMAP => "FILTER_NEAREST_MIPMAP", Self::LINEAR_MIPMAP => "FILTER_LINEAR_MIPMAP", Self::NEAREST_MIPMAP_ANISOTROPIC => "FILTER_NEAREST_MIPMAP_ANISOTROPIC", Self::LINEAR_MIPMAP_ANISOTROPIC => "FILTER_LINEAR_MIPMAP_ANISOTROPIC", Self::MAX => "FILTER_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
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
pub struct TextureRepeat {
    ord: i32
}
impl TextureRepeat {
    #[doc(alias = "REPEAT_DEFAULT")]
    #[doc = "Godot enumerator name: `REPEAT_DEFAULT`"]
    pub const DEFAULT: TextureRepeat = TextureRepeat {
        ord: 0i32
    };
    #[doc(alias = "REPEAT_ENABLED")]
    #[doc = "Godot enumerator name: `REPEAT_ENABLED`"]
    pub const ENABLED: TextureRepeat = TextureRepeat {
        ord: 1i32
    };
    #[doc(alias = "REPEAT_DISABLED")]
    #[doc = "Godot enumerator name: `REPEAT_DISABLED`"]
    pub const DISABLED: TextureRepeat = TextureRepeat {
        ord: 2i32
    };
    #[doc(alias = "REPEAT_MAX")]
    #[doc = "Godot enumerator name: `REPEAT_MAX`"]
    pub const MAX: TextureRepeat = TextureRepeat {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TextureRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureRepeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureRepeat {
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
            Self::DEFAULT => "DEFAULT", Self::ENABLED => "ENABLED", Self::DISABLED => "DISABLED", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "REPEAT_DEFAULT", Self::ENABLED => "REPEAT_ENABLED", Self::DISABLED => "REPEAT_DISABLED", Self::MAX => "REPEAT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureRepeat {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for TextureRepeat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureRepeat {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureSource {
    ord: i32
}
impl TextureSource {
    #[doc(alias = "SOURCE_NONE")]
    #[doc = "Godot enumerator name: `SOURCE_NONE`"]
    pub const NONE: TextureSource = TextureSource {
        ord: 0i32
    };
    #[doc(alias = "SOURCE_SCREEN")]
    #[doc = "Godot enumerator name: `SOURCE_SCREEN`"]
    pub const SCREEN: TextureSource = TextureSource {
        ord: 1i32
    };
    #[doc(alias = "SOURCE_DEPTH")]
    #[doc = "Godot enumerator name: `SOURCE_DEPTH`"]
    pub const DEPTH: TextureSource = TextureSource {
        ord: 2i32
    };
    #[doc(alias = "SOURCE_NORMAL_ROUGHNESS")]
    #[doc = "Godot enumerator name: `SOURCE_NORMAL_ROUGHNESS`"]
    pub const NORMAL_ROUGHNESS: TextureSource = TextureSource {
        ord: 3i32
    };
    #[doc(alias = "SOURCE_MAX")]
    #[doc = "Godot enumerator name: `SOURCE_MAX`"]
    pub const MAX: TextureSource = TextureSource {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TextureSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureSource {
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
            Self::NONE => "NONE", Self::SCREEN => "SCREEN", Self::DEPTH => "DEPTH", Self::NORMAL_ROUGHNESS => "NORMAL_ROUGHNESS", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "SOURCE_NONE", Self::SCREEN => "SOURCE_SCREEN", Self::DEPTH => "SOURCE_DEPTH", Self::NORMAL_ROUGHNESS => "SOURCE_NORMAL_ROUGHNESS", Self::MAX => "SOURCE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureSource {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for TextureSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureSource {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}