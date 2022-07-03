/* Copyright (C) 2022 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::event::DateRange;
use crate::Event;
use chrono::DateTime;
use chrono::Utc;
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParsePageError {
    #[error("the requested event is non-existing")]
    NonExistingEvent,
    #[error("parsed html is not following the expected format")]
    UnexpectedHtml,
    #[error("parsed html has a date in an unexpected format")]
    DateParseError,
}

pub fn parse_page(text: &str) -> Result<Event, ParsePageError> {
    let document = select::document::Document::from(text);

    let page_id = parse_page_id(&document)?;
    if page_id == "page_home_index" {
        return Err(ParsePageError::NonExistingEvent);
    }

    Ok(Event {
        canonical_url: parse_canonical_url(&document)?,
        title: parse_page_title(&document)?,
        final_date: parse_page_final_date(&document)?,
        open_registration_link: parse_page_open_registration_link(&document)?,
    })
}

fn parse_page_id(document: &select::document::Document) -> Result<String, ParsePageError> {
    Ok(document
        .find(select::predicate::Name("html"))
        .next()
        .ok_or(ParsePageError::UnexpectedHtml)?
        .attr("id")
        .ok_or(ParsePageError::UnexpectedHtml)?
        .to_string())
}

fn parse_canonical_url(document: &select::document::Document) -> Result<String, ParsePageError> {
    Ok(document
        .find(select::predicate::And(
            select::predicate::Name("link"),
            select::predicate::Attr("rel", "canonical"),
        ))
        .next()
        .ok_or(ParsePageError::UnexpectedHtml)?
        .attr("href")
        .ok_or(ParsePageError::UnexpectedHtml)?
        .to_string())
}

fn parse_page_title(document: &select::document::Document) -> Result<String, ParsePageError> {
    Ok(document
        .find(select::predicate::Name("article"))
        .next()
        .ok_or(ParsePageError::UnexpectedHtml)?
        .attr("data-event-title")
        .ok_or(ParsePageError::UnexpectedHtml)?
        .to_string())
}

fn parse_page_final_date(
    document: &select::document::Document,
) -> Result<Option<DateRange>, ParsePageError> {
    if let Some(final_summary) = document
        .find(select::predicate::Attr("id", "final_summary"))
        .next()
    {
        let final_date = final_summary
            .find(select::predicate::Class("date"))
            .next()
            .ok_or(ParsePageError::UnexpectedHtml)?;

        let start_text = final_date
            .attr("data-startdate")
            .ok_or(ParsePageError::UnexpectedHtml)?;
        let end_text = final_date
            .attr("data-enddate")
            .ok_or(ParsePageError::UnexpectedHtml)?;

        Ok(Some(DateRange {
            start: DateTime::parse_from_rfc3339(start_text)
                .map_err(|_err| ParsePageError::DateParseError)?
                .with_timezone(&Utc),
            end: DateTime::parse_from_rfc3339(end_text)
                .map_err(|_err| ParsePageError::DateParseError)?
                .with_timezone(&Utc),
        }))
    } else {
        Ok(None)
    }
}

fn parse_page_open_registration_link(
    document: &select::document::Document,
) -> Result<Option<String>, ParsePageError> {
    let link = document
        .find(select::predicate::Name("article"))
        .next()
        .ok_or(ParsePageError::UnexpectedHtml)?
        .attr("data-openregistration-link")
        .ok_or(ParsePageError::UnexpectedHtml)?
        .to_string();
    if link.is_empty() {
        Ok(None)
    } else {
        Ok(Some(link))
    }
}

#[cfg(test)]
mod tests {
    use crate::event::DateRange;
    use crate::event_overview_page::{parse_page, ParsePageError};
    use crate::Event;
    use chrono::{TimeZone, Utc};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn in_progress_event() {
        let text = include_str!("../data/afspraak_overzicht_in_progress.html");
        let event = parse_page(text).unwrap();
        assert_eq!(
            event,
            Event {
                canonical_url: String::from(
                    "http://datumprikker.nl/afspraak/overzicht/fewqvuycnmvgnx25"
                ),
                title: String::from("D&D Avernus week 29"),
                final_date: None,
                open_registration_link: Some(String::from(
                    "https://datumprikker.nl/pux6s6a4febgnx25"
                )),
            }
        )
    }

    #[test]
    fn finalized_event() {
        let text = include_str!("../data/afspraak_overzicht_finalized.html");
        let event = parse_page(text).unwrap();
        assert_eq!(
            event,
            Event {
                canonical_url: String::from(
                    "http://datumprikker.nl/afspraak/overzicht/f4wfumjp7a9ih2nq"
                ),
                title: String::from("D&D Avernus Week 22"),
                final_date: Some(DateRange {
                    start: Utc.ymd(2022, 6, 3).and_hms(17, 0, 0),
                    end: Utc.ymd(2022, 6, 3).and_hms(21, 0, 0),
                }),
                open_registration_link: Some(String::from(
                    "https://datumprikker.nl/pbxzxuf7c8sih2nq"
                )),
            }
        )
    }

    #[test]
    fn participant_event() {
        let text = include_str!("../data/afspraak_overzicht_participant.html");
        let event = parse_page(text).unwrap();
        assert_eq!(
            event,
            Event {
                canonical_url: String::from(
                    "http://datumprikker.nl/afspraak/overzicht/mu2edbyv3bfayubtm"
                ),
                title: String::from("test"),
                final_date: None,
                open_registration_link: None,
            }
        )
    }

    #[test]
    fn invalid_event() {
        let text = include_str!("../data/afspraak_overzicht_invalid.html");
        let event = parse_page(text);
        assert_eq!(event, Err(ParsePageError::NonExistingEvent))
    }
}
