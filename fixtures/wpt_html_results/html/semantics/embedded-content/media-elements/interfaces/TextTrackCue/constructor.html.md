# html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/constructor.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/constructor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>TextTrackCue constructor</title>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
    </head>
    <body>
        <script>
            test(function()
            {
                assert_not_equals(TextTrackCue, VTTCue);
            }, "TextTrackCue and VTTCue are separate interfaces");
            test(function()
            {
                assert_throws_js(TypeError, function()
                {
                    new TextTrackCue(0, 0, "");
                });
            }, "TextTrackCue constructor should not be supported");
        </script>
    </body>
</html>
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/constructor.html"
}
```
