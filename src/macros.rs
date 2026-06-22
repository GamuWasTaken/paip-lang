#[macro_export]
macro_rules! build_parser {
(
    singles: [ $($single_char: literal => $single_kind: ident),+ $(,)?],
    doubles: [
        $($start_char: literal => [
            $($last_char: literal => $double_kind: ident),+,
            else => $double_else: ident
        ]),+
        $(,)?
    ],
    strings: [ $( $func:path => $func_kind:ident ),+ $(,)?],
    comment: $comm_char: literal


) => {

    #[derive(Debug, Clone, Copy)]
    pub enum Kind {
        $(
            $single_kind,
        )+

        $(
           $(
              $double_kind,
           )+
          $double_else,
        )+

        $(
            $func_kind,
        )+

        Comment,
        Error, // An unrecognized token
        // EOI, // End Of Input TODO we may not need this, pos is just str.len()
    }

    impl<'a, T> Iterator for &'a mut Tokens<T>
    where T: PeekN<Item=(usize, char)>
    {
        type Item = Kind;

        fn next(&mut self) -> Option<Self::Item> {
            let (i, c) = self.iter.peek_n(0)?;
            let tok = match c {
                $( $single_char => {
                        self.iter.skip_n(1);
                        Kind::$single_kind
                   } ),+
                $( $start_char => match self.iter.peek_n(1) {
                    $( Some((_, $last_char)) => {
                            self.iter.skip_n(2);
                            Kind::$double_kind
                        } ),+
                        None | Some(_) => {
                            self.iter.skip_n(1);
                            Kind::$double_else
                        }
                   } ),+
                   $comm_char => { // Consume till new line (\n, \r\n, \r)
                        while let Some((_, c)) = self.iter.peek_n(0)
                            && !(c == '\n' || c == '\r')
                        {
                            self.iter.skip_n(1);
                        }
                        if let Some((_, c)) = self.iter.peek_n(0) && c == '\n' {
                            self.iter.skip_n(1);
                        }
                        Kind::Comment
                   }
                $( e if $func(e) => {
                        while let Some((_, c)) = self.iter.peek_n(0)
                            && $func(c)
                        {
                            self.iter.skip_n(1);
                        }
                        Kind::$func_kind
                   } ),+

                   _ => {
                        self.iter.skip_n(1);
                        Kind::Error
                   },
                };
            self.add(tok, i as _);
            Some(tok)
            }
        }
    };
}
