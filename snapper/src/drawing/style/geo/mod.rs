use tiny_skia::{Path, PathBuilder, Pixmap};

use crate::{drawing::Drawable, Snapper};

pub mod line;
pub mod point;
pub mod polygon;

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Circle { radius: f32 },
}

impl Shape {
    /// Converts the [`Shape`] to a [`Path`] modeling the selected variant.
    pub fn to_path(&self, x: f32, y: f32) -> Result<Path, crate::Error> {
        let mut path_builder = PathBuilder::new();

        match self {
            Self::Circle { radius } => {
                path_builder.push_circle(x, y, *radius);
            }
        }

        path_builder.finish().ok_or(crate::Error::PathConstruction)
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::Circle { radius: 4.0 }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyledGeometry<T: geo::CoordNum = f64> {
    Point(point::StyledPoint<T>),
    Line(line::StyledLine<T>),
    LineString(line::StyledLineString<T>),
    Polygon(polygon::StyledPolygon<T>),
    MultiPoint(point::StyledMultiPoint<T>),
    MultiLineString(line::StyledMultiLineString<T>),
    MultiPolygon(polygon::StyledMultiPolygon<T>),
    Rect(polygon::StyledRect<T>),
    Triangle(polygon::StyledTriangle<T>),
}

impl<T> Drawable for StyledGeometry<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        match self {
            Self::Point(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Line(geometry) => geometry.draw(snapper, pixmap, center),
            Self::LineString(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Polygon(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiPoint(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiLineString(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiPolygon(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Rect(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Triangle(geometry) => geometry.draw(snapper, pixmap, center),
        }
    }
}

// FIXME: The below `Into` implementation should probably be a `From` implementation.
// We don't currently represent a styled variant of `GeometryCollection`, but we probably should.

#[allow(clippy::from_over_into)]
impl<T: geo::CoordNum> Into<geo::Geometry<T>> for StyledGeometry<T> {
    fn into(self) -> geo::Geometry<T> {
        match self {
            Self::Point(geometry) => geo::Geometry::Point(geometry.0),
            Self::Line(geometry) => geo::Geometry::Line(geometry.0),
            Self::LineString(geometry) => geo::Geometry::LineString(geometry.0),
            Self::Polygon(geometry) => geo::Geometry::Polygon(geometry.0),
            Self::MultiPoint(geometry) => geo::Geometry::MultiPoint(geometry.0),
            Self::MultiLineString(geometry) => geo::Geometry::MultiLineString(geometry.0),
            Self::MultiPolygon(geometry) => geo::Geometry::MultiPolygon(geometry.0),
            Self::Rect(geometry) => geo::Geometry::Rect(geometry.0),
            Self::Triangle(geometry) => geo::Geometry::Triangle(geometry.0),
        }
    }
}

mod macros {
    /// Macro for implementing requirements for a styled geometry type.
    macro_rules! impl_styled {
        ($base: ident, $styled: ident, $options: ident) => {
            #[derive(Clone, Debug, PartialEq)]
            pub struct $styled<T: geo::CoordNum = f64>(pub geo::$base<T>, pub $options);

            impl<T: geo::CoordNum> From<geo::$base<T>> for $styled<T> {
                fn from(value: geo::$base<T>) -> Self {
                    Self(value, $options::default())
                }
            }

            impl<T: geo::CoordNum> From<geo::$base<T>> for super::StyledGeometry<T> {
                fn from(value: geo::$base<T>) -> Self {
                    Self::$base($styled(value, $options::default()))
                }
            }

            #[allow(clippy::from_over_into)]
            impl<T: geo::CoordNum> Into<super::StyledGeometry<T>> for $styled<T> {
                fn into(self) -> super::StyledGeometry<T> {
                    super::StyledGeometry::$base(self)
                }
            }

            impl<T: geo::CoordNum> std::ops::Deref for $styled<T> {
                type Target = geo::$base<T>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T: geo::CoordNum> std::ops::DerefMut for $styled<T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        };
    }

    pub(super) use impl_styled;
}
