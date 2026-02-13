# html/semantics/embedded-content/the-img-element/current-pixel-density/error.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/current-pixel-density/error.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img current pixel density error</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<img id=ref src="404" alt="testing">
<img srcset="404" alt="testing">
<img srcset="404 0.5x" alt="testing">
<img srcset="404 2x" alt="testing">
<img srcset="404 100w" alt="testing">
<img srcset="404 100w" sizes="500px" alt="testing">
<picture><img src="404 100w" sizes="500px" alt="testing"></picture>
<script>
setup({explicit_done:true});
onload = function() {
  var ref = document.getElementById("ref");
  var expected_width = ref.width;
  var expected_height = ref.height;
  [].forEach.call(document.images, function(img) {
    test(function() {
      assert_not_equals(expected_width, 0, 'expected_width');
      assert_not_equals(expected_height, 0, 'expected_height');
      assert_equals(img.width, expected_width, 'width');
      assert_equals(img.height, expected_height, 'height');
      assert_equals(img.naturalWidth, 0, 'naturalWidth');
      assert_equals(img.naturalHeight, 0, 'naturalHeight');
    }, img.outerHTML);
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
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 369,
        "byte_start": 332,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 479,
        "byte_start": 431,
        "col": 10,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.invalid",
      "message": "Bad value “404 100w” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 479,
        "byte_start": 431,
        "col": 10,
        "line": 12
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
  "source_name": "html/semantics/embedded-content/the-img-element/current-pixel-density/error.html"
}
```
