use crate::scene::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use crate::kinput::*;
use glutin::event::VirtualKeyCode;
use std::collections::HashSet;

use std::hash::{Hash, Hasher};

// how to even make this again: from delauney? I must of did it for that kingdom sim
// bowyer watson is the go
// add triangles one at a time

// I think bowyer watson can add delauney points while goin
// yea the points are the same, and then voronoi vertexes are the centroid of the triangles I assume

// suspect there will be a more optimal way of storage, like half edges with gen idx for adding and deleting
// also probably like a quadtree or just grid indexing for the lookups
// circumcenter or centroid of circle is the voronoi point?

fn det4(a: [[f32;4];4]) -> f32 {
    let s1=a[0][0]*(a[1][1]*(a[2][2]*a[3][3]-a[3][2]*a[2][3])-a[1][2]*(a[2][1]*a[3][3]-a[2][3]*a[3][1])+a[1][3]*(a[2][1]*a[3][2]-a[2][2]*a[3][1]));
    let s2=a[0][1]*(a[1][0]*(a[2][2]*a[3][3]-a[3][2]*a[2][3])-a[1][2]*(a[2][0]*a[3][3]-a[2][3]*a[3][0])+a[1][3]*(a[2][0]*a[3][2]-a[2][2]*a[3][0]));
    let s3=a[0][2]*(a[1][0]*(a[2][1]*a[3][3]-a[3][1]*a[2][3])-a[1][1]*(a[2][0]*a[3][3]-a[2][3]*a[3][0])+a[1][3]*(a[2][0]*a[3][1]-a[2][1]*a[3][0]));
    let s4=a[0][3]*(a[1][0]*(a[2][1]*a[3][2]-a[3][1]*a[2][2])-a[1][1]*(a[2][0]*a[3][2]-a[2][2]*a[3][0])+a[1][2]*(a[2][0]*a[3][1]-a[2][1]*a[3][0]));
    s1-s2+s3-s4
}

fn point_in_circumcircle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    det4([
        [a.x, a.y, a.x*a.x + a.y*a.y, 1.0],
        [b.x, b.y, b.x*b.x + b.y*b.y, 1.0],
        [c.x, c.y, c.x*c.x + c.y*c.y, 1.0],
        [p.x, p.y, p.x*p.x + p.y*p.y, 1.0],
    ]) > 0.0
}

#[test]
fn test_pic() {
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(1.0, 0.0);
    let c = Vec2::new(0.0, 1.0);

    assert_eq!(point_in_circumcircle(Vec2::new(0.5, 0.5), a, b, c), true);
    assert_eq!(point_in_circumcircle(Vec2::new(0.6, 0.6), a, b, c), true);
    assert_eq!(point_in_circumcircle(Vec2::new(1.0, 0.5), a, b, c), true);
    assert_eq!(point_in_circumcircle(Vec2::new(0.5, 1.0), a, b, c), true);
    assert_eq!(point_in_circumcircle(Vec2::new(0.99, 0.99), a, b, c), true);
    assert_eq!(point_in_circumcircle(Vec2::new(-0.5, -0.5), a, b, c), false);
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    a: usize,
    b: usize,
}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && other.a == self.b)
    }
}
impl Eq for Edge {}
impl Hash for Edge {
    fn hash<H>(&self, h: &mut H) where H: Hasher {
        let p = 2654435769;
        ((p * self.a) ^ (p * self.b)).hash(h);
    }
}
impl Edge {
    fn new(a: usize, b: usize) -> Edge {Edge{a,b}}
}

// ok its actually correct

// maybe just store edge values and not refs
pub struct VDGraph {
    centers: Vec<Vec2>,
    center_neighbours: Vec<HashSet<usize>>,   // adj idx into edge list
    tri_points: Vec<[usize;3]>,
    tri_edges: Vec<[Edge;3]>,
}

// tri points could be ordered to contain edge info

impl VDGraph {
    pub fn new() -> VDGraph {
        let mut vdg = VDGraph {
            centers: Vec::new(), 
            center_neighbours: Vec::new(),  
            tri_points: Vec::new(),
            tri_edges: Vec::new(),
        };

        vdg.centers.push(Vec2::new(0.0, 0.0));
        vdg.centers.push(Vec2::new(1.0, 0.0));
        vdg.centers.push(Vec2::new(1.0, 1.0));
        vdg.centers.push(Vec2::new(0.0, 1.0));
        vdg.tri_edges.push([Edge::new(0, 1), Edge::new(1, 3), Edge::new(3, 0)]);
        vdg.tri_points.push([0, 1, 3]);
        
        vdg.tri_edges.push([Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 3)]);
        vdg.tri_points.push([1, 2, 3]);


        vdg.center_neighbours.push(HashSet::from([1, 3]));
        vdg.center_neighbours.push(HashSet::from([0, 2, 3]));
        vdg.center_neighbours.push(HashSet::from([1, 3]));
        vdg.center_neighbours.push(HashSet::from([0, 1, 3]));

