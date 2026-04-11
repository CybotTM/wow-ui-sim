use wow_ui_sim::widget::WidgetRegistry;

pub struct WidgetRegistrySnapshot {
    pub frame_count: usize,
    pub storage_estimate_bytes: usize,
}

pub fn snapshot_widget_registry(widgets: &WidgetRegistry) -> WidgetRegistrySnapshot {
    WidgetRegistrySnapshot {
        frame_count: widgets.iter_ids().count(),
        storage_estimate_bytes: widgets.storage_estimate_bytes(),
    }
}
