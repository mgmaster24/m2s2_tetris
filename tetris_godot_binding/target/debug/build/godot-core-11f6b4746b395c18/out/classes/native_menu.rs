#![doc = "Sidecar module for class [`NativeMenu`][crate::classes::NativeMenu].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NativeMenu` enums](https://docs.godotengine.org/en/stable/classes/class_nativemenu.html#enumerations).\n\n"]
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
    #[doc = "Godot class `NativeMenu.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`native_menu`][crate::classes::native_menu]: sidecar module with related enum/flag types\n* [`INativeMenu`][crate::classes::INativeMenu]: virtual methods\n\n\nSee also [Godot docs for `NativeMenu`](https://docs.godotengine.org/en/stable/classes/class_nativemenu.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`NativeMenu::singleton()`][NativeMenu::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct NativeMenu {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`NativeMenu`][crate::classes::NativeMenu].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `NativeMenu` methods](https://docs.godotengine.org/en/stable/classes/class_nativemenu.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INativeMenu: crate::obj::GodotClass < Base = NativeMenu > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl NativeMenu {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"NativeMenu");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn has_feature(&self, feature: crate::classes::native_menu::Feature,) -> bool {
            type CallSig = (bool, crate::classes::native_menu::Feature);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "has_feature", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_system_menu(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> bool {
            type CallSig = (bool, crate::classes::native_menu::SystemMenus);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "has_system_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_menu(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> Rid {
            type CallSig = (Rid, crate::classes::native_menu::SystemMenus);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_system_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_menu_name(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> GString {
            type CallSig = (GString, crate::classes::native_menu::SystemMenus);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_system_menu_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_menu(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "create_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_menu(&self, rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "has_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_menu(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "free_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self, rid: Rid,) -> Vector2 {
            type CallSig = (Vector2, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn popup(&mut self, rid: Rid, position: Vector2i,) {
            type CallSig = ((), Rid, Vector2i);
            let args = (rid, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interface_direction(&mut self, rid: Rid, is_rtl: bool,) {
            type CallSig = ((), Rid, bool);
            let args = (rid, is_rtl,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_interface_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_popup_open_callback(&mut self, rid: Rid, callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Callable >);
            let args = (rid, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_popup_open_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_popup_open_callback(&self, rid: Rid,) -> Callable {
            type CallSig = (Callable, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_popup_open_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_popup_close_callback(&mut self, rid: Rid, callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, RefArg < 'a0, Callable >);
            let args = (rid, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_popup_close_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_popup_close_callback(&self, rid: Rid,) -> Callable {
            type CallSig = (Callable, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_popup_close_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_minimum_width(&mut self, rid: Rid, width: f32,) {
            type CallSig = ((), Rid, f32);
            let args = (rid, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_minimum_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_minimum_width(&self, rid: Rid,) -> f32 {
            type CallSig = (f32, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_minimum_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_opened(&self, rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "is_opened", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_submenu_item_full(&mut self, rid: Rid, label: CowArg < GString >, submenu_rid: Rid, tag: RefArg < Variant >, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, Rid, CowArg < 'a0, GString >, Rid, RefArg < 'a1, Variant >, i32);
            let args = (rid, label, submenu_rid, tag, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_submenu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_submenu_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_submenu_item(&mut self, rid: Rid, label: impl AsArg < GString >, submenu_rid: Rid,) -> i32 {
            self.add_submenu_item_ex(rid, label, submenu_rid,) . done()
        }
        #[inline]
        pub fn add_submenu_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a, submenu_rid: Rid,) -> ExAddSubmenuItem < 'a > {
            ExAddSubmenuItem::new(self, rid, label, submenu_rid,)
        }
        pub(crate) fn add_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (i32, Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_item_ex(rid, label,) . done()
        }
        #[inline]
        pub fn add_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a,) -> ExAddItem < 'a > {
            ExAddItem::new(self, rid, label,)
        }
        pub(crate) fn add_check_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (i32, Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_check_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_check_item_ex(rid, label,) . done()
        }
        #[inline]
        pub fn add_check_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a,) -> ExAddCheckItem < 'a > {
            ExAddCheckItem::new(self, rid, label,)
        }
        pub(crate) fn add_icon_item_full(&mut self, rid: Rid, icon: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (i32, Rid, ObjectArg < crate::classes::Texture2D >, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_item(&mut self, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_item_ex(rid, icon, label,) . done()
        }
        #[inline]
        pub fn add_icon_item_ex < 'a > (&'a mut self, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExAddIconItem < 'a > {
            ExAddIconItem::new(self, rid, icon, label,)
        }
        pub(crate) fn add_icon_check_item_full(&mut self, rid: Rid, icon: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (i32, Rid, ObjectArg < crate::classes::Texture2D >, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_check_item(&mut self, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_check_item_ex(rid, icon, label,) . done()
        }
        #[inline]
        pub fn add_icon_check_item_ex < 'a > (&'a mut self, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExAddIconCheckItem < 'a > {
            ExAddIconCheckItem::new(self, rid, icon, label,)
        }
        pub(crate) fn add_radio_check_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (i32, Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_radio_check_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_radio_check_item_ex(rid, label,) . done()
        }
        #[inline]
        pub fn add_radio_check_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a,) -> ExAddRadioCheckItem < 'a > {
            ExAddRadioCheckItem::new(self, rid, label,)
        }
        pub(crate) fn add_icon_radio_check_item_full(&mut self, rid: Rid, icon: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (i32, Rid, ObjectArg < crate::classes::Texture2D >, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_radio_check_item(&mut self, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_radio_check_item_ex(rid, icon, label,) . done()
        }
        #[inline]
        pub fn add_icon_radio_check_item_ex < 'a > (&'a mut self, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExAddIconRadioCheckItem < 'a > {
            ExAddIconRadioCheckItem::new(self, rid, icon, label,)
        }
        pub(crate) fn add_multistate_item_full(&mut self, rid: Rid, label: CowArg < GString >, max_states: i32, default_state: i32, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = (i32, Rid, CowArg < 'a0, GString >, i32, i32, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32);
            let args = (rid, label, max_states, default_state, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_multistate_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_multistate_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_multistate_item(&mut self, rid: Rid, label: impl AsArg < GString >, max_states: i32, default_state: i32,) -> i32 {
            self.add_multistate_item_ex(rid, label, max_states, default_state,) . done()
        }
        #[inline]
        pub fn add_multistate_item_ex < 'a > (&'a mut self, rid: Rid, label: impl AsArg < GString > + 'a, max_states: i32, default_state: i32,) -> ExAddMultistateItem < 'a > {
            ExAddMultistateItem::new(self, rid, label, max_states, default_state,)
        }
        pub(crate) fn add_separator_full(&mut self, rid: Rid, index: i32,) -> i32 {
            type CallSig = (i32, Rid, i32);
            let args = (rid, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "add_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_separator_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_separator(&mut self, rid: Rid,) -> i32 {
            self.add_separator_ex(rid,) . done()
        }
        #[inline]
        pub fn add_separator_ex < 'a > (&'a mut self, rid: Rid,) -> ExAddSeparator < 'a > {
            ExAddSeparator::new(self, rid,)
        }
        pub fn find_item_index_with_text(&self, rid: Rid, text: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, > = (i32, Rid, CowArg < 'a0, GString >);
            let args = (rid, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_item_index_with_tag(&self, rid: Rid, tag: &Variant,) -> i32 {
            type CallSig < 'a0, > = (i32, Rid, RefArg < 'a0, Variant >);
            let args = (rid, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_item_index_with_submenu(&self, rid: Rid, submenu_rid: Rid,) -> i32 {
            type CallSig = (i32, Rid, Rid);
            let args = (rid, submenu_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_checked(&self, rid: Rid, idx: i32,) -> bool {
            type CallSig = (bool, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_checkable(&self, rid: Rid, idx: i32,) -> bool {
            type CallSig = (bool, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_radio_checkable(&self, rid: Rid, idx: i32,) -> bool {
            type CallSig = (bool, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_callback(&self, rid: Rid, idx: i32,) -> Callable {
            type CallSig = (Callable, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_key_callback(&self, rid: Rid, idx: i32,) -> Callable {
            type CallSig = (Callable, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_key_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_tag(&self, rid: Rid, idx: i32,) -> Variant {
            type CallSig = (Variant, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text(&self, rid: Rid, idx: i32,) -> GString {
            type CallSig = (GString, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_submenu(&self, rid: Rid, idx: i32,) -> Rid {
            type CallSig = (Rid, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_accelerator(&self, rid: Rid, idx: i32,) -> crate::global::Key {
            type CallSig = (crate::global::Key, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_disabled(&self, rid: Rid, idx: i32,) -> bool {
            type CallSig = (bool, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_hidden(&self, rid: Rid, idx: i32,) -> bool {
            type CallSig = (bool, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_tooltip(&self, rid: Rid, idx: i32,) -> GString {
            type CallSig = (GString, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_state(&self, rid: Rid, idx: i32,) -> i32 {
            type CallSig = (i32, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_max_states(&self, rid: Rid, idx: i32,) -> i32 {
            type CallSig = (i32, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_max_states", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon(&self, rid: Rid, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_indentation_level(&self, rid: Rid, idx: i32,) -> i32 {
            type CallSig = (i32, Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_indentation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_checked(&mut self, rid: Rid, idx: i32, checked: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (rid, idx, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_checkable(&mut self, rid: Rid, idx: i32, checkable: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (rid, idx, checkable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_radio_checkable(&mut self, rid: Rid, idx: i32, checkable: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (rid, idx, checkable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_callback(&mut self, rid: Rid, idx: i32, callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, i32, RefArg < 'a0, Callable >);
            let args = (rid, idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_hover_callbacks(&mut self, rid: Rid, idx: i32, callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, i32, RefArg < 'a0, Callable >);
            let args = (rid, idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_hover_callbacks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_key_callback(&mut self, rid: Rid, idx: i32, key_callback: &Callable,) {
            type CallSig < 'a0, > = ((), Rid, i32, RefArg < 'a0, Callable >);
            let args = (rid, idx, RefArg::new(key_callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_key_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_tag(&mut self, rid: Rid, idx: i32, tag: &Variant,) {
            type CallSig < 'a0, > = ((), Rid, i32, RefArg < 'a0, Variant >);
            let args = (rid, idx, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_text(&mut self, rid: Rid, idx: i32, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), Rid, i32, CowArg < 'a0, GString >);
            let args = (rid, idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_submenu(&mut self, rid: Rid, idx: i32, submenu_rid: Rid,) {
            type CallSig = ((), Rid, i32, Rid);
            let args = (rid, idx, submenu_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_accelerator(&mut self, rid: Rid, idx: i32, keycode: crate::global::Key,) {
            type CallSig = ((), Rid, i32, crate::global::Key);
            let args = (rid, idx, keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_disabled(&mut self, rid: Rid, idx: i32, disabled: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (rid, idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_hidden(&mut self, rid: Rid, idx: i32, hidden: bool,) {
            type CallSig = ((), Rid, i32, bool);
            let args = (rid, idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, rid: Rid, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), Rid, i32, CowArg < 'a0, GString >);
            let args = (rid, idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_state(&mut self, rid: Rid, idx: i32, state: i32,) {
            type CallSig = ((), Rid, i32, i32);
            let args = (rid, idx, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_max_states(&mut self, rid: Rid, idx: i32, max_states: i32,) {
            type CallSig = ((), Rid, i32, i32);
            let args = (rid, idx, max_states,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_max_states", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon(&mut self, rid: Rid, idx: i32, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), Rid, i32, ObjectArg < crate::classes::Texture2D >);
            let args = (rid, idx, icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_indentation_level(&mut self, rid: Rid, idx: i32, level: i32,) {
            type CallSig = ((), Rid, i32, i32);
            let args = (rid, idx, level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_indentation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_count(&self, rid: Rid,) -> i32 {
            type CallSig = (i32, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_system_menu(&self, rid: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "is_system_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_item(&mut self, rid: Rid, idx: i32,) {
            type CallSig = ((), Rid, i32);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NativeMenu", "clear", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for NativeMenu {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"NativeMenu"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for NativeMenu {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for NativeMenu {
        
    }
    impl std::ops::Deref for NativeMenu {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for NativeMenu {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`NativeMenu`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_NativeMenu {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::NativeMenu > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_submenu_item_ex`][super::NativeMenu::add_submenu_item_ex]."]
#[must_use]
pub struct ExAddSubmenuItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, submenu_rid: Rid, tag: CowArg < 'a, Variant >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSubmenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a, submenu_rid: Rid,) -> Self {
        let tag = Variant::nil();
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), submenu_rid: submenu_rid, tag: CowArg::Owned(tag), index: index,
        }
    }
    #[inline]
    pub fn tag(self, tag: &'a Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
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
            _phantom, surround_object, rid, label, submenu_rid, tag, index,
        }
        = self;
        re_export::NativeMenu::add_submenu_item_full(surround_object, rid, label, submenu_rid, tag.cow_as_arg(), index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_item_ex`][super::NativeMenu::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_check_item_ex`][super::NativeMenu::add_check_item_ex]."]
#[must_use]
pub struct ExAddCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_check_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_item_ex`][super::NativeMenu::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.consume_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_item_full(surround_object, rid, icon.cow_as_object_arg(), label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_check_item_ex`][super::NativeMenu::add_icon_check_item_ex]."]
#[must_use]
pub struct ExAddIconCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.consume_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_check_item_full(surround_object, rid, icon.cow_as_object_arg(), label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_radio_check_item_ex`][super::NativeMenu::add_radio_check_item_ex]."]
#[must_use]
pub struct ExAddRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_radio_check_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_radio_check_item_ex`][super::NativeMenu::add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, icon: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.consume_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_radio_check_item_full(surround_object, rid, icon.cow_as_object_arg(), label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_multistate_item_ex`][super::NativeMenu::add_multistate_item_ex]."]
#[must_use]
pub struct ExAddMultistateItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: CowArg < 'a, GString >, max_states: i32, default_state: i32, callback: CowArg < 'a, Callable >, key_callback: CowArg < 'a, Callable >, tag: CowArg < 'a, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddMultistateItem < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'a, max_states: i32, default_state: i32,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), max_states: max_states, default_state: default_state, callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, label, max_states, default_state, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_multistate_item_full(surround_object, rid, label, max_states, default_state, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_separator_ex`][super::NativeMenu::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::NativeMenu, rid: Rid, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::NativeMenu, rid: Rid,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, index: index,
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
            _phantom, surround_object, rid, index,
        }
        = self;
        re_export::NativeMenu::add_separator_full(surround_object, rid, index,)
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
    #[doc(alias = "FEATURE_POPUP_MENU")]
    #[doc = "Godot enumerator name: `FEATURE_POPUP_MENU`"]
    pub const POPUP_MENU: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_OPEN_CLOSE_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_OPEN_CLOSE_CALLBACK`"]
    pub const OPEN_CLOSE_CALLBACK: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_HOVER_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_HOVER_CALLBACK`"]
    pub const HOVER_CALLBACK: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_KEY_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_KEY_CALLBACK`"]
    pub const KEY_CALLBACK: Feature = Feature {
        ord: 4i32
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
            Self::GLOBAL_MENU => "GLOBAL_MENU", Self::POPUP_MENU => "POPUP_MENU", Self::OPEN_CLOSE_CALLBACK => "OPEN_CLOSE_CALLBACK", Self::HOVER_CALLBACK => "HOVER_CALLBACK", Self::KEY_CALLBACK => "KEY_CALLBACK", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GLOBAL_MENU => "FEATURE_GLOBAL_MENU", Self::POPUP_MENU => "FEATURE_POPUP_MENU", Self::OPEN_CLOSE_CALLBACK => "FEATURE_OPEN_CLOSE_CALLBACK", Self::HOVER_CALLBACK => "FEATURE_HOVER_CALLBACK", Self::KEY_CALLBACK => "FEATURE_KEY_CALLBACK", _ => self.as_str(),
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
pub struct SystemMenus {
    ord: i32
}
impl SystemMenus {
    pub const INVALID_MENU_ID: SystemMenus = SystemMenus {
        ord: 0i32
    };
    pub const MAIN_MENU_ID: SystemMenus = SystemMenus {
        ord: 1i32
    };
    pub const APPLICATION_MENU_ID: SystemMenus = SystemMenus {
        ord: 2i32
    };
    pub const WINDOW_MENU_ID: SystemMenus = SystemMenus {
        ord: 3i32
    };
    pub const HELP_MENU_ID: SystemMenus = SystemMenus {
        ord: 4i32
    };
    pub const DOCK_MENU_ID: SystemMenus = SystemMenus {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SystemMenus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SystemMenus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SystemMenus {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::INVALID_MENU_ID => "INVALID_MENU_ID", Self::MAIN_MENU_ID => "MAIN_MENU_ID", Self::APPLICATION_MENU_ID => "APPLICATION_MENU_ID", Self::WINDOW_MENU_ID => "WINDOW_MENU_ID", Self::HELP_MENU_ID => "HELP_MENU_ID", Self::DOCK_MENU_ID => "DOCK_MENU_ID", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        self.as_str()
    }
}
impl crate::meta::GodotConvert for SystemMenus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SystemMenus {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SystemMenus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}