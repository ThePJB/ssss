use crate::scene::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use crate::kinput::*;
use glutin::event::VirtualKeyCode;
use std::collections::HashSet;

use std::hash::{Hash, Hasher};

impl Default for Voronoinoi {
    fn default() -> Self {
        Self::new()
    }
}

fn det4(a: [[f64;4];4]) -> f64 {
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

pub struct Voronoinoi {
    g: BW,
    play: bool,
    last_step: f64,
}

impl Voronoinoi {
    pub fn new() -> Voronoinoi {
        Voronoinoi {
            play: false,
            last_step: 0.0,
            g: BW::new(),
        }
    }
}

impl Demo for Voronoinoi {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {    
        if inputs.key_rising(VirtualKeyCode::Z) {
            self.g.step();
        }
        if inputs.key_rising(VirtualKeyCode::X) {
            self.play = !self.play;
        }
        if self.play && inputs.t - self.last_step > 0.25 {
            self.g.step();
            self.last_step = inputs.t;
        }

        for tri in self.g.tri_edges.iter() {
            for edge in tri.iter() {
                let start = self.g.centers[edge.a];
                let end = self.g.centers[edge.b];
                outputs.canvas.put_line(start, end, 0.005, 2.0, Vec4::new(0.6, 0.6, 0.6, 1.0));
            }
        }

        if self.g.state == BWState::BadTriangles {
            for bi in &self.g.bad_triangles {
                for edge in self.g.tri_edges[*bi].iter() {
                    let start = self.g.centers[edge.a];
                    let end = self.g.centers[edge.b];
                    outputs.canvas.put_line(start, end, 0.005, 2.1, Vec4::new(0.9, 0.0, 0.0, 1.0));
                }
            }
        }

        match self.g.state {
            BWState::BadEdges | BWState::BadTriangles | BWState::NewTriangles => {
                let s = 0.03;
                let x = self.g.new_point.unwrap().x;
                let y = self.g.new_point.unwrap().y;
                outputs.canvas.put_rect(Rect::new_centered(x, y, s, s), 2.2, Vec4::new(1.0, 1.0, 1.0, 1.0));
            },
            _ => {},
        }

        if self.g.state == BWState::BadEdges {
            for edge in self.g.bad_edges.iter() {
                let start = self.g.centers[edge.a];
                let end = self.g.centers[edge.b];
                outputs.canvas.put_line(start, end, 0.005, 2.0, Vec4::new(0.9, 0.0, 0.0, 1.0));
            }
            for edge in &self.g.poly {
                let start = self.g.centers[edge.a];
                let end = self.g.centers[edge.b];
                outputs.canvas.put_line(start, end, 0.005, 2.1, Vec4::new(0.0, 0.0, 0.9, 1.0));
            }
        }
    }
}

#[derive(PartialEq)]
enum BWState {
    AllGood,
    BadTriangles,
    BadEdges,
    NewTriangles,
}

struct BW {
    centers: Vec<Vec2>,
    tri_points: Vec<[usize;3]>,
    tri_edges: Vec<[Edge;3]>,

    bad_triangles: HashSet<usize>,
    bad_edges: HashSet<Edge>,
    poly: HashSet<Edge>,
    new_point: Option<Vec2>,
    state: BWState,
}

impl BW {
    pub fn new() -> Self {
        let mut vdg = BW {
            centers: Vec::new(),
            tri_points: Vec::new(),
            tri_edges: Vec::new(),

            bad_triangles: HashSet::new(),
            bad_edges: HashSet::new(),
            poly: HashSet::new(),
            new_point: None,
            state: BWState::AllGood,
        };
        
        vdg.centers.push(Vec2::new(0.0, 0.0));
        vdg.centers.push(Vec2::new(1.0, 0.0));
        vdg.centers.push(Vec2::new(1.0, 1.0));
        vdg.centers.push(Vec2::new(0.0, 1.0));
        vdg.tri_edges.push([Edge::new(0, 1), Edge::new(1, 3), Edge::new(3, 0)]);
        vdg.tri_points.push([0, 1, 3]);
        
        vdg.tri_edges.push([Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 3)]);
        vdg.tri_points.push([1, 2, 3]);

        vdg
    }

    pub fn step(&mut self) {
        match self.state {
            BWState::AllGood => {
                let seed = self.centers.len() as u32;
                let p = Vec2::new(krand(seed * 12313477), krand(seed * 13165747));
                self.new_point = Some(p);
                self.centers.push(p);
                self.bad_triangles = HashSet::new();
                for (t_idx, tp) in self.tri_points.iter().enumerate() {
                    if point_in_circumcircle(p, self.centers[tp[0]], self.centers[tp[1]], self.centers[tp[2]]) {
                        self.bad_triangles.insert(t_idx);
                    }
                }

                self.state = BWState::BadTriangles;
                println!("bad triangles");
            },
            BWState::BadTriangles => {
                self.bad_edges = HashSet::new();
                self.poly = HashSet::new();

                for bt in &self.bad_triangles {
                    for e in &self.tri_edges[*bt] {
                        if self.poly.contains(e) {
                            self.poly.remove(e);
                            self.bad_edges.insert(*e);
                        } else {
                            self.poly.insert(*e);
                        }
                    }
                }

                // delete bad triangles
                let mut btv: Vec<usize> = self.bad_triangles.iter().map(|x| *x).collect();
                btv.sort();
                for bti in btv.iter().rev() {
                    self.tri_edges.swap_remove(*bti);
                    self.tri_points.swap_remove(*bti);
                }

                self.state = BWState::BadEdges;
                println!("bad edges");
            },
            BWState::BadEdges => {
                for poly_edge in &self.poly {
                    self.tri_edges.push([Edge::new(poly_edge.a, poly_edge.b), Edge::new(poly_edge.b, self.centers.len() - 1), Edge::new(self.centers.len() - 1, poly_edge.a)]);
                    self.tri_points.push([poly_edge.a, poly_edge.b, self.centers.len() - 1]);
                }
                self.state = BWState::NewTriangles;
                println!("new triangles");

            },
            BWState::NewTriangles => {
                self.state = BWState::AllGood;
                println!("all good");
            },
        }
    }
}