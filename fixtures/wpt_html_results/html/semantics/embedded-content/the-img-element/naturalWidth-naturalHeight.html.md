# html/semantics/embedded-content/the-img-element/naturalWidth-naturalHeight.html

Counts:
- errors: 0
- warnings: 21
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/naturalWidth-naturalHeight.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTMLImageElement.prototype.naturalWidth/naturalHeight</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<img src="resources/cat.jpg"
     title="raster image" data-width="320" data-height="240">
<img src="resources/cat.jpg" width="10" height="10"
     title="raster image with width/height attributes" data-width="320" data-height="240">
<img src="non-existent.jpg"
     title="non existent image, no natural dimensions" data-width="0" data-height="0">
<img src="non-existent.jpg" width="10" height="10"
     title="non existent image with width/height attributes, no natural dimensions" data-width="0" data-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>"
     title="SVG image, no natural dimensions" data-width="0" data-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='500' height='400'></svg>"
     title="SVG image, width/height in pixels" data-width="500" data-height="400">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='500'></svg>"
     title="SVG image, width in pixels; height unspecified" data-width="500" data-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='500' height='100%'></svg>"
     title="SVG image, width in pixels; percentage height" data-width="500" data-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='500' height='400' viewBox='0 0 800 600'></svg>"
     title="SVG image, width/height in pixels; viewBox" data-width="500" data-height="400">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 800 600'></svg>"
     title="SVG image, width/height unspecified; viewBox" data-width="0" data-height="0">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='400' viewBox='0 0 800 600'></svg>"
     title="SVG image, width in pixels; height unspecified; viewBox" data-width="400" data-height="300">
<img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='300' viewBox='0 0 800 600'></svg>"
     title="SVG image, width unspecified; height in pixels; viewBox" data-width="400" data-height="300">
<script>
setup({explicit_done:true});
onload = function() {
  Array.from(document.images).forEach(img => {
    test(function() {
      const expectedWidth = parseFloat(img.dataset.width);
      const expectedHeight = parseFloat(img.dataset.height);
      assert_equals(img.naturalWidth, expectedWidth, 'naturalWidth');
      assert_equals(img.naturalHeight, expectedHeight, 'naturalHeight');
    }, `${document.title}, ${img.title}`);
  });
  done();
};
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 281,
        "byte_start": 191,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 424,
        "byte_start": 282,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 539,
        "byte_start": 425,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 706,
        "byte_start": 540,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 861,
        "byte_start": 707,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 861,
        "byte_start": 707,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='500' height='400'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1046,
        "byte_start": 862,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1046,
        "byte_start": 862,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='500'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1229,
        "byte_start": 1047,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1229,
        "byte_start": 1047,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='500' height='100%'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1425,
        "byte_start": 1230,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1425,
        "byte_start": 1230,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='500' height='400' viewBox='0 0 800 600'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1641,
        "byte_start": 1426,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1641,
        "byte_start": 1426,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 800 600'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1830,
        "byte_start": 1642,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1830,
        "byte_start": 1642,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='400' viewBox='0 0 800 600'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2046,
        "byte_start": 1831,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2046,
        "byte_start": 1831,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' height='300' viewBox='0 0 800 600'></svg>” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2263,
        "byte_start": 2047,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2263,
        "byte_start": 2047,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/naturalWidth-naturalHeight.html"
}
```
