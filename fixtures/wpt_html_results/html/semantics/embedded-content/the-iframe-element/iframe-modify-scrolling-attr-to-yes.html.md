# html/semantics/embedded-content/the-iframe-element/iframe-modify-scrolling-attr-to-yes.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-modify-scrolling-attr-to-yes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>modify iframe scrolling attr to yes</title>
<link rel="author" title="Jinfeng Ma" href="mailto:majinfeng1@xiaomi.org">
<link rel="help" href="https://www.w3.org/TR/html401/present/frames.html#adef-scrolling">
<link rel="match" href="iframe-modify-scrolling-attr-to-yes-ref.html">

<p>Test passes if you can see the scrollbars of the iframe displayed below.</p>
<iframe src="support/iframe-which-content-height-equals-400px.html" scrolling="no" width="200px" height="100px">
</iframe>

<script>
    let iframe = document.querySelector("iframe");
    iframe.onload = function () {
        iframe.scrolling = 'yes';
    };
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-modify-scrolling-attr-to-yes.html"
}
```
