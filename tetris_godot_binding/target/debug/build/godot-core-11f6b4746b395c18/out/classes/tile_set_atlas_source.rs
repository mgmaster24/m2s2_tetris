#![doc = "Sidecar module for class [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSetAtlasSource` enums](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSetAtlasSource.`\n\nInherits [`TileSetSource`][crate::classes::TileSetSource].\n\nRelated symbols:\n\n* [`tile_set_atlas_source`][crate::classes::tile_set_atlas_source]: sidecar module with related enum/flag types\n* [`ITileSetAtlasSource`][crate::classes::ITileSetAtlasSource]: virtual methods\n\n\nSee also [Godot docs for `TileSetAtlasSource`](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TileSetAtlasSource::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSetAtlasSource {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileSetAtlasSource` methods](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSetAtlasSource: crate::obj::GodotClass < Base = TileSetAtlasSource > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileSetAtlasSource {
        pub fn set_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_margins(&mut self, margins: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (margins,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_margins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_margins(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_margins", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_separation(&mut self, separation: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (separation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_separation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_separation(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_separation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_region_size(&mut self, texture_region_size: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (texture_region_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_texture_region_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_region_size(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_texture_region_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_texture_padding(&mut self, use_texture_padding: bool,) {
            type CallSig = ((), bool);
            let args = (use_texture_padding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_use_texture_padding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_texture_padding(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_use_texture_padding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_tile_full(&mut self, atlas_coords: Vector2i, size: Vector2i,) {
            type CallSig = ((), Vector2i, Vector2i);
            let args = (atlas_coords, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "create_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_tile_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_tile(&mut self, atlas_coords: Vector2i,) {
            self.create_tile_ex(atlas_coords,) . done()
        }
        #[inline]
        pub fn create_tile_ex < 'a > (&'a mut self, atlas_coords: Vector2i,) -> ExCreateTile < 'a > {
            ExCreateTile::new(self, atlas_coords,)
        }
        pub fn remove_tile(&mut self, atlas_coords: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "remove_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn move_tile_in_atlas_full(&mut self, atlas_coords: Vector2i, new_atlas_coords: Vector2i, new_size: Vector2i,) {
            type CallSig = ((), Vector2i, Vector2i, Vector2i);
            let args = (atlas_coords, new_atlas_coords, new_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "move_tile_in_atlas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::move_tile_in_atlas_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn move_tile_in_atlas(&mut self, atlas_coords: Vector2i,) {
            self.move_tile_in_atlas_ex(atlas_coords,) . done()
        }
        #[inline]
        pub fn move_tile_in_atlas_ex < 'a > (&'a mut self, atlas_coords: Vector2i,) -> ExMoveTileInAtlas < 'a > {
            ExMoveTileInAtlas::new(self, atlas_coords,)
        }
        pub fn get_tile_size_in_atlas(&self, atlas_coords: Vector2i,) -> Vector2i {
            type CallSig = (Vector2i, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_size_in_atlas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn has_room_for_tile_full(&self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32, ignored_tile: Vector2i,) -> bool {
            type CallSig = (bool, Vector2i, Vector2i, i32, Vector2i, i32, Vector2i);
            let args = (atlas_coords, size, animation_columns, animation_separation, frames_count, ignored_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "has_room_for_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_room_for_tile_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_room_for_tile(&self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> bool {
            self.has_room_for_tile_ex(atlas_coords, size, animation_columns, animation_separation, frames_count,) . done()
        }
        #[inline]
        pub fn has_room_for_tile_ex < 'a > (&'a self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> ExHasRoomForTile < 'a > {
            ExHasRoomForTile::new(self, atlas_coords, size, animation_columns, animation_separation, frames_count,)
        }
        pub fn get_tiles_to_be_removed_on_change(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, margins: Vector2i, separation: Vector2i, texture_region_size: Vector2i,) -> PackedVector2Array {
            type CallSig = (PackedVector2Array, ObjectArg < crate::classes::Texture2D >, Vector2i, Vector2i, Vector2i);
            let args = (texture.as_object_arg(), margins, separation, texture_region_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tiles_to_be_removed_on_change", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_at_coords(&self, atlas_coords: Vector2i,) -> Vector2i {
            type CallSig = (Vector2i, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_at_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_tiles_outside_texture(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "has_tiles_outside_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_tiles_outside_texture(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "clear_tiles_outside_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_columns(&mut self, atlas_coords: Vector2i, frame_columns: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (atlas_coords, frame_columns,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_columns(&self, atlas_coords: Vector2i,) -> i32 {
            type CallSig = (i32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_separation(&mut self, atlas_coords: Vector2i, separation: Vector2i,) {
            type CallSig = ((), Vector2i, Vector2i);
            let args = (atlas_coords, separation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_separation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_separation(&self, atlas_coords: Vector2i,) -> Vector2i {
            type CallSig = (Vector2i, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_separation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_speed(&mut self, atlas_coords: Vector2i, speed: f32,) {
            type CallSig = ((), Vector2i, f32);
            let args = (atlas_coords, speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_speed(&self, atlas_coords: Vector2i,) -> f32 {
            type CallSig = (f32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_mode(&mut self, atlas_coords: Vector2i, mode: crate::classes::tile_set_atlas_source::TileAnimationMode,) {
            type CallSig = ((), Vector2i, crate::classes::tile_set_atlas_source::TileAnimationMode);
            let args = (atlas_coords, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_mode(&self, atlas_coords: Vector2i,) -> crate::classes::tile_set_atlas_source::TileAnimationMode {
            type CallSig = (crate::classes::tile_set_atlas_source::TileAnimationMode, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_frames_count(&mut self, atlas_coords: Vector2i, frames_count: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (atlas_coords, frames_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_frames_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_frames_count(&self, atlas_coords: Vector2i,) -> i32 {
            type CallSig = (i32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_frames_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_animation_frame_duration(&mut self, atlas_coords: Vector2i, frame_index: i32, duration: f32,) {
            type CallSig = ((), Vector2i, i32, f32);
            let args = (atlas_coords, frame_index, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_frame_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_frame_duration(&self, atlas_coords: Vector2i, frame_index: i32,) -> f32 {
            type CallSig = (f32, Vector2i, i32);
            let args = (atlas_coords, frame_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_frame_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_animation_total_duration(&self, atlas_coords: Vector2i,) -> f32 {
            type CallSig = (f32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_total_duration", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_alternative_tile_full(&mut self, atlas_coords: Vector2i, alternative_id_override: i32,) -> i32 {
            type CallSig = (i32, Vector2i, i32);
            let args = (atlas_coords, alternative_id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "create_alternative_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_alternative_tile_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_alternative_tile(&mut self, atlas_coords: Vector2i,) -> i32 {
            self.create_alternative_tile_ex(atlas_coords,) . done()
        }
        #[inline]
        pub fn create_alternative_tile_ex < 'a > (&'a mut self, atlas_coords: Vector2i,) -> ExCreateAlternativeTile < 'a > {
            ExCreateAlternativeTile::new(self, atlas_coords,)
        }
        pub fn remove_alternative_tile(&mut self, atlas_coords: Vector2i, alternative_tile: i32,) {
            type CallSig = ((), Vector2i, i32);
            let args = (atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "remove_alternative_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alternative_tile_id(&mut self, atlas_coords: Vector2i, alternative_tile: i32, new_id: i32,) {
            type CallSig = ((), Vector2i, i32, i32);
            let args = (atlas_coords, alternative_tile, new_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_alternative_tile_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next_alternative_tile_id(&self, atlas_coords: Vector2i,) -> i32 {
            type CallSig = (i32, Vector2i);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_next_alternative_tile_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_data(&self, atlas_coords: Vector2i, alternative_tile: i32,) -> Option < Gd < crate::classes::TileData > > {
            type CallSig = (Option < Gd < crate::classes::TileData > >, Vector2i, i32);
            let args = (atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_atlas_grid_size(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_atlas_grid_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_tile_texture_region_full(&self, atlas_coords: Vector2i, frame: i32,) -> Rect2i {
            type CallSig = (Rect2i, Vector2i, i32);
            let args = (atlas_coords, frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_texture_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_tile_texture_region_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_tile_texture_region(&self, atlas_coords: Vector2i,) -> Rect2i {
            self.get_tile_texture_region_ex(atlas_coords,) . done()
        }
        #[inline]
        pub fn get_tile_texture_region_ex < 'a > (&'a self, atlas_coords: Vector2i,) -> ExGetTileTextureRegion < 'a > {
            ExGetTileTextureRegion::new(self, atlas_coords,)
        }
        pub fn get_runtime_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_runtime_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_runtime_tile_texture_region(&self, atlas_coords: Vector2i, frame: i32,) -> Rect2i {
            type CallSig = (Rect2i, Vector2i, i32);
            let args = (atlas_coords, frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_runtime_tile_texture_region", self.object_ptr, self.__checked_id(), args,)
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
        pub const TRANSFORM_FLIP_H: i32 = 4096i32;
        pub const TRANSFORM_FLIP_V: i32 = 8192i32;
        pub const TRANSFORM_TRANSPOSE: i32 = 16384i32;
        
    }
    impl crate::obj::GodotClass for TileSetAtlasSource {
        type Base = crate::classes::TileSetSource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TileSetAtlasSource"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSetAtlasSource {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TileSetSource > for TileSetAtlasSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TileSetAtlasSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TileSetAtlasSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileSetAtlasSource {
        
    }
    impl crate::obj::cap::GodotDefault for TileSetAtlasSource {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSetAtlasSource {
        type Target = crate::classes::TileSetSource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSetAtlasSource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileSetAtlasSource`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TileSetAtlasSource {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSetAtlasSource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSetSource > for $Class {
                
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
#[doc = "Default-param extender for [`TileSetAtlasSource::create_tile_ex`][super::TileSetAtlasSource::create_tile_ex]."]
#[must_use]
pub struct ExCreateTile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateTile < 'a > {
    fn new(surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        let size = Vector2i::new(1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, size: size,
        }
    }
    #[inline]
    pub fn size(self, size: Vector2i) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, atlas_coords, size,
        }
        = self;
        re_export::TileSetAtlasSource::create_tile_full(surround_object, atlas_coords, size,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::move_tile_in_atlas_ex`][super::TileSetAtlasSource::move_tile_in_atlas_ex]."]
#[must_use]
pub struct ExMoveTileInAtlas < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, new_atlas_coords: Vector2i, new_size: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveTileInAtlas < 'a > {
    fn new(surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        let new_atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let new_size = Vector2i::new(- 1 as _, - 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, new_atlas_coords: new_atlas_coords, new_size: new_size,
        }
    }
    #[inline]
    pub fn new_atlas_coords(self, new_atlas_coords: Vector2i) -> Self {
        Self {
            new_atlas_coords: new_atlas_coords, .. self
        }
    }
    #[inline]
    pub fn new_size(self, new_size: Vector2i) -> Self {
        Self {
            new_size: new_size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, atlas_coords, new_atlas_coords, new_size,
        }
        = self;
        re_export::TileSetAtlasSource::move_tile_in_atlas_full(surround_object, atlas_coords, new_atlas_coords, new_size,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::has_room_for_tile_ex`][super::TileSetAtlasSource::has_room_for_tile_ex]."]
#[must_use]
pub struct ExHasRoomForTile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32, ignored_tile: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasRoomForTile < 'a > {
    fn new(surround_object: &'a re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> Self {
        let ignored_tile = Vector2i::new(- 1 as _, - 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, size: size, animation_columns: animation_columns, animation_separation: animation_separation, frames_count: frames_count, ignored_tile: ignored_tile,
        }
    }
    #[inline]
    pub fn ignored_tile(self, ignored_tile: Vector2i) -> Self {
        Self {
            ignored_tile: ignored_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, atlas_coords, size, animation_columns, animation_separation, frames_count, ignored_tile,
        }
        = self;
        re_export::TileSetAtlasSource::has_room_for_tile_full(surround_object, atlas_coords, size, animation_columns, animation_separation, frames_count, ignored_tile,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::create_alternative_tile_ex`][super::TileSetAtlasSource::create_alternative_tile_ex]."]
#[must_use]
pub struct ExCreateAlternativeTile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, alternative_id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateAlternativeTile < 'a > {
    fn new(surround_object: &'a mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        let alternative_id_override = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, alternative_id_override: alternative_id_override,
        }
    }
    #[inline]
    pub fn alternative_id_override(self, alternative_id_override: i32) -> Self {
        Self {
            alternative_id_override: alternative_id_override, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, atlas_coords, alternative_id_override,
        }
        = self;
        re_export::TileSetAtlasSource::create_alternative_tile_full(surround_object, atlas_coords, alternative_id_override,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::get_tile_texture_region_ex`][super::TileSetAtlasSource::get_tile_texture_region_ex]."]
#[must_use]
pub struct ExGetTileTextureRegion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileSetAtlasSource, atlas_coords: Vector2i, frame: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTileTextureRegion < 'a > {
    fn new(surround_object: &'a re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        let frame = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, frame: frame,
        }
    }
    #[inline]
    pub fn frame(self, frame: i32) -> Self {
        Self {
            frame: frame, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2i {
        let Self {
            _phantom, surround_object, atlas_coords, frame,
        }
        = self;
        re_export::TileSetAtlasSource::get_tile_texture_region_full(surround_object, atlas_coords, frame,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileAnimationMode {
    ord: i32
}
impl TileAnimationMode {
    #[doc(alias = "TILE_ANIMATION_MODE_DEFAULT")]
    #[doc = "Godot enumerator name: `TILE_ANIMATION_MODE_DEFAULT`"]
    pub const DEFAULT: TileAnimationMode = TileAnimationMode {
        ord: 0i32
    };
    #[doc(alias = "TILE_ANIMATION_MODE_RANDOM_START_TIMES")]
    #[doc = "Godot enumerator name: `TILE_ANIMATION_MODE_RANDOM_START_TIMES`"]
    pub const RANDOM_START_TIMES: TileAnimationMode = TileAnimationMode {
        ord: 1i32
    };
    #[doc(alias = "TILE_ANIMATION_MODE_MAX")]
    #[doc = "Godot enumerator name: `TILE_ANIMATION_MODE_MAX`"]
    pub const MAX: TileAnimationMode = TileAnimationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TileAnimationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileAnimationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileAnimationMode {
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
            Self::DEFAULT => "DEFAULT", Self::RANDOM_START_TIMES => "RANDOM_START_TIMES", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "TILE_ANIMATION_MODE_DEFAULT", Self::RANDOM_START_TIMES => "TILE_ANIMATION_MODE_RANDOM_START_TIMES", Self::MAX => "TILE_ANIMATION_MODE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TileAnimationMode {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for TileAnimationMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TileAnimationMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileAnimationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}