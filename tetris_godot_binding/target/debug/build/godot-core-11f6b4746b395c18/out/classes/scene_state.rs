#![doc = "Sidecar module for class [`SceneState`][crate::classes::SceneState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneState` enums](https://docs.godotengine.org/en/stable/classes/class_scenestate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneState.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`scene_state`][crate::classes::scene_state]: sidecar module with related enum/flag types\n* [`ISceneState`][crate::classes::ISceneState]: virtual methods\n\n\nSee also [Godot docs for `SceneState`](https://docs.godotengine.org/en/stable/classes/class_scenestate.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<SceneState>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SceneState`][crate::classes::SceneState].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SceneState` methods](https://docs.godotengine.org/en/stable/classes/class_scenestate.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISceneState: crate::obj::GodotClass < Base = SceneState > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SceneState {
        pub fn get_node_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_type(&self, idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_name(&self, idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_node_path_full(&self, idx: i32, for_parent: bool,) -> NodePath {
            type CallSig = (NodePath, i32, bool);
            let args = (idx, for_parent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_node_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_node_path(&self, idx: i32,) -> NodePath {
            self.get_node_path_ex(idx,) . done()
        }
        #[inline]
        pub fn get_node_path_ex < 'a > (&'a self, idx: i32,) -> ExGetNodePath < 'a > {
            ExGetNodePath::new(self, idx,)
        }
        pub fn get_node_owner_path(&self, idx: i32,) -> NodePath {
            type CallSig = (NodePath, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_owner_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_node_instance_placeholder(&self, idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "is_node_instance_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_instance_placeholder(&self, idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_instance_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_instance(&self, idx: i32,) -> Option < Gd < crate::classes::PackedScene > > {
            type CallSig = (Option < Gd < crate::classes::PackedScene > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_groups(&self, idx: i32,) -> PackedStringArray {
            type CallSig = (PackedStringArray, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_groups", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_index(&self, idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_property_count(&self, idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_property_name(&self, idx: i32, prop_idx: i32,) -> StringName {
            type CallSig = (StringName, i32, i32);
            let args = (idx, prop_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_property_value(&self, idx: i32, prop_idx: i32,) -> Variant {
            type CallSig = (Variant, i32, i32);
            let args = (idx, prop_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_source(&self, idx: i32,) -> NodePath {
            type CallSig = (NodePath, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_signal(&self, idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_target(&self, idx: i32,) -> NodePath {
            type CallSig = (NodePath, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_target", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_method(&self, idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_flags(&self, idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_binds(&self, idx: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_binds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_unbinds(&self, idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_unbinds", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SceneState {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SceneState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SceneState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SceneState {
        
    }
    impl std::ops::Deref for SceneState {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SceneState`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SceneState {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SceneState > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SceneState::get_node_path_ex`][super::SceneState::get_node_path_ex]."]
#[must_use]
pub struct ExGetNodePath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::SceneState, idx: i32, for_parent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetNodePath < 'a > {
    fn new(surround_object: &'a re_export::SceneState, idx: i32,) -> Self {
        let for_parent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, for_parent: for_parent,
        }
    }
    #[inline]
    pub fn for_parent(self, for_parent: bool) -> Self {
        Self {
            for_parent: for_parent, .. self
        }
    }
    #[inline]
    pub fn done(self) -> NodePath {
        let Self {
            _phantom, surround_object, idx, for_parent,
        }
        = self;
        re_export::SceneState::get_node_path_full(surround_object, idx, for_parent,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GenEditState {
    ord: i32
}
impl GenEditState {
    #[doc(alias = "GEN_EDIT_STATE_DISABLED")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_DISABLED`"]
    pub const DISABLED: GenEditState = GenEditState {
        ord: 0i32
    };
    #[doc(alias = "GEN_EDIT_STATE_INSTANCE")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_INSTANCE`"]
    pub const INSTANCE: GenEditState = GenEditState {
        ord: 1i32
    };
    #[doc(alias = "GEN_EDIT_STATE_MAIN")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_MAIN`"]
    pub const MAIN: GenEditState = GenEditState {
        ord: 2i32
    };
    #[doc(alias = "GEN_EDIT_STATE_MAIN_INHERITED")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_MAIN_INHERITED`"]
    pub const MAIN_INHERITED: GenEditState = GenEditState {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for GenEditState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GenEditState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GenEditState {
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
            Self::DISABLED => "DISABLED", Self::INSTANCE => "INSTANCE", Self::MAIN => "MAIN", Self::MAIN_INHERITED => "MAIN_INHERITED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "GEN_EDIT_STATE_DISABLED", Self::INSTANCE => "GEN_EDIT_STATE_INSTANCE", Self::MAIN => "GEN_EDIT_STATE_MAIN", Self::MAIN_INHERITED => "GEN_EDIT_STATE_MAIN_INHERITED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for GenEditState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GenEditState {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GenEditState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}