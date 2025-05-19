#![doc = "Sidecar module for class [`InstancePlaceholder`][crate::classes::InstancePlaceholder].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InstancePlaceholder` enums](https://docs.godotengine.org/en/stable/classes/class_instanceplaceholder.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InstancePlaceholder.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`instance_placeholder`][crate::classes::instance_placeholder]: sidecar module with related enum/flag types\n* [`IInstancePlaceholder`][crate::classes::IInstancePlaceholder]: virtual methods\n\n\nSee also [Godot docs for `InstancePlaceholder`](https://docs.godotengine.org/en/stable/classes/class_instanceplaceholder.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InstancePlaceholder>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InstancePlaceholder {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InstancePlaceholder`][crate::classes::InstancePlaceholder].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InstancePlaceholder` methods](https://docs.godotengine.org/en/stable/classes/class_instanceplaceholder.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInstancePlaceholder: crate::obj::GodotClass < Base = InstancePlaceholder > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
    impl InstancePlaceholder {
        pub(crate) fn get_stored_values_full(&mut self, with_order: bool,) -> Dictionary {
            type CallSig = (Dictionary, bool);
            let args = (with_order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InstancePlaceholder", "get_stored_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_stored_values_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_stored_values(&mut self,) -> Dictionary {
            self.get_stored_values_ex() . done()
        }
        #[inline]
        pub fn get_stored_values_ex < 'a > (&'a mut self,) -> ExGetStoredValues < 'a > {
            ExGetStoredValues::new(self,)
        }
        pub(crate) fn create_instance_full(&mut self, replace: bool, custom_scene: ObjectArg < crate::classes::PackedScene >,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >, bool, ObjectArg < crate::classes::PackedScene >);
            let args = (replace, custom_scene,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InstancePlaceholder", "create_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_instance_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_instance(&mut self,) -> Option < Gd < crate::classes::Node > > {
            self.create_instance_ex() . done()
        }
        #[inline]
        pub fn create_instance_ex < 'a > (&'a mut self,) -> ExCreateInstance < 'a > {
            ExCreateInstance::new(self,)
        }
        pub fn get_instance_path(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "InstancePlaceholder", "get_instance_path", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for InstancePlaceholder {
        type Base = crate::classes::Node;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"InstancePlaceholder"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InstancePlaceholder {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for InstancePlaceholder {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InstancePlaceholder {
        
    }
    impl std::ops::Deref for InstancePlaceholder {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InstancePlaceholder {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`InstancePlaceholder`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_InstancePlaceholder {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::InstancePlaceholder > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`InstancePlaceholder::get_stored_values_ex`][super::InstancePlaceholder::get_stored_values_ex]."]
#[must_use]
pub struct ExGetStoredValues < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::InstancePlaceholder, with_order: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetStoredValues < 'a > {
    fn new(surround_object: &'a mut re_export::InstancePlaceholder,) -> Self {
        let with_order = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, with_order: with_order,
        }
    }
    #[inline]
    pub fn with_order(self, with_order: bool) -> Self {
        Self {
            with_order: with_order, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Dictionary {
        let Self {
            _phantom, surround_object, with_order,
        }
        = self;
        re_export::InstancePlaceholder::get_stored_values_full(surround_object, with_order,)
    }
}
#[doc = "Default-param extender for [`InstancePlaceholder::create_instance_ex`][super::InstancePlaceholder::create_instance_ex]."]
#[must_use]
pub struct ExCreateInstance < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::InstancePlaceholder, replace: bool, custom_scene: ObjectCow < crate::classes::PackedScene >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateInstance < 'a > {
    fn new(surround_object: &'a mut re_export::InstancePlaceholder,) -> Self {
        let replace = false;
        let custom_scene = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, replace: replace, custom_scene: custom_scene.consume_arg(),
        }
    }
    #[inline]
    pub fn replace(self, replace: bool) -> Self {
        Self {
            replace: replace, .. self
        }
    }
    #[inline]
    pub fn custom_scene(self, custom_scene: impl AsObjectArg < crate::classes::PackedScene >) -> Self {
        Self {
            custom_scene: custom_scene.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, replace, custom_scene,
        }
        = self;
        re_export::InstancePlaceholder::create_instance_full(surround_object, replace, custom_scene.cow_as_object_arg(),)
    }
}