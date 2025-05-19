#![doc = "Sidecar module for class [`BaseButton`][crate::classes::BaseButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BaseButton` enums](https://docs.godotengine.org/en/stable/classes/class_basebutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `BaseButton.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`base_button`][crate::classes::base_button]: sidecar module with related enum/flag types\n* [`IBaseButton`][crate::classes::IBaseButton]: virtual methods\n\n\nSee also [Godot docs for `BaseButton`](https://docs.godotengine.org/en/stable/classes/class_basebutton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`BaseButton::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct BaseButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`BaseButton`][crate::classes::BaseButton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `BaseButton` methods](https://docs.godotengine.org/en/stable/classes/class_basebutton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IBaseButton: crate::obj::GodotClass < Base = BaseButton > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn pressed(&mut self,) {
            unimplemented !()
        }
        fn toggled(&mut self, toggled_on: bool,) {
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
    impl BaseButton {
        pub fn set_pressed(&mut self, pressed: bool,) {
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "is_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pressed_no_signal(&mut self, pressed: bool,) {
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_pressed_no_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hovered(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "is_hovered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_toggle_mode(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_toggle_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_toggle_mode(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "is_toggle_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shortcut_in_tooltip(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_shortcut_in_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shortcut_in_tooltip_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "is_shortcut_in_tooltip_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disabled(&mut self, disabled: bool,) {
            type CallSig = ((), bool);
            let args = (disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_disabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "is_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_action_mode(&mut self, mode: crate::classes::base_button::ActionMode,) {
            type CallSig = ((), crate::classes::base_button::ActionMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_action_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action_mode(&self,) -> crate::classes::base_button::ActionMode {
            type CallSig = (crate::classes::base_button::ActionMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "get_action_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_mask(&mut self, mask: crate::global::MouseButtonMask,) {
            type CallSig = ((), crate::global::MouseButtonMask);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_button_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_mask(&self,) -> crate::global::MouseButtonMask {
            type CallSig = (crate::global::MouseButtonMask,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "get_button_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_mode(&self,) -> crate::classes::base_button::DrawMode {
            type CallSig = (crate::classes::base_button::DrawMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "get_draw_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keep_pressed_outside(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_keep_pressed_outside", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_keep_pressed_outside(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "is_keep_pressed_outside", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shortcut_feedback(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_shortcut_feedback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shortcut_feedback(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "is_shortcut_feedback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shortcut(&mut self, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) {
            type CallSig = ((), ObjectArg < crate::classes::Shortcut >);
            let args = (shortcut.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shortcut(&self,) -> Option < Gd < crate::classes::Shortcut > > {
            type CallSig = (Option < Gd < crate::classes::Shortcut > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "get_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_group(&mut self, button_group: impl AsObjectArg < crate::classes::ButtonGroup >,) {
            type CallSig = ((), ObjectArg < crate::classes::ButtonGroup >);
            let args = (button_group.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "set_button_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_group(&self,) -> Option < Gd < crate::classes::ButtonGroup > > {
            type CallSig = (Option < Gd < crate::classes::ButtonGroup > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BaseButton", "get_button_group", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for BaseButton {
        type Base = crate::classes::Control;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"BaseButton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for BaseButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for BaseButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for BaseButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for BaseButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for BaseButton {
        
    }
    impl crate::obj::cap::GodotDefault for BaseButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for BaseButton {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for BaseButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`BaseButton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_BaseButton {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::BaseButton > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DrawMode {
    ord: i32
}
impl DrawMode {
    #[doc(alias = "DRAW_NORMAL")]
    #[doc = "Godot enumerator name: `DRAW_NORMAL`"]
    pub const NORMAL: DrawMode = DrawMode {
        ord: 0i32
    };
    #[doc(alias = "DRAW_PRESSED")]
    #[doc = "Godot enumerator name: `DRAW_PRESSED`"]
    pub const PRESSED: DrawMode = DrawMode {
        ord: 1i32
    };
    #[doc(alias = "DRAW_HOVER")]
    #[doc = "Godot enumerator name: `DRAW_HOVER`"]
    pub const HOVER: DrawMode = DrawMode {
        ord: 2i32
    };
    #[doc(alias = "DRAW_DISABLED")]
    #[doc = "Godot enumerator name: `DRAW_DISABLED`"]
    pub const DISABLED: DrawMode = DrawMode {
        ord: 3i32
    };
    #[doc(alias = "DRAW_HOVER_PRESSED")]
    #[doc = "Godot enumerator name: `DRAW_HOVER_PRESSED`"]
    pub const HOVER_PRESSED: DrawMode = DrawMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DrawMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawMode {
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
            Self::NORMAL => "NORMAL", Self::PRESSED => "PRESSED", Self::HOVER => "HOVER", Self::DISABLED => "DISABLED", Self::HOVER_PRESSED => "HOVER_PRESSED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NORMAL => "DRAW_NORMAL", Self::PRESSED => "DRAW_PRESSED", Self::HOVER => "DRAW_HOVER", Self::DISABLED => "DRAW_DISABLED", Self::HOVER_PRESSED => "DRAW_HOVER_PRESSED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DrawMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DrawMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ActionMode {
    ord: i32
}
impl ActionMode {
    #[doc(alias = "ACTION_MODE_BUTTON_PRESS")]
    #[doc = "Godot enumerator name: `ACTION_MODE_BUTTON_PRESS`"]
    pub const PRESS: ActionMode = ActionMode {
        ord: 0i32
    };
    #[doc(alias = "ACTION_MODE_BUTTON_RELEASE")]
    #[doc = "Godot enumerator name: `ACTION_MODE_BUTTON_RELEASE`"]
    pub const RELEASE: ActionMode = ActionMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ActionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ActionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ActionMode {
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
            Self::PRESS => "PRESS", Self::RELEASE => "RELEASE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PRESS => "ACTION_MODE_BUTTON_PRESS", Self::RELEASE => "ACTION_MODE_BUTTON_RELEASE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ActionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ActionMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ActionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}