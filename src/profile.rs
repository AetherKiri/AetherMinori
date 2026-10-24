use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct ArchiveKeyProfile {
    pub archive: &'static str,
    pub index: &'static str,
    pub data: Option<&'static str>,
    /// The entry password for archives that use one password for everything.
    pub password: Option<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub struct TypePasswordProfile {
    pub png: &'static str,
    pub ogg: &'static str,
    pub sc: &'static str,
    pub avi: &'static str,
}

/// Where a game keeps its type passwords.
///
/// Supipara and Trinoline pick the password from the entry extension; ef uses a
/// single per-archive password for every encrypted (unpacked) entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypePasswordScheme {
    PerExtension,
    PerArchive,
}

#[derive(Debug)]
pub struct ArchiveProfile {
    pub id: &'static str,
    pub executable_markers: &'static [&'static str],
    pub archive_keys: &'static [ArchiveKeyProfile],
    pub type_passwords: TypePasswordProfile,
    pub password_scheme: TypePasswordScheme,
    pub rc4_skip: Rc4SkipScheme,
}

/// How many RC4 keystream bytes to discard before the first payload byte.
///
/// Supipara/Trinoline discard (crc32(key) >> 12) & 0xFF; ef starts at zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rc4SkipScheme {
    Crc32,
    None,
}

impl ArchiveProfile {
    pub fn matches_root(&self, root: &Path) -> bool {
        self.executable_markers
            .iter()
            .any(|name| root.join(name).is_file())
    }

