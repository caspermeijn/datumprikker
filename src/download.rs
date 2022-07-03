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

use crate::event::Event;
use crate::event_overview_page::ParsePageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("network error during download")]
    NetworkError(#[from] reqwest::Error),
    #[error("parse error of page")]
    ParseError(#[from] ParsePageError),
}

pub async fn download_event(url: &str) -> Result<Event, DownloadError> {
    let resp = reqwest::get(url).await?.text().await?;
    let event = crate::event_overview_page::parse_page(resp.as_str())?;
    Ok(event)
}
