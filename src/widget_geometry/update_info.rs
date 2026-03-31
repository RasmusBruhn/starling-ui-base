use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

/// Status when updating the widget coordinates to check if something was
/// changed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeometryUpdateStatus {
    /// True if the relative coordinates of the widget changed
    pub relative: bool,
    /// True if the size in absolute coordinates of the widget changed
    pub absolute: bool,
    /// True if anything has changed internally
    pub internal: bool,
}

impl GeometryUpdateStatus {
    /// Constructs a new update status with all values set to the same
    ///
    /// # Parameters
    ///
    /// value: The value to set
    pub fn new(value: bool) -> Self {
        return GeometryUpdateStatus {
            relative: value,
            absolute: value,
            internal: value,
        };
    }

    /// True if any of the flags are true
    pub fn any(&self) -> bool {
        return self.relative || self.absolute || self.internal;
    }
}

impl BitOr for &GeometryUpdateStatus {
    type Output = GeometryUpdateStatus;

    fn bitor(self, rhs: Self) -> Self::Output {
        return GeometryUpdateStatus {
            relative: self.relative | rhs.relative,
            absolute: self.absolute | rhs.absolute,
            internal: self.internal | rhs.internal,
        };
    }
}

impl BitOr<GeometryUpdateStatus> for &GeometryUpdateStatus {
    type Output = GeometryUpdateStatus;

    fn bitor(self, rhs: GeometryUpdateStatus) -> Self::Output {
        return self | &rhs;
    }
}

impl BitOr<&GeometryUpdateStatus> for GeometryUpdateStatus {
    type Output = GeometryUpdateStatus;

    fn bitor(self, rhs: &GeometryUpdateStatus) -> Self::Output {
        return &self | rhs;
    }
}

impl BitOr for GeometryUpdateStatus {
    type Output = GeometryUpdateStatus;

    fn bitor(self, rhs: Self) -> Self::Output {
        return &self | &rhs;
    }
}

impl BitOrAssign<&GeometryUpdateStatus> for GeometryUpdateStatus {
    fn bitor_assign(&mut self, rhs: &GeometryUpdateStatus) {
        self.relative |= rhs.relative;
        self.absolute |= rhs.absolute;
        self.internal |= rhs.internal;
    }
}

impl BitOrAssign for GeometryUpdateStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        *self |= &rhs;
    }
}

impl BitAnd for &GeometryUpdateStatus {
    type Output = GeometryUpdateStatus;

    fn bitand(self, rhs: Self) -> Self::Output {
        return GeometryUpdateStatus {
            relative: self.relative & rhs.relative,
            absolute: self.absolute & rhs.absolute,
            internal: self.internal & rhs.internal,
        };
    }
}

impl BitAnd<GeometryUpdateStatus> for &GeometryUpdateStatus {
    type Output = GeometryUpdateStatus;

    fn bitand(self, rhs: GeometryUpdateStatus) -> Self::Output {
        return self & &rhs;
    }
}

impl BitAnd<&GeometryUpdateStatus> for GeometryUpdateStatus {
    type Output = GeometryUpdateStatus;

    fn bitand(self, rhs: &GeometryUpdateStatus) -> Self::Output {
        return &self & rhs;
    }
}

impl BitAnd for GeometryUpdateStatus {
    type Output = GeometryUpdateStatus;

    fn bitand(self, rhs: Self) -> Self::Output {
        return &self & &rhs;
    }
}

impl BitAndAssign<&GeometryUpdateStatus> for GeometryUpdateStatus {
    fn bitand_assign(&mut self, rhs: &GeometryUpdateStatus) {
        self.relative &= rhs.relative;
        self.absolute &= rhs.absolute;
        self.internal &= rhs.internal;
    }
}

