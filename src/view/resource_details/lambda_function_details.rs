use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, row, scrollable, text, text_input},
    Element, Length,
};

use crate::{message::Message, models::resource::LambdaFunctionInfo, view::fonts};

pub struct LambdaFunctionDetails {}

impl LambdaFunctionDetails {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, f: &LambdaFunctionInfo) -> iced::Element<Message> {
        scrollable(
            column![
                self.string_prop("Name", &f.0.function_name),
                self.string_prop("Description", &f.0.description),
                self.string_prop("ARN", &f.0.function_arn),
                self.string_prop("Role", &f.0.role),
                row![
                    self.string_prop("Handler", &f.0.handler),
                    self.string_prop("Runtime", &f.0.runtime.as_ref().map(|r| r.to_string())),
                    self.string_prop(
                        "Architectures",
                        &f.0.architectures.as_ref().map(|a| a
                            .iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<_>>()
                            .join(", "))
                    ),
                    self.string_prop(
                        "Package type",
                        &f.0.package_type.as_ref().map(|p| p.to_string())
                    ),
                ]
                .spacing(8),
                row![
                    self.num_prop("Code size", &Some(f.0.code_size), Some("bytes")),
                    self.num_prop("Timeout", &f.0.timeout, Some("second(s)")),
                    self.num_prop("Memory size", &f.0.memory_size, Some("MB")),
                    self.num_prop(
                        "Ephemeral storage",
                        &f.0.ephemeral_storage.as_ref().map(|s| s.size),
                        Some("MB"),
                    ),
                ]
                .spacing(8),
                self.string_prop("Last modified", &f.0.last_modified),
                self.string_prop("Code SHA256", &f.0.code_sha256),
                self.string_prop("Version", &f.0.version),
                self.render_vpc_config(f),
                self.string_prop("KMS Key ARN", &f.0.kms_key_arn),
                self.string_prop("Master ARN", &f.0.master_arn),
                self.string_prop("Revision ID", &f.0.revision_id),
                //self.string_prop("State", &f.0.state.as_ref().map(|s| s.to_string())),
                //self.string_prop("State reason", &f.0.state_reason),
                //self.string_prop(
                //    "State reason code",
                //    &f.0.state_reason_code.as_ref().map(|c| c.to_string())
                //),
                //self.string_prop(
                //    "Last update status",
                //    &f.0.last_update_status.as_ref().map(|s| s.to_string())
                //),
                //self.string_prop("Last update status reason", &f.0.last_update_status_reason),
                //self.string_prop(
                //    "Last update status reason code",
                //    &f.0.last_update_status_reason_code
                //        .as_ref()
                //        .map(|c| c.to_string())
                //),
                //self.string_prop(
                //    "Signing profile version ARN",
                //    &f.0.signing_profile_version_arn
                //),
                //self.string_prop("Signing job ARN", &f.0.signing_job_arn),
                self.render_environment_variables(f),
            ]
            .padding(4)
            .spacing(8),
        )
        .into()
    }

    fn render_vpc_config(&self, f: &LambdaFunctionInfo) -> Element<Message> {
        let Some(vpc_config) = f.0.vpc_config.as_ref() else {
            return column![
                row![
                    self.string_prop("VPC ID", &None),
                    self.string_prop("Subnet IDs", &None),
                ]
                .spacing(8),
                row![
                    self.string_prop("Security group IDs", &None),
                    self.bool_prop("IPv6 allowed for dual stack", &None),
                ]
                .spacing(8)
            ]
            .spacing(8)
            .into();
        };

        let vpc_id = &vpc_config.vpc_id;
        let subnet_ids = self.join_option_vec_string(&vpc_config.subnet_ids);
        let security_group_ids = self.join_option_vec_string(&vpc_config.security_group_ids);

        column![
            row![
                self.string_prop("VPC ID", vpc_id),
                self.string_prop("Subnet IDs", &subnet_ids),
            ]
            .spacing(8),
            row![
                self.string_prop("Security group IDs", &security_group_ids),
                self.bool_prop(
                    "IPv6 allowed for dual stack",
                    &vpc_config.ipv6_allowed_for_dual_stack
                ),
            ]
            .spacing(8)
        ]
        .spacing(8)
        .into()
    }

    fn render_environment_variables(&self, f: &LambdaFunctionInfo) -> Element<Message> {
        let mut c = column![self.render_label("Environment variables: ")];

        let Some(env) = f.0.environment.as_ref() else {
            return c.into();
        };

        let Some(env_vars) = env.variables.as_ref() else {
            return c.into();
        };

        for (k, v) in env_vars {
            c = c.push(
                row![
                    text_input("", k).on_input(do_nothing),
                    text_input("", v).on_input(do_nothing),
                ]
                .spacing(4),
            )
        }

        c.into()
    }

    fn join_option_vec_string(&self, v: &Option<Vec<String>>) -> Option<String> {
        v.as_ref().map(|v| v.join(", "))
    }

    fn string_prop(&self, label: &str, value: &Option<String>) -> Element<Message> {
        let value = match value {
            Some(v) => text_input("", v).width(Length::Fill).on_input(do_nothing),
            None => text_input("-", "").font(fonts::get_default_font()).size(12),
        };

        column![self.render_label(label), value].into()
    }

    fn num_prop(
        &self,
        label: &str,
        value: &Option<impl num_traits::Num + std::fmt::Display + std::default::Default>,
        unit: Option<&str>,
    ) -> Element<Message> {
        let value = match value {
            Some(v) => text_input("", &v.to_string())
                .on_input(do_nothing)
                .align_x(Horizontal::Right),
            None => text_input("", "-"),
        };

        let unit = text(unit.unwrap_or("").to_string())
            .height(Length::Fixed(32.0))
            .align_y(iced::alignment::Vertical::Center);

        column![self.render_label(label), row![value, unit].spacing(4)]
            .spacing(4)
            .into()
    }

    fn bool_prop(&self, label: &str, value: &Option<bool>) -> Element<Message> {
        let value = match value {
            Some(v) => text_input("", if *v { "true" } else { "false" }).on_input(do_nothing),
            None => text_input("-", ""),
        };
        column![self.render_label(label), value.width(Length::Fill)]
            .spacing(4)
            .into()
    }

    fn render_label(&self, label: &str) -> Element<Message> {
        text(label.to_string())
            .font(fonts::get_bold_font())
            .width(Length::Fill)
            .size(10.0)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Center)
            .into()
    }
}

fn do_nothing(_: String) -> Message {
    Message::DoNothing
}
