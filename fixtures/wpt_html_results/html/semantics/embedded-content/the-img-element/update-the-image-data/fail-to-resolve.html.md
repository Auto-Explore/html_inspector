# html/semantics/embedded-content/the-img-element/update-the-image-data/fail-to-resolve.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-the-image-data/fail-to-resolve.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img update the image data: fail to resolve URL</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>

<img src="//[">
<img srcset="//[">
<img srcset="//[" src="/images/red.png">
<img srcset="//[, /images/red.png">

<script>
setup({explicit_done: true});

var expected = '//[';

onload = function() {
  [].forEach.call(document.images, function(img) {
    test(function() {
      assert_equals(img.currentSrc, expected);
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
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 219,
        "byte_start": 204,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “//[” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 238,
        "byte_start": 220,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 238,
        "byte_start": 220,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “//[” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 279,
        "byte_start": 239,
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
        "byte_end": 279,
        "byte_start": 239,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “//[, /images/red.png” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 315,
        "byte_start": 280,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 315,
        "byte_start": 280,
        "col": 1,
        "line": 10
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
  "source_name": "html/semantics/embedded-content/the-img-element/update-the-image-data/fail-to-resolve.html"
}
```
