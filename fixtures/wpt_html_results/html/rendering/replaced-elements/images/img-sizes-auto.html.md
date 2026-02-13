# html/rendering/replaced-elements/images/img-sizes-auto.html

Counts:
- errors: 0
- warnings: 36
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/images/img-sizes-auto.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>default styles for &lt;img sizes=auto></title>
<meta name="viewport" content="width=device-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/rendering/support/test-ua-stylesheet.js"></script>
<style>
/* Specify this bogus namespace, so the rules in this stylesheet only apply to the `fakeClone`d elements in #refs, not the HTML elements in #tests. */
@namespace url(urn:not-html);

img:is([sizes="auto" i], [sizes^="auto," i]) {
  contain: size !important;
  contain-intrinsic-size: 300px 150px;
}
</style>

<style>
/* Test !important */
img.test-important { contain: style; contain-intrinsic-size: 30px 15px; }
</style>

<div id="log"></div>

<div id="tests">
  <img sizes=auto>
  <img sizes=auto width=10 height=20>
  <img sizes=AuTo>
  <img sizes=auto,xyz>
  <img sizes=AuTo,xyz>

  <!-- UA style should not apply: -->
  <img>
  <img sizes>
  <img sizes=xyz,auto>
  <picture data-skip>
   <source data-skip sizes=auto>
   <img>
  </picture>

  <!-- Test !important -->
  <img sizes=auto class=test-important>
</div>

<div id="refs"></div>

<script>
  const props = [
    'contain',
    'contain-intrinsic-size'
  ];
  runUAStyleTests(props);

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.sizes_auto.requires_loading_lazy",
      "message": "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. Add “loading=”“lazy” to this element.",
      "severity": "Warning",
      "span": {
        "byte_end": 785,
        "byte_start": 769,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 785,
        "byte_start": 769,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 785,
        "byte_start": 769,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 785,
        "byte_start": 769,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes_auto.requires_loading_lazy",
      "message": "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. Add “loading=”“lazy” to this element.",
      "severity": "Warning",
      "span": {
        "byte_end": 823,
        "byte_start": 788,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 823,
        "byte_start": 788,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 823,
        "byte_start": 788,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 823,
        "byte_start": 788,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes_auto.requires_loading_lazy",
      "message": "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. Add “loading=”“lazy” to this element.",
      "severity": "Warning",
      "span": {
        "byte_end": 842,
        "byte_start": 826,
        "col": 3,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 842,
        "byte_start": 826,
        "col": 3,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 842,
        "byte_start": 826,
        "col": 3,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 842,
        "byte_start": 826,
        "col": 3,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes_auto.requires_loading_lazy",
      "message": "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. Add “loading=”“lazy” to this element.",
      "severity": "Warning",
      "span": {
        "byte_end": 865,
        "byte_start": 845,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 865,
        "byte_start": 845,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 865,
        "byte_start": 845,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 865,
        "byte_start": 845,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes_auto.requires_loading_lazy",
      "message": "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. Add “loading=”“lazy” to this element.",
      "severity": "Warning",
      "span": {
        "byte_end": 888,
        "byte_start": 868,
        "col": 3,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 888,
        "byte_start": 868,
        "col": 3,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 888,
        "byte_start": 868,
        "col": 3,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 888,
        "byte_start": 868,
        "col": 3,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 935,
        "byte_start": 930,
        "col": 3,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 935,
        "byte_start": 930,
        "col": 3,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 949,
        "byte_start": 938,
        "col": 3,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 949,
        "byte_start": 938,
        "col": 3,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 949,
        "byte_start": 938,
        "col": 3,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 972,
        "byte_start": 952,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 972,
        "byte_start": 952,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 972,
        "byte_start": 952,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1027,
        "byte_start": 998,
        "col": 4,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1036,
        "byte_start": 1031,
        "col": 4,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1036,
        "byte_start": 1031,
        "col": 4,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes_auto.requires_loading_lazy",
      "message": "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. Add “loading=”“lazy” to this element.",
      "severity": "Warning",
      "span": {
        "byte_end": 1117,
        "byte_start": 1080,
        "col": 3,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.requires_srcset",
      "message": "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 1117,
        "byte_start": 1080,
        "col": 3,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1117,
        "byte_start": 1080,
        "col": 3,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1117,
        "byte_start": 1080,
        "col": 3,
        "line": 41
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
  "source_name": "html/rendering/replaced-elements/images/img-sizes-auto.html"
}
```
