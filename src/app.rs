//! Screen layout. Everything is drawn inside a mock phone body so the interface reads as a
//! phone app at any window size.

use eframe::egui::{
    self, pos2, vec2, Align, Align2, Color32, CornerRadius, FontId, Layout, Painter, Rect, Response,
    RichText, Sense, Stroke, StrokeKind, Ui, Vec2,
};

use crate::charges::{Category, Charge, CHARGES};
use crate::{leaf, statutes, theme};

#[path = "saved.rs"]
pub(crate) mod saved;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Home,
    Charges,
    Saved,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sort {
    Number,
    Name,
}

pub struct App {
    pub tab: Tab,
    query: String,
    filter: Option<Category>,
    sort: Sort,
    open: Option<usize>,
    stuck: Vec<bool>,
    save_error: Option<String>,
    load_failed: bool,
    reset_list_scroll: bool,
    reset_detail_scroll: bool,
}

impl Default for App {
    fn default() -> Self {
        let saved_ids = saved::storage_path()
            .ok_or_else(|| "Saved charges are unavailable on this device.".to_owned())
            .and_then(|path| saved::read_saved(&path).map_err(|_| "Could not load saved charges.".to_owned()));
        let (stuck, save_error) = match saved_ids {
            Ok(ids) => (CHARGES.iter().map(|charge| saved_id_matches(charge, &ids)).collect(), None),
            Err(message) => (vec![false; CHARGES.len()], Some(message)),
        };
        let load_failed = save_error.is_some();
        Self {
            tab: Tab::Home,
            query: String::new(),
            filter: None,
            sort: Sort::Number,
            open: None,
            stuck,
            save_error,
            load_failed,
            reset_list_scroll: false,
            reset_detail_scroll: false,
        }
    }
}

fn saved_id(charge: &Charge) -> String {
    format!("{}:{}", charge.category.code(), charge.number)
}

