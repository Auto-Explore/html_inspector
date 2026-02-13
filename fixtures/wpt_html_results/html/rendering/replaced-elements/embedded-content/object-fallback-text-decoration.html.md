# html/rendering/replaced-elements/embedded-content/object-fallback-text-decoration.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/object-fallback-text-decoration.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Rendering of object element fallback with text-decoration</title>
<link rel="match" href="object-fallback-text-decoration-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#embedded-content-rendering-rules">
<meta name="assert" content="Checks that text-decoration applies to rendered object fallback.">
<style>
  object { text-decoration: underline; }
</style>
<object>This text should be underlined.</object>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 430,
        "byte_start": 422,
        "col": 1,
        "line": 9
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
  "source_name": "html/rendering/replaced-elements/embedded-content/object-fallback-text-decoration.html"
}
```
