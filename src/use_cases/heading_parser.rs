use crate::domain::toc_entry::TocEntry;
use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};
use std::collections::{HashMap, HashSet};

pub fn parse_and_rewrite_headings(raw_events: Vec<Event>) -> (Vec<Event>, Vec<TocEntry>) {
    let mut headings_data = Vec::new();
    let mut current_heading_idx = None;

    for (index, event) in raw_events.iter().enumerate() {
        match event {
            Event::Start(Tag::Heading { level, id, .. }) => {
                current_heading_idx = Some((
                    index,
                    *level,
                    id.as_ref().map(|heading_id| heading_id.to_string()),
                    String::new(),
                ));
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(data) = current_heading_idx.take() {
                    headings_data.push(data);
                }
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some((_, _, _, ref mut text_accumulator)) = current_heading_idx {
                    text_accumulator.push_str(text);
                }
            }
            _ => {}
        }
    }

    let mut toc = Vec::new();
    let mut seen_slugs = HashSet::new();
    let mut heading_id_mapping = HashMap::new();

    for (event_idx, level, existing_id, text) in headings_data {
        let level_num = match level {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        };

        let base_slug = match existing_id {
            Some(ref id) if !id.is_empty() => id.clone(),
            _ => {
                let slugified = slug::slugify(text.trim());
                if slugified.is_empty() {
                    format!("heading-{}", event_idx)
                } else {
                    slugified
                }
            }
        };

        let mut slug = base_slug.clone();
        let mut counter = 1;
        while seen_slugs.contains(&slug) {
            slug = format!("{}-{}", base_slug, counter);
            counter += 1;
        }
        seen_slugs.insert(slug.clone());

        toc.push(TocEntry {
            level: level_num,
            title: text.trim().to_string(),
            slug: slug.clone(),
        });

        heading_id_mapping.insert(event_idx, slug);
    }

    let mut new_events = Vec::new();
    for (index, event) in raw_events.into_iter().enumerate() {
        if let Event::Start(Tag::Heading {
            level,
            classes,
            attrs,
            ..
        }) = event
        {
            let assigned_id = heading_id_mapping.get(&index).cloned();
            let cow_id = assigned_id.map(pulldown_cmark::CowStr::from);
            new_events.push(Event::Start(Tag::Heading {
                level,
                id: cow_id,
                classes,
                attrs,
            }));
        } else {
            new_events.push(event);
        }
    }

    (new_events, toc)
}
