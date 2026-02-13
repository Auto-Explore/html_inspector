# html/embedded-content/the-img-element/attr-img-fetchpriority.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/embedded-content/the-img-element/attr-img-fetchpriority.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Fetch Priority - Image element</title>
<meta name="author" title="Dominic Farolino" href="mailto:domfarolino@gmail.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<img id=img1 src=/images/green.png fetchpriority=high>
<img id=img2 src=/images/green.png fetchpriority=low>
<img id=img3 src=/images/green.png fetchpriority=auto>
<img id=img4 src=/images/green.png fetchpriority=xyz>
<img id=img5 src=/images/green.png>

<script>
  test(() => {
    assert_equals(img1.fetchPriority, "high", "high fetchPriority is a valid IDL value on the image element");
    assert_equals(img2.fetchPriority, "low", "low fetchPriority is a valid IDL value on the image element");
    assert_equals(img3.fetchPriority, "auto", "auto fetchPriority is a valid IDL value on the image element");
    assert_equals(img4.fetchPriority, "auto", "invalid fetchPriority reflects as 'auto' IDL attribute on the image element");
    assert_equals(img5.fetchPriority, "auto", "missing fetchPriority reflects as 'auto' IDL attribute on the image element");
  }, "fetchpriority attribute on <img> elements should reflect valid IDL values");

  test(() => {
    const img = new Image();
    assert_equals(img.fetchPriority, "auto");
  }, "fetchPriority of new Image() is 'auto'");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 143,
        "byte_start": 62,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 143,
        "byte_start": 62,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 305,
        "byte_start": 251,
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
        "byte_end": 359,
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
        "byte_end": 414,
        "byte_start": 360,
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
        "byte_end": 468,
        "byte_start": 415,
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
        "byte_end": 504,
        "byte_start": 469,
        "col": 1,
        "line": 11
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
  "source_name": "html/embedded-content/the-img-element/attr-img-fetchpriority.html"
}
```
