use std::{fs::File, io::Write};

use handlebars::Handlebars;
use serde::Serialize;
use tempfile::tempfile;

pub struct ConfigFileBuilder<'a> {
    registry: Handlebars<'a>,
}

impl<'a> ConfigFileBuilder<'a> {
    pub fn new<'b>() -> anyhow::Result<ConfigFileBuilder<'b>> {
        let mut registry = Handlebars::new();
        registry.register_template_file("starter", "../templates/startup.template")?;
        registry.register_template_file("nginx", "../templates/nginx.template")?;
        registry.register_template_file("sysdsrv", "../templates/sysdsrv.template")?;
        Ok(ConfigFileBuilder { registry })
    }

    fn render_to_file<T: Serialize>(&self, template: &str, configs: &T) -> anyhow::Result<File> {
        let file_content = self.registry.render_template(template, configs)?;
        let mut temp_starter = tempfile()?;
        temp_starter.write_all(file_content.as_bytes())?;
        Ok(temp_starter)
    }

    pub fn create_starter<T: Serialize>(&self, configs: &T) -> anyhow::Result<File> {
        self.render_to_file("starter", configs)
    }

    pub fn create_nginx<T: Serialize>(&self, configs: &T) -> anyhow::Result<File> {
        self.render_to_file("nginx", configs)
    }

    pub fn create_systemd<T: Serialize>(&self, configs: &T) -> anyhow::Result<File> {
        self.render_to_file("sysdsrv", configs)
    }
}
