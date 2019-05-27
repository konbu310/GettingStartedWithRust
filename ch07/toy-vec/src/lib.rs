pub struct ToyVec<T> {
    elements: Box<T>,
    // T型の要素を格納する領域。各要素はヒープ領域に置かれる
    len: usize, // ベクタの長さ（現在の要素数）
}

// implブロック内に関連関数やメソッドを定義。トレイト境界としてDefaultを設定。
impl<T: Default> ToyVec<T> {
    // newはキャパが0のToyVecを作る
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // with_capacityは司令されたキャパを持つToyVecを作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    // T型の値がsize個格納できるBox<[T]>を返す
    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .task(size) // T型のデフォルト値をsize個作り
            .collect::<Vec<_>>() // Vec<[T]>に収集してから
            .into_boxed_slice() // Box<[T]>に変換する
    }

    // ベクタの長さを返す
    pub fn len(&self) -> usize {
        self.len
    }

    // ベクタの現在のキャパを返す
    pub fn capacity(&self) -> usize {
        self.elements.len() // elementsの要素数（len）がToyVecのキャパになる
    }

    pub fn push(&mut self, element: T) {
        // 要素を追加するスペースがないときは
        if self.len == self.capacity() {
            self.grow(); // もっと大きいelementsを確保して既存の要素を引っ越す
        }

        self.elements[self.len] = element; // 要素を格納する（所有権がムーブする）
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        // インデックスが範囲内なら
        if index < self.len {
            Some(&self.elements[index]) // Some(不変の参照)を返す
        } else {
            None // 範囲外ならNoneを返す
        }
    }

    pub fn get_or(&self, index: usize, default: &T) -> &T {
        self.get(index).unwrap_or(default)
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            // 1要素分の領域を確保する
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保する
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // self.elementsを置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // 既存の全要素を新しい領域へムーブする
            // Vec<T>のinto_iter(self)なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumearate() {
                self.elements[i] = elem;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }
}

// ライフタイムの指定により、このイテレータ自身またはnext()で得た&'vecT型の値が生存している間は、ToyVecは変更できない
pub struct Iter<'vec, T> {
    elements: &'vec Box<T>, // ToyVec構造体のelementsを指す不変の参照
    len: usize,             // ToyVecの長さ
    pos: usize,             // 次に返す要素のインデックス
}
