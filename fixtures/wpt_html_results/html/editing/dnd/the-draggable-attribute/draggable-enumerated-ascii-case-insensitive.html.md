# html/editing/dnd/the-draggable-attribute/draggable-enumerated-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-draggable-attribute/draggable-enumerated-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#the-draggable-attribute">
<link rel="help" href="https://html.spec.whatwg.org/#enumerated-attribute">
<meta name="assert" content="@draggable values are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<!--
  We use <img> elements here so the invalid value default (auto) can be
  distinguished from false through the IDL attribute. Most other elements
  return false from the IDL attribute for the auto state too.
-->
<img draggable="false">
<img draggable="FaLsE">
<img draggable="falſe">
<script>
const img = document.querySelectorAll("img");

test(() => {
  assert_equals(img[0].draggable, false, "lowercase valid");
  assert_equals(img[1].draggable, false, "mixed case valid");
  assert_equals(img[2].draggable, true, "non-ASCII invalid");
}, "keyword false");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 616,
        "byte_start": 593,
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
        "byte_end": 616,
        "byte_start": 593,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 640,
        "byte_start": 617,
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
        "byte_end": 640,
        "byte_start": 617,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 665,
        "byte_start": 641,
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
        "byte_end": 665,
        "byte_start": 641,
        "col": 1,
        "line": 15
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
  "source_name": "html/editing/dnd/the-draggable-attribute/draggable-enumerated-ascii-case-insensitive.html"
}
```
