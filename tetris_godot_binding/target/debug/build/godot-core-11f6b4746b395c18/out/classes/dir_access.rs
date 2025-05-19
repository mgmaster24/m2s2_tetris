#![doc = "Sidecar module for class [`DirAccess`][crate::classes::DirAccess].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DirAccess` enums](https://docs.godotengine.org/en/stable/classes/class_diraccess.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DirAccess.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`dir_access`][crate::classes::dir_access]: sidecar module with related enum/flag types\n* [`IDirAccess`][crate::classes::IDirAccess]: virtual methods\n\n\nSee also [Godot docs for `DirAccess`](https://docs.godotengine.org/en/stable/classes/class_diraccess.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<DirAccess>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DirAccess {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`DirAccess`][crate::classes::DirAccess].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `DirAccess` methods](https://docs.godotengine.org/en/stable/classes/class_diraccess.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IDirAccess: crate::obj::GodotClass < Base = DirAccess > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl DirAccess {
        pub fn open(path: impl AsArg < GString >,) -> Option < Gd < crate::classes::DirAccess > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::DirAccess > >, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "open", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_open_error() -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_open_error", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn list_dir_begin(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "list_dir_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn current_is_dir(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "current_is_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn list_dir_end(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "list_dir_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_files(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_files", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_files_at(path: impl AsArg < GString >,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_files_at", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_directories(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_directories", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_directories_at(path: impl AsArg < GString >,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_directories_at", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_drive_count() -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_drive_count", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_drive_name(idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_drive_name", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_current_drive(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_current_drive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn change_dir(&mut self, to_dir: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (to_dir.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "change_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_current_dir_full(&self, include_drive: bool,) -> GString {
            type CallSig = (GString, bool);
            let args = (include_drive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_current_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_current_dir_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_current_dir(&self,) -> GString {
            self.get_current_dir_ex() . done()
        }
        #[inline]
        pub fn get_current_dir_ex < 'a > (&'a self,) -> ExGetCurrentDir < 'a > {
            ExGetCurrentDir::new(self,)
        }
        pub fn make_dir(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_dir_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn make_dir_recursive(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_recursive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_dir_recursive_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_recursive_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn file_exists(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "file_exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dir_exists(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "dir_exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dir_exists_absolute(path: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "dir_exists_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_space_left(&mut self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_space_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn copy_full(&mut self, from: CowArg < GString >, to: CowArg < GString >, chmod_flags: i32,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32);
            let args = (from, to, chmod_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "copy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::copy_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn copy(&mut self, from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            self.copy_ex(from, to,) . done()
        }
        #[inline]
        pub fn copy_ex < 'a > (&'a mut self, from: impl AsArg < GString > + 'a, to: impl AsArg < GString > + 'a,) -> ExCopy < 'a > {
            ExCopy::new(self, from, to,)
        }
        pub(crate) fn copy_absolute_full(from: CowArg < GString >, to: CowArg < GString >, chmod_flags: i32,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32);
            let args = (from, to, chmod_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "copy_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::copy_absolute_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn copy_absolute(from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            Self::copy_absolute_ex(from, to,) . done()
        }
        #[inline]
        pub fn copy_absolute_ex < 'a > (from: impl AsArg < GString > + 'a, to: impl AsArg < GString > + 'a,) -> ExCopyAbsolute < 'a > {
            ExCopyAbsolute::new(from, to,)
        }
        pub fn rename(&mut self, from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "rename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_absolute(from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "rename_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn remove(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "remove", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "remove_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn is_link(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "is_link", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn read_link(&mut self, path: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "read_link", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_link(&mut self, source: impl AsArg < GString >, target: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (source.into_arg(), target.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "create_link", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_include_navigational(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "set_include_navigational", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_include_navigational(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_include_navigational", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_include_hidden(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "set_include_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_include_hidden(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "get_include_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_case_sensitive(&self, path: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DirAccess", "is_case_sensitive", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for DirAccess {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"DirAccess"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for DirAccess {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for DirAccess {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DirAccess {
        
    }
    impl std::ops::Deref for DirAccess {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DirAccess {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`DirAccess`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_DirAccess {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::DirAccess > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`DirAccess::get_current_dir_ex`][super::DirAccess::get_current_dir_ex]."]
#[must_use]
pub struct ExGetCurrentDir < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DirAccess, include_drive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCurrentDir < 'a > {
    fn new(surround_object: &'a re_export::DirAccess,) -> Self {
        let include_drive = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_drive: include_drive,
        }
    }
    #[inline]
    pub fn include_drive(self, include_drive: bool) -> Self {
        Self {
            include_drive: include_drive, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, include_drive,
        }
        = self;
        re_export::DirAccess::get_current_dir_full(surround_object, include_drive,)
    }
}
#[doc = "Default-param extender for [`DirAccess::copy_ex`][super::DirAccess::copy_ex]."]
#[must_use]
pub struct ExCopy < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DirAccess, from: CowArg < 'a, GString >, to: CowArg < 'a, GString >, chmod_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCopy < 'a > {
    fn new(surround_object: &'a mut re_export::DirAccess, from: impl AsArg < GString > + 'a, to: impl AsArg < GString > + 'a,) -> Self {
        let chmod_flags = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from.into_arg(), to: to.into_arg(), chmod_flags: chmod_flags,
        }
    }
    #[inline]
    pub fn chmod_flags(self, chmod_flags: i32) -> Self {
        Self {
            chmod_flags: chmod_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, from, to, chmod_flags,
        }
        = self;
        re_export::DirAccess::copy_full(surround_object, from, to, chmod_flags,)
    }
}
#[doc = "Default-param extender for [`DirAccess::copy_absolute_ex`][super::DirAccess::copy_absolute_ex]."]
#[must_use]
pub struct ExCopyAbsolute < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, from: CowArg < 'a, GString >, to: CowArg < 'a, GString >, chmod_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCopyAbsolute < 'a > {
    fn new(from: impl AsArg < GString > + 'a, to: impl AsArg < GString > + 'a,) -> Self {
        let chmod_flags = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, from: from.into_arg(), to: to.into_arg(), chmod_flags: chmod_flags,
        }
    }
    #[inline]
    pub fn chmod_flags(self, chmod_flags: i32) -> Self {
        Self {
            chmod_flags: chmod_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, from, to, chmod_flags,
        }
        = self;
        re_export::DirAccess::copy_absolute_full(from, to, chmod_flags,)
    }
}