#![doc = "Sidecar module for class [`GltfState`][crate::classes::GltfState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFState` enums](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFState.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IGltfState`][crate::classes::IGltfState]: virtual methods\n\n\nSee also [Godot docs for `GLTFState`](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfState::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfState`][crate::classes::GltfState].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFState` methods](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfState: crate::obj::GodotClass < Base = GltfState > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl GltfState {
        pub fn add_used_extension(&mut self, extension_name: impl AsArg < GString >, required: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (extension_name.into_arg(), required,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "add_used_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn append_data_to_buffers(&mut self, data: &PackedByteArray, deduplication: bool,) -> i32 {
            type CallSig < 'a0, > = (i32, RefArg < 'a0, PackedByteArray >, bool);
            let args = (RefArg::new(data), deduplication,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "append_data_to_buffers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_json(&mut self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_json", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_json(&mut self, json: &Dictionary,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(json),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_json", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_major_version(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_major_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_major_version(&mut self, major_version: i32,) {
            type CallSig = ((), i32);
            let args = (major_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_major_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_minor_version(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_minor_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_minor_version(&mut self, minor_version: i32,) {
            type CallSig = ((), i32);
            let args = (minor_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_minor_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_copyright(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_copyright", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_copyright(&mut self, copyright: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (copyright.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_copyright", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glb_data(&mut self,) -> PackedByteArray {
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_glb_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glb_data(&mut self, glb_data: &PackedByteArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedByteArray >);
            let args = (RefArg::new(glb_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_glb_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_named_skin_binds(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_use_named_skin_binds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_named_skin_binds(&mut self, use_named_skin_binds: bool,) {
            type CallSig = ((), bool);
            let args = (use_named_skin_binds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_use_named_skin_binds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_nodes(&mut self,) -> Array < Gd < crate::classes::GltfNode > > {
            type CallSig = (Array < Gd < crate::classes::GltfNode > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_nodes(&mut self, nodes: &Array < Gd < crate::classes::GltfNode > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfNode > > >);
            let args = (RefArg::new(nodes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffers(&mut self,) -> Array < PackedByteArray > {
            type CallSig = (Array < PackedByteArray >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_buffers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_buffers(&mut self, buffers: &Array < PackedByteArray >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < PackedByteArray > >);
            let args = (RefArg::new(buffers),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_buffers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffer_views(&mut self,) -> Array < Gd < crate::classes::GltfBufferView > > {
            type CallSig = (Array < Gd < crate::classes::GltfBufferView > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_buffer_views", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_buffer_views(&mut self, buffer_views: &Array < Gd < crate::classes::GltfBufferView > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfBufferView > > >);
            let args = (RefArg::new(buffer_views),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_buffer_views", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_accessors(&mut self,) -> Array < Gd < crate::classes::GltfAccessor > > {
            type CallSig = (Array < Gd < crate::classes::GltfAccessor > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_accessors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_accessors(&mut self, accessors: &Array < Gd < crate::classes::GltfAccessor > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfAccessor > > >);
            let args = (RefArg::new(accessors),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_accessors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_meshes(&mut self,) -> Array < Gd < crate::classes::GltfMesh > > {
            type CallSig = (Array < Gd < crate::classes::GltfMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_meshes(&mut self, meshes: &Array < Gd < crate::classes::GltfMesh > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfMesh > > >);
            let args = (RefArg::new(meshes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_meshes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_players_count(&mut self, idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_animation_players_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_player(&mut self, idx: i32,) -> Option < Gd < crate::classes::AnimationPlayer > > {
            type CallSig = (Option < Gd < crate::classes::AnimationPlayer > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_animation_player", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_materials(&mut self,) -> Array < Gd < crate::classes::Material > > {
            type CallSig = (Array < Gd < crate::classes::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_materials", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_materials(&mut self, materials: &Array < Gd < crate::classes::Material > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::Material > > >);
            let args = (RefArg::new(materials),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_materials", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_name(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_scene_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scene_name(&mut self, scene_name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (scene_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_scene_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_path(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_base_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_base_path(&mut self, base_path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (base_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_base_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_filename(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_filename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_filename(&mut self, filename: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_filename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_nodes(&mut self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_root_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root_nodes(&mut self, root_nodes: &PackedInt32Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedInt32Array >);
            let args = (RefArg::new(root_nodes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_root_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_textures(&mut self,) -> Array < Gd < crate::classes::GltfTexture > > {
            type CallSig = (Array < Gd < crate::classes::GltfTexture > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_textures(&mut self, textures: &Array < Gd < crate::classes::GltfTexture > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfTexture > > >);
            let args = (RefArg::new(textures),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_textures", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_samplers(&mut self,) -> Array < Gd < crate::classes::GltfTextureSampler > > {
            type CallSig = (Array < Gd < crate::classes::GltfTextureSampler > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_texture_samplers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_samplers(&mut self, texture_samplers: &Array < Gd < crate::classes::GltfTextureSampler > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfTextureSampler > > >);
            let args = (RefArg::new(texture_samplers),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_texture_samplers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_images(&mut self,) -> Array < Gd < crate::classes::Texture2D > > {
            type CallSig = (Array < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_images", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_images(&mut self, images: &Array < Gd < crate::classes::Texture2D > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::Texture2D > > >);
            let args = (RefArg::new(images),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_images", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skins(&mut self,) -> Array < Gd < crate::classes::GltfSkin > > {
            type CallSig = (Array < Gd < crate::classes::GltfSkin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_skins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skins(&mut self, skins: &Array < Gd < crate::classes::GltfSkin > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfSkin > > >);
            let args = (RefArg::new(skins),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_skins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cameras(&mut self,) -> Array < Gd < crate::classes::GltfCamera > > {
            type CallSig = (Array < Gd < crate::classes::GltfCamera > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_cameras", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cameras(&mut self, cameras: &Array < Gd < crate::classes::GltfCamera > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfCamera > > >);
            let args = (RefArg::new(cameras),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_cameras", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lights(&mut self,) -> Array < Gd < crate::classes::GltfLight > > {
            type CallSig = (Array < Gd < crate::classes::GltfLight > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_lights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lights(&mut self, lights: &Array < Gd < crate::classes::GltfLight > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfLight > > >);
            let args = (RefArg::new(lights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_lights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_names(&mut self,) -> Array < GString > {
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_unique_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unique_names(&mut self, unique_names: &Array < GString >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < GString > >);
            let args = (RefArg::new(unique_names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_unique_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_animation_names(&mut self,) -> Array < GString > {
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_unique_animation_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unique_animation_names(&mut self, unique_animation_names: &Array < GString >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < GString > >);
            let args = (RefArg::new(unique_animation_names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_unique_animation_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skeletons(&mut self,) -> Array < Gd < crate::classes::GltfSkeleton > > {
            type CallSig = (Array < Gd < crate::classes::GltfSkeleton > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_skeletons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skeletons(&mut self, skeletons: &Array < Gd < crate::classes::GltfSkeleton > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfSkeleton > > >);
            let args = (RefArg::new(skeletons),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_skeletons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_create_animations(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_create_animations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_create_animations(&mut self, create_animations: bool,) {
            type CallSig = ((), bool);
            let args = (create_animations,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_create_animations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_import_as_skeleton_bones(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_import_as_skeleton_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_import_as_skeleton_bones(&mut self, import_as_skeleton_bones: bool,) {
            type CallSig = ((), bool);
            let args = (import_as_skeleton_bones,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_import_as_skeleton_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animations(&mut self,) -> Array < Gd < crate::classes::GltfAnimation > > {
            type CallSig = (Array < Gd < crate::classes::GltfAnimation > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_animations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_animations(&mut self, animations: &Array < Gd < crate::classes::GltfAnimation > >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Gd < crate::classes::GltfAnimation > > >);
            let args = (RefArg::new(animations),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_animations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_node(&mut self, idx: i32,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_scene_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_index(&mut self, scene_node: impl AsObjectArg < crate::classes::Node >,) -> i32 {
            type CallSig = (i32, ObjectArg < crate::classes::Node >);
            let args = (scene_node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_node_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_additional_data(&mut self, extension_name: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (extension_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_additional_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_additional_data(&mut self, extension_name: impl AsArg < StringName >, additional_data: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (extension_name.into_arg(), RefArg::new(additional_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_additional_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_handle_binary_image(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_handle_binary_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handle_binary_image(&mut self, method: i32,) {
            type CallSig = ((), i32);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_handle_binary_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_fps(&mut self, value: f64,) {
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "set_bake_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_fps(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfState", "get_bake_fps", self.object_ptr, self.__checked_id(), args,)
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
        pub const HANDLE_BINARY_DISCARD_TEXTURES: i32 = 0i32;
        pub const HANDLE_BINARY_EXTRACT_TEXTURES: i32 = 1i32;
        pub const HANDLE_BINARY_EMBED_AS_BASISU: i32 = 2i32;
        pub const HANDLE_BINARY_EMBED_AS_UNCOMPRESSED: i32 = 3i32;
        
    }
    impl crate::obj::GodotClass for GltfState {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GLTFState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfState {
        
    }
    impl crate::obj::cap::GodotDefault for GltfState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfState {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfState`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GltfState {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfState > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}