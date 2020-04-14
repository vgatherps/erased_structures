#[cfg(feature = "erased")]
mod map {
    pub type Map<K, V> = erased_structures::erased_tree::ErasedBTreeMap<K, V>;
}

#[cfg(not(feature = "erased"))]
pub mod map {
    pub type Map<K, V> = std::collections::BTreeMap<K, V>;
}

pub use map::Map;

pub mod dummy_1 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_2 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_3 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_4 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_5 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_6 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_7 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_8 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_9 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}

pub mod dummy_10 {
    pub use super::Map;
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub struct Local {
        val: usize,
    }

    pub fn build_me(val: usize) -> Map<Local, usize> {
        let mut map = Map::new();
        map.insert(Local { val }, val);
        map
    }

    #[cfg(test)]
    mod test_me {
        pub use super::*;
        #[test]
        fn test() {
            build_me(10);
            build_me(0);
        }
    }
}
