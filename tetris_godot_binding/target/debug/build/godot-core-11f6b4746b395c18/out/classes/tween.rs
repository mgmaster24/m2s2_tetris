#![doc = "Sidecar module for class [`Tween`][crate::classes::Tween].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tween` enums](https://docs.godotengine.org/en/stable/classes/class_tween.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Tween.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`tween`][crate::classes::tween]: sidecar module with related enum/flag types\n* [`ITween`][crate::classes::ITween]: virtual methods\n\n\nSee also [Godot docs for `Tween`](https://docs.godotengine.org/en/stable/classes/class_tween.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Tween::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Tween {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Tween`][crate::classes::Tween].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Tween` methods](https://docs.godotengine.org/en/stable/classes/class_tween.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITween: crate::obj::GodotClass < Base = Tween > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Tween {
        pub fn tween_property(&mut self, object: impl AsObjectArg < crate::classes::Object >, property: impl AsArg < NodePath >, final_val: &Variant, duration: f64,) -> Option < Gd < crate::classes::PropertyTweener > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::PropertyTweener > >, ObjectArg < crate::classes::Object >, CowArg < 'a0, NodePath >, RefArg < 'a1, Variant >, f64);
            let args = (object.as_object_arg(), property.into_arg(), RefArg::new(final_val), duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "tween_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tween_interval(&mut self, time: f64,) -> Option < Gd < crate::classes::IntervalTweener > > {
            type CallSig = (Option < Gd < crate::classes::IntervalTweener > >, f64);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "tween_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tween_callback(&mut self, callback: &Callable,) -> Option < Gd < crate::classes::CallbackTweener > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::CallbackTweener > >, RefArg < 'a0, Callable >);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "tween_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tween_method(&mut self, method: &Callable, from: &Variant, to: &Variant, duration: f64,) -> Option < Gd < crate::classes::MethodTweener > > {
            type CallSig < 'a0, 'a1, 'a2, > = (Option < Gd < crate::classes::MethodTweener > >, RefArg < 'a0, Callable >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >, f64);
            let args = (RefArg::new(method), RefArg::new(from), RefArg::new(to), duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "tween_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn custom_step(&mut self, delta: f64,) -> bool {
            type CallSig = (bool, f64);
            let args = (delta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "custom_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pause(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn play(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "play", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn kill(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "kill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_elapsed_time(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "get_total_elapsed_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_running(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "is_running", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_valid(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn bind_node(&mut self, node: impl AsObjectArg < crate::classes::Node >,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >, ObjectArg < crate::classes::Node >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "bind_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_mode(&mut self, mode: crate::classes::tween::TweenProcessMode,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >, crate::classes::tween::TweenProcessMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "set_process_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pause_mode(&mut self, mode: crate::classes::tween::TweenPauseMode,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >, crate::classes::tween::TweenPauseMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "set_pause_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_parallel_full(&mut self, parallel: bool,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >, bool);
            let args = (parallel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "set_parallel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_parallel_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_parallel(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            self.set_parallel_ex() . done()
        }
        #[inline]
        pub fn set_parallel_ex < 'a > (&'a mut self,) -> ExSetParallel < 'a > {
            ExSetParallel::new(self,)
        }
        pub(crate) fn set_loops_full(&mut self, loops: i32,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >, i32);
            let args = (loops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "set_loops", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_loops_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_loops(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            self.set_loops_ex() . done()
        }
        #[inline]
        pub fn set_loops_ex < 'a > (&'a mut self,) -> ExSetLoops < 'a > {
            ExSetLoops::new(self,)
        }
        pub fn get_loops_left(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "get_loops_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed: f32,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >, f32);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_trans(&mut self, trans: crate::classes::tween::TransitionType,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >, crate::classes::tween::TransitionType);
            let args = (trans,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "set_trans", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ease(&mut self, ease: crate::classes::tween::EaseType,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >, crate::classes::tween::EaseType);
            let args = (ease,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "set_ease", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn parallel(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "parallel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn chain(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "chain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn interpolate_value(initial_value: &Variant, delta_value: &Variant, elapsed_time: f64, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,) -> Variant {
            type CallSig < 'a0, 'a1, > = (Variant, RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, f64, f64, crate::classes::tween::TransitionType, crate::classes::tween::EaseType);
            let args = (RefArg::new(initial_value), RefArg::new(delta_value), elapsed_time, duration, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tween", "interpolate_value", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for Tween {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Tween"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Tween {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Tween {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Tween {
        
    }
    impl crate::obj::cap::GodotDefault for Tween {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Tween {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Tween {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Tween`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Tween {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Tween > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Tween::set_parallel_ex`][super::Tween::set_parallel_ex]."]
#[must_use]
pub struct ExSetParallel < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tween, parallel: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetParallel < 'a > {
    fn new(surround_object: &'a mut re_export::Tween,) -> Self {
        let parallel = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parallel: parallel,
        }
    }
    #[inline]
    pub fn parallel(self, parallel: bool) -> Self {
        Self {
            parallel: parallel, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Tween > > {
        let Self {
            _phantom, surround_object, parallel,
        }
        = self;
        re_export::Tween::set_parallel_full(surround_object, parallel,)
    }
}
#[doc = "Default-param extender for [`Tween::set_loops_ex`][super::Tween::set_loops_ex]."]
#[must_use]
pub struct ExSetLoops < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tween, loops: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLoops < 'a > {
    fn new(surround_object: &'a mut re_export::Tween,) -> Self {
        let loops = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, loops: loops,
        }
    }
    #[inline]
    pub fn loops(self, loops: i32) -> Self {
        Self {
            loops: loops, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Tween > > {
        let Self {
            _phantom, surround_object, loops,
        }
        = self;
        re_export::Tween::set_loops_full(surround_object, loops,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TweenProcessMode {
    ord: i32
}
impl TweenProcessMode {
    #[doc(alias = "TWEEN_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `TWEEN_PROCESS_PHYSICS`"]
    pub const PHYSICS: TweenProcessMode = TweenProcessMode {
        ord: 0i32
    };
    #[doc(alias = "TWEEN_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `TWEEN_PROCESS_IDLE`"]
    pub const IDLE: TweenProcessMode = TweenProcessMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TweenProcessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TweenProcessMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TweenProcessMode {
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
            Self::PHYSICS => "TWEEN_PROCESS_PHYSICS", Self::IDLE => "TWEEN_PROCESS_IDLE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TweenProcessMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TweenProcessMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TweenProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TweenPauseMode {
    ord: i32
}
impl TweenPauseMode {
    #[doc(alias = "TWEEN_PAUSE_BOUND")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_BOUND`"]
    pub const BOUND: TweenPauseMode = TweenPauseMode {
        ord: 0i32
    };
    #[doc(alias = "TWEEN_PAUSE_STOP")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_STOP`"]
    pub const STOP: TweenPauseMode = TweenPauseMode {
        ord: 1i32
    };
    #[doc(alias = "TWEEN_PAUSE_PROCESS")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_PROCESS`"]
    pub const PROCESS: TweenPauseMode = TweenPauseMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TweenPauseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TweenPauseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TweenPauseMode {
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
            Self::BOUND => "BOUND", Self::STOP => "STOP", Self::PROCESS => "PROCESS", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BOUND => "TWEEN_PAUSE_BOUND", Self::STOP => "TWEEN_PAUSE_STOP", Self::PROCESS => "TWEEN_PAUSE_PROCESS", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TweenPauseMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TweenPauseMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TweenPauseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransitionType {
    ord: i32
}
impl TransitionType {
    #[doc(alias = "TRANS_LINEAR")]
    #[doc = "Godot enumerator name: `TRANS_LINEAR`"]
    pub const LINEAR: TransitionType = TransitionType {
        ord: 0i32
    };
    #[doc(alias = "TRANS_SINE")]
    #[doc = "Godot enumerator name: `TRANS_SINE`"]
    pub const SINE: TransitionType = TransitionType {
        ord: 1i32
    };
    #[doc(alias = "TRANS_QUINT")]
    #[doc = "Godot enumerator name: `TRANS_QUINT`"]
    pub const QUINT: TransitionType = TransitionType {
        ord: 2i32
    };
    #[doc(alias = "TRANS_QUART")]
    #[doc = "Godot enumerator name: `TRANS_QUART`"]
    pub const QUART: TransitionType = TransitionType {
        ord: 3i32
    };
    #[doc(alias = "TRANS_QUAD")]
    #[doc = "Godot enumerator name: `TRANS_QUAD`"]
    pub const QUAD: TransitionType = TransitionType {
        ord: 4i32
    };
    #[doc(alias = "TRANS_EXPO")]
    #[doc = "Godot enumerator name: `TRANS_EXPO`"]
    pub const EXPO: TransitionType = TransitionType {
        ord: 5i32
    };
    #[doc(alias = "TRANS_ELASTIC")]
    #[doc = "Godot enumerator name: `TRANS_ELASTIC`"]
    pub const ELASTIC: TransitionType = TransitionType {
        ord: 6i32
    };
    #[doc(alias = "TRANS_CUBIC")]
    #[doc = "Godot enumerator name: `TRANS_CUBIC`"]
    pub const CUBIC: TransitionType = TransitionType {
        ord: 7i32
    };
    #[doc(alias = "TRANS_CIRC")]
    #[doc = "Godot enumerator name: `TRANS_CIRC`"]
    pub const CIRC: TransitionType = TransitionType {
        ord: 8i32
    };
    #[doc(alias = "TRANS_BOUNCE")]
    #[doc = "Godot enumerator name: `TRANS_BOUNCE`"]
    pub const BOUNCE: TransitionType = TransitionType {
        ord: 9i32
    };
    #[doc(alias = "TRANS_BACK")]
    #[doc = "Godot enumerator name: `TRANS_BACK`"]
    pub const BACK: TransitionType = TransitionType {
        ord: 10i32
    };
    #[doc(alias = "TRANS_SPRING")]
    #[doc = "Godot enumerator name: `TRANS_SPRING`"]
    pub const SPRING: TransitionType = TransitionType {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for TransitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransitionType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransitionType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::LINEAR => "LINEAR", Self::SINE => "SINE", Self::QUINT => "QUINT", Self::QUART => "QUART", Self::QUAD => "QUAD", Self::EXPO => "EXPO", Self::ELASTIC => "ELASTIC", Self::CUBIC => "CUBIC", Self::CIRC => "CIRC", Self::BOUNCE => "BOUNCE", Self::BACK => "BACK", Self::SPRING => "SPRING", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LINEAR => "TRANS_LINEAR", Self::SINE => "TRANS_SINE", Self::QUINT => "TRANS_QUINT", Self::QUART => "TRANS_QUART", Self::QUAD => "TRANS_QUAD", Self::EXPO => "TRANS_EXPO", Self::ELASTIC => "TRANS_ELASTIC", Self::CUBIC => "TRANS_CUBIC", Self::CIRC => "TRANS_CIRC", Self::BOUNCE => "TRANS_BOUNCE", Self::BACK => "TRANS_BACK", Self::SPRING => "TRANS_SPRING", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TransitionType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransitionType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransitionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EaseType {
    ord: i32
}
impl EaseType {
    #[doc(alias = "EASE_IN")]
    #[doc = "Godot enumerator name: `EASE_IN`"]
    pub const IN: EaseType = EaseType {
        ord: 0i32
    };
    #[doc(alias = "EASE_OUT")]
    #[doc = "Godot enumerator name: `EASE_OUT`"]
    pub const OUT: EaseType = EaseType {
        ord: 1i32
    };
    #[doc(alias = "EASE_IN_OUT")]
    #[doc = "Godot enumerator name: `EASE_IN_OUT`"]
    pub const IN_OUT: EaseType = EaseType {
        ord: 2i32
    };
    #[doc(alias = "EASE_OUT_IN")]
    #[doc = "Godot enumerator name: `EASE_OUT_IN`"]
    pub const OUT_IN: EaseType = EaseType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EaseType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EaseType {
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
            Self::IN => "IN", Self::OUT => "OUT", Self::IN_OUT => "IN_OUT", Self::OUT_IN => "OUT_IN", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::IN => "EASE_IN", Self::OUT => "EASE_OUT", Self::IN_OUT => "EASE_IN_OUT", Self::OUT_IN => "EASE_OUT_IN", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for EaseType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EaseType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EaseType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}