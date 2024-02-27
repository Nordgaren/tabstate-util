# notepad-buffer
A buffer reader for notepad TabState buffers.  

## Does it work?
Lmao

## How do I use it?
The tab buffers are located in `%localappdata%\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState`.

Thank you to @nas_bench and @_JohnHammond for cluing me into this location.

Read one of the files and pass it in to `NPBufferReader::new()` as a slice. Call the `get_refs()` method to get references 
to parts of the buffer.

There are some printlines for things I am not sure about, so if you see this print some extra lines in your console, please
let me know, so I can check out what is wrong.

The `NPBufferReader` type is there for the future. This type will handle other stuff, later, probably.

```rust

#[test]
fn it_works() {
    let buffer = std::fs::read("P:/ath/to/notepad/tabstate/buffer.bin").unwrap();
    let np = NPBufferReader::new(&buffer[..]);
    let refs = np.get_refs().unwrap();

    println!("{:?}", refs.get_path().unwrap_or_default());
    println!("{:?}", refs.get_buffer());
}
```

## How can I contribute
Open a github issue, or message me on discord. Name is `Nordgaren`. GitHub issues is easier. If I don't get to you on Discord
you can @ me in any shared server we have. I am in John Hammonds Discord. You can also try e-mailing me at `nordgarentv@gmail.com`

## Todo
> Figure out any inkling of how TF the unsaved buffers work.  