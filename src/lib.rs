#[derive(Debug, Clone, Copy)]
pub struct File<'a> {
    pub chapter: &'a str,
    pub description: &'a str,
    pub listing: &'a str,
}

pub fn get_files() -> &'static [File<'static>] {
    &[
        File {
            chapter: "1",
            description: "test",
            listing: "1-1",
        },
        File {
            chapter: "2",
            description: "another test",
            listing: "2-1",
        },
    ]
}
