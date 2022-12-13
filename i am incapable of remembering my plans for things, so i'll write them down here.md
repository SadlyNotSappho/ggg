OK SO the view functions for the ui can do whatever the hell i want<br>
currently `init` gets the current page from config and just, like, does whatever with that.<br>
`prev` and `next` use functions I made in `lib.rs` to get the previous and next pages. it works, but can be improved.

i am instead going to have one function that gets a page (with an arg to say, basically, "use specified", "use previous from specified", and "use next from specified"), writes its data to the pages file, and saves the image (also updating the previous to say that the next one is cached, and what it is)

i will then have the `init` function go through a wrapper function for the previous, which will check the pages file, and show the current page if it's cached. if it somehow isn't, it downloads it.

`prev` and `next` do the same thing - they get (and, if not cached, cache) the current page, get the next page, download all of them to the cache, and update the pages file.