use std::collections::BTreeMap;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

// I wrote my own Box + vtable after spending hours battling:
// * std::Any and it's 'static requirement
// * * This makes the casting safer and easier, but requires 'static making it a non-starter
// * The lack of clarity the compiler has into Box<dyn Trait> objects
// * * Specifically, "this type object may not live long enough"
// * Cleanly retrieving actual values and references to said values out of the erased types
// * * The more I worked at this, the more I feel like I violated aliasing and referencing rules
// * * This also was hairy with trying to move out of the Box<dyn Trait> objects

// Overall, this is certainly not a production-ready library. I would not be surprised at all if
// my type erasure currently allows me to violate all sorts of lifetime requirements. This is just
// intended as a proof-of-concept of how type-erased structs can improve compile times

struct Vtable {
    layout: fn() -> Layout,
    ord: fn(NonNull<u8>, NonNull<u8>) -> Ordering,
    drop: fn(NonNull<u8>)
}

fn fixed_layout<T>() -> Layout {
    let size = std::mem::size_of::<T>();
    let align = std::mem::align_of::<T>();
    let size = if size == 0 { 1 } else { size };
    let align = if align == 0 { 1 } else { align };
    Layout::from_size_align(size, align).expect("Invalid type layouts")
}

fn no_ord(_: NonNull<u8>, _: NonNull<u8>) -> Ordering {
    unimplemented!()
}

fn ord<T: Ord>(a: NonNull<u8>, b: NonNull<u8>) -> Ordering {
    let a = a.cast::<T>();
    let b = b.cast::<T>();
    let (ar, br) = unsafe {
        (a.as_ref(), b.as_ref())
    };

    ar.cmp(br)
}

fn do_drop<T>(val: NonNull<u8>) {
    let val = val.cast::<T>().as_ptr();
    let to_drop: T = unsafe {
        std::ptr::read(val)
    };
    drop(to_drop);
}

trait VtableForType<K: Ord + Eq, V> {
    const KTABLE: Vtable = Vtable {
        layout: fixed_layout::<K>,
        ord: ord::<K>,
        drop: do_drop::<K>,
    };
    const VTABLE: Vtable = Vtable {
        layout: fixed_layout::<V>,
        ord: no_ord,
        drop: do_drop::<V>,
    };
}

struct UsableKeyBox {
    ptr: NonNull<u8>,
    table: &'static Vtable,
}

impl UsableKeyBox {
    fn new<V>(val: V, table: &'static Vtable) -> UsableKeyBox {
        // This layout is used since it will always allocate extra for a zero sized type
        let layout = (table.layout)();
        let ptr = NonNull::new(unsafe {alloc(layout)}).expect("Allocation returned null");
        let real_ptr = ptr.as_ptr() as *mut V;
        unsafe {
            std::ptr::write(real_ptr, val);
        }
        UsableKeyBox {
            ptr,
            table
        }
    }

    #[inline(never)]
    fn free_mem(self) {
        let layout = (self.table.layout)();
        unsafe { dealloc(self.ptr.as_ptr(), layout); }
    }

    fn claim<V>(self) -> V {
        let ptr = self.ptr.cast::<V>().as_ptr();
        let rval = unsafe {
            std::ptr::read(ptr)
        };

        self.free_mem();
        rval
    }
}

impl Drop for UsableKeyBox {
    fn drop(&mut self) {
        (self.table.drop)(self.ptr);
        let layout = (self.table.layout)();
        unsafe { dealloc(self.ptr.as_ptr(), layout); }
    }
}

impl Ord for UsableKeyBox {
    fn cmp(&self, other: &UsableKeyBox)-> Ordering {
        assert!(self.table as *const _ == other.table as *const _);
        (self.table.ord)(self.ptr, other.ptr)
    }
}

impl PartialOrd for UsableKeyBox {
    fn partial_cmp(&self, other: &UsableKeyBox)-> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for UsableKeyBox {}
impl PartialEq for UsableKeyBox {
    fn eq(&self, other: &UsableKeyBox) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

#[derive(Default)]
struct InnerTreeMap {
    map: BTreeMap<UsableKeyBox, UsableKeyBox>,
}

impl InnerTreeMap {
    #[inline(never)]
    fn new() -> InnerTreeMap {
        Self::default()
    }

    #[inline(never)]
    fn insert(&mut self, key: UsableKeyBox, val: UsableKeyBox) -> Option<UsableKeyBox> {
        self.map.insert(key, val)
    }
}

#[derive(Default)]
pub struct ErasedBTreeMap<K: Ord + Eq, V> {
    inner: InnerTreeMap,
    _marker: std::marker::PhantomData<std::collections::HashMap<K, V>>,
}

impl<K: Ord + Eq, V> VtableForType<K, V> for ErasedBTreeMap<K, V> {}

impl<K: Ord + Eq, V> ErasedBTreeMap<K, V> {
    pub fn new() -> Self {
        ErasedBTreeMap {
            inner: InnerTreeMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let key = UsableKeyBox::new(key, &<Self as VtableForType<K, V>>::KTABLE);
        let val = UsableKeyBox::new(val, &<Self as VtableForType<K, V>>::VTABLE);

        self.inner.insert(key, val).map(|ukb| ukb.claim())
    }
}
