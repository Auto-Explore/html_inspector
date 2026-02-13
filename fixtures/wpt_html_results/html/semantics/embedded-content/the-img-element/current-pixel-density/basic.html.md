# html/semantics/embedded-content/the-img-element/current-pixel-density/basic.html

Counts:
- errors: 0
- warnings: 19
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/current-pixel-density/basic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img current pixel density basic</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<img src="/images/green-256x256.png" data-expect="256">
<img srcset="/images/green-256x256.png 1x" data-expect="256">
<img srcset="/images/green-256x256.png 1.6x" data-expect="160">
<img srcset="/images/green-256x256.png 2x" data-expect="128">
<img srcset="/images/green-256x256.png 10000x" data-expect="0">
<img srcset="/images/green-256x256.png 9e99999999999999999999999x" data-expect="0">
<img srcset="/images/green-256x256.png 256w" sizes="256px" data-expect="256">
<img srcset="/images/green-256x256.png 512w" sizes="256px" data-expect="128">
<img srcset="/images/green-256x256.png 256w" sizes="512px" data-expect="512">
<img srcset="/images/green-256x256.png 256w" sizes="1px" data-expect="1">
<img srcset="/images/green-256x256.png 256w" sizes="0px" data-expect="0">
<!-- SVG -->
<img srcset="data:image/svg+xml,<svg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='-1%20-1%202%202'%20width='20'%20height='20'><circle%20r='1'/></svg> 2x" data-expect="10">
<img srcset="data:image/svg+xml,<svg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='-1%20-1%202%202'%20width='20'><circle%20r='1'/></svg> 2x" data-expect="10">
<img srcset="data:image/svg+xml,<svg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='-1%20-1%202%202'%20height='20'><circle%20r='1'/></svg> 2x" data-expect="10">
<script>
setup({explicit_done:true});
onload = function() {
  [].forEach.call(document.images, function(img) {
    test(function() {
      var expected = parseFloat(img.dataset.expect);
      assert_equals(img.width, expected, 'width');
      assert_equals(img.height, expected, 'height');
      assert_equals(img.clientWidth, expected, 'clientWidth');
      assert_equals(img.clientHeight, expected, 'clientHeight');
      assert_equals(img.naturalWidth, expected, 'naturalWidth');
      assert_equals(img.naturalHeight, expected, 'naturalHeight');
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
        "byte_end": 243,
        "byte_start": 188,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 305,
        "byte_start": 244,
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
        "byte_end": 369,
        "byte_start": 306,
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
        "byte_end": 431,
        "byte_start": 370,
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
        "byte_end": 495,
        "byte_start": 432,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “/images/green-256x256.png 9e99999999999999999999999x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 579,
        "byte_start": 496,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 579,
        "byte_start": 496,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 657,
        "byte_start": 580,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 735,
        "byte_start": 658,
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
        "byte_end": 813,
        "byte_start": 736,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 887,
        "byte_start": 814,
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
        "byte_end": 961,
        "byte_start": 888,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:image/svg+xml,<svg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='-1%20-1%202%202'%20width='20'%20height='20'><circle%20r='1'/></svg> 2x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1149,
        "byte_start": 975,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1149,
        "byte_start": 975,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:image/svg+xml,<svg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='-1%20-1%202%202'%20width='20'><circle%20r='1'/></svg> 2x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1310,
        "byte_start": 1150,
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
        "byte_end": 1310,
        "byte_start": 1150,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:image/svg+xml,<svg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='-1%20-1%202%202'%20height='20'><circle%20r='1'/></svg> 2x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1472,
        "byte_start": 1311,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1472,
        "byte_start": 1311,
        "col": 1,
        "line": 20
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
  "source_name": "html/semantics/embedded-content/the-img-element/current-pixel-density/basic.html"
}
```
