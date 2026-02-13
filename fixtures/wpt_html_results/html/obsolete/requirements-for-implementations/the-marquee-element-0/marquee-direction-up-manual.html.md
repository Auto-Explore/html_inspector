# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-direction-up-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-direction-up-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: marquee-direction-up</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/multipage/obsolete.html#the-marquee-element">
<meta name="assert" content="Check if the marquee direction is from bottom to top">
<p>Test passes if the text "Test Marquee" moves from bottom to top.</p>
<marquee direction="up">Test Marquee</marquee>
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
        "byte_end": 437,
        "byte_start": 413,
        "col": 1,
        "line": 8
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
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-direction-up-manual.html"
}
```
