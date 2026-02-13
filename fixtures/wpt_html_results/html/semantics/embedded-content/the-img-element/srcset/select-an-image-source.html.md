# html/semantics/embedded-content/the-img-element/srcset/select-an-image-source.html

Counts:
- errors: 0
- warnings: 32
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/srcset/select-an-image-source.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img select an image source</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src=common.js></script>
<div id=log></div>
<!-- dup entries -->
<img srcset='data:,a 1x, data:,b 1x' data-expect='data:,a'>
<img srcset='data:,a , data:,b 1x' data-expect='data:,a'>
<img srcset='data:,a 1x, data:,b' data-expect='data:,a'>
<img srcset='data:,a 1w, data:,b 1w' data-expect='data:,a'>
<img srcset='data:,a 1w 1h, data:,b 1w' data-expect='data:,a'>
<img srcset='data:,a 1w, data:,b 1w 1h' data-expect='data:,a'>
<img srcset='data:,a 1w 1h, data:,b 1w 2h' data-expect='data:,a'>
<img srcset='data:,a 1w 2h, data:,b 1w 1h' data-expect='data:,a'>
<img srcset='data:,a , data:,b' data-expect='data:,a'>
<img srcset='data:,a 1w, data:,b 1x' sizes='1px' data-expect='data:,a'>
<img srcset='data:,a 1x, data:,b 1w' sizes='1px' data-expect='data:,a'>
<img srcset='data:,a 1w, data:,b 2x' sizes='0.5px' data-expect='data:,a'>
<img srcset='data:,a 2x, data:,b 1w' sizes='0.5px' data-expect='data:,a'>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x, data:,b 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 295,
        "byte_start": 236,
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
        "byte_end": 295,
        "byte_start": 236,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a , data:,b 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 353,
        "byte_start": 296,
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
        "byte_end": 353,
        "byte_start": 296,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x, data:,b” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 410,
        "byte_start": 354,
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
        "byte_end": 410,
        "byte_start": 354,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w, data:,b 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 470,
        "byte_start": 411,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 470,
        "byte_start": 411,
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
        "byte_end": 470,
        "byte_start": 411,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1h, data:,b 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 533,
        "byte_start": 471,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 533,
        "byte_start": 471,
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
        "byte_end": 533,
        "byte_start": 471,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w, data:,b 1w 1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 596,
        "byte_start": 534,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 596,
        "byte_start": 534,
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
        "byte_end": 596,
        "byte_start": 534,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1h, data:,b 1w 2h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 662,
        "byte_start": 597,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 662,
        "byte_start": 597,
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
        "byte_end": 662,
        "byte_start": 597,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 2h, data:,b 1w 1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 728,
        "byte_start": 663,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 728,
        "byte_start": 663,
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
        "byte_end": 728,
        "byte_start": 663,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a , data:,b” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 783,
        "byte_start": 729,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 783,
        "byte_start": 729,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w, data:,b 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 855,
        "byte_start": 784,
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
        "byte_end": 855,
        "byte_start": 784,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x, data:,b 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 927,
        "byte_start": 856,
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
        "byte_end": 927,
        "byte_start": 856,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w, data:,b 2x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1001,
        "byte_start": 928,
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
        "byte_end": 1001,
        "byte_start": 928,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 2x, data:,b 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1075,
        "byte_start": 1002,
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
        "byte_end": 1075,
        "byte_start": 1002,
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
  "source_name": "html/semantics/embedded-content/the-img-element/srcset/select-an-image-source.html"
}
```
