use eframe::{
  egui::{self, Layout, RichText, Widget},
  emath::Align,
  epaint::{TextureId, Vec2},
};

use crate::icon;

pub struct Details(pub TextureId);
impl Widget for Details {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    let Details(txtr) = self;
    ui.scope(|ui| {
      ui.spacing_mut().item_spacing = Vec2::splat(5.0);
      ui.group(|ui| {
        ui.spacing_mut().item_spacing = Vec2::splat(15.0);

        ui.with_layout(Layout::top_down(Align::Center), |ui|
            ui.allocate_ui(Vec2::new(ui.available_width(), 0.0), |ui| {
                ui.spacing_mut().item_spacing = Vec2::splat(15.0);

                ui.add_space(10.0);

                // Info
                ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                    ui.add_space(10.0);

                    // Thumbnail
                    ui.image(txtr, Vec2::new(160.0, 90.0));
                    // Title
                    ui.label(RichText::new("Why do hurricane lanterns look like that?").size(24.0));

                    ui.add_space(10.0);
                });

                // Action buttons
                ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;
                    ui.add_space(ui.available_width() - 70.0);

                    if ui.button(RichText::new(icon::PLAY).size(24.0)).clicked() {
                        std::process::Command::new("mpv").arg("/home/lncn/TechConn/HurricaneLanterns.mkv").spawn().unwrap();
                    }

                    ui.button("...");

                    ui.add_space(10.0);
                });
                ui.add_space(10.0);
        }));
      });
        ui.vertical(|ui| {
          ui.spacing_mut().item_spacing.y = 5.0;
            ui.horizontal(|ui| {
                ui.button("Description (yt_meta)");
                ui.add_enabled_ui(false, |ui| {
                    ui.button("MediaInfo");
                });
            });
egui::ScrollArea::vertical().show(ui, |ui| {
            ui.group(|ui| {
                ui.spacing_mut().item_spacing = Vec2::splat(15.0);
                ui.label(RichText::new(r#"General
Unique ID                                : 248490056988033329573181413624658688534 (0xBAF171B4B43A2A284DF37FB6E6A67A16)
Complete name                            : TechConn/HurricaneLanterns.mkv
Format                                   : Matroska
Format version                           : Version 4
File size                                : 726 MiB
Duration                                 : 31 min 45 s
Overall bit rate                         : 3 195 kb/s
Writing application                      : Lavf58.76.100
Writing library                          : Lavf58.76.100
ErrorDetectionType                       : Per level 1

Video
ID                                       : 1
Format                                   : HEVC
Format/Info                              : High Efficiency Video Coding
Format profile                           : Main@L4@Main
Codec ID                                 : V_MPEGH/ISO/HEVC
Duration                                 : 31 min 45 s
Width                                    : 1 920 pixels
Height                                   : 1 080 pixels
Display aspect ratio                     : 16:9
Frame rate mode                          : Constant
Frame rate                               : 29.970 (30000/1001) FPS
Color space                              : YUV
Chroma subsampling                       : 4:2:0
Bit depth                                : 8 bits
Writing library                          : x265 3.5+1-f0c1022b6:[Linux][GCC 12.2.1][64 bit] 8bit+10bit+12bit
Encoding settings                        : cpuid=1111039 / frame-threads=2 / wpp / no-pmode / no-pme / no-psnr / no-ssim / log-level=2 / input-csp=1 / input-res=1920x1080 / interlace=0 / total-frames=0 / level-idc=0 / high-tier=1 / uhd-bd=0 / ref=4 / no-allow-non-conformance / no-repeat-headers / annexb / no-aud / no-hrd / info / hash=0 / no-temporal-layers / open-gop / min-keyint=25 / keyint=250 / gop-lookahead=0 / bframes=4 / b-adapt=2 / b-pyramid / bframe-bias=0 / rc-lookahead=25 / lookahead-slices=4 / scenecut=40 / hist-scenecut=0 / radl=0 / no-splice / no-intra-refresh / ctu=64 / min-cu-size=8 / rect / no-amp / max-tu-size=32 / tu-inter-depth=1 / tu-intra-depth=1 / limit-tu=0 / rdoq-level=2 / dynamic-rd=0.00 / no-ssim-rd / signhide / no-tskip / nr-intra=0 / nr-inter=0 / no-constrained-intra / strong-intra-smoothing / max-merge=3 / limit-refs=3 / limit-modes / me=3 / subme=3 / merange=57 / temporal-mvp / no-frame-dup / no-hme / weightp / no-weightb / no-analyze-src-pics / deblock=0:0 / sao / no-sao-non-deblock / rd=4 / selective-sao=4 / no-early-skip / rskip / no-fast-intra / no-tskip-fast / no-cu-lossless / no-b-intra / no-splitrd-skip / rdpenalty=0 / psy-rd=2.00 / psy-rdoq=1.00 / no-rd-refine / no-lossless / cbqpoffs=0 / crqpoffs=0 / rc=crf / crf=19.0 / qcomp=0.60 / qpstep=4 / stats-write=0 / stats-read=0 / ipratio=1.40 / pbratio=1.30 / aq-mode=2 / aq-strength=1.00 / cutree / zone-count=0 / no-strict-cbr / qg-size=32 / no-rc-grain / qpmax=69 / qpmin=0 / no-const-vbv / sar=1 / overscan=0 / videoformat=5 / range=0 / colorprim=1 / transfer=1 / colormatrix=1 / chromaloc=0 / display-window=0 / cll=0,0 / min-luma=0 / max-luma=255 / log2-max-poc-lsb=8 / vui-timing-info / vui-hrd-info / slices=1 / no-opt-qp-pps / no-opt-ref-list-length-pps / no-multi-pass-opt-rps / scenecut-bias=0.05 / hist-threshold=0.03 / no-opt-cu-delta-qp / no-aq-motion / no-hdr10 / no-hdr10-opt / no-dhdr10-opt / no-idr-recovery-sei / analysis-reuse-level=0 / analysis-save-reuse-level=0 / analysis-load-reuse-level=0 / scale-factor=0 / refine-intra=0 / refine-inter=0 / refine-mv=1 / refine-ctu-distortion=0 / no-limit-sao / ctu-info=0 / no-lowpass-dct / refine-analysis-type=0 / copy-pic=1 / max-ausize-factor=1.0 / no-dynamic-refine / no-single-sei / no-hevc-aq / no-svt / no-field / qp-adaptation-range=1.00 / scenecut-aware-qp=0conformance-window-offsets / right=0 / bottom=0 / decoder-max-rate=0 / no-vbv-live-multi-pass
Language                                 : English
Default                                  : Yes
Forced                                   : No
Color range                              : Limited
Color primaries                          : BT.709
Transfer characteristics                 : BT.709
Matrix coefficients                      : BT.709

Audio
ID                                       : 2
Format                                   : Opus
Codec ID                                 : A_OPUS
Duration                                 : 31 min 45 s
Channel(s)                               : 2 channels
Channel layout                           : L R
Sampling rate                            : 48.0 kHz
Bit depth                                : 32 bits
Compression mode                         : Lossy
Language                                 : English
Default                                  : Yes
Forced                                   : No

"#).monospace().size(10.0));
            });
        })
      });
    })
    .response
  }
}
