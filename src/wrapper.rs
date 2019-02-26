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
}

#[derive(Debug)]
pub struct VoronoiCell<'a> {
    index: usize,
    corners: &'a [Point],
}
impl<'a> VoronoiCell<'a> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn centroid(&self) -> Point {
        crate::lloyd::polygon_centroid(self.corners)
    }

    pub fn points(&'a self) -> impl Iterator<Item=Point> + 'a {
        self.corners.iter().map(|p| *p)
    }
}


pub struct SegmentIter<'a> {
    dcel: &'a DCEL,
    index: usize,
}
/*
    let mut result = vec![];
    for halfedge in &dcel.halfedges {
        if halfedge.origin != NIL && halfedge.next != NIL && halfedge.alive {
            if dcel.halfedges[halfedge.next].origin != NIL {
                result.push([dcel.vertices[halfedge.origin].coordinates,
                    dcel.get_origin(halfedge.next)])
            }
        }
    }
    result
    */
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