#![doc = "Sidecar module for class [`WorkerThreadPool`][crate::classes::WorkerThreadPool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WorkerThreadPool` enums](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WorkerThreadPool.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`worker_thread_pool`][crate::classes::worker_thread_pool]: sidecar module with related enum/flag types\n* [`IWorkerThreadPool`][crate::classes::IWorkerThreadPool]: virtual methods\n\n\nSee also [Godot docs for `WorkerThreadPool`](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`WorkerThreadPool::singleton()`][WorkerThreadPool::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WorkerThreadPool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WorkerThreadPool`][crate::classes::WorkerThreadPool].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WorkerThreadPool` methods](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWorkerThreadPool: crate::obj::GodotClass < Base = WorkerThreadPool > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WorkerThreadPool {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"WorkerThreadPool");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn add_task_full(&mut self, action: RefArg < Callable >, high_priority: bool, description: CowArg < GString >,) -> i64 {
            type CallSig < 'a0, 'a1, > = (i64, RefArg < 'a0, Callable >, bool, CowArg < 'a1, GString >);
            let args = (action, high_priority, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "add_task", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_task_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_task(&mut self, action: &Callable,) -> i64 {
            self.add_task_ex(action,) . done()
        }
        #[inline]
        pub fn add_task_ex < 'a > (&'a mut self, action: &'a Callable,) -> ExAddTask < 'a > {
            ExAddTask::new(self, action,)
        }
        pub fn is_task_completed(&self, task_id: i64,) -> bool {
            type CallSig = (bool, i64);
            let args = (task_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "is_task_completed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn wait_for_task_completion(&mut self, task_id: i64,) -> crate::global::Error {
            type CallSig = (crate::global::Error, i64);
            let args = (task_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "wait_for_task_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_group_task_full(&mut self, action: RefArg < Callable >, elements: i32, tasks_needed: i32, high_priority: bool, description: CowArg < GString >,) -> i64 {
            type CallSig < 'a0, 'a1, > = (i64, RefArg < 'a0, Callable >, i32, i32, bool, CowArg < 'a1, GString >);
            let args = (action, elements, tasks_needed, high_priority, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "add_group_task", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_group_task_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_group_task(&mut self, action: &Callable, elements: i32,) -> i64 {
            self.add_group_task_ex(action, elements,) . done()
        }
        #[inline]
        pub fn add_group_task_ex < 'a > (&'a mut self, action: &'a Callable, elements: i32,) -> ExAddGroupTask < 'a > {
            ExAddGroupTask::new(self, action, elements,)
        }
        pub fn is_group_task_completed(&self, group_id: i64,) -> bool {
            type CallSig = (bool, i64);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "is_group_task_completed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_processed_element_count(&self, group_id: i64,) -> u32 {
            type CallSig = (u32, i64);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "get_group_processed_element_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn wait_for_group_task_completion(&mut self, group_id: i64,) {
            type CallSig = ((), i64);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "wait_for_group_task_completion", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WorkerThreadPool {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"WorkerThreadPool"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WorkerThreadPool {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WorkerThreadPool {
        
    }
    impl std::ops::Deref for WorkerThreadPool {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WorkerThreadPool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WorkerThreadPool`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_WorkerThreadPool {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WorkerThreadPool > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`WorkerThreadPool::add_task_ex`][super::WorkerThreadPool::add_task_ex]."]
#[must_use]
pub struct ExAddTask < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WorkerThreadPool, action: CowArg < 'a, Callable >, high_priority: bool, description: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTask < 'a > {
    fn new(surround_object: &'a mut re_export::WorkerThreadPool, action: &'a Callable,) -> Self {
        let high_priority = false;
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: CowArg::Borrowed(action), high_priority: high_priority, description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn high_priority(self, high_priority: bool) -> Self {
        Self {
            high_priority: high_priority, .. self
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'a) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, action, high_priority, description,
        }
        = self;
        re_export::WorkerThreadPool::add_task_full(surround_object, action.cow_as_arg(), high_priority, description,)
    }
}
#[doc = "Default-param extender for [`WorkerThreadPool::add_group_task_ex`][super::WorkerThreadPool::add_group_task_ex]."]
#[must_use]
pub struct ExAddGroupTask < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WorkerThreadPool, action: CowArg < 'a, Callable >, elements: i32, tasks_needed: i32, high_priority: bool, description: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddGroupTask < 'a > {
    fn new(surround_object: &'a mut re_export::WorkerThreadPool, action: &'a Callable, elements: i32,) -> Self {
        let tasks_needed = - 1i32;
        let high_priority = false;
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: CowArg::Borrowed(action), elements: elements, tasks_needed: tasks_needed, high_priority: high_priority, description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn tasks_needed(self, tasks_needed: i32) -> Self {
        Self {
            tasks_needed: tasks_needed, .. self
        }
    }
    #[inline]
    pub fn high_priority(self, high_priority: bool) -> Self {
        Self {
            high_priority: high_priority, .. self
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'a) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, action, elements, tasks_needed, high_priority, description,
        }
        = self;
        re_export::WorkerThreadPool::add_group_task_full(surround_object, action.cow_as_arg(), elements, tasks_needed, high_priority, description,)
    }
}