fn saved_id_matches(charge: &Charge, ids: &[String]) -> bool {
    let canonical = saved_id(charge);
    ids.contains(&canonical)
        // Earlier builds incorrectly identified this offence by its punishment section.
        || (canonical == "PR:354" && ids.iter().any(|id| id == "PR:355"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        App {
            tab: Tab::Saved,
            query: String::new(),
            filter: None,
            sort: Sort::Number,
            open: None,
            stuck: vec![false; CHARGES.len()],
            save_error: None,
            load_failed: false,
            reset_list_scroll: false,
            reset_detail_scroll: false,
        }
    }

    #[test]
    fn saved_list_is_not_hidden_by_filters_from_the_charges_screen() {
        let saved_index = CHARGES.iter().position(|charge| charge.number == "266").unwrap();
        let mut app = test_app();
        app.query = "nothing matches".to_string();
        app.filter = Some(Category::Property);
        app.stuck[saved_index] = true;
        assert_eq!(app.visible(true), vec![saved_index]);
        assert!(app.visible(false).is_empty());
    }

    #[test]
    fn search_and_category_filter_combine_on_the_charge_list() {
        let mut app = test_app();
        app.query = "  320.14 ".to_string();
        app.filter = Some(Category::Driving);
        let found = app.visible(false);
        assert_eq!(found.len(), 1);
        assert_eq!(CHARGES[found[0]].number, "320.14");
        app.filter = Some(Category::Property);
        assert!(app.visible(false).is_empty());
        app.query = "aSsAuLt".to_string();
        app.filter = Some(Category::Violent);
        assert!(app.visible(false).iter().any(|i| CHARGES[*i].number == "266"));
        app.query = "265".to_string();
        assert!(app.visible(false).iter().any(|i| CHARGES[*i].number == "266"));
    }

    #[test]
    fn legacy_saved_property_charge_is_restored_after_the_section_correction() {
        let charge = Charge {
            number: "354", name: "Possession of property obtained by crime",
            category: Category::Property, kind: crate::charges::Kind::Hybrid,
            penalty: "", plain: "",
        };
        assert!(saved_id_matches(&charge, &["PR:355".to_string()]));
        assert!(saved_id_matches(&charge, &["PR:354".to_string()]));
        assert!(!saved_id_matches(&charge, &["PR:356".to_string()]));
    }

    #[test]
    fn treason_subsections_remain_distinct_when_searched_and_saved() {
        let mut app = test_app();
        app.query = "46(1)".to_string();
        let high = app.visible(false);
        app.query = "46(2)".to_string();
        let other = app.visible(false);
        assert_eq!(high.len(), 1);
        assert_eq!(other.len(), 1);
        assert_eq!(saved_id(&CHARGES[high[0]]), "PO:46(1)");
        assert_eq!(saved_id(&CHARGES[other[0]]), "PO:46(2)");
    }
}

const NAV_H: f32 = 64.0;
const PAD: f32 = 18.0;

impl App {
    /// Test hook: put the app into a named state so a headless check can draw it.
    #[allow(dead_code)]
    pub fn open_mode(&mut self, mode: &str) {
        let find = |number: &str| CHARGES.iter().position(|c| c.number == number).unwrap_or(0);
        match mode {
            "detail" => self.open = Some(find("344")),
            "indictable" => self.open = Some(find("229")),
            "stuck" | "saved" => {
                self.tab = Tab::Saved;
                for n in ["344", "320.14", "5(1)"] {
                    self.stuck[find(n)] = true;
                }
            }
            "home" => self.tab = Tab::Home,
            "charges" | "criminal_code" => self.tab = Tab::Charges,
            "about" => self.tab = Tab::Saved,
            "filtered" => {
                self.tab = Tab::Charges;
                self.filter = Some(Category::Weapons);
            }
            "search" => {
                self.tab = Tab::Charges;
                self.query = "assault".to_string();
            }
            "empty" => {
                self.tab = Tab::Charges;
                self.query = "zzzz".to_string();
            }
            _ => {}
        }
    }
}

impl App {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme::page()))
            .show(ctx, |ui| {
                let avail = ui.available_size();
                let (outer, _) = ui.allocate_exact_size(avail, Sense::hover());

                // The phone: never wider than a real one, never taller than its 2:1 screen.
                let margin = 16.0;
                let phone_w = (avail.x - margin * 2.0).min(430.0).max(300.0);
                let phone_h = (avail.y - margin * 2.0).min(phone_w * 2.02).max(420.0);
                let phone = Rect::from_center_size(outer.center(), vec2(phone_w, phone_h));

                let p = ui.painter();
                for i in 0..6 {
                    let f = i as f32;
                    p.rect_filled(
                        phone.expand(2.0 + f * 3.0),
                        CornerRadius::same((40.0 + f * 3.0) as u8),
                        Color32::from_black_alpha(20 - i * 3),
                    );
                }
                p.rect_filled(phone, CornerRadius::same(40), theme::body());
                p.rect_stroke(
                    phone,
                    CornerRadius::same(40),
                    Stroke::new(1.0, Color32::from_rgb(0x1c, 0x1c, 0x22)),
                    StrokeKind::Inside,
                );

                let screen_rect = phone.shrink(9.0);
                p.rect_filled(screen_rect, CornerRadius::same(32), theme::screen());

                let mut screen = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(screen_rect)
                        .layout(Layout::top_down(Align::Min)),
                );
                screen.set_clip_rect(screen_rect.intersect(ui.clip_rect()));
                self.screen(&mut screen, screen_rect);
            });
    }

    fn screen(&mut self, ui: &mut Ui, rect: Rect) {
        status_bar(ui, rect);

        if self.open.is_some() {
            self.detail(ui, rect);
            return;
        }

        match self.tab {
            Tab::Home => {
                self.title_bar(ui);
                self.home(ui, rect);
            }
            Tab::Charges => {
                self.title_bar(ui);
                self.search_box(ui);
                self.chips(ui);
                self.list(ui, rect, false);
            }
            Tab::Saved => {
                self.title_bar(ui);
                self.list(ui, rect, true);
            }
        }

        self.nav(ui, rect);
    }

    // --- home ---------------------------------------------------------------------

    fn home(&mut self, ui: &mut Ui, rect: Rect) {
        let p = ui.painter();
        let center_x = rect.center().x;
        let inner_w = (rect.width() - PAD * 2.0).max(120.0);

        let h = rect.height();
        let leaf_y = rect.top() + (h * 0.25).max(160.0);

        // Centered red maple leaf icon
        let leaf_size = 42.0;
        let leaf_rect = Rect::from_center_size(pos2(center_x, leaf_y), Vec2::splat(leaf_size));
        for shape in leaf::flag_leaf_shapes(leaf_rect, theme::accent()) {
            p.add(shape);
        }

        // Hero title lines
        let title_line1 = "Before the charge,";
        let title_line2 = "check the Code.";
        let title_y1 = leaf_y + 46.0;
        let title_y2 = title_y1 + 34.0;

        p.text(
            pos2(center_x, title_y1),
            Align2::CENTER_TOP,
            title_line1,
            FontId::proportional(27.0),
            theme::ink(),
        );
        p.text(
            pos2(center_x, title_y2),
            Align2::CENTER_TOP,
            title_line2,
            FontId::proportional(27.0),
            theme::crimson(),
        );

        // Subtitle
        let sub_y = title_y2 + 48.0;
        p.text(
            pos2(center_x, sub_y),
            Align2::CENTER_TOP,
            "Find an offence. Review the essential elements.",
            FontId::proportional(13.0),
            theme::ink_faint(),
        );

        // Search pill towards bottom, just above nav bar
        let pill_h = 50.0;
        let pill_y = rect.bottom() - NAV_H - 34.0;
        let pill_rect = Rect::from_center_size(pos2(center_x, pill_y), vec2(inner_w, pill_h));
        let pill_resp = ui.interact(pill_rect, ui.id().with("home_search_pill"), Sense::click());

        let pill_bg = if pill_resp.hovered() { theme::surface_hover() } else { theme::surface() };
        p.rect_filled(pill_rect, CornerRadius::same(25), pill_bg);
        p.rect_stroke(pill_rect, CornerRadius::same(25), Stroke::new(1.2, theme::line()), StrokeKind::Inside);

        // Search icon (magnifying glass)
        let icon_c = pos2(pill_rect.left() + 24.0, pill_rect.center().y);
        p.circle_stroke(icon_c, 5.5, Stroke::new(1.6, theme::ink_faint()));
        p.line_segment(
            [pos2(icon_c.x + 4.0, icon_c.y + 4.0), pos2(icon_c.x + 8.0, icon_c.y + 8.0)],
            Stroke::new(1.6, theme::ink_faint()),
        );

        // Text input field inside pill
        let text_rect = Rect::from_min_max(
            pos2(pill_rect.left() + 42.0, pill_rect.top() + 6.0),
            pos2(pill_rect.right() - 16.0, pill_rect.bottom() - 6.0),
        );
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(text_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        let response = child.add(
            egui::TextEdit::singleline(&mut self.query)
                .hint_text(
                    RichText::new("Offence, keyword or section")
                        .size(13.5)
                        .color(theme::ink_faint()),
                )
                .desired_width(text_rect.width() - 10.0)
                .frame(false)
                .text_color(theme::ink())
                .font(FontId::proportional(13.5)),
        );

        if pill_resp.clicked() || response.changed() || response.gained_focus() {
            self.tab = Tab::Charges;
            self.reset_list_scroll = true;
        }
    }

    // --- header -------------------------------------------------------------------

    fn title_bar(&mut self, ui: &mut Ui) {
        let w = ui.available_width();
        let (rect, _) = ui.allocate_exact_size(vec2(w, 52.0), Sense::hover());
        let p = ui.painter();
        let leaf_rect = Rect::from_min_size(pos2(rect.left() + PAD, rect.top() + 8.0), Vec2::splat(28.0));
        for shape in leaf::flag_leaf_shapes(leaf_rect, theme::accent()) {
            p.add(shape);
        }
        p.text(
            pos2(leaf_rect.right() + 12.0, rect.top() + 9.0),
            Align2::LEFT_TOP,
            "Stick the Charge",
            FontId::proportional(if w < 340.0 { 16.5 } else { 19.0 }),
            theme::ink(),
        );
        p.text(
            pos2(leaf_rect.right() + 12.5, rect.top() + 32.0),
            Align2::LEFT_TOP,
            "CANADIAN CRIMINAL CODE",
            FontId::proportional(8.5),
            theme::ink_faint(),
        );
        let switch = Rect::from_min_size(pos2(rect.right() - PAD - 58.0, rect.top() + 2.0), vec2(58.0, 44.0));
        let (label, hint) = if theme::is_dark() { ("Light", "Switch to light mode") } else { ("Dark", "Switch to dark mode") };
        let button = egui::Button::new(RichText::new(label).size(12.0).color(theme::ink()))
            .fill(theme::surface())
            .stroke(Stroke::new(1.0, theme::line()))
            .corner_radius(CornerRadius::same(9));
        if ui.put(switch, button).on_hover_text(hint).clicked() {
            theme::toggle(ui.ctx());
        }
    }

    fn search_box(&mut self, ui: &mut Ui) {
        let full = ui.available_width();
        let (rect, _) = ui.allocate_exact_size(vec2(full, 50.0), Sense::hover());
        let field = Rect::from_min_size(pos2(rect.left() + PAD, rect.top() + 4.0), vec2(full - PAD * 2.0, 40.0));
        let p = ui.painter();
        p.rect_filled(field, CornerRadius::same(12), theme::surface());
        p.rect_stroke(field, CornerRadius::same(12), Stroke::new(1.0, theme::line()), StrokeKind::Inside);

        let c = pos2(field.left() + 18.0, field.center().y - 1.0);
        p.circle_stroke(c, 5.5, Stroke::new(1.6, theme::ink_dim()));
        p.line_segment(
            [pos2(c.x + 4.0, c.y + 4.0), pos2(c.x + 8.0, c.y + 8.0)],
            Stroke::new(1.6, theme::ink_dim()),
        );

        let text_rect = Rect::from_min_max(pos2(field.left() + 33.0, field.top() + 8.0), pos2(field.right() - 8.0, field.bottom() - 8.0));
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(text_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        let response = child.add(
            egui::TextEdit::singleline(&mut self.query)
                .hint_text(
                    RichText::new("Search a number or charge")
                        .size(13.0)
                        .color(theme::ink_faint()),
                )
                .desired_width(text_rect.width() - 20.0)
                .frame(false)
                .text_color(theme::ink())
                .font(FontId::proportional(13.0)),
        );
        if response.changed() {
            self.reset_list_scroll = true;
        }
    }

    fn chips(&mut self, ui: &mut Ui) {
        ui.add_space(4.0);
        let avail = ui.available_width();
        let chip_w = |text: &str| -> f32 {
            let gal = ui
                .painter()
                .layout_no_wrap(text.to_string(), FontId::proportional(12.0), theme::ink());
            gal.size().x + 22.0
        };

        // Greedy row packing, measured before allocating, so every row starts at the same
        // inset and no category is ever hidden off the edge.
        let mut labels: Vec<(Option<Category>, &str)> = vec![(None, "All")];
        for cat in Category::ALL {
            labels.push((Some(cat), cat.label()));
        }
        let gap = 7.0;
        let mut rows: Vec<Vec<(Option<Category>, &str, f32)>> = Vec::new();
        let mut row: Vec<(Option<Category>, &str, f32)> = Vec::new();
        let mut used = 0.0;
        for (cat, label) in labels {
            let w = chip_w(label);
            if !row.is_empty() && used + gap + w > avail - PAD * 2.0 {
                rows.push(std::mem::take(&mut row));
                used = 0.0;
            }
            used += if row.is_empty() { w } else { gap + w };
            row.push((cat, label, w));
        }
        if !row.is_empty() {
            rows.push(row);
        }

        let height = rows.len() as f32 * (28.0 + gap) - gap;
        let (area, _) = ui.allocate_exact_size(vec2(avail, height.max(28.0)), Sense::hover());
        let mut new_filter = self.filter;
        for (r, row) in rows.iter().enumerate() {
            let mut x = area.left() + PAD;
            let y = area.top() + r as f32 * (28.0 + gap);
            for (cat, label, w) in row {
                let rect = Rect::from_min_size(pos2(x, y), vec2(*w, 28.0));
                let selected = match cat {
                    None => self.filter.is_none(),
                    Some(c) => self.filter == Some(*c),
                };
                let resp = ui.interact(rect, ui.id().with(("chip", label)), Sense::click());
                paint_chip(ui.painter(), rect, selected, resp.hovered(), label);
                if resp.clicked() {
                    new_filter = match cat {
                        None => None,
                        Some(c) => {
                            if self.filter == Some(*c) {
                                None
                            } else {
                                Some(*c)
                            }
                        }
                    };
                }
                x += w + gap;
            }
        }
        if self.filter != new_filter {
            self.reset_list_scroll = true;
            self.filter = new_filter;
        }
        ui.add_space(2.0);
    }

    // --- charge list --------------------------------------------------------------

    fn visible(&self, stuck_only: bool) -> Vec<usize> {
        let q = self.query.trim().to_lowercase();
        let mut out: Vec<usize> = CHARGES
            .iter()
            .enumerate()
            .filter(|(i, c)| {
                if stuck_only {
                    self.stuck[*i]
                } else {
                    self.filter.map_or(true, |f| c.category == f)
                        && (q.is_empty()
                        || c.name.to_lowercase().contains(&q)
                        || c.number.to_lowercase().contains(&q)
                        || c.related_section().map_or(false, |(_, section)| section.contains(&q)))
                }
            })
            .map(|(i, _)| i)
            .collect();
        match self.sort {
            Sort::Number => out.sort_by(|a, b| number_key(&CHARGES[*a]).cmp(&number_key(&CHARGES[*b]))),
            Sort::Name => out.sort_by(|a, b| CHARGES[*a].name.cmp(CHARGES[*b].name)),
        }
        out
    }

    fn list(&mut self, ui: &mut Ui, rect: Rect, stuck_only: bool) {
        let inner_w = rect.width();
        let rows = self.visible(stuck_only);

        // small caption row: count on the left, sort toggle on the right
        let (head, _) = ui.allocate_exact_size(vec2(rect.width(), 32.0), Sense::hover());
        let caption = if stuck_only {
            format!("STUCK  ·  {}", rows.len())
        } else {
            match self.filter {
                Some(c) => format!("{}  ·  {}", c.label().to_uppercase(), rows.len()),
                None => format!("ALL CHARGES  ·  {}", rows.len()),
            }
        };
        let p = ui.painter();
        p.text(
            pos2(head.left() + PAD, head.center().y),
            Align2::LEFT_CENTER,
            caption,
            FontId::proportional(9.5),
            theme::ink_faint(),
        );

        let label = match self.sort {
            Sort::Number => "BY NUMBER",
            Sort::Name => "BY NAME",
        };
        let size = vec2(76.0, 22.0);
        let sort_rect = Rect::from_min_size(
            pos2(head.right() - PAD - size.x, head.center().y - size.y / 2.0),
            size,
        );
        let resp = ui.interact(sort_rect, ui.id().with("sort"), Sense::click());
        let p = ui.painter();
        p.rect_filled(
            sort_rect,
            CornerRadius::same(9),
            if resp.hovered() { theme::surface_hover() } else { theme::surface() },
        );
        p.text(
            sort_rect.center(),
            Align2::CENTER_CENTER,
            label,
            FontId::proportional(8.5),
            if resp.hovered() { theme::ink() } else { theme::ink_dim() },
        );
        if resp.clicked() {
            self.sort = match self.sort {
                Sort::Number => Sort::Name,
                Sort::Name => Sort::Number,
            };
            self.reset_list_scroll = true;
        }

        if rows.is_empty() {
            let (box_rect, _) = ui.allocate_exact_size(vec2(rect.width(), 170.0), Sense::hover());
            let p = ui.painter();
            let c = box_rect.center();
            for shape in leaf::leaf_shapes(
                Rect::from_center_size(pos2(c.x, c.y - 26.0), Vec2::splat(46.0)),
                Color32::from_rgb(0x2e, 0x32, 0x3a),
            ) {
                p.add(shape);
            }
            let (title, hint) = if stuck_only {
                ("Nothing stuck yet", "Open a charge and stick it here")
            } else {
                ("No charge matches that", "Try another number or word")
            };
            p.text(pos2(c.x, c.y + 24.0), Align2::CENTER_CENTER, title, FontId::proportional(13.0), theme::ink_dim());
            p.text(pos2(c.x, c.y + 44.0), Align2::CENTER_CENTER, hint, FontId::proportional(11.0), theme::ink_faint());
            return;
        }

        let top = ui.cursor().top();
        let max_h = (rect.bottom() - NAV_H - 10.0 - top).max(80.0);
        let mut open = None;
        if self.reset_list_scroll {
            let id = ui.make_persistent_id(egui::Id::new("charges"));
            egui::containers::scroll_area::State::default().store(ui.ctx(), id);
            self.reset_list_scroll = false;
        }
        egui::ScrollArea::vertical()
            .id_salt("charges")
            .max_height(max_h)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                for idx in rows {
                    let (row, resp) = charge_row(ui, &CHARGES[idx], inner_w, self.stuck[idx]);
                    if resp.clicked() {
                        open = Some(idx);
                    }
                    if self.stuck[idx] {
                        ui.painter()
                            .circle_filled(pos2(row.right() - 32.0, row.top() + 14.0), 3.5, theme::accent());
                    }
                    ui.add_space(8.0);
                }
                ui.add_space(4.0);
            });

        if let Some(idx) = open {
            self.open = Some(idx);
            self.reset_detail_scroll = true;
        }
    }

    // --- detail -------------------------------------------------------------------

    fn detail(&mut self, ui: &mut Ui, rect: Rect) {
        let Some(idx) = self.open else { return };
        let charge = &CHARGES[idx];
        let inner_w = rect.width() - PAD * 2.0;
        let mut toggle = false;

        // back bar
        let (bar, _) = ui.allocate_exact_size(vec2(rect.width(), 52.0), Sense::hover());
        let back = Rect::from_center_size(pos2(bar.left() + PAD + 16.0, bar.center().y), Vec2::splat(44.0));
        let resp = ui.interact(back, ui.id().with("back"), Sense::click());
        let p = ui.painter();
        p.circle_filled(
            back.center(),
            17.0,
            if resp.hovered() { theme::surface_hover() } else { theme::surface() },
        );
        let c = back.center();
        let chev = if resp.hovered() { theme::ink() } else { theme::ink_dim() };
        p.line_segment([pos2(c.x + 2.5, c.y - 5.0), pos2(c.x - 3.0, c.y)], Stroke::new(1.8, chev));
        p.line_segment([pos2(c.x - 3.0, c.y), pos2(c.x + 2.5, c.y + 5.0)], Stroke::new(1.8, chev));
        let leaf_rect = Rect::from_center_size(bar.center(), Vec2::splat(36.0));
        for shape in leaf::flag_leaf_shapes(leaf_rect, Color32::from_rgb(0xe3, 0x18, 0x37)) {
            ui.painter().add(shape);
        }
        let save = Rect::from_center_size(pos2(bar.right() - PAD - 38.0, bar.center().y), vec2(76.0, 44.0));
        let save_response = ui.interact(save, ui.id().with("save-detail"), Sense::click());
        ui.painter().text(
            save.center(),
            Align2::CENTER_CENTER,
            if self.stuck[idx] { "Saved" } else { "Save" },
            FontId::proportional(14.0),
            theme::ink(),
        );
        if save_response.clicked() { toggle = true; }
        if resp.clicked() {
            self.open = None;
            return;
        }

        let max_h = (rect.bottom() - 14.0 - ui.cursor().top()).max(120.0);
        if self.reset_detail_scroll {
            let id = ui.make_persistent_id(egui::Id::new("detail"));
            egui::containers::scroll_area::State::default().store(ui.ctx(), id);
            self.reset_detail_scroll = false;
        }
        egui::ScrollArea::vertical()
            .id_salt("detail")
            .max_height(max_h)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;

                name_block(ui, inner_w, charge.name);
                ui.add_space(6.0);
                let section = if charge.category == Category::Violent && charge.number == "266" {
                    "Sections 265 and 266".to_owned()
                } else {
                    format!("Section {}", charge.number)
                };
                paragraph_faint(ui, inner_w, &section);
                if let Some(message) = &self.save_error {
                    ui.add_space(12.0);
                    paragraph(ui, inner_w, message);
                }

                ui.add_space(24.0);
                reading_heading(ui, rect, "Easy breakdown (draft)");
                ui.add_space(10.0);
                if let Some(guide) = charge.guide() {
                    breakdown_paragraphs(ui, inner_w, guide.introduction);
                    ui.add_space(12.0);
                    for point in guide.points {
                        paragraph(ui, inner_w, &format!("• {point}"));
                        ui.add_space(12.0);
                    }
                    if !guide.context.is_empty() {
                        breakdown_paragraphs(ui, inner_w, guide.context);
                    }
                    ui.add_space(20.0);
                    reading_heading(ui, rect, &format!("Penalty · section {}", guide.penalty_section));
                    ui.add_space(10.0);
                    paragraph(ui, inner_w, &format!("Minimum: {}", guide.minimum));
                    ui.add_space(12.0);
                    for penalty in guide.penalties {
                        paragraph(ui, inner_w, &format!("• {penalty}"));
                        ui.add_space(12.0);
                    }
                    if !guide.penalty_note.is_empty() {
                        paragraph_faint(ui, inner_w, guide.penalty_note);
                    }
                } else if let Some(draft) = charge.breakdown() {
                    breakdown_paragraphs(ui, inner_w, draft.overview);
                    ui.add_space(16.0);
                    for (index, check) in draft.checks.iter().enumerate() {
                        paragraph(ui, inner_w, &format!("{}. {check}", index + 1));
                        ui.add_space(12.0);
                    }
                    breakdown_paragraphs(ui, inner_w, draft.limits);
                } else {
                    let body = format!("{}.", charge.plain.trim_end_matches('.'));
                    breakdown_paragraphs(ui, inner_w, &body);
                }

                ui.add_space(24.0);
                reading_heading(ui, rect, "The law");
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(PAD);
                    ui.add(egui::Hyperlink::from_label_and_url(
                        format!("Official {}", charge.act_name()),
                        charge.statute_url(),
                    ).open_in_new_tab(true));
                });
                ui.add_space(12.0);
                for (index, section) in charge.displayed_law_sections().iter().enumerate() {
                    if index > 0 { ui.add_space(20.0); }
                    statute_text(ui, inner_w, charge.act_code(), charge.act_name(), section);
                }
                ui.add_space(20.0);
            });

        if toggle && !self.load_failed {
            self.stuck[idx] = !self.stuck[idx];
            let ids: Vec<String> = CHARGES.iter().zip(&self.stuck)
                .filter_map(|(charge, is_saved)| is_saved.then(|| saved_id(charge)))
                .collect();
            let result = saved::storage_path()
                .ok_or_else(|| "Saved charges are unavailable on this device.".to_owned())
                .and_then(|path| saved::write_saved(&path, &ids)
                    .map_err(|_| "Could not save this charge. Please try again.".to_owned()));
            if let Err(message) = result {
                self.stuck[idx] = !self.stuck[idx];
                self.save_error = Some(message);
            } else {
                self.save_error = None;
            }
        }
    }

    // --- about --------------------------------------------------------------------

    #[allow(dead_code)]
    fn info(&mut self, ui: &mut Ui, rect: Rect) {
        let inner_w = rect.width() - PAD * 2.0;
        let max_h = (rect.bottom() - NAV_H - 14.0 - ui.cursor().top()).max(100.0);
        egui::ScrollArea::vertical().id_salt("about").max_height(max_h)
            .auto_shrink([false, false]).show(ui, |ui| {
        let (top, _) = ui.allocate_exact_size(vec2(rect.width(), 96.0), Sense::hover());
        let p = ui.painter();
        let disc = Rect::from_center_size(pos2(top.center().x, top.top() + 44.0), Vec2::splat(68.0));
        p.circle_filled(disc.center(), 34.0, theme::brand());
        for shape in leaf::leaf_shapes(Rect::from_center_size(disc.center(), Vec2::splat(44.0)), Color32::WHITE) {
            p.add(shape);
        }
        p.text(
            pos2(top.center().x, disc.bottom() + 18.0),
            Align2::CENTER_CENTER,
            "Stick the Charge",
            FontId::proportional(20.0),
            theme::ink(),
        );

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add_space(PAD);
            let label = if theme::is_dark() { "Dark mode: On" } else { "Dark mode: Off" };
            let button = egui::Button::new(RichText::new(label).color(theme::ink()))
                .fill(theme::surface())
                .stroke(Stroke::new(1.0, theme::line()))
                .corner_radius(CornerRadius::same(10))
                .min_size(vec2(inner_w, 44.0));
            if ui.add(button).clicked() { theme::toggle(ui.ctx()); }
        });
        ui.add_space(12.0);
        paragraph_center(
            ui,
            rect.width() - PAD * 2.0,
            "A quiet reference for Canadian criminal charges, sorted the way you need them.",
        );
        ui.add_space(22.0);

        for (k, v) in [
            ("Charges listed", &format!("{}", CHARGES.len()) as &str),
            ("Stuck to your board", &format!("{}", self.stuck.iter().filter(|s| **s).count()) as &str),
            ("Classification", "Summary · Hybrid · Indictable"),
        ] {
            let (row, _) = ui.allocate_exact_size(vec2(rect.width(), 44.0), Sense::hover());
            let p = ui.painter();
            p.rect_filled(
                Rect::from_min_size(pos2(row.left() + PAD, row.top()), vec2(inner_w, 44.0)),
                CornerRadius::same(11),
                theme::surface(),
            );
            p.text(
                pos2(row.left() + PAD + 14.0, row.center().y),
                Align2::LEFT_CENTER,
                k,
                FontId::proportional(if inner_w < 280.0 { 10.0 } else { 12.0 }),
                theme::ink_dim(),
            );
            p.text(
                pos2(row.right() - PAD - 14.0, row.center().y),
                Align2::RIGHT_CENTER,
                v,
                FontId::proportional(if inner_w < 280.0 { 10.0 } else { 12.0 }),
                theme::ink(),
            );
            ui.add_space(6.0);
        }

        ui.add_space(16.0);
        heading(ui, rect, "HOW IT WORKS");
        ui.add_space(6.0);
        paragraph_faint(
            ui,
            inner_w,
            "Scroll the list to see each sample charge and number. Tap a row for a summary and a link to the law. Tap Stick to save a charge to your board.",
        );

        ui.add_space(16.0);
        paragraph_center(
            ui,
            inner_w,
            "Built in Rust with egui · maple leaf drawn from an outline · sample content, not legal advice",
        );
        });
    }

    // --- bottom navigation ---------------------------------------------------------

    fn nav(&mut self, ui: &mut Ui, rect: Rect) {
        let bar = Rect::from_min_max(
            pos2(rect.left(), rect.bottom() - NAV_H),
            pos2(rect.right(), rect.bottom() - 12.0),
        );
        let p = ui.painter();
        p.rect_filled(
            bar,
            CornerRadius { nw: 16, ne: 16, sw: 0, se: 0 },
            theme::screen(),
        );
        p.line_segment(
            [pos2(rect.left(), bar.top()), pos2(rect.right(), bar.top())],
            Stroke::new(1.0, theme::line()),
        );

        let items = [(Tab::Home, "Home"), (Tab::Charges, "Criminal Code"), (Tab::Saved, "Saved")];
        let cw = bar.width() / items.len() as f32;
        for (i, (tab, label)) in items.iter().enumerate() {
            let item = Rect::from_min_size(
                pos2(bar.left() + cw * i as f32, bar.top()),
                vec2(cw, bar.height()),
            );
            let resp = ui.interact(item, ui.id().with(("nav", i)), Sense::click());
            let active = self.tab == *tab;
            let color = if active {
                theme::crimson()
            } else if resp.hovered() {
                theme::ink()
            } else {
                theme::ink_faint()
            };
            let p = ui.painter();
            let cx = item.center().x;
            let cy = item.top() + 19.0;
            match tab {
                Tab::Home => {
                    p.line_segment([pos2(cx - 7.5, cy + 1.0), pos2(cx, cy - 6.5)], Stroke::new(1.6, color));
                    p.line_segment([pos2(cx, cy - 6.5), pos2(cx + 7.5, cy + 1.0)], Stroke::new(1.6, color));
                    let house_body = Rect::from_min_max(pos2(cx - 5.5, cy + 0.5), pos2(cx + 5.5, cy + 8.0));
                    p.rect_stroke(house_body, CornerRadius::same(1), Stroke::new(1.5, color), StrokeKind::Inside);
                    let door = Rect::from_min_max(pos2(cx - 2.0, cy + 3.5), pos2(cx + 2.0, cy + 8.0));
                    p.rect_stroke(door, CornerRadius::same(1), Stroke::new(1.2, color), StrokeKind::Inside);
                }
                Tab::Charges => {
                    p.line_segment([pos2(cx, cy - 5.0), pos2(cx, cy + 6.0)], Stroke::new(1.5, color));
                    let left_pts = vec![
                        pos2(cx, cy - 5.0),
                        pos2(cx - 4.0, cy - 6.0),
                        pos2(cx - 7.5, cy - 4.5),
                        pos2(cx - 7.5, cy + 4.5),
                        pos2(cx - 4.0, cy + 4.0),
                        pos2(cx, cy + 6.0),
                    ];
                    p.add(egui::Shape::line(left_pts, Stroke::new(1.5, color)));
                    let right_pts = vec![
                        pos2(cx, cy - 5.0),
                        pos2(cx + 4.0, cy - 6.0),
                        pos2(cx + 7.5, cy - 4.5),
                        pos2(cx + 7.5, cy + 4.5),
                        pos2(cx + 4.0, cy + 4.0),
                        pos2(cx, cy + 6.0),
                    ];
                    p.add(egui::Shape::line(right_pts, Stroke::new(1.5, color)));
                }
                Tab::Saved => {
                    let bm_pts = vec![
                        pos2(cx - 5.5, cy - 6.5),
                        pos2(cx + 5.5, cy - 6.5),
                        pos2(cx + 5.5, cy + 6.5),
                        pos2(cx, cy + 3.0),
                        pos2(cx - 5.5, cy + 6.5),
                        pos2(cx - 5.5, cy - 6.5),
                    ];
                    p.add(egui::Shape::line(bm_pts, Stroke::new(1.5, color)));
                }
            }
            p.text(
                pos2(cx, cy + 15.0),
                Align2::CENTER_CENTER,
                label,
                FontId::proportional(9.5),
                color,
            );
            if resp.clicked() {
                if self.tab != *tab {
                    self.reset_list_scroll = true;
                }
                self.tab = *tab;
                self.open = None;
            }
        }

        ui.painter().rect_filled(
            Rect::from_center_size(pos2(rect.center().x, rect.bottom() - 8.0), vec2(134.0, 4.0)),
            CornerRadius::same(2),
            theme::ink_dim(),
        );
    }
}

