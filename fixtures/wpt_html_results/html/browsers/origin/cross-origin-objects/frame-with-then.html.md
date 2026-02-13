# html/browsers/origin/cross-origin-objects/frame-with-then.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/cross-origin-objects/frame-with-then.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <script>
    if (location.search == "?setdomain") {
      document.domain = document.domain;
    }
  </script>
  <body>
    <!--- Some frames to test ordering -->
    <iframe name="a"></iframe>
    <!-- A subframe to test "then" behavior -->
    <iframe name="then"></iframe>
    <iframe name="b"></iframe>
    <!-- Two subframes with names corresponding to IDL-defined properties; one
         a cross-origin-exposed property and one not exposed cross-origin -->
    <iframe name="close"></iframe>
    <iframe name="open"></iframe>
  </body>
</html>
```

```json
{
  "messages": [
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
  "source_name": "html/browsers/origin/cross-origin-objects/frame-with-then.html"
}
```
