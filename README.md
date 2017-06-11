# Favicon Maker

Favicon Maker will quickly create a set of very simple favicons from a template. See screenshots below.


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

<img src="https://raw.githubusercontent.com/at1as/favicon_maker/master/output/favicon512.png" width="150px">


### Example – Output Scaled

`cargo run` versus `cargo run 0 65`
* Horizontal Shift : 0%
* Horizontal Scale : 65%

<div>
<img src="https://raw.githubusercontent.com/at1as/favicon_maker/master/output/example_favicon_MM.jpg" width="150px" style="margin:20px; padding:10px">
<img src="https://raw.githubusercontent.com/at1as/favicon_maker/master/output/example_favicon_MM_scale_65_percent.jpg" width="150px" style="margin:20px; padding:10px">
</div>

### Example - Output Shifted

`cargo run` versus `cargo run 30`
* Horizontal Shift : 30%
* Horizontal Scale : 80% (the default. The above is idential to `cargo run 30 80`)

<div>
<img src="https://raw.githubusercontent.com/at1as/favicon_maker/master/output/example_favicon_II.jpg" width="150px" style="margin:20px; padding:10px">
<img src="https://raw.githubusercontent.com/at1as/favicon_maker/master/output/example_favicon_II_shift_30_percent.jpg" width="150px" style="margin:20px; padding:10px">
</div>

### Notes:

* Built with Rust 1.17.0
* Built on MacOS 10.11


### TODO:

* Font Autoscaling
* Add Tests
* Make colors configurable

