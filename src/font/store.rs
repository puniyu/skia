use skia_safe::{FontMgr, Typeface};
use std::collections::HashMap;

/// 字体存储,管理已加载的字体
#[derive(Debug, Default, Clone)]
pub(crate) struct FontStore(HashMap<String, Typeface>);

impl FontStore {
    /// 插入新字体，成功返回字体族名
    pub fn insert(&mut self, font_data: &[u8], font_family: Option<&str>) -> Option<String> {
        let typeface = FontMgr::default().new_from_data(font_data, None)?;
        let family_name = font_family
            .map(|s| s.to_string())
            .unwrap_or_else(|| typeface.family_name());
        self.0.insert(family_name.clone(), typeface);
        Some(family_name)
    }

    /// 移除指定字体族的字体
    ///
    /// # 返回
    /// - `true`: 成功移除
    /// - `false`: 字体族不存在
    pub fn remove(&mut self, font_family: &str) -> bool {
        self.0.remove(font_family).is_some()
    }

    /// 获取指定字体族的字体
    pub(crate) fn get(&self, font_family: &str) -> Option<&Typeface> {
        self.0.get(font_family)
    }

}

impl<'a> IntoIterator for &'a FontStore {
    type Item = (&'a String, &'a Typeface);
    type IntoIter = std::collections::hash_map::Iter<'a, String, Typeface>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
