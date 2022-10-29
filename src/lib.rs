use glam::*;

pub trait HasX {
    fn x_component(&self) -> Self
    where
        Self: Sized;
}

pub trait HasY {
    fn y_component(&self) -> Self
    where
        Self: Sized;
}

pub trait HasZ {
    fn z_component(&self) -> Self
    where
        Self: Sized;
}

pub trait HasW {
    fn w_component(&self) -> Self
    where
        Self: Sized;
}

//
// IVec2	A 2-dimensional vector.
//

impl HasX for IVec2 {
    fn x_component(&self) -> Self {
        IVec2::X * self.x
    }
}

impl HasY for IVec2 {
    fn y_component(&self) -> Self {
        IVec2::Y * self.y
    }
}

//
// IVec3	A 3-dimensional vector.
//

impl HasX for IVec3 {
    fn x_component(&self) -> Self {
        IVec3::X * self.x
    }
}

impl HasY for IVec3 {
    fn y_component(&self) -> Self {
        IVec3::Y * self.y
    }
}

impl HasZ for IVec3 {
    fn z_component(&self) -> Self {
        IVec3::Z * self.z
    }
}

//
// IVec4	A 4-dimensional vector.
//

impl HasX for IVec4 {
    fn x_component(&self) -> Self {
        IVec4::X * self.x
    }
}

impl HasY for IVec4 {
    fn y_component(&self) -> Self {
        IVec4::Y * self.y
    }
}

impl HasZ for IVec4 {
    fn z_component(&self) -> Self {
        IVec4::Z * self.z
    }
}

impl HasW for IVec4 {
    fn w_component(&self) -> Self {
        IVec4::W * self.w
    }
}

//
// UVec2	A 2-dimensional vector.
//

impl HasX for UVec2 {
    fn x_component(&self) -> Self {
        UVec2::X * self.x
    }
}

impl HasY for UVec2 {
    fn y_component(&self) -> Self {
        UVec2::Y * self.y
    }
}

//
// UVec3	A 3-dimensional vector.
//

impl HasX for UVec3 {
    fn x_component(&self) -> Self {
        UVec3::X * self.x
    }
}

impl HasY for UVec3 {
    fn y_component(&self) -> Self {
        UVec3::Y * self.y
    }
}

impl HasZ for UVec3 {
    fn z_component(&self) -> Self {
        UVec3::Z * self.z
    }
}

//
// UVec4	A 4-dimensional vector.
//

impl HasX for UVec4 {
    fn x_component(&self) -> Self {
        UVec4::X * self.x
    }
}

impl HasY for UVec4 {
    fn y_component(&self) -> Self {
        UVec4::Y * self.y
    }
}

impl HasZ for UVec4 {
    fn z_component(&self) -> Self {
        UVec4::Z * self.z
    }
}

impl HasW for UVec4 {
    fn w_component(&self) -> Self {
        UVec4::W * self.w
    }
}

//
// Vec2	A 2-dimensional vector.
//

impl HasX for Vec2 {
    fn x_component(&self) -> Self {
        Vec2::X * self.x
    }
}

impl HasY for Vec2 {
    fn y_component(&self) -> Self {
        Vec2::Y * self.y
    }
}

//
// Vec3	A 3-dimensional vector.
//

impl HasX for Vec3 {
    fn x_component(&self) -> Self {
        Vec3::X * self.x
    }
}

impl HasY for Vec3 {
    fn y_component(&self) -> Self {
        Vec3::Y * self.y
    }
}

impl HasZ for Vec3 {
    fn z_component(&self) -> Self {
        Vec3::Z * self.z
    }
}

//
// Vec4
//

impl HasX for Vec4 {
    fn x_component(&self) -> Self {
        Vec4::X * self.x
    }
}

impl HasY for Vec4 {
    fn y_component(&self) -> Self {
        Vec4::Y * self.y
    }
}

impl HasZ for Vec4 {
    fn z_component(&self) -> Self {
        Vec4::Z * self.z
    }
}

impl HasW for Vec4 {
    fn w_component(&self) -> Self {
        Vec4::W * self.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Vec2::new(1.0, 1.0).x_component(), Vec2::new(1.0, 0.0));
    }
}