        vdg
    }

    pub fn insert_point(&mut self, p: Vec2) {
        println!("inserting {:?} npts: {}, ntris: {}", p, self.centers.len(), self.tri_edges.len());
        // insert p
        let p_idx = self.centers.len();
        self.centers.push(p);
        self.center_neighbours.push(HashSet::new());
        // find all bad triangles
        let mut bad_triangles = Vec::new();
        for (t_idx, tp) in self.tri_points.iter().enumerate() {
            if point_in_circumcircle(p, self.centers[tp[0]], self.centers[tp[1]], self.centers[tp[2]]) {
                bad_triangles.push(t_idx);
            }
        }
        // go through the edges of the bad triangles
        // only these are points not connections
        
        // these are getting overwritten
        let mut bad_edges = HashSet::new();
        // by edges from this to p
        let mut poly = HashSet::new();

        println!("find tris, bt len {}", bad_triangles.len());
        // its getting stuck in some sort of infinite loop here?
        // or just marking every single triangle as bad and for some reason there are 25000 triangles
        for bti in bad_triangles.iter() {
            for btj in bad_triangles.iter() {
                if *bti == *btj {
                    continue;
                }

                let i_edges = self.tri_edges[*bti];
                let j_edges = self.tri_edges[*btj];

                for ei in i_edges {
                    for ej in j_edges {
                        if ei == ej {
                            bad_edges.insert(ei);
                        }
                    }
                }
                for ei in i_edges {
                    if !bad_edges.contains(&ei) {
                        poly.insert(ei);
                    }
                }
                for ej in j_edges {
                    if !bad_edges.contains(&ej) {
                        poly.insert(ej);
                    }
                }
            }
        }
        println!("delete bad tris");
        // delete bad triangles
        let mut idx = (bad_triangles.len() - 1) as i32;
        while idx >= 0 {
            self.tri_edges.swap_remove(idx as usize);
            self.tri_points.swap_remove(idx as usize);
            idx -= 1;
        }
        // put new edges and overwrite bad edges
        // should be more new edges than bad edges

        // ok fuck i need to get just the edges and dedup them, thats why there are too many
        // println!("poly: {:?}", poly);

        // still needs to not push duplicate edges, just index them

        // remove bad edges
        println!("removing bad edges");
        for bad_edge in bad_edges {
            println!("removing edge between {:?}, center neighs len {}", bad_edge, self.center_neighbours.len());
            self.center_neighbours[bad_edge.a].retain(|x| *x != bad_edge.b);
            self.center_neighbours[bad_edge.b].retain(|x| *x != bad_edge.a);
        }

        println!("update edges etc");
        for poly_edge in poly {
            self.center_neighbours[poly_edge.a].insert(p_idx);
            self.center_neighbours[poly_edge.b].insert(p_idx);
            self.center_neighbours[p_idx].insert(poly_edge.a);
            self.center_neighbours[p_idx].insert(poly_edge.b);
            self.tri_edges.push([Edge::new(poly_edge.a, poly_edge.b), Edge::new(poly_edge.b, p_idx), Edge::new(p_idx, poly_edge.a)]);
            self.tri_points.push([poly_edge.a, poly_edge.b, p_idx]);
        }
        println!("done");
    }
}

pub struct Voronoinoi {
    r: Rect,
    g: VDGraph,
}

impl Voronoinoi {
    pub fn new() -> Voronoinoi {
        let mut g = VDGraph::new();
        g.insert_point(Vec2::new(0.25, 0.25));
        g.insert_point(Vec2::new(0.6, 0.75));
        // g.insert_point(Vec2::new(0.6, 0.25));
        // for i in 0..100 {
        //     g.insert_point(Vec2::new(krand(i), krand(i * 1312377)));
        // }

        Voronoinoi {
            r: Rect::new(-2.0, -1.5, 3.0, 3.0),
            g,
        }
    }
}

impl DoFrame for Voronoinoi {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {    
        for i in 0..self.g.centers.len() {
            for n in &self.g.center_neighbours[i] {
                outputs.canvas.put_line(self.g.centers[i], self.g.centers[*n], 0.005, 2.0, Vec4::new(0.8, 0.0, 0.0, 1.0));
            }
        }
    
        // for tri in self.g.tri_points.iter() {
        //     outputs.canvas.put_line(self.g.centers[tri[0]], self.g.centers[tri[1]], 0.005, 2.0, Vec4::new(0.8, 0.8, 0.8, 1.0));
        //     outputs.canvas.put_line(self.g.centers[tri[2]], self.g.centers[tri[1]], 0.005, 2.0, Vec4::new(0.8, 0.8, 0.8, 1.0));
        //     outputs.canvas.put_line(self.g.centers[tri[2]], self.g.centers[tri[0]], 0.005, 2.0, Vec4::new(0.8, 0.8, 0.8, 1.0));
        // }
    
        for tri in self.g.tri_edges.iter() {
            for edge in tri.iter() {
                let start = self.g.centers[edge.a];
                let end = self.g.centers[edge.b];
                outputs.canvas.put_line(start, end, 0.005, 2.0, Vec4::new(0.0, 0.0, 0.8, 1.0));
            }
        }
    }
}

#[test]
fn voronoi_test() {
    let mut g = VDGraph::new();
    g.insert_point(Vec2::new(0.25, 0.25));
    g.insert_point(Vec2::new(0.6, 0.75));

    println!("{:?}", g.centers);
    println!("{:?}", g.center_neighbours);
    println!("{:?}", g.tri_points);
    println!("{:?}", g.tri_edges);
}