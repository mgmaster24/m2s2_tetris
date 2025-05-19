#![doc = "Sidecar module for class [`StreamPeer`][crate::classes::StreamPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StreamPeer` enums](https://docs.godotengine.org/en/stable/classes/class_streampeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StreamPeer.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`stream_peer`][crate::classes::stream_peer]: sidecar module with related enum/flag types\n* [`IStreamPeer`][crate::classes::IStreamPeer]: virtual methods\n\n\nSee also [Godot docs for `StreamPeer`](https://docs.godotengine.org/en/stable/classes/class_streampeer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<StreamPeer>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StreamPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`StreamPeer`][crate::classes::StreamPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `StreamPeer` methods](https://docs.godotengine.org/en/stable/classes/class_streampeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStreamPeer: crate::obj::GodotClass < Base = StreamPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl StreamPeer {
        pub fn put_data(&mut self, data: &PackedByteArray,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, RefArg < 'a0, PackedByteArray >);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_partial_data(&mut self, data: &PackedByteArray,) -> VariantArray {
            type CallSig < 'a0, > = (VariantArray, RefArg < 'a0, PackedByteArray >);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_partial_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data(&mut self, bytes: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_partial_data(&mut self, bytes: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_partial_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_available_bytes(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_available_bytes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_big_endian(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "set_big_endian", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_big_endian_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "is_big_endian_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_8(&mut self, value: i8,) {
            type CallSig = ((), i8);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u8(&mut self, value: u8,) {
            type CallSig = ((), u8);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_16(&mut self, value: i16,) {
            type CallSig = ((), i16);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u16(&mut self, value: u16,) {
            type CallSig = ((), u16);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_32(&mut self, value: i32,) {
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u32(&mut self, value: u32,) {
            type CallSig = ((), u32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_64(&mut self, value: i64,) {
            type CallSig = ((), i64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u64(&mut self, value: u64,) {
            type CallSig = ((), u64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_float(&mut self, value: f32,) {
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_double(&mut self, value: f64,) {
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_string(&mut self, value: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (value.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_utf8_string(&mut self, value: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (value.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_utf8_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn put_var_full(&mut self, value: RefArg < Variant >, full_objects: bool,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >, bool);
            let args = (value, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "put_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::put_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn put_var(&mut self, value: &Variant,) {
            self.put_var_ex(value,) . done()
        }
        #[inline]
        pub fn put_var_ex < 'a > (&'a mut self, value: &'a Variant,) -> ExPutVar < 'a > {
            ExPutVar::new(self, value,)
        }
        pub fn get_8(&mut self,) -> i8 {
            type CallSig = (i8,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u8(&mut self,) -> u8 {
            type CallSig = (u8,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_16(&mut self,) -> i16 {
            type CallSig = (i16,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u16(&mut self,) -> u16 {
            type CallSig = (u16,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_32(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u32(&mut self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_64(&mut self,) -> i64 {
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u64(&mut self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_float(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_double(&mut self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_string_full(&mut self, bytes: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_string(&mut self,) -> GString {
            self.get_string_ex() . done()
        }
        #[inline]
        pub fn get_string_ex < 'a > (&'a mut self,) -> ExGetString < 'a > {
            ExGetString::new(self,)
        }
        pub(crate) fn get_utf8_string_full(&mut self, bytes: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_utf8_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_utf8_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_utf8_string(&mut self,) -> GString {
            self.get_utf8_string_ex() . done()
        }
        #[inline]
        pub fn get_utf8_string_ex < 'a > (&'a mut self,) -> ExGetUtf8String < 'a > {
            ExGetUtf8String::new(self,)
        }
        pub(crate) fn get_var_full(&mut self, allow_objects: bool,) -> Variant {
            type CallSig = (Variant, bool);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeer", "get_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_var(&mut self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[inline]
        pub fn get_var_ex < 'a > (&'a mut self,) -> ExGetVar < 'a > {
            ExGetVar::new(self,)
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
    impl crate::obj::GodotClass for StreamPeer {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"StreamPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StreamPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StreamPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StreamPeer {
        
    }
    impl std::ops::Deref for StreamPeer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StreamPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`StreamPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_StreamPeer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::StreamPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`StreamPeer::put_var_ex`][super::StreamPeer::put_var_ex]."]
#[must_use]
pub struct ExPutVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeer, value: CowArg < 'a, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPutVar < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer, value: &'a Variant,) -> Self {
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
        re_export::StreamPeer::put_var_full(surround_object, value.cow_as_arg(), full_objects,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_string_ex`][super::StreamPeer::get_string_ex]."]
#[must_use]
pub struct ExGetString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeer, bytes: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetString < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
        let bytes = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: bytes,
        }
    }
    #[inline]
    pub fn bytes(self, bytes: i32) -> Self {
        Self {
            bytes: bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, bytes,
        }
        = self;
        re_export::StreamPeer::get_string_full(surround_object, bytes,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_utf8_string_ex`][super::StreamPeer::get_utf8_string_ex]."]
#[must_use]
pub struct ExGetUtf8String < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeer, bytes: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUtf8String < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
        let bytes = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: bytes,
        }
    }
    #[inline]
    pub fn bytes(self, bytes: i32) -> Self {
        Self {
            bytes: bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, bytes,
        }
        = self;
        re_export::StreamPeer::get_utf8_string_full(surround_object, bytes,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_var_ex`][super::StreamPeer::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeer, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVar < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
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
        re_export::StreamPeer::get_var_full(surround_object, allow_objects,)
    }
}