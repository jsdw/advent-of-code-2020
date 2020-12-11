#[derive(PartialEq,Eq,Clone,Debug)]
pub struct Grid<T> {
    inner: Vec<T>,
    width: usize
}

impl <T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Grid<T> {
        Grid {
            inner: vec![default; width * height],
            width
        }
    }
}

impl <T> Grid<T> {
    pub fn from_iter(width: usize, it: impl IntoIterator<Item = T>) -> Grid<T> {
        assert_ne!(width, 0, "width cannot be 0");
        Grid {
            inner: it.into_iter().collect(),
            width
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        let mut h = self.inner.len() / self.width;
        if self.inner.len() % self.width != 0 { h += 1 }
        h
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width { return None }
        self.inner.get(y * self.width + x)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width { return None }
        self.inner.get_mut(y * self.width + x)
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize,usize), &T)> {
        let w = self.width;
        self.inner
            .iter()
            .enumerate()
            .map(move |(idx, t)| {
                let x = idx % w;
                let y = idx / w;
                ((x,y),t)
            })
    }
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = ((usize,usize), &'a mut T)> + 'a {
        let w = self.width;
        self.inner
            .iter_mut()
            .enumerate()
            .map(move |(idx, t)| {
                let x = idx % w;
                let y = idx / w;
                ((x,y),t)
            })
    }

    pub fn surrounding(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        surrounding(x,y).filter_map(move |(x,y)| self.get(x,y))
    }
}

impl <T> std::ops::Index<(usize,usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x,y): (usize,usize)) -> &Self::Output {
        self.get(x,y).unwrap()
    }
}

impl <T> std::ops::IndexMut<(usize,usize)> for Grid<T> {
    fn index_mut(&mut self, (x,y): (usize,usize)) -> &mut Self::Output {
        self.get_mut(x,y).unwrap()
    }
}

// Hideous code to get surrounding coords:
fn surrounding(x: usize, y: usize) -> impl Iterator<Item = (usize,usize)> {
    std::iter::empty()
        // Top row
        .chain(y.checked_sub(1).and_then(|y| Some((x.checked_sub(1)?, y))))
        .chain(y.checked_sub(1).map(|y| (x,y)))
        .chain(y.checked_sub(1).map(|y| (x+1,y)))
        // Middle row
        .chain(x.checked_sub(1).map(|x| (x,y)))
        .chain(Some((x+1,y)))
        // Bottom row
        .chain(x.checked_sub(1).map(|x| (x,y+1)))
        .chain(Some((x,y+1)))
        .chain(Some((x+1,y+1)))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_surrounding() {
        let s = vec![
            ((0,0), vec![(1,0), (0,1), (1,1)]),
            ((1,1), vec![(0,0), (1,0), (2,0), (0,1), (2,1), (0,2), (1,2), (2,2)]),
            ((2,0), vec![(1,0), (3,0), (1,1), (2,1), (3,1)])
        ];
        for ((x,y),expected) in s {
            assert_eq!(surrounding(x,y).collect::<Vec<_>>(), expected, "expected right, got left with {:?}", (x,y));
        }
    }

    #[test]
    fn test_grid_surrounding() {
        let g = Grid::from_iter(3, vec![0,1,2,3,4,5,6,7,8]);
        let s = vec![
            ((0,0), vec![1,3,4]),
            ((1,0), vec![0,2,3,4,5]),
            ((2,0), vec![1,4,5]),
            ((0,1), vec![0,1,4,6,7]),
            ((1,1), vec![0,1,2,3,5,6,7,8]),
            ((2,1), vec![1,2,4,7,8]),
            ((0,2), vec![3,4,7]),
            ((1,2), vec![3,4,5,6,8]),
            ((2,2), vec![4,5,7]),
        ];
        for ((x,y),expected) in s {
            let actual: Vec<_> = g.surrounding(x,y).copied().collect();
            assert_eq!(actual, expected, "expected right, got left with {:?}", (x,y));
        }
    }

    #[test]
    fn test_iter() {
        let g = Grid::from_iter(3, vec![0,1,2,3,4,5,6,7,8]);
        let actual: Vec<_> = g.iter().map(|(xy,i)| (xy,*i)).collect();
        let expected: Vec<((usize,usize),_)> = vec![
            ((0,0), 0),
            ((1,0), 1),
            ((2,0), 2),
            ((0,1), 3),
            ((1,1), 4),
            ((2,1), 5),
            ((0,2), 6),
            ((1,2), 7),
            ((2,2), 8)
        ];
        assert_eq!(actual, expected);
    }

}