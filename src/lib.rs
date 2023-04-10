

///
/// 交替加权轮询选择器
///
#[derive(Debug, Clone)]
pub struct IWRRSelector<const LEN: usize> {
    pos:        usize,      //当前选择的位置
    round:      u8,         //选择的当前轮数
    max_weight: u8,         //最大的权重
    weights:    [u8; LEN],  //待选择的权重数组
}

impl<const LEN: usize> Default for IWRRSelector<LEN> {
    /// 默认构建指定长度且权重相同的交替加权轮询选择器
    fn default() -> Self {
        Self::new([1; LEN])
    }
}

impl<const LEN: usize> IWRRSelector<LEN> {
    /// 构建指定待选择的权重数组的交替加权轮询选择器
    pub fn new(weights: [u8; LEN]) -> Self {
        let mut max_weight = 0;
        for weight in weights {
            if weight >= u8::MAX {
                panic!("Create IWRRSelector failed, weight: {}, reason: invalid weight",
                       weight);
            }

            if max_weight < weight {
                //替换最大的权重
                max_weight = weight;
            }
        }

        IWRRSelector {
            round: 0,
            max_weight,
            pos: 0,
            weights,
        }
    }

    /// 获取待选择的权重数组的长度
    pub const fn len(&self) -> usize {
        LEN
    }

    /// 获取选择的当前轮数
    pub fn round(&self) -> u8 {
        self.round
    }

    /// 获取最大的权重
    pub fn max_weight(&self) -> u8 {
        self.max_weight
    }

    /// 尝试获取指定位置的权重
    pub fn try_weight(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            None
        } else {
            Some(self.weights[index])
        }
    }

    /// 改变指定位置的权重，改变成功则返回指定位置的上个权重
    pub fn change_weight(&mut self,
                         index: usize,
                         weight: u8) -> Option<u8> {
        if weight >= u8::MAX {
            return None;
        }

        if let Some(old) = self.try_weight(index) {
            self.weights[index] = weight;
            Some(old)
        } else {
            None
        }
    }

    /// 获取当前选择的位置
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// 根据权重选择，并返回被选择的位置
    pub fn select(&mut self) -> usize {
        loop {
            for pos in self.pos..self.len() {
                let weight = self.weights[pos];
                if weight == 0 || weight < self.round {
                    //被忽略，则继续下一个位置的选择
                    self.pos += 1;
                    continue;
                }

                //返回被选择的位置
                self.pos += 1;
                return pos;
            }

            if self.round > self.max_weight {
                //完成当前周期的选择，则重置选择器
                self.reset();
            } else {
                //完成当前轮的选择，则重置位置，并继续下一轮的选择
                self.pos = 0;
                self.round += 1;
            }
        }
    }

    /// 重置选择器
    pub fn reset(&mut self) {
        self.round = 0;
        self.pos = 0;
    }
}

///
/// 交替加权轮询选择器
///
#[derive(Debug, Clone)]
pub struct IWRRSelectorByWider<const LEN: usize> {
    pos:        usize,          //当前选择的位置
    round:      usize,          //选择的当前轮数
    max_weight: usize,          //最大的权重
    weights:    [usize; LEN],   //待选择的权重数组
}

impl<const LEN: usize> Default for IWRRSelectorByWider<LEN> {
    /// 默认构建指定长度且权重相同的交替加权轮询选择器
    fn default() -> Self {
        Self::new([1; LEN])
    }
}

impl<const LEN: usize> IWRRSelectorByWider<LEN> {
    /// 构建指定待选择的权重数组的交替加权轮询选择器
    pub fn new(weights: [usize; LEN]) -> Self {
        let mut max_weight = 0;
        for weight in weights {
            if weight >= usize::MAX {
                panic!("Create IWRRSelector failed, weight: {}, reason: invalid weight",
                       weight);
            }

            if max_weight < weight {
                //替换最大的权重
                max_weight = weight;
            }
        }

        IWRRSelectorByWider {
            round: 0,
            max_weight,
            pos: 0,
            weights,
        }
    }

    /// 获取待选择的权重数组的长度
    pub const fn len(&self) -> usize {
        LEN
    }

    /// 获取选择的当前轮数
    pub fn round(&self) -> usize {
        self.round
    }

    /// 获取最大的权重
    pub fn max_weight(&self) -> usize {
        self.max_weight
    }

    /// 尝试获取指定位置的权重
    pub fn try_weight(&self, index: usize) -> Option<usize> {
        if index >= self.len() {
            None
        } else {
            Some(self.weights[index])
        }
    }

    /// 改变指定位置的权重，改变成功则返回指定位置的上个权重
    pub fn change_weight(&mut self,
                         index: usize,
                         weight: usize) -> Option<usize> {
        if weight >= usize::MAX {
            return None;
        }

        if let Some(old) = self.try_weight(index) {
            self.weights[index] = weight;
            Some(old)
        } else {
            None
        }
    }

    /// 获取当前选择的位置
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// 根据权重选择，并返回被选择的位置
    pub fn select(&mut self) -> usize {
        loop {
            for pos in self.pos..self.len() {
                let weight = self.weights[pos];
                if weight == 0 || weight < self.round {
                    //被忽略，则继续下一个位置的选择
                    self.pos += 1;
                    continue;
                }

                //返回被选择的位置
                self.pos += 1;
                return pos;
            }

            if self.round > self.max_weight {
                //完成当前周期的选择，则重置选择器
                self.reset();
            } else {
                //完成当前轮的选择，则重置位置，并继续下一轮的选择
                self.pos = 0;
                self.round += 1;
            }
        }
    }

    /// 重置选择器
    pub fn reset(&mut self) {
        self.round = 0;
        self.pos = 0;
    }
}