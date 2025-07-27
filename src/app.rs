use chrono::NaiveDate;
use chrono::Local;
use eframe::egui::{self, RichText, Grid};
use egui_extras::DatePickerButton;

use crate::task::Task;
use crate::sorting::{SortField, SortOrder, sort_tasks};
use crate::storage::{load_tasks_from_file, save_to_text_file};

pub struct MyApp {
    pub tasks: Vec<Task>,
    pub new_description: String,
    pub selected_date: NaiveDate,
    pub new_reminder: bool,
    pub error_message: Option<String>,
    pub sort_field: SortField,
    pub sort_order: SortOrder,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut app = Self {
            tasks: vec![],
            new_description: String::new(),
            selected_date: chrono::Local::now().naive_local().date(),
            new_reminder: false,
            error_message: None,
            sort_field: SortField::Deadline,
            sort_order: SortOrder::Ascending,
        };

        match load_tasks_from_file("C:/RUST_EXERCISE/todo_gui/target/debug/todo_tasks.txt") {
            Ok(tasks) => app.tasks = tasks,
            Err(e) => app.error_message = Some(format!("Failed to load tasks: {}", e)),
        }

        app
    }
}

impl MyApp {
    fn toggle_sort(&mut self, field: SortField) {
        if self.sort_field == field {
            self.sort_order = match self.sort_order {
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::Ascending,
            };
        } else {
            self.sort_field = field;
            self.sort_order = SortOrder::Ascending;
        }
        sort_tasks(&mut self.tasks, &self.sort_field, &self.sort_order);
    }

    fn auto_save(&mut self) {
        if let Err(e) = save_to_text_file("C:/RUST_EXERCISE/todo_gui/target/debug/todo_tasks.txt", &self.tasks) {
            self.error_message = Some(format!("Auto-save failed: {}", e));
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("-->TASK TRACKER<--");

            if let Some(err) = &self.error_message {
                ui.colored_label(egui::Color32::RED, err);
            }

            ui.horizontal(|ui| {
                ui.label("Task:");
                ui.text_edit_singleline(&mut self.new_description);
            });

    
            ui.horizontal(|ui| {
                ui.label("Deadline:");
                if ui.add(DatePickerButton::new(&mut self.selected_date)).clicked() {
                 let today = Local::now().naive_local().date();
                    if self.selected_date < today {
                        self.error_message = Some("Please select a future date.".to_string());
                        self.selected_date = today;
                    }
                }
            });


            ui.checkbox(&mut self.new_reminder, "Reminder");

            if ui.button("Add Task").clicked() {
                if self.new_description.trim().is_empty() {
                    self.error_message = Some("Task description cannot be empty.".to_string());
                } else {
                    self.tasks.push(Task {
                        description: self.new_description.clone(),
                        status: "Not Completed".to_string(),
                        deadline: self.selected_date,
                        reminder: if self.new_reminder { "Yes" } else { "No" }.to_string(),
                    });
                    self.new_description.clear();
                    self.new_reminder = false;
                    self.error_message = None;
                    sort_tasks(&mut self.tasks, &self.sort_field, &self.sort_order);
                    self.auto_save();
                }
            }

            ui.separator();

            let sort_field = self.sort_field;
            let sort_order = self.sort_order;

            let arrow = |field: &SortField| {
                if sort_field == *field {
                    match sort_order {
                        SortOrder::Ascending => " ↑",
                        SortOrder::Descending => " ↓",
                    }
                } else {
                    ""
                }
            };

            ui.horizontal(|ui| {
                ui.label("Sort by:");
                if ui.button(RichText::new(format!("Task{}", arrow(&SortField::Description))).strong()).clicked() {
                    self.toggle_sort(SortField::Description);
                }
                if ui.button(RichText::new(format!("Status{}", arrow(&SortField::Status))).strong()).clicked() {
                    self.toggle_sort(SortField::Status);
                }
                if ui.button(RichText::new(format!("Deadline{}", arrow(&SortField::Deadline))).strong()).clicked() {
                    self.toggle_sort(SortField::Deadline);
                }
                if ui.button(RichText::new(format!("Reminder{}", arrow(&SortField::Reminder))).strong()).clicked() {
                    self.toggle_sort(SortField::Reminder);
                }
            });

            ui.separator();
            ui.label("Tasks:");

            Grid::new("task_table")
                .striped(true)
                .spacing([10.0, 4.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Task").strong());
                    ui.label(RichText::new("Status").strong());
                    ui.label(RichText::new("Deadline").strong());
                    ui.label(RichText::new("Reminder").strong());
                    ui.label(""); 
                    ui.end_row();

                    let mut to_delete: Option<usize> = None;
                    let mut needs_save = false;

                    for (i, task) in self.tasks.iter_mut().enumerate() {
                        ui.label(&task.description);

                        if ui.button(&task.status).clicked() {
                            task.status = if task.status == "Completed" {
                                "Not Completed".to_string()
                            } else {
                                "Completed".to_string()
                            };
                            needs_save = true;
                        }

                        ui.label(task.deadline.to_string());

                        if ui.button(&task.reminder).clicked() {
                            task.reminder = if task.reminder == "Yes" {
                                "No".to_string()
                            } else {
                                "Yes".to_string()
                            };
                            needs_save = true;
                        }

                        if ui.button("Remove").clicked() {
                            to_delete = Some(i);
                        }

                        ui.end_row();
                    }

                    if let Some(index) = to_delete {
                        self.tasks.remove(index);
                        needs_save = true;
                    }

                    if needs_save {
                        self.auto_save();
                    }
                });

            ui.separator();

            if ui.button("->Export<-").clicked() {
                match save_to_text_file("todo_tasks.txt", &self.tasks) {
                    Ok(_) => self.error_message = None,
                    Err(e) => self.error_message = Some(format!("Failed to save: {}", e)),
                }
            }
        });
    }
}



