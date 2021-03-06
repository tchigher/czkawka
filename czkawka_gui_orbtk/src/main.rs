use orbtk::prelude::*;
fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("Czkawka - Orbtk backend")
                .position((100.0, 100.0))
                .size(600.0, 500.0)
                .resizeable(true)
                .child(
                    TabWidget::new()
                        .close_button(false)
                        .tab(
                            "Duplicate Finder",
                            Grid::new()
                                .columns(Columns::create().push(105.0).push(67.0))
                                .rows(Rows::create().push(50.0).push(32.0).push(32.0).push(32.0).push(32.0))
                                .margin((84, 40))
                                .child(TextBlock::new().text("Included Directory:").v_align("center").h_align("start").attach(Grid::column(0)).attach(Grid::row(0)).build(ctx))
                                .child(
                                    TextBox::new()
                                        .id("included_directory")
                                        .water_mark("Included Directory")
                                        .v_align("center")
                                        .h_align("start")
                                        .attach(Grid::column(1))
                                        .attach(Grid::row(0))
                                        .min_width(300.0)
                                        .build(ctx),
                                )
                                .child(TextBlock::new().text("Excluded Directory:").v_align("center").h_align("start").attach(Grid::column(0)).attach(Grid::row(1)).build(ctx))
                                .child(
                                    TextBox::new()
                                        .id("excluded_directory")
                                        .water_mark("Excluded Directory")
                                        .v_align("center")
                                        .h_align("start")
                                        .attach(Grid::column(1))
                                        .attach(Grid::row(1))
                                        .min_width(300.0)
                                        .build(ctx),
                                )
                                .child(TextBlock::new().text("Info:\n\n rr").v_align("center").h_align("start").attach(Grid::column(0)).attach(Grid::row(2)).build(ctx))
                                .build(ctx),
                        )
                        .close_button(false)
                        .tab("Empty Folders", TextBlock::new().text("Empty Folders").build(ctx))
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
