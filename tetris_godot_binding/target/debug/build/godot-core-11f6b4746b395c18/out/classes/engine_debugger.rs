#![doc = "Sidecar module for class [`EngineDebugger`][crate::classes::EngineDebugger].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EngineDebugger` enums](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EngineDebugger.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`engine_debugger`][crate::classes::engine_debugger]: sidecar module with related enum/flag types\n* [`IEngineDebugger`][crate::classes::IEngineDebugger]: virtual methods\n\n\nSee also [Godot docs for `EngineDebugger`](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`EngineDebugger::singleton()`][EngineDebugger::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EngineDebugger {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EngineDebugger`][crate::classes::EngineDebugger].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EngineDebugger` methods](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEngineDebugger: crate::obj::GodotClass < Base = EngineDebugger > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl EngineDebugger {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"EngineDebugger");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn is_active(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_profiler(&mut self, name: impl AsArg < StringName >, profiler: impl AsObjectArg < crate::classes::EngineProfiler >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, ObjectArg < crate::classes::EngineProfiler >);
            let args = (name.into_arg(), profiler.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "register_profiler", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_profiler(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "unregister_profiler", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_profiling(&mut self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_profiling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_profiler(&mut self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "has_profiler", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn profiler_add_frame_data(&mut self, name: impl AsArg < StringName >, data: &VariantArray,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, VariantArray >);
            let args = (name.into_arg(), RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "profiler_add_frame_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn profiler_enable_full(&mut self, name: CowArg < StringName >, enable: bool, arguments: RefArg < VariantArray >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, bool, RefArg < 'a1, VariantArray >);
            let args = (name, enable, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "profiler_enable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::profiler_enable_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn profiler_enable(&mut self, name: impl AsArg < StringName >, enable: bool,) {
            self.profiler_enable_ex(name, enable,) . done()
        }
        #[inline]
        pub fn profiler_enable_ex < 'a > (&'a mut self, name: impl AsArg < StringName > + 'a, enable: bool,) -> ExProfilerEnable < 'a > {
            ExProfilerEnable::new(self, name, enable,)
        }
        pub fn register_message_capture(&mut self, name: impl AsArg < StringName >, callable: &Callable,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Callable >);
            let args = (name.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "register_message_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_message_capture(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "unregister_message_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_capture(&mut self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "has_capture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn line_poll(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "line_poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn send_message(&mut self, message: impl AsArg < GString >, data: &VariantArray,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, VariantArray >);
            let args = (message.into_arg(), RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "send_message", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn debug_full(&mut self, can_continue: bool, is_error_breakpoint: bool,) {
            type CallSig = ((), bool, bool);
            let args = (can_continue, is_error_breakpoint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "debug", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::debug_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn debug(&mut self,) {
            self.debug_ex() . done()
        }
        #[inline]
        pub fn debug_ex < 'a > (&'a mut self,) -> ExDebug < 'a > {
            ExDebug::new(self,)
        }
        pub(crate) fn script_debug_full(&mut self, language: ObjectArg < crate::classes::ScriptLanguage >, can_continue: bool, is_error_breakpoint: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::ScriptLanguage >, bool, bool);
            let args = (language, can_continue, is_error_breakpoint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "script_debug", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::script_debug_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn script_debug(&mut self, language: impl AsObjectArg < crate::classes::ScriptLanguage >,) {
            self.script_debug_ex(language,) . done()
        }
        #[inline]
        pub fn script_debug_ex < 'a > (&'a mut self, language: impl AsObjectArg < crate::classes::ScriptLanguage >,) -> ExScriptDebug < 'a > {
            ExScriptDebug::new(self, language,)
        }
        pub fn set_lines_left(&mut self, lines: i32,) {
            type CallSig = ((), i32);
            let args = (lines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "set_lines_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lines_left(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "get_lines_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth(&mut self, depth: i32,) {
            type CallSig = ((), i32);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "set_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "get_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_breakpoint(&self, line: i32, source: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, i32, CowArg < 'a0, StringName >);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_breakpoint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_skipping_breakpoints(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_skipping_breakpoints", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn insert_breakpoint(&mut self, line: i32, source: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "insert_breakpoint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_breakpoint(&mut self, line: i32, source: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "remove_breakpoint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_breakpoints(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EngineDebugger", "clear_breakpoints", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EngineDebugger {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"EngineDebugger"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for EngineDebugger {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EngineDebugger {
        
    }
    impl std::ops::Deref for EngineDebugger {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EngineDebugger {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EngineDebugger`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_EngineDebugger {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EngineDebugger > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EngineDebugger::profiler_enable_ex`][super::EngineDebugger::profiler_enable_ex]."]
#[must_use]
pub struct ExProfilerEnable < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EngineDebugger, name: CowArg < 'a, StringName >, enable: bool, arguments: CowArg < 'a, VariantArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExProfilerEnable < 'a > {
    fn new(surround_object: &'a mut re_export::EngineDebugger, name: impl AsArg < StringName > + 'a, enable: bool,) -> Self {
        let arguments = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), enable: enable, arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a VariantArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, enable, arguments,
        }
        = self;
        re_export::EngineDebugger::profiler_enable_full(surround_object, name, enable, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EngineDebugger::debug_ex`][super::EngineDebugger::debug_ex]."]
#[must_use]
pub struct ExDebug < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EngineDebugger, can_continue: bool, is_error_breakpoint: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDebug < 'a > {
    fn new(surround_object: &'a mut re_export::EngineDebugger,) -> Self {
        let can_continue = true;
        let is_error_breakpoint = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, can_continue: can_continue, is_error_breakpoint: is_error_breakpoint,
        }
    }
    #[inline]
    pub fn can_continue(self, can_continue: bool) -> Self {
        Self {
            can_continue: can_continue, .. self
        }
    }
    #[inline]
    pub fn is_error_breakpoint(self, is_error_breakpoint: bool) -> Self {
        Self {
            is_error_breakpoint: is_error_breakpoint, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, can_continue, is_error_breakpoint,
        }
        = self;
        re_export::EngineDebugger::debug_full(surround_object, can_continue, is_error_breakpoint,)
    }
}
#[doc = "Default-param extender for [`EngineDebugger::script_debug_ex`][super::EngineDebugger::script_debug_ex]."]
#[must_use]
pub struct ExScriptDebug < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EngineDebugger, language: ObjectCow < crate::classes::ScriptLanguage >, can_continue: bool, is_error_breakpoint: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScriptDebug < 'a > {
    fn new(surround_object: &'a mut re_export::EngineDebugger, language: impl AsObjectArg < crate::classes::ScriptLanguage >,) -> Self {
        let can_continue = true;
        let is_error_breakpoint = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, language: language.consume_arg(), can_continue: can_continue, is_error_breakpoint: is_error_breakpoint,
        }
    }
    #[inline]
    pub fn can_continue(self, can_continue: bool) -> Self {
        Self {
            can_continue: can_continue, .. self
        }
    }
    #[inline]
    pub fn is_error_breakpoint(self, is_error_breakpoint: bool) -> Self {
        Self {
            is_error_breakpoint: is_error_breakpoint, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, language, can_continue, is_error_breakpoint,
        }
        = self;
        re_export::EngineDebugger::script_debug_full(surround_object, language.cow_as_object_arg(), can_continue, is_error_breakpoint,)
    }
}