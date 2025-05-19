#![doc = "Sidecar module for class [`FileAccess`][crate::classes::FileAccess].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FileAccess` enums](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FileAccess.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`file_access`][crate::classes::file_access]: sidecar module with related enum/flag types\n* [`IFileAccess`][crate::classes::IFileAccess]: virtual methods\n\n\nSee also [Godot docs for `FileAccess`](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html).\n\n# Specific notes for this class\n\nThe gdext library provides a higher-level abstraction, which should be preferred: [`GFile`][crate::tools::GFile]."]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<FileAccess>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FileAccess {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`FileAccess`][crate::classes::FileAccess].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FileAccess` methods](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFileAccess: crate::obj::GodotClass < Base = FileAccess > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FileAccess {
        pub fn open(path: impl AsArg < GString >, flags: crate::classes::file_access::ModeFlags,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::FileAccess > >, CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags);
            let args = (path.into_arg(), flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "open", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn open_encrypted(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, key: &PackedByteArray,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::FileAccess > >, CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, RefArg < 'a1, PackedByteArray >);
            let args = (path.into_arg(), mode_flags, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "open_encrypted", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn open_encrypted_with_pass(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, pass: impl AsArg < GString >,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::FileAccess > >, CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, CowArg < 'a1, GString >);
            let args = (path.into_arg(), mode_flags, pass.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "open_encrypted_with_pass", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn open_compressed_full(path: CowArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, compression_mode: crate::classes::file_access::CompressionMode,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::FileAccess > >, CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, crate::classes::file_access::CompressionMode);
            let args = (path, mode_flags, compression_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "open_compressed", std::ptr::null_mut(), None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::open_compressed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn open_compressed(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags,) -> Option < Gd < crate::classes::FileAccess > > {
            Self::open_compressed_ex(path, mode_flags,) . done()
        }
        #[inline]
        pub fn open_compressed_ex < 'a > (path: impl AsArg < GString > + 'a, mode_flags: crate::classes::file_access::ModeFlags,) -> ExOpenCompressed < 'a > {
            ExOpenCompressed::new(path, mode_flags,)
        }
        pub fn get_open_error() -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_open_error", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_file_as_bytes(path: impl AsArg < GString >,) -> PackedByteArray {
            type CallSig < 'a0, > = (PackedByteArray, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_file_as_bytes", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_file_as_string(path: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_file_as_string", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn resize(&mut self, length: i64,) -> crate::global::Error {
            type CallSig = (crate::global::Error, i64);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "resize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn flush(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "flush", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_absolute(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_path_absolute", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_open(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "is_open", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn seek(&mut self, position: u64,) {
            type CallSig = ((), u64);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "seek", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn seek_end_full(&mut self, position: i64,) {
            type CallSig = ((), i64);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "seek_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::seek_end_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn seek_end(&mut self,) {
            self.seek_end_ex() . done()
        }
        #[inline]
        pub fn seek_end_ex < 'a > (&'a mut self,) -> ExSeekEnd < 'a > {
            ExSeekEnd::new(self,)
        }
        pub fn get_position(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_length(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn eof_reached(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "eof_reached", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_8(&self,) -> u8 {
            type CallSig = (u8,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_16(&self,) -> u16 {
            type CallSig = (u16,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_32(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_64(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_float(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_double(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_real(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_real", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffer(&self, length: i64,) -> PackedByteArray {
            type CallSig = (PackedByteArray, i64);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_csv_line_full(&self, delim: CowArg < GString >,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >);
            let args = (delim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_csv_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_csv_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_csv_line(&self,) -> PackedStringArray {
            self.get_csv_line_ex() . done()
        }
        #[inline]
        pub fn get_csv_line_ex < 'a > (&'a self,) -> ExGetCsvLine < 'a > {
            ExGetCsvLine::new(self,)
        }
        pub(crate) fn get_as_text_full(&self, skip_cr: bool,) -> GString {
            type CallSig = (GString, bool);
            let args = (skip_cr,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_as_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_as_text_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_as_text(&self,) -> GString {
            self.get_as_text_ex() . done()
        }
        #[inline]
        pub fn get_as_text_ex < 'a > (&'a self,) -> ExGetAsText < 'a > {
            ExGetAsText::new(self,)
        }
        pub fn get_md5(path: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_md5", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_sha256(path: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_sha256", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn is_big_endian(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "is_big_endian", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_big_endian(&mut self, big_endian: bool,) {
            type CallSig = ((), bool);
            let args = (big_endian,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "set_big_endian", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_error(&self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_error", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_var_full(&self, allow_objects: bool,) -> Variant {
            type CallSig = (Variant, bool);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_var(&self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[inline]
        pub fn get_var_ex < 'a > (&'a self,) -> ExGetVar < 'a > {
            ExGetVar::new(self,)
        }
        pub fn store_8(&mut self, value: u8,) {
            type CallSig = ((), u8);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_16(&mut self, value: u16,) {
            type CallSig = ((), u16);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_32(&mut self, value: u32,) {
            type CallSig = ((), u32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_64(&mut self, value: u64,) {
            type CallSig = ((), u64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_float(&mut self, value: f32,) {
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_double(&mut self, value: f64,) {
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_real(&mut self, value: f32,) {
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_real", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_buffer(&mut self, buffer: &PackedByteArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedByteArray >);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_line(&mut self, line: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (line.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn store_csv_line_full(&mut self, values: RefArg < PackedStringArray >, delim: CowArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), RefArg < 'a0, PackedStringArray >, CowArg < 'a1, GString >);
            let args = (values, delim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_csv_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::store_csv_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn store_csv_line(&mut self, values: &PackedStringArray,) {
            self.store_csv_line_ex(values,) . done()
        }
        #[inline]
        pub fn store_csv_line_ex < 'a > (&'a mut self, values: &'a PackedStringArray,) -> ExStoreCsvLine < 'a > {
            ExStoreCsvLine::new(self, values,)
        }
        pub fn store_string(&mut self, string: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn store_var_full(&mut self, value: RefArg < Variant >, full_objects: bool,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >, bool);
            let args = (value, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::store_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn store_var(&mut self, value: &Variant,) {
            self.store_var_ex(value,) . done()
        }
        #[inline]
        pub fn store_var_ex < 'a > (&'a mut self, value: &'a Variant,) -> ExStoreVar < 'a > {
            ExStoreVar::new(self, value,)
        }
        pub fn store_pascal_string(&mut self, string: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "store_pascal_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pascal_string(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_pascal_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn file_exists(path: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "file_exists", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_modified_time(file: impl AsArg < GString >,) -> u64 {
            type CallSig < 'a0, > = (u64, CowArg < 'a0, GString >);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_modified_time", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_unix_permissions(file: impl AsArg < GString >,) -> crate::classes::file_access::UnixPermissionFlags {
            type CallSig < 'a0, > = (crate::classes::file_access::UnixPermissionFlags, CowArg < 'a0, GString >);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_unix_permissions", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn set_unix_permissions(file: impl AsArg < GString >, permissions: crate::classes::file_access::UnixPermissionFlags,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >, crate::classes::file_access::UnixPermissionFlags);
            let args = (file.into_arg(), permissions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "set_unix_permissions", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_hidden_attribute(file: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_hidden_attribute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn set_hidden_attribute(file: impl AsArg < GString >, hidden: bool,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >, bool);
            let args = (file.into_arg(), hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "set_hidden_attribute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn set_read_only_attribute(file: impl AsArg < GString >, ro: bool,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >, bool);
            let args = (file.into_arg(), ro,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "set_read_only_attribute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_read_only_attribute(file: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FileAccess", "get_read_only_attribute", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for FileAccess {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"FileAccess"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FileAccess {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FileAccess {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FileAccess {
        
    }
    impl std::ops::Deref for FileAccess {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FileAccess {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FileAccess`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_FileAccess {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FileAccess > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`FileAccess::open_compressed_ex`][super::FileAccess::open_compressed_ex]."]
#[must_use]
pub struct ExOpenCompressed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, path: CowArg < 'a, GString >, mode_flags: crate::classes::file_access::ModeFlags, compression_mode: crate::classes::file_access::CompressionMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOpenCompressed < 'a > {
    fn new(path: impl AsArg < GString > + 'a, mode_flags: crate::classes::file_access::ModeFlags,) -> Self {
        let compression_mode = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, path: path.into_arg(), mode_flags: mode_flags, compression_mode: compression_mode,
        }
    }
    #[inline]
    pub fn compression_mode(self, compression_mode: crate::classes::file_access::CompressionMode) -> Self {
        Self {
            compression_mode: compression_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::FileAccess > > {
        let Self {
            _phantom, path, mode_flags, compression_mode,
        }
        = self;
        re_export::FileAccess::open_compressed_full(path, mode_flags, compression_mode,)
    }
}
#[doc = "Default-param extender for [`FileAccess::seek_end_ex`][super::FileAccess::seek_end_ex]."]
#[must_use]
pub struct ExSeekEnd < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::FileAccess, position: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSeekEnd < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess,) -> Self {
        let position = 0i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: i64) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position,
        }
        = self;
        re_export::FileAccess::seek_end_full(surround_object, position,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_csv_line_ex`][super::FileAccess::get_csv_line_ex]."]
#[must_use]
pub struct ExGetCsvLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::FileAccess, delim: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCsvLine < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        let delim = GString::from(",");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, delim: CowArg::Owned(delim),
        }
    }
    #[inline]
    pub fn delim(self, delim: impl AsArg < GString > + 'a) -> Self {
        Self {
            delim: delim.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, delim,
        }
        = self;
        re_export::FileAccess::get_csv_line_full(surround_object, delim,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_as_text_ex`][super::FileAccess::get_as_text_ex]."]
#[must_use]
pub struct ExGetAsText < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::FileAccess, skip_cr: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetAsText < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        let skip_cr = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, skip_cr: skip_cr,
        }
    }
    #[inline]
    pub fn skip_cr(self, skip_cr: bool) -> Self {
        Self {
            skip_cr: skip_cr, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, skip_cr,
        }
        = self;
        re_export::FileAccess::get_as_text_full(surround_object, skip_cr,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_var_ex`][super::FileAccess::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::FileAccess, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVar < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        let allow_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, allow_objects: allow_objects,
        }
    }
    #[inline]
    pub fn allow_objects(self, allow_objects: bool) -> Self {
        Self {
            allow_objects: allow_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, allow_objects,
        }
        = self;
        re_export::FileAccess::get_var_full(surround_object, allow_objects,)
    }
}
#[doc = "Default-param extender for [`FileAccess::store_csv_line_ex`][super::FileAccess::store_csv_line_ex]."]
#[must_use]
pub struct ExStoreCsvLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::FileAccess, values: CowArg < 'a, PackedStringArray >, delim: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStoreCsvLine < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess, values: &'a PackedStringArray,) -> Self {
        let delim = GString::from(",");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, values: CowArg::Borrowed(values), delim: CowArg::Owned(delim),
        }
    }
    #[inline]
    pub fn delim(self, delim: impl AsArg < GString > + 'a) -> Self {
        Self {
            delim: delim.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, values, delim,
        }
        = self;
        re_export::FileAccess::store_csv_line_full(surround_object, values.cow_as_arg(), delim,)
    }
}
#[doc = "Default-param extender for [`FileAccess::store_var_ex`][super::FileAccess::store_var_ex]."]
#[must_use]
pub struct ExStoreVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::FileAccess, value: CowArg < 'a, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStoreVar < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess, value: &'a Variant,) -> Self {
        let full_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, value: CowArg::Borrowed(value), full_objects: full_objects,
        }
    }
    #[inline]
    pub fn full_objects(self, full_objects: bool) -> Self {
        Self {
            full_objects: full_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, value, full_objects,
        }
        = self;
        re_export::FileAccess::store_var_full(surround_object, value.cow_as_arg(), full_objects,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModeFlags {
    ord: i32
}
impl ModeFlags {
    pub const READ: ModeFlags = ModeFlags {
        ord: 1i32
    };
    pub const WRITE: ModeFlags = ModeFlags {
        ord: 2i32
    };
    pub const READ_WRITE: ModeFlags = ModeFlags {
        ord: 3i32
    };
    pub const WRITE_READ: ModeFlags = ModeFlags {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for ModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ModeFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ModeFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 7i32 => Some(Self {
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
            Self::READ => "READ", Self::WRITE => "WRITE", Self::READ_WRITE => "READ_WRITE", Self::WRITE_READ => "WRITE_READ", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        self.as_str()
    }
}
impl crate::meta::GodotConvert for ModeFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ModeFlags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ModeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressionMode {
    ord: i32
}
impl CompressionMode {
    #[doc(alias = "COMPRESSION_FASTLZ")]
    #[doc = "Godot enumerator name: `COMPRESSION_FASTLZ`"]
    pub const FASTLZ: CompressionMode = CompressionMode {
        ord: 0i32
    };
    #[doc(alias = "COMPRESSION_DEFLATE")]
    #[doc = "Godot enumerator name: `COMPRESSION_DEFLATE`"]
    pub const DEFLATE: CompressionMode = CompressionMode {
        ord: 1i32
    };
    #[doc(alias = "COMPRESSION_ZSTD")]
    #[doc = "Godot enumerator name: `COMPRESSION_ZSTD`"]
    pub const ZSTD: CompressionMode = CompressionMode {
        ord: 2i32
    };
    #[doc(alias = "COMPRESSION_GZIP")]
    #[doc = "Godot enumerator name: `COMPRESSION_GZIP`"]
    pub const GZIP: CompressionMode = CompressionMode {
        ord: 3i32
    };
    #[doc(alias = "COMPRESSION_BROTLI")]
    #[doc = "Godot enumerator name: `COMPRESSION_BROTLI`"]
    pub const BROTLI: CompressionMode = CompressionMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CompressionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressionMode {
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
            Self::FASTLZ => "FASTLZ", Self::DEFLATE => "DEFLATE", Self::ZSTD => "ZSTD", Self::GZIP => "GZIP", Self::BROTLI => "BROTLI", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::FASTLZ => "COMPRESSION_FASTLZ", Self::DEFLATE => "COMPRESSION_DEFLATE", Self::ZSTD => "COMPRESSION_ZSTD", Self::GZIP => "COMPRESSION_GZIP", Self::BROTLI => "COMPRESSION_BROTLI", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CompressionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompressionMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct UnixPermissionFlags {
    ord: u64
}
impl UnixPermissionFlags {
    #[doc(alias = "UNIX_READ_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_READ_OWNER`"]
    pub const READ_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 256u64
    };
    #[doc(alias = "UNIX_WRITE_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_OWNER`"]
    pub const WRITE_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 128u64
    };
    #[doc(alias = "UNIX_EXECUTE_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_OWNER`"]
    pub const EXECUTE_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 64u64
    };
    #[doc(alias = "UNIX_READ_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_READ_GROUP`"]
    pub const READ_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 32u64
    };
    #[doc(alias = "UNIX_WRITE_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_GROUP`"]
    pub const WRITE_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 16u64
    };
    #[doc(alias = "UNIX_EXECUTE_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_GROUP`"]
    pub const EXECUTE_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 8u64
    };
    #[doc(alias = "UNIX_READ_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_READ_OTHER`"]
    pub const READ_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 4u64
    };
    #[doc(alias = "UNIX_WRITE_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_OTHER`"]
    pub const WRITE_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 2u64
    };
    #[doc(alias = "UNIX_EXECUTE_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_OTHER`"]
    pub const EXECUTE_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 1u64
    };
    #[doc(alias = "UNIX_SET_USER_ID")]
    #[doc = "Godot enumerator name: `UNIX_SET_USER_ID`"]
    pub const SET_USER_ID: UnixPermissionFlags = UnixPermissionFlags {
        ord: 2048u64
    };
    #[doc(alias = "UNIX_SET_GROUP_ID")]
    #[doc = "Godot enumerator name: `UNIX_SET_GROUP_ID`"]
    pub const SET_GROUP_ID: UnixPermissionFlags = UnixPermissionFlags {
        ord: 1024u64
    };
    #[doc(alias = "UNIX_RESTRICTED_DELETE")]
    #[doc = "Godot enumerator name: `UNIX_RESTRICTED_DELETE`"]
    pub const RESTRICTED_DELETE: UnixPermissionFlags = UnixPermissionFlags {
        ord: 512u64
    };
    
}
impl std::fmt::Debug for UnixPermissionFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::READ_OWNER => "READ_OWNER", Self::WRITE_OWNER => "WRITE_OWNER", Self::EXECUTE_OWNER => "EXECUTE_OWNER", Self::READ_GROUP => "READ_GROUP", Self::WRITE_GROUP => "WRITE_GROUP", Self::EXECUTE_GROUP => "EXECUTE_GROUP", Self::READ_OTHER => "READ_OTHER", Self::WRITE_OTHER => "WRITE_OTHER", Self::EXECUTE_OTHER => "EXECUTE_OTHER", Self::SET_USER_ID => "SET_USER_ID", Self::SET_GROUP_ID => "SET_GROUP_ID", Self::RESTRICTED_DELETE => "RESTRICTED_DELETE", _ => {
                f.debug_struct("UnixPermissionFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for UnixPermissionFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for UnixPermissionFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for UnixPermissionFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for UnixPermissionFlags {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UnixPermissionFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}