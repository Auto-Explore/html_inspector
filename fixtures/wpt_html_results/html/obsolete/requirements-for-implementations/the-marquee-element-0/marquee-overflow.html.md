# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-overflow.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-overflow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Marquee forces overflow: hidden</title>
<link rel="help" href="https://html.spec.whatwg.org/#the-marquee-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<marquee style="overflow: visible">&nbsp;</marquee>
<marquee style="overflow: scroll">&nbsp;</marquee>
<marquee style="overflow: clip">&nbsp;</marquee>
<marquee style="overflow: auto">&nbsp;</marquee>

<script>
test(() => {
  for (let m of document.querySelectorAll("marquee")) {
    assert_equals(getComputedStyle(m).overflow, "hidden", m.style);
  }
}, "Marquee should have overflow: hidden !important in the UA stylesheet");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 302,
        "byte_start": 267,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 353,
        "byte_start": 319,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 402,
        "byte_start": 370,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 451,
        "byte_start": 419,
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
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-overflow.html"
}
```
