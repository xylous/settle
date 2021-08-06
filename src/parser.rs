use rayon::prelude::*;

pub enum Block
{
    Paragraph(String),
}

pub fn parse(md: &str) -> Vec<Block>
{
    let elems: Vec<String> = elements(md);

    elems.into_par_iter()
        .map(|e| {
            Block::Paragraph(e)
        })
        .collect()
}

fn elements(md: &str) -> Vec<String>
{
    md.split("\n\n")
        .map(|s|
            s.to_string()
        )
        .collect()
}
