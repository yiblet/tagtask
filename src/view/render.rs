extern crate kdtree;
extern crate ropey;

use self::kdtree::*;
use std::collections::HashSet;
use std::rc::Rc;
use view::id::Id;
use view::space::Space;
use view::widget::Square;
use view::widget::Widget;

struct VirtualScreen<T: Space> {
    layout: KdTree<f32, Rc<T>, [f32; 2]>,
}

impl<T: Space + Id> VirtualScreen<T> {
    pub fn new() -> Self {
        VirtualScreen {
            layout: KdTree::new(2),
        }
    }

    pub fn insert(&mut self, x: u16, y: u16, item: Rc<T>) -> Result<(), ErrorKind> {
        let bot = y + item.as_ref().full_height();
        let right = x + item.as_ref().full_width();

        for horiz in [x, right].into_iter() {
            for vert in [y, bot].into_iter() {
                self.layout
                    .add([*horiz as f32, *vert as f32], item.clone())?
            }
        }
        Ok(())
    }

    pub fn in_range(&mut self, square: &Square) -> Result<Vec<Rc<T>>, ErrorKind> {
        let midx = square.top + (square.bottom - square.top) / 2;
        let midy = square.left + (square.right - square.left) / 2;

        let mut corners: [(f32, f32); 4] = [(0f32, 0f32); 4];
        let mut i = 0;

        for h in [square.top, square.bottom].iter() {
            for v in [square.left, square.right].iter() {
                corners[i] = (*h as f32, *v as f32);
                i += 1;
            }
        }

        let radius = corners
            .into_iter()
            .map(|(x, y)| ((midx as f32 - *x).powi(2) + (midy as f32 - *y).powi(2)).sqrt())
            .fold(0f32, |b, dist| if b <= dist { dist } else { b });

        let mut set = HashSet::new();
        let point = [midx as f32, midy as f32];
        let res: Vec<Rc<T>> = self
            .layout
            .iter_nearest(&point, &kdtree::distance::squared_euclidean)?
            .take_while(|(dist, _)| dist <= &radius)
            .filter_map(|(_, t)| {
                if *&set.contains(&t.id()) {
                    None
                } else {
                    &set.insert(t.id());
                    Some((*t).clone())
                }
            })
            .collect();
        Ok(res)
    }
}
