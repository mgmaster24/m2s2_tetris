#![doc = "Sidecar module for class [`ClassDb`][crate::classes::ClassDb].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ClassDB` enums](https://docs.godotengine.org/en/stable/classes/class_classdb.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ClassDB.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`class_db`][crate::classes::class_db]: sidecar module with related enum/flag types\n* [`IClassDb`][crate::classes::IClassDb]: virtual methods\n\n\nSee also [Godot docs for `ClassDB`](https://docs.godotengine.org/en/stable/classes/class_classdb.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`ClassDb::singleton()`][ClassDb::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ClassDb {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ClassDb`][crate::classes::ClassDb].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ClassDB` methods](https://docs.godotengine.org/en/stable/classes/class_classdb.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IClassDb: crate::obj::GodotClass < Base = ClassDb > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ClassDb {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"ClassDB");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_class_list(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "get_class_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inheriters_from_class(&self, class: impl AsArg < StringName >,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, StringName >);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "get_inheriters_from_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_class(&self, class: impl AsArg < StringName >,) -> StringName {
            type CallSig < 'a0, > = (StringName, CowArg < 'a0, StringName >);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "get_parent_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_exists(&self, class: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_parent_class(&self, class: impl AsArg < StringName >, inherits: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (class.into_arg(), inherits.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "is_parent_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_instantiate(&self, class: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "can_instantiate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn instantiate(&self, class: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "instantiate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_has_signal(&self, class: impl AsArg < StringName >, signal: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (class.into_arg(), signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_has_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_get_signal(&self, class: impl AsArg < StringName >, signal: impl AsArg < StringName >,) -> Dictionary {
            type CallSig < 'a0, 'a1, > = (Dictionary, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (class.into_arg(), signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn class_get_signal_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> Array < Dictionary > {
            type CallSig < 'a0, > = (Array < Dictionary >, CowArg < 'a0, StringName >, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_signal_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_signal_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_signal_list(&self, class: impl AsArg < StringName >,) -> Array < Dictionary > {
            self.class_get_signal_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_signal_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetSignalList < 'a > {
            ExClassGetSignalList::new(self, class,)
        }
        pub(crate) fn class_get_property_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> Array < Dictionary > {
            type CallSig < 'a0, > = (Array < Dictionary >, CowArg < 'a0, StringName >, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_property_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_property_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_property_list(&self, class: impl AsArg < StringName >,) -> Array < Dictionary > {
            self.class_get_property_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_property_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetPropertyList < 'a > {
            ExClassGetPropertyList::new(self, class,)
        }
        pub fn class_get_property(&self, object: impl AsObjectArg < crate::classes::Object >, property: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >);
            let args = (object.as_object_arg(), property.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_set_property(&self, object: impl AsObjectArg < crate::classes::Object >, property: impl AsArg < StringName >, value: &Variant,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (object.as_object_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_set_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_get_property_default_value(&self, class: impl AsArg < StringName >, property: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, 'a1, > = (Variant, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (class.into_arg(), property.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_property_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn class_has_method_full(&self, class: CowArg < StringName >, method: CowArg < StringName >, no_inheritance: bool,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool);
            let args = (class, method, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_has_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_has_method_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_has_method(&self, class: impl AsArg < StringName >, method: impl AsArg < StringName >,) -> bool {
            self.class_has_method_ex(class, method,) . done()
        }
        #[inline]
        pub fn class_has_method_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, method: impl AsArg < StringName > + 'a,) -> ExClassHasMethod < 'a > {
            ExClassHasMethod::new(self, class, method,)
        }
        pub(crate) fn class_get_method_argument_count_full(&self, class: CowArg < StringName >, method: CowArg < StringName >, no_inheritance: bool,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool);
            let args = (class, method, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_method_argument_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_method_argument_count_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_method_argument_count(&self, class: impl AsArg < StringName >, method: impl AsArg < StringName >,) -> i32 {
            self.class_get_method_argument_count_ex(class, method,) . done()
        }
        #[inline]
        pub fn class_get_method_argument_count_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, method: impl AsArg < StringName > + 'a,) -> ExClassGetMethodArgumentCount < 'a > {
            ExClassGetMethodArgumentCount::new(self, class, method,)
        }
        pub(crate) fn class_get_method_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> Array < Dictionary > {
            type CallSig < 'a0, > = (Array < Dictionary >, CowArg < 'a0, StringName >, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_method_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_method_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_method_list(&self, class: impl AsArg < StringName >,) -> Array < Dictionary > {
            self.class_get_method_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_method_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetMethodList < 'a > {
            ExClassGetMethodList::new(self, class,)
        }
        pub(crate) fn class_get_integer_constant_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, StringName >, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_integer_constant_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_integer_constant_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_integer_constant_list(&self, class: impl AsArg < StringName >,) -> PackedStringArray {
            self.class_get_integer_constant_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_integer_constant_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetIntegerConstantList < 'a > {
            ExClassGetIntegerConstantList::new(self, class,)
        }
        pub fn class_has_integer_constant(&self, class: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (class.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_has_integer_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn class_get_integer_constant(&self, class: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> i64 {
            type CallSig < 'a0, 'a1, > = (i64, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (class.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_integer_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn class_has_enum_full(&self, class: CowArg < StringName >, name: CowArg < StringName >, no_inheritance: bool,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool);
            let args = (class, name, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_has_enum", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_has_enum_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_has_enum(&self, class: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> bool {
            self.class_has_enum_ex(class, name,) . done()
        }
        #[inline]
        pub fn class_has_enum_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, name: impl AsArg < StringName > + 'a,) -> ExClassHasEnum < 'a > {
            ExClassHasEnum::new(self, class, name,)
        }
        pub(crate) fn class_get_enum_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, StringName >, bool);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_enum_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_enum_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_enum_list(&self, class: impl AsArg < StringName >,) -> PackedStringArray {
            self.class_get_enum_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_enum_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetEnumList < 'a > {
            ExClassGetEnumList::new(self, class,)
        }
        pub(crate) fn class_get_enum_constants_full(&self, class: CowArg < StringName >, enum_: CowArg < StringName >, no_inheritance: bool,) -> PackedStringArray {
            type CallSig < 'a0, 'a1, > = (PackedStringArray, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool);
            let args = (class, enum_, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_enum_constants", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_enum_constants_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_enum_constants(&self, class: impl AsArg < StringName >, enum_: impl AsArg < StringName >,) -> PackedStringArray {
            self.class_get_enum_constants_ex(class, enum_,) . done()
        }
        #[inline]
        pub fn class_get_enum_constants_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, enum_: impl AsArg < StringName > + 'a,) -> ExClassGetEnumConstants < 'a > {
            ExClassGetEnumConstants::new(self, class, enum_,)
        }
        pub(crate) fn class_get_integer_constant_enum_full(&self, class: CowArg < StringName >, name: CowArg < StringName >, no_inheritance: bool,) -> StringName {
            type CallSig < 'a0, 'a1, > = (StringName, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool);
            let args = (class, name, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_integer_constant_enum", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_integer_constant_enum_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_integer_constant_enum(&self, class: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> StringName {
            self.class_get_integer_constant_enum_ex(class, name,) . done()
        }
        #[inline]
        pub fn class_get_integer_constant_enum_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, name: impl AsArg < StringName > + 'a,) -> ExClassGetIntegerConstantEnum < 'a > {
            ExClassGetIntegerConstantEnum::new(self, class, name,)
        }
        pub(crate) fn is_class_enum_bitfield_full(&self, class: CowArg < StringName >, enum_: CowArg < StringName >, no_inheritance: bool,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool);
            let args = (class, enum_, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "is_class_enum_bitfield", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_class_enum_bitfield_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_class_enum_bitfield(&self, class: impl AsArg < StringName >, enum_: impl AsArg < StringName >,) -> bool {
            self.is_class_enum_bitfield_ex(class, enum_,) . done()
        }
        #[inline]
        pub fn is_class_enum_bitfield_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, enum_: impl AsArg < StringName > + 'a,) -> ExIsClassEnumBitfield < 'a > {
            ExIsClassEnumBitfield::new(self, class, enum_,)
        }
        pub fn is_class_enabled(&self, class: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ClassDb", "is_class_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ClassDb {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ClassDB"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ClassDb {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ClassDb {
        
    }
    impl std::ops::Deref for ClassDb {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ClassDb {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ClassDb`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ClassDb {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ClassDb > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_signal_list_ex`][super::ClassDb::class_get_signal_list_ex]."]
#[must_use]
pub struct ExClassGetSignalList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetSignalList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_signal_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_property_list_ex`][super::ClassDb::class_get_property_list_ex]."]
#[must_use]
pub struct ExClassGetPropertyList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetPropertyList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_property_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_has_method_ex`][super::ClassDb::class_has_method_ex]."]
#[must_use]
pub struct ExClassHasMethod < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, method: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassHasMethod < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, method: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), method: method.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, class, method, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_has_method_full(surround_object, class, method, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_method_argument_count_ex`][super::ClassDb::class_get_method_argument_count_ex]."]
#[must_use]
pub struct ExClassGetMethodArgumentCount < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, method: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetMethodArgumentCount < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, method: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), method: method.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, class, method, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_method_argument_count_full(surround_object, class, method, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_method_list_ex`][super::ClassDb::class_get_method_list_ex]."]
#[must_use]
pub struct ExClassGetMethodList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetMethodList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_method_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_integer_constant_list_ex`][super::ClassDb::class_get_integer_constant_list_ex]."]
#[must_use]
pub struct ExClassGetIntegerConstantList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetIntegerConstantList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_integer_constant_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_has_enum_ex`][super::ClassDb::class_has_enum_ex]."]
#[must_use]
pub struct ExClassHasEnum < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, name: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassHasEnum < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, name: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), name: name.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, class, name, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_has_enum_full(surround_object, class, name, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_enum_list_ex`][super::ClassDb::class_get_enum_list_ex]."]
#[must_use]
pub struct ExClassGetEnumList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetEnumList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_enum_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_enum_constants_ex`][super::ClassDb::class_get_enum_constants_ex]."]
#[must_use]
pub struct ExClassGetEnumConstants < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, enum_: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetEnumConstants < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, enum_: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), enum_: enum_.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, class, enum_, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_enum_constants_full(surround_object, class, enum_, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_integer_constant_enum_ex`][super::ClassDb::class_get_integer_constant_enum_ex]."]
#[must_use]
pub struct ExClassGetIntegerConstantEnum < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, name: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetIntegerConstantEnum < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, name: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), name: name.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, class, name, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_integer_constant_enum_full(surround_object, class, name, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::is_class_enum_bitfield_ex`][super::ClassDb::is_class_enum_bitfield_ex]."]
#[must_use]
pub struct ExIsClassEnumBitfield < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, enum_: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsClassEnumBitfield < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, enum_: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), enum_: enum_.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, class, enum_, no_inheritance,
        }
        = self;
        re_export::ClassDb::is_class_enum_bitfield_full(surround_object, class, enum_, no_inheritance,)
    }
}