    pub fn archive_key(&self, archive: &str) -> Option<ArchiveKeyProfile> {
        self.archive_keys
            .iter()
            .copied()
            .find(|keys| keys.archive == archive)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LogicalRect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl LogicalRect {
    pub fn contains(self, x: f64, y: f64) -> bool {
        (self.left..self.right).contains(&x) && (self.top..self.bottom).contains(&y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMenuItemKind {
    Skip,
    QuickSave,
    System,
    Load,
    Save,
}

#[derive(Debug, Clone, Copy)]
pub struct GameMenuItemProfile {
    pub kind: GameMenuItemKind,
    pub resource: &'static str,
    pub x: i32,
    pub y: i32,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct GameMenuProfile {
    pub background: &'static str,
    pub right_margin: i32,
    pub bottom_margin: i32,
    pub trigger_size: f64,
    pub items: &'static [GameMenuItemProfile],
}

#[derive(Debug)]
pub struct SaveLoadProfile {
    pub base: &'static str,
    pub save_heading: &'static str,
    pub load_heading: &'static str,
    pub page_resource: &'static str,
    pub page_x: i32,
    pub page_y: i32,
    pub left_column: LogicalRect,
    pub right_column: LogicalRect,
    pub first_row_top: f64,
    pub row_height: f64,
    pub row_step: f64,
    pub close: LogicalRect,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
    pub thumbnail_left: [i32; 2],
    pub thumbnail_top: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct MessageWaitProfile {
    pub resource: &'static str,
    pub frames: u8,
    pub interval_ms: u32,
    pub x: f64,
    pub y: f64,
}

/// The in-engine main menu, when the game has one.
///
/// Only the games whose title screen is drawn by the runtime itself carry this.
/// ef draws its title screen in native code and has none in the script corpus,
/// so its profile leaves this as `None` and the game starts straight away --
/// the host's own library UI is the launcher.
#[derive(Debug)]
pub struct TitleProfile {
    pub background: &'static str,
    pub new_game: LogicalRect,
    pub load_game: LogicalRect,
}

/// Everything the runtime draws or lays out on the game's behalf.
///
/// This is the part that used to be duplicated between game profiles even when
/// the two games shared the same artwork and geometry.
#[derive(Debug)]
pub struct UiProfile {
    pub title: Option<TitleProfile>,
    pub message_panel: &'static str,
    /// How far below the bottom edge the panel is anchored. The panel art
    /// carries its own soft top edge, so the offset is part of the asset, not
    /// a layout constant.
    pub message_panel_offset_y: u32,
    pub message_wait: MessageWaitProfile,
    pub game_menu: GameMenuProfile,
    pub save_load: SaveLoadProfile,
}

#[derive(Debug)]
pub struct GameProfile {
    pub id: &'static str,
    pub archive: &'static ArchiveProfile,
    pub entry_script: &'static str,
    /// The pixel space the game's script coordinates live in.
    pub canvas: crate::scene::Canvas,
    pub ui: UiProfile,
}

impl GameProfile {
    pub fn matches_root(&self, root: &Path) -> bool {
        self.archive.matches_root(root)
    }

    /// True when the runtime owns the title screen and must show it first.
    pub fn has_title_screen(&self) -> bool {
        self.ui.title.is_some()
    }

    /// The drawing surface the game's script coordinates are expressed in.
    pub fn canvas(&self) -> crate::scene::Canvas {
        self.canvas
    }
}

const EF_ARCHIVE_KEYS: &[ArchiveKeyProfile] = &[
    // Movie index key is the negated executable bytes at 0x1a7c00. Payloads
    // use the index's 256-byte table, not a Blowfish data key.
    ArchiveKeyProfile { archive: "mov", index: "5A4CAF2034CC9510780CE2B9BAE97B9C2026274E7FDD36C58D10325F0337E1EA", password: Some("qmzj4Y9C"), data: None },
    ArchiveKeyProfile { archive: "bgm", index: "78E52F7768FA49EA1B0EF53F7B38014A835BC8C948EB78D0AB2688B85D46F307", password: Some("52AUwF8B"), data: Some("8B7731E7AA9DAE4C1F216750C32B9D395794B257C353A3C9DFA880733026D32B") },
    ArchiveKeyProfile { archive: "voice", index: "2FD5D774760B34126F7C3E964FE6CA4CFCDF6A730B891CB1618335270FEB79DF", password: Some("52AUwF8B"), data: Some("151694FDCF2CE124A14E6D1E46E1A7B17395C0CC172DA47CA3AD8CAE42043258") },
    ArchiveKeyProfile { archive: "se", index: "C3011A3ED51FCD2BD3904DD1772E469709B10D5DC0CB4652CFF5B3A57CDE9038", password: Some("52AUwF8B"), data: Some("3DCA17D9518DE518087077409C8BCB396A6BE53DA76E650238E09905096D1AA8") },
    ArchiveKeyProfile { archive: "scr", index: "72634E755D2C980B205F82828BDE4379F37BD2B66D4390B6436A4D9F8389CA5A", password: Some("102ycz1s"), data: Some("B7D2955CF7A0D3ECC20CCBF887A8C1B6D498ECF5F9B8361E897E4DBC7BAD1465") },
    ArchiveKeyProfile { archive: "bg",  index: "8C7919517E51A0FD8348DC85CB68858E68905BD7D5B96336EA3699C15B683F83", password: Some("a7b77qgx"), data: Some("CA3C7817E1C631E5F88DE7AFF702A3697CB9D5E337E54EDA45546C2910C75FDE") },
    ArchiveKeyProfile { archive: "st",  index: "B891BB7B469812FE98CC267BF1356131C694D340D79D8CD10DB45C5D81C44A9C", password: Some("a7b77qgx"), data: Some("630BDF10C9E57EF99126B50476F59800240CD1B3766A1416E31EBFEE27694F93") },
    ArchiveKeyProfile { archive: "sys", index: "0112C9880D9DD23A0573BAC209226D0E80A8AE1EF8B33A76494962531BD1110B", password: Some("a7b77qgx"), data: Some("FD399525E2B2762FF9BC115DD145F6EB9E6DE3574574D04BCD21146BE6537951") },
];

pub static EF_ARCHIVE_PROFILE: ArchiveProfile = ArchiveProfile {
    id: "ef-first-tale",
    executable_markers: &["ef_first_AA.exe", "ef_first_cn_AA.dll", "ef_first_en_AA.chm"],
    archive_keys: EF_ARCHIVE_KEYS,
    type_passwords: TypePasswordProfile {
        png: "a7b77qgx",
        ogg: "a7b77qgx",
        sc: "102ycz1s",
        avi: "a7b77qgx",
    },
    password_scheme: TypePasswordScheme::PerArchive,
    rc4_skip: Rc4SkipScheme::None,
};
const SUPIPARA_ARCHIVE_KEYS: &[ArchiveKeyProfile] = &[
    ArchiveKeyProfile {
        archive: "sys",
        index: "DC2EA1EA00FEA73CCB10E86DC56FCEB026A9C31CE2315BFB5C8F6FD6973B8E83",
        password: None, data: Some("E6AF033FB4191EBA5A06709EB7FD482917E08DE670C7E5B5B82224C2C8F3A76F"),
    },
    ArchiveKeyProfile {
        archive: "scr",
        index: "3F6BD315E116186C16C520D9634D69A966E93223D1032D333B72D9CA01569DC2",
        password: None, data: Some("67409C9A2A59ABC9FDD27755C832ED2107A8F5B15FC4FCF90B707B5FD04B92F8"),
    },
    ArchiveKeyProfile {
        archive: "bg",
        index: "3EEEE8E90D94AAB6AA162783107F617905AC8F8A9AEFC38C05B6A7A1D64A2127",
        password: None, data: Some("442844CAF0466C67613310F16990D9258BB8FE05E41DFA1234D3AED5FAA5F615"),
    },
    ArchiveKeyProfile {
        archive: "st",
        index: "972F3265474FDA1D7F2896135690716C15788941DAE332CC0BA151C88712BE82",
        password: None, data: Some("397F9CAEE01FF67AF70BAC717BAA14D0CE0DA7BCAFC5CF3A926BFA464186AA8D"),
    },
    ArchiveKeyProfile {
        archive: "bgm",
        index: "E0E3A1A338FAB67CE04B09D79219B17611CC9A6BC306D101B8A62EB2802FD069",
        password: None, data: Some("E6697893B71850FC28B28B43A4B3DE789825784D4F6A787A344150E46C317F73"),
    },
    ArchiveKeyProfile {
        archive: "se",
        index: "DE4699D80BF8EC5890B8752CA68B6C94474AECD71EF5DC989A67C61414D8460A",
        password: None, data: Some("5AA4666FAE80E8AFE70CC0AA81F522E1C861CE7E0B0AB2B06EC68155F0E20834"),
    },
    ArchiveKeyProfile {
        archive: "voice",
        index: "72EFDF542ECD672CA1DFF05D1F753FA33C3F270A99E1919CC99DFDF086FAB3E8",
        password: None, data: Some("963A964FF9784A657D8621929C1502F8339926F1E33996A7935FDFA653FFA1A4"),
    },
    ArchiveKeyProfile {
        archive: "mov",
        index: "102CCE3EB7B6694BD1ADBEC6D675245D643A32A0DF18A922A952468B249225BA",
        password: None, data: None,
    },
];

pub static SUPIPARA_ARCHIVE_PROFILE: ArchiveProfile = ArchiveProfile {
    id: "supipara-story-01",
    executable_markers: &["sppl1.exe", "sppl1_cn.exe"],
    archive_keys: SUPIPARA_ARCHIVE_KEYS,
    type_passwords: TypePasswordProfile {
        png: "36TXHE5N",
        ogg: "oRQCAU1o",
        sc: "gTcL74ch",
        avi: "hYPH3Fxw",
    },
    password_scheme: TypePasswordScheme::PerExtension,
    rc4_skip: Rc4SkipScheme::Crc32,
};

const TRINOLINE_ARCHIVE_KEYS: &[ArchiveKeyProfile] = &[
    ArchiveKeyProfile {
        archive: "bg",
        index: "955FC5ACAA293514A2B75A8A9D06543CE75B22BBFA57BD049762FC07D4232F7F",
        password: None, data: Some("5B81450BC03E5E1510A0CC66C41AC8047C5A4C05B05C419C8A13088259033CE2"),
    },
    ArchiveKeyProfile {
        archive: "bgm",
        index: "B90688022DAA47DA531AD6A181CCB10EDB05604E8A6C8DFF010E2DD9E9355158",
        password: None, data: Some("43778DDEB785AD88BA47EC887D80ADDE00FED2531A868F151CD10AB5611209CA"),
    },
    ArchiveKeyProfile {
        archive: "mov",
        index: "9C49892514D8BE366A529ECEE68801BAF9B7B3720EF23F81C0EECAF0D05CD96E",
        password: None, data: None,
    },
    ArchiveKeyProfile {
        archive: "scr",
        index: "B3B3E21EEE2CA5CA220358256DB6033CD4257D06B89637654AEA1C6283B8436E",
        password: None, data: Some("FB8E18A6FDE6CFFFB366027EE508F10017477713112E34D732377721F8505653"),
    },
    ArchiveKeyProfile {
        archive: "se",
        index: "0479A5EC44A8BF0CA5D3702D4C9DF27F86B85D19FE12778748BEAD2FAC5D3155",
        password: None, data: Some("5E37BB71679A7A654F468FF9D59F46E604186F09FC7D42F4B8231BAA854FBBE2"),
    },
    ArchiveKeyProfile {
        archive: "st",
        index: "EC7EABF7F19EE3E0466DC3BFC04D945A3BE3B7F950878E7C69146F34EF9DAAAA",
        password: None, data: Some("B955B50065E91822A6A9708366519E4AA5F6C86D671EB286DFB778F7BFD4EEB4"),
    },
    ArchiveKeyProfile {
        archive: "sys",
        index: "F6DBE06AFD38B69695438F3D10F0BF7578C3462697BA1A4ECCDDE1B88CC57BB9",
        password: None, data: Some("5B934AEDFC8687438DA7015C9933E525A1EF96FFF49E8784B0DB48586F49F9AC"),
    },
    ArchiveKeyProfile {
        archive: "voice",
        index: "2CA40C776100657BCBD15085F737718AA69C475F00FCD9D0C206E52D07FC1C34",
        password: None, data: Some("BC60BE1B82807E1E886FAB45E9ECDBD3FD301ECDA7D7D3E639EA54DAE53325B4"),
    },
];

pub static TRINOLINE_ARCHIVE_PROFILE: ArchiveProfile = ArchiveProfile {
    id: "trinoline",
    executable_markers: &["trinoline.exe", "trinoline_chs.exe"],
    archive_keys: TRINOLINE_ARCHIVE_KEYS,
    type_passwords: TypePasswordProfile {
        png: "B64oH18C",
        ogg: "PiSKji5i",
        sc: "z3Mw9x2r",
        avi: "8n4DgsEb",
    },
    password_scheme: TypePasswordScheme::PerExtension,
    rc4_skip: Rc4SkipScheme::Crc32,
};

const SUPIPARA_GAME_MENU_ITEMS: &[GameMenuItemProfile] = &[
    GameMenuItemProfile {
        kind: GameMenuItemKind::Skip,
        resource: "gameMenuSkip.png",
        x: 23,
        y: 41,
        width: 52.0,
        height: 39.0,
    },
    GameMenuItemProfile {
        kind: GameMenuItemKind::QuickSave,
        resource: "gameMenuQSave.png",
        x: 100,
        y: 22,
        width: 50.0,
        height: 51.0,
    },
    GameMenuItemProfile {
        kind: GameMenuItemKind::System,
        resource: "gameMenuSystem.png",
        x: 112,
        y: 97,
        width: 36.0,
        height: 27.0,
    },
    GameMenuItemProfile {
        kind: GameMenuItemKind::Load,
        resource: "gameMenuLoad.png",
        x: 72,
        y: 127,
        width: 33.0,
        height: 34.0,
    },
    GameMenuItemProfile {
        kind: GameMenuItemKind::Save,
        resource: "gameMenuSave.png",
        x: 27,
        y: 101,
        width: 37.0,
        height: 37.0,
    },
];

pub static TRINOLINE_PROFILE: GameProfile = GameProfile {
    id: "trinoline",
    archive: &TRINOLINE_ARCHIVE_PROFILE,
    entry_script: "test.sc",
    canvas: crate::scene::Canvas::new(1280, 720),
    ui: UiProfile {
        title: Some(TitleProfile {
            background: "topmenu\\TopMenu.png",
            new_game: LogicalRect {
                left: 260.0,
                top: 220.0,
                right: 400.0,
                bottom: 280.0,
            },
            load_game: LogicalRect {
                left: 420.0,
                top: 220.0,
                right: 510.0,
                bottom: 280.0,
            },
        }),
        message_panel: "msgPanel.png",
        message_panel_offset_y: 64,
        message_wait: MessageWaitProfile {
            resource: "pauseAnimeNextPage.png",
            frames: 9,
            interval_ms: 100,
            x: 1138.0,
            y: 674.0,
        },
        game_menu: GameMenuProfile {
            background: "gameMenuBack.png",
            right_margin: 6,
            bottom_margin: 15,
            trigger_size: 96.0,
            items: SUPIPARA_GAME_MENU_ITEMS,
        },
        save_load: SaveLoadProfile {
            base: "saveload\\saveDefaultBackground.png",
            save_heading: "saveload\\saveText.png",
            load_heading: "saveload\\loadText.png",
            page_resource: "saveload\\page01.png",
            page_x: 0,
            page_y: 0,
            left_column: LogicalRect {
                left: 0.0,
                top: 0.0,
                right: 640.0,
                bottom: 720.0,
            },
            right_column: LogicalRect {
                left: 640.0,
                top: 0.0,
                right: 1280.0,
                bottom: 720.0,
            },
            first_row_top: 81.0,
            row_height: 96.0,
            row_step: 108.0,
            close: LogicalRect {
                left: 1080.0,
                top: 0.0,
                right: 1280.0,
                bottom: 80.0,
            },
            thumbnail_width: 160,
            thumbnail_height: 90,
            thumbnail_left: [68, 459],
            thumbnail_top: 84,
        },
    },
};

pub static SUPIPARA_PROFILE: GameProfile = GameProfile {
    id: "supipara-story-01",
    archive: &SUPIPARA_ARCHIVE_PROFILE,
    entry_script: "test.sc",
    canvas: crate::scene::Canvas::new(1280, 720),
    ui: UiProfile {
        title: Some(TitleProfile {
            background: "topMenu1.png",
            new_game: LogicalRect {
                left: 1056.0,
                top: 32.0,
                right: 1280.0,
                bottom: 64.0,
            },
            load_game: LogicalRect {
                left: 1056.0,
                top: 76.0,
                right: 1280.0,
                bottom: 112.0,
            },
        }),
        message_panel: "msgPanel.png",
        message_panel_offset_y: 64,
        message_wait: MessageWaitProfile {
            resource: "pauseAnimeNextPage.png",
            frames: 9,
            interval_ms: 100,
            x: 1138.0,
            y: 674.0,
        },
        game_menu: GameMenuProfile {
            background: "gameMenuBack.png",
            right_margin: 6,
            bottom_margin: 15,
            trigger_size: 96.0,
            items: SUPIPARA_GAME_MENU_ITEMS,
        },
        save_load: SaveLoadProfile {
            base: "saveloadBase.png",
            save_heading: "saveloadSave.png",
            load_heading: "saveloadLoad.png",
            page_resource: "saveload_Page1.png",
            page_x: 1072,
            page_y: 0,
            left_column: LogicalRect {
                left: 65.0,
                top: 0.0,
                right: 408.0,
                bottom: 720.0,
            },
            right_column: LogicalRect {
                left: 456.0,
                top: 0.0,
                right: 800.0,
                bottom: 720.0,
            },
            first_row_top: 81.0,
            row_height: 96.0,
            row_step: 108.0,
            close: LogicalRect {
                left: 1080.0,
                top: 0.0,
                right: 1280.0,
                bottom: 80.0,
            },
            thumbnail_width: 160,
            thumbnail_height: 90,
            thumbnail_left: [68, 459],
            thumbnail_top: 84,
        },
    },
};

/// ef - the first title, at its native 800x600.
pub static EF_PROFILE: GameProfile = GameProfile {
    id: "ef-first-tale",
    archive: &EF_ARCHIVE_PROFILE,
    entry_script: "test.sc",
    canvas: crate::scene::Canvas::new(800, 600),
    ui: UiProfile {
        // The retail title screen is drawn by native code and the script
        // corpus has no title script, so the runtime does not own one.
        title: None,
        message_panel: "msgPanel.png",
        message_panel_offset_y: 0,
        message_wait: MessageWaitProfile {
            resource: "pauseAnimeNextPage.png",
            frames: 9,
            interval_ms: 100,
            x: 780.0,
            y: 574.0,
        },
        game_menu: GameMenuProfile {
            background: "gameMenuBack.png",
            right_margin: 6,
            bottom_margin: 15,
            trigger_size: 96.0,
            items: SUPIPARA_GAME_MENU_ITEMS,
        },
        save_load: SaveLoadProfile {
            base: "saveloadBase.png",
            save_heading: "saveloadSave.png",
            load_heading: "saveloadLoad.png",
            page_resource: "saveload_Page1.png",
            page_x: 700,
            page_y: 0,
            left_column: LogicalRect {
                left: 40.0,
                top: 0.0,
                right: 400.0,
                bottom: 600.0,
            },
            right_column: LogicalRect {
                left: 410.0,
                top: 0.0,
                right: 760.0,
                bottom: 600.0,
            },
            first_row_top: 66.0,
            row_height: 80.0,
            row_step: 90.0,
            close: LogicalRect {
                left: 700.0,
                top: 0.0,
                right: 800.0,
                bottom: 64.0,
            },
            thumbnail_width: 160,
            thumbnail_height: 120,
            thumbnail_left: [44, 414],
            thumbnail_top: 70,
        },
    },
};

pub fn detect_archive_profile(root: &Path) -> Option<&'static ArchiveProfile> {
    [&SUPIPARA_ARCHIVE_PROFILE, &TRINOLINE_ARCHIVE_PROFILE, &EF_ARCHIVE_PROFILE]
        .into_iter()
        .find(|profile| profile.matches_root(root))
}

pub fn detect_game_profile(root: &Path) -> Option<&'static GameProfile> {
    [&SUPIPARA_PROFILE, &TRINOLINE_PROFILE, &EF_PROFILE]
        .into_iter()
        .find(|profile| profile.matches_root(root))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supipara_profile_keeps_work_specific_resources_out_of_the_core() {
        assert_eq!(SUPIPARA_PROFILE.entry_script, "test.sc");
        let title = SUPIPARA_PROFILE.ui.title.as_ref().unwrap();
        assert!(title.new_game.contains(1100.0, 48.0));
        assert!(title.load_game.contains(1100.0, 96.0));
        assert_eq!(SUPIPARA_PROFILE.ui.game_menu.items.len(), 5);
        assert_eq!(SUPIPARA_PROFILE.archive.archive_keys.len(), 8);
        assert!(SUPIPARA_PROFILE.archive.archive_key("voice").is_some());
    }

    #[test]
    fn ef_profile_has_no_in_engine_title_screen() {
        assert_eq!(EF_ARCHIVE_PROFILE.archive_keys.len(), 8);
        let movie = EF_ARCHIVE_PROFILE.archive_key("mov").unwrap();
        assert_eq!(movie.password, Some("qmzj4Y9C"));
        assert!(movie.data.is_none());
        for archive in ["bgm", "voice", "se"] {
            let keys = EF_ARCHIVE_PROFILE.archive_key(archive).unwrap();
            assert_eq!(keys.password, Some("52AUwF8B"));
            assert_eq!(keys.index.len(), 64);
            assert_eq!(keys.data.unwrap().len(), 64);
        }
        assert_eq!(EF_ARCHIVE_PROFILE.password_scheme, TypePasswordScheme::PerArchive);
        assert_eq!(EF_ARCHIVE_PROFILE.rc4_skip, Rc4SkipScheme::None);
        assert_eq!(EF_ARCHIVE_PROFILE.type_passwords.sc, "102ycz1s");
        assert!(EF_ARCHIVE_PROFILE.archive_key("sys").is_some());
    }

    #[test]
    fn trinoline_archive_profile_uses_garbro_v2_scheme() {
        assert_eq!(TRINOLINE_ARCHIVE_PROFILE.archive_keys.len(), 8);
        assert_eq!(TRINOLINE_ARCHIVE_PROFILE.type_passwords.sc, "z3Mw9x2r");
        assert!(TRINOLINE_ARCHIVE_PROFILE.archive_key("scr").is_some());
    }
}