// --- painting helpers ---------------------------------------------------------------

fn status_bar(ui: &mut Ui, rect: Rect) {
    let (bar, _) = ui.allocate_exact_size(vec2(rect.width(), 44.0), Sense::hover());
    let p = ui.painter();
    p.text(
        pos2(bar.left() + 30.0, bar.bottom() - 12.0),
        Align2::LEFT_CENTER,
        "9:41",
        FontId::proportional(12.0),
        theme::ink(),
    );
    for k in 0..4 {
        let h = 3.0 + k as f32 * 1.6;
        p.rect_filled(
            Rect::from_min_size(
                pos2(bar.right() - 96.0 + k as f32 * 4.0, bar.bottom() - 16.0 - h),
                vec2(2.6, h),
            ),
            CornerRadius::same(1),
            theme::ink(),
        );
    }
    let c = pos2(bar.right() - 70.0, bar.bottom() - 13.0);
    for k in 0..3 {
        let r = 2.5 + k as f32 * 3.0;
        let mut pts = Vec::new();
        for s in 0..10 {
            let a = std::f32::consts::PI * (1.22 + 0.56 * s as f32 / 9.0);
            pts.push(pos2(c.x + r * a.cos(), c.y - r * a.sin() * 0.85));
        }
        p.add(egui::Shape::line(pts, Stroke::new(1.2, theme::ink())));
    }
    let bat = Rect::from_min_size(pos2(bar.right() - 52.0, bar.bottom() - 18.0), vec2(20.0, 10.0));
    p.rect_stroke(bat, CornerRadius::same(3), Stroke::new(1.0, theme::ink_dim()), StrokeKind::Inside);
    p.rect_filled(
        Rect::from_min_size(bat.min + vec2(1.8, 1.8), vec2(12.0, bat.height() - 3.6)),
        CornerRadius::same(2),
        theme::ink(),
    );
    p.rect_filled(
        Rect::from_min_size(pos2(bat.right() + 1.2, bat.center().y - 2.0), vec2(2.0, 4.0)),
        CornerRadius::same(1),
        theme::ink_dim(),
    );

    p.rect_filled(
        Rect::from_center_size(pos2(bar.center().x, bar.top() + 1.0), vec2(92.0, 22.0)),
        CornerRadius::same(11),
        Color32::BLACK,
    );
}

