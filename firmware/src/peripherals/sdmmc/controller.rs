use crate::error::AppError;

use super::{SdMmcController, SdMmcDirectory, SdMmcFile, SdMmcVolume};

pub struct Controller<'a> {
    ctl: SdMmcController<'a>,
    vol: SdMmcVolume,
    dir: SdMmcDirectory,
}

impl<'a> Controller<'a> {
    pub fn new(ctl: SdMmcController<'a>, vol: SdMmcVolume, dir: SdMmcDirectory) -> Self {
        Self { ctl, vol, dir }
    }

    pub fn close(mut self) {
        self.ctl.close_dir(&self.vol, self.dir);
    }

    pub fn is_file_exists(&mut self, name: &str) -> Result<bool, AppError> {
        match self.ctl.find_directory_entry(&self.vol, &self.dir, name) {
            Ok(_) => Ok(true),
            Err(embedded_sdmmc::Error::FileNotFound) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }

    pub fn open_file_read(&mut self, name: &str) -> Result<SdMmcFile, AppError> {
        let file = self.ctl.open_file_in_dir(
            &mut self.vol,
            &self.dir,
            name,
            embedded_sdmmc::Mode::ReadOnly,
        )?;

        Ok(file)
    }

    pub fn oped_file_append(&mut self, name: &str) -> Result<SdMmcFile, AppError> {
        let file = self.ctl.open_file_in_dir(
            &mut self.vol,
            &self.dir,
            name,
            embedded_sdmmc::Mode::ReadWriteCreateOrAppend,
        )?;

        Ok(file)
    }

    pub fn close_file(&mut self, file: SdMmcFile) -> Result<(), AppError> {
        self.ctl.close_file(&self.vol, file)?;
        Ok(())
    }

    pub fn delete_file(&mut self, name: &str) -> Result<bool, AppError> {
        match self.ctl.delete_file_in_dir(&self.vol, &self.dir, name) {
            Ok(_) => Ok(true),
            Err(embedded_sdmmc::Error::FileNotFound) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }

    pub fn copy_file(&mut self, src: &str, dst: &str) -> Result<bool, AppError> {
        let mut src_file = self.ctl.open_file_in_dir(
            &mut self.vol,
            &self.dir,
            src,
            embedded_sdmmc::Mode::ReadOnly,
        )?;
        let mut dst_file = self.ctl.open_file_in_dir(
            &mut self.vol,
            &self.dir,
            dst,
            embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
        )?;
        let mut buf = [0; 64];
        loop {
            match self.ctl.read(&self.vol, &mut src_file, &mut buf) {
                Ok(size) => {
                    if size == 0 {
                        self.ctl.close_file(&self.vol, dst_file)?;
                        self.ctl.close_file(&self.vol, src_file)?;
                        return Ok(true);
                    } else {
                        self.ctl.write(&mut self.vol, &mut dst_file, &buf[..size])?;
                    }
                }
                Err(embedded_sdmmc::Error::EndOfFile) => {
                    self.ctl.close_file(&self.vol, dst_file)?;
                    self.ctl.close_file(&self.vol, src_file)?;
                    return Ok(true);
                }
                Err(err) => {
                    self.ctl.close_file(&self.vol, dst_file)?;
                    self.ctl.close_file(&self.vol, src_file)?;
                    return Err(err.into());
                }
            };
        }
    }

    pub fn read(&mut self, file: &mut SdMmcFile, buf: &mut [u8]) -> Result<usize, AppError> {
        let size = self.ctl.read(&self.vol, file, buf)?;
        Ok(size)
    }

    pub fn write(&mut self, file: &mut SdMmcFile, buf: &[u8]) -> Result<usize, AppError> {
        let size = self.ctl.write(&mut self.vol, file, buf)?;
        Ok(size)
    }
}
