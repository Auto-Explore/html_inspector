# html/canvas/element/manual/wide-gamut-canvas/ImageData-fidelity.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/wide-gamut-canvas/ImageData-fidelity.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(function(t) {
  var image = new Image();
  // This image is 256 by 1 and contains an opaque grayscale ramp from 0 to 255.
  // The image has no embedded color profile.
  image.src = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAABCAYAAAAx" +
    "WXB3AAAAG0lEQVQ4T2NkYGD4z8jIyDCKR8NgNA2MvDQAAPiPA/5tZ8G+AAAAAElFTkSuQmCC";

  image.onload = function() {
    var canvas = document.createElement('canvas');
    canvas.width = 256;
    canvas.height = 1;
    var ctx = canvas.getContext('2d');
    ctx.drawImage(image, 0, 0);
    var img = ctx.getImageData(0, 0, 256, 1);
    t.step(function() {
      for (var i = 0; i < 256; i++) {
        assert_equals(img.data[4 * i], i, "red component");
        assert_equals(img.data[4 * i + 1], i, "green component");
        assert_equals(img.data[4 * i + 2], i, "blue component");
        assert_equals(img.data[4 * i + 3], 255, "alpha component");
      }
    });
    t.done();
  }
}, "Verify that drawImage->getImageData round trip preserves color values " +
    "when image metadata has no color space and canvas uses the default " +
    "color space.");

async_test(function(t) {
  var image = new Image();
  // This image is 256 by 1 and contains an opaque grayscale ramp from 0 to 255.
  // The image has an embedded sRGB color profile.
  image.src =
      "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAABCAYAAAAxWXB3AAAA" +
      "G0lEQVQ4T2NkYGD4z8jIyDCKR8NgNA2MvDQAAPiPA/5tZ8G+AAAAo3pUWHRSYXcgcHJvZm" +
      "lsZSB0eXBlIEFQUDEAAHicZU5bCsMwDPv3KXoEv/I6TrampTC20ft/LE7WETJBkK1YQrCX" +
      "ZzmP+/I+X9vxKLAYyCGoC9En77FCV10ROWNHrM8hUW7cQZ00V/026tDZMRKbUQYDt4lJJr" +
      "2FxeCTJc5BV4svNE4Nxl1Tn8N1LCgMIoKJ2sHvo25sHfK/odYT02luCWMP+AA5M0KbNr61" +
      "PwAAAABJRU5ErkJggg==";

  image.onload = function() {
    var canvas = document.createElement('canvas');
    canvas.width = 256;
    canvas.height = 1;
    var ctx = canvas.getContext('2d', {colorSpace: 'srgb'});
    ctx.drawImage(image, 0, 0);
    var img = ctx.getImageData(0, 0, 256, 1);
    t.step(function() {
      for (var i = 0; i < 256; i++) {
        assert_equals(img.data[4 * i], i, "red component");
        assert_equals(img.data[4 * i + 1], i, "green component");
        assert_equals(img.data[4 * i + 2], i, "blue component");
        assert_equals(img.data[4 * i + 3], 255, "alpha component");
      }
    });
    t.done();
  }
}, "Verify that drawImage->getImageData round trip preserves color values " +
    "when image metadata has srgb color space and canvas uses the srgb " +
    "color space.");

async_test(function(t) {
  var image = new Image();
  // This image is 256 by 1 and contains an opaque grayscale ramp from 0 to 255.
  // The image has no embedded color profile.
  image.src = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAABCAYAAAAx" +
    "WXB3AAAAG0lEQVQ4T2NkYGD4z8jIyDCKR8NgNA2MvDQAAPiPA/5tZ8G+AAAAAElFTkSuQmCC";

  image.onload = function() {
    var canvas = document.createElement('canvas');
    canvas.width = 256;
    canvas.height = 1;
    var ctx = canvas.getContext('2d', {colorSpace: 'srgb'});
    ctx.drawImage(image, 0, 0);
    var img = ctx.getImageData(0, 0, 256, 1);
    t.step(function() {
      for (var i = 0; i < 256; i++) {
        assert_equals(img.data[4 * i], i, "red component");
        assert_equals(img.data[4 * i + 1], i, "green component");
        assert_equals(img.data[4 * i + 2], i, "blue component");
        assert_equals(img.data[4 * i + 3], 255, "alpha component");
      }
    });
    t.done();
  }
}, "Verify that drawImage->getImageData round trip preserves color values " +
    "when image metadata has no color space and canvas uses the srgb " +
    "color space.");


async_test(function(t) {
  var image = new Image();
  // This image is 256 by 1 and contains an opaque grayscale ramp from 0 to 255.
  // The image has an embedded sRGB color profile.
  image.src =
      "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAABCAYAAAAxWXB3AAAA" +
      "G0lEQVQ4T2NkYGD4z8jIyDCKR8NgNA2MvDQAAPiPA/5tZ8G+AAAAo3pUWHRSYXcgcHJvZm" +
      "lsZSB0eXBlIEFQUDEAAHicZU5bCsMwDPv3KXoEv/I6TrampTC20ft/LE7WETJBkK1YQrCX" +
      "ZzmP+/I+X9vxKLAYyCGoC9En77FCV10ROWNHrM8hUW7cQZ00V/026tDZMRKbUQYDt4lJJr" +
      "2FxeCTJc5BV4svNE4Nxl1Tn8N1LCgMIoKJ2sHvo25sHfK/odYT02luCWMP+AA5M0KbNr61" +
      "PwAAAABJRU5ErkJggg==";

  image.onload = function() {
    var canvas = document.createElement('canvas');
    canvas.width = 256;
    canvas.height = 1;
    var ctx = canvas.getContext('2d');
    ctx.drawImage(image, 0, 0);
    var img = ctx.getImageData(0, 0, 256, 1);
    t.step(function() {
      for (var i = 0; i < 256; i++) {
        assert_equals(img.data[4 * i], i, "red component");
        assert_equals(img.data[4 * i + 1], i, "green component");
        assert_equals(img.data[4 * i + 2], i, "blue component");
        assert_equals(img.data[4 * i + 3], 255, "alpha component");
      }
    });
    t.done();
  }
}, "Verify that drawImage->getImageData round trip preserves color values " +
    "when image metadata has srgb color space and canvas uses the default " +
    "color space.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 40,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/element/manual/wide-gamut-canvas/ImageData-fidelity.html"
}
```
