use std::{collections::{BTreeMap, BTreeSet, HashMap}, fmt::Write, time::{Duration}};
use serde::{Deserialize, Serialize};

/// A structure for collecting and formatting timing data across multiple categories and descriptions.
///
/// Each *key* in [`times`] represents a group (e.g., a component or operation).
/// Each group holds multiple labeled durations identified by a *description* string.
///
/// This type supports printing organized timing summaries in tabular form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeLogs {
    pub times: HashMap<String, BTreeMap<String, Duration>>,
    pub max_key_len: usize,
}

impl TimeLogs {

    /// Creates a new empty [`TimeLogs`] instance.
    ///
    /// # Returns
    /// A new [`TimeLogs`] with no entries and a maximum key length of zero.
    pub fn new() -> Self {
        Self { times: HashMap::new(), max_key_len: 0}
    }

    /// Records a duration for a given key and description.
    ///
    /// If the key or description doesn't exist, it will be inserted automatically.
    /// The method also updates the internal `max_key_len` to ensure aligned output
    /// when generating table views.
    ///
    /// # Arguments
    /// * `key` - The primary identifier for the timing group.
    /// * `description` - A textual label for a specific measurement in that group.
    pub fn push<S: Into<String>>(&mut self, key: &String, description: S, time: Duration) {
        let time_store = match self.times.get_mut(key) {
            Some(val) => val,
            None => {
                self.times.insert(key.clone(), BTreeMap::new());
                self.times.get_mut(key).unwrap()
            },
        };

        let string = description.into();
        self.max_key_len = self.max_key_len
            .max(key.len())
            .max(string.len());

        time_store.insert(string, time);
    }

    /// Produces a formatted table string summarizing all timing data, grouped per key and description.
    ///
    /// The output table includes both individual durations and total durations per row.
    /// If there are more keys than can fit in the given `max_len`, columns are chunked accordingly.
    ///
    /// # Arguments
    /// * `max_len` - The maximum allowed line width for the table display.
    ///
    /// # Returns
    /// A human-readable table string suitable for printing or logging.
    pub fn to_table_string(&self, max_len: usize) -> String {
        let mut keys: Vec<&String> = self.times.keys().collect();
        keys.sort();

        let descriptions: BTreeSet<String> = self.times.values()
            .flat_map(|map| map.keys().cloned())
            .collect();

        let mut table = String::new();

        let max_desc_len = self.max_key_len.max("Description".len()) + 1;
        let bar_len = max_desc_len;
        
        let col_width = bar_len + 2;
        let max_key_columns = ((max_len - bar_len) / col_width).max(1);
        
        for chunk in keys.chunks(max_key_columns) {
            write!(&mut table, "{:<width$}", "Description", width = bar_len).unwrap();
            for key in chunk {
                write!(&mut table, "| {:<width$}", key, width = bar_len).unwrap();
            }
            write!(&mut table, "| {:<width$}", "Total", width = bar_len).unwrap();
            table.push('\n');

            write!(&mut table, "{:-<width$}", "", width = bar_len).unwrap();
            for _ in chunk {
                write!(&mut table, "+{:-<width$}", "", width = bar_len + 1).unwrap();
            }
            write!(&mut table, "+{:-<width$}", "", width = bar_len + 1).unwrap();
            table.push('\n');

            for desc in descriptions.iter().rev() {
                write!(&mut table, "{:<width$}", desc, width = bar_len).unwrap();

                let mut total = std::time::Duration::new(0, 0);

                for key in chunk {
                    if let Some(inner) = self.times.get(*key) {
                        if let Some(dur) = inner.get(desc) {
                            total += *dur;
                            write!(
                                &mut table,
                                "| {:<width$}",
                                format_duration(*dur),
                                width = bar_len
                            ).unwrap();
                        } else {
                            write!(&mut table, "| {:<width$}", "_", width = bar_len).unwrap();
                        }
                    } else {
                        write!(&mut table, "| {:<width$}", "_", width = bar_len).unwrap();
                    }
                }

                write!(
                    &mut table,
                    "| {:<width$}",
                    format_duration(total),
                    width = bar_len
                ).unwrap();
                table.push('\n');
            }

            write!(&mut table, "{:-<width$}", "", width = bar_len).unwrap();
            table.push_str("\n\n");
        }

        table
    }

    /// Generates a simplified table showing only total duration per description.
    ///
    /// This summary omits per-key columns and aggregates all entries across keys.
    ///
    /// # Returns
    /// A string representing the formatted totals table.
    pub fn to_total_only_table_string(&self) -> String {
        let descriptions: BTreeSet<String> = self.times
            .values()
            .flat_map(|map| map.keys().cloned())
            .collect();

        let mut table = String::new();

        let max_desc_len = self.max_key_len.max("Description".len()) + 1;
        let bar_len = max_desc_len;

        write!(&mut table, "{:<width$} | {:<width2$}",
            "Description",
            "Total",
            width = bar_len,
            width2 = bar_len
        ).unwrap();
        table.push('\n');

        write!(&mut table, "{:-<width$}-+-{:-<width2$}",
            "",
            "",
            width = bar_len,
            width2 = bar_len
        ).unwrap();
        table.push('\n');

        for desc in descriptions.into_iter().rev() {
            let mut total = std::time::Duration::new(0, 0);

            for inner in self.times.values() {
                if let Some(dur) = inner.get(&desc) {
                    total += *dur;
                }
            }

            write!(
                &mut table,
                "{:<width$} | {:<width2$}",
                desc,
                format_duration(total),
                width = bar_len,
                width2 = bar_len
            ).unwrap();
            table.push('\n');
        }

        write!(&mut table, "{:-<width$}", "", width = bar_len).unwrap();
        table
    }

}

/// Formats a [`Duration`] into a compact human-readable string.
///
/// The output uses seconds, with millisecond precision when applicable.
/// For example:
/// - 2.530s  
/// - 4s
///
/// # Arguments
/// * `dur` - A [`Duration`] value to format.
///
/// # Returns
/// A string representing the formatted duration.
pub fn format_duration(dur: Duration) -> String {
    let secs = dur.as_secs();
    let millis = dur.subsec_millis();
    
    if millis > 0 {
        format!("{}.{}s", secs, format!("{:03}", millis))
    } else {
        format!("{}s", secs)
    }
}