fn charge_row(ui: &mut Ui, charge: &Charge, width: f32, stuck: bool) -> (Rect, Response) {
    // Measure the name first, so a two-line name gets a taller row instead of colliding
    // with the caption underneath.
    let inner_w = (width - PAD * 2.0).max(120.0);
    let name_left_off = 78.0;
    let name_w = (inner_w - 42.0 - name_left_off).max(60.0);
    let galley = ui.painter().layout(
        charge.name.to_string(),
        FontId::proportional(14.0),
        theme::ink(),
        name_w,
    );
    let name_h = galley.size().y;
    let height = (16.0 + name_h + 10.0 + 9.0 + 8.0).max(56.0);

    let (full, resp) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    let rect = Rect::from_min_max(full.min + vec2(PAD, 0.0), full.max - vec2(PAD, 0.0));

    let p = ui.painter();
    let bg = if resp.is_pointer_button_down_on() {
        theme::surface_press()
    } else if resp.hovered() {
        theme::surface_hover()
    } else {
        theme::surface()
    };
    p.rect_filled(rect, CornerRadius::same(13), bg);
    p.rect_stroke(rect, CornerRadius::same(13), Stroke::new(1.0, theme::line()), StrokeKind::Inside);

    let name_top = rect.top() + 14.0;
    p.text(
        pos2(rect.left() + 14.0, name_top + name_h / 2.0 - 1.0),
        Align2::LEFT_CENTER,
        charge.number,
        FontId::monospace(13.0),
        if stuck { theme::crimson() } else { theme::ink() },
    );

    p.galley(
        pos2(rect.left() + name_left_off, name_top),
        galley,
        theme::ink(),
    );
    p.text(
        pos2(rect.left() + name_left_off, rect.bottom() - 9.0),
        Align2::LEFT_BOTTOM,
        format!("{} · CANADIAN CRIMINAL CODE", charge.category.label().to_uppercase()),
        FontId::proportional(8.0),
        theme::ink_faint(),
    );

    if stuck {
        let mark_pos = pos2(rect.right() - 36.0, name_top + name_h / 2.0);
        let leaf_r = Rect::from_center_size(mark_pos, Vec2::splat(13.0));
        for s in leaf::flag_leaf_shapes(leaf_r, theme::crimson()) {
            p.add(s);
        }
    }

    let cx = rect.right() - 18.0;
    let cy = name_top + name_h / 2.0;
    let chev = if resp.hovered() { theme::ink() } else { theme::ink_faint() };
    p.line_segment([pos2(cx - 3.0, cy - 5.0), pos2(cx + 2.0, cy)], Stroke::new(1.5, chev));
    p.line_segment([pos2(cx + 2.0, cy), pos2(cx - 3.0, cy + 5.0)], Stroke::new(1.5, chev));

    (rect, resp)
}

