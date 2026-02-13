# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-alt-crash-001.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-alt-crash-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Crash test: img alt rendering in combination with style attribute selector</title>
<link rel="help" href="https://crbug.com/1057210">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  img { display: block; width: 100px; }
  [style] + * {}
</style>
<img id="img" alt="alternative text">
<script>
  test(() => {
    assert_equals(getComputedStyle(img).width, "100px");
  }, "Should not crash.");
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
        "byte_end": 374,
        "byte_start": 337,
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-alt-crash-001.html"
}
```
