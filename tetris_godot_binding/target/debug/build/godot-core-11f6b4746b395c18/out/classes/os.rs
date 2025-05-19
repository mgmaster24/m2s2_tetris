#![doc = "Sidecar module for class [`Os`][crate::classes::Os].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OS` enums](https://docs.godotengine.org/en/stable/classes/class_os.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OS.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`os`][crate::classes::os]: sidecar module with related enum/flag types\n* [`IOs`][crate::classes::IOs]: virtual methods\n\n\nSee also [Godot docs for `OS`](https://docs.godotengine.org/en/stable/classes/class_os.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Os::singleton()`][Os::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Os {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Os`][crate::classes::Os].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OS` methods](https://docs.godotengine.org/en/stable/classes/class_os.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOs: crate::obj::GodotClass < Base = Os > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Os {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"OS");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_entropy(&mut self, size: i32,) -> PackedByteArray {
            type CallSig = (PackedByteArray, i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_entropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_ca_certificates(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_system_ca_certificates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connected_midi_inputs(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_connected_midi_inputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn open_midi_inputs(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "open_midi_inputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close_midi_inputs(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "close_midi_inputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn alert_full(&mut self, text: CowArg < GString >, title: CowArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (text, title,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "alert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::alert_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn alert(&mut self, text: impl AsArg < GString >,) {
            self.alert_ex(text,) . done()
        }
        #[inline]
        pub fn alert_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a,) -> ExAlert < 'a > {
            ExAlert::new(self, text,)
        }
        pub fn crash(&mut self, message: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "crash", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_low_processor_usage_mode(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "set_low_processor_usage_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_low_processor_usage_mode(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_in_low_processor_usage_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_low_processor_usage_mode_sleep_usec(&mut self, usec: i32,) {
            type CallSig = ((), i32);
            let args = (usec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "set_low_processor_usage_mode_sleep_usec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_low_processor_usage_mode_sleep_usec(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_low_processor_usage_mode_sleep_usec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_delta_smoothing(&mut self, delta_smoothing_enabled: bool,) {
            type CallSig = ((), bool);
            let args = (delta_smoothing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "set_delta_smoothing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_delta_smoothing_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_delta_smoothing_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_processor_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_processor_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_processor_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_processor_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_fonts(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_system_fonts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_system_font_path_full(&self, font_name: CowArg < GString >, weight: i32, stretch: i32, italic: bool,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >, i32, i32, bool);
            let args = (font_name, weight, stretch, italic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_system_font_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_system_font_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_system_font_path(&self, font_name: impl AsArg < GString >,) -> GString {
            self.get_system_font_path_ex(font_name,) . done()
        }
        #[inline]
        pub fn get_system_font_path_ex < 'a > (&'a self, font_name: impl AsArg < GString > + 'a,) -> ExGetSystemFontPath < 'a > {
            ExGetSystemFontPath::new(self, font_name,)
        }
        pub(crate) fn get_system_font_path_for_text_full(&self, font_name: CowArg < GString >, text: CowArg < GString >, locale: CowArg < GString >, script: CowArg < GString >, weight: i32, stretch: i32, italic: bool,) -> PackedStringArray {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (PackedStringArray, CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, CowArg < 'a3, GString >, i32, i32, bool);
            let args = (font_name, text, locale, script, weight, stretch, italic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_system_font_path_for_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_system_font_path_for_text_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_system_font_path_for_text(&self, font_name: impl AsArg < GString >, text: impl AsArg < GString >,) -> PackedStringArray {
            self.get_system_font_path_for_text_ex(font_name, text,) . done()
        }
        #[inline]
        pub fn get_system_font_path_for_text_ex < 'a > (&'a self, font_name: impl AsArg < GString > + 'a, text: impl AsArg < GString > + 'a,) -> ExGetSystemFontPathForText < 'a > {
            ExGetSystemFontPathForText::new(self, font_name, text,)
        }
        pub fn get_executable_path(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_executable_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn read_string_from_stdin(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "read_string_from_stdin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn execute_full(&mut self, path: CowArg < GString >, arguments: RefArg < PackedStringArray >, output: RefArg < VariantArray >, read_stderr: bool, open_console: bool,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, > = (i32, CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, RefArg < 'a2, VariantArray >, bool, bool);
            let args = (path, arguments, output, read_stderr, open_console,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "execute", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::execute_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn execute(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> i32 {
            self.execute_ex(path, arguments,) . done()
        }
        #[inline]
        pub fn execute_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> ExExecute < 'a > {
            ExExecute::new(self, path, arguments,)
        }
        pub fn execute_with_pipe(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> Dictionary {
            type CallSig < 'a0, 'a1, > = (Dictionary, CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >);
            let args = (path.into_arg(), RefArg::new(arguments),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "execute_with_pipe", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_process_full(&mut self, path: CowArg < GString >, arguments: RefArg < PackedStringArray >, open_console: bool,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, bool);
            let args = (path, arguments, open_console,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "create_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_process_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_process(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> i32 {
            self.create_process_ex(path, arguments,) . done()
        }
        #[inline]
        pub fn create_process_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> ExCreateProcess < 'a > {
            ExCreateProcess::new(self, path, arguments,)
        }
        pub fn create_instance(&mut self, arguments: &PackedStringArray,) -> i32 {
            type CallSig < 'a0, > = (i32, RefArg < 'a0, PackedStringArray >);
            let args = (RefArg::new(arguments),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "create_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn kill(&mut self, pid: i32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, i32);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "kill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shell_open(&mut self, uri: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (uri.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "shell_open", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shell_show_in_file_manager_full(&mut self, file_or_dir_path: CowArg < GString >, open_folder: bool,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >, bool);
            let args = (file_or_dir_path, open_folder,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "shell_show_in_file_manager", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shell_show_in_file_manager_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shell_show_in_file_manager(&mut self, file_or_dir_path: impl AsArg < GString >,) -> crate::global::Error {
            self.shell_show_in_file_manager_ex(file_or_dir_path,) . done()
        }
        #[inline]
        pub fn shell_show_in_file_manager_ex < 'a > (&'a mut self, file_or_dir_path: impl AsArg < GString > + 'a,) -> ExShellShowInFileManager < 'a > {
            ExShellShowInFileManager::new(self, file_or_dir_path,)
        }
        pub fn is_process_running(&self, pid: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_process_running", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_exit_code(&self, pid: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_process_exit_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_id(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_process_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_environment(&self, variable: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "has_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment(&self, variable: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment(&self, variable: impl AsArg < GString >, value: impl AsArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (variable.into_arg(), value.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "set_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unset_environment(&self, variable: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "unset_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_distribution_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_distribution_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cmdline_args(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_cmdline_args", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cmdline_user_args(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_cmdline_user_args", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_video_adapter_driver_info(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_video_adapter_driver_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_restart_on_exit_full(&mut self, restart: bool, arguments: RefArg < PackedStringArray >,) {
            type CallSig < 'a0, > = ((), bool, RefArg < 'a0, PackedStringArray >);
            let args = (restart, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "set_restart_on_exit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_restart_on_exit_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_restart_on_exit(&mut self, restart: bool,) {
            self.set_restart_on_exit_ex(restart,) . done()
        }
        #[inline]
        pub fn set_restart_on_exit_ex < 'a > (&'a mut self, restart: bool,) -> ExSetRestartOnExit < 'a > {
            ExSetRestartOnExit::new(self, restart,)
        }
        pub fn is_restart_on_exit_set(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_restart_on_exit_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_restart_on_exit_arguments(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_restart_on_exit_arguments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn delay_usec(&self, usec: i32,) {
            type CallSig = ((), i32);
            let args = (usec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "delay_usec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn delay_msec(&self, msec: i32,) {
            type CallSig = ((), i32);
            let args = (msec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "delay_msec", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale_language(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_locale_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_model_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_model_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_userfs_persistent(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_userfs_persistent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_stdout_verbose(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_stdout_verbose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_debug_build(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_debug_build", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_static_memory_usage(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_static_memory_usage", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_static_memory_peak_usage(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_static_memory_peak_usage", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_memory_info(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_memory_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_to_trash(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "move_to_trash", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_user_data_dir(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_user_data_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_system_dir_full(&self, dir: crate::classes::os::SystemDir, shared_storage: bool,) -> GString {
            type CallSig = (GString, crate::classes::os::SystemDir, bool);
            let args = (dir, shared_storage,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_system_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_system_dir_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_system_dir(&self, dir: crate::classes::os::SystemDir,) -> GString {
            self.get_system_dir_ex(dir,) . done()
        }
        #[inline]
        pub fn get_system_dir_ex < 'a > (&'a self, dir: crate::classes::os::SystemDir,) -> ExGetSystemDir < 'a > {
            ExGetSystemDir::new(self, dir,)
        }
        pub fn get_config_dir(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_config_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data_dir(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_data_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cache_dir(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_cache_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_id(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_unique_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keycode_string(&self, code: crate::global::Key,) -> GString {
            type CallSig = (GString, crate::global::Key);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_keycode_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_keycode_unicode(&self, code: i64,) -> bool {
            type CallSig = (bool, i64);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_keycode_unicode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_keycode_from_string(&self, string: impl AsArg < GString >,) -> crate::global::Key {
            type CallSig < 'a0, > = (crate::global::Key, CowArg < 'a0, GString >);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "find_keycode_from_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_file_access_save_and_swap(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "set_use_file_access_save_and_swap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_thread_name(&mut self, name: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "set_thread_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_thread_caller_id(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_thread_caller_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_main_thread_id(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_main_thread_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_feature(&self, tag_name: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (tag_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "has_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sandboxed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "is_sandboxed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_permission(&mut self, name: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "request_permission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_permissions(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "request_permissions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_granted_permissions(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "get_granted_permissions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn revoke_granted_permissions(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Os", "revoke_granted_permissions", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Os {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"OS"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Os {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Os {
        
    }
    impl std::ops::Deref for Os {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Os {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Os`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Os {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Os > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Os::alert_ex`][super::Os::alert_ex]."]
#[must_use]
pub struct ExAlert < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, text: CowArg < 'a, GString >, title: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAlert < 'a > {
    fn new(surround_object: &'a mut re_export::Os, text: impl AsArg < GString > + 'a,) -> Self {
        let title = GString::from("Alert!");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), title: CowArg::Owned(title),
        }
    }
    #[inline]
    pub fn title(self, title: impl AsArg < GString > + 'a) -> Self {
        Self {
            title: title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, title,
        }
        = self;
        re_export::Os::alert_full(surround_object, text, title,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_font_path_ex`][super::Os::get_system_font_path_ex]."]
#[must_use]
pub struct ExGetSystemFontPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Os, font_name: CowArg < 'a, GString >, weight: i32, stretch: i32, italic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemFontPath < 'a > {
    fn new(surround_object: &'a re_export::Os, font_name: impl AsArg < GString > + 'a,) -> Self {
        let weight = 400i32;
        let stretch = 100i32;
        let italic = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_name: font_name.into_arg(), weight: weight, stretch: stretch, italic: italic,
        }
    }
    #[inline]
    pub fn weight(self, weight: i32) -> Self {
        Self {
            weight: weight, .. self
        }
    }
    #[inline]
    pub fn stretch(self, stretch: i32) -> Self {
        Self {
            stretch: stretch, .. self
        }
    }
    #[inline]
    pub fn italic(self, italic: bool) -> Self {
        Self {
            italic: italic, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, font_name, weight, stretch, italic,
        }
        = self;
        re_export::Os::get_system_font_path_full(surround_object, font_name, weight, stretch, italic,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_font_path_for_text_ex`][super::Os::get_system_font_path_for_text_ex]."]
#[must_use]
pub struct ExGetSystemFontPathForText < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Os, font_name: CowArg < 'a, GString >, text: CowArg < 'a, GString >, locale: CowArg < 'a, GString >, script: CowArg < 'a, GString >, weight: i32, stretch: i32, italic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemFontPathForText < 'a > {
    fn new(surround_object: &'a re_export::Os, font_name: impl AsArg < GString > + 'a, text: impl AsArg < GString > + 'a,) -> Self {
        let locale = GString::from("");
        let script = GString::from("");
        let weight = 400i32;
        let stretch = 100i32;
        let italic = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_name: font_name.into_arg(), text: text.into_arg(), locale: CowArg::Owned(locale), script: CowArg::Owned(script), weight: weight, stretch: stretch, italic: italic,
        }
    }
    #[inline]
    pub fn locale(self, locale: impl AsArg < GString > + 'a) -> Self {
        Self {
            locale: locale.into_arg(), .. self
        }
    }
    #[inline]
    pub fn script(self, script: impl AsArg < GString > + 'a) -> Self {
        Self {
            script: script.into_arg(), .. self
        }
    }
    #[inline]
    pub fn weight(self, weight: i32) -> Self {
        Self {
            weight: weight, .. self
        }
    }
    #[inline]
    pub fn stretch(self, stretch: i32) -> Self {
        Self {
            stretch: stretch, .. self
        }
    }
    #[inline]
    pub fn italic(self, italic: bool) -> Self {
        Self {
            italic: italic, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, font_name, text, locale, script, weight, stretch, italic,
        }
        = self;
        re_export::Os::get_system_font_path_for_text_full(surround_object, font_name, text, locale, script, weight, stretch, italic,)
    }
}
#[doc = "Default-param extender for [`Os::execute_ex`][super::Os::execute_ex]."]
#[must_use]
pub struct ExExecute < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, path: CowArg < 'a, GString >, arguments: CowArg < 'a, PackedStringArray >, output: CowArg < 'a, VariantArray >, read_stderr: bool, open_console: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExecute < 'a > {
    fn new(surround_object: &'a mut re_export::Os, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> Self {
        let output = Array::new();
        let read_stderr = false;
        let open_console = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), arguments: CowArg::Borrowed(arguments), output: CowArg::Owned(output), read_stderr: read_stderr, open_console: open_console,
        }
    }
    #[inline]
    pub fn output(self, output: &'a VariantArray) -> Self {
        Self {
            output: CowArg::Borrowed(output), .. self
        }
    }
    #[inline]
    pub fn read_stderr(self, read_stderr: bool) -> Self {
        Self {
            read_stderr: read_stderr, .. self
        }
    }
    #[inline]
    pub fn open_console(self, open_console: bool) -> Self {
        Self {
            open_console: open_console, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, path, arguments, output, read_stderr, open_console,
        }
        = self;
        re_export::Os::execute_full(surround_object, path, arguments.cow_as_arg(), output.cow_as_arg(), read_stderr, open_console,)
    }
}
#[doc = "Default-param extender for [`Os::create_process_ex`][super::Os::create_process_ex]."]
#[must_use]
pub struct ExCreateProcess < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, path: CowArg < 'a, GString >, arguments: CowArg < 'a, PackedStringArray >, open_console: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateProcess < 'a > {
    fn new(surround_object: &'a mut re_export::Os, path: impl AsArg < GString > + 'a, arguments: &'a PackedStringArray,) -> Self {
        let open_console = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), arguments: CowArg::Borrowed(arguments), open_console: open_console,
        }
    }
    #[inline]
    pub fn open_console(self, open_console: bool) -> Self {
        Self {
            open_console: open_console, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, path, arguments, open_console,
        }
        = self;
        re_export::Os::create_process_full(surround_object, path, arguments.cow_as_arg(), open_console,)
    }
}
#[doc = "Default-param extender for [`Os::shell_show_in_file_manager_ex`][super::Os::shell_show_in_file_manager_ex]."]
#[must_use]
pub struct ExShellShowInFileManager < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, file_or_dir_path: CowArg < 'a, GString >, open_folder: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShellShowInFileManager < 'a > {
    fn new(surround_object: &'a mut re_export::Os, file_or_dir_path: impl AsArg < GString > + 'a,) -> Self {
        let open_folder = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, file_or_dir_path: file_or_dir_path.into_arg(), open_folder: open_folder,
        }
    }
    #[inline]
    pub fn open_folder(self, open_folder: bool) -> Self {
        Self {
            open_folder: open_folder, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, file_or_dir_path, open_folder,
        }
        = self;
        re_export::Os::shell_show_in_file_manager_full(surround_object, file_or_dir_path, open_folder,)
    }
}
#[doc = "Default-param extender for [`Os::set_restart_on_exit_ex`][super::Os::set_restart_on_exit_ex]."]
#[must_use]
pub struct ExSetRestartOnExit < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Os, restart: bool, arguments: CowArg < 'a, PackedStringArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetRestartOnExit < 'a > {
    fn new(surround_object: &'a mut re_export::Os, restart: bool,) -> Self {
        let arguments = PackedStringArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, restart: restart, arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a PackedStringArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, restart, arguments,
        }
        = self;
        re_export::Os::set_restart_on_exit_full(surround_object, restart, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Os::get_system_dir_ex`][super::Os::get_system_dir_ex]."]
#[must_use]
pub struct ExGetSystemDir < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Os, dir: crate::classes::os::SystemDir, shared_storage: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSystemDir < 'a > {
    fn new(surround_object: &'a re_export::Os, dir: crate::classes::os::SystemDir,) -> Self {
        let shared_storage = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dir: dir, shared_storage: shared_storage,
        }
    }
    #[inline]
    pub fn shared_storage(self, shared_storage: bool) -> Self {
        Self {
            shared_storage: shared_storage, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, dir, shared_storage,
        }
        = self;
        re_export::Os::get_system_dir_full(surround_object, dir, shared_storage,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderingDriver {
    ord: i32
}
impl RenderingDriver {
    #[doc(alias = "RENDERING_DRIVER_VULKAN")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_VULKAN`"]
    pub const VULKAN: RenderingDriver = RenderingDriver {
        ord: 0i32
    };
    #[doc(alias = "RENDERING_DRIVER_OPENGL3")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_OPENGL3`"]
    pub const OPENGL3: RenderingDriver = RenderingDriver {
        ord: 1i32
    };
    #[doc(alias = "RENDERING_DRIVER_D3D12")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_D3D12`"]
    pub const D3D12: RenderingDriver = RenderingDriver {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RenderingDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderingDriver") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderingDriver {
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
            Self::VULKAN => "VULKAN", Self::OPENGL3 => "OPENGL3", Self::D3D12 => "D3D12", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VULKAN => "RENDERING_DRIVER_VULKAN", Self::OPENGL3 => "RENDERING_DRIVER_OPENGL3", Self::D3D12 => "RENDERING_DRIVER_D3D12", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for RenderingDriver {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderingDriver {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderingDriver {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SystemDir {
    ord: i32
}
impl SystemDir {
    #[doc(alias = "SYSTEM_DIR_DESKTOP")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DESKTOP`"]
    pub const DESKTOP: SystemDir = SystemDir {
        ord: 0i32
    };
    #[doc(alias = "SYSTEM_DIR_DCIM")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DCIM`"]
    pub const DCIM: SystemDir = SystemDir {
        ord: 1i32
    };
    #[doc(alias = "SYSTEM_DIR_DOCUMENTS")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DOCUMENTS`"]
    pub const DOCUMENTS: SystemDir = SystemDir {
        ord: 2i32
    };
    #[doc(alias = "SYSTEM_DIR_DOWNLOADS")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DOWNLOADS`"]
    pub const DOWNLOADS: SystemDir = SystemDir {
        ord: 3i32
    };
    #[doc(alias = "SYSTEM_DIR_MOVIES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_MOVIES`"]
    pub const MOVIES: SystemDir = SystemDir {
        ord: 4i32
    };
    #[doc(alias = "SYSTEM_DIR_MUSIC")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_MUSIC`"]
    pub const MUSIC: SystemDir = SystemDir {
        ord: 5i32
    };
    #[doc(alias = "SYSTEM_DIR_PICTURES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_PICTURES`"]
    pub const PICTURES: SystemDir = SystemDir {
        ord: 6i32
    };
    #[doc(alias = "SYSTEM_DIR_RINGTONES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_RINGTONES`"]
    pub const RINGTONES: SystemDir = SystemDir {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for SystemDir {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SystemDir") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SystemDir {
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
            Self::DESKTOP => "DESKTOP", Self::DCIM => "DCIM", Self::DOCUMENTS => "DOCUMENTS", Self::DOWNLOADS => "DOWNLOADS", Self::MOVIES => "MOVIES", Self::MUSIC => "MUSIC", Self::PICTURES => "PICTURES", Self::RINGTONES => "RINGTONES", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DESKTOP => "SYSTEM_DIR_DESKTOP", Self::DCIM => "SYSTEM_DIR_DCIM", Self::DOCUMENTS => "SYSTEM_DIR_DOCUMENTS", Self::DOWNLOADS => "SYSTEM_DIR_DOWNLOADS", Self::MOVIES => "SYSTEM_DIR_MOVIES", Self::MUSIC => "SYSTEM_DIR_MUSIC", Self::PICTURES => "SYSTEM_DIR_PICTURES", Self::RINGTONES => "SYSTEM_DIR_RINGTONES", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SystemDir {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SystemDir {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SystemDir {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}