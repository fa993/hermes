use std::{io::Write, path::PathBuf};

use handlebars::Handlebars;
use serde::Serialize;
use tempfile::NamedTempFile;

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

    fn render_to_file<T: Serialize>(&self, template: &str, configs: &T) -> anyhow::Result<PathBuf> {
        let file_content = self.registry.render_template(template, configs)?;
        let mut temp_starter = NamedTempFile::new()?;
        temp_starter.write_all(file_content.as_bytes())?;
        Ok(temp_starter.into_temp_path().to_path_buf())
    }

    pub fn create_starter<T: Serialize>(&self, configs: &T) -> anyhow::Result<PathBuf> {
        self.render_to_file("starter", configs)
    }

    pub fn create_nginx<T: Serialize>(&self, configs: &T) -> anyhow::Result<PathBuf> {
        self.render_to_file("nginx", configs)
    }

    pub fn create_systemd<T: Serialize>(&self, configs: &T) -> anyhow::Result<PathBuf> {
        self.render_to_file("sysdsrv", configs)
    }
}
