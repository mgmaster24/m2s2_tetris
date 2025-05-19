#![doc = "Sidecar module for class [`EditorUndoRedoManager`][crate::classes::EditorUndoRedoManager].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorUndoRedoManager` enums](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorUndoRedoManager.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`editor_undo_redo_manager`][crate::classes::editor_undo_redo_manager]: sidecar module with related enum/flag types\n* [`IEditorUndoRedoManager`][crate::classes::IEditorUndoRedoManager]: virtual methods\n\n\nSee also [Godot docs for `EditorUndoRedoManager`](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorUndoRedoManager>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorUndoRedoManager {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorUndoRedoManager`][crate::classes::EditorUndoRedoManager].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorUndoRedoManager` methods](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorUndoRedoManager: crate::obj::GodotClass < Base = EditorUndoRedoManager > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl EditorUndoRedoManager {
        pub(crate) fn create_action_full(&mut self, name: CowArg < GString >, merge_mode: crate::classes::undo_redo::MergeMode, custom_context: ObjectArg < crate::classes::Object >, backward_undo_ops: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, crate::classes::undo_redo::MergeMode, ObjectArg < crate::classes::Object >, bool);
            let args = (name, merge_mode, custom_context, backward_undo_ops,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "create_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_action(&mut self, name: impl AsArg < GString >,) {
            self.create_action_ex(name,) . done()
        }
        #[inline]
        pub fn create_action_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a,) -> ExCreateAction < 'a > {
            ExCreateAction::new(self, name,)
        }
        pub(crate) fn commit_action_full(&mut self, execute: bool,) {
            type CallSig = ((), bool);
            let args = (execute,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "commit_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::commit_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn commit_action(&mut self,) {
            self.commit_action_ex() . done()
        }
        #[inline]
        pub fn commit_action_ex < 'a > (&'a mut self,) -> ExCommitAction < 'a > {
            ExCommitAction::new(self,)
        }
        pub fn is_committing_action(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "is_committing_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_fixed_history(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "force_fixed_history", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn add_do_method(&mut self, object: impl AsObjectArg < crate::classes::Object >, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_add_do_method(self, object, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_add_do_method(&mut self, object: impl AsObjectArg < crate::classes::Object >, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >);
            let args = (object.as_object_arg(), method.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(304usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "EditorUndoRedoManager", "add_do_method", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn add_undo_method(&mut self, object: impl AsObjectArg < crate::classes::Object >, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_add_undo_method(self, object, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_add_undo_method(&mut self, object: impl AsObjectArg < crate::classes::Object >, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >);
            let args = (object.as_object_arg(), method.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(305usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "EditorUndoRedoManager", "add_undo_method", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn add_do_property(&mut self, object: impl AsObjectArg < crate::classes::Object >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (object.as_object_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "add_do_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_property(&mut self, object: impl AsObjectArg < crate::classes::Object >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (object.as_object_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "add_undo_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_reference(&mut self, object: impl AsObjectArg < crate::classes::Object >,) {
            type CallSig = ((), ObjectArg < crate::classes::Object >);
            let args = (object.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "add_do_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_reference(&mut self, object: impl AsObjectArg < crate::classes::Object >,) {
            type CallSig = ((), ObjectArg < crate::classes::Object >);
            let args = (object.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "add_undo_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_object_history_id(&self, object: impl AsObjectArg < crate::classes::Object >,) -> i32 {
            type CallSig = (i32, ObjectArg < crate::classes::Object >);
            let args = (object.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "get_object_history_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_history_undo_redo(&self, id: i32,) -> Option < Gd < crate::classes::UndoRedo > > {
            type CallSig = (Option < Gd < crate::classes::UndoRedo > >, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "get_history_undo_redo", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorUndoRedoManager {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"EditorUndoRedoManager"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorUndoRedoManager {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorUndoRedoManager {
        
    }
    impl std::ops::Deref for EditorUndoRedoManager {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorUndoRedoManager {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorUndoRedoManager`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_EditorUndoRedoManager {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorUndoRedoManager > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorUndoRedoManager::create_action_ex`][super::EditorUndoRedoManager::create_action_ex]."]
#[must_use]
pub struct ExCreateAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorUndoRedoManager, name: CowArg < 'a, GString >, merge_mode: crate::classes::undo_redo::MergeMode, custom_context: ObjectCow < crate::classes::Object >, backward_undo_ops: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateAction < 'a > {
    fn new(surround_object: &'a mut re_export::EditorUndoRedoManager, name: impl AsArg < GString > + 'a,) -> Self {
        let merge_mode = crate::obj::EngineEnum::from_ord(0);
        let custom_context = Gd::null_arg();
        let backward_undo_ops = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), merge_mode: merge_mode, custom_context: custom_context.consume_arg(), backward_undo_ops: backward_undo_ops,
        }
    }
    #[inline]
    pub fn merge_mode(self, merge_mode: crate::classes::undo_redo::MergeMode) -> Self {
        Self {
            merge_mode: merge_mode, .. self
        }
    }
    #[inline]
    pub fn custom_context(self, custom_context: impl AsObjectArg < crate::classes::Object >) -> Self {
        Self {
            custom_context: custom_context.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn backward_undo_ops(self, backward_undo_ops: bool) -> Self {
        Self {
            backward_undo_ops: backward_undo_ops, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, merge_mode, custom_context, backward_undo_ops,
        }
        = self;
        re_export::EditorUndoRedoManager::create_action_full(surround_object, name, merge_mode, custom_context.cow_as_object_arg(), backward_undo_ops,)
    }
}
#[doc = "Default-param extender for [`EditorUndoRedoManager::commit_action_ex`][super::EditorUndoRedoManager::commit_action_ex]."]
#[must_use]
pub struct ExCommitAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorUndoRedoManager, execute: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitAction < 'a > {
    fn new(surround_object: &'a mut re_export::EditorUndoRedoManager,) -> Self {
        let execute = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, execute: execute,
        }
    }
    #[inline]
    pub fn execute(self, execute: bool) -> Self {
        Self {
            execute: execute, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, execute,
        }
        = self;
        re_export::EditorUndoRedoManager::commit_action_full(surround_object, execute,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpecialHistory {
    ord: i32
}
impl SpecialHistory {
    pub const GLOBAL_HISTORY: SpecialHistory = SpecialHistory {
        ord: 0i32
    };
    pub const REMOTE_HISTORY: SpecialHistory = SpecialHistory {
        ord: - 9i32
    };
    pub const INVALID_HISTORY: SpecialHistory = SpecialHistory {
        ord: - 99i32
    };
    
}
impl std::fmt::Debug for SpecialHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpecialHistory") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpecialHistory {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 99i32 | ord @ - 9i32 | ord @ 0i32 => Some(Self {
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
            Self::GLOBAL_HISTORY => "GLOBAL_HISTORY", Self::REMOTE_HISTORY => "REMOTE_HISTORY", Self::INVALID_HISTORY => "INVALID_HISTORY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        self.as_str()
    }
}
impl crate::meta::GodotConvert for SpecialHistory {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpecialHistory {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpecialHistory {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}