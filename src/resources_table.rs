use iced::{
    widget::{
        container,
        scrollable::{self, AbsoluteOffset},
        text,
    },
    Element, Renderer, Task, Theme,
};
use iced_table::table;

use crate::{message::Message, workspace::Project};

pub struct ResourcesTable {
    header_id: iced::widget::scrollable::Id,
    body_id: iced::widget::scrollable::Id,
    footer_id: iced::widget::scrollable::Id,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl ResourcesTable {
    pub fn new() -> Self {
        Self {
            header_id: iced::widget::scrollable::Id::unique(),
            body_id: iced::widget::scrollable::Id::unique(),
            footer_id: iced::widget::scrollable::Id::unique(),
            columns: vec![
                Column::new(ColumnKind::Name),
                Column::new(ColumnKind::Profile),
                Column::new(ColumnKind::Region),
                Column::new(ColumnKind::Service),
            ],
            rows: vec![],
        }
    }

    pub fn set_selected_project(&mut self, project: Option<Project>) {
        let Some(project) = project else {
            self.rows = vec![];
            return;
        };

        self.rows = project
            .resources
            .iter()
            .map(|r| Row {
                profile: r.profile.clone(),
                region: r.region.clone(),
                service: r.service.to_string(),
                name: r.get_display_name(),
            })
            .collect();
    }

    pub fn sync_header_offset(&self, offset: AbsoluteOffset) -> Task<Message> {
        Task::batch(vec![
            scrollable::scroll_to(self.header_id.clone(), offset),
            scrollable::scroll_to(self.footer_id.clone(), offset),
        ])
    }

    pub fn view(&self) -> Element<Message> {
        table(
            self.header_id.clone(),
            self.body_id.clone(),
            &self.columns,
            &self.rows,
            Message::SyncResourcesTableHeader,
        )
        .into()
    }
}

struct Column {
    kind: ColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}

impl Column {
    fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Name => 320.0,
            ColumnKind::Profile => 120.0,
            ColumnKind::Region => 180.0,
            ColumnKind::Service => 100.0,
        };
        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

enum ColumnKind {
    Name,
    Profile,
    Region,
    Service,
}

impl<'a> iced_table::table::Column<'a, Message, Theme, Renderer> for Column {
    type Row = Row;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Name => "Name",
            ColumnKind::Profile => "Profile",
            ColumnKind::Region => "Region",
            ColumnKind::Service => "Service",
        };

        container(text(content)).center_y(24).into()
    }

    fn cell(&'a self, _col_index: usize, _row_index: usize, row: &'a Row) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Name => text(row.name.clone()),
            ColumnKind::Profile => text(row.profile.clone()),
            ColumnKind::Region => text(row.region.clone()),
            ColumnKind::Service => text(row.service.clone()),
        };
        container(content).center_y(24).into()
    }

    fn footer(&'a self, _col_index: usize, _rows: &'a [Row]) -> Option<Element<'a, Message>> {
        Some(container(text("")).center_y(24).into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}

struct Row {
    name: String,
    profile: String,
    region: String,
    service: String,
}
