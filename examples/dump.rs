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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args()
        .skip(1)
        .next()
        .expect("1 argument is expected: event url");

    let event = datumprikker::download_event(url.as_str()).await?;
    println!("event url: {}", event.canonical_url);
    println!("title: {}", event.title);
    if let Some(final_date) = event.final_date {
        println!("start: {}", final_date.start.with_timezone(&chrono::Local));
        println!("end: {}", final_date.end.with_timezone(&chrono::Local));
    } else {
        println!("no final date selected")
    }

    Ok(())
}
