# html/semantics/embedded-content/the-object-element/object-construct-in-document-with-null-browsing-context-crash.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-construct-in-document-with-null-browsing-context-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>HTMLObjectElement: construct in a document with a null browsing context</title>
<link rel="author" title="Nate Chapin" href="mailto:japhet@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/C/#the-object-element">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1083437">
<meta name="assert" content="Constructing an HTMLObjectElement in a document with a null browsing context should not crash"/>
<iframe id="i"></iframe>
<script>
var doc = i.contentDocument;
i.remove();
doc.createElement('object');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-construct-in-document-with-null-browsing-context-crash.html"
}
```