#[allow(dead_code)]
fn heading(ui: &mut Ui, rect: Rect, label: &str) {
    let (r, _) = ui.allocate_exact_size(vec2(rect.width(), 11.0), Sense::hover());
    ui.painter().text(
        pos2(r.left() + PAD, r.top()),
        Align2::LEFT_TOP,
        label,
        FontId::proportional(8.5),
        theme::ink_faint(),
    );
}

fn reading_heading(ui: &mut Ui, rect: Rect, label: &str) {
    let (r, _) = ui.allocate_exact_size(vec2(rect.width(), 15.0), Sense::hover());
    ui.painter().text(
        pos2(r.left() + PAD, r.top()),
        Align2::LEFT_TOP,
        label,
        FontId::proportional(12.0),
        theme::ink(),
    );
}

fn name_block(ui: &mut Ui, width: f32, text: &str) {
    let size = 24.0;
    let gal = ui
        .painter()
        .layout(text.to_string(), FontId::proportional(size), theme::ink(), width);
    let h = gal.size().y.max(size);
    let (r, _) = ui.allocate_exact_size(vec2(width, h), Sense::hover());
    let x = r.left() + PAD;
    ui.painter().galley(pos2(x, r.top()), gal, theme::ink());
}

fn paragraph(ui: &mut Ui, width: f32, text: &str) {
    let gal = ui
        .painter()
        .layout(text.to_string(), FontId::proportional(14.0), theme::ink_dim(), width);
    let h = gal.size().y.max(14.0);
    let (r, _) = ui.allocate_exact_size(vec2(width, h), Sense::hover());
    ui.painter().galley(pos2(r.left() + PAD, r.top()), gal, theme::ink_dim());
}

