error_chain!{
    errors {
        OutOfBounds
    }

   foreign_links {
       IoError(::std::io::Error);
   }
}
