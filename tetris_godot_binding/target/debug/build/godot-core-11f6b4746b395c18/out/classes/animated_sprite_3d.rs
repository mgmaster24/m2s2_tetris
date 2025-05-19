#![doc = "Sidecar module for class [`AnimatedSprite3D`][crate::classes::AnimatedSprite3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimatedSprite3D` enums](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimatedSprite3D.`\n\nInherits [`SpriteBase3D`][crate::classes::SpriteBase3D].\n\nRelated symbols:\n\n* [`animated_sprite_3d`][crate::classes::animated_sprite_3d]: sidecar module with related enum/flag types\n* [`IAnimatedSprite3D`][crate::classes::IAnimatedSprite3D]: virtual methods\n\n\nSee also [Godot docs for `AnimatedSprite3D`](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AnimatedSprite3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimatedSprite3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimatedSprite3D`][crate::classes::AnimatedSprite3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimatedSprite3D` methods](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimatedSprite3D: crate::obj::GodotClass < Base = AnimatedSprite3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl AnimatedSprite3D {
        pub fn set_sprite_frames(&mut self, sprite_frames: impl AsObjectArg < crate::classes::SpriteFrames >,) {
            type CallSig = ((), ObjectArg < crate::classes::SpriteFrames >);
            let args = (sprite_frames.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_sprite_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sprite_frames(&self,) -> Option < Gd < crate::classes::SpriteFrames > > {
            type CallSig = (Option < Gd < crate::classes::SpriteFrames > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_sprite_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_animation(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autoplay(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autoplay(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "is_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn play_full(&mut self, name: CowArg < StringName >, custom_speed: f32, from_end: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, f32, bool);
            let args = (name, custom_speed, from_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "play", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play(&mut self,) {
            self.play_ex() . done()
        }
        #[inline]
        pub fn play_ex < 'a > (&'a mut self,) -> ExPlay < 'a > {
            ExPlay::new(self,)
        }
        pub(crate) fn play_backwards_full(&mut self, name: CowArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "play_backwards", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_backwards_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play_backwards(&mut self,) {
            self.play_backwards_ex() . done()
        }
        #[inline]
        pub fn play_backwards_ex < 'a > (&'a mut self,) -> ExPlayBackwards < 'a > {
            ExPlayBackwards::new(self,)
        }
        pub fn pause(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame(&mut self, frame: i32,) {
            type CallSig = ((), i32);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame_progress(&mut self, progress: f32,) {
            type CallSig = ((), f32);
            let args = (progress,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_frame_progress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_progress(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_frame_progress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame_and_progress(&mut self, frame: i32, progress: f32,) {
            type CallSig = ((), i32, f32);
            let args = (frame, progress,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_frame_and_progress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed_scale: f32,) {
            type CallSig = ((), f32);
            let args = (speed_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playing_speed(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_playing_speed", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimatedSprite3D {
        type Base = crate::classes::SpriteBase3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AnimatedSprite3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimatedSprite3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SpriteBase3D > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimatedSprite3D {
        
    }
    impl crate::obj::cap::GodotDefault for AnimatedSprite3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimatedSprite3D {
        type Target = crate::classes::SpriteBase3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimatedSprite3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimatedSprite3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AnimatedSprite3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimatedSprite3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::SpriteBase3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AnimatedSprite3D::play_ex`][super::AnimatedSprite3D::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimatedSprite3D, name: CowArg < 'a, StringName >, custom_speed: f32, from_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AnimatedSprite3D,) -> Self {
        let name = StringName::from("");
        let custom_speed = 1f32;
        let from_end = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), custom_speed: custom_speed, from_end: from_end,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, custom_speed: f32) -> Self {
        Self {
            custom_speed: custom_speed, .. self
        }
    }
    #[inline]
    pub fn from_end(self, from_end: bool) -> Self {
        Self {
            from_end: from_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, custom_speed, from_end,
        }
        = self;
        re_export::AnimatedSprite3D::play_full(surround_object, name, custom_speed, from_end,)
    }
}
#[doc = "Default-param extender for [`AnimatedSprite3D::play_backwards_ex`][super::AnimatedSprite3D::play_backwards_ex]."]
#[must_use]
pub struct ExPlayBackwards < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimatedSprite3D, name: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlayBackwards < 'a > {
    fn new(surround_object: &'a mut re_export::AnimatedSprite3D,) -> Self {
        let name = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name,
        }
        = self;
        re_export::AnimatedSprite3D::play_backwards_full(surround_object, name,)
    }
}