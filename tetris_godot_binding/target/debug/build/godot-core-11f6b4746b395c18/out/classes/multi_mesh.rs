#![doc = "Sidecar module for class [`MultiMesh`][crate::classes::MultiMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiMesh` enums](https://docs.godotengine.org/en/stable/classes/class_multimesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiMesh.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`multi_mesh`][crate::classes::multi_mesh]: sidecar module with related enum/flag types\n* [`IMultiMesh`][crate::classes::IMultiMesh]: virtual methods\n\n\nSee also [Godot docs for `MultiMesh`](https://docs.godotengine.org/en/stable/classes/class_multimesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MultiMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MultiMesh`][crate::classes::MultiMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MultiMesh` methods](https://docs.godotengine.org/en/stable/classes/class_multimesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiMesh: crate::obj::GodotClass < Base = MultiMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiMesh {
        pub fn set_mesh(&mut self, mesh: impl AsObjectArg < crate::classes::Mesh >,) {
            type CallSig = ((), ObjectArg < crate::classes::Mesh >);
            let args = (mesh.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::classes::Mesh > > {
            type CallSig = (Option < Gd < crate::classes::Mesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_colors(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_use_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_colors(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "is_using_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_custom_data(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_use_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_custom_data(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "is_using_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform_format(&mut self, format: crate::classes::multi_mesh::TransformFormat,) {
            type CallSig = ((), crate::classes::multi_mesh::TransformFormat);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_transform_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform_format(&self,) -> crate::classes::multi_mesh::TransformFormat {
            type CallSig = (crate::classes::multi_mesh::TransformFormat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_transform_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible_instance_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_visible_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_instance_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_visible_instance_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_transform(&mut self, instance: i32, transform: Transform3D,) {
            type CallSig = ((), i32, Transform3D);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_transform_2d(&mut self, instance: i32, transform: Transform2D,) {
            type CallSig = ((), i32, Transform2D);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_transform(&self, instance: i32,) -> Transform3D {
            type CallSig = (Transform3D, i32);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_transform_2d(&self, instance: i32,) -> Transform2D {
            type CallSig = (Transform2D, i32);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_transform_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_color(&mut self, instance: i32, color: Color,) {
            type CallSig = ((), i32, Color);
            let args = (instance, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_color(&self, instance: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_custom_data(&mut self, instance: i32, custom_data: Color,) {
            type CallSig = ((), i32, Color);
            let args = (instance, custom_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_custom_data(&self, instance: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_custom_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallSig = ((), Aabb);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_aabb(&self,) -> Aabb {
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffer(&self,) -> PackedFloat32Array {
            type CallSig = (PackedFloat32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "get_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_buffer(&mut self, buffer: &PackedFloat32Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedFloat32Array >);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiMesh", "set_buffer", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MultiMesh {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"MultiMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for MultiMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MultiMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiMesh {
        
    }
    impl crate::obj::cap::GodotDefault for MultiMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MultiMesh {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MultiMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_MultiMesh {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiMesh > for $Class {
                
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
pub struct TransformFormat {
    ord: i32
}
impl TransformFormat {
    pub const TRANSFORM_2D: TransformFormat = TransformFormat {
        ord: 0i32
    };
    pub const TRANSFORM_3D: TransformFormat = TransformFormat {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TransformFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransformFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransformFormat {
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
            Self::TRANSFORM_2D => "TRANSFORM_2D", Self::TRANSFORM_3D => "TRANSFORM_3D", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        self.as_str()
    }
}
impl crate::meta::GodotConvert for TransformFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransformFormat {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransformFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}