# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/embedded-and-images-presentational-hints-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 11
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/embedded-and-images-presentational-hints-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#attributes-for-embedded-content-and-images:presentational-hints">
<link rel="help" href="https://drafts.csswg.org/selectors-4/#attribute-case">
<meta name="assert" content="@align values on embedded content and images are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<img src="fuchsia.png" align="absbottom">
<img src="fuchsia.png" align="AbSbOtToM">
<img src="fuchsia.png" align="abſbottom">
<img src="fuchsia.png" align="abscenter">
<img src="fuchsia.png" align="AbScEnTeR">
<img src="fuchsia.png" align="abſcenter">
<img src="fuchsia.png" align="absmiddle">
<img src="fuchsia.png" align="AbSmIdDlE">
<img src="fuchsia.png" align="abſmiddle">
<script>
const img = document.querySelectorAll("img");

test(() => {
  assert_equals(getComputedStyle(img[0]).getPropertyValue("vertical-align"),
    "bottom", "lowercase valid");
  assert_equals(getComputedStyle(img[1]).getPropertyValue("vertical-align"),
    "bottom", "mixed case valid");
  assert_equals(getComputedStyle(img[2]).getPropertyValue("vertical-align"),
    "baseline", "non-ASCII invalid");
}, "keyword absbottom");

test(() => {
  assert_equals(getComputedStyle(img[3]).getPropertyValue("vertical-align"),
    "middle", "lowercase valid");
  assert_equals(getComputedStyle(img[4]).getPropertyValue("vertical-align"),
    "middle", "mixed case valid");
  assert_equals(getComputedStyle(img[5]).getPropertyValue("vertical-align"),
    "baseline", "non-ASCII invalid");
}, "keyword abscenter");

test(() => {
  assert_equals(getComputedStyle(img[6]).getPropertyValue("vertical-align"),
    "middle", "lowercase valid");
  assert_equals(getComputedStyle(img[7]).getPropertyValue("vertical-align"),
    "middle", "mixed case valid");
  assert_equals(getComputedStyle(img[8]).getPropertyValue("vertical-align"),
    "baseline", "non-ASCII invalid");
}, "keyword absmiddle");
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
        "byte_end": 486,
        "byte_start": 445,
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
        "byte_end": 528,
        "byte_start": 487,
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
        "byte_end": 571,
        "byte_start": 529,
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
        "byte_end": 613,
        "byte_start": 572,
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
        "byte_end": 655,
        "byte_start": 614,
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
        "byte_end": 698,
        "byte_start": 656,
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
        "byte_end": 740,
        "byte_start": 699,
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
        "byte_end": 782,
        "byte_start": 741,
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
        "byte_end": 825,
        "byte_start": 783,
        "col": 1,
        "line": 16
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/embedded-and-images-presentational-hints-ascii-case-insensitive.html"
}
```