impl BitAndAssign for GeometryUpdateStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        *self &= &rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let result1 = GeometryUpdateStatus::new(false);
        let result2 = GeometryUpdateStatus::new(true);

        let correct1 = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: false,
        };
        let correct2 = GeometryUpdateStatus {
            relative: true,
            absolute: true,
            internal: true,
        };

        assert_eq!(result1, correct1);
        assert_eq!(result2, correct2);
    }

    #[test]
    fn any() {
        let input1 = GeometryUpdateStatus::new(false);
        let input2 = GeometryUpdateStatus {
            relative: false,
            absolute: true,
            internal: false,
        };
        let input3 = GeometryUpdateStatus::new(true);

        let result1 = input1.any();
        let result2 = input2.any();
        let result3 = input3.any();

        let correct1 = false;
        let correct2 = true;
        let correct3 = true;

        assert_eq!(result1, correct1);
        assert_eq!(result2, correct2);
        assert_eq!(result3, correct3);
    }

    #[test]
    fn bitor() {
        let input11 = GeometryUpdateStatus::new(false);
        let input12 = GeometryUpdateStatus::new(true);

        let input21 = GeometryUpdateStatus {
            relative: true,
            absolute: false,
            internal: false,
        };
        let input22 = GeometryUpdateStatus {
            relative: false,
            absolute: true,
            internal: false,
        };
        let input23 = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: true,
        };

        let result111 = input11 | input21;
        let result112 = &input11 | input21;
        let result113 = input11 | &input21;
        let result114 = &input11 | &input21;
        let result121 = input11 | input22;
        let result122 = &input11 | input22;
        let result123 = input11 | &input22;
        let result124 = &input11 | &input22;
        let result131 = input11 | input23;
        let result132 = &input11 | input23;
        let result133 = input11 | &input23;
        let result134 = &input11 | &input23;
        let result211 = input12 | input21;
        let result212 = &input12 | input21;
        let result213 = input12 | &input21;
        let result214 = &input12 | &input21;
        let result221 = input12 | input22;
        let result222 = &input12 | input22;
        let result223 = input12 | &input22;
        let result224 = &input12 | &input22;
        let result231 = input12 | input23;
        let result232 = &input12 | input23;
        let result233 = input12 | &input23;
        let result234 = &input12 | &input23;

        let correct11 = input21;
        let correct12 = input22;
        let correct13 = input23;
        let correct21 = input12;
        let correct22 = input12;
        let correct23 = input12;

        assert_eq!(result111, correct11);
        assert_eq!(result112, correct11);
        assert_eq!(result113, correct11);
        assert_eq!(result114, correct11);
        assert_eq!(result121, correct12);
        assert_eq!(result122, correct12);
        assert_eq!(result123, correct12);
        assert_eq!(result124, correct12);
        assert_eq!(result131, correct13);
        assert_eq!(result132, correct13);
        assert_eq!(result133, correct13);
        assert_eq!(result134, correct13);
        assert_eq!(result211, correct21);
        assert_eq!(result212, correct21);
        assert_eq!(result213, correct21);
        assert_eq!(result214, correct21);
        assert_eq!(result221, correct22);
        assert_eq!(result222, correct22);
        assert_eq!(result223, correct22);
        assert_eq!(result224, correct22);
        assert_eq!(result231, correct23);
        assert_eq!(result232, correct23);
        assert_eq!(result233, correct23);
        assert_eq!(result234, correct23);
    }

    #[test]
    fn bitor_assign() {
        let input11 = GeometryUpdateStatus::new(false);
        let input12 = GeometryUpdateStatus::new(true);

        let input21 = GeometryUpdateStatus {
            relative: true,
            absolute: false,
            internal: false,
        };
        let input22 = GeometryUpdateStatus {
            relative: false,
            absolute: true,
            internal: false,
        };
        let input23 = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: true,
        };

        let mut result111 = input11;
        let mut result112 = input11;
        let mut result121 = input11;
        let mut result122 = input11;
        let mut result131 = input11;
        let mut result132 = input11;
        let mut result211 = input12;
        let mut result212 = input12;
        let mut result221 = input12;
        let mut result222 = input12;
        let mut result231 = input12;
        let mut result232 = input12;

        result111 |= input21;
        result112 |= &input21;
        result121 |= input22;
        result122 |= &input22;
        result131 |= input23;
        result132 |= &input23;
        result211 |= input21;
        result212 |= &input21;
        result221 |= input22;
        result222 |= &input22;
        result231 |= input23;
        result232 |= &input23;

        let correct11 = input21;
        let correct12 = input22;
        let correct13 = input23;
        let correct21 = input12;
        let correct22 = input12;
        let correct23 = input12;

        assert_eq!(result111, correct11);
        assert_eq!(result112, correct11);
        assert_eq!(result121, correct12);
        assert_eq!(result122, correct12);
        assert_eq!(result131, correct13);
        assert_eq!(result132, correct13);
        assert_eq!(result211, correct21);
        assert_eq!(result212, correct21);
        assert_eq!(result221, correct22);
        assert_eq!(result222, correct22);
        assert_eq!(result231, correct23);
        assert_eq!(result232, correct23);
    }

    #[test]
    fn bitand() {
        let input11 = GeometryUpdateStatus::new(false);
        let input12 = GeometryUpdateStatus::new(true);

        let input21 = GeometryUpdateStatus {
            relative: true,
            absolute: false,
            internal: false,
        };
        let input22 = GeometryUpdateStatus {
            relative: false,
            absolute: true,
            internal: false,
        };
        let input23 = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: true,
        };

        let result111 = input11 & input21;
        let result112 = &input11 & input21;
        let result113 = input11 & &input21;
        let result114 = &input11 & &input21;
        let result121 = input11 & input22;
        let result122 = &input11 & input22;
        let result123 = input11 & &input22;
        let result124 = &input11 & &input22;
        let result131 = input11 & input23;
        let result132 = &input11 & input23;
        let result133 = input11 & &input23;
        let result134 = &input11 & &input23;
        let result211 = input12 & input21;
        let result212 = &input12 & input21;
        let result213 = input12 & &input21;
        let result214 = &input12 & &input21;
        let result221 = input12 & input22;
        let result222 = &input12 & input22;
        let result223 = input12 & &input22;
        let result224 = &input12 & &input22;
        let result231 = input12 & input23;
        let result232 = &input12 & input23;
        let result233 = input12 & &input23;
        let result234 = &input12 & &input23;

        let correct11 = input11;
        let correct12 = input11;
        let correct13 = input11;
        let correct21 = input21;
        let correct22 = input22;
        let correct23 = input23;

        assert_eq!(result111, correct11);
        assert_eq!(result112, correct11);
        assert_eq!(result113, correct11);
        assert_eq!(result114, correct11);
        assert_eq!(result121, correct12);
        assert_eq!(result122, correct12);
        assert_eq!(result123, correct12);
        assert_eq!(result124, correct12);
        assert_eq!(result131, correct13);
        assert_eq!(result132, correct13);
        assert_eq!(result133, correct13);
        assert_eq!(result134, correct13);
        assert_eq!(result211, correct21);
        assert_eq!(result212, correct21);
        assert_eq!(result213, correct21);
        assert_eq!(result214, correct21);
        assert_eq!(result221, correct22);
        assert_eq!(result222, correct22);
        assert_eq!(result223, correct22);
        assert_eq!(result224, correct22);
        assert_eq!(result231, correct23);
        assert_eq!(result232, correct23);
        assert_eq!(result233, correct23);
        assert_eq!(result234, correct23);
    }

    #[test]
    fn bitand_assign() {
        let input11 = GeometryUpdateStatus::new(false);
        let input12 = GeometryUpdateStatus::new(true);

        let input21 = GeometryUpdateStatus {
            relative: true,
            absolute: false,
            internal: false,
        };
        let input22 = GeometryUpdateStatus {
            relative: false,
            absolute: true,
            internal: false,
        };
        let input23 = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: true,
        };

        let mut result111 = input11;
        let mut result112 = input11;
        let mut result121 = input11;
        let mut result122 = input11;
        let mut result131 = input11;
        let mut result132 = input11;
        let mut result211 = input12;
        let mut result212 = input12;
        let mut result221 = input12;
        let mut result222 = input12;
        let mut result231 = input12;
        let mut result232 = input12;

        result111 &= input21;
        result112 &= &input21;
        result121 &= input22;
        result122 &= &input22;
        result131 &= input23;
        result132 &= &input23;
        result211 &= input21;
        result212 &= &input21;
        result221 &= input22;
        result222 &= &input22;
        result231 &= input23;
        result232 &= &input23;

        let correct11 = input11;
        let correct12 = input11;
        let correct13 = input11;
        let correct21 = input21;
        let correct22 = input22;
        let correct23 = input23;

        assert_eq!(result111, correct11);
        assert_eq!(result112, correct11);
        assert_eq!(result121, correct12);
        assert_eq!(result122, correct12);
        assert_eq!(result131, correct13);
        assert_eq!(result132, correct13);
        assert_eq!(result211, correct21);
        assert_eq!(result212, correct21);
        assert_eq!(result221, correct22);
        assert_eq!(result222, correct22);
        assert_eq!(result231, correct23);
        assert_eq!(result232, correct23);
    }
}
