use crate::builtin::*;
use crate::classes::Object;
use crate::obj::Gd;
impl crate::obj::EngineEnum for VariantType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 => Some(Self {
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
            Self::NIL => "NIL", Self::BOOL => "BOOL", Self::INT => "INT", Self::FLOAT => "FLOAT", Self::STRING => "STRING", Self::VECTOR2 => "VECTOR2", Self::VECTOR2I => "VECTOR2I", Self::RECT2 => "RECT2", Self::RECT2I => "RECT2I", Self::VECTOR3 => "VECTOR3", Self::VECTOR3I => "VECTOR3I", Self::TRANSFORM2D => "TRANSFORM2D", Self::VECTOR4 => "VECTOR4", Self::VECTOR4I => "VECTOR4I", Self::PLANE => "PLANE", Self::QUATERNION => "QUATERNION", Self::AABB => "AABB", Self::BASIS => "BASIS", Self::TRANSFORM3D => "TRANSFORM3D", Self::PROJECTION => "PROJECTION", Self::COLOR => "COLOR", Self::STRING_NAME => "STRING_NAME", Self::NODE_PATH => "NODE_PATH", Self::RID => "RID", Self::OBJECT => "OBJECT", Self::CALLABLE => "CALLABLE", Self::SIGNAL => "SIGNAL", Self::DICTIONARY => "DICTIONARY", Self::ARRAY => "ARRAY", Self::PACKED_BYTE_ARRAY => "PACKED_BYTE_ARRAY", Self::PACKED_INT32_ARRAY => "PACKED_INT32_ARRAY", Self::PACKED_INT64_ARRAY => "PACKED_INT64_ARRAY", Self::PACKED_FLOAT32_ARRAY => "PACKED_FLOAT32_ARRAY", Self::PACKED_FLOAT64_ARRAY => "PACKED_FLOAT64_ARRAY", Self::PACKED_STRING_ARRAY => "PACKED_STRING_ARRAY", Self::PACKED_VECTOR2_ARRAY => "PACKED_VECTOR2_ARRAY", Self::PACKED_VECTOR3_ARRAY => "PACKED_VECTOR3_ARRAY", Self::PACKED_COLOR_ARRAY => "PACKED_COLOR_ARRAY", Self::PACKED_VECTOR4_ARRAY => "PACKED_VECTOR4_ARRAY", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NIL => "TYPE_NIL", Self::BOOL => "TYPE_BOOL", Self::INT => "TYPE_INT", Self::FLOAT => "TYPE_FLOAT", Self::STRING => "TYPE_STRING", Self::VECTOR2 => "TYPE_VECTOR2", Self::VECTOR2I => "TYPE_VECTOR2I", Self::RECT2 => "TYPE_RECT2", Self::RECT2I => "TYPE_RECT2I", Self::VECTOR3 => "TYPE_VECTOR3", Self::VECTOR3I => "TYPE_VECTOR3I", Self::TRANSFORM2D => "TYPE_TRANSFORM2D", Self::VECTOR4 => "TYPE_VECTOR4", Self::VECTOR4I => "TYPE_VECTOR4I", Self::PLANE => "TYPE_PLANE", Self::QUATERNION => "TYPE_QUATERNION", Self::AABB => "TYPE_AABB", Self::BASIS => "TYPE_BASIS", Self::TRANSFORM3D => "TYPE_TRANSFORM3D", Self::PROJECTION => "TYPE_PROJECTION", Self::COLOR => "TYPE_COLOR", Self::STRING_NAME => "TYPE_STRING_NAME", Self::NODE_PATH => "TYPE_NODE_PATH", Self::RID => "TYPE_RID", Self::OBJECT => "TYPE_OBJECT", Self::CALLABLE => "TYPE_CALLABLE", Self::SIGNAL => "TYPE_SIGNAL", Self::DICTIONARY => "TYPE_DICTIONARY", Self::ARRAY => "TYPE_ARRAY", Self::PACKED_BYTE_ARRAY => "TYPE_PACKED_BYTE_ARRAY", Self::PACKED_INT32_ARRAY => "TYPE_PACKED_INT32_ARRAY", Self::PACKED_INT64_ARRAY => "TYPE_PACKED_INT64_ARRAY", Self::PACKED_FLOAT32_ARRAY => "TYPE_PACKED_FLOAT32_ARRAY", Self::PACKED_FLOAT64_ARRAY => "TYPE_PACKED_FLOAT64_ARRAY", Self::PACKED_STRING_ARRAY => "TYPE_PACKED_STRING_ARRAY", Self::PACKED_VECTOR2_ARRAY => "TYPE_PACKED_VECTOR2_ARRAY", Self::PACKED_VECTOR3_ARRAY => "TYPE_PACKED_VECTOR3_ARRAY", Self::PACKED_COLOR_ARRAY => "TYPE_PACKED_COLOR_ARRAY", Self::PACKED_VECTOR4_ARRAY => "TYPE_PACKED_VECTOR4_ARRAY", Self::MAX => "TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for VariantType {
    const ENUMERATOR_COUNT: usize = 39usize;
    
}
impl crate::meta::GodotConvert for VariantType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VariantType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VariantType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[allow(dead_code)]
pub enum VariantDispatch {
    Nil, Bool(bool), Int(i64), Float(f64), String(GString), Vector2(Vector2), Vector2i(Vector2i), Rect2(Rect2), Rect2i(Rect2i), Vector3(Vector3), Vector3i(Vector3i), Transform2D(Transform2D), Vector4(Vector4), Vector4i(Vector4i), Plane(Plane), Quaternion(Quaternion), Aabb(Aabb), Basis(Basis), Transform3D(Transform3D), Projection(Projection), Color(Color), StringName(StringName), NodePath(NodePath), Rid(Rid), Object(Gd < crate::classes::Object >), Callable(Callable), Signal(Signal), Dictionary(Dictionary), Array(VariantArray), PackedByteArray(PackedByteArray), PackedInt32Array(PackedInt32Array), PackedInt64Array(PackedInt64Array), PackedFloat32Array(PackedFloat32Array), PackedFloat64Array(PackedFloat64Array), PackedStringArray(PackedStringArray), PackedVector2Array(PackedVector2Array), PackedVector3Array(PackedVector3Array), PackedColorArray(PackedColorArray), PackedVector4Array(PackedVector4Array), #[doc = r" Special case of a `Variant` holding an object that has been destroyed."]
    FreedObject,
}
impl VariantDispatch {
    pub fn from_variant(variant: &Variant) -> Self {
        match variant.get_type() {
            VariantType::NIL => Self::Nil, VariantType::OBJECT if !variant.is_object_alive() => Self::FreedObject, VariantType::BOOL => Self::Bool(variant.to::< bool > ()), VariantType::INT => Self::Int(variant.to::< i64 > ()), VariantType::FLOAT => Self::Float(variant.to::< f64 > ()), VariantType::STRING => Self::String(variant.to::< GString > ()), VariantType::VECTOR2 => Self::Vector2(variant.to::< Vector2 > ()), VariantType::VECTOR2I => Self::Vector2i(variant.to::< Vector2i > ()), VariantType::RECT2 => Self::Rect2(variant.to::< Rect2 > ()), VariantType::RECT2I => Self::Rect2i(variant.to::< Rect2i > ()), VariantType::VECTOR3 => Self::Vector3(variant.to::< Vector3 > ()), VariantType::VECTOR3I => Self::Vector3i(variant.to::< Vector3i > ()), VariantType::TRANSFORM2D => Self::Transform2D(variant.to::< Transform2D > ()), VariantType::VECTOR4 => Self::Vector4(variant.to::< Vector4 > ()), VariantType::VECTOR4I => Self::Vector4i(variant.to::< Vector4i > ()), VariantType::PLANE => Self::Plane(variant.to::< Plane > ()), VariantType::QUATERNION => Self::Quaternion(variant.to::< Quaternion > ()), VariantType::AABB => Self::Aabb(variant.to::< Aabb > ()), VariantType::BASIS => Self::Basis(variant.to::< Basis > ()), VariantType::TRANSFORM3D => Self::Transform3D(variant.to::< Transform3D > ()), VariantType::PROJECTION => Self::Projection(variant.to::< Projection > ()), VariantType::COLOR => Self::Color(variant.to::< Color > ()), VariantType::STRING_NAME => Self::StringName(variant.to::< StringName > ()), VariantType::NODE_PATH => Self::NodePath(variant.to::< NodePath > ()), VariantType::RID => Self::Rid(variant.to::< Rid > ()), VariantType::OBJECT => Self::Object(variant.to::< Gd < crate::classes::Object > > ()), VariantType::CALLABLE => Self::Callable(variant.to::< Callable > ()), VariantType::SIGNAL => Self::Signal(variant.to::< Signal > ()), VariantType::DICTIONARY => Self::Dictionary(variant.to::< Dictionary > ()), VariantType::ARRAY => Self::Array(variant.to::< VariantArray > ()), VariantType::PACKED_BYTE_ARRAY => Self::PackedByteArray(variant.to::< PackedByteArray > ()), VariantType::PACKED_INT32_ARRAY => Self::PackedInt32Array(variant.to::< PackedInt32Array > ()), VariantType::PACKED_INT64_ARRAY => Self::PackedInt64Array(variant.to::< PackedInt64Array > ()), VariantType::PACKED_FLOAT32_ARRAY => Self::PackedFloat32Array(variant.to::< PackedFloat32Array > ()), VariantType::PACKED_FLOAT64_ARRAY => Self::PackedFloat64Array(variant.to::< PackedFloat64Array > ()), VariantType::PACKED_STRING_ARRAY => Self::PackedStringArray(variant.to::< PackedStringArray > ()), VariantType::PACKED_VECTOR2_ARRAY => Self::PackedVector2Array(variant.to::< PackedVector2Array > ()), VariantType::PACKED_VECTOR3_ARRAY => Self::PackedVector3Array(variant.to::< PackedVector3Array > ()), VariantType::PACKED_COLOR_ARRAY => Self::PackedColorArray(variant.to::< PackedColorArray > ()), VariantType::PACKED_VECTOR4_ARRAY => Self::PackedVector4Array(variant.to::< PackedVector4Array > ()), _ => panic !("Variant type not supported: {:?}", variant.get_type()),
        }
    }
}
impl std::fmt::Debug for VariantDispatch {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        match self {
            Self::Nil => write !(f, "null"), Self::Bool(v) => write !(f, "{v:?}"), Self::Int(v) => write !(f, "{v:?}"), Self::Float(v) => write !(f, "{v:?}"), Self::String(v) => write !(f, "{v:?}"), Self::Vector2(v) => write !(f, "{v:?}"), Self::Vector2i(v) => write !(f, "{v:?}"), Self::Rect2(v) => write !(f, "{v:?}"), Self::Rect2i(v) => write !(f, "{v:?}"), Self::Vector3(v) => write !(f, "{v:?}"), Self::Vector3i(v) => write !(f, "{v:?}"), Self::Transform2D(v) => write !(f, "{v:?}"), Self::Vector4(v) => write !(f, "{v:?}"), Self::Vector4i(v) => write !(f, "{v:?}"), Self::Plane(v) => write !(f, "{v:?}"), Self::Quaternion(v) => write !(f, "{v:?}"), Self::Aabb(v) => write !(f, "{v:?}"), Self::Basis(v) => write !(f, "{v:?}"), Self::Transform3D(v) => write !(f, "{v:?}"), Self::Projection(v) => write !(f, "{v:?}"), Self::Color(v) => write !(f, "{v:?}"), Self::StringName(v) => write !(f, "{v:?}"), Self::NodePath(v) => write !(f, "{v:?}"), Self::Rid(v) => write !(f, "{v:?}"), Self::Object(v) => write !(f, "{v:?}"), Self::Callable(v) => write !(f, "{v:?}"), Self::Signal(v) => write !(f, "{v:?}"), Self::Dictionary(v) => write !(f, "{v:?}"), Self::Array(v) => write !(f, "{v:?}"), Self::PackedByteArray(v) => write !(f, "{v:?}"), Self::PackedInt32Array(v) => write !(f, "{v:?}"), Self::PackedInt64Array(v) => write !(f, "{v:?}"), Self::PackedFloat32Array(v) => write !(f, "{v:?}"), Self::PackedFloat64Array(v) => write !(f, "{v:?}"), Self::PackedStringArray(v) => write !(f, "{v:?}"), Self::PackedVector2Array(v) => write !(f, "{v:?}"), Self::PackedVector3Array(v) => write !(f, "{v:?}"), Self::PackedColorArray(v) => write !(f, "{v:?}"), Self::PackedVector4Array(v) => write !(f, "{v:?}"), Self::FreedObject => write !(f, "<Freed Object>"),
        }
    }
}
#[doc = r" Global enums and constants, generated by Godot."]
pub mod global_enums {
    use crate::sys;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[doc = r""]
    #[doc = r" This enum is exhaustive; you should not expect future Godot versions to add new enumerators."]
    #[allow(non_camel_case_types)]
    pub enum Orientation {
        VERTICAL = 1i32, HORIZONTAL = 0i32,
    }
    impl crate::obj::EngineEnum for Orientation {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                1i32 => Some(Self::VERTICAL), 0i32 => Some(Self::HORIZONTAL), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self as i32
        }
        #[inline]
        fn as_str(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::VERTICAL => "VERTICAL", Self::HORIZONTAL => "HORIZONTAL", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            self.as_str()
        }
    }
    impl crate::meta::GodotConvert for Orientation {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for Orientation {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for Orientation {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[doc = r""]
    #[doc = r" This enum is exhaustive; you should not expect future Godot versions to add new enumerators."]
    #[allow(non_camel_case_types)]
    pub enum ClockDirection {
        CLOCKWISE = 0i32, COUNTERCLOCKWISE = 1i32,
    }
    impl crate::obj::EngineEnum for ClockDirection {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                0i32 => Some(Self::CLOCKWISE), 1i32 => Some(Self::COUNTERCLOCKWISE), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self as i32
        }
        #[inline]
        fn as_str(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::CLOCKWISE => "CLOCKWISE", Self::COUNTERCLOCKWISE => "COUNTERCLOCKWISE", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            self.as_str()
        }
    }
    impl crate::meta::GodotConvert for ClockDirection {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for ClockDirection {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for ClockDirection {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HorizontalAlignment {
        ord: i32
    }
    impl HorizontalAlignment {
        #[doc(alias = "HORIZONTAL_ALIGNMENT_LEFT")]
        #[doc = "Godot enumerator name: `HORIZONTAL_ALIGNMENT_LEFT`"]
        pub const LEFT: HorizontalAlignment = HorizontalAlignment {
            ord: 0i32
        };
        #[doc(alias = "HORIZONTAL_ALIGNMENT_CENTER")]
        #[doc = "Godot enumerator name: `HORIZONTAL_ALIGNMENT_CENTER`"]
        pub const CENTER: HorizontalAlignment = HorizontalAlignment {
            ord: 1i32
        };
        #[doc(alias = "HORIZONTAL_ALIGNMENT_RIGHT")]
        #[doc = "Godot enumerator name: `HORIZONTAL_ALIGNMENT_RIGHT`"]
        pub const RIGHT: HorizontalAlignment = HorizontalAlignment {
            ord: 2i32
        };
        #[doc(alias = "HORIZONTAL_ALIGNMENT_FILL")]
        #[doc = "Godot enumerator name: `HORIZONTAL_ALIGNMENT_FILL`"]
        pub const FILL: HorizontalAlignment = HorizontalAlignment {
            ord: 3i32
        };
        
    }
    impl std::fmt::Debug for HorizontalAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("HorizontalAlignment") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for HorizontalAlignment {
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
                Self::LEFT => "LEFT", Self::CENTER => "CENTER", Self::RIGHT => "RIGHT", Self::FILL => "FILL", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::LEFT => "HORIZONTAL_ALIGNMENT_LEFT", Self::CENTER => "HORIZONTAL_ALIGNMENT_CENTER", Self::RIGHT => "HORIZONTAL_ALIGNMENT_RIGHT", Self::FILL => "HORIZONTAL_ALIGNMENT_FILL", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for HorizontalAlignment {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for HorizontalAlignment {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for HorizontalAlignment {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct VerticalAlignment {
        ord: i32
    }
    impl VerticalAlignment {
        #[doc(alias = "VERTICAL_ALIGNMENT_TOP")]
        #[doc = "Godot enumerator name: `VERTICAL_ALIGNMENT_TOP`"]
        pub const TOP: VerticalAlignment = VerticalAlignment {
            ord: 0i32
        };
        #[doc(alias = "VERTICAL_ALIGNMENT_CENTER")]
        #[doc = "Godot enumerator name: `VERTICAL_ALIGNMENT_CENTER`"]
        pub const CENTER: VerticalAlignment = VerticalAlignment {
            ord: 1i32
        };
        #[doc(alias = "VERTICAL_ALIGNMENT_BOTTOM")]
        #[doc = "Godot enumerator name: `VERTICAL_ALIGNMENT_BOTTOM`"]
        pub const BOTTOM: VerticalAlignment = VerticalAlignment {
            ord: 2i32
        };
        #[doc(alias = "VERTICAL_ALIGNMENT_FILL")]
        #[doc = "Godot enumerator name: `VERTICAL_ALIGNMENT_FILL`"]
        pub const FILL: VerticalAlignment = VerticalAlignment {
            ord: 3i32
        };
        
    }
    impl std::fmt::Debug for VerticalAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("VerticalAlignment") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for VerticalAlignment {
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
                Self::TOP => "TOP", Self::CENTER => "CENTER", Self::BOTTOM => "BOTTOM", Self::FILL => "FILL", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::TOP => "VERTICAL_ALIGNMENT_TOP", Self::CENTER => "VERTICAL_ALIGNMENT_CENTER", Self::BOTTOM => "VERTICAL_ALIGNMENT_BOTTOM", Self::FILL => "VERTICAL_ALIGNMENT_FILL", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for VerticalAlignment {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for VerticalAlignment {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for VerticalAlignment {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct InlineAlignment {
        ord: i32
    }
    impl InlineAlignment {
        #[doc(alias = "INLINE_ALIGNMENT_TOP_TO")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_TOP_TO`"]
        pub const TOP_TO: InlineAlignment = InlineAlignment {
            ord: 0i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_CENTER_TO")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_CENTER_TO`"]
        pub const CENTER_TO: InlineAlignment = InlineAlignment {
            ord: 1i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_BASELINE_TO")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_BASELINE_TO`"]
        pub const BASELINE_TO: InlineAlignment = InlineAlignment {
            ord: 3i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_BOTTOM_TO")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_BOTTOM_TO`"]
        pub const BOTTOM_TO: InlineAlignment = InlineAlignment {
            ord: 2i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_TO_TOP")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_TO_TOP`"]
        pub const TO_TOP: InlineAlignment = InlineAlignment {
            ord: 0i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_TO_CENTER")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_TO_CENTER`"]
        pub const TO_CENTER: InlineAlignment = InlineAlignment {
            ord: 4i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_TO_BASELINE")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_TO_BASELINE`"]
        pub const TO_BASELINE: InlineAlignment = InlineAlignment {
            ord: 8i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_TO_BOTTOM")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_TO_BOTTOM`"]
        pub const TO_BOTTOM: InlineAlignment = InlineAlignment {
            ord: 12i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_TOP")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_TOP`"]
        pub const TOP: InlineAlignment = InlineAlignment {
            ord: 0i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_CENTER")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_CENTER`"]
        pub const CENTER: InlineAlignment = InlineAlignment {
            ord: 5i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_BOTTOM")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_BOTTOM`"]
        pub const BOTTOM: InlineAlignment = InlineAlignment {
            ord: 14i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_IMAGE_MASK")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_IMAGE_MASK`"]
        pub const IMAGE_MASK: InlineAlignment = InlineAlignment {
            ord: 3i32
        };
        #[doc(alias = "INLINE_ALIGNMENT_TEXT_MASK")]
        #[doc = "Godot enumerator name: `INLINE_ALIGNMENT_TEXT_MASK`"]
        pub const TEXT_MASK: InlineAlignment = InlineAlignment {
            ord: 12i32
        };
        
    }
    impl std::fmt::Debug for InlineAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("InlineAlignment") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for InlineAlignment {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 8i32 | ord @ 12i32 | ord @ 14i32 => Some(Self {
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
                Self::TOP_TO => "TOP_TO", Self::CENTER_TO => "CENTER_TO", Self::BASELINE_TO => "BASELINE_TO", Self::BOTTOM_TO => "BOTTOM_TO", Self::TO_TOP => "TO_TOP", Self::TO_CENTER => "TO_CENTER", Self::TO_BASELINE => "TO_BASELINE", Self::TO_BOTTOM => "TO_BOTTOM", Self::TOP => "TOP", Self::CENTER => "CENTER", Self::BOTTOM => "BOTTOM", Self::IMAGE_MASK => "IMAGE_MASK", Self::TEXT_MASK => "TEXT_MASK", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::TOP_TO => "INLINE_ALIGNMENT_TOP_TO", Self::CENTER_TO => "INLINE_ALIGNMENT_CENTER_TO", Self::BASELINE_TO => "INLINE_ALIGNMENT_BASELINE_TO", Self::BOTTOM_TO => "INLINE_ALIGNMENT_BOTTOM_TO", Self::TO_TOP => "INLINE_ALIGNMENT_TO_TOP", Self::TO_CENTER => "INLINE_ALIGNMENT_TO_CENTER", Self::TO_BASELINE => "INLINE_ALIGNMENT_TO_BASELINE", Self::TO_BOTTOM => "INLINE_ALIGNMENT_TO_BOTTOM", Self::TOP => "INLINE_ALIGNMENT_TOP", Self::CENTER => "INLINE_ALIGNMENT_CENTER", Self::BOTTOM => "INLINE_ALIGNMENT_BOTTOM", Self::IMAGE_MASK => "INLINE_ALIGNMENT_IMAGE_MASK", Self::TEXT_MASK => "INLINE_ALIGNMENT_TEXT_MASK", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for InlineAlignment {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for InlineAlignment {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for InlineAlignment {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Key {
        ord: i32
    }
    impl Key {
        #[doc(alias = "KEY_NONE")]
        #[doc = "Godot enumerator name: `KEY_NONE`"]
        pub const NONE: Key = Key {
            ord: 0i32
        };
        #[doc(alias = "KEY_SPECIAL")]
        #[doc = "Godot enumerator name: `KEY_SPECIAL`"]
        pub const SPECIAL: Key = Key {
            ord: 4194304i32
        };
        #[doc(alias = "KEY_ESCAPE")]
        #[doc = "Godot enumerator name: `KEY_ESCAPE`"]
        pub const ESCAPE: Key = Key {
            ord: 4194305i32
        };
        #[doc(alias = "KEY_TAB")]
        #[doc = "Godot enumerator name: `KEY_TAB`"]
        pub const TAB: Key = Key {
            ord: 4194306i32
        };
        #[doc(alias = "KEY_BACKTAB")]
        #[doc = "Godot enumerator name: `KEY_BACKTAB`"]
        pub const BACKTAB: Key = Key {
            ord: 4194307i32
        };
        #[doc(alias = "KEY_BACKSPACE")]
        #[doc = "Godot enumerator name: `KEY_BACKSPACE`"]
        pub const BACKSPACE: Key = Key {
            ord: 4194308i32
        };
        #[doc(alias = "KEY_ENTER")]
        #[doc = "Godot enumerator name: `KEY_ENTER`"]
        pub const ENTER: Key = Key {
            ord: 4194309i32
        };
        #[doc(alias = "KEY_KP_ENTER")]
        #[doc = "Godot enumerator name: `KEY_KP_ENTER`"]
        pub const KP_ENTER: Key = Key {
            ord: 4194310i32
        };
        #[doc(alias = "KEY_INSERT")]
        #[doc = "Godot enumerator name: `KEY_INSERT`"]
        pub const INSERT: Key = Key {
            ord: 4194311i32
        };
        #[doc(alias = "KEY_DELETE")]
        #[doc = "Godot enumerator name: `KEY_DELETE`"]
        pub const DELETE: Key = Key {
            ord: 4194312i32
        };
        #[doc(alias = "KEY_PAUSE")]
        #[doc = "Godot enumerator name: `KEY_PAUSE`"]
        pub const PAUSE: Key = Key {
            ord: 4194313i32
        };
        #[doc(alias = "KEY_PRINT")]
        #[doc = "Godot enumerator name: `KEY_PRINT`"]
        pub const PRINT: Key = Key {
            ord: 4194314i32
        };
        #[doc(alias = "KEY_SYSREQ")]
        #[doc = "Godot enumerator name: `KEY_SYSREQ`"]
        pub const SYSREQ: Key = Key {
            ord: 4194315i32
        };
        #[doc(alias = "KEY_CLEAR")]
        #[doc = "Godot enumerator name: `KEY_CLEAR`"]
        pub const CLEAR: Key = Key {
            ord: 4194316i32
        };
        #[doc(alias = "KEY_HOME")]
        #[doc = "Godot enumerator name: `KEY_HOME`"]
        pub const HOME: Key = Key {
            ord: 4194317i32
        };
        #[doc(alias = "KEY_END")]
        #[doc = "Godot enumerator name: `KEY_END`"]
        pub const END: Key = Key {
            ord: 4194318i32
        };
        #[doc(alias = "KEY_LEFT")]
        #[doc = "Godot enumerator name: `KEY_LEFT`"]
        pub const LEFT: Key = Key {
            ord: 4194319i32
        };
        #[doc(alias = "KEY_UP")]
        #[doc = "Godot enumerator name: `KEY_UP`"]
        pub const UP: Key = Key {
            ord: 4194320i32
        };
        #[doc(alias = "KEY_RIGHT")]
        #[doc = "Godot enumerator name: `KEY_RIGHT`"]
        pub const RIGHT: Key = Key {
            ord: 4194321i32
        };
        #[doc(alias = "KEY_DOWN")]
        #[doc = "Godot enumerator name: `KEY_DOWN`"]
        pub const DOWN: Key = Key {
            ord: 4194322i32
        };
        #[doc(alias = "KEY_PAGEUP")]
        #[doc = "Godot enumerator name: `KEY_PAGEUP`"]
        pub const PAGEUP: Key = Key {
            ord: 4194323i32
        };
        #[doc(alias = "KEY_PAGEDOWN")]
        #[doc = "Godot enumerator name: `KEY_PAGEDOWN`"]
        pub const PAGEDOWN: Key = Key {
            ord: 4194324i32
        };
        #[doc(alias = "KEY_SHIFT")]
        #[doc = "Godot enumerator name: `KEY_SHIFT`"]
        pub const SHIFT: Key = Key {
            ord: 4194325i32
        };
        #[doc(alias = "KEY_CTRL")]
        #[doc = "Godot enumerator name: `KEY_CTRL`"]
        pub const CTRL: Key = Key {
            ord: 4194326i32
        };
        #[doc(alias = "KEY_META")]
        #[doc = "Godot enumerator name: `KEY_META`"]
        pub const META: Key = Key {
            ord: 4194327i32
        };
        #[doc(alias = "KEY_ALT")]
        #[doc = "Godot enumerator name: `KEY_ALT`"]
        pub const ALT: Key = Key {
            ord: 4194328i32
        };
        #[doc(alias = "KEY_CAPSLOCK")]
        #[doc = "Godot enumerator name: `KEY_CAPSLOCK`"]
        pub const CAPSLOCK: Key = Key {
            ord: 4194329i32
        };
        #[doc(alias = "KEY_NUMLOCK")]
        #[doc = "Godot enumerator name: `KEY_NUMLOCK`"]
        pub const NUMLOCK: Key = Key {
            ord: 4194330i32
        };
        #[doc(alias = "KEY_SCROLLLOCK")]
        #[doc = "Godot enumerator name: `KEY_SCROLLLOCK`"]
        pub const SCROLLLOCK: Key = Key {
            ord: 4194331i32
        };
        #[doc(alias = "KEY_F1")]
        #[doc = "Godot enumerator name: `KEY_F1`"]
        pub const F1: Key = Key {
            ord: 4194332i32
        };
        #[doc(alias = "KEY_F2")]
        #[doc = "Godot enumerator name: `KEY_F2`"]
        pub const F2: Key = Key {
            ord: 4194333i32
        };
        #[doc(alias = "KEY_F3")]
        #[doc = "Godot enumerator name: `KEY_F3`"]
        pub const F3: Key = Key {
            ord: 4194334i32
        };
        #[doc(alias = "KEY_F4")]
        #[doc = "Godot enumerator name: `KEY_F4`"]
        pub const F4: Key = Key {
            ord: 4194335i32
        };
        #[doc(alias = "KEY_F5")]
        #[doc = "Godot enumerator name: `KEY_F5`"]
        pub const F5: Key = Key {
            ord: 4194336i32
        };
        #[doc(alias = "KEY_F6")]
        #[doc = "Godot enumerator name: `KEY_F6`"]
        pub const F6: Key = Key {
            ord: 4194337i32
        };
        #[doc(alias = "KEY_F7")]
        #[doc = "Godot enumerator name: `KEY_F7`"]
        pub const F7: Key = Key {
            ord: 4194338i32
        };
        #[doc(alias = "KEY_F8")]
        #[doc = "Godot enumerator name: `KEY_F8`"]
        pub const F8: Key = Key {
            ord: 4194339i32
        };
        #[doc(alias = "KEY_F9")]
        #[doc = "Godot enumerator name: `KEY_F9`"]
        pub const F9: Key = Key {
            ord: 4194340i32
        };
        #[doc(alias = "KEY_F10")]
        #[doc = "Godot enumerator name: `KEY_F10`"]
        pub const F10: Key = Key {
            ord: 4194341i32
        };
        #[doc(alias = "KEY_F11")]
        #[doc = "Godot enumerator name: `KEY_F11`"]
        pub const F11: Key = Key {
            ord: 4194342i32
        };
        #[doc(alias = "KEY_F12")]
        #[doc = "Godot enumerator name: `KEY_F12`"]
        pub const F12: Key = Key {
            ord: 4194343i32
        };
        #[doc(alias = "KEY_F13")]
        #[doc = "Godot enumerator name: `KEY_F13`"]
        pub const F13: Key = Key {
            ord: 4194344i32
        };
        #[doc(alias = "KEY_F14")]
        #[doc = "Godot enumerator name: `KEY_F14`"]
        pub const F14: Key = Key {
            ord: 4194345i32
        };
        #[doc(alias = "KEY_F15")]
        #[doc = "Godot enumerator name: `KEY_F15`"]
        pub const F15: Key = Key {
            ord: 4194346i32
        };
        #[doc(alias = "KEY_F16")]
        #[doc = "Godot enumerator name: `KEY_F16`"]
        pub const F16: Key = Key {
            ord: 4194347i32
        };
        #[doc(alias = "KEY_F17")]
        #[doc = "Godot enumerator name: `KEY_F17`"]
        pub const F17: Key = Key {
            ord: 4194348i32
        };
        #[doc(alias = "KEY_F18")]
        #[doc = "Godot enumerator name: `KEY_F18`"]
        pub const F18: Key = Key {
            ord: 4194349i32
        };
        #[doc(alias = "KEY_F19")]
        #[doc = "Godot enumerator name: `KEY_F19`"]
        pub const F19: Key = Key {
            ord: 4194350i32
        };
        #[doc(alias = "KEY_F20")]
        #[doc = "Godot enumerator name: `KEY_F20`"]
        pub const F20: Key = Key {
            ord: 4194351i32
        };
        #[doc(alias = "KEY_F21")]
        #[doc = "Godot enumerator name: `KEY_F21`"]
        pub const F21: Key = Key {
            ord: 4194352i32
        };
        #[doc(alias = "KEY_F22")]
        #[doc = "Godot enumerator name: `KEY_F22`"]
        pub const F22: Key = Key {
            ord: 4194353i32
        };
        #[doc(alias = "KEY_F23")]
        #[doc = "Godot enumerator name: `KEY_F23`"]
        pub const F23: Key = Key {
            ord: 4194354i32
        };
        #[doc(alias = "KEY_F24")]
        #[doc = "Godot enumerator name: `KEY_F24`"]
        pub const F24: Key = Key {
            ord: 4194355i32
        };
        #[doc(alias = "KEY_F25")]
        #[doc = "Godot enumerator name: `KEY_F25`"]
        pub const F25: Key = Key {
            ord: 4194356i32
        };
        #[doc(alias = "KEY_F26")]
        #[doc = "Godot enumerator name: `KEY_F26`"]
        pub const F26: Key = Key {
            ord: 4194357i32
        };
        #[doc(alias = "KEY_F27")]
        #[doc = "Godot enumerator name: `KEY_F27`"]
        pub const F27: Key = Key {
            ord: 4194358i32
        };
        #[doc(alias = "KEY_F28")]
        #[doc = "Godot enumerator name: `KEY_F28`"]
        pub const F28: Key = Key {
            ord: 4194359i32
        };
        #[doc(alias = "KEY_F29")]
        #[doc = "Godot enumerator name: `KEY_F29`"]
        pub const F29: Key = Key {
            ord: 4194360i32
        };
        #[doc(alias = "KEY_F30")]
        #[doc = "Godot enumerator name: `KEY_F30`"]
        pub const F30: Key = Key {
            ord: 4194361i32
        };
        #[doc(alias = "KEY_F31")]
        #[doc = "Godot enumerator name: `KEY_F31`"]
        pub const F31: Key = Key {
            ord: 4194362i32
        };
        #[doc(alias = "KEY_F32")]
        #[doc = "Godot enumerator name: `KEY_F32`"]
        pub const F32: Key = Key {
            ord: 4194363i32
        };
        #[doc(alias = "KEY_F33")]
        #[doc = "Godot enumerator name: `KEY_F33`"]
        pub const F33: Key = Key {
            ord: 4194364i32
        };
        #[doc(alias = "KEY_F34")]
        #[doc = "Godot enumerator name: `KEY_F34`"]
        pub const F34: Key = Key {
            ord: 4194365i32
        };
        #[doc(alias = "KEY_F35")]
        #[doc = "Godot enumerator name: `KEY_F35`"]
        pub const F35: Key = Key {
            ord: 4194366i32
        };
        #[doc(alias = "KEY_KP_MULTIPLY")]
        #[doc = "Godot enumerator name: `KEY_KP_MULTIPLY`"]
        pub const KP_MULTIPLY: Key = Key {
            ord: 4194433i32
        };
        #[doc(alias = "KEY_KP_DIVIDE")]
        #[doc = "Godot enumerator name: `KEY_KP_DIVIDE`"]
        pub const KP_DIVIDE: Key = Key {
            ord: 4194434i32
        };
        #[doc(alias = "KEY_KP_SUBTRACT")]
        #[doc = "Godot enumerator name: `KEY_KP_SUBTRACT`"]
        pub const KP_SUBTRACT: Key = Key {
            ord: 4194435i32
        };
        #[doc(alias = "KEY_KP_PERIOD")]
        #[doc = "Godot enumerator name: `KEY_KP_PERIOD`"]
        pub const KP_PERIOD: Key = Key {
            ord: 4194436i32
        };
        #[doc(alias = "KEY_KP_ADD")]
        #[doc = "Godot enumerator name: `KEY_KP_ADD`"]
        pub const KP_ADD: Key = Key {
            ord: 4194437i32
        };
        #[doc(alias = "KEY_KP_0")]
        #[doc = "Godot enumerator name: `KEY_KP_0`"]
        pub const KP_0: Key = Key {
            ord: 4194438i32
        };
        #[doc(alias = "KEY_KP_1")]
        #[doc = "Godot enumerator name: `KEY_KP_1`"]
        pub const KP_1: Key = Key {
            ord: 4194439i32
        };
        #[doc(alias = "KEY_KP_2")]
        #[doc = "Godot enumerator name: `KEY_KP_2`"]
        pub const KP_2: Key = Key {
            ord: 4194440i32
        };
        #[doc(alias = "KEY_KP_3")]
        #[doc = "Godot enumerator name: `KEY_KP_3`"]
        pub const KP_3: Key = Key {
            ord: 4194441i32
        };
        #[doc(alias = "KEY_KP_4")]
        #[doc = "Godot enumerator name: `KEY_KP_4`"]
        pub const KP_4: Key = Key {
            ord: 4194442i32
        };
        #[doc(alias = "KEY_KP_5")]
        #[doc = "Godot enumerator name: `KEY_KP_5`"]
        pub const KP_5: Key = Key {
            ord: 4194443i32
        };
        #[doc(alias = "KEY_KP_6")]
        #[doc = "Godot enumerator name: `KEY_KP_6`"]
        pub const KP_6: Key = Key {
            ord: 4194444i32
        };
        #[doc(alias = "KEY_KP_7")]
        #[doc = "Godot enumerator name: `KEY_KP_7`"]
        pub const KP_7: Key = Key {
            ord: 4194445i32
        };
        #[doc(alias = "KEY_KP_8")]
        #[doc = "Godot enumerator name: `KEY_KP_8`"]
        pub const KP_8: Key = Key {
            ord: 4194446i32
        };
        #[doc(alias = "KEY_KP_9")]
        #[doc = "Godot enumerator name: `KEY_KP_9`"]
        pub const KP_9: Key = Key {
            ord: 4194447i32
        };
        #[doc(alias = "KEY_MENU")]
        #[doc = "Godot enumerator name: `KEY_MENU`"]
        pub const MENU: Key = Key {
            ord: 4194370i32
        };
        #[doc(alias = "KEY_HYPER")]
        #[doc = "Godot enumerator name: `KEY_HYPER`"]
        pub const HYPER: Key = Key {
            ord: 4194371i32
        };
        #[doc(alias = "KEY_HELP")]
        #[doc = "Godot enumerator name: `KEY_HELP`"]
        pub const HELP: Key = Key {
            ord: 4194373i32
        };
        #[doc(alias = "KEY_BACK")]
        #[doc = "Godot enumerator name: `KEY_BACK`"]
        pub const BACK: Key = Key {
            ord: 4194376i32
        };
        #[doc(alias = "KEY_FORWARD")]
        #[doc = "Godot enumerator name: `KEY_FORWARD`"]
        pub const FORWARD: Key = Key {
            ord: 4194377i32
        };
        #[doc(alias = "KEY_STOP")]
        #[doc = "Godot enumerator name: `KEY_STOP`"]
        pub const STOP: Key = Key {
            ord: 4194378i32
        };
        #[doc(alias = "KEY_REFRESH")]
        #[doc = "Godot enumerator name: `KEY_REFRESH`"]
        pub const REFRESH: Key = Key {
            ord: 4194379i32
        };
        #[doc(alias = "KEY_VOLUMEDOWN")]
        #[doc = "Godot enumerator name: `KEY_VOLUMEDOWN`"]
        pub const VOLUMEDOWN: Key = Key {
            ord: 4194380i32
        };
        #[doc(alias = "KEY_VOLUMEMUTE")]
        #[doc = "Godot enumerator name: `KEY_VOLUMEMUTE`"]
        pub const VOLUMEMUTE: Key = Key {
            ord: 4194381i32
        };
        #[doc(alias = "KEY_VOLUMEUP")]
        #[doc = "Godot enumerator name: `KEY_VOLUMEUP`"]
        pub const VOLUMEUP: Key = Key {
            ord: 4194382i32
        };
        #[doc(alias = "KEY_MEDIAPLAY")]
        #[doc = "Godot enumerator name: `KEY_MEDIAPLAY`"]
        pub const MEDIAPLAY: Key = Key {
            ord: 4194388i32
        };
        #[doc(alias = "KEY_MEDIASTOP")]
        #[doc = "Godot enumerator name: `KEY_MEDIASTOP`"]
        pub const MEDIASTOP: Key = Key {
            ord: 4194389i32
        };
        #[doc(alias = "KEY_MEDIAPREVIOUS")]
        #[doc = "Godot enumerator name: `KEY_MEDIAPREVIOUS`"]
        pub const MEDIAPREVIOUS: Key = Key {
            ord: 4194390i32
        };
        #[doc(alias = "KEY_MEDIANEXT")]
        #[doc = "Godot enumerator name: `KEY_MEDIANEXT`"]
        pub const MEDIANEXT: Key = Key {
            ord: 4194391i32
        };
        #[doc(alias = "KEY_MEDIARECORD")]
        #[doc = "Godot enumerator name: `KEY_MEDIARECORD`"]
        pub const MEDIARECORD: Key = Key {
            ord: 4194392i32
        };
        #[doc(alias = "KEY_HOMEPAGE")]
        #[doc = "Godot enumerator name: `KEY_HOMEPAGE`"]
        pub const HOMEPAGE: Key = Key {
            ord: 4194393i32
        };
        #[doc(alias = "KEY_FAVORITES")]
        #[doc = "Godot enumerator name: `KEY_FAVORITES`"]
        pub const FAVORITES: Key = Key {
            ord: 4194394i32
        };
        #[doc(alias = "KEY_SEARCH")]
        #[doc = "Godot enumerator name: `KEY_SEARCH`"]
        pub const SEARCH: Key = Key {
            ord: 4194395i32
        };
        #[doc(alias = "KEY_STANDBY")]
        #[doc = "Godot enumerator name: `KEY_STANDBY`"]
        pub const STANDBY: Key = Key {
            ord: 4194396i32
        };
        #[doc(alias = "KEY_OPENURL")]
        #[doc = "Godot enumerator name: `KEY_OPENURL`"]
        pub const OPENURL: Key = Key {
            ord: 4194397i32
        };
        #[doc(alias = "KEY_LAUNCHMAIL")]
        #[doc = "Godot enumerator name: `KEY_LAUNCHMAIL`"]
        pub const LAUNCHMAIL: Key = Key {
            ord: 4194398i32
        };
        #[doc(alias = "KEY_LAUNCHMEDIA")]
        #[doc = "Godot enumerator name: `KEY_LAUNCHMEDIA`"]
        pub const LAUNCHMEDIA: Key = Key {
            ord: 4194399i32
        };
        #[doc(alias = "KEY_LAUNCH0")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH0`"]
        pub const LAUNCH0: Key = Key {
            ord: 4194400i32
        };
        #[doc(alias = "KEY_LAUNCH1")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH1`"]
        pub const LAUNCH1: Key = Key {
            ord: 4194401i32
        };
        #[doc(alias = "KEY_LAUNCH2")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH2`"]
        pub const LAUNCH2: Key = Key {
            ord: 4194402i32
        };
        #[doc(alias = "KEY_LAUNCH3")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH3`"]
        pub const LAUNCH3: Key = Key {
            ord: 4194403i32
        };
        #[doc(alias = "KEY_LAUNCH4")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH4`"]
        pub const LAUNCH4: Key = Key {
            ord: 4194404i32
        };
        #[doc(alias = "KEY_LAUNCH5")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH5`"]
        pub const LAUNCH5: Key = Key {
            ord: 4194405i32
        };
        #[doc(alias = "KEY_LAUNCH6")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH6`"]
        pub const LAUNCH6: Key = Key {
            ord: 4194406i32
        };
        #[doc(alias = "KEY_LAUNCH7")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH7`"]
        pub const LAUNCH7: Key = Key {
            ord: 4194407i32
        };
        #[doc(alias = "KEY_LAUNCH8")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH8`"]
        pub const LAUNCH8: Key = Key {
            ord: 4194408i32
        };
        #[doc(alias = "KEY_LAUNCH9")]
        #[doc = "Godot enumerator name: `KEY_LAUNCH9`"]
        pub const LAUNCH9: Key = Key {
            ord: 4194409i32
        };
        #[doc(alias = "KEY_LAUNCHA")]
        #[doc = "Godot enumerator name: `KEY_LAUNCHA`"]
        pub const LAUNCHA: Key = Key {
            ord: 4194410i32
        };
        #[doc(alias = "KEY_LAUNCHB")]
        #[doc = "Godot enumerator name: `KEY_LAUNCHB`"]
        pub const LAUNCHB: Key = Key {
            ord: 4194411i32
        };
        #[doc(alias = "KEY_LAUNCHC")]
        #[doc = "Godot enumerator name: `KEY_LAUNCHC`"]
        pub const LAUNCHC: Key = Key {
            ord: 4194412i32
        };
        #[doc(alias = "KEY_LAUNCHD")]
        #[doc = "Godot enumerator name: `KEY_LAUNCHD`"]
        pub const LAUNCHD: Key = Key {
            ord: 4194413i32
        };
        #[doc(alias = "KEY_LAUNCHE")]
        #[doc = "Godot enumerator name: `KEY_LAUNCHE`"]
        pub const LAUNCHE: Key = Key {
            ord: 4194414i32
        };
        #[doc(alias = "KEY_LAUNCHF")]
        #[doc = "Godot enumerator name: `KEY_LAUNCHF`"]
        pub const LAUNCHF: Key = Key {
            ord: 4194415i32
        };
        #[doc(alias = "KEY_GLOBE")]
        #[doc = "Godot enumerator name: `KEY_GLOBE`"]
        pub const GLOBE: Key = Key {
            ord: 4194416i32
        };
        #[doc(alias = "KEY_KEYBOARD")]
        #[doc = "Godot enumerator name: `KEY_KEYBOARD`"]
        pub const KEYBOARD: Key = Key {
            ord: 4194417i32
        };
        #[doc(alias = "KEY_JIS_EISU")]
        #[doc = "Godot enumerator name: `KEY_JIS_EISU`"]
        pub const JIS_EISU: Key = Key {
            ord: 4194418i32
        };
        #[doc(alias = "KEY_JIS_KANA")]
        #[doc = "Godot enumerator name: `KEY_JIS_KANA`"]
        pub const JIS_KANA: Key = Key {
            ord: 4194419i32
        };
        #[doc(alias = "KEY_UNKNOWN")]
        #[doc = "Godot enumerator name: `KEY_UNKNOWN`"]
        pub const UNKNOWN: Key = Key {
            ord: 8388607i32
        };
        #[doc(alias = "KEY_SPACE")]
        #[doc = "Godot enumerator name: `KEY_SPACE`"]
        pub const SPACE: Key = Key {
            ord: 32i32
        };
        #[doc(alias = "KEY_EXCLAM")]
        #[doc = "Godot enumerator name: `KEY_EXCLAM`"]
        pub const EXCLAM: Key = Key {
            ord: 33i32
        };
        #[doc(alias = "KEY_QUOTEDBL")]
        #[doc = "Godot enumerator name: `KEY_QUOTEDBL`"]
        pub const QUOTEDBL: Key = Key {
            ord: 34i32
        };
        #[doc(alias = "KEY_NUMBERSIGN")]
        #[doc = "Godot enumerator name: `KEY_NUMBERSIGN`"]
        pub const NUMBERSIGN: Key = Key {
            ord: 35i32
        };
        #[doc(alias = "KEY_DOLLAR")]
        #[doc = "Godot enumerator name: `KEY_DOLLAR`"]
        pub const DOLLAR: Key = Key {
            ord: 36i32
        };
        #[doc(alias = "KEY_PERCENT")]
        #[doc = "Godot enumerator name: `KEY_PERCENT`"]
        pub const PERCENT: Key = Key {
            ord: 37i32
        };
        #[doc(alias = "KEY_AMPERSAND")]
        #[doc = "Godot enumerator name: `KEY_AMPERSAND`"]
        pub const AMPERSAND: Key = Key {
            ord: 38i32
        };
        #[doc(alias = "KEY_APOSTROPHE")]
        #[doc = "Godot enumerator name: `KEY_APOSTROPHE`"]
        pub const APOSTROPHE: Key = Key {
            ord: 39i32
        };
        #[doc(alias = "KEY_PARENLEFT")]
        #[doc = "Godot enumerator name: `KEY_PARENLEFT`"]
        pub const PARENLEFT: Key = Key {
            ord: 40i32
        };
        #[doc(alias = "KEY_PARENRIGHT")]
        #[doc = "Godot enumerator name: `KEY_PARENRIGHT`"]
        pub const PARENRIGHT: Key = Key {
            ord: 41i32
        };
        #[doc(alias = "KEY_ASTERISK")]
        #[doc = "Godot enumerator name: `KEY_ASTERISK`"]
        pub const ASTERISK: Key = Key {
            ord: 42i32
        };
        #[doc(alias = "KEY_PLUS")]
        #[doc = "Godot enumerator name: `KEY_PLUS`"]
        pub const PLUS: Key = Key {
            ord: 43i32
        };
        #[doc(alias = "KEY_COMMA")]
        #[doc = "Godot enumerator name: `KEY_COMMA`"]
        pub const COMMA: Key = Key {
            ord: 44i32
        };
        #[doc(alias = "KEY_MINUS")]
        #[doc = "Godot enumerator name: `KEY_MINUS`"]
        pub const MINUS: Key = Key {
            ord: 45i32
        };
        #[doc(alias = "KEY_PERIOD")]
        #[doc = "Godot enumerator name: `KEY_PERIOD`"]
        pub const PERIOD: Key = Key {
            ord: 46i32
        };
        #[doc(alias = "KEY_SLASH")]
        #[doc = "Godot enumerator name: `KEY_SLASH`"]
        pub const SLASH: Key = Key {
            ord: 47i32
        };
        pub const KEY_0: Key = Key {
            ord: 48i32
        };
        pub const KEY_1: Key = Key {
            ord: 49i32
        };
        pub const KEY_2: Key = Key {
            ord: 50i32
        };
        pub const KEY_3: Key = Key {
            ord: 51i32
        };
        pub const KEY_4: Key = Key {
            ord: 52i32
        };
        pub const KEY_5: Key = Key {
            ord: 53i32
        };
        pub const KEY_6: Key = Key {
            ord: 54i32
        };
        pub const KEY_7: Key = Key {
            ord: 55i32
        };
        pub const KEY_8: Key = Key {
            ord: 56i32
        };
        pub const KEY_9: Key = Key {
            ord: 57i32
        };
        #[doc(alias = "KEY_COLON")]
        #[doc = "Godot enumerator name: `KEY_COLON`"]
        pub const COLON: Key = Key {
            ord: 58i32
        };
        #[doc(alias = "KEY_SEMICOLON")]
        #[doc = "Godot enumerator name: `KEY_SEMICOLON`"]
        pub const SEMICOLON: Key = Key {
            ord: 59i32
        };
        #[doc(alias = "KEY_LESS")]
        #[doc = "Godot enumerator name: `KEY_LESS`"]
        pub const LESS: Key = Key {
            ord: 60i32
        };
        #[doc(alias = "KEY_EQUAL")]
        #[doc = "Godot enumerator name: `KEY_EQUAL`"]
        pub const EQUAL: Key = Key {
            ord: 61i32
        };
        #[doc(alias = "KEY_GREATER")]
        #[doc = "Godot enumerator name: `KEY_GREATER`"]
        pub const GREATER: Key = Key {
            ord: 62i32
        };
        #[doc(alias = "KEY_QUESTION")]
        #[doc = "Godot enumerator name: `KEY_QUESTION`"]
        pub const QUESTION: Key = Key {
            ord: 63i32
        };
        #[doc(alias = "KEY_AT")]
        #[doc = "Godot enumerator name: `KEY_AT`"]
        pub const AT: Key = Key {
            ord: 64i32
        };
        #[doc(alias = "KEY_A")]
        #[doc = "Godot enumerator name: `KEY_A`"]
        pub const A: Key = Key {
            ord: 65i32
        };
        #[doc(alias = "KEY_B")]
        #[doc = "Godot enumerator name: `KEY_B`"]
        pub const B: Key = Key {
            ord: 66i32
        };
        #[doc(alias = "KEY_C")]
        #[doc = "Godot enumerator name: `KEY_C`"]
        pub const C: Key = Key {
            ord: 67i32
        };
        #[doc(alias = "KEY_D")]
        #[doc = "Godot enumerator name: `KEY_D`"]
        pub const D: Key = Key {
            ord: 68i32
        };
        #[doc(alias = "KEY_E")]
        #[doc = "Godot enumerator name: `KEY_E`"]
        pub const E: Key = Key {
            ord: 69i32
        };
        #[doc(alias = "KEY_F")]
        #[doc = "Godot enumerator name: `KEY_F`"]
        pub const F: Key = Key {
            ord: 70i32
        };
        #[doc(alias = "KEY_G")]
        #[doc = "Godot enumerator name: `KEY_G`"]
        pub const G: Key = Key {
            ord: 71i32
        };
        #[doc(alias = "KEY_H")]
        #[doc = "Godot enumerator name: `KEY_H`"]
        pub const H: Key = Key {
            ord: 72i32
        };
        #[doc(alias = "KEY_I")]
        #[doc = "Godot enumerator name: `KEY_I`"]
        pub const I: Key = Key {
            ord: 73i32
        };
        #[doc(alias = "KEY_J")]
        #[doc = "Godot enumerator name: `KEY_J`"]
        pub const J: Key = Key {
            ord: 74i32
        };
        #[doc(alias = "KEY_K")]
        #[doc = "Godot enumerator name: `KEY_K`"]
        pub const K: Key = Key {
            ord: 75i32
        };
        #[doc(alias = "KEY_L")]
        #[doc = "Godot enumerator name: `KEY_L`"]
        pub const L: Key = Key {
            ord: 76i32
        };
        #[doc(alias = "KEY_M")]
        #[doc = "Godot enumerator name: `KEY_M`"]
        pub const M: Key = Key {
            ord: 77i32
        };
        #[doc(alias = "KEY_N")]
        #[doc = "Godot enumerator name: `KEY_N`"]
        pub const N: Key = Key {
            ord: 78i32
        };
        #[doc(alias = "KEY_O")]
        #[doc = "Godot enumerator name: `KEY_O`"]
        pub const O: Key = Key {
            ord: 79i32
        };
        #[doc(alias = "KEY_P")]
        #[doc = "Godot enumerator name: `KEY_P`"]
        pub const P: Key = Key {
            ord: 80i32
        };
        #[doc(alias = "KEY_Q")]
        #[doc = "Godot enumerator name: `KEY_Q`"]
        pub const Q: Key = Key {
            ord: 81i32
        };
        #[doc(alias = "KEY_R")]
        #[doc = "Godot enumerator name: `KEY_R`"]
        pub const R: Key = Key {
            ord: 82i32
        };
        #[doc(alias = "KEY_S")]
        #[doc = "Godot enumerator name: `KEY_S`"]
        pub const S: Key = Key {
            ord: 83i32
        };
        #[doc(alias = "KEY_T")]
        #[doc = "Godot enumerator name: `KEY_T`"]
        pub const T: Key = Key {
            ord: 84i32
        };
        #[doc(alias = "KEY_U")]
        #[doc = "Godot enumerator name: `KEY_U`"]
        pub const U: Key = Key {
            ord: 85i32
        };
        #[doc(alias = "KEY_V")]
        #[doc = "Godot enumerator name: `KEY_V`"]
        pub const V: Key = Key {
            ord: 86i32
        };
        #[doc(alias = "KEY_W")]
        #[doc = "Godot enumerator name: `KEY_W`"]
        pub const W: Key = Key {
            ord: 87i32
        };
        #[doc(alias = "KEY_X")]
        #[doc = "Godot enumerator name: `KEY_X`"]
        pub const X: Key = Key {
            ord: 88i32
        };
        #[doc(alias = "KEY_Y")]
        #[doc = "Godot enumerator name: `KEY_Y`"]
        pub const Y: Key = Key {
            ord: 89i32
        };
        #[doc(alias = "KEY_Z")]
        #[doc = "Godot enumerator name: `KEY_Z`"]
        pub const Z: Key = Key {
            ord: 90i32
        };
        #[doc(alias = "KEY_BRACKETLEFT")]
        #[doc = "Godot enumerator name: `KEY_BRACKETLEFT`"]
        pub const BRACKETLEFT: Key = Key {
            ord: 91i32
        };
        #[doc(alias = "KEY_BACKSLASH")]
        #[doc = "Godot enumerator name: `KEY_BACKSLASH`"]
        pub const BACKSLASH: Key = Key {
            ord: 92i32
        };
        #[doc(alias = "KEY_BRACKETRIGHT")]
        #[doc = "Godot enumerator name: `KEY_BRACKETRIGHT`"]
        pub const BRACKETRIGHT: Key = Key {
            ord: 93i32
        };
        #[doc(alias = "KEY_ASCIICIRCUM")]
        #[doc = "Godot enumerator name: `KEY_ASCIICIRCUM`"]
        pub const ASCIICIRCUM: Key = Key {
            ord: 94i32
        };
        #[doc(alias = "KEY_UNDERSCORE")]
        #[doc = "Godot enumerator name: `KEY_UNDERSCORE`"]
        pub const UNDERSCORE: Key = Key {
            ord: 95i32
        };
        #[doc(alias = "KEY_QUOTELEFT")]
        #[doc = "Godot enumerator name: `KEY_QUOTELEFT`"]
        pub const QUOTELEFT: Key = Key {
            ord: 96i32
        };
        #[doc(alias = "KEY_BRACELEFT")]
        #[doc = "Godot enumerator name: `KEY_BRACELEFT`"]
        pub const BRACELEFT: Key = Key {
            ord: 123i32
        };
        #[doc(alias = "KEY_BAR")]
        #[doc = "Godot enumerator name: `KEY_BAR`"]
        pub const BAR: Key = Key {
            ord: 124i32
        };
        #[doc(alias = "KEY_BRACERIGHT")]
        #[doc = "Godot enumerator name: `KEY_BRACERIGHT`"]
        pub const BRACERIGHT: Key = Key {
            ord: 125i32
        };
        #[doc(alias = "KEY_ASCIITILDE")]
        #[doc = "Godot enumerator name: `KEY_ASCIITILDE`"]
        pub const ASCIITILDE: Key = Key {
            ord: 126i32
        };
        #[doc(alias = "KEY_YEN")]
        #[doc = "Godot enumerator name: `KEY_YEN`"]
        pub const YEN: Key = Key {
            ord: 165i32
        };
        #[doc(alias = "KEY_SECTION")]
        #[doc = "Godot enumerator name: `KEY_SECTION`"]
        pub const SECTION: Key = Key {
            ord: 167i32
        };
        
    }
    impl std::fmt::Debug for Key {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("Key") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for Key {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 | ord @ 48i32 | ord @ 49i32 | ord @ 50i32 | ord @ 51i32 | ord @ 52i32 | ord @ 53i32 | ord @ 54i32 | ord @ 55i32 | ord @ 56i32 | ord @ 57i32 | ord @ 58i32 | ord @ 59i32 | ord @ 60i32 | ord @ 61i32 | ord @ 62i32 | ord @ 63i32 | ord @ 64i32 | ord @ 65i32 | ord @ 66i32 | ord @ 67i32 | ord @ 68i32 | ord @ 69i32 | ord @ 70i32 | ord @ 71i32 | ord @ 72i32 | ord @ 73i32 | ord @ 74i32 | ord @ 75i32 | ord @ 76i32 | ord @ 77i32 | ord @ 78i32 | ord @ 79i32 | ord @ 80i32 | ord @ 81i32 | ord @ 82i32 | ord @ 83i32 | ord @ 84i32 | ord @ 85i32 | ord @ 86i32 | ord @ 87i32 | ord @ 88i32 | ord @ 89i32 | ord @ 90i32 | ord @ 91i32 | ord @ 92i32 | ord @ 93i32 | ord @ 94i32 | ord @ 95i32 | ord @ 96i32 | ord @ 123i32 | ord @ 124i32 | ord @ 125i32 | ord @ 126i32 | ord @ 165i32 | ord @ 167i32 | ord @ 4194304i32 | ord @ 4194305i32 | ord @ 4194306i32 | ord @ 4194307i32 | ord @ 4194308i32 | ord @ 4194309i32 | ord @ 4194310i32 | ord @ 4194311i32 | ord @ 4194312i32 | ord @ 4194313i32 | ord @ 4194314i32 | ord @ 4194315i32 | ord @ 4194316i32 | ord @ 4194317i32 | ord @ 4194318i32 | ord @ 4194319i32 | ord @ 4194320i32 | ord @ 4194321i32 | ord @ 4194322i32 | ord @ 4194323i32 | ord @ 4194324i32 | ord @ 4194325i32 | ord @ 4194326i32 | ord @ 4194327i32 | ord @ 4194328i32 | ord @ 4194329i32 | ord @ 4194330i32 | ord @ 4194331i32 | ord @ 4194332i32 | ord @ 4194333i32 | ord @ 4194334i32 | ord @ 4194335i32 | ord @ 4194336i32 | ord @ 4194337i32 | ord @ 4194338i32 | ord @ 4194339i32 | ord @ 4194340i32 | ord @ 4194341i32 | ord @ 4194342i32 | ord @ 4194343i32 | ord @ 4194344i32 | ord @ 4194345i32 | ord @ 4194346i32 | ord @ 4194347i32 | ord @ 4194348i32 | ord @ 4194349i32 | ord @ 4194350i32 | ord @ 4194351i32 | ord @ 4194352i32 | ord @ 4194353i32 | ord @ 4194354i32 | ord @ 4194355i32 | ord @ 4194356i32 | ord @ 4194357i32 | ord @ 4194358i32 | ord @ 4194359i32 | ord @ 4194360i32 | ord @ 4194361i32 | ord @ 4194362i32 | ord @ 4194363i32 | ord @ 4194364i32 | ord @ 4194365i32 | ord @ 4194366i32 | ord @ 4194370i32 | ord @ 4194371i32 | ord @ 4194373i32 | ord @ 4194376i32 | ord @ 4194377i32 | ord @ 4194378i32 | ord @ 4194379i32 | ord @ 4194380i32 | ord @ 4194381i32 | ord @ 4194382i32 | ord @ 4194388i32 | ord @ 4194389i32 | ord @ 4194390i32 | ord @ 4194391i32 | ord @ 4194392i32 | ord @ 4194393i32 | ord @ 4194394i32 | ord @ 4194395i32 | ord @ 4194396i32 | ord @ 4194397i32 | ord @ 4194398i32 | ord @ 4194399i32 | ord @ 4194400i32 | ord @ 4194401i32 | ord @ 4194402i32 | ord @ 4194403i32 | ord @ 4194404i32 | ord @ 4194405i32 | ord @ 4194406i32 | ord @ 4194407i32 | ord @ 4194408i32 | ord @ 4194409i32 | ord @ 4194410i32 | ord @ 4194411i32 | ord @ 4194412i32 | ord @ 4194413i32 | ord @ 4194414i32 | ord @ 4194415i32 | ord @ 4194416i32 | ord @ 4194417i32 | ord @ 4194418i32 | ord @ 4194419i32 | ord @ 4194433i32 | ord @ 4194434i32 | ord @ 4194435i32 | ord @ 4194436i32 | ord @ 4194437i32 | ord @ 4194438i32 | ord @ 4194439i32 | ord @ 4194440i32 | ord @ 4194441i32 | ord @ 4194442i32 | ord @ 4194443i32 | ord @ 4194444i32 | ord @ 4194445i32 | ord @ 4194446i32 | ord @ 4194447i32 | ord @ 8388607i32 => Some(Self {
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
                Self::NONE => "NONE", Self::SPECIAL => "SPECIAL", Self::ESCAPE => "ESCAPE", Self::TAB => "TAB", Self::BACKTAB => "BACKTAB", Self::BACKSPACE => "BACKSPACE", Self::ENTER => "ENTER", Self::KP_ENTER => "KP_ENTER", Self::INSERT => "INSERT", Self::DELETE => "DELETE", Self::PAUSE => "PAUSE", Self::PRINT => "PRINT", Self::SYSREQ => "SYSREQ", Self::CLEAR => "CLEAR", Self::HOME => "HOME", Self::END => "END", Self::LEFT => "LEFT", Self::UP => "UP", Self::RIGHT => "RIGHT", Self::DOWN => "DOWN", Self::PAGEUP => "PAGEUP", Self::PAGEDOWN => "PAGEDOWN", Self::SHIFT => "SHIFT", Self::CTRL => "CTRL", Self::META => "META", Self::ALT => "ALT", Self::CAPSLOCK => "CAPSLOCK", Self::NUMLOCK => "NUMLOCK", Self::SCROLLLOCK => "SCROLLLOCK", Self::F1 => "F1", Self::F2 => "F2", Self::F3 => "F3", Self::F4 => "F4", Self::F5 => "F5", Self::F6 => "F6", Self::F7 => "F7", Self::F8 => "F8", Self::F9 => "F9", Self::F10 => "F10", Self::F11 => "F11", Self::F12 => "F12", Self::F13 => "F13", Self::F14 => "F14", Self::F15 => "F15", Self::F16 => "F16", Self::F17 => "F17", Self::F18 => "F18", Self::F19 => "F19", Self::F20 => "F20", Self::F21 => "F21", Self::F22 => "F22", Self::F23 => "F23", Self::F24 => "F24", Self::F25 => "F25", Self::F26 => "F26", Self::F27 => "F27", Self::F28 => "F28", Self::F29 => "F29", Self::F30 => "F30", Self::F31 => "F31", Self::F32 => "F32", Self::F33 => "F33", Self::F34 => "F34", Self::F35 => "F35", Self::KP_MULTIPLY => "KP_MULTIPLY", Self::KP_DIVIDE => "KP_DIVIDE", Self::KP_SUBTRACT => "KP_SUBTRACT", Self::KP_PERIOD => "KP_PERIOD", Self::KP_ADD => "KP_ADD", Self::KP_0 => "KP_0", Self::KP_1 => "KP_1", Self::KP_2 => "KP_2", Self::KP_3 => "KP_3", Self::KP_4 => "KP_4", Self::KP_5 => "KP_5", Self::KP_6 => "KP_6", Self::KP_7 => "KP_7", Self::KP_8 => "KP_8", Self::KP_9 => "KP_9", Self::MENU => "MENU", Self::HYPER => "HYPER", Self::HELP => "HELP", Self::BACK => "BACK", Self::FORWARD => "FORWARD", Self::STOP => "STOP", Self::REFRESH => "REFRESH", Self::VOLUMEDOWN => "VOLUMEDOWN", Self::VOLUMEMUTE => "VOLUMEMUTE", Self::VOLUMEUP => "VOLUMEUP", Self::MEDIAPLAY => "MEDIAPLAY", Self::MEDIASTOP => "MEDIASTOP", Self::MEDIAPREVIOUS => "MEDIAPREVIOUS", Self::MEDIANEXT => "MEDIANEXT", Self::MEDIARECORD => "MEDIARECORD", Self::HOMEPAGE => "HOMEPAGE", Self::FAVORITES => "FAVORITES", Self::SEARCH => "SEARCH", Self::STANDBY => "STANDBY", Self::OPENURL => "OPENURL", Self::LAUNCHMAIL => "LAUNCHMAIL", Self::LAUNCHMEDIA => "LAUNCHMEDIA", Self::LAUNCH0 => "LAUNCH0", Self::LAUNCH1 => "LAUNCH1", Self::LAUNCH2 => "LAUNCH2", Self::LAUNCH3 => "LAUNCH3", Self::LAUNCH4 => "LAUNCH4", Self::LAUNCH5 => "LAUNCH5", Self::LAUNCH6 => "LAUNCH6", Self::LAUNCH7 => "LAUNCH7", Self::LAUNCH8 => "LAUNCH8", Self::LAUNCH9 => "LAUNCH9", Self::LAUNCHA => "LAUNCHA", Self::LAUNCHB => "LAUNCHB", Self::LAUNCHC => "LAUNCHC", Self::LAUNCHD => "LAUNCHD", Self::LAUNCHE => "LAUNCHE", Self::LAUNCHF => "LAUNCHF", Self::GLOBE => "GLOBE", Self::KEYBOARD => "KEYBOARD", Self::JIS_EISU => "JIS_EISU", Self::JIS_KANA => "JIS_KANA", Self::UNKNOWN => "UNKNOWN", Self::SPACE => "SPACE", Self::EXCLAM => "EXCLAM", Self::QUOTEDBL => "QUOTEDBL", Self::NUMBERSIGN => "NUMBERSIGN", Self::DOLLAR => "DOLLAR", Self::PERCENT => "PERCENT", Self::AMPERSAND => "AMPERSAND", Self::APOSTROPHE => "APOSTROPHE", Self::PARENLEFT => "PARENLEFT", Self::PARENRIGHT => "PARENRIGHT", Self::ASTERISK => "ASTERISK", Self::PLUS => "PLUS", Self::COMMA => "COMMA", Self::MINUS => "MINUS", Self::PERIOD => "PERIOD", Self::SLASH => "SLASH", Self::KEY_0 => "KEY_0", Self::KEY_1 => "KEY_1", Self::KEY_2 => "KEY_2", Self::KEY_3 => "KEY_3", Self::KEY_4 => "KEY_4", Self::KEY_5 => "KEY_5", Self::KEY_6 => "KEY_6", Self::KEY_7 => "KEY_7", Self::KEY_8 => "KEY_8", Self::KEY_9 => "KEY_9", Self::COLON => "COLON", Self::SEMICOLON => "SEMICOLON", Self::LESS => "LESS", Self::EQUAL => "EQUAL", Self::GREATER => "GREATER", Self::QUESTION => "QUESTION", Self::AT => "AT", Self::A => "A", Self::B => "B", Self::C => "C", Self::D => "D", Self::E => "E", Self::F => "F", Self::G => "G", Self::H => "H", Self::I => "I", Self::J => "J", Self::K => "K", Self::L => "L", Self::M => "M", Self::N => "N", Self::O => "O", Self::P => "P", Self::Q => "Q", Self::R => "R", Self::S => "S", Self::T => "T", Self::U => "U", Self::V => "V", Self::W => "W", Self::X => "X", Self::Y => "Y", Self::Z => "Z", Self::BRACKETLEFT => "BRACKETLEFT", Self::BACKSLASH => "BACKSLASH", Self::BRACKETRIGHT => "BRACKETRIGHT", Self::ASCIICIRCUM => "ASCIICIRCUM", Self::UNDERSCORE => "UNDERSCORE", Self::QUOTELEFT => "QUOTELEFT", Self::BRACELEFT => "BRACELEFT", Self::BAR => "BAR", Self::BRACERIGHT => "BRACERIGHT", Self::ASCIITILDE => "ASCIITILDE", Self::YEN => "YEN", Self::SECTION => "SECTION", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::NONE => "KEY_NONE", Self::SPECIAL => "KEY_SPECIAL", Self::ESCAPE => "KEY_ESCAPE", Self::TAB => "KEY_TAB", Self::BACKTAB => "KEY_BACKTAB", Self::BACKSPACE => "KEY_BACKSPACE", Self::ENTER => "KEY_ENTER", Self::KP_ENTER => "KEY_KP_ENTER", Self::INSERT => "KEY_INSERT", Self::DELETE => "KEY_DELETE", Self::PAUSE => "KEY_PAUSE", Self::PRINT => "KEY_PRINT", Self::SYSREQ => "KEY_SYSREQ", Self::CLEAR => "KEY_CLEAR", Self::HOME => "KEY_HOME", Self::END => "KEY_END", Self::LEFT => "KEY_LEFT", Self::UP => "KEY_UP", Self::RIGHT => "KEY_RIGHT", Self::DOWN => "KEY_DOWN", Self::PAGEUP => "KEY_PAGEUP", Self::PAGEDOWN => "KEY_PAGEDOWN", Self::SHIFT => "KEY_SHIFT", Self::CTRL => "KEY_CTRL", Self::META => "KEY_META", Self::ALT => "KEY_ALT", Self::CAPSLOCK => "KEY_CAPSLOCK", Self::NUMLOCK => "KEY_NUMLOCK", Self::SCROLLLOCK => "KEY_SCROLLLOCK", Self::F1 => "KEY_F1", Self::F2 => "KEY_F2", Self::F3 => "KEY_F3", Self::F4 => "KEY_F4", Self::F5 => "KEY_F5", Self::F6 => "KEY_F6", Self::F7 => "KEY_F7", Self::F8 => "KEY_F8", Self::F9 => "KEY_F9", Self::F10 => "KEY_F10", Self::F11 => "KEY_F11", Self::F12 => "KEY_F12", Self::F13 => "KEY_F13", Self::F14 => "KEY_F14", Self::F15 => "KEY_F15", Self::F16 => "KEY_F16", Self::F17 => "KEY_F17", Self::F18 => "KEY_F18", Self::F19 => "KEY_F19", Self::F20 => "KEY_F20", Self::F21 => "KEY_F21", Self::F22 => "KEY_F22", Self::F23 => "KEY_F23", Self::F24 => "KEY_F24", Self::F25 => "KEY_F25", Self::F26 => "KEY_F26", Self::F27 => "KEY_F27", Self::F28 => "KEY_F28", Self::F29 => "KEY_F29", Self::F30 => "KEY_F30", Self::F31 => "KEY_F31", Self::F32 => "KEY_F32", Self::F33 => "KEY_F33", Self::F34 => "KEY_F34", Self::F35 => "KEY_F35", Self::KP_MULTIPLY => "KEY_KP_MULTIPLY", Self::KP_DIVIDE => "KEY_KP_DIVIDE", Self::KP_SUBTRACT => "KEY_KP_SUBTRACT", Self::KP_PERIOD => "KEY_KP_PERIOD", Self::KP_ADD => "KEY_KP_ADD", Self::KP_0 => "KEY_KP_0", Self::KP_1 => "KEY_KP_1", Self::KP_2 => "KEY_KP_2", Self::KP_3 => "KEY_KP_3", Self::KP_4 => "KEY_KP_4", Self::KP_5 => "KEY_KP_5", Self::KP_6 => "KEY_KP_6", Self::KP_7 => "KEY_KP_7", Self::KP_8 => "KEY_KP_8", Self::KP_9 => "KEY_KP_9", Self::MENU => "KEY_MENU", Self::HYPER => "KEY_HYPER", Self::HELP => "KEY_HELP", Self::BACK => "KEY_BACK", Self::FORWARD => "KEY_FORWARD", Self::STOP => "KEY_STOP", Self::REFRESH => "KEY_REFRESH", Self::VOLUMEDOWN => "KEY_VOLUMEDOWN", Self::VOLUMEMUTE => "KEY_VOLUMEMUTE", Self::VOLUMEUP => "KEY_VOLUMEUP", Self::MEDIAPLAY => "KEY_MEDIAPLAY", Self::MEDIASTOP => "KEY_MEDIASTOP", Self::MEDIAPREVIOUS => "KEY_MEDIAPREVIOUS", Self::MEDIANEXT => "KEY_MEDIANEXT", Self::MEDIARECORD => "KEY_MEDIARECORD", Self::HOMEPAGE => "KEY_HOMEPAGE", Self::FAVORITES => "KEY_FAVORITES", Self::SEARCH => "KEY_SEARCH", Self::STANDBY => "KEY_STANDBY", Self::OPENURL => "KEY_OPENURL", Self::LAUNCHMAIL => "KEY_LAUNCHMAIL", Self::LAUNCHMEDIA => "KEY_LAUNCHMEDIA", Self::LAUNCH0 => "KEY_LAUNCH0", Self::LAUNCH1 => "KEY_LAUNCH1", Self::LAUNCH2 => "KEY_LAUNCH2", Self::LAUNCH3 => "KEY_LAUNCH3", Self::LAUNCH4 => "KEY_LAUNCH4", Self::LAUNCH5 => "KEY_LAUNCH5", Self::LAUNCH6 => "KEY_LAUNCH6", Self::LAUNCH7 => "KEY_LAUNCH7", Self::LAUNCH8 => "KEY_LAUNCH8", Self::LAUNCH9 => "KEY_LAUNCH9", Self::LAUNCHA => "KEY_LAUNCHA", Self::LAUNCHB => "KEY_LAUNCHB", Self::LAUNCHC => "KEY_LAUNCHC", Self::LAUNCHD => "KEY_LAUNCHD", Self::LAUNCHE => "KEY_LAUNCHE", Self::LAUNCHF => "KEY_LAUNCHF", Self::GLOBE => "KEY_GLOBE", Self::KEYBOARD => "KEY_KEYBOARD", Self::JIS_EISU => "KEY_JIS_EISU", Self::JIS_KANA => "KEY_JIS_KANA", Self::UNKNOWN => "KEY_UNKNOWN", Self::SPACE => "KEY_SPACE", Self::EXCLAM => "KEY_EXCLAM", Self::QUOTEDBL => "KEY_QUOTEDBL", Self::NUMBERSIGN => "KEY_NUMBERSIGN", Self::DOLLAR => "KEY_DOLLAR", Self::PERCENT => "KEY_PERCENT", Self::AMPERSAND => "KEY_AMPERSAND", Self::APOSTROPHE => "KEY_APOSTROPHE", Self::PARENLEFT => "KEY_PARENLEFT", Self::PARENRIGHT => "KEY_PARENRIGHT", Self::ASTERISK => "KEY_ASTERISK", Self::PLUS => "KEY_PLUS", Self::COMMA => "KEY_COMMA", Self::MINUS => "KEY_MINUS", Self::PERIOD => "KEY_PERIOD", Self::SLASH => "KEY_SLASH", Self::COLON => "KEY_COLON", Self::SEMICOLON => "KEY_SEMICOLON", Self::LESS => "KEY_LESS", Self::EQUAL => "KEY_EQUAL", Self::GREATER => "KEY_GREATER", Self::QUESTION => "KEY_QUESTION", Self::AT => "KEY_AT", Self::A => "KEY_A", Self::B => "KEY_B", Self::C => "KEY_C", Self::D => "KEY_D", Self::E => "KEY_E", Self::F => "KEY_F", Self::G => "KEY_G", Self::H => "KEY_H", Self::I => "KEY_I", Self::J => "KEY_J", Self::K => "KEY_K", Self::L => "KEY_L", Self::M => "KEY_M", Self::N => "KEY_N", Self::O => "KEY_O", Self::P => "KEY_P", Self::Q => "KEY_Q", Self::R => "KEY_R", Self::S => "KEY_S", Self::T => "KEY_T", Self::U => "KEY_U", Self::V => "KEY_V", Self::W => "KEY_W", Self::X => "KEY_X", Self::Y => "KEY_Y", Self::Z => "KEY_Z", Self::BRACKETLEFT => "KEY_BRACKETLEFT", Self::BACKSLASH => "KEY_BACKSLASH", Self::BRACKETRIGHT => "KEY_BRACKETRIGHT", Self::ASCIICIRCUM => "KEY_ASCIICIRCUM", Self::UNDERSCORE => "KEY_UNDERSCORE", Self::QUOTELEFT => "KEY_QUOTELEFT", Self::BRACELEFT => "KEY_BRACELEFT", Self::BAR => "KEY_BAR", Self::BRACERIGHT => "KEY_BRACERIGHT", Self::ASCIITILDE => "KEY_ASCIITILDE", Self::YEN => "KEY_YEN", Self::SECTION => "KEY_SECTION", _ => self.as_str(),
            }
        }
    }
    impl std::ops::BitOr < crate::global::KeyModifierMask > for Key {
        type Output = Self;
        #[inline]
        fn bitor(self, rhs: crate::global::KeyModifierMask) -> Self::Output {
            Self {
                ord: self.ord | i32::try_from(rhs.ord) . expect("masking bitfield outside integer range")
            }
        }
    }
    impl std::ops::BitOr < Key > for crate::global::KeyModifierMask {
        type Output = Key;
        #[inline]
        fn bitor(self, rhs: Key) -> Self::Output {
            rhs | self
        }
    }
    impl std::ops::BitOrAssign < crate::global::KeyModifierMask > for Key {
        #[inline]
        fn bitor_assign(&mut self, rhs: crate::global::KeyModifierMask) {
            * self = * self | rhs;
            
        }
    }
    impl crate::meta::GodotConvert for Key {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for Key {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for Key {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
    pub struct KeyModifierMask {
        ord: u64
    }
    impl KeyModifierMask {
        #[doc(alias = "KEY_CODE_MASK")]
        #[doc = "Godot enumerator name: `KEY_CODE_MASK`"]
        pub const CODE_MASK: KeyModifierMask = KeyModifierMask {
            ord: 8388607u64
        };
        #[doc(alias = "KEY_MODIFIER_MASK")]
        #[doc = "Godot enumerator name: `KEY_MODIFIER_MASK`"]
        pub const MODIFIER_MASK: KeyModifierMask = KeyModifierMask {
            ord: 532676608u64
        };
        #[doc(alias = "KEY_MASK_CMD_OR_CTRL")]
        #[doc = "Godot enumerator name: `KEY_MASK_CMD_OR_CTRL`"]
        pub const CMD_OR_CTRL: KeyModifierMask = KeyModifierMask {
            ord: 16777216u64
        };
        #[doc(alias = "KEY_MASK_SHIFT")]
        #[doc = "Godot enumerator name: `KEY_MASK_SHIFT`"]
        pub const SHIFT: KeyModifierMask = KeyModifierMask {
            ord: 33554432u64
        };
        #[doc(alias = "KEY_MASK_ALT")]
        #[doc = "Godot enumerator name: `KEY_MASK_ALT`"]
        pub const ALT: KeyModifierMask = KeyModifierMask {
            ord: 67108864u64
        };
        #[doc(alias = "KEY_MASK_META")]
        #[doc = "Godot enumerator name: `KEY_MASK_META`"]
        pub const META: KeyModifierMask = KeyModifierMask {
            ord: 134217728u64
        };
        #[doc(alias = "KEY_MASK_CTRL")]
        #[doc = "Godot enumerator name: `KEY_MASK_CTRL`"]
        pub const CTRL: KeyModifierMask = KeyModifierMask {
            ord: 268435456u64
        };
        #[doc(alias = "KEY_MASK_KPAD")]
        #[doc = "Godot enumerator name: `KEY_MASK_KPAD`"]
        pub const KPAD: KeyModifierMask = KeyModifierMask {
            ord: 536870912u64
        };
        #[doc(alias = "KEY_MASK_GROUP_SWITCH")]
        #[doc = "Godot enumerator name: `KEY_MASK_GROUP_SWITCH`"]
        pub const GROUP_SWITCH: KeyModifierMask = KeyModifierMask {
            ord: 1073741824u64
        };
        
    }
    impl std::fmt::Debug for KeyModifierMask {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            #[allow(unreachable_patterns)]
            let enumerator = match * self {
                Self::CODE_MASK => "CODE_MASK", Self::MODIFIER_MASK => "MODIFIER_MASK", Self::CMD_OR_CTRL => "CMD_OR_CTRL", Self::SHIFT => "SHIFT", Self::ALT => "ALT", Self::META => "META", Self::CTRL => "CTRL", Self::KPAD => "KPAD", Self::GROUP_SWITCH => "GROUP_SWITCH", _ => {
                    f.debug_struct("KeyModifierMask") . field("ord", &self.ord) . finish() ?;
                    return Ok(());
                    
                }
            };
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineBitfield for KeyModifierMask {
        fn try_from_ord(ord: u64) -> Option < Self > {
            Some(Self {
                ord
            })
        }
        fn ord(self) -> u64 {
            self.ord
        }
    }
    impl std::ops::BitOr for KeyModifierMask {
        type Output = Self;
        #[inline]
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord
            }
        }
    }
    impl crate::meta::GodotConvert for KeyModifierMask {
        type Via = u64;
        
    }
    impl crate::meta::ToGodot for KeyModifierMask {
        type ToVia < 'v > = u64;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineBitfield > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for KeyModifierMask {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KeyLocation {
        ord: i32
    }
    impl KeyLocation {
        #[doc(alias = "KEY_LOCATION_UNSPECIFIED")]
        #[doc = "Godot enumerator name: `KEY_LOCATION_UNSPECIFIED`"]
        pub const UNSPECIFIED: KeyLocation = KeyLocation {
            ord: 0i32
        };
        #[doc(alias = "KEY_LOCATION_LEFT")]
        #[doc = "Godot enumerator name: `KEY_LOCATION_LEFT`"]
        pub const LEFT: KeyLocation = KeyLocation {
            ord: 1i32
        };
        #[doc(alias = "KEY_LOCATION_RIGHT")]
        #[doc = "Godot enumerator name: `KEY_LOCATION_RIGHT`"]
        pub const RIGHT: KeyLocation = KeyLocation {
            ord: 2i32
        };
        
    }
    impl std::fmt::Debug for KeyLocation {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("KeyLocation") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for KeyLocation {
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
                Self::UNSPECIFIED => "UNSPECIFIED", Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::UNSPECIFIED => "KEY_LOCATION_UNSPECIFIED", Self::LEFT => "KEY_LOCATION_LEFT", Self::RIGHT => "KEY_LOCATION_RIGHT", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for KeyLocation {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for KeyLocation {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for KeyLocation {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MouseButton {
        ord: i32
    }
    impl MouseButton {
        #[doc(alias = "MOUSE_BUTTON_NONE")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_NONE`"]
        pub const NONE: MouseButton = MouseButton {
            ord: 0i32
        };
        #[doc(alias = "MOUSE_BUTTON_LEFT")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_LEFT`"]
        pub const LEFT: MouseButton = MouseButton {
            ord: 1i32
        };
        #[doc(alias = "MOUSE_BUTTON_RIGHT")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_RIGHT`"]
        pub const RIGHT: MouseButton = MouseButton {
            ord: 2i32
        };
        #[doc(alias = "MOUSE_BUTTON_MIDDLE")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_MIDDLE`"]
        pub const MIDDLE: MouseButton = MouseButton {
            ord: 3i32
        };
        #[doc(alias = "MOUSE_BUTTON_WHEEL_UP")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_WHEEL_UP`"]
        pub const WHEEL_UP: MouseButton = MouseButton {
            ord: 4i32
        };
        #[doc(alias = "MOUSE_BUTTON_WHEEL_DOWN")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_WHEEL_DOWN`"]
        pub const WHEEL_DOWN: MouseButton = MouseButton {
            ord: 5i32
        };
        #[doc(alias = "MOUSE_BUTTON_WHEEL_LEFT")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_WHEEL_LEFT`"]
        pub const WHEEL_LEFT: MouseButton = MouseButton {
            ord: 6i32
        };
        #[doc(alias = "MOUSE_BUTTON_WHEEL_RIGHT")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_WHEEL_RIGHT`"]
        pub const WHEEL_RIGHT: MouseButton = MouseButton {
            ord: 7i32
        };
        #[doc(alias = "MOUSE_BUTTON_XBUTTON1")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_XBUTTON1`"]
        pub const XBUTTON1: MouseButton = MouseButton {
            ord: 8i32
        };
        #[doc(alias = "MOUSE_BUTTON_XBUTTON2")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_XBUTTON2`"]
        pub const XBUTTON2: MouseButton = MouseButton {
            ord: 9i32
        };
        
    }
    impl std::fmt::Debug for MouseButton {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("MouseButton") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for MouseButton {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
                Self::NONE => "NONE", Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::MIDDLE => "MIDDLE", Self::WHEEL_UP => "WHEEL_UP", Self::WHEEL_DOWN => "WHEEL_DOWN", Self::WHEEL_LEFT => "WHEEL_LEFT", Self::WHEEL_RIGHT => "WHEEL_RIGHT", Self::XBUTTON1 => "XBUTTON1", Self::XBUTTON2 => "XBUTTON2", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::NONE => "MOUSE_BUTTON_NONE", Self::LEFT => "MOUSE_BUTTON_LEFT", Self::RIGHT => "MOUSE_BUTTON_RIGHT", Self::MIDDLE => "MOUSE_BUTTON_MIDDLE", Self::WHEEL_UP => "MOUSE_BUTTON_WHEEL_UP", Self::WHEEL_DOWN => "MOUSE_BUTTON_WHEEL_DOWN", Self::WHEEL_LEFT => "MOUSE_BUTTON_WHEEL_LEFT", Self::WHEEL_RIGHT => "MOUSE_BUTTON_WHEEL_RIGHT", Self::XBUTTON1 => "MOUSE_BUTTON_XBUTTON1", Self::XBUTTON2 => "MOUSE_BUTTON_XBUTTON2", _ => self.as_str(),
            }
        }
    }
    impl std::ops::BitOr < crate::global::MouseButtonMask > for MouseButton {
        type Output = Self;
        #[inline]
        fn bitor(self, rhs: crate::global::MouseButtonMask) -> Self::Output {
            Self {
                ord: self.ord | i32::try_from(rhs.ord) . expect("masking bitfield outside integer range")
            }
        }
    }
    impl std::ops::BitOr < MouseButton > for crate::global::MouseButtonMask {
        type Output = MouseButton;
        #[inline]
        fn bitor(self, rhs: MouseButton) -> Self::Output {
            rhs | self
        }
    }
    impl std::ops::BitOrAssign < crate::global::MouseButtonMask > for MouseButton {
        #[inline]
        fn bitor_assign(&mut self, rhs: crate::global::MouseButtonMask) {
            * self = * self | rhs;
            
        }
    }
    impl crate::meta::GodotConvert for MouseButton {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for MouseButton {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for MouseButton {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
    pub struct MouseButtonMask {
        ord: u64
    }
    impl MouseButtonMask {
        #[doc(alias = "MOUSE_BUTTON_MASK_LEFT")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_MASK_LEFT`"]
        pub const LEFT: MouseButtonMask = MouseButtonMask {
            ord: 1u64
        };
        #[doc(alias = "MOUSE_BUTTON_MASK_RIGHT")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_MASK_RIGHT`"]
        pub const RIGHT: MouseButtonMask = MouseButtonMask {
            ord: 2u64
        };
        #[doc(alias = "MOUSE_BUTTON_MASK_MIDDLE")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_MASK_MIDDLE`"]
        pub const MIDDLE: MouseButtonMask = MouseButtonMask {
            ord: 4u64
        };
        #[doc(alias = "MOUSE_BUTTON_MASK_MB_XBUTTON1")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_MASK_MB_XBUTTON1`"]
        pub const MB_XBUTTON1: MouseButtonMask = MouseButtonMask {
            ord: 128u64
        };
        #[doc(alias = "MOUSE_BUTTON_MASK_MB_XBUTTON2")]
        #[doc = "Godot enumerator name: `MOUSE_BUTTON_MASK_MB_XBUTTON2`"]
        pub const MB_XBUTTON2: MouseButtonMask = MouseButtonMask {
            ord: 256u64
        };
        
    }
    impl std::fmt::Debug for MouseButtonMask {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            #[allow(unreachable_patterns)]
            let enumerator = match * self {
                Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::MIDDLE => "MIDDLE", Self::MB_XBUTTON1 => "MB_XBUTTON1", Self::MB_XBUTTON2 => "MB_XBUTTON2", _ => {
                    f.debug_struct("MouseButtonMask") . field("ord", &self.ord) . finish() ?;
                    return Ok(());
                    
                }
            };
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineBitfield for MouseButtonMask {
        fn try_from_ord(ord: u64) -> Option < Self > {
            Some(Self {
                ord
            })
        }
        fn ord(self) -> u64 {
            self.ord
        }
    }
    impl std::ops::BitOr for MouseButtonMask {
        type Output = Self;
        #[inline]
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord
            }
        }
    }
    impl crate::meta::GodotConvert for MouseButtonMask {
        type Via = u64;
        
    }
    impl crate::meta::ToGodot for MouseButtonMask {
        type ToVia < 'v > = u64;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineBitfield > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for MouseButtonMask {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct JoyButton {
        ord: i32
    }
    impl JoyButton {
        #[doc(alias = "JOY_BUTTON_INVALID")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_INVALID`"]
        pub const INVALID: JoyButton = JoyButton {
            ord: - 1i32
        };
        #[doc(alias = "JOY_BUTTON_A")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_A`"]
        pub const A: JoyButton = JoyButton {
            ord: 0i32
        };
        #[doc(alias = "JOY_BUTTON_B")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_B`"]
        pub const B: JoyButton = JoyButton {
            ord: 1i32
        };
        #[doc(alias = "JOY_BUTTON_X")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_X`"]
        pub const X: JoyButton = JoyButton {
            ord: 2i32
        };
        #[doc(alias = "JOY_BUTTON_Y")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_Y`"]
        pub const Y: JoyButton = JoyButton {
            ord: 3i32
        };
        #[doc(alias = "JOY_BUTTON_BACK")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_BACK`"]
        pub const BACK: JoyButton = JoyButton {
            ord: 4i32
        };
        #[doc(alias = "JOY_BUTTON_GUIDE")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_GUIDE`"]
        pub const GUIDE: JoyButton = JoyButton {
            ord: 5i32
        };
        #[doc(alias = "JOY_BUTTON_START")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_START`"]
        pub const START: JoyButton = JoyButton {
            ord: 6i32
        };
        #[doc(alias = "JOY_BUTTON_LEFT_STICK")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_LEFT_STICK`"]
        pub const LEFT_STICK: JoyButton = JoyButton {
            ord: 7i32
        };
        #[doc(alias = "JOY_BUTTON_RIGHT_STICK")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_RIGHT_STICK`"]
        pub const RIGHT_STICK: JoyButton = JoyButton {
            ord: 8i32
        };
        #[doc(alias = "JOY_BUTTON_LEFT_SHOULDER")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_LEFT_SHOULDER`"]
        pub const LEFT_SHOULDER: JoyButton = JoyButton {
            ord: 9i32
        };
        #[doc(alias = "JOY_BUTTON_RIGHT_SHOULDER")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_RIGHT_SHOULDER`"]
        pub const RIGHT_SHOULDER: JoyButton = JoyButton {
            ord: 10i32
        };
        #[doc(alias = "JOY_BUTTON_DPAD_UP")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_DPAD_UP`"]
        pub const DPAD_UP: JoyButton = JoyButton {
            ord: 11i32
        };
        #[doc(alias = "JOY_BUTTON_DPAD_DOWN")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_DPAD_DOWN`"]
        pub const DPAD_DOWN: JoyButton = JoyButton {
            ord: 12i32
        };
        #[doc(alias = "JOY_BUTTON_DPAD_LEFT")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_DPAD_LEFT`"]
        pub const DPAD_LEFT: JoyButton = JoyButton {
            ord: 13i32
        };
        #[doc(alias = "JOY_BUTTON_DPAD_RIGHT")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_DPAD_RIGHT`"]
        pub const DPAD_RIGHT: JoyButton = JoyButton {
            ord: 14i32
        };
        #[doc(alias = "JOY_BUTTON_MISC1")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_MISC1`"]
        pub const MISC1: JoyButton = JoyButton {
            ord: 15i32
        };
        #[doc(alias = "JOY_BUTTON_PADDLE1")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_PADDLE1`"]
        pub const PADDLE1: JoyButton = JoyButton {
            ord: 16i32
        };
        #[doc(alias = "JOY_BUTTON_PADDLE2")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_PADDLE2`"]
        pub const PADDLE2: JoyButton = JoyButton {
            ord: 17i32
        };
        #[doc(alias = "JOY_BUTTON_PADDLE3")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_PADDLE3`"]
        pub const PADDLE3: JoyButton = JoyButton {
            ord: 18i32
        };
        #[doc(alias = "JOY_BUTTON_PADDLE4")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_PADDLE4`"]
        pub const PADDLE4: JoyButton = JoyButton {
            ord: 19i32
        };
        #[doc(alias = "JOY_BUTTON_TOUCHPAD")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_TOUCHPAD`"]
        pub const TOUCHPAD: JoyButton = JoyButton {
            ord: 20i32
        };
        #[doc(alias = "JOY_BUTTON_SDL_MAX")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_SDL_MAX`"]
        pub const SDL_MAX: JoyButton = JoyButton {
            ord: 21i32
        };
        #[doc(alias = "JOY_BUTTON_MAX")]
        #[doc = "Godot enumerator name: `JOY_BUTTON_MAX`"]
        pub const MAX: JoyButton = JoyButton {
            ord: 128i32
        };
        
    }
    impl std::fmt::Debug for JoyButton {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("JoyButton") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for JoyButton {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 128i32 => Some(Self {
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
                Self::INVALID => "INVALID", Self::A => "A", Self::B => "B", Self::X => "X", Self::Y => "Y", Self::BACK => "BACK", Self::GUIDE => "GUIDE", Self::START => "START", Self::LEFT_STICK => "LEFT_STICK", Self::RIGHT_STICK => "RIGHT_STICK", Self::LEFT_SHOULDER => "LEFT_SHOULDER", Self::RIGHT_SHOULDER => "RIGHT_SHOULDER", Self::DPAD_UP => "DPAD_UP", Self::DPAD_DOWN => "DPAD_DOWN", Self::DPAD_LEFT => "DPAD_LEFT", Self::DPAD_RIGHT => "DPAD_RIGHT", Self::MISC1 => "MISC1", Self::PADDLE1 => "PADDLE1", Self::PADDLE2 => "PADDLE2", Self::PADDLE3 => "PADDLE3", Self::PADDLE4 => "PADDLE4", Self::TOUCHPAD => "TOUCHPAD", Self::SDL_MAX => "SDL_MAX", Self::MAX => "MAX", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::INVALID => "JOY_BUTTON_INVALID", Self::A => "JOY_BUTTON_A", Self::B => "JOY_BUTTON_B", Self::X => "JOY_BUTTON_X", Self::Y => "JOY_BUTTON_Y", Self::BACK => "JOY_BUTTON_BACK", Self::GUIDE => "JOY_BUTTON_GUIDE", Self::START => "JOY_BUTTON_START", Self::LEFT_STICK => "JOY_BUTTON_LEFT_STICK", Self::RIGHT_STICK => "JOY_BUTTON_RIGHT_STICK", Self::LEFT_SHOULDER => "JOY_BUTTON_LEFT_SHOULDER", Self::RIGHT_SHOULDER => "JOY_BUTTON_RIGHT_SHOULDER", Self::DPAD_UP => "JOY_BUTTON_DPAD_UP", Self::DPAD_DOWN => "JOY_BUTTON_DPAD_DOWN", Self::DPAD_LEFT => "JOY_BUTTON_DPAD_LEFT", Self::DPAD_RIGHT => "JOY_BUTTON_DPAD_RIGHT", Self::MISC1 => "JOY_BUTTON_MISC1", Self::PADDLE1 => "JOY_BUTTON_PADDLE1", Self::PADDLE2 => "JOY_BUTTON_PADDLE2", Self::PADDLE3 => "JOY_BUTTON_PADDLE3", Self::PADDLE4 => "JOY_BUTTON_PADDLE4", Self::TOUCHPAD => "JOY_BUTTON_TOUCHPAD", Self::SDL_MAX => "JOY_BUTTON_SDL_MAX", Self::MAX => "JOY_BUTTON_MAX", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for JoyButton {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for JoyButton {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for JoyButton {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct JoyAxis {
        ord: i32
    }
    impl JoyAxis {
        #[doc(alias = "JOY_AXIS_INVALID")]
        #[doc = "Godot enumerator name: `JOY_AXIS_INVALID`"]
        pub const INVALID: JoyAxis = JoyAxis {
            ord: - 1i32
        };
        #[doc(alias = "JOY_AXIS_LEFT_X")]
        #[doc = "Godot enumerator name: `JOY_AXIS_LEFT_X`"]
        pub const LEFT_X: JoyAxis = JoyAxis {
            ord: 0i32
        };
        #[doc(alias = "JOY_AXIS_LEFT_Y")]
        #[doc = "Godot enumerator name: `JOY_AXIS_LEFT_Y`"]
        pub const LEFT_Y: JoyAxis = JoyAxis {
            ord: 1i32
        };
        #[doc(alias = "JOY_AXIS_RIGHT_X")]
        #[doc = "Godot enumerator name: `JOY_AXIS_RIGHT_X`"]
        pub const RIGHT_X: JoyAxis = JoyAxis {
            ord: 2i32
        };
        #[doc(alias = "JOY_AXIS_RIGHT_Y")]
        #[doc = "Godot enumerator name: `JOY_AXIS_RIGHT_Y`"]
        pub const RIGHT_Y: JoyAxis = JoyAxis {
            ord: 3i32
        };
        #[doc(alias = "JOY_AXIS_TRIGGER_LEFT")]
        #[doc = "Godot enumerator name: `JOY_AXIS_TRIGGER_LEFT`"]
        pub const TRIGGER_LEFT: JoyAxis = JoyAxis {
            ord: 4i32
        };
        #[doc(alias = "JOY_AXIS_TRIGGER_RIGHT")]
        #[doc = "Godot enumerator name: `JOY_AXIS_TRIGGER_RIGHT`"]
        pub const TRIGGER_RIGHT: JoyAxis = JoyAxis {
            ord: 5i32
        };
        #[doc(alias = "JOY_AXIS_SDL_MAX")]
        #[doc = "Godot enumerator name: `JOY_AXIS_SDL_MAX`"]
        pub const SDL_MAX: JoyAxis = JoyAxis {
            ord: 6i32
        };
        #[doc(alias = "JOY_AXIS_MAX")]
        #[doc = "Godot enumerator name: `JOY_AXIS_MAX`"]
        pub const MAX: JoyAxis = JoyAxis {
            ord: 10i32
        };
        
    }
    impl std::fmt::Debug for JoyAxis {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("JoyAxis") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for JoyAxis {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 10i32 => Some(Self {
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
                Self::INVALID => "INVALID", Self::LEFT_X => "LEFT_X", Self::LEFT_Y => "LEFT_Y", Self::RIGHT_X => "RIGHT_X", Self::RIGHT_Y => "RIGHT_Y", Self::TRIGGER_LEFT => "TRIGGER_LEFT", Self::TRIGGER_RIGHT => "TRIGGER_RIGHT", Self::SDL_MAX => "SDL_MAX", Self::MAX => "MAX", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::INVALID => "JOY_AXIS_INVALID", Self::LEFT_X => "JOY_AXIS_LEFT_X", Self::LEFT_Y => "JOY_AXIS_LEFT_Y", Self::RIGHT_X => "JOY_AXIS_RIGHT_X", Self::RIGHT_Y => "JOY_AXIS_RIGHT_Y", Self::TRIGGER_LEFT => "JOY_AXIS_TRIGGER_LEFT", Self::TRIGGER_RIGHT => "JOY_AXIS_TRIGGER_RIGHT", Self::SDL_MAX => "JOY_AXIS_SDL_MAX", Self::MAX => "JOY_AXIS_MAX", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for JoyAxis {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for JoyAxis {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for JoyAxis {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    #[doc = "Godot enum name: `MIDIMessage`."]
    pub struct MidiMessage {
        ord: i32
    }
    impl MidiMessage {
        #[doc(alias = "MIDI_MESSAGE_NONE")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_NONE`"]
        pub const NONE: MidiMessage = MidiMessage {
            ord: 0i32
        };
        #[doc(alias = "MIDI_MESSAGE_NOTE_OFF")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_NOTE_OFF`"]
        pub const NOTE_OFF: MidiMessage = MidiMessage {
            ord: 8i32
        };
        #[doc(alias = "MIDI_MESSAGE_NOTE_ON")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_NOTE_ON`"]
        pub const NOTE_ON: MidiMessage = MidiMessage {
            ord: 9i32
        };
        #[doc(alias = "MIDI_MESSAGE_AFTERTOUCH")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_AFTERTOUCH`"]
        pub const AFTERTOUCH: MidiMessage = MidiMessage {
            ord: 10i32
        };
        #[doc(alias = "MIDI_MESSAGE_CONTROL_CHANGE")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_CONTROL_CHANGE`"]
        pub const CONTROL_CHANGE: MidiMessage = MidiMessage {
            ord: 11i32
        };
        #[doc(alias = "MIDI_MESSAGE_PROGRAM_CHANGE")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_PROGRAM_CHANGE`"]
        pub const PROGRAM_CHANGE: MidiMessage = MidiMessage {
            ord: 12i32
        };
        #[doc(alias = "MIDI_MESSAGE_CHANNEL_PRESSURE")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_CHANNEL_PRESSURE`"]
        pub const CHANNEL_PRESSURE: MidiMessage = MidiMessage {
            ord: 13i32
        };
        #[doc(alias = "MIDI_MESSAGE_PITCH_BEND")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_PITCH_BEND`"]
        pub const PITCH_BEND: MidiMessage = MidiMessage {
            ord: 14i32
        };
        #[doc(alias = "MIDI_MESSAGE_SYSTEM_EXCLUSIVE")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_SYSTEM_EXCLUSIVE`"]
        pub const SYSTEM_EXCLUSIVE: MidiMessage = MidiMessage {
            ord: 240i32
        };
        #[doc(alias = "MIDI_MESSAGE_QUARTER_FRAME")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_QUARTER_FRAME`"]
        pub const QUARTER_FRAME: MidiMessage = MidiMessage {
            ord: 241i32
        };
        #[doc(alias = "MIDI_MESSAGE_SONG_POSITION_POINTER")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_SONG_POSITION_POINTER`"]
        pub const SONG_POSITION_POINTER: MidiMessage = MidiMessage {
            ord: 242i32
        };
        #[doc(alias = "MIDI_MESSAGE_SONG_SELECT")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_SONG_SELECT`"]
        pub const SONG_SELECT: MidiMessage = MidiMessage {
            ord: 243i32
        };
        #[doc(alias = "MIDI_MESSAGE_TUNE_REQUEST")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_TUNE_REQUEST`"]
        pub const TUNE_REQUEST: MidiMessage = MidiMessage {
            ord: 246i32
        };
        #[doc(alias = "MIDI_MESSAGE_TIMING_CLOCK")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_TIMING_CLOCK`"]
        pub const TIMING_CLOCK: MidiMessage = MidiMessage {
            ord: 248i32
        };
        #[doc(alias = "MIDI_MESSAGE_START")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_START`"]
        pub const START: MidiMessage = MidiMessage {
            ord: 250i32
        };
        #[doc(alias = "MIDI_MESSAGE_CONTINUE")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_CONTINUE`"]
        pub const CONTINUE: MidiMessage = MidiMessage {
            ord: 251i32
        };
        #[doc(alias = "MIDI_MESSAGE_STOP")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_STOP`"]
        pub const STOP: MidiMessage = MidiMessage {
            ord: 252i32
        };
        #[doc(alias = "MIDI_MESSAGE_ACTIVE_SENSING")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_ACTIVE_SENSING`"]
        pub const ACTIVE_SENSING: MidiMessage = MidiMessage {
            ord: 254i32
        };
        #[doc(alias = "MIDI_MESSAGE_SYSTEM_RESET")]
        #[doc = "Godot enumerator name: `MIDI_MESSAGE_SYSTEM_RESET`"]
        pub const SYSTEM_RESET: MidiMessage = MidiMessage {
            ord: 255i32
        };
        
    }
    impl std::fmt::Debug for MidiMessage {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("MidiMessage") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for MidiMessage {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 240i32 | ord @ 241i32 | ord @ 242i32 | ord @ 243i32 | ord @ 246i32 | ord @ 248i32 | ord @ 250i32 | ord @ 251i32 | ord @ 252i32 | ord @ 254i32 | ord @ 255i32 => Some(Self {
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
                Self::NONE => "NONE", Self::NOTE_OFF => "NOTE_OFF", Self::NOTE_ON => "NOTE_ON", Self::AFTERTOUCH => "AFTERTOUCH", Self::CONTROL_CHANGE => "CONTROL_CHANGE", Self::PROGRAM_CHANGE => "PROGRAM_CHANGE", Self::CHANNEL_PRESSURE => "CHANNEL_PRESSURE", Self::PITCH_BEND => "PITCH_BEND", Self::SYSTEM_EXCLUSIVE => "SYSTEM_EXCLUSIVE", Self::QUARTER_FRAME => "QUARTER_FRAME", Self::SONG_POSITION_POINTER => "SONG_POSITION_POINTER", Self::SONG_SELECT => "SONG_SELECT", Self::TUNE_REQUEST => "TUNE_REQUEST", Self::TIMING_CLOCK => "TIMING_CLOCK", Self::START => "START", Self::CONTINUE => "CONTINUE", Self::STOP => "STOP", Self::ACTIVE_SENSING => "ACTIVE_SENSING", Self::SYSTEM_RESET => "SYSTEM_RESET", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::NONE => "MIDI_MESSAGE_NONE", Self::NOTE_OFF => "MIDI_MESSAGE_NOTE_OFF", Self::NOTE_ON => "MIDI_MESSAGE_NOTE_ON", Self::AFTERTOUCH => "MIDI_MESSAGE_AFTERTOUCH", Self::CONTROL_CHANGE => "MIDI_MESSAGE_CONTROL_CHANGE", Self::PROGRAM_CHANGE => "MIDI_MESSAGE_PROGRAM_CHANGE", Self::CHANNEL_PRESSURE => "MIDI_MESSAGE_CHANNEL_PRESSURE", Self::PITCH_BEND => "MIDI_MESSAGE_PITCH_BEND", Self::SYSTEM_EXCLUSIVE => "MIDI_MESSAGE_SYSTEM_EXCLUSIVE", Self::QUARTER_FRAME => "MIDI_MESSAGE_QUARTER_FRAME", Self::SONG_POSITION_POINTER => "MIDI_MESSAGE_SONG_POSITION_POINTER", Self::SONG_SELECT => "MIDI_MESSAGE_SONG_SELECT", Self::TUNE_REQUEST => "MIDI_MESSAGE_TUNE_REQUEST", Self::TIMING_CLOCK => "MIDI_MESSAGE_TIMING_CLOCK", Self::START => "MIDI_MESSAGE_START", Self::CONTINUE => "MIDI_MESSAGE_CONTINUE", Self::STOP => "MIDI_MESSAGE_STOP", Self::ACTIVE_SENSING => "MIDI_MESSAGE_ACTIVE_SENSING", Self::SYSTEM_RESET => "MIDI_MESSAGE_SYSTEM_RESET", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for MidiMessage {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for MidiMessage {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for MidiMessage {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Error {
        ord: i32
    }
    impl Error {
        pub const OK: Error = Error {
            ord: 0i32
        };
        pub const FAILED: Error = Error {
            ord: 1i32
        };
        pub const ERR_UNAVAILABLE: Error = Error {
            ord: 2i32
        };
        pub const ERR_UNCONFIGURED: Error = Error {
            ord: 3i32
        };
        pub const ERR_UNAUTHORIZED: Error = Error {
            ord: 4i32
        };
        pub const ERR_PARAMETER_RANGE_ERROR: Error = Error {
            ord: 5i32
        };
        pub const ERR_OUT_OF_MEMORY: Error = Error {
            ord: 6i32
        };
        pub const ERR_FILE_NOT_FOUND: Error = Error {
            ord: 7i32
        };
        pub const ERR_FILE_BAD_DRIVE: Error = Error {
            ord: 8i32
        };
        pub const ERR_FILE_BAD_PATH: Error = Error {
            ord: 9i32
        };
        pub const ERR_FILE_NO_PERMISSION: Error = Error {
            ord: 10i32
        };
        pub const ERR_FILE_ALREADY_IN_USE: Error = Error {
            ord: 11i32
        };
        pub const ERR_FILE_CANT_OPEN: Error = Error {
            ord: 12i32
        };
        pub const ERR_FILE_CANT_WRITE: Error = Error {
            ord: 13i32
        };
        pub const ERR_FILE_CANT_READ: Error = Error {
            ord: 14i32
        };
        pub const ERR_FILE_UNRECOGNIZED: Error = Error {
            ord: 15i32
        };
        pub const ERR_FILE_CORRUPT: Error = Error {
            ord: 16i32
        };
        pub const ERR_FILE_MISSING_DEPENDENCIES: Error = Error {
            ord: 17i32
        };
        pub const ERR_FILE_EOF: Error = Error {
            ord: 18i32
        };
        pub const ERR_CANT_OPEN: Error = Error {
            ord: 19i32
        };
        pub const ERR_CANT_CREATE: Error = Error {
            ord: 20i32
        };
        pub const ERR_QUERY_FAILED: Error = Error {
            ord: 21i32
        };
        pub const ERR_ALREADY_IN_USE: Error = Error {
            ord: 22i32
        };
        pub const ERR_LOCKED: Error = Error {
            ord: 23i32
        };
        pub const ERR_TIMEOUT: Error = Error {
            ord: 24i32
        };
        pub const ERR_CANT_CONNECT: Error = Error {
            ord: 25i32
        };
        pub const ERR_CANT_RESOLVE: Error = Error {
            ord: 26i32
        };
        pub const ERR_CONNECTION_ERROR: Error = Error {
            ord: 27i32
        };
        pub const ERR_CANT_ACQUIRE_RESOURCE: Error = Error {
            ord: 28i32
        };
        pub const ERR_CANT_FORK: Error = Error {
            ord: 29i32
        };
        pub const ERR_INVALID_DATA: Error = Error {
            ord: 30i32
        };
        pub const ERR_INVALID_PARAMETER: Error = Error {
            ord: 31i32
        };
        pub const ERR_ALREADY_EXISTS: Error = Error {
            ord: 32i32
        };
        pub const ERR_DOES_NOT_EXIST: Error = Error {
            ord: 33i32
        };
        pub const ERR_DATABASE_CANT_READ: Error = Error {
            ord: 34i32
        };
        pub const ERR_DATABASE_CANT_WRITE: Error = Error {
            ord: 35i32
        };
        pub const ERR_COMPILATION_FAILED: Error = Error {
            ord: 36i32
        };
        pub const ERR_METHOD_NOT_FOUND: Error = Error {
            ord: 37i32
        };
        pub const ERR_LINK_FAILED: Error = Error {
            ord: 38i32
        };
        pub const ERR_SCRIPT_FAILED: Error = Error {
            ord: 39i32
        };
        pub const ERR_CYCLIC_LINK: Error = Error {
            ord: 40i32
        };
        pub const ERR_INVALID_DECLARATION: Error = Error {
            ord: 41i32
        };
        pub const ERR_DUPLICATE_SYMBOL: Error = Error {
            ord: 42i32
        };
        pub const ERR_PARSE_ERROR: Error = Error {
            ord: 43i32
        };
        pub const ERR_BUSY: Error = Error {
            ord: 44i32
        };
        pub const ERR_SKIP: Error = Error {
            ord: 45i32
        };
        pub const ERR_HELP: Error = Error {
            ord: 46i32
        };
        pub const ERR_BUG: Error = Error {
            ord: 47i32
        };
        pub const ERR_PRINTER_ON_FIRE: Error = Error {
            ord: 48i32
        };
        
    }
    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("Error") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for Error {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 | ord @ 48i32 => Some(Self {
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
                Self::OK => "OK", Self::FAILED => "FAILED", Self::ERR_UNAVAILABLE => "ERR_UNAVAILABLE", Self::ERR_UNCONFIGURED => "ERR_UNCONFIGURED", Self::ERR_UNAUTHORIZED => "ERR_UNAUTHORIZED", Self::ERR_PARAMETER_RANGE_ERROR => "ERR_PARAMETER_RANGE_ERROR", Self::ERR_OUT_OF_MEMORY => "ERR_OUT_OF_MEMORY", Self::ERR_FILE_NOT_FOUND => "ERR_FILE_NOT_FOUND", Self::ERR_FILE_BAD_DRIVE => "ERR_FILE_BAD_DRIVE", Self::ERR_FILE_BAD_PATH => "ERR_FILE_BAD_PATH", Self::ERR_FILE_NO_PERMISSION => "ERR_FILE_NO_PERMISSION", Self::ERR_FILE_ALREADY_IN_USE => "ERR_FILE_ALREADY_IN_USE", Self::ERR_FILE_CANT_OPEN => "ERR_FILE_CANT_OPEN", Self::ERR_FILE_CANT_WRITE => "ERR_FILE_CANT_WRITE", Self::ERR_FILE_CANT_READ => "ERR_FILE_CANT_READ", Self::ERR_FILE_UNRECOGNIZED => "ERR_FILE_UNRECOGNIZED", Self::ERR_FILE_CORRUPT => "ERR_FILE_CORRUPT", Self::ERR_FILE_MISSING_DEPENDENCIES => "ERR_FILE_MISSING_DEPENDENCIES", Self::ERR_FILE_EOF => "ERR_FILE_EOF", Self::ERR_CANT_OPEN => "ERR_CANT_OPEN", Self::ERR_CANT_CREATE => "ERR_CANT_CREATE", Self::ERR_QUERY_FAILED => "ERR_QUERY_FAILED", Self::ERR_ALREADY_IN_USE => "ERR_ALREADY_IN_USE", Self::ERR_LOCKED => "ERR_LOCKED", Self::ERR_TIMEOUT => "ERR_TIMEOUT", Self::ERR_CANT_CONNECT => "ERR_CANT_CONNECT", Self::ERR_CANT_RESOLVE => "ERR_CANT_RESOLVE", Self::ERR_CONNECTION_ERROR => "ERR_CONNECTION_ERROR", Self::ERR_CANT_ACQUIRE_RESOURCE => "ERR_CANT_ACQUIRE_RESOURCE", Self::ERR_CANT_FORK => "ERR_CANT_FORK", Self::ERR_INVALID_DATA => "ERR_INVALID_DATA", Self::ERR_INVALID_PARAMETER => "ERR_INVALID_PARAMETER", Self::ERR_ALREADY_EXISTS => "ERR_ALREADY_EXISTS", Self::ERR_DOES_NOT_EXIST => "ERR_DOES_NOT_EXIST", Self::ERR_DATABASE_CANT_READ => "ERR_DATABASE_CANT_READ", Self::ERR_DATABASE_CANT_WRITE => "ERR_DATABASE_CANT_WRITE", Self::ERR_COMPILATION_FAILED => "ERR_COMPILATION_FAILED", Self::ERR_METHOD_NOT_FOUND => "ERR_METHOD_NOT_FOUND", Self::ERR_LINK_FAILED => "ERR_LINK_FAILED", Self::ERR_SCRIPT_FAILED => "ERR_SCRIPT_FAILED", Self::ERR_CYCLIC_LINK => "ERR_CYCLIC_LINK", Self::ERR_INVALID_DECLARATION => "ERR_INVALID_DECLARATION", Self::ERR_DUPLICATE_SYMBOL => "ERR_DUPLICATE_SYMBOL", Self::ERR_PARSE_ERROR => "ERR_PARSE_ERROR", Self::ERR_BUSY => "ERR_BUSY", Self::ERR_SKIP => "ERR_SKIP", Self::ERR_HELP => "ERR_HELP", Self::ERR_BUG => "ERR_BUG", Self::ERR_PRINTER_ON_FIRE => "ERR_PRINTER_ON_FIRE", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            self.as_str()
        }
    }
    impl crate::meta::GodotConvert for Error {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for Error {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for Error {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PropertyHint {
        ord: i32
    }
    impl PropertyHint {
        #[doc(alias = "PROPERTY_HINT_NONE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_NONE`"]
        pub const NONE: PropertyHint = PropertyHint {
            ord: 0i32
        };
        #[doc(alias = "PROPERTY_HINT_RANGE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_RANGE`"]
        pub const RANGE: PropertyHint = PropertyHint {
            ord: 1i32
        };
        #[doc(alias = "PROPERTY_HINT_ENUM")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_ENUM`"]
        pub const ENUM: PropertyHint = PropertyHint {
            ord: 2i32
        };
        #[doc(alias = "PROPERTY_HINT_ENUM_SUGGESTION")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_ENUM_SUGGESTION`"]
        pub const ENUM_SUGGESTION: PropertyHint = PropertyHint {
            ord: 3i32
        };
        #[doc(alias = "PROPERTY_HINT_EXP_EASING")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_EXP_EASING`"]
        pub const EXP_EASING: PropertyHint = PropertyHint {
            ord: 4i32
        };
        #[doc(alias = "PROPERTY_HINT_LINK")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LINK`"]
        pub const LINK: PropertyHint = PropertyHint {
            ord: 5i32
        };
        #[doc(alias = "PROPERTY_HINT_FLAGS")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_FLAGS`"]
        pub const FLAGS: PropertyHint = PropertyHint {
            ord: 6i32
        };
        #[doc(alias = "PROPERTY_HINT_LAYERS_2D_RENDER")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LAYERS_2D_RENDER`"]
        pub const LAYERS_2D_RENDER: PropertyHint = PropertyHint {
            ord: 7i32
        };
        #[doc(alias = "PROPERTY_HINT_LAYERS_2D_PHYSICS")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LAYERS_2D_PHYSICS`"]
        pub const LAYERS_2D_PHYSICS: PropertyHint = PropertyHint {
            ord: 8i32
        };
        #[doc(alias = "PROPERTY_HINT_LAYERS_2D_NAVIGATION")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LAYERS_2D_NAVIGATION`"]
        pub const LAYERS_2D_NAVIGATION: PropertyHint = PropertyHint {
            ord: 9i32
        };
        #[doc(alias = "PROPERTY_HINT_LAYERS_3D_RENDER")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LAYERS_3D_RENDER`"]
        pub const LAYERS_3D_RENDER: PropertyHint = PropertyHint {
            ord: 10i32
        };
        #[doc(alias = "PROPERTY_HINT_LAYERS_3D_PHYSICS")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LAYERS_3D_PHYSICS`"]
        pub const LAYERS_3D_PHYSICS: PropertyHint = PropertyHint {
            ord: 11i32
        };
        #[doc(alias = "PROPERTY_HINT_LAYERS_3D_NAVIGATION")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LAYERS_3D_NAVIGATION`"]
        pub const LAYERS_3D_NAVIGATION: PropertyHint = PropertyHint {
            ord: 12i32
        };
        #[doc(alias = "PROPERTY_HINT_LAYERS_AVOIDANCE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LAYERS_AVOIDANCE`"]
        pub const LAYERS_AVOIDANCE: PropertyHint = PropertyHint {
            ord: 37i32
        };
        #[doc(alias = "PROPERTY_HINT_FILE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_FILE`"]
        pub const FILE: PropertyHint = PropertyHint {
            ord: 13i32
        };
        #[doc(alias = "PROPERTY_HINT_DIR")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_DIR`"]
        pub const DIR: PropertyHint = PropertyHint {
            ord: 14i32
        };
        #[doc(alias = "PROPERTY_HINT_GLOBAL_FILE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_GLOBAL_FILE`"]
        pub const GLOBAL_FILE: PropertyHint = PropertyHint {
            ord: 15i32
        };
        #[doc(alias = "PROPERTY_HINT_GLOBAL_DIR")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_GLOBAL_DIR`"]
        pub const GLOBAL_DIR: PropertyHint = PropertyHint {
            ord: 16i32
        };
        #[doc(alias = "PROPERTY_HINT_RESOURCE_TYPE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_RESOURCE_TYPE`"]
        pub const RESOURCE_TYPE: PropertyHint = PropertyHint {
            ord: 17i32
        };
        #[doc(alias = "PROPERTY_HINT_MULTILINE_TEXT")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_MULTILINE_TEXT`"]
        pub const MULTILINE_TEXT: PropertyHint = PropertyHint {
            ord: 18i32
        };
        #[doc(alias = "PROPERTY_HINT_EXPRESSION")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_EXPRESSION`"]
        pub const EXPRESSION: PropertyHint = PropertyHint {
            ord: 19i32
        };
        #[doc(alias = "PROPERTY_HINT_PLACEHOLDER_TEXT")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_PLACEHOLDER_TEXT`"]
        pub const PLACEHOLDER_TEXT: PropertyHint = PropertyHint {
            ord: 20i32
        };
        #[doc(alias = "PROPERTY_HINT_COLOR_NO_ALPHA")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_COLOR_NO_ALPHA`"]
        pub const COLOR_NO_ALPHA: PropertyHint = PropertyHint {
            ord: 21i32
        };
        #[doc(alias = "PROPERTY_HINT_OBJECT_ID")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_OBJECT_ID`"]
        pub const OBJECT_ID: PropertyHint = PropertyHint {
            ord: 22i32
        };
        #[doc(alias = "PROPERTY_HINT_TYPE_STRING")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_TYPE_STRING`"]
        pub const TYPE_STRING: PropertyHint = PropertyHint {
            ord: 23i32
        };
        #[doc(alias = "PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE`"]
        pub const NODE_PATH_TO_EDITED_NODE: PropertyHint = PropertyHint {
            ord: 24i32
        };
        #[doc(alias = "PROPERTY_HINT_OBJECT_TOO_BIG")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_OBJECT_TOO_BIG`"]
        pub const OBJECT_TOO_BIG: PropertyHint = PropertyHint {
            ord: 25i32
        };
        #[doc(alias = "PROPERTY_HINT_NODE_PATH_VALID_TYPES")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_NODE_PATH_VALID_TYPES`"]
        pub const NODE_PATH_VALID_TYPES: PropertyHint = PropertyHint {
            ord: 26i32
        };
        #[doc(alias = "PROPERTY_HINT_SAVE_FILE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_SAVE_FILE`"]
        pub const SAVE_FILE: PropertyHint = PropertyHint {
            ord: 27i32
        };
        #[doc(alias = "PROPERTY_HINT_GLOBAL_SAVE_FILE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_GLOBAL_SAVE_FILE`"]
        pub const GLOBAL_SAVE_FILE: PropertyHint = PropertyHint {
            ord: 28i32
        };
        #[doc(alias = "PROPERTY_HINT_INT_IS_OBJECTID")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_INT_IS_OBJECTID`"]
        pub const INT_IS_OBJECTID: PropertyHint = PropertyHint {
            ord: 29i32
        };
        #[doc(alias = "PROPERTY_HINT_INT_IS_POINTER")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_INT_IS_POINTER`"]
        pub const INT_IS_POINTER: PropertyHint = PropertyHint {
            ord: 30i32
        };
        #[doc(alias = "PROPERTY_HINT_ARRAY_TYPE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_ARRAY_TYPE`"]
        pub const ARRAY_TYPE: PropertyHint = PropertyHint {
            ord: 31i32
        };
        #[doc(alias = "PROPERTY_HINT_LOCALE_ID")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LOCALE_ID`"]
        pub const LOCALE_ID: PropertyHint = PropertyHint {
            ord: 32i32
        };
        #[doc(alias = "PROPERTY_HINT_LOCALIZABLE_STRING")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_LOCALIZABLE_STRING`"]
        pub const LOCALIZABLE_STRING: PropertyHint = PropertyHint {
            ord: 33i32
        };
        #[doc(alias = "PROPERTY_HINT_NODE_TYPE")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_NODE_TYPE`"]
        pub const NODE_TYPE: PropertyHint = PropertyHint {
            ord: 34i32
        };
        #[doc(alias = "PROPERTY_HINT_HIDE_QUATERNION_EDIT")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_HIDE_QUATERNION_EDIT`"]
        pub const HIDE_QUATERNION_EDIT: PropertyHint = PropertyHint {
            ord: 35i32
        };
        #[doc(alias = "PROPERTY_HINT_PASSWORD")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_PASSWORD`"]
        pub const PASSWORD: PropertyHint = PropertyHint {
            ord: 36i32
        };
        #[doc(alias = "PROPERTY_HINT_MAX")]
        #[doc = "Godot enumerator name: `PROPERTY_HINT_MAX`"]
        pub const MAX: PropertyHint = PropertyHint {
            ord: 38i32
        };
        
    }
    impl std::fmt::Debug for PropertyHint {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("PropertyHint") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for PropertyHint {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 => Some(Self {
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
                Self::NONE => "NONE", Self::RANGE => "RANGE", Self::ENUM => "ENUM", Self::ENUM_SUGGESTION => "ENUM_SUGGESTION", Self::EXP_EASING => "EXP_EASING", Self::LINK => "LINK", Self::FLAGS => "FLAGS", Self::LAYERS_2D_RENDER => "LAYERS_2D_RENDER", Self::LAYERS_2D_PHYSICS => "LAYERS_2D_PHYSICS", Self::LAYERS_2D_NAVIGATION => "LAYERS_2D_NAVIGATION", Self::LAYERS_3D_RENDER => "LAYERS_3D_RENDER", Self::LAYERS_3D_PHYSICS => "LAYERS_3D_PHYSICS", Self::LAYERS_3D_NAVIGATION => "LAYERS_3D_NAVIGATION", Self::LAYERS_AVOIDANCE => "LAYERS_AVOIDANCE", Self::FILE => "FILE", Self::DIR => "DIR", Self::GLOBAL_FILE => "GLOBAL_FILE", Self::GLOBAL_DIR => "GLOBAL_DIR", Self::RESOURCE_TYPE => "RESOURCE_TYPE", Self::MULTILINE_TEXT => "MULTILINE_TEXT", Self::EXPRESSION => "EXPRESSION", Self::PLACEHOLDER_TEXT => "PLACEHOLDER_TEXT", Self::COLOR_NO_ALPHA => "COLOR_NO_ALPHA", Self::OBJECT_ID => "OBJECT_ID", Self::TYPE_STRING => "TYPE_STRING", Self::NODE_PATH_TO_EDITED_NODE => "NODE_PATH_TO_EDITED_NODE", Self::OBJECT_TOO_BIG => "OBJECT_TOO_BIG", Self::NODE_PATH_VALID_TYPES => "NODE_PATH_VALID_TYPES", Self::SAVE_FILE => "SAVE_FILE", Self::GLOBAL_SAVE_FILE => "GLOBAL_SAVE_FILE", Self::INT_IS_OBJECTID => "INT_IS_OBJECTID", Self::INT_IS_POINTER => "INT_IS_POINTER", Self::ARRAY_TYPE => "ARRAY_TYPE", Self::LOCALE_ID => "LOCALE_ID", Self::LOCALIZABLE_STRING => "LOCALIZABLE_STRING", Self::NODE_TYPE => "NODE_TYPE", Self::HIDE_QUATERNION_EDIT => "HIDE_QUATERNION_EDIT", Self::PASSWORD => "PASSWORD", Self::MAX => "MAX", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::NONE => "PROPERTY_HINT_NONE", Self::RANGE => "PROPERTY_HINT_RANGE", Self::ENUM => "PROPERTY_HINT_ENUM", Self::ENUM_SUGGESTION => "PROPERTY_HINT_ENUM_SUGGESTION", Self::EXP_EASING => "PROPERTY_HINT_EXP_EASING", Self::LINK => "PROPERTY_HINT_LINK", Self::FLAGS => "PROPERTY_HINT_FLAGS", Self::LAYERS_2D_RENDER => "PROPERTY_HINT_LAYERS_2D_RENDER", Self::LAYERS_2D_PHYSICS => "PROPERTY_HINT_LAYERS_2D_PHYSICS", Self::LAYERS_2D_NAVIGATION => "PROPERTY_HINT_LAYERS_2D_NAVIGATION", Self::LAYERS_3D_RENDER => "PROPERTY_HINT_LAYERS_3D_RENDER", Self::LAYERS_3D_PHYSICS => "PROPERTY_HINT_LAYERS_3D_PHYSICS", Self::LAYERS_3D_NAVIGATION => "PROPERTY_HINT_LAYERS_3D_NAVIGATION", Self::LAYERS_AVOIDANCE => "PROPERTY_HINT_LAYERS_AVOIDANCE", Self::FILE => "PROPERTY_HINT_FILE", Self::DIR => "PROPERTY_HINT_DIR", Self::GLOBAL_FILE => "PROPERTY_HINT_GLOBAL_FILE", Self::GLOBAL_DIR => "PROPERTY_HINT_GLOBAL_DIR", Self::RESOURCE_TYPE => "PROPERTY_HINT_RESOURCE_TYPE", Self::MULTILINE_TEXT => "PROPERTY_HINT_MULTILINE_TEXT", Self::EXPRESSION => "PROPERTY_HINT_EXPRESSION", Self::PLACEHOLDER_TEXT => "PROPERTY_HINT_PLACEHOLDER_TEXT", Self::COLOR_NO_ALPHA => "PROPERTY_HINT_COLOR_NO_ALPHA", Self::OBJECT_ID => "PROPERTY_HINT_OBJECT_ID", Self::TYPE_STRING => "PROPERTY_HINT_TYPE_STRING", Self::NODE_PATH_TO_EDITED_NODE => "PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE", Self::OBJECT_TOO_BIG => "PROPERTY_HINT_OBJECT_TOO_BIG", Self::NODE_PATH_VALID_TYPES => "PROPERTY_HINT_NODE_PATH_VALID_TYPES", Self::SAVE_FILE => "PROPERTY_HINT_SAVE_FILE", Self::GLOBAL_SAVE_FILE => "PROPERTY_HINT_GLOBAL_SAVE_FILE", Self::INT_IS_OBJECTID => "PROPERTY_HINT_INT_IS_OBJECTID", Self::INT_IS_POINTER => "PROPERTY_HINT_INT_IS_POINTER", Self::ARRAY_TYPE => "PROPERTY_HINT_ARRAY_TYPE", Self::LOCALE_ID => "PROPERTY_HINT_LOCALE_ID", Self::LOCALIZABLE_STRING => "PROPERTY_HINT_LOCALIZABLE_STRING", Self::NODE_TYPE => "PROPERTY_HINT_NODE_TYPE", Self::HIDE_QUATERNION_EDIT => "PROPERTY_HINT_HIDE_QUATERNION_EDIT", Self::PASSWORD => "PROPERTY_HINT_PASSWORD", Self::MAX => "PROPERTY_HINT_MAX", _ => self.as_str(),
            }
        }
    }
    impl crate::obj::IndexEnum for PropertyHint {
        const ENUMERATOR_COUNT: usize = 38usize;
        
    }
    impl crate::meta::GodotConvert for PropertyHint {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for PropertyHint {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for PropertyHint {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
    pub struct PropertyUsageFlags {
        ord: u64
    }
    impl PropertyUsageFlags {
        #[doc(alias = "PROPERTY_USAGE_NONE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_NONE`"]
        pub const NONE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 0u64
        };
        #[doc(alias = "PROPERTY_USAGE_STORAGE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_STORAGE`"]
        pub const STORAGE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 2u64
        };
        #[doc(alias = "PROPERTY_USAGE_EDITOR")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_EDITOR`"]
        pub const EDITOR: PropertyUsageFlags = PropertyUsageFlags {
            ord: 4u64
        };
        #[doc(alias = "PROPERTY_USAGE_INTERNAL")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_INTERNAL`"]
        pub const INTERNAL: PropertyUsageFlags = PropertyUsageFlags {
            ord: 8u64
        };
        #[doc(alias = "PROPERTY_USAGE_CHECKABLE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_CHECKABLE`"]
        pub const CHECKABLE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 16u64
        };
        #[doc(alias = "PROPERTY_USAGE_CHECKED")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_CHECKED`"]
        pub const CHECKED: PropertyUsageFlags = PropertyUsageFlags {
            ord: 32u64
        };
        #[doc(alias = "PROPERTY_USAGE_GROUP")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_GROUP`"]
        pub const GROUP: PropertyUsageFlags = PropertyUsageFlags {
            ord: 64u64
        };
        #[doc(alias = "PROPERTY_USAGE_CATEGORY")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_CATEGORY`"]
        pub const CATEGORY: PropertyUsageFlags = PropertyUsageFlags {
            ord: 128u64
        };
        #[doc(alias = "PROPERTY_USAGE_SUBGROUP")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_SUBGROUP`"]
        pub const SUBGROUP: PropertyUsageFlags = PropertyUsageFlags {
            ord: 256u64
        };
        #[doc(alias = "PROPERTY_USAGE_CLASS_IS_BITFIELD")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_CLASS_IS_BITFIELD`"]
        pub const CLASS_IS_BITFIELD: PropertyUsageFlags = PropertyUsageFlags {
            ord: 512u64
        };
        #[doc(alias = "PROPERTY_USAGE_NO_INSTANCE_STATE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_NO_INSTANCE_STATE`"]
        pub const NO_INSTANCE_STATE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 1024u64
        };
        #[doc(alias = "PROPERTY_USAGE_RESTART_IF_CHANGED")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_RESTART_IF_CHANGED`"]
        pub const RESTART_IF_CHANGED: PropertyUsageFlags = PropertyUsageFlags {
            ord: 2048u64
        };
        #[doc(alias = "PROPERTY_USAGE_SCRIPT_VARIABLE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_SCRIPT_VARIABLE`"]
        pub const SCRIPT_VARIABLE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 4096u64
        };
        #[doc(alias = "PROPERTY_USAGE_STORE_IF_NULL")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_STORE_IF_NULL`"]
        pub const STORE_IF_NULL: PropertyUsageFlags = PropertyUsageFlags {
            ord: 8192u64
        };
        #[doc(alias = "PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED`"]
        pub const UPDATE_ALL_IF_MODIFIED: PropertyUsageFlags = PropertyUsageFlags {
            ord: 16384u64
        };
        #[doc(alias = "PROPERTY_USAGE_SCRIPT_DEFAULT_VALUE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_SCRIPT_DEFAULT_VALUE`"]
        pub const SCRIPT_DEFAULT_VALUE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 32768u64
        };
        #[doc(alias = "PROPERTY_USAGE_CLASS_IS_ENUM")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_CLASS_IS_ENUM`"]
        pub const CLASS_IS_ENUM: PropertyUsageFlags = PropertyUsageFlags {
            ord: 65536u64
        };
        #[doc(alias = "PROPERTY_USAGE_NIL_IS_VARIANT")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_NIL_IS_VARIANT`"]
        pub const NIL_IS_VARIANT: PropertyUsageFlags = PropertyUsageFlags {
            ord: 131072u64
        };
        #[doc(alias = "PROPERTY_USAGE_ARRAY")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_ARRAY`"]
        pub const ARRAY: PropertyUsageFlags = PropertyUsageFlags {
            ord: 262144u64
        };
        #[doc(alias = "PROPERTY_USAGE_ALWAYS_DUPLICATE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_ALWAYS_DUPLICATE`"]
        pub const ALWAYS_DUPLICATE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 524288u64
        };
        #[doc(alias = "PROPERTY_USAGE_NEVER_DUPLICATE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_NEVER_DUPLICATE`"]
        pub const NEVER_DUPLICATE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 1048576u64
        };
        #[doc(alias = "PROPERTY_USAGE_HIGH_END_GFX")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_HIGH_END_GFX`"]
        pub const HIGH_END_GFX: PropertyUsageFlags = PropertyUsageFlags {
            ord: 2097152u64
        };
        #[doc(alias = "PROPERTY_USAGE_NODE_PATH_FROM_SCENE_ROOT")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_NODE_PATH_FROM_SCENE_ROOT`"]
        pub const NODE_PATH_FROM_SCENE_ROOT: PropertyUsageFlags = PropertyUsageFlags {
            ord: 4194304u64
        };
        #[doc(alias = "PROPERTY_USAGE_RESOURCE_NOT_PERSISTENT")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_RESOURCE_NOT_PERSISTENT`"]
        pub const RESOURCE_NOT_PERSISTENT: PropertyUsageFlags = PropertyUsageFlags {
            ord: 8388608u64
        };
        #[doc(alias = "PROPERTY_USAGE_KEYING_INCREMENTS")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_KEYING_INCREMENTS`"]
        pub const KEYING_INCREMENTS: PropertyUsageFlags = PropertyUsageFlags {
            ord: 16777216u64
        };
        #[doc(alias = "PROPERTY_USAGE_DEFERRED_SET_RESOURCE")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_DEFERRED_SET_RESOURCE`"]
        pub const DEFERRED_SET_RESOURCE: PropertyUsageFlags = PropertyUsageFlags {
            ord: 33554432u64
        };
        #[doc(alias = "PROPERTY_USAGE_EDITOR_INSTANTIATE_OBJECT")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_EDITOR_INSTANTIATE_OBJECT`"]
        pub const EDITOR_INSTANTIATE_OBJECT: PropertyUsageFlags = PropertyUsageFlags {
            ord: 67108864u64
        };
        #[doc(alias = "PROPERTY_USAGE_EDITOR_BASIC_SETTING")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_EDITOR_BASIC_SETTING`"]
        pub const EDITOR_BASIC_SETTING: PropertyUsageFlags = PropertyUsageFlags {
            ord: 134217728u64
        };
        #[doc(alias = "PROPERTY_USAGE_READ_ONLY")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_READ_ONLY`"]
        pub const READ_ONLY: PropertyUsageFlags = PropertyUsageFlags {
            ord: 268435456u64
        };
        #[doc(alias = "PROPERTY_USAGE_SECRET")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_SECRET`"]
        pub const SECRET: PropertyUsageFlags = PropertyUsageFlags {
            ord: 536870912u64
        };
        #[doc(alias = "PROPERTY_USAGE_DEFAULT")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_DEFAULT`"]
        pub const DEFAULT: PropertyUsageFlags = PropertyUsageFlags {
            ord: 6u64
        };
        #[doc(alias = "PROPERTY_USAGE_NO_EDITOR")]
        #[doc = "Godot enumerator name: `PROPERTY_USAGE_NO_EDITOR`"]
        pub const NO_EDITOR: PropertyUsageFlags = PropertyUsageFlags {
            ord: 2u64
        };
        
    }
    impl std::fmt::Debug for PropertyUsageFlags {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            #[allow(unreachable_patterns)]
            let enumerator = match * self {
                Self::NONE => "NONE", Self::STORAGE => "STORAGE", Self::EDITOR => "EDITOR", Self::INTERNAL => "INTERNAL", Self::CHECKABLE => "CHECKABLE", Self::CHECKED => "CHECKED", Self::GROUP => "GROUP", Self::CATEGORY => "CATEGORY", Self::SUBGROUP => "SUBGROUP", Self::CLASS_IS_BITFIELD => "CLASS_IS_BITFIELD", Self::NO_INSTANCE_STATE => "NO_INSTANCE_STATE", Self::RESTART_IF_CHANGED => "RESTART_IF_CHANGED", Self::SCRIPT_VARIABLE => "SCRIPT_VARIABLE", Self::STORE_IF_NULL => "STORE_IF_NULL", Self::UPDATE_ALL_IF_MODIFIED => "UPDATE_ALL_IF_MODIFIED", Self::SCRIPT_DEFAULT_VALUE => "SCRIPT_DEFAULT_VALUE", Self::CLASS_IS_ENUM => "CLASS_IS_ENUM", Self::NIL_IS_VARIANT => "NIL_IS_VARIANT", Self::ARRAY => "ARRAY", Self::ALWAYS_DUPLICATE => "ALWAYS_DUPLICATE", Self::NEVER_DUPLICATE => "NEVER_DUPLICATE", Self::HIGH_END_GFX => "HIGH_END_GFX", Self::NODE_PATH_FROM_SCENE_ROOT => "NODE_PATH_FROM_SCENE_ROOT", Self::RESOURCE_NOT_PERSISTENT => "RESOURCE_NOT_PERSISTENT", Self::KEYING_INCREMENTS => "KEYING_INCREMENTS", Self::DEFERRED_SET_RESOURCE => "DEFERRED_SET_RESOURCE", Self::EDITOR_INSTANTIATE_OBJECT => "EDITOR_INSTANTIATE_OBJECT", Self::EDITOR_BASIC_SETTING => "EDITOR_BASIC_SETTING", Self::READ_ONLY => "READ_ONLY", Self::SECRET => "SECRET", Self::DEFAULT => "DEFAULT", Self::NO_EDITOR => "NO_EDITOR", _ => {
                    f.debug_struct("PropertyUsageFlags") . field("ord", &self.ord) . finish() ?;
                    return Ok(());
                    
                }
            };
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineBitfield for PropertyUsageFlags {
        fn try_from_ord(ord: u64) -> Option < Self > {
            Some(Self {
                ord
            })
        }
        fn ord(self) -> u64 {
            self.ord
        }
    }
    impl std::ops::BitOr for PropertyUsageFlags {
        type Output = Self;
        #[inline]
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord
            }
        }
    }
    impl crate::meta::GodotConvert for PropertyUsageFlags {
        type Via = u64;
        
    }
    impl crate::meta::ToGodot for PropertyUsageFlags {
        type ToVia < 'v > = u64;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineBitfield > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for PropertyUsageFlags {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
    pub struct MethodFlags {
        ord: u64
    }
    impl MethodFlags {
        #[doc(alias = "METHOD_FLAG_NORMAL")]
        #[doc = "Godot enumerator name: `METHOD_FLAG_NORMAL`"]
        pub const NORMAL: MethodFlags = MethodFlags {
            ord: 1u64
        };
        #[doc(alias = "METHOD_FLAG_EDITOR")]
        #[doc = "Godot enumerator name: `METHOD_FLAG_EDITOR`"]
        pub const EDITOR: MethodFlags = MethodFlags {
            ord: 2u64
        };
        #[doc(alias = "METHOD_FLAG_CONST")]
        #[doc = "Godot enumerator name: `METHOD_FLAG_CONST`"]
        pub const CONST: MethodFlags = MethodFlags {
            ord: 4u64
        };
        #[doc(alias = "METHOD_FLAG_VIRTUAL")]
        #[doc = "Godot enumerator name: `METHOD_FLAG_VIRTUAL`"]
        pub const VIRTUAL: MethodFlags = MethodFlags {
            ord: 8u64
        };
        #[doc(alias = "METHOD_FLAG_VARARG")]
        #[doc = "Godot enumerator name: `METHOD_FLAG_VARARG`"]
        pub const VARARG: MethodFlags = MethodFlags {
            ord: 16u64
        };
        #[doc(alias = "METHOD_FLAG_STATIC")]
        #[doc = "Godot enumerator name: `METHOD_FLAG_STATIC`"]
        pub const STATIC: MethodFlags = MethodFlags {
            ord: 32u64
        };
        #[doc(alias = "METHOD_FLAG_OBJECT_CORE")]
        #[doc = "Godot enumerator name: `METHOD_FLAG_OBJECT_CORE`"]
        pub const OBJECT_CORE: MethodFlags = MethodFlags {
            ord: 64u64
        };
        #[doc(alias = "METHOD_FLAGS_DEFAULT")]
        #[doc = "Godot enumerator name: `METHOD_FLAGS_DEFAULT`"]
        pub const DEFAULT: MethodFlags = MethodFlags {
            ord: 1u64
        };
        
    }
    impl std::fmt::Debug for MethodFlags {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            #[allow(unreachable_patterns)]
            let enumerator = match * self {
                Self::NORMAL => "NORMAL", Self::EDITOR => "EDITOR", Self::CONST => "CONST", Self::VIRTUAL => "VIRTUAL", Self::VARARG => "VARARG", Self::STATIC => "STATIC", Self::OBJECT_CORE => "OBJECT_CORE", Self::DEFAULT => "DEFAULT", _ => {
                    f.debug_struct("MethodFlags") . field("ord", &self.ord) . finish() ?;
                    return Ok(());
                    
                }
            };
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineBitfield for MethodFlags {
        fn try_from_ord(ord: u64) -> Option < Self > {
            Some(Self {
                ord
            })
        }
        fn ord(self) -> u64 {
            self.ord
        }
    }
    impl std::ops::BitOr for MethodFlags {
        type Output = Self;
        #[inline]
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord
            }
        }
    }
    impl crate::meta::GodotConvert for MethodFlags {
        type Via = u64;
        
    }
    impl crate::meta::ToGodot for MethodFlags {
        type ToVia < 'v > = u64;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineBitfield > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for MethodFlags {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
}
pub mod global_reexported_enums {
    use crate::sys;
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[doc = r""]
    #[doc = r" This enum is exhaustive; you should not expect future Godot versions to add new enumerators."]
    #[allow(non_camel_case_types)]
    pub enum Side {
        #[doc(alias = "SIDE_LEFT")]
        #[doc = "Godot enumerator name: `SIDE_LEFT`"]
        LEFT = 0i32, #[doc(alias = "SIDE_TOP")]
        #[doc = "Godot enumerator name: `SIDE_TOP`"]
        TOP = 1i32, #[doc(alias = "SIDE_RIGHT")]
        #[doc = "Godot enumerator name: `SIDE_RIGHT`"]
        RIGHT = 2i32, #[doc(alias = "SIDE_BOTTOM")]
        #[doc = "Godot enumerator name: `SIDE_BOTTOM`"]
        BOTTOM = 3i32,
    }
    impl crate::obj::EngineEnum for Side {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                0i32 => Some(Self::LEFT), 1i32 => Some(Self::TOP), 2i32 => Some(Self::RIGHT), 3i32 => Some(Self::BOTTOM), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self as i32
        }
        #[inline]
        fn as_str(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::LEFT => "LEFT", Self::TOP => "TOP", Self::RIGHT => "RIGHT", Self::BOTTOM => "BOTTOM", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::LEFT => "SIDE_LEFT", Self::TOP => "SIDE_TOP", Self::RIGHT => "SIDE_RIGHT", Self::BOTTOM => "SIDE_BOTTOM", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for Side {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for Side {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for Side {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[doc = r""]
    #[doc = r" This enum is exhaustive; you should not expect future Godot versions to add new enumerators."]
    #[allow(non_camel_case_types)]
    pub enum Corner {
        #[doc(alias = "CORNER_TOP_LEFT")]
        #[doc = "Godot enumerator name: `CORNER_TOP_LEFT`"]
        TOP_LEFT = 0i32, #[doc(alias = "CORNER_TOP_RIGHT")]
        #[doc = "Godot enumerator name: `CORNER_TOP_RIGHT`"]
        TOP_RIGHT = 1i32, #[doc(alias = "CORNER_BOTTOM_RIGHT")]
        #[doc = "Godot enumerator name: `CORNER_BOTTOM_RIGHT`"]
        BOTTOM_RIGHT = 2i32, #[doc(alias = "CORNER_BOTTOM_LEFT")]
        #[doc = "Godot enumerator name: `CORNER_BOTTOM_LEFT`"]
        BOTTOM_LEFT = 3i32,
    }
    impl crate::obj::EngineEnum for Corner {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                0i32 => Some(Self::TOP_LEFT), 1i32 => Some(Self::TOP_RIGHT), 2i32 => Some(Self::BOTTOM_RIGHT), 3i32 => Some(Self::BOTTOM_LEFT), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self as i32
        }
        #[inline]
        fn as_str(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::TOP_LEFT => "TOP_LEFT", Self::TOP_RIGHT => "TOP_RIGHT", Self::BOTTOM_RIGHT => "BOTTOM_RIGHT", Self::BOTTOM_LEFT => "BOTTOM_LEFT", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::TOP_LEFT => "CORNER_TOP_LEFT", Self::TOP_RIGHT => "CORNER_TOP_RIGHT", Self::BOTTOM_RIGHT => "CORNER_BOTTOM_RIGHT", Self::BOTTOM_LEFT => "CORNER_BOTTOM_LEFT", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for Corner {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for Corner {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for Corner {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[doc = r""]
    #[doc = r" This enum is exhaustive; you should not expect future Godot versions to add new enumerators."]
    #[allow(non_camel_case_types)]
    pub enum EulerOrder {
        #[doc(alias = "EULER_ORDER_XYZ")]
        #[doc = "Godot enumerator name: `EULER_ORDER_XYZ`"]
        XYZ = 0i32, #[doc(alias = "EULER_ORDER_XZY")]
        #[doc = "Godot enumerator name: `EULER_ORDER_XZY`"]
        XZY = 1i32, #[doc(alias = "EULER_ORDER_YXZ")]
        #[doc = "Godot enumerator name: `EULER_ORDER_YXZ`"]
        YXZ = 2i32, #[doc(alias = "EULER_ORDER_YZX")]
        #[doc = "Godot enumerator name: `EULER_ORDER_YZX`"]
        YZX = 3i32, #[doc(alias = "EULER_ORDER_ZXY")]
        #[doc = "Godot enumerator name: `EULER_ORDER_ZXY`"]
        ZXY = 4i32, #[doc(alias = "EULER_ORDER_ZYX")]
        #[doc = "Godot enumerator name: `EULER_ORDER_ZYX`"]
        ZYX = 5i32,
    }
    impl crate::obj::EngineEnum for EulerOrder {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                0i32 => Some(Self::XYZ), 1i32 => Some(Self::XZY), 2i32 => Some(Self::YXZ), 3i32 => Some(Self::YZX), 4i32 => Some(Self::ZXY), 5i32 => Some(Self::ZYX), _ => None,
            }
        }
        fn ord(self) -> i32 {
            self as i32
        }
        #[inline]
        fn as_str(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::XYZ => "XYZ", Self::XZY => "XZY", Self::YXZ => "YXZ", Self::YZX => "YZX", Self::ZXY => "ZXY", Self::ZYX => "ZYX", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::XYZ => "EULER_ORDER_XYZ", Self::XZY => "EULER_ORDER_XZY", Self::YXZ => "EULER_ORDER_YXZ", Self::YZX => "EULER_ORDER_YZX", Self::ZXY => "EULER_ORDER_ZXY", Self::ZYX => "EULER_ORDER_ZYX", _ => self.as_str(),
            }
        }
    }
    impl crate::meta::GodotConvert for EulerOrder {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for EulerOrder {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for EulerOrder {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    #[doc = "Godot enum name: `Variant.Operator`."]
    pub struct VariantOperator {
        ord: i32
    }
    impl VariantOperator {
        #[doc(alias = "OP_EQUAL")]
        #[doc = "Godot enumerator name: `OP_EQUAL`"]
        pub const EQUAL: VariantOperator = VariantOperator {
            ord: 0i32
        };
        #[doc(alias = "OP_NOT_EQUAL")]
        #[doc = "Godot enumerator name: `OP_NOT_EQUAL`"]
        pub const NOT_EQUAL: VariantOperator = VariantOperator {
            ord: 1i32
        };
        #[doc(alias = "OP_LESS")]
        #[doc = "Godot enumerator name: `OP_LESS`"]
        pub const LESS: VariantOperator = VariantOperator {
            ord: 2i32
        };
        #[doc(alias = "OP_LESS_EQUAL")]
        #[doc = "Godot enumerator name: `OP_LESS_EQUAL`"]
        pub const LESS_EQUAL: VariantOperator = VariantOperator {
            ord: 3i32
        };
        #[doc(alias = "OP_GREATER")]
        #[doc = "Godot enumerator name: `OP_GREATER`"]
        pub const GREATER: VariantOperator = VariantOperator {
            ord: 4i32
        };
        #[doc(alias = "OP_GREATER_EQUAL")]
        #[doc = "Godot enumerator name: `OP_GREATER_EQUAL`"]
        pub const GREATER_EQUAL: VariantOperator = VariantOperator {
            ord: 5i32
        };
        #[doc(alias = "OP_ADD")]
        #[doc = "Godot enumerator name: `OP_ADD`"]
        pub const ADD: VariantOperator = VariantOperator {
            ord: 6i32
        };
        #[doc(alias = "OP_SUBTRACT")]
        #[doc = "Godot enumerator name: `OP_SUBTRACT`"]
        pub const SUBTRACT: VariantOperator = VariantOperator {
            ord: 7i32
        };
        #[doc(alias = "OP_MULTIPLY")]
        #[doc = "Godot enumerator name: `OP_MULTIPLY`"]
        pub const MULTIPLY: VariantOperator = VariantOperator {
            ord: 8i32
        };
        #[doc(alias = "OP_DIVIDE")]
        #[doc = "Godot enumerator name: `OP_DIVIDE`"]
        pub const DIVIDE: VariantOperator = VariantOperator {
            ord: 9i32
        };
        #[doc(alias = "OP_NEGATE")]
        #[doc = "Godot enumerator name: `OP_NEGATE`"]
        pub const NEGATE: VariantOperator = VariantOperator {
            ord: 10i32
        };
        #[doc(alias = "OP_POSITIVE")]
        #[doc = "Godot enumerator name: `OP_POSITIVE`"]
        pub const POSITIVE: VariantOperator = VariantOperator {
            ord: 11i32
        };
        #[doc(alias = "OP_MODULE")]
        #[doc = "Godot enumerator name: `OP_MODULE`"]
        pub const MODULO: VariantOperator = VariantOperator {
            ord: 12i32
        };
        #[doc(alias = "OP_POWER")]
        #[doc = "Godot enumerator name: `OP_POWER`"]
        pub const POWER: VariantOperator = VariantOperator {
            ord: 13i32
        };
        #[doc(alias = "OP_SHIFT_LEFT")]
        #[doc = "Godot enumerator name: `OP_SHIFT_LEFT`"]
        pub const SHIFT_LEFT: VariantOperator = VariantOperator {
            ord: 14i32
        };
        #[doc(alias = "OP_SHIFT_RIGHT")]
        #[doc = "Godot enumerator name: `OP_SHIFT_RIGHT`"]
        pub const SHIFT_RIGHT: VariantOperator = VariantOperator {
            ord: 15i32
        };
        #[doc(alias = "OP_BIT_AND")]
        #[doc = "Godot enumerator name: `OP_BIT_AND`"]
        pub const BIT_AND: VariantOperator = VariantOperator {
            ord: 16i32
        };
        #[doc(alias = "OP_BIT_OR")]
        #[doc = "Godot enumerator name: `OP_BIT_OR`"]
        pub const BIT_OR: VariantOperator = VariantOperator {
            ord: 17i32
        };
        #[doc(alias = "OP_BIT_XOR")]
        #[doc = "Godot enumerator name: `OP_BIT_XOR`"]
        pub const BIT_XOR: VariantOperator = VariantOperator {
            ord: 18i32
        };
        #[doc(alias = "OP_BIT_NEGATE")]
        #[doc = "Godot enumerator name: `OP_BIT_NEGATE`"]
        pub const BIT_NEGATE: VariantOperator = VariantOperator {
            ord: 19i32
        };
        #[doc(alias = "OP_AND")]
        #[doc = "Godot enumerator name: `OP_AND`"]
        pub const AND: VariantOperator = VariantOperator {
            ord: 20i32
        };
        #[doc(alias = "OP_OR")]
        #[doc = "Godot enumerator name: `OP_OR`"]
        pub const OR: VariantOperator = VariantOperator {
            ord: 21i32
        };
        #[doc(alias = "OP_XOR")]
        #[doc = "Godot enumerator name: `OP_XOR`"]
        pub const XOR: VariantOperator = VariantOperator {
            ord: 22i32
        };
        #[doc(alias = "OP_NOT")]
        #[doc = "Godot enumerator name: `OP_NOT`"]
        pub const NOT: VariantOperator = VariantOperator {
            ord: 23i32
        };
        #[doc(alias = "OP_IN")]
        #[doc = "Godot enumerator name: `OP_IN`"]
        pub const IN: VariantOperator = VariantOperator {
            ord: 24i32
        };
        #[doc(alias = "OP_MAX")]
        #[doc = "Godot enumerator name: `OP_MAX`"]
        pub const MAX: VariantOperator = VariantOperator {
            ord: 25i32
        };
        
    }
    impl std::fmt::Debug for VariantOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
            use crate::obj::EngineEnum;
            let enumerator = self.as_str();
            if enumerator.is_empty() {
                f.debug_struct("VariantOperator") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
            f.write_str(enumerator)
        }
    }
    impl crate::obj::EngineEnum for VariantOperator {
        fn try_from_ord(ord: i32) -> Option < Self > {
            match ord {
                ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 => Some(Self {
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
                Self::EQUAL => "EQUAL", Self::NOT_EQUAL => "NOT_EQUAL", Self::LESS => "LESS", Self::LESS_EQUAL => "LESS_EQUAL", Self::GREATER => "GREATER", Self::GREATER_EQUAL => "GREATER_EQUAL", Self::ADD => "ADD", Self::SUBTRACT => "SUBTRACT", Self::MULTIPLY => "MULTIPLY", Self::DIVIDE => "DIVIDE", Self::NEGATE => "NEGATE", Self::POSITIVE => "POSITIVE", Self::MODULO => "MODULO", Self::POWER => "POWER", Self::SHIFT_LEFT => "SHIFT_LEFT", Self::SHIFT_RIGHT => "SHIFT_RIGHT", Self::BIT_AND => "BIT_AND", Self::BIT_OR => "BIT_OR", Self::BIT_XOR => "BIT_XOR", Self::BIT_NEGATE => "BIT_NEGATE", Self::AND => "AND", Self::OR => "OR", Self::XOR => "XOR", Self::NOT => "NOT", Self::IN => "IN", Self::MAX => "MAX", _ => "",
            }
        }
        fn godot_name(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match * self {
                Self::EQUAL => "OP_EQUAL", Self::NOT_EQUAL => "OP_NOT_EQUAL", Self::LESS => "OP_LESS", Self::LESS_EQUAL => "OP_LESS_EQUAL", Self::GREATER => "OP_GREATER", Self::GREATER_EQUAL => "OP_GREATER_EQUAL", Self::ADD => "OP_ADD", Self::SUBTRACT => "OP_SUBTRACT", Self::MULTIPLY => "OP_MULTIPLY", Self::DIVIDE => "OP_DIVIDE", Self::NEGATE => "OP_NEGATE", Self::POSITIVE => "OP_POSITIVE", Self::MODULO => "OP_MODULE", Self::POWER => "OP_POWER", Self::SHIFT_LEFT => "OP_SHIFT_LEFT", Self::SHIFT_RIGHT => "OP_SHIFT_RIGHT", Self::BIT_AND => "OP_BIT_AND", Self::BIT_OR => "OP_BIT_OR", Self::BIT_XOR => "OP_BIT_XOR", Self::BIT_NEGATE => "OP_BIT_NEGATE", Self::AND => "OP_AND", Self::OR => "OP_OR", Self::XOR => "OP_XOR", Self::NOT => "OP_NOT", Self::IN => "OP_IN", Self::MAX => "OP_MAX", _ => self.as_str(),
            }
        }
    }
    impl crate::obj::IndexEnum for VariantOperator {
        const ENUMERATOR_COUNT: usize = 25usize;
        
    }
    impl crate::meta::GodotConvert for VariantOperator {
        type Via = i32;
        
    }
    impl crate::meta::ToGodot for VariantOperator {
        type ToVia < 'v > = i32;
        fn to_godot(&self) -> Self::ToVia < '_ > {
            < Self as crate::obj::EngineEnum > ::ord(* self)
        }
    }
    impl crate::meta::FromGodot for VariantOperator {
        fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
            < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
        }
    }
}