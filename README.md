# Favicon Maker

Favicon Maker will quickly build a simple favicons from a template for webprojects 


### Usage

```
$ git clone https://github.com/at1as/favicon_maker
$ cd favicon_maker
$ cargo run
```

### Configuration

See src/conf.json:

```
{
  "sizes": [
    {"pixels": 16,  "format": "jpg"},
    {"pixels": 512, "format": "jpg"},
    {"pixels": 512, "format": "png"}
  ],
    "text": "FM"
}
```

* Text
  * The text to display on the favicon (works best with 2 or 3 characters)
* Sizes
  * list of desired format and pixel size combinations (not favicons are square, "pixels" represents both the horizontal and vertical dimensions)


### Command Line Parameters

As different font characters will have different widths

```
$ cargo run {horizontal offset as percent (default: 0%)} {horizontal scaling factor as percent (default 80%)}


# For Example: 
#   this line would shift the characters left by 10% of the favicon width
#   and scale the favicon text to 60% of the favicon width

$ cargo run 10 60
```


### Example Output

<img src="https://raw.githubusercontent.com/at1as/favicon_maker/master/output/favicon512.png" width="200px">


### Notes:

* Built with Rust 1.17.0
* Built on MacOS 10.11


### TODO:

* Font Autoscaling
* Add Tests
* Make colors configurable

