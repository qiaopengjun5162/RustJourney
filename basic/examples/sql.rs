use basic::sql;

fn main() {
    sql!(SELECT * FROM table1 WHERE id = 10 AND timestamp > 1000 ORDER BY timestamp DESC LIMIT 10);
}
