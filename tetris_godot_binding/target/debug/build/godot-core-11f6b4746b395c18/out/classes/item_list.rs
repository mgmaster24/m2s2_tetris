#![doc = "Sidecar module for class [`ItemList`][crate::classes::ItemList].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ItemList` enums](https://docs.godotengine.org/en/stable/classes/class_itemlist.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ItemList.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`item_list`][crate::classes::item_list]: sidecar module with related enum/flag types\n* [`IItemList`][crate::classes::IItemList]: virtual methods\n\n\nSee also [Godot docs for `ItemList`](https://docs.godotengine.org/en/stable/classes/class_itemlist.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ItemList::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ItemList {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ItemList`][crate::classes::ItemList].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ItemList` methods](https://docs.godotengine.org/en/stable/classes/class_itemlist.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IItemList: crate::obj::GodotClass < Base = ItemList > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl ItemList {
        pub(crate) fn add_item_full(&mut self, text: CowArg < GString >, icon: ObjectArg < crate::classes::Texture2D >, selectable: bool,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >, ObjectArg < crate::classes::Texture2D >, bool);
            let args = (text, icon, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "add_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_item(&mut self, text: impl AsArg < GString >,) -> i32 {
            self.add_item_ex(text,) . done()
        }
        #[inline]
        pub fn add_item_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a,) -> ExAddItem < 'a > {
            ExAddItem::new(self, text,)
        }
        pub(crate) fn add_icon_item_full(&mut self, icon: ObjectArg < crate::classes::Texture2D >, selectable: bool,) -> i32 {
            type CallSig = (i32, ObjectArg < crate::classes::Texture2D >, bool);
            let args = (icon, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "add_icon_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_item(&mut self, icon: impl AsObjectArg < crate::classes::Texture2D >,) -> i32 {
            self.add_icon_item_ex(icon,) . done()
        }
        #[inline]
        pub fn add_icon_item_ex < 'a > (&'a mut self, icon: impl AsObjectArg < crate::classes::Texture2D >,) -> ExAddIconItem < 'a > {
            ExAddIconItem::new(self, icon,)
        }
        pub fn set_item_text(&mut self, idx: i32, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text(&self, idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon(&mut self, idx: i32, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Texture2D >);
            let args = (idx, icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon(&self, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_text_direction(&mut self, idx: i32, direction: crate::classes::control::TextDirection,) {
            type CallSig = ((), i32, crate::classes::control::TextDirection);
            let args = (idx, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text_direction(&self, idx: i32,) -> crate::classes::control::TextDirection {
            type CallSig = (crate::classes::control::TextDirection, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_language(&mut self, idx: i32, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (idx, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_language(&self, idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_transposed(&mut self, idx: i32, transposed: bool,) {
            type CallSig = ((), i32, bool);
            let args = (idx, transposed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_transposed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_icon_transposed(&self, idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "is_item_icon_transposed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_region(&mut self, idx: i32, rect: Rect2,) {
            type CallSig = ((), i32, Rect2);
            let args = (idx, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon_region(&self, idx: i32,) -> Rect2 {
            type CallSig = (Rect2, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_modulate(&mut self, idx: i32, modulate: Color,) {
            type CallSig = ((), i32, Color);
            let args = (idx, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon_modulate(&self, idx: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_selectable(&mut self, idx: i32, selectable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (idx, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_selectable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_selectable(&self, idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "is_item_selectable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_disabled(&mut self, idx: i32, disabled: bool,) {
            type CallSig = ((), i32, bool);
            let args = (idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_disabled(&self, idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_metadata(&mut self, idx: i32, metadata: &Variant,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Variant >);
            let args = (idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_metadata(&self, idx: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_custom_bg_color(&mut self, idx: i32, custom_bg_color: Color,) {
            type CallSig = ((), i32, Color);
            let args = (idx, custom_bg_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_custom_bg_color(&self, idx: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_custom_fg_color(&mut self, idx: i32, custom_fg_color: Color,) {
            type CallSig = ((), i32, Color);
            let args = (idx, custom_fg_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_custom_fg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_custom_fg_color(&self, idx: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_custom_fg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_item_rect_full(&self, idx: i32, expand: bool,) -> Rect2 {
            type CallSig = (Rect2, i32, bool);
            let args = (idx, expand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_item_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_item_rect(&self, idx: i32,) -> Rect2 {
            self.get_item_rect_ex(idx,) . done()
        }
        #[inline]
        pub fn get_item_rect_ex < 'a > (&'a self, idx: i32,) -> ExGetItemRect < 'a > {
            ExGetItemRect::new(self, idx,)
        }
        pub fn set_item_tooltip_enabled(&mut self, idx: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (idx, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_tooltip_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_tooltip_enabled(&self, idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "is_item_tooltip_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_tooltip(&self, idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn select_full(&mut self, idx: i32, single: bool,) {
            type CallSig = ((), i32, bool);
            let args = (idx, single,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::select_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn select(&mut self, idx: i32,) {
            self.select_ex(idx,) . done()
        }
        #[inline]
        pub fn select_ex < 'a > (&'a mut self, idx: i32,) -> ExSelect < 'a > {
            ExSelect::new(self, idx,)
        }
        pub fn deselect(&mut self, idx: i32,) {
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "deselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deselect_all(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "deselect_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selected(&self, idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "is_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_items(&mut self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_selected_items", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_item(&mut self, from_idx: i32, to_idx: i32,) {
            type CallSig = ((), i32, i32);
            let args = (from_idx, to_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "move_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_item(&mut self, idx: i32,) {
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sort_items_by_text(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "sort_items_by_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_column_width(&mut self, width: i32,) {
            type CallSig = ((), i32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_fixed_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_column_width(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_fixed_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_same_column_width(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_same_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_same_column_width(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "is_same_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_text_lines(&mut self, lines: i32,) {
            type CallSig = ((), i32);
            let args = (lines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_max_text_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_text_lines(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_max_text_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_columns(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_max_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_columns(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_max_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_select_mode(&mut self, mode: crate::classes::item_list::SelectMode,) {
            type CallSig = ((), crate::classes::item_list::SelectMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_select_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_select_mode(&self,) -> crate::classes::item_list::SelectMode {
            type CallSig = (crate::classes::item_list::SelectMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_select_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_mode(&mut self, mode: crate::classes::item_list::IconMode,) {
            type CallSig = ((), crate::classes::item_list::IconMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_icon_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_mode(&self,) -> crate::classes::item_list::IconMode {
            type CallSig = (crate::classes::item_list::IconMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_icon_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_icon_size(&mut self, size: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_fixed_icon_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_icon_size(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4514usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_fixed_icon_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_scale(&mut self, scale: f32,) {
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_icon_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_icon_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_rmb_select(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_rmb_select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_rmb_select(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_rmb_select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_height(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_auto_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_auto_height(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "has_auto_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_anything_selected(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "is_anything_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_item_at_position_full(&self, position: Vector2, exact: bool,) -> i32 {
            type CallSig = (i32, Vector2, bool);
            let args = (position, exact,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_item_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_item_at_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_item_at_position(&self, position: Vector2,) -> i32 {
            self.get_item_at_position_ex(position,) . done()
        }
        #[inline]
        pub fn get_item_at_position_ex < 'a > (&'a self, position: Vector2,) -> ExGetItemAtPosition < 'a > {
            ExGetItemAtPosition::new(self, position,)
        }
        pub fn ensure_current_is_visible(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "ensure_current_is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_scroll_bar(&mut self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallSig = (Option < Gd < crate::classes::VScrollBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_v_scroll_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallSig = ((), crate::classes::text_server::OverrunBehavior);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "set_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallSig = (crate::classes::text_server::OverrunBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "get_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_list_size(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ItemList", "force_update_list_size", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ItemList {
        type Base = crate::classes::Control;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ItemList"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ItemList {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ItemList {
        
    }
    impl crate::obj::cap::GodotDefault for ItemList {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ItemList {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ItemList {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ItemList`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ItemList {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ItemList > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ItemList::add_item_ex`][super::ItemList::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ItemList, text: CowArg < 'a, GString >, icon: ObjectCow < crate::classes::Texture2D >, selectable: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, text: impl AsArg < GString > + 'a,) -> Self {
        let icon = Gd::null_arg();
        let selectable = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), icon: icon.consume_arg(), selectable: selectable,
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsObjectArg < crate::classes::Texture2D >) -> Self {
        Self {
            icon: icon.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            selectable: selectable, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, text, icon, selectable,
        }
        = self;
        re_export::ItemList::add_item_full(surround_object, text, icon.cow_as_object_arg(), selectable,)
    }
}
#[doc = "Default-param extender for [`ItemList::add_icon_item_ex`][super::ItemList::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ItemList, icon: ObjectCow < crate::classes::Texture2D >, selectable: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, icon: impl AsObjectArg < crate::classes::Texture2D >,) -> Self {
        let selectable = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, icon: icon.consume_arg(), selectable: selectable,
        }
    }
    #[inline]
    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            selectable: selectable, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, icon, selectable,
        }
        = self;
        re_export::ItemList::add_icon_item_full(surround_object, icon.cow_as_object_arg(), selectable,)
    }
}
#[doc = "Default-param extender for [`ItemList::get_item_rect_ex`][super::ItemList::get_item_rect_ex]."]
#[must_use]
pub struct ExGetItemRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ItemList, idx: i32, expand: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemRect < 'a > {
    fn new(surround_object: &'a re_export::ItemList, idx: i32,) -> Self {
        let expand = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, expand: expand,
        }
    }
    #[inline]
    pub fn expand(self, expand: bool) -> Self {
        Self {
            expand: expand, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2 {
        let Self {
            _phantom, surround_object, idx, expand,
        }
        = self;
        re_export::ItemList::get_item_rect_full(surround_object, idx, expand,)
    }
}
#[doc = "Default-param extender for [`ItemList::select_ex`][super::ItemList::select_ex]."]
#[must_use]
pub struct ExSelect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ItemList, idx: i32, single: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSelect < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, idx: i32,) -> Self {
        let single = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, single: single,
        }
    }
    #[inline]
    pub fn single(self, single: bool) -> Self {
        Self {
            single: single, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, idx, single,
        }
        = self;
        re_export::ItemList::select_full(surround_object, idx, single,)
    }
}
#[doc = "Default-param extender for [`ItemList::get_item_at_position_ex`][super::ItemList::get_item_at_position_ex]."]
#[must_use]
pub struct ExGetItemAtPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ItemList, position: Vector2, exact: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemAtPosition < 'a > {
    fn new(surround_object: &'a re_export::ItemList, position: Vector2,) -> Self {
        let exact = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, exact: exact,
        }
    }
    #[inline]
    pub fn exact(self, exact: bool) -> Self {
        Self {
            exact: exact, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, position, exact,
        }
        = self;
        re_export::ItemList::get_item_at_position_full(surround_object, position, exact,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct IconMode {
    ord: i32
}
impl IconMode {
    #[doc(alias = "ICON_MODE_TOP")]
    #[doc = "Godot enumerator name: `ICON_MODE_TOP`"]
    pub const TOP: IconMode = IconMode {
        ord: 0i32
    };
    #[doc(alias = "ICON_MODE_LEFT")]
    #[doc = "Godot enumerator name: `ICON_MODE_LEFT`"]
    pub const LEFT: IconMode = IconMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for IconMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("IconMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for IconMode {
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
            Self::TOP => "TOP", Self::LEFT => "LEFT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TOP => "ICON_MODE_TOP", Self::LEFT => "ICON_MODE_LEFT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for IconMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for IconMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for IconMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SelectMode {
    ord: i32
}
impl SelectMode {
    #[doc(alias = "SELECT_SINGLE")]
    #[doc = "Godot enumerator name: `SELECT_SINGLE`"]
    pub const SINGLE: SelectMode = SelectMode {
        ord: 0i32
    };
    #[doc(alias = "SELECT_MULTI")]
    #[doc = "Godot enumerator name: `SELECT_MULTI`"]
    pub const MULTI: SelectMode = SelectMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for SelectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SelectMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SelectMode {
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
            Self::SINGLE => "SINGLE", Self::MULTI => "MULTI", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SINGLE => "SELECT_SINGLE", Self::MULTI => "SELECT_MULTI", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SelectMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SelectMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SelectMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}