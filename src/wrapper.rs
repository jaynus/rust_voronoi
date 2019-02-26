use crate::{DCEL, Point};
use crate::dcel::NIL;

#[derive(Debug)]
pub struct VoronoiDiagram {
    /// DCEL is public allowing low-level access
    pub dcel: DCEL,
}

impl VoronoiDiagram {
    pub fn new(dcel: DCEL) -> Self {
        Self {
            dcel,
        }
    }

    pub fn segments<'a>(&'a self) -> impl Iterator<Item=(Point, Point)> + 'a {
        SegmentIter::new(&self.dcel)
    }

    pub fn points<'a>(&'a self) -> impl Iterator<Item=Point> + 'a {
        self.dcel.vertices.iter().map(|v| {
            v.coordinates
        })
    }

    pub fn cells<'a>(&'a self) -> impl Iterator<Item=VoronoiCell> + 'a {
        CellIter::new(&self.dcel)
    }
}


pub struct CellIter<'a> {
    dcel: &'a DCEL,
    index: usize,
}
impl<'a> CellIter<'a> {
    pub fn new(dcel: &'a DCEL,) -> Self {
        Self {
            dcel,
            index: 0,
        }
    }
}
impl<'a> Iterator for CellIter<'a> {
    type Item = VoronoiCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.dcel.faces.len() {
            return None;
        }

        let mut face = &self.dcel.faces[self.index];

        while !face.alive {
            self.index += 1;
            if self.index >= self.dcel.faces.len() {
                return None;
            }
            face = &self.dcel.faces[self.index];
        }
        let mut current_edge = face.outer_component;
        let mut start_edge = current_edge;

        let mut current_edge = start_edge;
        let mut this_poly = Vec::new();
        while current_edge != start_edge {
            this_poly.push(self.dcel.get_origin(current_edge));
            current_edge = self.dcel.halfedges[current_edge].next;
        }

        self.index += 1;

        Some(VoronoiCell {
            index: self.index,
            points: this_poly,
        })
    }
}

#[derive(Debug)]
pub struct VoronoiCell {
    index: usize,
    points: Vec<Point>,
}
impl VoronoiCell {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn centroid(&self) -> Point {
        crate::lloyd::polygon_centroid(&self.points)
    }

    pub fn points<'a>(&'a self) -> impl Iterator<Item=Point> + 'a {
        self.points.iter().map(|p| *p)
    }
}


pub struct SegmentIter<'a> {
    dcel: &'a DCEL,
    index: usize,
}
impl<'a> SegmentIter<'a> {
    pub fn new(dcel: &'a DCEL,) -> Self {
        Self {
            dcel,
            index: 0,
        }
    }
}
impl<'a> Iterator for SegmentIter<'a> {
    type Item = (Point, Point);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.dcel.halfedges.len() {
            return None;
        }

        let mut halfedge = &self.dcel.halfedges[self.index];
        while halfedge.origin == NIL || halfedge.next == NIL || !halfedge.alive {
            self.index += 1;
            if self.index >= self.dcel.halfedges.len() {
                return None;
            }
            halfedge = &self.dcel.halfedges[self.index];
        }

        if self.dcel.halfedges[halfedge.next].origin != NIL {
            self.index += 1;
            return Some((self.dcel.vertices[halfedge.origin].coordinates, self.dcel.get_origin(halfedge.next)));
        } else {
            self.index += 1;
            return Some((self.dcel.vertices[halfedge.origin].coordinates, self.dcel.get_origin(halfedge.next)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voronoi;

    #[test]
    fn wrapper_readme_example() {
        let vor_pts = vec![Point::new(0.0, 1.0), Point::new(2.0, 3.0), Point::new(10.0, 12.0)];
        let vor_diagram = VoronoiDiagram::new(voronoi(&vor_pts, 800.));
        // This returns 4 because we cant skip the outer face
        assert_eq!(vor_diagram.cells().count(), 4);
    }
}