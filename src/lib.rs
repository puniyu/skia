use skia_safe::FontMgr;
use skia_safe::textlayout::{FontCollection, TypefaceFontProvider};
use std::sync::{LazyLock, RwLock};

mod error;
pub use error::Error;
mod store;

static FONT_STORE: LazyLock<RwLock<store::FontStore>> =
    LazyLock::new(|| RwLock::new(store::FontStore::default()));

pub struct FontManger {
    font_collection: FontCollection,
    font_provider: TypefaceFontProvider,
}

impl Default for FontManger {
    fn default() -> Self {
        let mut font_collection = FontCollection::new();
        let font_provider = TypefaceFontProvider::new();

        font_collection.set_default_font_manager(FontMgr::new(), None);

        Self {
            font_collection,
            font_provider,
        }
    }
}

impl FontManger {
    /// 创建新的字体管理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册字体，返回字体族名
    pub fn register_font(
        &mut self,
        font_data: &[u8],
        font_family: Option<&str>,
    ) -> Result<(), Error> {
        let mut store = FONT_STORE.write().map_err(|_| Error::LockFailed)?;

        let family_name = store
            .insert(font_data, font_family)
            .ok_or(Error::InvalidFontData)?;

        if let Some(typeface) = store.get(&family_name) {
            self.font_provider
                .register_typeface(typeface.clone(), Some(family_name.as_str()));
            self.font_collection
                .set_asset_font_manager(Some(self.font_provider.clone().into()));
            Ok(())
        } else {
            Err(Error::FamilyNameNotFound)
        }
    }

    pub fn font_collection(&self) -> &FontCollection {
        &self.font_collection
    }
}
