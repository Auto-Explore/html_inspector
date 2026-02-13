# html/semantics/embedded-content/the-iframe-element/iframe-modify-scrolling-attr-to-yes-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-modify-scrolling-attr-to-yes-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>iframe with scrolling attr equals yes</title>
<link rel="author" title="Jinfeng Ma" href="mailto:majinfeng1@xiaomi.org">

<p>Test passes if you can see the scrollbars of the iframe displayed below.</p>
<iframe src="support/iframe-which-content-height-equals-400px.html" scrolling="yes" width="200px" height="100px">
</iframe>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-modify-scrolling-attr-to-yes-ref.html"
}
```
