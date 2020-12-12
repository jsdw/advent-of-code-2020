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

#[cfg(test)]
mod test {

    use super::*;

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