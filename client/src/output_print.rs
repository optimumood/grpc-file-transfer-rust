use comfy_table::{presets::NOTHING, Cell, Table};
use proto::api::ListFilesResponse;
use std::fmt;
use ubyte::{ByteUnit, ToByteUnit};

struct FileOutputPrint {
    name: String,
    size: ByteUnit,
}

impl FileOutputPrint {
    pub fn new(name: &str, size: u64) -> Self {
        FileOutputPrint {
            name: name.to_string(),
            size: size.bytes(),
        }
    }
}

pub struct FilesOutputPrint {
    files: Vec<FileOutputPrint>,
}

impl From<ListFilesResponse> for FileOutputPrint {
    fn from(file_resp: ListFilesResponse) -> Self {
        FileOutputPrint::new(&file_resp.name, file_resp.size)
    }
}

impl From<Vec<FileOutputPrint>> for FilesOutputPrint {
    fn from(files: Vec<FileOutputPrint>) -> Self {
        FilesOutputPrint { files }
    }
}

impl From<Vec<ListFilesResponse>> for FilesOutputPrint {
    fn from(file_resps: Vec<ListFilesResponse>) -> Self {
        file_resps
            .into_iter()
            .map(|file| file.into())
            .collect::<Vec<FileOutputPrint>>()
            .into()
    }
}

impl fmt::Display for FilesOutputPrint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut table = Table::new();
        table
            .set_header(vec!["File name", "Size"])
            .load_preset(NOTHING);

        for file in &self.files {
            table.add_row(vec![Cell::new(file.name.clone()), Cell::new(file.size)]);
        }

        write!(f, "{table}")
    }
}
