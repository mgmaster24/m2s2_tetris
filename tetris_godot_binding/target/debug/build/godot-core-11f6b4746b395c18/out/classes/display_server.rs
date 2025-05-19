#![doc = "Sidecar module for class [`DisplayServer`][crate::classes::DisplayServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DisplayServer` enums](https://docs.godotengine.org/en/stable/classes/class_displayserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DisplayServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`display_server`][crate::classes::display_server]: sidecar module with related enum/flag types\n* [`IDisplayServer`][crate::classes::IDisplayServer]: virtual methods\n\n\nSee also [Godot docs for `DisplayServer`](https://docs.godotengine.org/en/stable/classes/class_displayserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`DisplayServer::singleton()`][DisplayServer::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DisplayServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`DisplayServer`][crate::classes::DisplayServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `DisplayServer` methods](https://docs.godotengine.org/en/stable/classes/class_displayserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IDisplayServer: crate::obj::GodotClass < Base = DisplayServer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl DisplayServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"DisplayServer");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn has_feature(&self, feature: crate::classes::display_server::Feature,) -> bool {
            type CallSig = (bool, crate::classes::display_server::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "has_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn help_set_search_callbacks(&mut self, search_callback: &Callable, action_callback: &Callable,) {
            type CallSig < 'a0, 'a1, > = ((), RefArg < 'a0, Callable >, RefArg < 'a1, Callable >);
            let args = (RefArg::new(search_callback), RefArg::new(action_callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "help_set_search_callbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_popup_callbacks(&mut self, menu_root: impl AsArg < GString >, open_callback: &Callable, close_callback: &Callable,) {
            type CallSig < 'a0, 'a1, 'a2, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >);
            let args = (menu_root.into_arg(), RefArg::new(open_callback), RefArg::new(close_callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_popup_callbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn global_menu_add_submenu_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, submenu: CowArg < GString >, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, > = (i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, i32);
            let args = (menu_root, label, submenu, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_submenu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_submenu_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_submenu_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >, submenu: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_submenu_item_ex(menu_root, label, submenu,) . done()
        }
        #[inline]
        pub fn global_menu_add_submenu_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a, submenu: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddSubmenuItem < 'a > {
            ExGlobalMenuAddSubmenuItem::new(self, menu_root, label, submenu,)
        }
        pub(crate) fn global_menu_add_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddItem < 'a > {
            ExGlobalMenuAddItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_check_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_check_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_check_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_check_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddCheckItem < 'a > {
            ExGlobalMenuAddCheckItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_icon_item_full(&mut self, menu_root: CowArg < GString >, icon: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (i32, CowArg < 'a0, GString >, ObjectArg < crate::classes::Texture2D >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_icon_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_icon_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddIconItem < 'a > {
            ExGlobalMenuAddIconItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_icon_check_item_full(&mut self, menu_root: CowArg < GString >, icon: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (i32, CowArg < 'a0, GString >, ObjectArg < crate::classes::Texture2D >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_icon_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_icon_check_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_check_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_check_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddIconCheckItem < 'a > {
            ExGlobalMenuAddIconCheckItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_radio_check_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_radio_check_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_radio_check_item_ex(menu_root, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_radio_check_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddRadioCheckItem < 'a > {
            ExGlobalMenuAddRadioCheckItem::new(self, menu_root, label,)
        }
        pub(crate) fn global_menu_add_icon_radio_check_item_full(&mut self, menu_root: CowArg < GString >, icon: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (i32, CowArg < 'a0, GString >, ObjectArg < crate::classes::Texture2D >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_icon_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_icon_radio_check_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_radio_check_item_ex(menu_root, icon, label,) . done()
        }
        #[inline]
        pub fn global_menu_add_icon_radio_check_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddIconRadioCheckItem < 'a > {
            ExGlobalMenuAddIconRadioCheckItem::new(self, menu_root, icon, label,)
        }
        pub(crate) fn global_menu_add_multistate_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, max_states: i32, default_state: i32, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32, i32, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32);
            let args = (menu_root, label, max_states, default_state, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_multistate_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_multistate_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_multistate_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >, max_states: i32, default_state: i32,) -> i32 {
            self.global_menu_add_multistate_item_ex(menu_root, label, max_states, default_state,) . done()
        }
        #[inline]
        pub fn global_menu_add_multistate_item_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a, max_states: i32, default_state: i32,) -> ExGlobalMenuAddMultistateItem < 'a > {
            ExGlobalMenuAddMultistateItem::new(self, menu_root, label, max_states, default_state,)
        }
        pub(crate) fn global_menu_add_separator_full(&mut self, menu_root: CowArg < GString >, index: i32,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >, i32);
            let args = (menu_root, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::global_menu_add_separator_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn global_menu_add_separator(&mut self, menu_root: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_separator_ex(menu_root,) . done()
        }
        #[inline]
        pub fn global_menu_add_separator_ex < 'a > (&'a mut self, menu_root: impl AsArg < GString > + 'a,) -> ExGlobalMenuAddSeparator < 'a > {
            ExGlobalMenuAddSeparator::new(self, menu_root,)
        }
        pub fn global_menu_get_item_index_from_text(&self, menu_root: impl AsArg < GString >, text: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (menu_root.into_arg(), text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_index_from_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_index_from_tag(&self, menu_root: impl AsArg < GString >, tag: &Variant,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, CowArg < 'a0, GString >, RefArg < 'a1, Variant >);
            let args = (menu_root.into_arg(), RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_index_from_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_checked(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_checkable(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_radio_checkable(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_callback(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Callable {
            type CallSig < 'a0, > = (Callable, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_key_callback(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Callable {
            type CallSig < 'a0, > = (Callable, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_key_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_tag(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_text(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_submenu(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_accelerator(&self, menu_root: impl AsArg < GString >, idx: i32,) -> crate::global::Key {
            type CallSig < 'a0, > = (crate::global::Key, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_disabled(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_is_item_hidden(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_tooltip(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_state(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_max_states(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_max_states", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_icon(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Texture2D > >, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_indentation_level(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_indentation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_checked(&mut self, menu_root: impl AsArg < GString >, idx: i32, checked: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, bool);
            let args = (menu_root.into_arg(), idx, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_checkable(&mut self, menu_root: impl AsArg < GString >, idx: i32, checkable: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, bool);
            let args = (menu_root.into_arg(), idx, checkable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_radio_checkable(&mut self, menu_root: impl AsArg < GString >, idx: i32, checkable: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, bool);
            let args = (menu_root.into_arg(), idx, checkable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_callback(&mut self, menu_root: impl AsArg < GString >, idx: i32, callback: &Callable,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >);
            let args = (menu_root.into_arg(), idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_hover_callbacks(&mut self, menu_root: impl AsArg < GString >, idx: i32, callback: &Callable,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >);
            let args = (menu_root.into_arg(), idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_hover_callbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_key_callback(&mut self, menu_root: impl AsArg < GString >, idx: i32, key_callback: &Callable,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >);
            let args = (menu_root.into_arg(), idx, RefArg::new(key_callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_key_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_tag(&mut self, menu_root: impl AsArg < GString >, idx: i32, tag: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, i32, RefArg < 'a1, Variant >);
            let args = (menu_root.into_arg(), idx, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_text(&mut self, menu_root: impl AsArg < GString >, idx: i32, text: impl AsArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >);
            let args = (menu_root.into_arg(), idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_submenu(&mut self, menu_root: impl AsArg < GString >, idx: i32, submenu: impl AsArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >);
            let args = (menu_root.into_arg(), idx, submenu.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_accelerator(&mut self, menu_root: impl AsArg < GString >, idx: i32, keycode: crate::global::Key,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, crate::global::Key);
            let args = (menu_root.into_arg(), idx, keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_disabled(&mut self, menu_root: impl AsArg < GString >, idx: i32, disabled: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, bool);
            let args = (menu_root.into_arg(), idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_hidden(&mut self, menu_root: impl AsArg < GString >, idx: i32, hidden: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, bool);
            let args = (menu_root.into_arg(), idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_tooltip(&mut self, menu_root: impl AsArg < GString >, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >);
            let args = (menu_root.into_arg(), idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_state(&mut self, menu_root: impl AsArg < GString >, idx: i32, state: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, i32);
            let args = (menu_root.into_arg(), idx, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_max_states(&mut self, menu_root: impl AsArg < GString >, idx: i32, max_states: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, i32);
            let args = (menu_root.into_arg(), idx, max_states,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_max_states", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_icon(&mut self, menu_root: impl AsArg < GString >, idx: i32, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, ObjectArg < crate::classes::Texture2D >);
            let args = (menu_root.into_arg(), idx, icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_set_item_indentation_level(&mut self, menu_root: impl AsArg < GString >, idx: i32, level: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, i32);
            let args = (menu_root.into_arg(), idx, level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_indentation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_item_count(&self, menu_root: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >);
            let args = (menu_root.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_remove_item(&mut self, menu_root: impl AsArg < GString >, idx: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_clear(&mut self, menu_root: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (menu_root.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_menu_get_system_menu_roots(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_system_menu_roots", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_is_speaking(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_is_speaking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_is_paused(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_is_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_get_voices(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_get_voices", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_get_voices_for_language(&self, language: impl AsArg < GString >,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_get_voices_for_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn tts_speak_full(&mut self, text: CowArg < GString >, voice: CowArg < GString >, volume: i32, pitch: f32, rate: f32, utterance_id: i32, interrupt: bool,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32, f32, f32, i32, bool);
            let args = (text, voice, volume, pitch, rate, utterance_id, interrupt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_speak", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::tts_speak_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn tts_speak(&mut self, text: impl AsArg < GString >, voice: impl AsArg < GString >,) {
            self.tts_speak_ex(text, voice,) . done()
        }
        #[inline]
        pub fn tts_speak_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a, voice: impl AsArg < GString > + 'a,) -> ExTtsSpeak < 'a > {
            ExTtsSpeak::new(self, text, voice,)
        }
        pub fn tts_pause(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_resume(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_resume", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_stop(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tts_set_utterance_callback(&mut self, event: crate::classes::display_server::TtsUtteranceEvent, callable: &Callable,) {
            type CallSig < 'a0, > = ((), crate::classes::display_server::TtsUtteranceEvent, RefArg < 'a0, Callable >);
            let args = (event, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_set_utterance_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_dark_mode_supported(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "is_dark_mode_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_dark_mode(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "is_dark_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_accent_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_accent_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_base_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_system_theme_change_callback(&mut self, callable: &Callable,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "set_system_theme_change_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mouse_set_mode(&mut self, mouse_mode: crate::classes::display_server::MouseMode,) {
            type CallSig = ((), crate::classes::display_server::MouseMode);
            let args = (mouse_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mouse_get_mode(&self,) -> crate::classes::display_server::MouseMode {
            type CallSig = (crate::classes::display_server::MouseMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "warp_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mouse_get_position(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mouse_get_button_state(&self,) -> crate::global::MouseButtonMask {
            type CallSig = (crate::global::MouseButtonMask,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_button_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_set(&mut self, clipboard: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (clipboard.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_get(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_get_image(&self,) -> Option < Gd < crate::classes::Image > > {
            type CallSig = (Option < Gd < crate::classes::Image > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_has(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_has", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_has_image(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_has_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_set_primary(&mut self, clipboard_primary: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (clipboard_primary.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_set_primary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clipboard_get_primary(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get_primary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_display_cutouts(&self,) -> Array < Rect2 > {
            type CallSig = (Array < Rect2 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_display_cutouts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_display_safe_area(&self,) -> Rect2i {
            type CallSig = (Rect2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_display_safe_area", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_screen_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_primary_screen(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_primary_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keyboard_focus_screen(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_keyboard_focus_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_from_rect(&self, rect: Rect2,) -> i32 {
            type CallSig = (i32, Rect2);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_screen_from_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn screen_get_position_full(&self, screen: i32,) -> Vector2i {
            type CallSig = (Vector2i, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_position(&self,) -> Vector2i {
            self.screen_get_position_ex() . done()
        }
        #[inline]
        pub fn screen_get_position_ex < 'a > (&'a self,) -> ExScreenGetPosition < 'a > {
            ExScreenGetPosition::new(self,)
        }
        pub(crate) fn screen_get_size_full(&self, screen: i32,) -> Vector2i {
            type CallSig = (Vector2i, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_size(&self,) -> Vector2i {
            self.screen_get_size_ex() . done()
        }
        #[inline]
        pub fn screen_get_size_ex < 'a > (&'a self,) -> ExScreenGetSize < 'a > {
            ExScreenGetSize::new(self,)
        }
        pub(crate) fn screen_get_usable_rect_full(&self, screen: i32,) -> Rect2i {
            type CallSig = (Rect2i, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_usable_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_usable_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_usable_rect(&self,) -> Rect2i {
            self.screen_get_usable_rect_ex() . done()
        }
        #[inline]
        pub fn screen_get_usable_rect_ex < 'a > (&'a self,) -> ExScreenGetUsableRect < 'a > {
            ExScreenGetUsableRect::new(self,)
        }
        pub(crate) fn screen_get_dpi_full(&self, screen: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_dpi", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_dpi_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_dpi(&self,) -> i32 {
            self.screen_get_dpi_ex() . done()
        }
        #[inline]
        pub fn screen_get_dpi_ex < 'a > (&'a self,) -> ExScreenGetDpi < 'a > {
            ExScreenGetDpi::new(self,)
        }
        pub(crate) fn screen_get_scale_full(&self, screen: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_scale_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_scale(&self,) -> f32 {
            self.screen_get_scale_ex() . done()
        }
        #[inline]
        pub fn screen_get_scale_ex < 'a > (&'a self,) -> ExScreenGetScale < 'a > {
            ExScreenGetScale::new(self,)
        }
        pub fn is_touchscreen_available(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "is_touchscreen_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn screen_get_max_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_max_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn screen_get_refresh_rate_full(&self, screen: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_refresh_rate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_refresh_rate(&self,) -> f32 {
            self.screen_get_refresh_rate_ex() . done()
        }
        #[inline]
        pub fn screen_get_refresh_rate_ex < 'a > (&'a self,) -> ExScreenGetRefreshRate < 'a > {
            ExScreenGetRefreshRate::new(self,)
        }
        pub fn screen_get_pixel(&self, position: Vector2i,) -> Color {
            type CallSig = (Color, Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn screen_get_image_full(&self, screen: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallSig = (Option < Gd < crate::classes::Image > >, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_image(&self,) -> Option < Gd < crate::classes::Image > > {
            self.screen_get_image_ex() . done()
        }
        #[inline]
        pub fn screen_get_image_ex < 'a > (&'a self,) -> ExScreenGetImage < 'a > {
            ExScreenGetImage::new(self,)
        }
        pub(crate) fn screen_set_orientation_full(&mut self, orientation: crate::classes::display_server::ScreenOrientation, screen: i32,) {
            type CallSig = ((), crate::classes::display_server::ScreenOrientation, i32);
            let args = (orientation, screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_set_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_set_orientation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_set_orientation(&mut self, orientation: crate::classes::display_server::ScreenOrientation,) {
            self.screen_set_orientation_ex(orientation,) . done()
        }
        #[inline]
        pub fn screen_set_orientation_ex < 'a > (&'a mut self, orientation: crate::classes::display_server::ScreenOrientation,) -> ExScreenSetOrientation < 'a > {
            ExScreenSetOrientation::new(self, orientation,)
        }
        pub(crate) fn screen_get_orientation_full(&self, screen: i32,) -> crate::classes::display_server::ScreenOrientation {
            type CallSig = (crate::classes::display_server::ScreenOrientation, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_orientation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_orientation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_orientation(&self,) -> crate::classes::display_server::ScreenOrientation {
            self.screen_get_orientation_ex() . done()
        }
        #[inline]
        pub fn screen_get_orientation_ex < 'a > (&'a self,) -> ExScreenGetOrientation < 'a > {
            ExScreenGetOrientation::new(self,)
        }
        pub fn screen_set_keep_on(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_set_keep_on", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn screen_is_kept_on(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_is_kept_on", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_window_list(&self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_window_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_window_at_screen_position(&self, position: Vector2i,) -> i32 {
            type CallSig = (i32, Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_window_at_screen_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn window_get_native_handle_full(&self, handle_type: crate::classes::display_server::HandleType, window_id: i32,) -> i64 {
            type CallSig = (i64, crate::classes::display_server::HandleType, i32);
            let args = (handle_type, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_native_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_native_handle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_native_handle(&self, handle_type: crate::classes::display_server::HandleType,) -> i64 {
            self.window_get_native_handle_ex(handle_type,) . done()
        }
        #[inline]
        pub fn window_get_native_handle_ex < 'a > (&'a self, handle_type: crate::classes::display_server::HandleType,) -> ExWindowGetNativeHandle < 'a > {
            ExWindowGetNativeHandle::new(self, handle_type,)
        }
        pub fn window_get_active_popup(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_active_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn window_set_popup_safe_rect(&mut self, window: i32, rect: Rect2i,) {
            type CallSig = ((), i32, Rect2i);
            let args = (window, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_popup_safe_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn window_get_popup_safe_rect(&self, window: i32,) -> Rect2i {
            type CallSig = (Rect2i, i32);
            let args = (window,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_popup_safe_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn window_set_title_full(&mut self, title: CowArg < GString >, window_id: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (title, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_title_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_title(&mut self, title: impl AsArg < GString >,) {
            self.window_set_title_ex(title,) . done()
        }
        #[inline]
        pub fn window_set_title_ex < 'a > (&'a mut self, title: impl AsArg < GString > + 'a,) -> ExWindowSetTitle < 'a > {
            ExWindowSetTitle::new(self, title,)
        }
        pub(crate) fn window_get_title_size_full(&self, title: CowArg < GString >, window_id: i32,) -> Vector2i {
            type CallSig < 'a0, > = (Vector2i, CowArg < 'a0, GString >, i32);
            let args = (title, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_title_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_title_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_title_size(&self, title: impl AsArg < GString >,) -> Vector2i {
            self.window_get_title_size_ex(title,) . done()
        }
        #[inline]
        pub fn window_get_title_size_ex < 'a > (&'a self, title: impl AsArg < GString > + 'a,) -> ExWindowGetTitleSize < 'a > {
            ExWindowGetTitleSize::new(self, title,)
        }
        pub(crate) fn window_set_mouse_passthrough_full(&mut self, region: RefArg < PackedVector2Array >, window_id: i32,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedVector2Array >, i32);
            let args = (region, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_mouse_passthrough", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_mouse_passthrough_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_mouse_passthrough(&mut self, region: &PackedVector2Array,) {
            self.window_set_mouse_passthrough_ex(region,) . done()
        }
        #[inline]
        pub fn window_set_mouse_passthrough_ex < 'a > (&'a mut self, region: &'a PackedVector2Array,) -> ExWindowSetMousePassthrough < 'a > {
            ExWindowSetMousePassthrough::new(self, region,)
        }
        pub(crate) fn window_get_current_screen_full(&self, window_id: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_current_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_current_screen_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_current_screen(&self,) -> i32 {
            self.window_get_current_screen_ex() . done()
        }
        #[inline]
        pub fn window_get_current_screen_ex < 'a > (&'a self,) -> ExWindowGetCurrentScreen < 'a > {
            ExWindowGetCurrentScreen::new(self,)
        }
        pub(crate) fn window_set_current_screen_full(&mut self, screen: i32, window_id: i32,) {
            type CallSig = ((), i32, i32);
            let args = (screen, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_current_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_current_screen_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_current_screen(&mut self, screen: i32,) {
            self.window_set_current_screen_ex(screen,) . done()
        }
        #[inline]
        pub fn window_set_current_screen_ex < 'a > (&'a mut self, screen: i32,) -> ExWindowSetCurrentScreen < 'a > {
            ExWindowSetCurrentScreen::new(self, screen,)
        }
        pub(crate) fn window_get_position_full(&self, window_id: i32,) -> Vector2i {
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_position(&self,) -> Vector2i {
            self.window_get_position_ex() . done()
        }
        #[inline]
        pub fn window_get_position_ex < 'a > (&'a self,) -> ExWindowGetPosition < 'a > {
            ExWindowGetPosition::new(self,)
        }
        pub(crate) fn window_get_position_with_decorations_full(&self, window_id: i32,) -> Vector2i {
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_position_with_decorations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_position_with_decorations_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_position_with_decorations(&self,) -> Vector2i {
            self.window_get_position_with_decorations_ex() . done()
        }
        #[inline]
        pub fn window_get_position_with_decorations_ex < 'a > (&'a self,) -> ExWindowGetPositionWithDecorations < 'a > {
            ExWindowGetPositionWithDecorations::new(self,)
        }
        pub(crate) fn window_set_position_full(&mut self, position: Vector2i, window_id: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (position, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_position(&mut self, position: Vector2i,) {
            self.window_set_position_ex(position,) . done()
        }
        #[inline]
        pub fn window_set_position_ex < 'a > (&'a mut self, position: Vector2i,) -> ExWindowSetPosition < 'a > {
            ExWindowSetPosition::new(self, position,)
        }
        pub(crate) fn window_get_size_full(&self, window_id: i32,) -> Vector2i {
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_size(&self,) -> Vector2i {
            self.window_get_size_ex() . done()
        }
        #[inline]
        pub fn window_get_size_ex < 'a > (&'a self,) -> ExWindowGetSize < 'a > {
            ExWindowGetSize::new(self,)
        }
        pub(crate) fn window_set_size_full(&mut self, size: Vector2i, window_id: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (size, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_size(&mut self, size: Vector2i,) {
            self.window_set_size_ex(size,) . done()
        }
        #[inline]
        pub fn window_set_size_ex < 'a > (&'a mut self, size: Vector2i,) -> ExWindowSetSize < 'a > {
            ExWindowSetSize::new(self, size,)
        }
        pub(crate) fn window_set_rect_changed_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_rect_changed_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_rect_changed_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_rect_changed_callback(&mut self, callback: &Callable,) {
            self.window_set_rect_changed_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_rect_changed_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetRectChangedCallback < 'a > {
            ExWindowSetRectChangedCallback::new(self, callback,)
        }
        pub(crate) fn window_set_window_event_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_window_event_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_window_event_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_window_event_callback(&mut self, callback: &Callable,) {
            self.window_set_window_event_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_window_event_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetWindowEventCallback < 'a > {
            ExWindowSetWindowEventCallback::new(self, callback,)
        }
        pub(crate) fn window_set_input_event_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_input_event_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_input_event_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_input_event_callback(&mut self, callback: &Callable,) {
            self.window_set_input_event_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_input_event_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetInputEventCallback < 'a > {
            ExWindowSetInputEventCallback::new(self, callback,)
        }
        pub(crate) fn window_set_input_text_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_input_text_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_input_text_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_input_text_callback(&mut self, callback: &Callable,) {
            self.window_set_input_text_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_input_text_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetInputTextCallback < 'a > {
            ExWindowSetInputTextCallback::new(self, callback,)
        }
        pub(crate) fn window_set_drop_files_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >, i32);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_drop_files_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_drop_files_callback_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_drop_files_callback(&mut self, callback: &Callable,) {
            self.window_set_drop_files_callback_ex(callback,) . done()
        }
        #[inline]
        pub fn window_set_drop_files_callback_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExWindowSetDropFilesCallback < 'a > {
            ExWindowSetDropFilesCallback::new(self, callback,)
        }
        pub(crate) fn window_get_attached_instance_id_full(&self, window_id: i32,) -> u64 {
            type CallSig = (u64, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_attached_instance_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_attached_instance_id_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_attached_instance_id(&self,) -> u64 {
            self.window_get_attached_instance_id_ex() . done()
        }
        #[inline]
        pub fn window_get_attached_instance_id_ex < 'a > (&'a self,) -> ExWindowGetAttachedInstanceId < 'a > {
            ExWindowGetAttachedInstanceId::new(self,)
        }
        pub(crate) fn window_get_max_size_full(&self, window_id: i32,) -> Vector2i {
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_max_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_max_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_max_size(&self,) -> Vector2i {
            self.window_get_max_size_ex() . done()
        }
        #[inline]
        pub fn window_get_max_size_ex < 'a > (&'a self,) -> ExWindowGetMaxSize < 'a > {
            ExWindowGetMaxSize::new(self,)
        }
        pub(crate) fn window_set_max_size_full(&mut self, max_size: Vector2i, window_id: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (max_size, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_max_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_max_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_max_size(&mut self, max_size: Vector2i,) {
            self.window_set_max_size_ex(max_size,) . done()
        }
        #[inline]
        pub fn window_set_max_size_ex < 'a > (&'a mut self, max_size: Vector2i,) -> ExWindowSetMaxSize < 'a > {
            ExWindowSetMaxSize::new(self, max_size,)
        }
        pub(crate) fn window_get_min_size_full(&self, window_id: i32,) -> Vector2i {
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_min_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_min_size(&self,) -> Vector2i {
            self.window_get_min_size_ex() . done()
        }
        #[inline]
        pub fn window_get_min_size_ex < 'a > (&'a self,) -> ExWindowGetMinSize < 'a > {
            ExWindowGetMinSize::new(self,)
        }
        pub(crate) fn window_set_min_size_full(&mut self, min_size: Vector2i, window_id: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (min_size, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_min_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_min_size(&mut self, min_size: Vector2i,) {
            self.window_set_min_size_ex(min_size,) . done()
        }
        #[inline]
        pub fn window_set_min_size_ex < 'a > (&'a mut self, min_size: Vector2i,) -> ExWindowSetMinSize < 'a > {
            ExWindowSetMinSize::new(self, min_size,)
        }
        pub(crate) fn window_get_size_with_decorations_full(&self, window_id: i32,) -> Vector2i {
            type CallSig = (Vector2i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_size_with_decorations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_size_with_decorations_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_size_with_decorations(&self,) -> Vector2i {
            self.window_get_size_with_decorations_ex() . done()
        }
        #[inline]
        pub fn window_get_size_with_decorations_ex < 'a > (&'a self,) -> ExWindowGetSizeWithDecorations < 'a > {
            ExWindowGetSizeWithDecorations::new(self,)
        }
        pub(crate) fn window_get_mode_full(&self, window_id: i32,) -> crate::classes::display_server::WindowMode {
            type CallSig = (crate::classes::display_server::WindowMode, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_mode(&self,) -> crate::classes::display_server::WindowMode {
            self.window_get_mode_ex() . done()
        }
        #[inline]
        pub fn window_get_mode_ex < 'a > (&'a self,) -> ExWindowGetMode < 'a > {
            ExWindowGetMode::new(self,)
        }
        pub(crate) fn window_set_mode_full(&mut self, mode: crate::classes::display_server::WindowMode, window_id: i32,) {
            type CallSig = ((), crate::classes::display_server::WindowMode, i32);
            let args = (mode, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_mode(&mut self, mode: crate::classes::display_server::WindowMode,) {
            self.window_set_mode_ex(mode,) . done()
        }
        #[inline]
        pub fn window_set_mode_ex < 'a > (&'a mut self, mode: crate::classes::display_server::WindowMode,) -> ExWindowSetMode < 'a > {
            ExWindowSetMode::new(self, mode,)
        }
        pub(crate) fn window_set_flag_full(&mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool, window_id: i32,) {
            type CallSig = ((), crate::classes::display_server::WindowFlags, bool, i32);
            let args = (flag, enabled, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_flag_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_flag(&mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool,) {
            self.window_set_flag_ex(flag, enabled,) . done()
        }
        #[inline]
        pub fn window_set_flag_ex < 'a > (&'a mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool,) -> ExWindowSetFlag < 'a > {
            ExWindowSetFlag::new(self, flag, enabled,)
        }
        pub(crate) fn window_get_flag_full(&self, flag: crate::classes::display_server::WindowFlags, window_id: i32,) -> bool {
            type CallSig = (bool, crate::classes::display_server::WindowFlags, i32);
            let args = (flag, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_flag_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_flag(&self, flag: crate::classes::display_server::WindowFlags,) -> bool {
            self.window_get_flag_ex(flag,) . done()
        }
        #[inline]
        pub fn window_get_flag_ex < 'a > (&'a self, flag: crate::classes::display_server::WindowFlags,) -> ExWindowGetFlag < 'a > {
            ExWindowGetFlag::new(self, flag,)
        }
        pub(crate) fn window_set_window_buttons_offset_full(&mut self, offset: Vector2i, window_id: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (offset, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_window_buttons_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_window_buttons_offset_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_window_buttons_offset(&mut self, offset: Vector2i,) {
            self.window_set_window_buttons_offset_ex(offset,) . done()
        }
        #[inline]
        pub fn window_set_window_buttons_offset_ex < 'a > (&'a mut self, offset: Vector2i,) -> ExWindowSetWindowButtonsOffset < 'a > {
            ExWindowSetWindowButtonsOffset::new(self, offset,)
        }
        pub(crate) fn window_get_safe_title_margins_full(&self, window_id: i32,) -> Vector3i {
            type CallSig = (Vector3i, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_safe_title_margins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_safe_title_margins_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_safe_title_margins(&self,) -> Vector3i {
            self.window_get_safe_title_margins_ex() . done()
        }
        #[inline]
        pub fn window_get_safe_title_margins_ex < 'a > (&'a self,) -> ExWindowGetSafeTitleMargins < 'a > {
            ExWindowGetSafeTitleMargins::new(self,)
        }
        pub(crate) fn window_request_attention_full(&mut self, window_id: i32,) {
            type CallSig = ((), i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_request_attention", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_request_attention_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_request_attention(&mut self,) {
            self.window_request_attention_ex() . done()
        }
        #[inline]
        pub fn window_request_attention_ex < 'a > (&'a mut self,) -> ExWindowRequestAttention < 'a > {
            ExWindowRequestAttention::new(self,)
        }
        pub(crate) fn window_move_to_foreground_full(&mut self, window_id: i32,) {
            type CallSig = ((), i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_move_to_foreground", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_move_to_foreground_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_move_to_foreground(&mut self,) {
            self.window_move_to_foreground_ex() . done()
        }
        #[inline]
        pub fn window_move_to_foreground_ex < 'a > (&'a mut self,) -> ExWindowMoveToForeground < 'a > {
            ExWindowMoveToForeground::new(self,)
        }
        pub(crate) fn window_is_focused_full(&self, window_id: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_is_focused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_is_focused_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_is_focused(&self,) -> bool {
            self.window_is_focused_ex() . done()
        }
        #[inline]
        pub fn window_is_focused_ex < 'a > (&'a self,) -> ExWindowIsFocused < 'a > {
            ExWindowIsFocused::new(self,)
        }
        pub(crate) fn window_can_draw_full(&self, window_id: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_can_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_can_draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_can_draw(&self,) -> bool {
            self.window_can_draw_ex() . done()
        }
        #[inline]
        pub fn window_can_draw_ex < 'a > (&'a self,) -> ExWindowCanDraw < 'a > {
            ExWindowCanDraw::new(self,)
        }
        pub fn window_set_transient(&mut self, window_id: i32, parent_window_id: i32,) {
            type CallSig = ((), i32, i32);
            let args = (window_id, parent_window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_transient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn window_set_exclusive(&mut self, window_id: i32, exclusive: bool,) {
            type CallSig = ((), i32, bool);
            let args = (window_id, exclusive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_exclusive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn window_set_ime_active_full(&mut self, active: bool, window_id: i32,) {
            type CallSig = ((), bool, i32);
            let args = (active, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_ime_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_ime_active_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_ime_active(&mut self, active: bool,) {
            self.window_set_ime_active_ex(active,) . done()
        }
        #[inline]
        pub fn window_set_ime_active_ex < 'a > (&'a mut self, active: bool,) -> ExWindowSetImeActive < 'a > {
            ExWindowSetImeActive::new(self, active,)
        }
        pub(crate) fn window_set_ime_position_full(&mut self, position: Vector2i, window_id: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (position, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_ime_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_ime_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_ime_position(&mut self, position: Vector2i,) {
            self.window_set_ime_position_ex(position,) . done()
        }
        #[inline]
        pub fn window_set_ime_position_ex < 'a > (&'a mut self, position: Vector2i,) -> ExWindowSetImePosition < 'a > {
            ExWindowSetImePosition::new(self, position,)
        }
        pub(crate) fn window_set_vsync_mode_full(&mut self, vsync_mode: crate::classes::display_server::VSyncMode, window_id: i32,) {
            type CallSig = ((), crate::classes::display_server::VSyncMode, i32);
            let args = (vsync_mode, window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_vsync_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_set_vsync_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_set_vsync_mode(&mut self, vsync_mode: crate::classes::display_server::VSyncMode,) {
            self.window_set_vsync_mode_ex(vsync_mode,) . done()
        }
        #[inline]
        pub fn window_set_vsync_mode_ex < 'a > (&'a mut self, vsync_mode: crate::classes::display_server::VSyncMode,) -> ExWindowSetVSyncMode < 'a > {
            ExWindowSetVSyncMode::new(self, vsync_mode,)
        }
        pub(crate) fn window_get_vsync_mode_full(&self, window_id: i32,) -> crate::classes::display_server::VSyncMode {
            type CallSig = (crate::classes::display_server::VSyncMode, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_vsync_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_get_vsync_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_get_vsync_mode(&self,) -> crate::classes::display_server::VSyncMode {
            self.window_get_vsync_mode_ex() . done()
        }
        #[inline]
        pub fn window_get_vsync_mode_ex < 'a > (&'a self,) -> ExWindowGetVSyncMode < 'a > {
            ExWindowGetVSyncMode::new(self,)
        }
        pub(crate) fn window_is_maximize_allowed_full(&self, window_id: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_is_maximize_allowed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::window_is_maximize_allowed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn window_is_maximize_allowed(&self,) -> bool {
            self.window_is_maximize_allowed_ex() . done()
        }
        #[inline]
        pub fn window_is_maximize_allowed_ex < 'a > (&'a self,) -> ExWindowIsMaximizeAllowed < 'a > {
            ExWindowIsMaximizeAllowed::new(self,)
        }
        pub fn window_maximize_on_title_dbl_click(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_maximize_on_title_dbl_click", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn window_minimize_on_title_dbl_click(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "window_minimize_on_title_dbl_click", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ime_get_selection(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "ime_get_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ime_get_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "ime_get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn virtual_keyboard_show_full(&mut self, existing_text: CowArg < GString >, position: Rect2, type_: crate::classes::display_server::VirtualKeyboardType, max_length: i32, cursor_start: i32, cursor_end: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, Rect2, crate::classes::display_server::VirtualKeyboardType, i32, i32, i32);
            let args = (existing_text, position, type_, max_length, cursor_start, cursor_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::virtual_keyboard_show_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn virtual_keyboard_show(&mut self, existing_text: impl AsArg < GString >,) {
            self.virtual_keyboard_show_ex(existing_text,) . done()
        }
        #[inline]
        pub fn virtual_keyboard_show_ex < 'a > (&'a mut self, existing_text: impl AsArg < GString > + 'a,) -> ExVirtualKeyboardShow < 'a > {
            ExVirtualKeyboardShow::new(self, existing_text,)
        }
        pub fn virtual_keyboard_hide(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_hide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn virtual_keyboard_get_height(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cursor_set_shape(&mut self, shape: crate::classes::display_server::CursorShape,) {
            type CallSig = ((), crate::classes::display_server::CursorShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cursor_get_shape(&self,) -> crate::classes::display_server::CursorShape {
            type CallSig = (crate::classes::display_server::CursorShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn cursor_set_custom_image_full(&mut self, cursor: ObjectArg < crate::classes::Resource >, shape: crate::classes::display_server::CursorShape, hotspot: Vector2,) {
            type CallSig = ((), ObjectArg < crate::classes::Resource >, crate::classes::display_server::CursorShape, Vector2);
            let args = (cursor, shape, hotspot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_set_custom_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::cursor_set_custom_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn cursor_set_custom_image(&mut self, cursor: impl AsObjectArg < crate::classes::Resource >,) {
            self.cursor_set_custom_image_ex(cursor,) . done()
        }
        #[inline]
        pub fn cursor_set_custom_image_ex < 'a > (&'a mut self, cursor: impl AsObjectArg < crate::classes::Resource >,) -> ExCursorSetCustomImage < 'a > {
            ExCursorSetCustomImage::new(self, cursor,)
        }
        pub fn get_swap_cancel_ok(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "get_swap_cancel_ok", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn enable_for_stealing_focus(&mut self, process_id: i64,) {
            type CallSig = ((), i64);
            let args = (process_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "enable_for_stealing_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dialog_show(&mut self, title: impl AsArg < GString >, description: impl AsArg < GString >, buttons: &PackedStringArray, callback: &Callable,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, RefArg < 'a3, Callable >);
            let args = (title.into_arg(), description.into_arg(), RefArg::new(buttons), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "dialog_show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dialog_input_text(&mut self, title: impl AsArg < GString >, description: impl AsArg < GString >, existing_text: impl AsArg < GString >, callback: &Callable,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >);
            let args = (title.into_arg(), description.into_arg(), existing_text.into_arg(), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "dialog_input_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn file_dialog_show(&mut self, title: impl AsArg < GString >, current_directory: impl AsArg < GString >, filename: impl AsArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &PackedStringArray, callback: &Callable,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, bool, crate::classes::display_server::FileDialogMode, RefArg < 'a3, PackedStringArray >, RefArg < 'a4, Callable >);
            let args = (title.into_arg(), current_directory.into_arg(), filename.into_arg(), show_hidden, mode, RefArg::new(filters), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "file_dialog_show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn file_dialog_with_options_show(&mut self, title: impl AsArg < GString >, current_directory: impl AsArg < GString >, root: impl AsArg < GString >, filename: impl AsArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &PackedStringArray, options: &Array < Dictionary >, callback: &Callable,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, 'a6, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, CowArg < 'a3, GString >, bool, crate::classes::display_server::FileDialogMode, RefArg < 'a4, PackedStringArray >, RefArg < 'a5, Array < Dictionary > >, RefArg < 'a6, Callable >);
            let args = (title.into_arg(), current_directory.into_arg(), root.into_arg(), filename.into_arg(), show_hidden, mode, RefArg::new(filters), RefArg::new(options), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "file_dialog_with_options_show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_layout_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_current_layout(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_current_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_set_current_layout(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_set_current_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_layout_language(&self, index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_layout_name(&self, index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_keycode_from_physical(&self, keycode: crate::global::Key,) -> crate::global::Key {
            type CallSig = (crate::global::Key, crate::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_keycode_from_physical", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn keyboard_get_label_from_physical(&self, keycode: crate::global::Key,) -> crate::global::Key {
            type CallSig = (crate::global::Key, crate::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_label_from_physical", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn process_events(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "process_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_process_and_drop_events(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "force_process_and_drop_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_native_icon(&mut self, filename: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "set_native_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon(&mut self, image: impl AsObjectArg < crate::classes::Image >,) {
            type CallSig = ((), ObjectArg < crate::classes::Image >);
            let args = (image.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "set_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_status_indicator(&mut self, icon: impl AsObjectArg < crate::classes::Texture2D >, tooltip: impl AsArg < GString >, callback: &Callable,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, ObjectArg < crate::classes::Texture2D >, CowArg < 'a0, GString >, RefArg < 'a1, Callable >);
            let args = (icon.as_object_arg(), tooltip.into_arg(), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "create_status_indicator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn status_indicator_set_icon(&mut self, id: i32, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Texture2D >);
            let args = (id, icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn status_indicator_set_tooltip(&mut self, id: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (id, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn status_indicator_set_menu(&mut self, id: i32, menu_rid: Rid,) {
            type CallSig = ((), i32, Rid);
            let args = (id, menu_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn status_indicator_set_callback(&mut self, id: i32, callback: &Callable,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Callable >);
            let args = (id, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn status_indicator_get_rect(&self, id: i32,) -> Rect2 {
            type CallSig = (Rect2, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_get_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn delete_status_indicator(&mut self, id: i32,) {
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "delete_status_indicator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tablet_get_driver_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_driver_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tablet_get_driver_name(&self, idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_driver_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tablet_get_current_driver(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_current_driver", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn tablet_set_current_driver(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_set_current_driver", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_window_transparency_available(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "is_window_transparency_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_additional_output(&mut self, object: impl AsObjectArg < crate::classes::Object >,) {
            type CallSig = ((), ObjectArg < crate::classes::Object >);
            let args = (object.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "register_additional_output", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_additional_output(&mut self, object: impl AsObjectArg < crate::classes::Object >,) {
            type CallSig = ((), ObjectArg < crate::classes::Object >);
            let args = (object.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "unregister_additional_output", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_additional_outputs(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "DisplayServer", "has_additional_outputs", self.object_ptr, self.__checked_id(), args,)
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
        pub const SCREEN_WITH_MOUSE_FOCUS: i32 = - 4i32;
        pub const SCREEN_WITH_KEYBOARD_FOCUS: i32 = - 3i32;
        pub const SCREEN_PRIMARY: i32 = - 2i32;
        pub const SCREEN_OF_MAIN_WINDOW: i32 = - 1i32;
        pub const MAIN_WINDOW_ID: i32 = 0i32;
        pub const INVALID_WINDOW_ID: i32 = - 1i32;
        pub const INVALID_INDICATOR_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for DisplayServer {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"DisplayServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for DisplayServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DisplayServer {
        
    }
    impl std::ops::Deref for DisplayServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DisplayServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`DisplayServer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_DisplayServer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::DisplayServer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_submenu_item_ex`][super::DisplayServer::global_menu_add_submenu_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddSubmenuItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, submenu: CowArg < 'a, GString >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddSubmenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a, submenu: impl AsArg < GString > + 'a,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), submenu: submenu.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, submenu, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_submenu_item_full(surround_object, menu_root, label, submenu, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_item_ex`][super::DisplayServer::global_menu_add_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_check_item_ex`][super::DisplayServer::global_menu_add_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_check_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_item_ex`][super::DisplayServer::global_menu_add_icon_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, icon: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.consume_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_item_full(surround_object, menu_root, icon.cow_as_object_arg(), label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_check_item_ex`][super::DisplayServer::global_menu_add_icon_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, icon: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.consume_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_check_item_full(surround_object, menu_root, icon.cow_as_object_arg(), label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_radio_check_item_ex`][super::DisplayServer::global_menu_add_radio_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_radio_check_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_radio_check_item_ex`][super::DisplayServer::global_menu_add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, icon: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddIconRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.consume_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_radio_check_item_full(surround_object, menu_root, icon.cow_as_object_arg(), label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_multistate_item_ex`][super::DisplayServer::global_menu_add_multistate_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddMultistateItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, label: CowArg < 'a, GString >, max_states: i32, default_state: i32, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddMultistateItem < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a, label: impl AsArg < GString > + 'a, max_states: i32, default_state: i32,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), max_states: max_states, default_state: default_state, callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'a Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'a Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, max_states, default_state, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_multistate_item_full(surround_object, menu_root, label, max_states, default_state, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_separator_ex`][super::DisplayServer::global_menu_add_separator_ex]."]
#[must_use]
pub struct ExGlobalMenuAddSeparator < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, menu_root: CowArg < 'a, GString >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGlobalMenuAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'a,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_separator_full(surround_object, menu_root, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::tts_speak_ex`][super::DisplayServer::tts_speak_ex]."]
#[must_use]
pub struct ExTtsSpeak < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, text: CowArg < 'a, GString >, voice: CowArg < 'a, GString >, volume: i32, pitch: f32, rate: f32, utterance_id: i32, interrupt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTtsSpeak < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, text: impl AsArg < GString > + 'a, voice: impl AsArg < GString > + 'a,) -> Self {
        let volume = 50i32;
        let pitch = 1f32;
        let rate = 1f32;
        let utterance_id = 0i32;
        let interrupt = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), voice: voice.into_arg(), volume: volume, pitch: pitch, rate: rate, utterance_id: utterance_id, interrupt: interrupt,
        }
    }
    #[inline]
    pub fn volume(self, volume: i32) -> Self {
        Self {
            volume: volume, .. self
        }
    }
    #[inline]
    pub fn pitch(self, pitch: f32) -> Self {
        Self {
            pitch: pitch, .. self
        }
    }
    #[inline]
    pub fn rate(self, rate: f32) -> Self {
        Self {
            rate: rate, .. self
        }
    }
    #[inline]
    pub fn utterance_id(self, utterance_id: i32) -> Self {
        Self {
            utterance_id: utterance_id, .. self
        }
    }
    #[inline]
    pub fn interrupt(self, interrupt: bool) -> Self {
        Self {
            interrupt: interrupt, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, voice, volume, pitch, rate, utterance_id, interrupt,
        }
        = self;
        re_export::DisplayServer::tts_speak_full(surround_object, text, voice, volume, pitch, rate, utterance_id, interrupt,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_position_ex`][super::DisplayServer::screen_get_position_ex]."]
#[must_use]
pub struct ExScreenGetPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetPosition < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_position_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_size_ex`][super::DisplayServer::screen_get_size_ex]."]
#[must_use]
pub struct ExScreenGetSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_size_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_usable_rect_ex`][super::DisplayServer::screen_get_usable_rect_ex]."]
#[must_use]
pub struct ExScreenGetUsableRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetUsableRect < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_usable_rect_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_dpi_ex`][super::DisplayServer::screen_get_dpi_ex]."]
#[must_use]
pub struct ExScreenGetDpi < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetDpi < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_dpi_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_scale_ex`][super::DisplayServer::screen_get_scale_ex]."]
#[must_use]
pub struct ExScreenGetScale < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetScale < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_scale_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_refresh_rate_ex`][super::DisplayServer::screen_get_refresh_rate_ex]."]
#[must_use]
pub struct ExScreenGetRefreshRate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetRefreshRate < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_refresh_rate_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_image_ex`][super::DisplayServer::screen_get_image_ex]."]
#[must_use]
pub struct ExScreenGetImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetImage < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_image_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_set_orientation_ex`][super::DisplayServer::screen_set_orientation_ex]."]
#[must_use]
pub struct ExScreenSetOrientation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, orientation: crate::classes::display_server::ScreenOrientation, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenSetOrientation < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, orientation: crate::classes::display_server::ScreenOrientation,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, orientation: orientation, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, orientation, screen,
        }
        = self;
        re_export::DisplayServer::screen_set_orientation_full(surround_object, orientation, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_orientation_ex`][super::DisplayServer::screen_get_orientation_ex]."]
#[must_use]
pub struct ExScreenGetOrientation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetOrientation < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::ScreenOrientation {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_orientation_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_native_handle_ex`][super::DisplayServer::window_get_native_handle_ex]."]
#[must_use]
pub struct ExWindowGetNativeHandle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, handle_type: crate::classes::display_server::HandleType, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetNativeHandle < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, handle_type: crate::classes::display_server::HandleType,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, handle_type: handle_type, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, handle_type, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_native_handle_full(surround_object, handle_type, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_title_ex`][super::DisplayServer::window_set_title_ex]."]
#[must_use]
pub struct ExWindowSetTitle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, title: CowArg < 'a, GString >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetTitle < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, title: impl AsArg < GString > + 'a,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, title, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_title_full(surround_object, title, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_title_size_ex`][super::DisplayServer::window_get_title_size_ex]."]
#[must_use]
pub struct ExWindowGetTitleSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, title: CowArg < 'a, GString >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetTitleSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, title: impl AsArg < GString > + 'a,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, title, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_title_size_full(surround_object, title, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_mouse_passthrough_ex`][super::DisplayServer::window_set_mouse_passthrough_ex]."]
#[must_use]
pub struct ExWindowSetMousePassthrough < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, region: CowArg < 'a, PackedVector2Array >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMousePassthrough < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, region: &'a PackedVector2Array,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, region: CowArg::Borrowed(region), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, region, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_mouse_passthrough_full(surround_object, region.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_current_screen_ex`][super::DisplayServer::window_get_current_screen_ex]."]
#[must_use]
pub struct ExWindowGetCurrentScreen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetCurrentScreen < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_current_screen_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_current_screen_ex`][super::DisplayServer::window_set_current_screen_ex]."]
#[must_use]
pub struct ExWindowSetCurrentScreen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, screen: i32, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetCurrentScreen < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, screen: i32,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, screen, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_current_screen_full(surround_object, screen, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_position_ex`][super::DisplayServer::window_get_position_ex]."]
#[must_use]
pub struct ExWindowGetPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetPosition < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_position_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_position_with_decorations_ex`][super::DisplayServer::window_get_position_with_decorations_ex]."]
#[must_use]
pub struct ExWindowGetPositionWithDecorations < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetPositionWithDecorations < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_position_with_decorations_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_position_ex`][super::DisplayServer::window_set_position_ex]."]
#[must_use]
pub struct ExWindowSetPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, position: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetPosition < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, position: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_position_full(surround_object, position, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_size_ex`][super::DisplayServer::window_get_size_ex]."]
#[must_use]
pub struct ExWindowGetSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_size_ex`][super::DisplayServer::window_set_size_ex]."]
#[must_use]
pub struct ExWindowSetSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size: size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_size_full(surround_object, size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_rect_changed_callback_ex`][super::DisplayServer::window_set_rect_changed_callback_ex]."]
#[must_use]
pub struct ExWindowSetRectChangedCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetRectChangedCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_rect_changed_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_window_event_callback_ex`][super::DisplayServer::window_set_window_event_callback_ex]."]
#[must_use]
pub struct ExWindowSetWindowEventCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetWindowEventCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_window_event_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_input_event_callback_ex`][super::DisplayServer::window_set_input_event_callback_ex]."]
#[must_use]
pub struct ExWindowSetInputEventCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetInputEventCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_input_event_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_input_text_callback_ex`][super::DisplayServer::window_set_input_text_callback_ex]."]
#[must_use]
pub struct ExWindowSetInputTextCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetInputTextCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_input_text_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_drop_files_callback_ex`][super::DisplayServer::window_set_drop_files_callback_ex]."]
#[must_use]
pub struct ExWindowSetDropFilesCallback < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, callback: CowArg < 'a, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetDropFilesCallback < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, callback: &'a Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_drop_files_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_attached_instance_id_ex`][super::DisplayServer::window_get_attached_instance_id_ex]."]
#[must_use]
pub struct ExWindowGetAttachedInstanceId < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetAttachedInstanceId < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_attached_instance_id_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_max_size_ex`][super::DisplayServer::window_get_max_size_ex]."]
#[must_use]
pub struct ExWindowGetMaxSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMaxSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_max_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_max_size_ex`][super::DisplayServer::window_set_max_size_ex]."]
#[must_use]
pub struct ExWindowSetMaxSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, max_size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMaxSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, max_size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, max_size: max_size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, max_size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_max_size_full(surround_object, max_size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_min_size_ex`][super::DisplayServer::window_get_min_size_ex]."]
#[must_use]
pub struct ExWindowGetMinSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMinSize < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_min_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_min_size_ex`][super::DisplayServer::window_set_min_size_ex]."]
#[must_use]
pub struct ExWindowSetMinSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, min_size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMinSize < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, min_size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, min_size: min_size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, min_size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_min_size_full(surround_object, min_size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_size_with_decorations_ex`][super::DisplayServer::window_get_size_with_decorations_ex]."]
#[must_use]
pub struct ExWindowGetSizeWithDecorations < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSizeWithDecorations < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_size_with_decorations_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_mode_ex`][super::DisplayServer::window_get_mode_ex]."]
#[must_use]
pub struct ExWindowGetMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetMode < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::WindowMode {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_mode_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_mode_ex`][super::DisplayServer::window_set_mode_ex]."]
#[must_use]
pub struct ExWindowSetMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, mode: crate::classes::display_server::WindowMode, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetMode < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, mode: crate::classes::display_server::WindowMode,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mode: mode, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mode, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_mode_full(surround_object, mode, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_flag_ex`][super::DisplayServer::window_set_flag_ex]."]
#[must_use]
pub struct ExWindowSetFlag < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, enabled: bool, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetFlag < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, enabled: bool,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flag: flag, enabled: enabled, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, flag, enabled, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_flag_full(surround_object, flag, enabled, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_flag_ex`][super::DisplayServer::window_get_flag_ex]."]
#[must_use]
pub struct ExWindowGetFlag < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetFlag < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flag: flag, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, flag, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_flag_full(surround_object, flag, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_window_buttons_offset_ex`][super::DisplayServer::window_set_window_buttons_offset_ex]."]
#[must_use]
pub struct ExWindowSetWindowButtonsOffset < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, offset: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetWindowButtonsOffset < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, offset: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, offset: offset, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, offset, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_window_buttons_offset_full(surround_object, offset, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_safe_title_margins_ex`][super::DisplayServer::window_get_safe_title_margins_ex]."]
#[must_use]
pub struct ExWindowGetSafeTitleMargins < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetSafeTitleMargins < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_safe_title_margins_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_request_attention_ex`][super::DisplayServer::window_request_attention_ex]."]
#[must_use]
pub struct ExWindowRequestAttention < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowRequestAttention < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_request_attention_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_move_to_foreground_ex`][super::DisplayServer::window_move_to_foreground_ex]."]
#[must_use]
pub struct ExWindowMoveToForeground < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowMoveToForeground < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_move_to_foreground_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_is_focused_ex`][super::DisplayServer::window_is_focused_ex]."]
#[must_use]
pub struct ExWindowIsFocused < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowIsFocused < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_is_focused_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_can_draw_ex`][super::DisplayServer::window_can_draw_ex]."]
#[must_use]
pub struct ExWindowCanDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowCanDraw < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_can_draw_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_ime_active_ex`][super::DisplayServer::window_set_ime_active_ex]."]
#[must_use]
pub struct ExWindowSetImeActive < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, active: bool, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetImeActive < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, active: bool,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, active: active, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, active, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_ime_active_full(surround_object, active, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_ime_position_ex`][super::DisplayServer::window_set_ime_position_ex]."]
#[must_use]
pub struct ExWindowSetImePosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, position: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetImePosition < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, position: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_ime_position_full(surround_object, position, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_vsync_mode_ex`][super::DisplayServer::window_set_vsync_mode_ex]."]
#[must_use]
pub struct ExWindowSetVSyncMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, vsync_mode: crate::classes::display_server::VSyncMode, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowSetVSyncMode < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, vsync_mode: crate::classes::display_server::VSyncMode,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, vsync_mode: vsync_mode, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, vsync_mode, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_vsync_mode_full(surround_object, vsync_mode, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_vsync_mode_ex`][super::DisplayServer::window_get_vsync_mode_ex]."]
#[must_use]
pub struct ExWindowGetVSyncMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowGetVSyncMode < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::VSyncMode {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_vsync_mode_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_is_maximize_allowed_ex`][super::DisplayServer::window_is_maximize_allowed_ex]."]
#[must_use]
pub struct ExWindowIsMaximizeAllowed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExWindowIsMaximizeAllowed < 'a > {
    fn new(surround_object: &'a re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_is_maximize_allowed_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::virtual_keyboard_show_ex`][super::DisplayServer::virtual_keyboard_show_ex]."]
#[must_use]
pub struct ExVirtualKeyboardShow < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, existing_text: CowArg < 'a, GString >, position: Rect2, type_: crate::classes::display_server::VirtualKeyboardType, max_length: i32, cursor_start: i32, cursor_end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVirtualKeyboardShow < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, existing_text: impl AsArg < GString > + 'a,) -> Self {
        let position = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let type_ = crate::obj::EngineEnum::from_ord(0);
        let max_length = - 1i32;
        let cursor_start = - 1i32;
        let cursor_end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, existing_text: existing_text.into_arg(), position: position, type_: type_, max_length: max_length, cursor_start: cursor_start, cursor_end: cursor_end,
        }
    }
    #[inline]
    pub fn position(self, position: Rect2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn type_(self, type_: crate::classes::display_server::VirtualKeyboardType) -> Self {
        Self {
            type_: type_, .. self
        }
    }
    #[inline]
    pub fn max_length(self, max_length: i32) -> Self {
        Self {
            max_length: max_length, .. self
        }
    }
    #[inline]
    pub fn cursor_start(self, cursor_start: i32) -> Self {
        Self {
            cursor_start: cursor_start, .. self
        }
    }
    #[inline]
    pub fn cursor_end(self, cursor_end: i32) -> Self {
        Self {
            cursor_end: cursor_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, existing_text, position, type_, max_length, cursor_start, cursor_end,
        }
        = self;
        re_export::DisplayServer::virtual_keyboard_show_full(surround_object, existing_text, position, type_, max_length, cursor_start, cursor_end,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::cursor_set_custom_image_ex`][super::DisplayServer::cursor_set_custom_image_ex]."]
#[must_use]
pub struct ExCursorSetCustomImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DisplayServer, cursor: ObjectCow < crate::classes::Resource >, shape: crate::classes::display_server::CursorShape, hotspot: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCursorSetCustomImage < 'a > {
    fn new(surround_object: &'a mut re_export::DisplayServer, cursor: impl AsObjectArg < crate::classes::Resource >,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        let hotspot = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cursor: cursor.consume_arg(), shape: shape, hotspot: hotspot,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::display_server::CursorShape) -> Self {
        Self {
            shape: shape, .. self
        }
    }
    #[inline]
    pub fn hotspot(self, hotspot: Vector2) -> Self {
        Self {
            hotspot: hotspot, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, cursor, shape, hotspot,
        }
        = self;
        re_export::DisplayServer::cursor_set_custom_image_full(surround_object, cursor.cow_as_object_arg(), shape, hotspot,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    #[doc(alias = "FEATURE_GLOBAL_MENU")]
    #[doc = "Godot enumerator name: `FEATURE_GLOBAL_MENU`"]
    pub const GLOBAL_MENU: Feature = Feature {
        ord: 0i32
    };
    #[doc(alias = "FEATURE_SUBWINDOWS")]
    #[doc = "Godot enumerator name: `FEATURE_SUBWINDOWS`"]
    pub const SUBWINDOWS: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_TOUCHSCREEN")]
    #[doc = "Godot enumerator name: `FEATURE_TOUCHSCREEN`"]
    pub const TOUCHSCREEN: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_MOUSE")]
    #[doc = "Godot enumerator name: `FEATURE_MOUSE`"]
    pub const MOUSE: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_MOUSE_WARP")]
    #[doc = "Godot enumerator name: `FEATURE_MOUSE_WARP`"]
    pub const MOUSE_WARP: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_CLIPBOARD")]
    #[doc = "Godot enumerator name: `FEATURE_CLIPBOARD`"]
    pub const CLIPBOARD: Feature = Feature {
        ord: 5i32
    };
    #[doc(alias = "FEATURE_VIRTUAL_KEYBOARD")]
    #[doc = "Godot enumerator name: `FEATURE_VIRTUAL_KEYBOARD`"]
    pub const VIRTUAL_KEYBOARD: Feature = Feature {
        ord: 6i32
    };
    #[doc(alias = "FEATURE_CURSOR_SHAPE")]
    #[doc = "Godot enumerator name: `FEATURE_CURSOR_SHAPE`"]
    pub const CURSOR_SHAPE: Feature = Feature {
        ord: 7i32
    };
    #[doc(alias = "FEATURE_CUSTOM_CURSOR_SHAPE")]
    #[doc = "Godot enumerator name: `FEATURE_CUSTOM_CURSOR_SHAPE`"]
    pub const CUSTOM_CURSOR_SHAPE: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG`"]
    pub const NATIVE_DIALOG: Feature = Feature {
        ord: 9i32
    };
    #[doc(alias = "FEATURE_IME")]
    #[doc = "Godot enumerator name: `FEATURE_IME`"]
    pub const IME: Feature = Feature {
        ord: 10i32
    };
    #[doc(alias = "FEATURE_WINDOW_TRANSPARENCY")]
    #[doc = "Godot enumerator name: `FEATURE_WINDOW_TRANSPARENCY`"]
    pub const WINDOW_TRANSPARENCY: Feature = Feature {
        ord: 11i32
    };
    #[doc(alias = "FEATURE_HIDPI")]
    #[doc = "Godot enumerator name: `FEATURE_HIDPI`"]
    pub const HIDPI: Feature = Feature {
        ord: 12i32
    };
    #[doc(alias = "FEATURE_ICON")]
    #[doc = "Godot enumerator name: `FEATURE_ICON`"]
    pub const ICON: Feature = Feature {
        ord: 13i32
    };
    #[doc(alias = "FEATURE_NATIVE_ICON")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_ICON`"]
    pub const NATIVE_ICON: Feature = Feature {
        ord: 14i32
    };
    #[doc(alias = "FEATURE_ORIENTATION")]
    #[doc = "Godot enumerator name: `FEATURE_ORIENTATION`"]
    pub const ORIENTATION: Feature = Feature {
        ord: 15i32
    };
    #[doc(alias = "FEATURE_SWAP_BUFFERS")]
    #[doc = "Godot enumerator name: `FEATURE_SWAP_BUFFERS`"]
    pub const SWAP_BUFFERS: Feature = Feature {
        ord: 16i32
    };
    #[doc(alias = "FEATURE_CLIPBOARD_PRIMARY")]
    #[doc = "Godot enumerator name: `FEATURE_CLIPBOARD_PRIMARY`"]
    pub const CLIPBOARD_PRIMARY: Feature = Feature {
        ord: 18i32
    };
    #[doc(alias = "FEATURE_TEXT_TO_SPEECH")]
    #[doc = "Godot enumerator name: `FEATURE_TEXT_TO_SPEECH`"]
    pub const TEXT_TO_SPEECH: Feature = Feature {
        ord: 19i32
    };
    #[doc(alias = "FEATURE_EXTEND_TO_TITLE")]
    #[doc = "Godot enumerator name: `FEATURE_EXTEND_TO_TITLE`"]
    pub const EXTEND_TO_TITLE: Feature = Feature {
        ord: 20i32
    };
    #[doc(alias = "FEATURE_SCREEN_CAPTURE")]
    #[doc = "Godot enumerator name: `FEATURE_SCREEN_CAPTURE`"]
    pub const SCREEN_CAPTURE: Feature = Feature {
        ord: 21i32
    };
    #[doc(alias = "FEATURE_STATUS_INDICATOR")]
    #[doc = "Godot enumerator name: `FEATURE_STATUS_INDICATOR`"]
    pub const STATUS_INDICATOR: Feature = Feature {
        ord: 22i32
    };
    #[doc(alias = "FEATURE_NATIVE_HELP")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_HELP`"]
    pub const NATIVE_HELP: Feature = Feature {
        ord: 23i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_INPUT")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_INPUT`"]
    pub const NATIVE_DIALOG_INPUT: Feature = Feature {
        ord: 24i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_FILE")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_FILE`"]
    pub const NATIVE_DIALOG_FILE: Feature = Feature {
        ord: 25i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 => Some(Self {
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
            Self::GLOBAL_MENU => "GLOBAL_MENU", Self::SUBWINDOWS => "SUBWINDOWS", Self::TOUCHSCREEN => "TOUCHSCREEN", Self::MOUSE => "MOUSE", Self::MOUSE_WARP => "MOUSE_WARP", Self::CLIPBOARD => "CLIPBOARD", Self::VIRTUAL_KEYBOARD => "VIRTUAL_KEYBOARD", Self::CURSOR_SHAPE => "CURSOR_SHAPE", Self::CUSTOM_CURSOR_SHAPE => "CUSTOM_CURSOR_SHAPE", Self::NATIVE_DIALOG => "NATIVE_DIALOG", Self::IME => "IME", Self::WINDOW_TRANSPARENCY => "WINDOW_TRANSPARENCY", Self::HIDPI => "HIDPI", Self::ICON => "ICON", Self::NATIVE_ICON => "NATIVE_ICON", Self::ORIENTATION => "ORIENTATION", Self::SWAP_BUFFERS => "SWAP_BUFFERS", Self::CLIPBOARD_PRIMARY => "CLIPBOARD_PRIMARY", Self::TEXT_TO_SPEECH => "TEXT_TO_SPEECH", Self::EXTEND_TO_TITLE => "EXTEND_TO_TITLE", Self::SCREEN_CAPTURE => "SCREEN_CAPTURE", Self::STATUS_INDICATOR => "STATUS_INDICATOR", Self::NATIVE_HELP => "NATIVE_HELP", Self::NATIVE_DIALOG_INPUT => "NATIVE_DIALOG_INPUT", Self::NATIVE_DIALOG_FILE => "NATIVE_DIALOG_FILE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GLOBAL_MENU => "FEATURE_GLOBAL_MENU", Self::SUBWINDOWS => "FEATURE_SUBWINDOWS", Self::TOUCHSCREEN => "FEATURE_TOUCHSCREEN", Self::MOUSE => "FEATURE_MOUSE", Self::MOUSE_WARP => "FEATURE_MOUSE_WARP", Self::CLIPBOARD => "FEATURE_CLIPBOARD", Self::VIRTUAL_KEYBOARD => "FEATURE_VIRTUAL_KEYBOARD", Self::CURSOR_SHAPE => "FEATURE_CURSOR_SHAPE", Self::CUSTOM_CURSOR_SHAPE => "FEATURE_CUSTOM_CURSOR_SHAPE", Self::NATIVE_DIALOG => "FEATURE_NATIVE_DIALOG", Self::IME => "FEATURE_IME", Self::WINDOW_TRANSPARENCY => "FEATURE_WINDOW_TRANSPARENCY", Self::HIDPI => "FEATURE_HIDPI", Self::ICON => "FEATURE_ICON", Self::NATIVE_ICON => "FEATURE_NATIVE_ICON", Self::ORIENTATION => "FEATURE_ORIENTATION", Self::SWAP_BUFFERS => "FEATURE_SWAP_BUFFERS", Self::CLIPBOARD_PRIMARY => "FEATURE_CLIPBOARD_PRIMARY", Self::TEXT_TO_SPEECH => "FEATURE_TEXT_TO_SPEECH", Self::EXTEND_TO_TITLE => "FEATURE_EXTEND_TO_TITLE", Self::SCREEN_CAPTURE => "FEATURE_SCREEN_CAPTURE", Self::STATUS_INDICATOR => "FEATURE_STATUS_INDICATOR", Self::NATIVE_HELP => "FEATURE_NATIVE_HELP", Self::NATIVE_DIALOG_INPUT => "FEATURE_NATIVE_DIALOG_INPUT", Self::NATIVE_DIALOG_FILE => "FEATURE_NATIVE_DIALOG_FILE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Feature {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseMode {
    ord: i32
}
impl MouseMode {
    #[doc(alias = "MOUSE_MODE_VISIBLE")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_VISIBLE`"]
    pub const VISIBLE: MouseMode = MouseMode {
        ord: 0i32
    };
    #[doc(alias = "MOUSE_MODE_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_HIDDEN`"]
    pub const HIDDEN: MouseMode = MouseMode {
        ord: 1i32
    };
    #[doc(alias = "MOUSE_MODE_CAPTURED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CAPTURED`"]
    pub const CAPTURED: MouseMode = MouseMode {
        ord: 2i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED`"]
    pub const CONFINED: MouseMode = MouseMode {
        ord: 3i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED_HIDDEN`"]
    pub const CONFINED_HIDDEN: MouseMode = MouseMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for MouseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MouseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MouseMode {
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
            Self::VISIBLE => "VISIBLE", Self::HIDDEN => "HIDDEN", Self::CAPTURED => "CAPTURED", Self::CONFINED => "CONFINED", Self::CONFINED_HIDDEN => "CONFINED_HIDDEN", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VISIBLE => "MOUSE_MODE_VISIBLE", Self::HIDDEN => "MOUSE_MODE_HIDDEN", Self::CAPTURED => "MOUSE_MODE_CAPTURED", Self::CONFINED => "MOUSE_MODE_CONFINED", Self::CONFINED_HIDDEN => "MOUSE_MODE_CONFINED_HIDDEN", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for MouseMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MouseMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MouseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ScreenOrientation {
    ord: i32
}
impl ScreenOrientation {
    #[doc(alias = "SCREEN_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_LANDSCAPE`"]
    pub const LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 0i32
    };
    #[doc(alias = "SCREEN_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_PORTRAIT`"]
    pub const PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 1i32
    };
    #[doc(alias = "SCREEN_REVERSE_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_REVERSE_LANDSCAPE`"]
    pub const REVERSE_LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 2i32
    };
    #[doc(alias = "SCREEN_REVERSE_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_REVERSE_PORTRAIT`"]
    pub const REVERSE_PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 3i32
    };
    #[doc(alias = "SCREEN_SENSOR_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR_LANDSCAPE`"]
    pub const SENSOR_LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 4i32
    };
    #[doc(alias = "SCREEN_SENSOR_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR_PORTRAIT`"]
    pub const SENSOR_PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 5i32
    };
    #[doc(alias = "SCREEN_SENSOR")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR`"]
    pub const SENSOR: ScreenOrientation = ScreenOrientation {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ScreenOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScreenOrientation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScreenOrientation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::LANDSCAPE => "LANDSCAPE", Self::PORTRAIT => "PORTRAIT", Self::REVERSE_LANDSCAPE => "REVERSE_LANDSCAPE", Self::REVERSE_PORTRAIT => "REVERSE_PORTRAIT", Self::SENSOR_LANDSCAPE => "SENSOR_LANDSCAPE", Self::SENSOR_PORTRAIT => "SENSOR_PORTRAIT", Self::SENSOR => "SENSOR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LANDSCAPE => "SCREEN_LANDSCAPE", Self::PORTRAIT => "SCREEN_PORTRAIT", Self::REVERSE_LANDSCAPE => "SCREEN_REVERSE_LANDSCAPE", Self::REVERSE_PORTRAIT => "SCREEN_REVERSE_PORTRAIT", Self::SENSOR_LANDSCAPE => "SCREEN_SENSOR_LANDSCAPE", Self::SENSOR_PORTRAIT => "SCREEN_SENSOR_PORTRAIT", Self::SENSOR => "SCREEN_SENSOR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ScreenOrientation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ScreenOrientation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScreenOrientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VirtualKeyboardType {
    ord: i32
}
impl VirtualKeyboardType {
    #[doc(alias = "KEYBOARD_TYPE_DEFAULT")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_DEFAULT`"]
    pub const DEFAULT: VirtualKeyboardType = VirtualKeyboardType {
        ord: 0i32
    };
    #[doc(alias = "KEYBOARD_TYPE_MULTILINE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_MULTILINE`"]
    pub const MULTILINE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 1i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER`"]
    pub const NUMBER: VirtualKeyboardType = VirtualKeyboardType {
        ord: 2i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER_DECIMAL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER_DECIMAL`"]
    pub const NUMBER_DECIMAL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 3i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PHONE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PHONE`"]
    pub const PHONE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 4i32
    };
    #[doc(alias = "KEYBOARD_TYPE_EMAIL_ADDRESS")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_EMAIL_ADDRESS`"]
    pub const EMAIL_ADDRESS: VirtualKeyboardType = VirtualKeyboardType {
        ord: 5i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PASSWORD")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PASSWORD`"]
    pub const PASSWORD: VirtualKeyboardType = VirtualKeyboardType {
        ord: 6i32
    };
    #[doc(alias = "KEYBOARD_TYPE_URL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_URL`"]
    pub const URL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for VirtualKeyboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VirtualKeyboardType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VirtualKeyboardType {
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
            Self::DEFAULT => "DEFAULT", Self::MULTILINE => "MULTILINE", Self::NUMBER => "NUMBER", Self::NUMBER_DECIMAL => "NUMBER_DECIMAL", Self::PHONE => "PHONE", Self::EMAIL_ADDRESS => "EMAIL_ADDRESS", Self::PASSWORD => "PASSWORD", Self::URL => "URL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "KEYBOARD_TYPE_DEFAULT", Self::MULTILINE => "KEYBOARD_TYPE_MULTILINE", Self::NUMBER => "KEYBOARD_TYPE_NUMBER", Self::NUMBER_DECIMAL => "KEYBOARD_TYPE_NUMBER_DECIMAL", Self::PHONE => "KEYBOARD_TYPE_PHONE", Self::EMAIL_ADDRESS => "KEYBOARD_TYPE_EMAIL_ADDRESS", Self::PASSWORD => "KEYBOARD_TYPE_PASSWORD", Self::URL => "KEYBOARD_TYPE_URL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for VirtualKeyboardType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VirtualKeyboardType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VirtualKeyboardType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CursorShape {
    ord: i32
}
impl CursorShape {
    #[doc(alias = "CURSOR_ARROW")]
    #[doc = "Godot enumerator name: `CURSOR_ARROW`"]
    pub const ARROW: CursorShape = CursorShape {
        ord: 0i32
    };
    #[doc(alias = "CURSOR_IBEAM")]
    #[doc = "Godot enumerator name: `CURSOR_IBEAM`"]
    pub const IBEAM: CursorShape = CursorShape {
        ord: 1i32
    };
    #[doc(alias = "CURSOR_POINTING_HAND")]
    #[doc = "Godot enumerator name: `CURSOR_POINTING_HAND`"]
    pub const POINTING_HAND: CursorShape = CursorShape {
        ord: 2i32
    };
    #[doc(alias = "CURSOR_CROSS")]
    #[doc = "Godot enumerator name: `CURSOR_CROSS`"]
    pub const CROSS: CursorShape = CursorShape {
        ord: 3i32
    };
    #[doc(alias = "CURSOR_WAIT")]
    #[doc = "Godot enumerator name: `CURSOR_WAIT`"]
    pub const WAIT: CursorShape = CursorShape {
        ord: 4i32
    };
    #[doc(alias = "CURSOR_BUSY")]
    #[doc = "Godot enumerator name: `CURSOR_BUSY`"]
    pub const BUSY: CursorShape = CursorShape {
        ord: 5i32
    };
    #[doc(alias = "CURSOR_DRAG")]
    #[doc = "Godot enumerator name: `CURSOR_DRAG`"]
    pub const DRAG: CursorShape = CursorShape {
        ord: 6i32
    };
    #[doc(alias = "CURSOR_CAN_DROP")]
    #[doc = "Godot enumerator name: `CURSOR_CAN_DROP`"]
    pub const CAN_DROP: CursorShape = CursorShape {
        ord: 7i32
    };
    #[doc(alias = "CURSOR_FORBIDDEN")]
    #[doc = "Godot enumerator name: `CURSOR_FORBIDDEN`"]
    pub const FORBIDDEN: CursorShape = CursorShape {
        ord: 8i32
    };
    #[doc(alias = "CURSOR_VSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_VSIZE`"]
    pub const VSIZE: CursorShape = CursorShape {
        ord: 9i32
    };
    #[doc(alias = "CURSOR_HSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_HSIZE`"]
    pub const HSIZE: CursorShape = CursorShape {
        ord: 10i32
    };
    #[doc(alias = "CURSOR_BDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_BDIAGSIZE`"]
    pub const BDIAGSIZE: CursorShape = CursorShape {
        ord: 11i32
    };
    #[doc(alias = "CURSOR_FDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_FDIAGSIZE`"]
    pub const FDIAGSIZE: CursorShape = CursorShape {
        ord: 12i32
    };
    #[doc(alias = "CURSOR_MOVE")]
    #[doc = "Godot enumerator name: `CURSOR_MOVE`"]
    pub const MOVE: CursorShape = CursorShape {
        ord: 13i32
    };
    #[doc(alias = "CURSOR_VSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_VSPLIT`"]
    pub const VSPLIT: CursorShape = CursorShape {
        ord: 14i32
    };
    #[doc(alias = "CURSOR_HSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_HSPLIT`"]
    pub const HSPLIT: CursorShape = CursorShape {
        ord: 15i32
    };
    #[doc(alias = "CURSOR_HELP")]
    #[doc = "Godot enumerator name: `CURSOR_HELP`"]
    pub const HELP: CursorShape = CursorShape {
        ord: 16i32
    };
    #[doc(alias = "CURSOR_MAX")]
    #[doc = "Godot enumerator name: `CURSOR_MAX`"]
    pub const MAX: CursorShape = CursorShape {
        ord: 17i32
    };
    
}
impl std::fmt::Debug for CursorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CursorShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CursorShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 => Some(Self {
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
            Self::ARROW => "ARROW", Self::IBEAM => "IBEAM", Self::POINTING_HAND => "POINTING_HAND", Self::CROSS => "CROSS", Self::WAIT => "WAIT", Self::BUSY => "BUSY", Self::DRAG => "DRAG", Self::CAN_DROP => "CAN_DROP", Self::FORBIDDEN => "FORBIDDEN", Self::VSIZE => "VSIZE", Self::HSIZE => "HSIZE", Self::BDIAGSIZE => "BDIAGSIZE", Self::FDIAGSIZE => "FDIAGSIZE", Self::MOVE => "MOVE", Self::VSPLIT => "VSPLIT", Self::HSPLIT => "HSPLIT", Self::HELP => "HELP", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ARROW => "CURSOR_ARROW", Self::IBEAM => "CURSOR_IBEAM", Self::POINTING_HAND => "CURSOR_POINTING_HAND", Self::CROSS => "CURSOR_CROSS", Self::WAIT => "CURSOR_WAIT", Self::BUSY => "CURSOR_BUSY", Self::DRAG => "CURSOR_DRAG", Self::CAN_DROP => "CURSOR_CAN_DROP", Self::FORBIDDEN => "CURSOR_FORBIDDEN", Self::VSIZE => "CURSOR_VSIZE", Self::HSIZE => "CURSOR_HSIZE", Self::BDIAGSIZE => "CURSOR_BDIAGSIZE", Self::FDIAGSIZE => "CURSOR_FDIAGSIZE", Self::MOVE => "CURSOR_MOVE", Self::VSPLIT => "CURSOR_VSPLIT", Self::HSPLIT => "CURSOR_HSPLIT", Self::HELP => "CURSOR_HELP", Self::MAX => "CURSOR_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for CursorShape {
    const ENUMERATOR_COUNT: usize = 17usize;
    
}
impl crate::meta::GodotConvert for CursorShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CursorShape {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CursorShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileDialogMode {
    ord: i32
}
impl FileDialogMode {
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_FILE")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_FILE`"]
    pub const OPEN_FILE: FileDialogMode = FileDialogMode {
        ord: 0i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_FILES")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_FILES`"]
    pub const OPEN_FILES: FileDialogMode = FileDialogMode {
        ord: 1i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_DIR")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_DIR`"]
    pub const OPEN_DIR: FileDialogMode = FileDialogMode {
        ord: 2i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_ANY")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_ANY`"]
    pub const OPEN_ANY: FileDialogMode = FileDialogMode {
        ord: 3i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_SAVE_FILE")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_SAVE_FILE`"]
    pub const SAVE_FILE: FileDialogMode = FileDialogMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FileDialogMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FileDialogMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FileDialogMode {
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
            Self::OPEN_FILE => "OPEN_FILE", Self::OPEN_FILES => "OPEN_FILES", Self::OPEN_DIR => "OPEN_DIR", Self::OPEN_ANY => "OPEN_ANY", Self::SAVE_FILE => "SAVE_FILE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OPEN_FILE => "FILE_DIALOG_MODE_OPEN_FILE", Self::OPEN_FILES => "FILE_DIALOG_MODE_OPEN_FILES", Self::OPEN_DIR => "FILE_DIALOG_MODE_OPEN_DIR", Self::OPEN_ANY => "FILE_DIALOG_MODE_OPEN_ANY", Self::SAVE_FILE => "FILE_DIALOG_MODE_SAVE_FILE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FileDialogMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FileDialogMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FileDialogMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowMode {
    ord: i32
}
impl WindowMode {
    #[doc(alias = "WINDOW_MODE_WINDOWED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_WINDOWED`"]
    pub const WINDOWED: WindowMode = WindowMode {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_MODE_MINIMIZED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_MINIMIZED`"]
    pub const MINIMIZED: WindowMode = WindowMode {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_MODE_MAXIMIZED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_MAXIMIZED`"]
    pub const MAXIMIZED: WindowMode = WindowMode {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_MODE_FULLSCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_FULLSCREEN`"]
    pub const FULLSCREEN: WindowMode = WindowMode {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_MODE_EXCLUSIVE_FULLSCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_EXCLUSIVE_FULLSCREEN`"]
    pub const EXCLUSIVE_FULLSCREEN: WindowMode = WindowMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for WindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowMode {
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
            Self::WINDOWED => "WINDOWED", Self::MINIMIZED => "MINIMIZED", Self::MAXIMIZED => "MAXIMIZED", Self::FULLSCREEN => "FULLSCREEN", Self::EXCLUSIVE_FULLSCREEN => "EXCLUSIVE_FULLSCREEN", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::WINDOWED => "WINDOW_MODE_WINDOWED", Self::MINIMIZED => "WINDOW_MODE_MINIMIZED", Self::MAXIMIZED => "WINDOW_MODE_MAXIMIZED", Self::FULLSCREEN => "WINDOW_MODE_FULLSCREEN", Self::EXCLUSIVE_FULLSCREEN => "WINDOW_MODE_EXCLUSIVE_FULLSCREEN", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for WindowMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WindowMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowFlags {
    ord: i32
}
impl WindowFlags {
    #[doc(alias = "WINDOW_FLAG_RESIZE_DISABLED")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_RESIZE_DISABLED`"]
    pub const RESIZE_DISABLED: WindowFlags = WindowFlags {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_FLAG_BORDERLESS")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_BORDERLESS`"]
    pub const BORDERLESS: WindowFlags = WindowFlags {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_FLAG_ALWAYS_ON_TOP")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_ALWAYS_ON_TOP`"]
    pub const ALWAYS_ON_TOP: WindowFlags = WindowFlags {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_FLAG_TRANSPARENT")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_TRANSPARENT`"]
    pub const TRANSPARENT: WindowFlags = WindowFlags {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_FLAG_NO_FOCUS")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_NO_FOCUS`"]
    pub const NO_FOCUS: WindowFlags = WindowFlags {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_FLAG_POPUP")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_POPUP`"]
    pub const POPUP: WindowFlags = WindowFlags {
        ord: 5i32
    };
    #[doc(alias = "WINDOW_FLAG_EXTEND_TO_TITLE")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_EXTEND_TO_TITLE`"]
    pub const EXTEND_TO_TITLE: WindowFlags = WindowFlags {
        ord: 6i32
    };
    #[doc(alias = "WINDOW_FLAG_MOUSE_PASSTHROUGH")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MOUSE_PASSTHROUGH`"]
    pub const MOUSE_PASSTHROUGH: WindowFlags = WindowFlags {
        ord: 7i32
    };
    #[doc(alias = "WINDOW_FLAG_MAX")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MAX`"]
    pub const MAX: WindowFlags = WindowFlags {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for WindowFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::RESIZE_DISABLED => "RESIZE_DISABLED", Self::BORDERLESS => "BORDERLESS", Self::ALWAYS_ON_TOP => "ALWAYS_ON_TOP", Self::TRANSPARENT => "TRANSPARENT", Self::NO_FOCUS => "NO_FOCUS", Self::POPUP => "POPUP", Self::EXTEND_TO_TITLE => "EXTEND_TO_TITLE", Self::MOUSE_PASSTHROUGH => "MOUSE_PASSTHROUGH", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::RESIZE_DISABLED => "WINDOW_FLAG_RESIZE_DISABLED", Self::BORDERLESS => "WINDOW_FLAG_BORDERLESS", Self::ALWAYS_ON_TOP => "WINDOW_FLAG_ALWAYS_ON_TOP", Self::TRANSPARENT => "WINDOW_FLAG_TRANSPARENT", Self::NO_FOCUS => "WINDOW_FLAG_NO_FOCUS", Self::POPUP => "WINDOW_FLAG_POPUP", Self::EXTEND_TO_TITLE => "WINDOW_FLAG_EXTEND_TO_TITLE", Self::MOUSE_PASSTHROUGH => "WINDOW_FLAG_MOUSE_PASSTHROUGH", Self::MAX => "WINDOW_FLAG_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for WindowFlags {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for WindowFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WindowFlags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowEvent {
    ord: i32
}
impl WindowEvent {
    #[doc(alias = "WINDOW_EVENT_MOUSE_ENTER")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_MOUSE_ENTER`"]
    pub const MOUSE_ENTER: WindowEvent = WindowEvent {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_EVENT_MOUSE_EXIT")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_MOUSE_EXIT`"]
    pub const MOUSE_EXIT: WindowEvent = WindowEvent {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_EVENT_FOCUS_IN")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_FOCUS_IN`"]
    pub const FOCUS_IN: WindowEvent = WindowEvent {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_EVENT_FOCUS_OUT")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_FOCUS_OUT`"]
    pub const FOCUS_OUT: WindowEvent = WindowEvent {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_EVENT_CLOSE_REQUEST")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_CLOSE_REQUEST`"]
    pub const CLOSE_REQUEST: WindowEvent = WindowEvent {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_EVENT_GO_BACK_REQUEST")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_GO_BACK_REQUEST`"]
    pub const GO_BACK_REQUEST: WindowEvent = WindowEvent {
        ord: 5i32
    };
    #[doc(alias = "WINDOW_EVENT_DPI_CHANGE")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_DPI_CHANGE`"]
    pub const DPI_CHANGE: WindowEvent = WindowEvent {
        ord: 6i32
    };
    #[doc(alias = "WINDOW_EVENT_TITLEBAR_CHANGE")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_TITLEBAR_CHANGE`"]
    pub const TITLEBAR_CHANGE: WindowEvent = WindowEvent {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for WindowEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowEvent") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowEvent {
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
            Self::MOUSE_ENTER => "MOUSE_ENTER", Self::MOUSE_EXIT => "MOUSE_EXIT", Self::FOCUS_IN => "FOCUS_IN", Self::FOCUS_OUT => "FOCUS_OUT", Self::CLOSE_REQUEST => "CLOSE_REQUEST", Self::GO_BACK_REQUEST => "GO_BACK_REQUEST", Self::DPI_CHANGE => "DPI_CHANGE", Self::TITLEBAR_CHANGE => "TITLEBAR_CHANGE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::MOUSE_ENTER => "WINDOW_EVENT_MOUSE_ENTER", Self::MOUSE_EXIT => "WINDOW_EVENT_MOUSE_EXIT", Self::FOCUS_IN => "WINDOW_EVENT_FOCUS_IN", Self::FOCUS_OUT => "WINDOW_EVENT_FOCUS_OUT", Self::CLOSE_REQUEST => "WINDOW_EVENT_CLOSE_REQUEST", Self::GO_BACK_REQUEST => "WINDOW_EVENT_GO_BACK_REQUEST", Self::DPI_CHANGE => "WINDOW_EVENT_DPI_CHANGE", Self::TITLEBAR_CHANGE => "WINDOW_EVENT_TITLEBAR_CHANGE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for WindowEvent {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WindowEvent {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowEvent {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VSyncMode {
    ord: i32
}
impl VSyncMode {
    #[doc(alias = "VSYNC_DISABLED")]
    #[doc = "Godot enumerator name: `VSYNC_DISABLED`"]
    pub const DISABLED: VSyncMode = VSyncMode {
        ord: 0i32
    };
    #[doc(alias = "VSYNC_ENABLED")]
    #[doc = "Godot enumerator name: `VSYNC_ENABLED`"]
    pub const ENABLED: VSyncMode = VSyncMode {
        ord: 1i32
    };
    #[doc(alias = "VSYNC_ADAPTIVE")]
    #[doc = "Godot enumerator name: `VSYNC_ADAPTIVE`"]
    pub const ADAPTIVE: VSyncMode = VSyncMode {
        ord: 2i32
    };
    #[doc(alias = "VSYNC_MAILBOX")]
    #[doc = "Godot enumerator name: `VSYNC_MAILBOX`"]
    pub const MAILBOX: VSyncMode = VSyncMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for VSyncMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VSyncMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VSyncMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::ADAPTIVE => "ADAPTIVE", Self::MAILBOX => "MAILBOX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "VSYNC_DISABLED", Self::ENABLED => "VSYNC_ENABLED", Self::ADAPTIVE => "VSYNC_ADAPTIVE", Self::MAILBOX => "VSYNC_MAILBOX", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for VSyncMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VSyncMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VSyncMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandleType {
    ord: i32
}
impl HandleType {
    pub const DISPLAY_HANDLE: HandleType = HandleType {
        ord: 0i32
    };
    pub const WINDOW_HANDLE: HandleType = HandleType {
        ord: 1i32
    };
    pub const WINDOW_VIEW: HandleType = HandleType {
        ord: 2i32
    };
    pub const OPENGL_CONTEXT: HandleType = HandleType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for HandleType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandleType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandleType {
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
            Self::DISPLAY_HANDLE => "DISPLAY_HANDLE", Self::WINDOW_HANDLE => "WINDOW_HANDLE", Self::WINDOW_VIEW => "WINDOW_VIEW", Self::OPENGL_CONTEXT => "OPENGL_CONTEXT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        self.as_str()
    }
}
impl crate::meta::GodotConvert for HandleType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandleType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandleType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `TTSUtteranceEvent`."]
pub struct TtsUtteranceEvent {
    ord: i32
}
impl TtsUtteranceEvent {
    #[doc(alias = "TTS_UTTERANCE_STARTED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_STARTED`"]
    pub const STARTED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 0i32
    };
    #[doc(alias = "TTS_UTTERANCE_ENDED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_ENDED`"]
    pub const ENDED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 1i32
    };
    #[doc(alias = "TTS_UTTERANCE_CANCELED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_CANCELED`"]
    pub const CANCELED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 2i32
    };
    #[doc(alias = "TTS_UTTERANCE_BOUNDARY")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_BOUNDARY`"]
    pub const BOUNDARY: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TtsUtteranceEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TtsUtteranceEvent") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TtsUtteranceEvent {
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
            Self::STARTED => "STARTED", Self::ENDED => "ENDED", Self::CANCELED => "CANCELED", Self::BOUNDARY => "BOUNDARY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STARTED => "TTS_UTTERANCE_STARTED", Self::ENDED => "TTS_UTTERANCE_ENDED", Self::CANCELED => "TTS_UTTERANCE_CANCELED", Self::BOUNDARY => "TTS_UTTERANCE_BOUNDARY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TtsUtteranceEvent {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TtsUtteranceEvent {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TtsUtteranceEvent {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}