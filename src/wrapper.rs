use crate::{DCEL, Point};
use crate::dcel::NIL;
use rayon::prelude::*;

#[derive(Debug)]
pub struct VoronoiDiagram {
    /// DCEL is public allowing low-level access
    pub dcel: DCEL,
}

impl VoronoiDiagram
{
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

    pub fn neighbors<'a>(&'a self, cell: &VoronoiCell) -> Vec<VoronoiCell> {
        use std::sync::RwLock;
        use rayon::iter::ParallelIterator;
        use rayon::iter::ParallelBridge;
        let neighbors = RwLock::new(Vec::new());

        self.cells().par_bridge().for_each(|other|{
            for self_point in cell.points() {
                for other_point in other.points() {
                    if self_point == other_point {
                        neighbors.write().unwrap().push(other.clone());
                        break;
                    }
                }
            }
        });

        let r = neighbors.read().unwrap().to_vec();
        r
    }
}

#[derive(Debug, Clone)]
pub struct VoronoiCell<'a> {
    index: usize,
    points: Vec<usize>,
    dcel: &'a DCEL,
}

impl<'a> VoronoiCell<'a> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn centroid(&self, ) -> Point {
        crate::lloyd::polygon_centroid(&self.points())
    }

    pub fn points(&self, ) -> Vec<Point>  {
        self.points.iter().map(|p| self.dcel.get_origin(*p)).collect::<Vec<_>>()
    }

    /// Create a line segment list for this poly
    pub fn segments(&self, ) -> Vec<(Point, Point)> {
        let mut ret = Vec::new();
        if self.points.len() < 2 {
            return ret;
        }

        let mut i = 0;
        for i in 0..self.points.len() {
            let cur = self.dcel.get_origin(self.points[i]);
            let mut next;
            if i == self.points.len() - 1 {
                next = self.dcel.get_origin(self.points[0]);
            } else {
                next = self.dcel.get_origin(self.points[i + 1]);
            }

            ret.push((cur, next));
        }

        ret
    }
}
impl<'a> PartialEq for VoronoiCell<'a> {
    fn eq(&self, other: &VoronoiCell<'a>) -> bool {
        self.index == other.index
    }
}



pub struct CellIter<'a> {
    dcel: &'a DCEL,
    index: usize,
    end: usize,
}
impl<'a> CellIter<'a> {
    pub fn new(dcel: &'a DCEL,) -> Self {
        Self {
            dcel,
            index: 0,
            end: 9999999,
        }
    }
    pub fn range(mut self, begin: usize, end: usize) -> Self {
        self.index = begin;
        self.end = end;
        self
    }
}
impl<'a> Iterator for CellIter<'a> {
    type Item = VoronoiCell<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.dcel.faces.len() || self.index >= self.end {
            return None;
        }

        let mut face = &self.dcel.faces[self.index];

        while !face.alive {
            self.index += 1;
            if self.index >= self.dcel.faces.len() || self.index >= self.end {
                return None;
            }
            face = &self.dcel.faces[self.index];
        }
        let mut current_edge = face.outer_component;
        let mut start_edge = current_edge;

        let mut this_poly = Vec::new();
        loop {
            this_poly.push(current_edge);
            current_edge = self.dcel.halfedges[current_edge].next;
            if current_edge == start_edge { break; }
        }

        self.index += 1;

        Some(VoronoiCell {
            index: self.index-1,
            points: this_poly,
            dcel: self.dcel,
        })
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