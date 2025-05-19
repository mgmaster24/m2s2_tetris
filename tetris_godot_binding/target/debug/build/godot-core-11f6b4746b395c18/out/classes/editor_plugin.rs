#![doc = "Sidecar module for class [`EditorPlugin`][crate::classes::EditorPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorPlugin.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`editor_plugin`][crate::classes::editor_plugin]: sidecar module with related enum/flag types\n* [`IEditorPlugin`][crate::classes::IEditorPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorPlugin::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorPlugin`][crate::classes::EditorPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorPlugin: crate::obj::GodotClass < Base = EditorPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn forward_canvas_gui_input(&mut self, event: Option < Gd < crate::classes::InputEvent > >,) -> bool {
            unimplemented !()
        }
        fn forward_canvas_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        fn forward_canvas_force_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        fn forward_3d_gui_input(&mut self, viewport_camera: Option < Gd < crate::classes::Camera3D > >, event: Option < Gd < crate::classes::InputEvent > >,) -> i32 {
            unimplemented !()
        }
        fn forward_3d_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        fn forward_3d_force_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        fn get_plugin_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_plugin_icon(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            unimplemented !()
        }
        fn has_main_screen(&self,) -> bool {
            unimplemented !()
        }
        fn make_visible(&mut self, visible: bool,) {
            unimplemented !()
        }
        fn edit(&mut self, object: Option < Gd < crate::classes::Object > >,) {
            unimplemented !()
        }
        fn handles(&self, object: Gd < crate::classes::Object >,) -> bool {
            unimplemented !()
        }
        fn get_state(&self,) -> Dictionary {
            unimplemented !()
        }
        fn set_state(&mut self, state: Dictionary,) {
            unimplemented !()
        }
        fn clear(&mut self,) {
            unimplemented !()
        }
        fn get_unsaved_status(&self, for_scene: GString,) -> GString {
            unimplemented !()
        }
        fn save_external_data(&mut self,) {
            unimplemented !()
        }
        fn apply_changes(&mut self,) {
            unimplemented !()
        }
        fn get_breakpoints(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn set_window_layout(&mut self, configuration: Option < Gd < crate::classes::ConfigFile > >,) {
            unimplemented !()
        }
        fn get_window_layout(&mut self, configuration: Option < Gd < crate::classes::ConfigFile > >,) {
            unimplemented !()
        }
        fn build(&mut self,) -> bool {
            unimplemented !()
        }
        fn enable_plugin(&mut self,) {
            unimplemented !()
        }
        fn disable_plugin(&mut self,) {
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
    impl EditorPlugin {
        pub fn add_control_to_container(&mut self, container: crate::classes::editor_plugin::CustomControlContainer, control: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig = ((), crate::classes::editor_plugin::CustomControlContainer, ObjectArg < crate::classes::Control >);
            let args = (container, control.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_container", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_control_to_bottom_panel_full(&mut self, control: ObjectArg < crate::classes::Control >, title: CowArg < GString >, shortcut: ObjectArg < crate::classes::Shortcut >,) -> Option < Gd < crate::classes::Button > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Button > >, ObjectArg < crate::classes::Control >, CowArg < 'a0, GString >, ObjectArg < crate::classes::Shortcut >);
            let args = (control, title, shortcut,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_bottom_panel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_control_to_bottom_panel_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_control_to_bottom_panel(&mut self, control: impl AsObjectArg < crate::classes::Control >, title: impl AsArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            self.add_control_to_bottom_panel_ex(control, title,) . done()
        }
        #[inline]
        pub fn add_control_to_bottom_panel_ex < 'a > (&'a mut self, control: impl AsObjectArg < crate::classes::Control >, title: impl AsArg < GString > + 'a,) -> ExAddControlToBottomPanel < 'a > {
            ExAddControlToBottomPanel::new(self, control, title,)
        }
        pub(crate) fn add_control_to_dock_full(&mut self, slot: crate::classes::editor_plugin::DockSlot, control: ObjectArg < crate::classes::Control >, shortcut: ObjectArg < crate::classes::Shortcut >,) {
            type CallSig = ((), crate::classes::editor_plugin::DockSlot, ObjectArg < crate::classes::Control >, ObjectArg < crate::classes::Shortcut >);
            let args = (slot, control, shortcut,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_dock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_control_to_dock_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_control_to_dock(&mut self, slot: crate::classes::editor_plugin::DockSlot, control: impl AsObjectArg < crate::classes::Control >,) {
            self.add_control_to_dock_ex(slot, control,) . done()
        }
        #[inline]
        pub fn add_control_to_dock_ex < 'a > (&'a mut self, slot: crate::classes::editor_plugin::DockSlot, control: impl AsObjectArg < crate::classes::Control >,) -> ExAddControlToDock < 'a > {
            ExAddControlToDock::new(self, slot, control,)
        }
        pub fn remove_control_from_docks(&mut self, control: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig = ((), ObjectArg < crate::classes::Control >);
            let args = (control.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_docks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_control_from_bottom_panel(&mut self, control: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig = ((), ObjectArg < crate::classes::Control >);
            let args = (control.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_bottom_panel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_control_from_container(&mut self, container: crate::classes::editor_plugin::CustomControlContainer, control: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig = ((), crate::classes::editor_plugin::CustomControlContainer, ObjectArg < crate::classes::Control >);
            let args = (container, control.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_container", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dock_tab_icon(&mut self, control: impl AsObjectArg < crate::classes::Control >, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Control >, ObjectArg < crate::classes::Texture2D >);
            let args = (control.as_object_arg(), icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_dock_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_tool_menu_item(&mut self, name: impl AsArg < GString >, callable: &Callable,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, Callable >);
            let args = (name.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_tool_menu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_tool_submenu_item(&mut self, name: impl AsArg < GString >, submenu: impl AsObjectArg < crate::classes::PopupMenu >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, ObjectArg < crate::classes::PopupMenu >);
            let args = (name.into_arg(), submenu.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_tool_submenu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_tool_menu_item(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_tool_menu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_export_as_menu(&mut self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallSig = (Option < Gd < crate::classes::PopupMenu > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_export_as_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_custom_type(&mut self, type_: impl AsArg < GString >, base: impl AsArg < GString >, script: impl AsObjectArg < crate::classes::Script >, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >, ObjectArg < crate::classes::Script >, ObjectArg < crate::classes::Texture2D >);
            let args = (type_.into_arg(), base.into_arg(), script.as_object_arg(), icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_custom_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_custom_type(&mut self, type_: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_custom_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_autoload_singleton(&mut self, name: impl AsArg < GString >, path: impl AsArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (name.into_arg(), path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_autoload_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_autoload_singleton(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_autoload_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_overlays(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "update_overlays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_bottom_panel_item_visible(&mut self, item: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig = ((), ObjectArg < crate::classes::Control >);
            let args = (item.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "make_bottom_panel_item_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hide_bottom_panel(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "hide_bottom_panel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_undo_redo(&mut self,) -> Option < Gd < crate::classes::EditorUndoRedoManager > > {
            type CallSig = (Option < Gd < crate::classes::EditorUndoRedoManager > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_undo_redo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_redo_inspector_hook_callback(&mut self, callable: &Callable,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_undo_redo_inspector_hook_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_undo_redo_inspector_hook_callback(&mut self, callable: &Callable,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_undo_redo_inspector_hook_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn queue_save_layout(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "queue_save_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_translation_parser_plugin(&mut self, parser: impl AsObjectArg < crate::classes::EditorTranslationParserPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorTranslationParserPlugin >);
            let args = (parser.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_translation_parser_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_translation_parser_plugin(&mut self, parser: impl AsObjectArg < crate::classes::EditorTranslationParserPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorTranslationParserPlugin >);
            let args = (parser.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_translation_parser_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_import_plugin_full(&mut self, importer: ObjectArg < crate::classes::EditorImportPlugin >, first_priority: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorImportPlugin >, bool);
            let args = (importer, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_import_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_import_plugin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_import_plugin(&mut self, importer: impl AsObjectArg < crate::classes::EditorImportPlugin >,) {
            self.add_import_plugin_ex(importer,) . done()
        }
        #[inline]
        pub fn add_import_plugin_ex < 'a > (&'a mut self, importer: impl AsObjectArg < crate::classes::EditorImportPlugin >,) -> ExAddImportPlugin < 'a > {
            ExAddImportPlugin::new(self, importer,)
        }
        pub fn remove_import_plugin(&mut self, importer: impl AsObjectArg < crate::classes::EditorImportPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorImportPlugin >);
            let args = (importer.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_import_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_scene_format_importer_plugin_full(&mut self, scene_format_importer: ObjectArg < crate::classes::EditorSceneFormatImporter >, first_priority: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorSceneFormatImporter >, bool);
            let args = (scene_format_importer, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_scene_format_importer_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_scene_format_importer_plugin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_scene_format_importer_plugin(&mut self, scene_format_importer: impl AsObjectArg < crate::classes::EditorSceneFormatImporter >,) {
            self.add_scene_format_importer_plugin_ex(scene_format_importer,) . done()
        }
        #[inline]
        pub fn add_scene_format_importer_plugin_ex < 'a > (&'a mut self, scene_format_importer: impl AsObjectArg < crate::classes::EditorSceneFormatImporter >,) -> ExAddSceneFormatImporterPlugin < 'a > {
            ExAddSceneFormatImporterPlugin::new(self, scene_format_importer,)
        }
        pub fn remove_scene_format_importer_plugin(&mut self, scene_format_importer: impl AsObjectArg < crate::classes::EditorSceneFormatImporter >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorSceneFormatImporter >);
            let args = (scene_format_importer.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_scene_format_importer_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_scene_post_import_plugin_full(&mut self, scene_import_plugin: ObjectArg < crate::classes::EditorScenePostImportPlugin >, first_priority: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorScenePostImportPlugin >, bool);
            let args = (scene_import_plugin, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_scene_post_import_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_scene_post_import_plugin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_scene_post_import_plugin(&mut self, scene_import_plugin: impl AsObjectArg < crate::classes::EditorScenePostImportPlugin >,) {
            self.add_scene_post_import_plugin_ex(scene_import_plugin,) . done()
        }
        #[inline]
        pub fn add_scene_post_import_plugin_ex < 'a > (&'a mut self, scene_import_plugin: impl AsObjectArg < crate::classes::EditorScenePostImportPlugin >,) -> ExAddScenePostImportPlugin < 'a > {
            ExAddScenePostImportPlugin::new(self, scene_import_plugin,)
        }
        pub fn remove_scene_post_import_plugin(&mut self, scene_import_plugin: impl AsObjectArg < crate::classes::EditorScenePostImportPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorScenePostImportPlugin >);
            let args = (scene_import_plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_scene_post_import_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_export_plugin(&mut self, plugin: impl AsObjectArg < crate::classes::EditorExportPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorExportPlugin >);
            let args = (plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_export_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_export_plugin(&mut self, plugin: impl AsObjectArg < crate::classes::EditorExportPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorExportPlugin >);
            let args = (plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_export_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_node_3d_gizmo_plugin(&mut self, plugin: impl AsObjectArg < crate::classes::EditorNode3DGizmoPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorNode3DGizmoPlugin >);
            let args = (plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_node_3d_gizmo_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_node_3d_gizmo_plugin(&mut self, plugin: impl AsObjectArg < crate::classes::EditorNode3DGizmoPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorNode3DGizmoPlugin >);
            let args = (plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_node_3d_gizmo_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_inspector_plugin(&mut self, plugin: impl AsObjectArg < crate::classes::EditorInspectorPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorInspectorPlugin >);
            let args = (plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_inspector_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_inspector_plugin(&mut self, plugin: impl AsObjectArg < crate::classes::EditorInspectorPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorInspectorPlugin >);
            let args = (plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_inspector_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_resource_conversion_plugin(&mut self, plugin: impl AsObjectArg < crate::classes::EditorResourceConversionPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorResourceConversionPlugin >);
            let args = (plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_resource_conversion_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_resource_conversion_plugin(&mut self, plugin: impl AsObjectArg < crate::classes::EditorResourceConversionPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorResourceConversionPlugin >);
            let args = (plugin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_resource_conversion_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_event_forwarding_always_enabled(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_input_event_forwarding_always_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_force_draw_over_forwarding_enabled(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_force_draw_over_forwarding_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_interface(&mut self,) -> Option < Gd < crate::classes::EditorInterface > > {
            type CallSig = (Option < Gd < crate::classes::EditorInterface > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_editor_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_create_dialog(&mut self,) -> Option < Gd < crate::classes::ScriptCreateDialog > > {
            type CallSig = (Option < Gd < crate::classes::ScriptCreateDialog > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_script_create_dialog", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_debugger_plugin(&mut self, script: impl AsObjectArg < crate::classes::EditorDebuggerPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorDebuggerPlugin >);
            let args = (script.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_debugger_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_debugger_plugin(&mut self, script: impl AsObjectArg < crate::classes::EditorDebuggerPlugin >,) {
            type CallSig = ((), ObjectArg < crate::classes::EditorDebuggerPlugin >);
            let args = (script.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_debugger_plugin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_plugin_version(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_plugin_version", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorPlugin {
        type Base = crate::classes::Node;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"EditorPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorPlugin {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorPlugin {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_EditorPlugin {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_control_to_bottom_panel_ex`][super::EditorPlugin::add_control_to_bottom_panel_ex]."]
#[must_use]
pub struct ExAddControlToBottomPanel < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, control: ObjectCow < crate::classes::Control >, title: CowArg < 'a, GString >, shortcut: ObjectCow < crate::classes::Shortcut >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddControlToBottomPanel < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, control: impl AsObjectArg < crate::classes::Control >, title: impl AsArg < GString > + 'a,) -> Self {
        let shortcut = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, control: control.consume_arg(), title: title.into_arg(), shortcut: shortcut.consume_arg(),
        }
    }
    #[inline]
    pub fn shortcut(self, shortcut: impl AsObjectArg < crate::classes::Shortcut >) -> Self {
        Self {
            shortcut: shortcut.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Button > > {
        let Self {
            _phantom, surround_object, control, title, shortcut,
        }
        = self;
        re_export::EditorPlugin::add_control_to_bottom_panel_full(surround_object, control.cow_as_object_arg(), title, shortcut.cow_as_object_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_control_to_dock_ex`][super::EditorPlugin::add_control_to_dock_ex]."]
#[must_use]
pub struct ExAddControlToDock < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, slot: crate::classes::editor_plugin::DockSlot, control: ObjectCow < crate::classes::Control >, shortcut: ObjectCow < crate::classes::Shortcut >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddControlToDock < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, slot: crate::classes::editor_plugin::DockSlot, control: impl AsObjectArg < crate::classes::Control >,) -> Self {
        let shortcut = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, slot: slot, control: control.consume_arg(), shortcut: shortcut.consume_arg(),
        }
    }
    #[inline]
    pub fn shortcut(self, shortcut: impl AsObjectArg < crate::classes::Shortcut >) -> Self {
        Self {
            shortcut: shortcut.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, slot, control, shortcut,
        }
        = self;
        re_export::EditorPlugin::add_control_to_dock_full(surround_object, slot, control.cow_as_object_arg(), shortcut.cow_as_object_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_import_plugin_ex`][super::EditorPlugin::add_import_plugin_ex]."]
#[must_use]
pub struct ExAddImportPlugin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, importer: ObjectCow < crate::classes::EditorImportPlugin >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddImportPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, importer: impl AsObjectArg < crate::classes::EditorImportPlugin >,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, importer: importer.consume_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, importer, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_import_plugin_full(surround_object, importer.cow_as_object_arg(), first_priority,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_scene_format_importer_plugin_ex`][super::EditorPlugin::add_scene_format_importer_plugin_ex]."]
#[must_use]
pub struct ExAddSceneFormatImporterPlugin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, scene_format_importer: ObjectCow < crate::classes::EditorSceneFormatImporter >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSceneFormatImporterPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, scene_format_importer: impl AsObjectArg < crate::classes::EditorSceneFormatImporter >,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scene_format_importer: scene_format_importer.consume_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, scene_format_importer, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_scene_format_importer_plugin_full(surround_object, scene_format_importer.cow_as_object_arg(), first_priority,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_scene_post_import_plugin_ex`][super::EditorPlugin::add_scene_post_import_plugin_ex]."]
#[must_use]
pub struct ExAddScenePostImportPlugin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, scene_import_plugin: ObjectCow < crate::classes::EditorScenePostImportPlugin >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddScenePostImportPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, scene_import_plugin: impl AsObjectArg < crate::classes::EditorScenePostImportPlugin >,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scene_import_plugin: scene_import_plugin.consume_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, scene_import_plugin, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_scene_post_import_plugin_full(surround_object, scene_import_plugin.cow_as_object_arg(), first_priority,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CustomControlContainer {
    ord: i32
}
impl CustomControlContainer {
    #[doc(alias = "CONTAINER_TOOLBAR")]
    #[doc = "Godot enumerator name: `CONTAINER_TOOLBAR`"]
    pub const TOOLBAR: CustomControlContainer = CustomControlContainer {
        ord: 0i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_MENU")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_MENU`"]
    pub const SPATIAL_EDITOR_MENU: CustomControlContainer = CustomControlContainer {
        ord: 1i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_SIDE_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_SIDE_LEFT`"]
    pub const SPATIAL_EDITOR_SIDE_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 2i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT`"]
    pub const SPATIAL_EDITOR_SIDE_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 3i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_BOTTOM`"]
    pub const SPATIAL_EDITOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 4i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_MENU")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_MENU`"]
    pub const CANVAS_EDITOR_MENU: CustomControlContainer = CustomControlContainer {
        ord: 5i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_SIDE_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_SIDE_LEFT`"]
    pub const CANVAS_EDITOR_SIDE_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 6i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_SIDE_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_SIDE_RIGHT`"]
    pub const CANVAS_EDITOR_SIDE_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 7i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_BOTTOM`"]
    pub const CANVAS_EDITOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 8i32
    };
    #[doc(alias = "CONTAINER_INSPECTOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_INSPECTOR_BOTTOM`"]
    pub const INSPECTOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 9i32
    };
    #[doc(alias = "CONTAINER_PROJECT_SETTING_TAB_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_PROJECT_SETTING_TAB_LEFT`"]
    pub const PROJECT_SETTING_TAB_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 10i32
    };
    #[doc(alias = "CONTAINER_PROJECT_SETTING_TAB_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_PROJECT_SETTING_TAB_RIGHT`"]
    pub const PROJECT_SETTING_TAB_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for CustomControlContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CustomControlContainer") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CustomControlContainer {
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
            Self::TOOLBAR => "TOOLBAR", Self::SPATIAL_EDITOR_MENU => "SPATIAL_EDITOR_MENU", Self::SPATIAL_EDITOR_SIDE_LEFT => "SPATIAL_EDITOR_SIDE_LEFT", Self::SPATIAL_EDITOR_SIDE_RIGHT => "SPATIAL_EDITOR_SIDE_RIGHT", Self::SPATIAL_EDITOR_BOTTOM => "SPATIAL_EDITOR_BOTTOM", Self::CANVAS_EDITOR_MENU => "CANVAS_EDITOR_MENU", Self::CANVAS_EDITOR_SIDE_LEFT => "CANVAS_EDITOR_SIDE_LEFT", Self::CANVAS_EDITOR_SIDE_RIGHT => "CANVAS_EDITOR_SIDE_RIGHT", Self::CANVAS_EDITOR_BOTTOM => "CANVAS_EDITOR_BOTTOM", Self::INSPECTOR_BOTTOM => "INSPECTOR_BOTTOM", Self::PROJECT_SETTING_TAB_LEFT => "PROJECT_SETTING_TAB_LEFT", Self::PROJECT_SETTING_TAB_RIGHT => "PROJECT_SETTING_TAB_RIGHT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TOOLBAR => "CONTAINER_TOOLBAR", Self::SPATIAL_EDITOR_MENU => "CONTAINER_SPATIAL_EDITOR_MENU", Self::SPATIAL_EDITOR_SIDE_LEFT => "CONTAINER_SPATIAL_EDITOR_SIDE_LEFT", Self::SPATIAL_EDITOR_SIDE_RIGHT => "CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT", Self::SPATIAL_EDITOR_BOTTOM => "CONTAINER_SPATIAL_EDITOR_BOTTOM", Self::CANVAS_EDITOR_MENU => "CONTAINER_CANVAS_EDITOR_MENU", Self::CANVAS_EDITOR_SIDE_LEFT => "CONTAINER_CANVAS_EDITOR_SIDE_LEFT", Self::CANVAS_EDITOR_SIDE_RIGHT => "CONTAINER_CANVAS_EDITOR_SIDE_RIGHT", Self::CANVAS_EDITOR_BOTTOM => "CONTAINER_CANVAS_EDITOR_BOTTOM", Self::INSPECTOR_BOTTOM => "CONTAINER_INSPECTOR_BOTTOM", Self::PROJECT_SETTING_TAB_LEFT => "CONTAINER_PROJECT_SETTING_TAB_LEFT", Self::PROJECT_SETTING_TAB_RIGHT => "CONTAINER_PROJECT_SETTING_TAB_RIGHT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CustomControlContainer {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CustomControlContainer {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CustomControlContainer {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DockSlot {
    ord: i32
}
impl DockSlot {
    #[doc(alias = "DOCK_SLOT_LEFT_UL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_UL`"]
    pub const LEFT_UL: DockSlot = DockSlot {
        ord: 0i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_BL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_BL`"]
    pub const LEFT_BL: DockSlot = DockSlot {
        ord: 1i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_UR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_UR`"]
    pub const LEFT_UR: DockSlot = DockSlot {
        ord: 2i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_BR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_BR`"]
    pub const LEFT_BR: DockSlot = DockSlot {
        ord: 3i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_UL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_UL`"]
    pub const RIGHT_UL: DockSlot = DockSlot {
        ord: 4i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_BL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_BL`"]
    pub const RIGHT_BL: DockSlot = DockSlot {
        ord: 5i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_UR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_UR`"]
    pub const RIGHT_UR: DockSlot = DockSlot {
        ord: 6i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_BR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_BR`"]
    pub const RIGHT_BR: DockSlot = DockSlot {
        ord: 7i32
    };
    #[doc(alias = "DOCK_SLOT_MAX")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_MAX`"]
    pub const MAX: DockSlot = DockSlot {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for DockSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DockSlot") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DockSlot {
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
            Self::LEFT_UL => "LEFT_UL", Self::LEFT_BL => "LEFT_BL", Self::LEFT_UR => "LEFT_UR", Self::LEFT_BR => "LEFT_BR", Self::RIGHT_UL => "RIGHT_UL", Self::RIGHT_BL => "RIGHT_BL", Self::RIGHT_UR => "RIGHT_UR", Self::RIGHT_BR => "RIGHT_BR", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LEFT_UL => "DOCK_SLOT_LEFT_UL", Self::LEFT_BL => "DOCK_SLOT_LEFT_BL", Self::LEFT_UR => "DOCK_SLOT_LEFT_UR", Self::LEFT_BR => "DOCK_SLOT_LEFT_BR", Self::RIGHT_UL => "DOCK_SLOT_RIGHT_UL", Self::RIGHT_BL => "DOCK_SLOT_RIGHT_BL", Self::RIGHT_UR => "DOCK_SLOT_RIGHT_UR", Self::RIGHT_BR => "DOCK_SLOT_RIGHT_BR", Self::MAX => "DOCK_SLOT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for DockSlot {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for DockSlot {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DockSlot {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DockSlot {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `AfterGUIInput`."]
pub struct AfterGuiInput {
    ord: i32
}
impl AfterGuiInput {
    #[doc(alias = "AFTER_GUI_INPUT_PASS")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_PASS`"]
    pub const PASS: AfterGuiInput = AfterGuiInput {
        ord: 0i32
    };
    #[doc(alias = "AFTER_GUI_INPUT_STOP")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_STOP`"]
    pub const STOP: AfterGuiInput = AfterGuiInput {
        ord: 1i32
    };
    #[doc(alias = "AFTER_GUI_INPUT_CUSTOM")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_CUSTOM`"]
    pub const CUSTOM: AfterGuiInput = AfterGuiInput {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AfterGuiInput {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AfterGuiInput") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AfterGuiInput {
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
            Self::PASS => "PASS", Self::STOP => "STOP", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PASS => "AFTER_GUI_INPUT_PASS", Self::STOP => "AFTER_GUI_INPUT_STOP", Self::CUSTOM => "AFTER_GUI_INPUT_CUSTOM", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AfterGuiInput {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AfterGuiInput {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AfterGuiInput {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}