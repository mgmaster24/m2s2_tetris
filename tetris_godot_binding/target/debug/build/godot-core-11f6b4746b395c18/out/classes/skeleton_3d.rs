#![doc = "Sidecar module for class [`Skeleton3D`][crate::classes::Skeleton3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Skeleton3D` enums](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Skeleton3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`skeleton_3d`][crate::classes::skeleton_3d]: sidecar module with related enum/flag types\n* [`ISkeleton3D`][crate::classes::ISkeleton3D]: virtual methods\n* [`Skeleton3DNotification`][crate::classes::notify::Skeleton3DNotification]: notification type\n\n\nSee also [Godot docs for `Skeleton3D`](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Skeleton3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Skeleton3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Skeleton3D`][crate::classes::Skeleton3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Skeleton3D` methods](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkeleton3D: crate::obj::GodotClass < Base = Skeleton3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Skeleton3DNotification) {
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
    #[doc = "Notification type for class [`Skeleton3D`][crate::classes::Skeleton3D]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum Skeleton3DNotification {
        UPDATE_SKELETON = 50i32, TRANSFORM_CHANGED = 2000i32, ENTER_WORLD = 41i32, EXIT_WORLD = 42i32, VISIBILITY_CHANGED = 43i32, LOCAL_TRANSFORM_CHANGED = 44i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for Skeleton3DNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                50i32 => Self::UPDATE_SKELETON, 2000i32 => Self::TRANSFORM_CHANGED, 41i32 => Self::ENTER_WORLD, 42i32 => Self::EXIT_WORLD, 43i32 => Self::VISIBILITY_CHANGED, 44i32 => Self::LOCAL_TRANSFORM_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < Skeleton3DNotification > for i32 {
        fn from(notification: Skeleton3DNotification) -> i32 {
            match notification {
                Skeleton3DNotification::UPDATE_SKELETON => 50i32, Skeleton3DNotification::TRANSFORM_CHANGED => 2000i32, Skeleton3DNotification::ENTER_WORLD => 41i32, Skeleton3DNotification::EXIT_WORLD => 42i32, Skeleton3DNotification::VISIBILITY_CHANGED => 43i32, Skeleton3DNotification::LOCAL_TRANSFORM_CHANGED => 44i32, Skeleton3DNotification::ENTER_TREE => 10i32, Skeleton3DNotification::EXIT_TREE => 11i32, Skeleton3DNotification::MOVED_IN_PARENT => 12i32, Skeleton3DNotification::READY => 13i32, Skeleton3DNotification::PAUSED => 14i32, Skeleton3DNotification::UNPAUSED => 15i32, Skeleton3DNotification::PHYSICS_PROCESS => 16i32, Skeleton3DNotification::PROCESS => 17i32, Skeleton3DNotification::PARENTED => 18i32, Skeleton3DNotification::UNPARENTED => 19i32, Skeleton3DNotification::SCENE_INSTANTIATED => 20i32, Skeleton3DNotification::DRAG_BEGIN => 21i32, Skeleton3DNotification::DRAG_END => 22i32, Skeleton3DNotification::PATH_RENAMED => 23i32, Skeleton3DNotification::CHILD_ORDER_CHANGED => 24i32, Skeleton3DNotification::INTERNAL_PROCESS => 25i32, Skeleton3DNotification::INTERNAL_PHYSICS_PROCESS => 26i32, Skeleton3DNotification::POST_ENTER_TREE => 27i32, Skeleton3DNotification::DISABLED => 28i32, Skeleton3DNotification::ENABLED => 29i32, Skeleton3DNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, Skeleton3DNotification::EDITOR_PRE_SAVE => 9001i32, Skeleton3DNotification::EDITOR_POST_SAVE => 9002i32, Skeleton3DNotification::WM_MOUSE_ENTER => 1002i32, Skeleton3DNotification::WM_MOUSE_EXIT => 1003i32, Skeleton3DNotification::WM_WINDOW_FOCUS_IN => 1004i32, Skeleton3DNotification::WM_WINDOW_FOCUS_OUT => 1005i32, Skeleton3DNotification::WM_CLOSE_REQUEST => 1006i32, Skeleton3DNotification::WM_GO_BACK_REQUEST => 1007i32, Skeleton3DNotification::WM_SIZE_CHANGED => 1008i32, Skeleton3DNotification::WM_DPI_CHANGE => 1009i32, Skeleton3DNotification::VP_MOUSE_ENTER => 1010i32, Skeleton3DNotification::VP_MOUSE_EXIT => 1011i32, Skeleton3DNotification::OS_MEMORY_WARNING => 2009i32, Skeleton3DNotification::TRANSLATION_CHANGED => 2010i32, Skeleton3DNotification::WM_ABOUT => 2011i32, Skeleton3DNotification::CRASH => 2012i32, Skeleton3DNotification::OS_IME_UPDATE => 2013i32, Skeleton3DNotification::APPLICATION_RESUMED => 2014i32, Skeleton3DNotification::APPLICATION_PAUSED => 2015i32, Skeleton3DNotification::APPLICATION_FOCUS_IN => 2016i32, Skeleton3DNotification::APPLICATION_FOCUS_OUT => 2017i32, Skeleton3DNotification::TEXT_SERVER_CHANGED => 2018i32, Skeleton3DNotification::POSTINITIALIZE => 0i32, Skeleton3DNotification::PREDELETE => 1i32, Skeleton3DNotification::EXTENSION_RELOADED => 2i32, Skeleton3DNotification::Unknown(int) => int,
            }
        }
    }
    impl Skeleton3D {
        pub fn add_bone(&mut self, name: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "add_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_bone(&self, name: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "find_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_name(&self, bone_idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_name(&mut self, bone_idx: i32, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (bone_idx, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_concatenated_bone_names(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_concatenated_bone_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_parent(&self, bone_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_parent(&mut self, bone_idx: i32, parent_idx: i32,) {
            type CallSig = ((), i32, i32);
            let args = (bone_idx, parent_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unparent_bone_and_rest(&mut self, bone_idx: i32,) {
            type CallSig = ((), i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "unparent_bone_and_rest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_children(&self, bone_idx: i32,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_children", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parentless_bones(&self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_parentless_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_rest(&self, bone_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_rest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_rest(&mut self, bone_idx: i32, rest: Transform3D,) {
            type CallSig = ((), i32, Transform3D);
            let args = (bone_idx, rest,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_rest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_global_rest(&self, bone_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_global_rest", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_skin_from_rest_transforms(&mut self,) -> Option < Gd < crate::classes::Skin > > {
            type CallSig = (Option < Gd < crate::classes::Skin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "create_skin_from_rest_transforms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_skin(&mut self, skin: impl AsObjectArg < crate::classes::Skin >,) -> Option < Gd < crate::classes::SkinReference > > {
            type CallSig = (Option < Gd < crate::classes::SkinReference > >, ObjectArg < crate::classes::Skin >);
            let args = (skin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "register_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn localize_rests(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "localize_rests", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_bones(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "clear_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_pose(&self, bone_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_pose(&mut self, bone_idx: i32, pose: Transform3D,) {
            type CallSig = ((), i32, Transform3D);
            let args = (bone_idx, pose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_pose_position(&mut self, bone_idx: i32, position: Vector3,) {
            type CallSig = ((), i32, Vector3);
            let args = (bone_idx, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_pose_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_pose_rotation(&mut self, bone_idx: i32, rotation: Quaternion,) {
            type CallSig = ((), i32, Quaternion);
            let args = (bone_idx, rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_pose_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_pose_scale(&mut self, bone_idx: i32, scale: Vector3,) {
            type CallSig = ((), i32, Vector3);
            let args = (bone_idx, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_pose_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_pose_position(&self, bone_idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_pose_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_pose_rotation(&self, bone_idx: i32,) -> Quaternion {
            type CallSig = (Quaternion, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_pose_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_pose_scale(&self, bone_idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_pose_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset_bone_pose(&mut self, bone_idx: i32,) {
            type CallSig = ((), i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "reset_bone_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset_bone_poses(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "reset_bone_poses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bone_enabled(&self, bone_idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "is_bone_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_bone_enabled_full(&mut self, bone_idx: i32, enabled: bool,) {
            type CallSig = ((), i32, bool);
            let args = (bone_idx, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_bone_enabled_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_bone_enabled(&mut self, bone_idx: i32,) {
            self.set_bone_enabled_ex(bone_idx,) . done()
        }
        #[inline]
        pub fn set_bone_enabled_ex < 'a > (&'a mut self, bone_idx: i32,) -> ExSetBoneEnabled < 'a > {
            ExSetBoneEnabled::new(self, bone_idx,)
        }
        pub fn get_bone_global_pose(&self, bone_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_global_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_global_pose(&mut self, bone_idx: i32, pose: Transform3D,) {
            type CallSig = ((), i32, Transform3D);
            let args = (bone_idx, pose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_global_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_all_bone_transforms(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "force_update_all_bone_transforms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_bone_child_transform(&mut self, bone_idx: i32,) {
            type CallSig = ((), i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "force_update_bone_child_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_scale(&mut self, motion_scale: f32,) {
            type CallSig = ((), f32);
            let args = (motion_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_motion_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_motion_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_show_rest_only(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_show_rest_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_show_rest_only(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "is_show_rest_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modifier_callback_mode_process(&mut self, mode: crate::classes::skeleton_3d::ModifierCallbackModeProcess,) {
            type CallSig = ((), crate::classes::skeleton_3d::ModifierCallbackModeProcess);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_modifier_callback_mode_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modifier_callback_mode_process(&self,) -> crate::classes::skeleton_3d::ModifierCallbackModeProcess {
            type CallSig = (crate::classes::skeleton_3d::ModifierCallbackModeProcess,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_modifier_callback_mode_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_bones_global_pose_override(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "clear_bones_global_pose_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_bone_global_pose_override_full(&mut self, bone_idx: i32, pose: Transform3D, amount: f32, persistent: bool,) {
            type CallSig = ((), i32, Transform3D, f32, bool);
            let args = (bone_idx, pose, amount, persistent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_bone_global_pose_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_bone_global_pose_override_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_bone_global_pose_override(&mut self, bone_idx: i32, pose: Transform3D, amount: f32,) {
            self.set_bone_global_pose_override_ex(bone_idx, pose, amount,) . done()
        }
        #[inline]
        pub fn set_bone_global_pose_override_ex < 'a > (&'a mut self, bone_idx: i32, pose: Transform3D, amount: f32,) -> ExSetBoneGlobalPoseOverride < 'a > {
            ExSetBoneGlobalPoseOverride::new(self, bone_idx, pose, amount,)
        }
        pub fn get_bone_global_pose_override(&self, bone_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_global_pose_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_global_pose_no_override(&self, bone_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_bone_global_pose_no_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_animate_physical_bones(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "set_animate_physical_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animate_physical_bones(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "get_animate_physical_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn physical_bones_stop_simulation(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "physical_bones_stop_simulation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn physical_bones_start_simulation_full(&mut self, bones: RefArg < Array < StringName > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < StringName > >);
            let args = (bones,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "physical_bones_start_simulation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::physical_bones_start_simulation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn physical_bones_start_simulation(&mut self,) {
            self.physical_bones_start_simulation_ex() . done()
        }
        #[inline]
        pub fn physical_bones_start_simulation_ex < 'a > (&'a mut self,) -> ExPhysicalBonesStartSimulation < 'a > {
            ExPhysicalBonesStartSimulation::new(self,)
        }
        pub fn physical_bones_add_collision_exception(&mut self, exception: Rid,) {
            type CallSig = ((), Rid);
            let args = (exception,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "physical_bones_add_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn physical_bones_remove_collision_exception(&mut self, exception: Rid,) {
            type CallSig = ((), Rid);
            let args = (exception,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Skeleton3D", "physical_bones_remove_collision_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: Skeleton3DNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Skeleton3DNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_UPDATE_SKELETON: i32 = 50i32;
        
    }
    impl crate::obj::GodotClass for Skeleton3D {
        type Base = crate::classes::Node3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Skeleton3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Skeleton3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Skeleton3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Skeleton3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Skeleton3D {
        
    }
    impl crate::obj::cap::GodotDefault for Skeleton3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Skeleton3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Skeleton3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Skeleton3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Skeleton3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Skeleton3D > for $Class {
                
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
#[doc = "Default-param extender for [`Skeleton3D::set_bone_enabled_ex`][super::Skeleton3D::set_bone_enabled_ex]."]
#[must_use]
pub struct ExSetBoneEnabled < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Skeleton3D, bone_idx: i32, enabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetBoneEnabled < 'a > {
    fn new(surround_object: &'a mut re_export::Skeleton3D, bone_idx: i32,) -> Self {
        let enabled = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bone_idx: bone_idx, enabled: enabled,
        }
    }
    #[inline]
    pub fn enabled(self, enabled: bool) -> Self {
        Self {
            enabled: enabled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, bone_idx, enabled,
        }
        = self;
        re_export::Skeleton3D::set_bone_enabled_full(surround_object, bone_idx, enabled,)
    }
}
#[doc = "Default-param extender for [`Skeleton3D::set_bone_global_pose_override_ex`][super::Skeleton3D::set_bone_global_pose_override_ex]."]
#[must_use]
pub struct ExSetBoneGlobalPoseOverride < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Skeleton3D, bone_idx: i32, pose: Transform3D, amount: f32, persistent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetBoneGlobalPoseOverride < 'a > {
    fn new(surround_object: &'a mut re_export::Skeleton3D, bone_idx: i32, pose: Transform3D, amount: f32,) -> Self {
        let persistent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bone_idx: bone_idx, pose: pose, amount: amount, persistent: persistent,
        }
    }
    #[inline]
    pub fn persistent(self, persistent: bool) -> Self {
        Self {
            persistent: persistent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, bone_idx, pose, amount, persistent,
        }
        = self;
        re_export::Skeleton3D::set_bone_global_pose_override_full(surround_object, bone_idx, pose, amount, persistent,)
    }
}
#[doc = "Default-param extender for [`Skeleton3D::physical_bones_start_simulation_ex`][super::Skeleton3D::physical_bones_start_simulation_ex]."]
#[must_use]
pub struct ExPhysicalBonesStartSimulation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Skeleton3D, bones: CowArg < 'a, Array < StringName > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPhysicalBonesStartSimulation < 'a > {
    fn new(surround_object: &'a mut re_export::Skeleton3D,) -> Self {
        let bones = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bones: CowArg::Owned(bones),
        }
    }
    #[inline]
    pub fn bones(self, bones: &'a Array < StringName >) -> Self {
        Self {
            bones: CowArg::Borrowed(bones), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, bones,
        }
        = self;
        re_export::Skeleton3D::physical_bones_start_simulation_full(surround_object, bones.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModifierCallbackModeProcess {
    ord: i32
}
impl ModifierCallbackModeProcess {
    #[doc(alias = "MODIFIER_CALLBACK_MODE_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `MODIFIER_CALLBACK_MODE_PROCESS_PHYSICS`"]
    pub const PHYSICS: ModifierCallbackModeProcess = ModifierCallbackModeProcess {
        ord: 0i32
    };
    #[doc(alias = "MODIFIER_CALLBACK_MODE_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `MODIFIER_CALLBACK_MODE_PROCESS_IDLE`"]
    pub const IDLE: ModifierCallbackModeProcess = ModifierCallbackModeProcess {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ModifierCallbackModeProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ModifierCallbackModeProcess") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ModifierCallbackModeProcess {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PHYSICS => "MODIFIER_CALLBACK_MODE_PROCESS_PHYSICS", Self::IDLE => "MODIFIER_CALLBACK_MODE_PROCESS_IDLE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ModifierCallbackModeProcess {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ModifierCallbackModeProcess {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ModifierCallbackModeProcess {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}