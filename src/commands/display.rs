use crate::bind::bind::Bind;

pub fn display_comments(binds: &Vec<Bind>) {
    if binds.is_empty() {
        return;
    }

    for bind in binds {
        if let Some(comment) = &bind.comment {
            println!("{}", comment);
        }
    }
}