fn breakdown_paragraphs(ui: &mut Ui, width: f32, text: &str) {
    let wording = text
        .replace("This provision concerns", "This section is about")
        .replace("This offence concerns", "This offence is about");
    for (index, sentence) in wording.split_inclusive(". ").enumerate() {
        if index > 0 {
            ui.add_space(10.0);
        }
        paragraph(ui, width, sentence.trim());
    }
}

fn paragraph_faint(ui: &mut Ui, width: f32, text: &str) {
    let gal = ui
        .painter()
        .layout(text.to_string(), FontId::proportional(12.0), theme::ink_faint(), width);
    let h = gal.size().y.max(12.0);
    let (r, _) = ui.allocate_exact_size(vec2(width, h), Sense::hover());
    ui.painter().galley(pos2(r.left() + PAD, r.top()), gal, theme::ink_faint());
}

fn statute_text(ui: &mut Ui, width: f32, act: &str, act_name: &str, section: &str) {
    match statutes::lookup(act, section) {
        Some(law) => {
            paragraph_faint(ui, width, &format!("{act_name} · full section {section}\nUnofficial reproduction · current to {}", law.current_to));
            ui.add_space(8.0);
            for line in law.text.lines() {
                paragraph(ui, width, line);
                ui.add_space(8.0);
            }
        }
        None => paragraph_faint(ui, width, "The statutory text is unavailable here. Open the official source above."),
    }
}

