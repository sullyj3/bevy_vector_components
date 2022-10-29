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
        self.x * Self::X
    }
}

impl HasY for IVec2 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

//
// IVec3	A 3-dimensional vector.
//

impl HasX for IVec3 {
    fn x_component(&self) -> Self {
        self.x * Self::X
    }
}

impl HasY for IVec3 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

impl HasZ for IVec3 {
    fn z_component(&self) -> Self {
        self.z * Self::Z
    }
}

//
// IVec4	A 4-dimensional vector.
//

impl HasX for IVec4 {
    fn x_component(&self) -> Self {
        self.x * Self::X
    }
}

impl HasY for IVec4 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

impl HasZ for IVec4 {
    fn z_component(&self) -> Self {
        self.z * Self::Z
    }
}

impl HasW for IVec4 {
    fn w_component(&self) -> Self {
        self.w * Self::W
    }
}

//
// UVec2	A 2-dimensional vector.
//

impl HasX for UVec2 {
    fn x_component(&self) -> Self {
        self.x * Self::X
    }
}

impl HasY for UVec2 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

//
// UVec3	A 3-dimensional vector.
//

impl HasX for UVec3 {
    fn x_component(&self) -> Self {
        self.x * Self::X
    }
}

impl HasY for UVec3 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

impl HasZ for UVec3 {
    fn z_component(&self) -> Self {
        self.z * Self::Z
    }
}

//
// UVec4	A 4-dimensional vector.
//

impl HasX for UVec4 {
    fn x_component(&self) -> Self {
        self.x * Self::X
    }
}

impl HasY for UVec4 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

impl HasZ for UVec4 {
    fn z_component(&self) -> Self {
        self.z * Self::Z
    }
}

impl HasW for UVec4 {
    fn w_component(&self) -> Self {
        self.w * Self::W
    }
}

//
// Vec2	A 2-dimensional vector.
//

impl HasX for Vec2 {
    fn x_component(&self) -> Self {
        self.x * Self::X
    }
}

impl HasY for Vec2 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

//
// Vec3	A 3-dimensional vector.
//

impl HasX for Vec3 {
    fn x_component(&self) -> Self {
        self.x * Self::X
    }
}

impl HasY for Vec3 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

impl HasZ for Vec3 {
    fn z_component(&self) -> Self {
        self.z * Self::Z
    }
}

//
// Vec4
//

impl HasX for Vec4 {
    fn x_component(&self) -> Self {
        self.x * Self::X
    }
}

impl HasY for Vec4 {
    fn y_component(&self) -> Self {
        self.y * Self::Y
    }
}

impl HasZ for Vec4 {
    fn z_component(&self) -> Self {
        self.z * Self::Z
    }
}

impl HasW for Vec4 {
    fn w_component(&self) -> Self {
        self.w * Self::W
    }
}
