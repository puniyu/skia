mod store;

use std::sync::{LazyLock, RwLock};

use skia_safe::textlayout::{FontCollection, TypefaceFontProvider};
use skia_safe::FontMgr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FontError {
    #[error("获取字体存储锁失败")]
    LockFailed,
    #[error("无效的字体数据")]
    InvalidFontData,
    #[error("无法获取字体族名")]
    FamilyNameNotFound,
}

static FONT_STORE: LazyLock<RwLock<store::FontStore>> =
    LazyLock::new(|| RwLock::new(store::FontStore::default()));

pub struct FontManger {
    font_collection: FontCollection,
    font_provider: TypefaceFontProvider,
}

impl Default for FontManger {
    fn default() -> Self {
        Self::new()
    }
}

impl FontManger {
    /// 创建新的字体管理器
    pub fn new() -> Self {
        let mut font_collection = FontCollection::new();
        let font_provider = TypefaceFontProvider::new();

        font_collection.set_default_font_manager(FontMgr::new(), None);

        Self {
            font_collection,
            font_provider,
        }
    }

    /// 注册字体，返回字体族名
    pub fn register_font(
        &mut self,
        font_data: &[u8],
        font_family: Option<&str>,
    ) -> Result<String, FontError> {
        let mut store = FONT_STORE.write().map_err(|_| FontError::LockFailed)?;

        let family_name = store
            .insert(font_data, font_family)
            .ok_or(FontError::InvalidFontData)?;

        if let Some(typeface) = store.get(&family_name) {
            self.font_provider
                .register_typeface(typeface.clone(), Some(family_name.as_str()));
            self.font_collection
                .set_asset_font_manager(Some(self.font_provider.clone().into()));
            Ok(family_name)
        } else {
            Err(FontError::FamilyNameNotFound)
        }
    }

    pub fn font_collection(&self) -> &FontCollection {
        &self.font_collection
    }
}
