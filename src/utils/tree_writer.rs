use std::fmt::{Display, Write};

pub struct TreePrinter {
    wr: IndentedWriter<String>,
}

impl TreePrinter {
    pub fn start(name: &str) -> Self {
        let mut me = Self {
            wr: IndentedWriter::new(String::new()),
        };

        writeln!(me.wr, "{name} {{");
        me.wr.indent();
        me
    }

    pub fn field<T: Display>(mut self, name: &str, t: T) -> Self {
        writeln!(self.wr, "{name}: {t}");
        self
    }

    pub fn field_list<T, I: IntoIterator<Item = T>, D: Display, F: FnMut(T) -> D>(
        mut self,
        name: &str,
        ts: I,
        mut f: F,
    ) -> Self {
        write!(self.wr, "{name}: [");
        self.wr.indent();

        let mut short = true;
        for t in ts.into_iter() {
            short = false;
            writeln!(self.wr);
            write!(self.wr, "{}", f(t));
        }

        self.wr.outdent();
        if !short {
            writeln!(self.wr);
        }
        writeln!(self.wr, "]");
        self
    }

    pub fn finish(mut self) -> String {
        self.wr.outdent();
        write!(self.wr, "}}");
        self.wr.to_inner()
    }
}

struct IndentedWriter<W: Write> {
    buf: W,
    on_newline: bool,
    depth: usize,
}

impl<W: Write> IndentedWriter<W> {
    fn new(buf: W) -> Self {
        Self {
            buf,
            on_newline: false,
            depth: 0,
        }
    }

    fn indent(&mut self) -> &mut Self {
        self.depth += 1;
        self
    }

    fn outdent(&mut self) -> &mut Self {
        self.depth = self.depth.saturating_sub(1);
        self
    }

    fn to_inner(self) -> W {
        self.buf
    }
}

impl<W: Write> Write for IndentedWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for s in s.split_inclusive('\n') {
            if self.on_newline {
                for _ in 0..self.depth {
                    self.buf.write_str("  ")?;
                }
            }

            self.on_newline = s.ends_with('\n');
            self.buf.write_str(s)?;
        }

        Ok(())
    }
}