#[allow(dead_code)]
fn paragraph_center(ui: &mut Ui, width: f32, text: &str) {
    let gal = ui
        .painter()
        .layout(text.to_string(), FontId::proportional(12.0), theme::ink_dim(), width - PAD * 2.0);
    let h = gal.size().y.max(12.0);
    let (r, _) = ui.allocate_exact_size(vec2(width, h), Sense::hover());
    ui.painter().galley(
        pos2(r.center().x - gal.size().x / 2.0, r.top()),
        gal,
        theme::ink_dim(),
    );
}

fn paint_chip(p: &Painter, rect: Rect, selected: bool, hovered: bool, label: &str) {
    let (bg, fg) = if selected {
        (theme::ink(), theme::screen())
    } else if hovered {
        (theme::surface_hover(), theme::ink())
    } else {
        (theme::surface(), theme::ink_dim())
    };
    p.rect_filled(rect, CornerRadius::same(14), bg);
    if !selected {
        p.rect_stroke(rect, CornerRadius::same(14), Stroke::new(1.0, theme::line()), StrokeKind::Inside);
    }
    p.text(rect.center(), Align2::CENTER_CENTER, label, FontId::proportional(12.0), fg);
}

fn number_key(charge: &Charge) -> (u32, u32, u32) {
    let mut parts = charge.number.split(['.', '(', ')']);
    let a = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(9999);
    let b = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    let c = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    (a, b, c)
}
