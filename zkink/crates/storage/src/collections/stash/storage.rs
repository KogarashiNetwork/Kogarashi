// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implementation of ink! storage traits.

use super::{
    Entry,
    Header,
    Stash as StorageStash,
};
use crate::{
    lazy::LazyIndexMap,
    traits::{
        forward_allocate_packed,
        forward_clear_packed,
        forward_pull_packed,
        forward_push_packed,
        KeyPtr,
        PackedAllocate,
        PackedLayout,
        SpreadAllocate,
        SpreadLayout,
    },
};
use ink_primitives::Key;

#[cfg(feature = "std")]
const _: () = {
    use crate::{
        collections::Vec as StorageVec,
        traits::StorageLayout,
    };
    use ink_metadata::layout::{
        CellLayout,
        FieldLayout,
        Layout,
        LayoutKey,
        StructLayout,
    };
    use scale_info::TypeInfo;

    impl StorageLayout for Header {
        fn layout(key_ptr: &mut KeyPtr) -> Layout {
            Layout::Cell(CellLayout::new::<Header>(LayoutKey::from(
                key_ptr.advance_by(1),
            )))
        }
    }

    impl<T> StorageLayout for StorageStash<T>
    where
        T: PackedLayout + TypeInfo + 'static,
    {
        fn layout(key_ptr: &mut KeyPtr) -> Layout {
            Layout::Struct(StructLayout::new([
                FieldLayout::new("header", <Header as StorageLayout>::layout(key_ptr)),
                FieldLayout::new(
                    "entries",
                    <StorageVec<Entry<T>> as StorageLayout>::layout(key_ptr),
                ),
            ]))
        }
    }
};

impl SpreadLayout for Header {
    const FOOTPRINT: u64 = 1;
    const REQUIRES_DEEP_CLEAN_UP: bool = false;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        forward_pull_packed::<Self>(ptr)
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        forward_push_packed::<Self>(self, ptr)
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        forward_clear_packed::<Self>(self, ptr)
    }
}

impl SpreadAllocate for Header {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        forward_allocate_packed::<Self>(ptr)
    }
}

impl PackedLayout for Header {
    #[inline]
    fn pull_packed(&mut self, _at: &Key) {}
    #[inline]
    fn push_packed(&self, _at: &Key) {}
    #[inline]
    fn clear_packed(&self, _at: &Key) {}
}

impl PackedAllocate for Header {
    #[inline]
    fn allocate_packed(&mut self, _at: &Key) {}
}

impl<T> SpreadLayout for Entry<T>
where
    T: PackedLayout,
{
    const FOOTPRINT: u64 = 1;
    const REQUIRES_DEEP_CLEAN_UP: bool = <T as SpreadLayout>::REQUIRES_DEEP_CLEAN_UP;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        forward_pull_packed::<Self>(ptr)
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        forward_push_packed::<Self>(self, ptr)
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        forward_clear_packed::<Self>(self, ptr)
    }
}

impl<T> PackedLayout for Entry<T>
where
    T: PackedLayout,
{
    fn pull_packed(&mut self, at: &Key) {
        if let Entry::Occupied(value) = self {
            <T as PackedLayout>::pull_packed(value, at)
        }
    }

    fn push_packed(&self, at: &Key) {
        if let Entry::Occupied(value) = self {
            <T as PackedLayout>::push_packed(value, at)
        }
    }

    fn clear_packed(&self, at: &Key) {
        if let Entry::Occupied(value) = self {
            <T as PackedLayout>::clear_packed(value, at)
        }
    }
}

impl<T> SpreadLayout for StorageStash<T>
where
    T: PackedLayout,
{
    const FOOTPRINT: u64 = 1 + <LazyIndexMap<T> as SpreadLayout>::FOOTPRINT;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            header: SpreadLayout::pull_spread(ptr),
            entries: SpreadLayout::pull_spread(ptr),
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::push_spread(&self.header, ptr);
        SpreadLayout::push_spread(&self.entries, ptr);
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        self.clear_cells();
        SpreadLayout::clear_spread(&self.header, ptr);
        SpreadLayout::clear_spread(&self.entries, ptr);
    }
}

impl<T> SpreadAllocate for StorageStash<T>
where
    T: PackedLayout,
{
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            header: SpreadAllocate::allocate_spread(ptr),
            entries: SpreadAllocate::allocate_spread(ptr),
        }
    }
}
