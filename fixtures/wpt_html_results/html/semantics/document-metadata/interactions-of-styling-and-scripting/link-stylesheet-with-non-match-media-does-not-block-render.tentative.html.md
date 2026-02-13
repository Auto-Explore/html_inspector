# html/semantics/document-metadata/interactions-of-styling-and-scripting/link-stylesheet-with-non-match-media-does-not-block-render.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/interactions-of-styling-and-scripting/link-stylesheet-with-non-match-media-does-not-block-render.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>
  Delayed Stylesheet imported using link tag should not block rendering
  or JS execution when media doesn't match.
</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/utils.js"></script>
<link rel="stylesheet" href="support/link-style.css?pipe=trickle(d1)"
  media="print" onload="this.media='all'">
<h1>
  This text is black in color till stylesheet is fetched.
</h1>
<script>
test(() => {
  assert_false(styleExists("h1 { color: purple; }"),
               'Stylesheet should still be pending due to delay');
  const h1 = document.querySelector('h1');
  assert_equals(getComputedStyle(h1).color, 'rgb(0, 0, 0)');
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/interactions-of-styling-and-scripting/link-stylesheet-with-non-match-media-does-not-block-render.tentative.html"
}
```
