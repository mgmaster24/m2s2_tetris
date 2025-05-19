#![doc = "Sidecar module for class [`GltfSkeleton`][crate::classes::GltfSkeleton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFSkeleton` enums](https://docs.godotengine.org/en/stable/classes/class_gltfskeleton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFSkeleton.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IGltfSkeleton`][crate::classes::IGltfSkeleton]: virtual methods\n\n\nSee also [Godot docs for `GLTFSkeleton`](https://docs.godotengine.org/en/stable/classes/class_gltfskeleton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfSkeleton::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfSkeleton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfSkeleton`][crate::classes::GltfSkeleton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFSkeleton` methods](https://docs.godotengine.org/en/stable/classes/class_gltfskeleton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfSkeleton: crate::obj::GodotClass < Base = GltfSkeleton > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfSkeleton {
        pub fn get_joints(&mut self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "get_joints", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_joints(&mut self, joints: &PackedInt32Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedInt32Array >);
            let args = (RefArg::new(joints),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "set_joints", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_roots(&mut self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "get_roots", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_roots(&mut self, roots: &PackedInt32Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedInt32Array >);
            let args = (RefArg::new(roots),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "set_roots", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_godot_skeleton(&mut self,) -> Option < Gd < crate::classes::Skeleton3D > > {
            type CallSig = (Option < Gd < crate::classes::Skeleton3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "get_godot_skeleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_names(&mut self,) -> Array < GString > {
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "get_unique_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unique_names(&mut self, unique_names: &Array < GString >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < GString > >);
            let args = (RefArg::new(unique_names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "set_unique_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_godot_bone_node(&mut self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "get_godot_bone_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_godot_bone_node(&mut self, godot_bone_node: &Dictionary,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(godot_bone_node),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "set_godot_bone_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_attachment_count(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "get_bone_attachment_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_attachment(&mut self, idx: i32,) -> Option < Gd < crate::classes::BoneAttachment3D > > {
            type CallSig = (Option < Gd < crate::classes::BoneAttachment3D > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfSkeleton", "get_bone_attachment", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GltfSkeleton {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GLTFSkeleton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfSkeleton {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfSkeleton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfSkeleton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfSkeleton {
        
    }
    impl crate::obj::cap::GodotDefault for GltfSkeleton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfSkeleton {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfSkeleton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfSkeleton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GltfSkeleton {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfSkeleton > for $Class {
                
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