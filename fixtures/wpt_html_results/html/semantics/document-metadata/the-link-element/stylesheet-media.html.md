# html/semantics/document-metadata/the-link-element/stylesheet-media.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/stylesheet-media.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test</title>
<link rel=match href=stylesheet-media-ref.html>
<style>
body {
  color: green;
}
</style>
<link rel=stylesheet id=link>
<script>
// This tests for a bug in Servo, where it would treat the media attribute as
// if it was the href attribute.
var link = document.getElementById("link");
link.setAttribute("media", "all")
</script>
<p>This text should be green.
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 176,
        "byte_start": 147,
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
  "source_name": "html/semantics/document-metadata/the-link-element/stylesheet-media.html"
}
```
