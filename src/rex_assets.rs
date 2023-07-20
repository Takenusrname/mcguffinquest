use rltk::rex::XpFile;

rltk::embedded_resource!(MENU_IMAGE, "../resources/McGuffinQuest_80x50.xp");

pub struct RexAssets {
    pub menu: XpFile
}

impl RexAssets {
    
    pub fn new() -> RexAssets {
        rltk::link_resource!(MENU_IMAGE, "../resources/McGuffinQuest_80x50.xp");

        RexAssets{
            menu: XpFile::from_resource("../resources/McGuffinQuest_80x50.xp").unwrap()
        }
    }
}