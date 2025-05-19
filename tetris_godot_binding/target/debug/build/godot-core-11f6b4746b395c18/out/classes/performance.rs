#![doc = "Sidecar module for class [`Performance`][crate::classes::Performance].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Performance` enums](https://docs.godotengine.org/en/stable/classes/class_performance.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Performance.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`performance`][crate::classes::performance]: sidecar module with related enum/flag types\n* [`IPerformance`][crate::classes::IPerformance]: virtual methods\n\n\nSee also [Godot docs for `Performance`](https://docs.godotengine.org/en/stable/classes/class_performance.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Performance::singleton()`][Performance::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Performance {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Performance`][crate::classes::Performance].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Performance` methods](https://docs.godotengine.org/en/stable/classes/class_performance.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPerformance: crate::obj::GodotClass < Base = Performance > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Performance {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"Performance");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_monitor(&self, monitor: crate::classes::performance::Monitor,) -> f64 {
            type CallSig = (f64, crate::classes::performance::Monitor);
            let args = (monitor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Performance", "get_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_custom_monitor_full(&mut self, id: CowArg < StringName >, callable: RefArg < Callable >, arguments: RefArg < VariantArray >,) {
            type CallSig < 'a0, 'a1, 'a2, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Callable >, RefArg < 'a2, VariantArray >);
            let args = (id, callable, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Performance", "add_custom_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_custom_monitor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_custom_monitor(&mut self, id: impl AsArg < StringName >, callable: &Callable,) {
            self.add_custom_monitor_ex(id, callable,) . done()
        }
        #[inline]
        pub fn add_custom_monitor_ex < 'a > (&'a mut self, id: impl AsArg < StringName > + 'a, callable: &'a Callable,) -> ExAddCustomMonitor < 'a > {
            ExAddCustomMonitor::new(self, id, callable,)
        }
        pub fn remove_custom_monitor(&mut self, id: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (id.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Performance", "remove_custom_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_custom_monitor(&mut self, id: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (id.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Performance", "has_custom_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_monitor(&mut self, id: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (id.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Performance", "get_custom_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_monitor_modification_time(&mut self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Performance", "get_monitor_modification_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_monitor_names(&mut self,) -> Array < StringName > {
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Performance", "get_custom_monitor_names", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Performance {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Performance"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Performance {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Performance {
        
    }
    impl std::ops::Deref for Performance {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Performance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Performance`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Performance {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Performance > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Performance::add_custom_monitor_ex`][super::Performance::add_custom_monitor_ex]."]
#[must_use]
pub struct ExAddCustomMonitor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Performance, id: CowArg < 'a, StringName >, callable: CowArg < 'a, Callable >, arguments: CowArg < 'a, VariantArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCustomMonitor < 'a > {
    fn new(surround_object: &'a mut re_export::Performance, id: impl AsArg < StringName > + 'a, callable: &'a Callable,) -> Self {
        let arguments = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id.into_arg(), callable: CowArg::Borrowed(callable), arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a VariantArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, callable, arguments,
        }
        = self;
        re_export::Performance::add_custom_monitor_full(surround_object, id, callable.cow_as_arg(), arguments.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Monitor {
    ord: i32
}
impl Monitor {
    pub const TIME_FPS: Monitor = Monitor {
        ord: 0i32
    };
    pub const TIME_PROCESS: Monitor = Monitor {
        ord: 1i32
    };
    pub const TIME_PHYSICS_PROCESS: Monitor = Monitor {
        ord: 2i32
    };
    pub const TIME_NAVIGATION_PROCESS: Monitor = Monitor {
        ord: 3i32
    };
    pub const MEMORY_STATIC: Monitor = Monitor {
        ord: 4i32
    };
    pub const MEMORY_STATIC_MAX: Monitor = Monitor {
        ord: 5i32
    };
    pub const MEMORY_MESSAGE_BUFFER_MAX: Monitor = Monitor {
        ord: 6i32
    };
    pub const OBJECT_COUNT: Monitor = Monitor {
        ord: 7i32
    };
    pub const OBJECT_RESOURCE_COUNT: Monitor = Monitor {
        ord: 8i32
    };
    pub const OBJECT_NODE_COUNT: Monitor = Monitor {
        ord: 9i32
    };
    pub const OBJECT_ORPHAN_NODE_COUNT: Monitor = Monitor {
        ord: 10i32
    };
    pub const RENDER_TOTAL_OBJECTS_IN_FRAME: Monitor = Monitor {
        ord: 11i32
    };
    pub const RENDER_TOTAL_PRIMITIVES_IN_FRAME: Monitor = Monitor {
        ord: 12i32
    };
    pub const RENDER_TOTAL_DRAW_CALLS_IN_FRAME: Monitor = Monitor {
        ord: 13i32
    };
    pub const RENDER_VIDEO_MEM_USED: Monitor = Monitor {
        ord: 14i32
    };
    pub const RENDER_TEXTURE_MEM_USED: Monitor = Monitor {
        ord: 15i32
    };
    pub const RENDER_BUFFER_MEM_USED: Monitor = Monitor {
        ord: 16i32
    };
    pub const PHYSICS_2D_ACTIVE_OBJECTS: Monitor = Monitor {
        ord: 17i32
    };
    pub const PHYSICS_2D_COLLISION_PAIRS: Monitor = Monitor {
        ord: 18i32
    };
    pub const PHYSICS_2D_ISLAND_COUNT: Monitor = Monitor {
        ord: 19i32
    };
    pub const PHYSICS_3D_ACTIVE_OBJECTS: Monitor = Monitor {
        ord: 20i32
    };
    pub const PHYSICS_3D_COLLISION_PAIRS: Monitor = Monitor {
        ord: 21i32
    };
    pub const PHYSICS_3D_ISLAND_COUNT: Monitor = Monitor {
        ord: 22i32
    };
    pub const AUDIO_OUTPUT_LATENCY: Monitor = Monitor {
        ord: 23i32
    };
    pub const NAVIGATION_ACTIVE_MAPS: Monitor = Monitor {
        ord: 24i32
    };
    pub const NAVIGATION_REGION_COUNT: Monitor = Monitor {
        ord: 25i32
    };
    pub const NAVIGATION_AGENT_COUNT: Monitor = Monitor {
        ord: 26i32
    };
    pub const NAVIGATION_LINK_COUNT: Monitor = Monitor {
        ord: 27i32
    };
    pub const NAVIGATION_POLYGON_COUNT: Monitor = Monitor {
        ord: 28i32
    };
    pub const NAVIGATION_EDGE_COUNT: Monitor = Monitor {
        ord: 29i32
    };
    pub const NAVIGATION_EDGE_MERGE_COUNT: Monitor = Monitor {
        ord: 30i32
    };
    pub const NAVIGATION_EDGE_CONNECTION_COUNT: Monitor = Monitor {
        ord: 31i32
    };
    pub const NAVIGATION_EDGE_FREE_COUNT: Monitor = Monitor {
        ord: 32i32
    };
    #[doc(alias = "MONITOR_MAX")]
    #[doc = "Godot enumerator name: `MONITOR_MAX`"]
    pub const MAX: Monitor = Monitor {
        ord: 33i32
    };
    
}
impl std::fmt::Debug for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Monitor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Monitor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 => Some(Self {
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
            Self::TIME_FPS => "TIME_FPS", Self::TIME_PROCESS => "TIME_PROCESS", Self::TIME_PHYSICS_PROCESS => "TIME_PHYSICS_PROCESS", Self::TIME_NAVIGATION_PROCESS => "TIME_NAVIGATION_PROCESS", Self::MEMORY_STATIC => "MEMORY_STATIC", Self::MEMORY_STATIC_MAX => "MEMORY_STATIC_MAX", Self::MEMORY_MESSAGE_BUFFER_MAX => "MEMORY_MESSAGE_BUFFER_MAX", Self::OBJECT_COUNT => "OBJECT_COUNT", Self::OBJECT_RESOURCE_COUNT => "OBJECT_RESOURCE_COUNT", Self::OBJECT_NODE_COUNT => "OBJECT_NODE_COUNT", Self::OBJECT_ORPHAN_NODE_COUNT => "OBJECT_ORPHAN_NODE_COUNT", Self::RENDER_TOTAL_OBJECTS_IN_FRAME => "RENDER_TOTAL_OBJECTS_IN_FRAME", Self::RENDER_TOTAL_PRIMITIVES_IN_FRAME => "RENDER_TOTAL_PRIMITIVES_IN_FRAME", Self::RENDER_TOTAL_DRAW_CALLS_IN_FRAME => "RENDER_TOTAL_DRAW_CALLS_IN_FRAME", Self::RENDER_VIDEO_MEM_USED => "RENDER_VIDEO_MEM_USED", Self::RENDER_TEXTURE_MEM_USED => "RENDER_TEXTURE_MEM_USED", Self::RENDER_BUFFER_MEM_USED => "RENDER_BUFFER_MEM_USED", Self::PHYSICS_2D_ACTIVE_OBJECTS => "PHYSICS_2D_ACTIVE_OBJECTS", Self::PHYSICS_2D_COLLISION_PAIRS => "PHYSICS_2D_COLLISION_PAIRS", Self::PHYSICS_2D_ISLAND_COUNT => "PHYSICS_2D_ISLAND_COUNT", Self::PHYSICS_3D_ACTIVE_OBJECTS => "PHYSICS_3D_ACTIVE_OBJECTS", Self::PHYSICS_3D_COLLISION_PAIRS => "PHYSICS_3D_COLLISION_PAIRS", Self::PHYSICS_3D_ISLAND_COUNT => "PHYSICS_3D_ISLAND_COUNT", Self::AUDIO_OUTPUT_LATENCY => "AUDIO_OUTPUT_LATENCY", Self::NAVIGATION_ACTIVE_MAPS => "NAVIGATION_ACTIVE_MAPS", Self::NAVIGATION_REGION_COUNT => "NAVIGATION_REGION_COUNT", Self::NAVIGATION_AGENT_COUNT => "NAVIGATION_AGENT_COUNT", Self::NAVIGATION_LINK_COUNT => "NAVIGATION_LINK_COUNT", Self::NAVIGATION_POLYGON_COUNT => "NAVIGATION_POLYGON_COUNT", Self::NAVIGATION_EDGE_COUNT => "NAVIGATION_EDGE_COUNT", Self::NAVIGATION_EDGE_MERGE_COUNT => "NAVIGATION_EDGE_MERGE_COUNT", Self::NAVIGATION_EDGE_CONNECTION_COUNT => "NAVIGATION_EDGE_CONNECTION_COUNT", Self::NAVIGATION_EDGE_FREE_COUNT => "NAVIGATION_EDGE_FREE_COUNT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::MAX => "MONITOR_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Monitor {
    const ENUMERATOR_COUNT: usize = 33usize;
    
}
impl crate::meta::GodotConvert for Monitor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Monitor {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Monitor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}