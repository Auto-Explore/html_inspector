# html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>iframe: changing the scrolling attribute</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-page">
<link rel="match" href="iframe-scrolling-attribute-ref.html">

<style>
    iframe {
        width: 100px;
        height: 100px;
    }
</style>

<p>These two iframes should *both* render with scrollbars:</p>
<iframe src="support/big-page.html" scrolling="unknown"></iframe>
<iframe src="support/big-page.html" scrolling="unknown"></iframe>

<script>
  var iframe = document.getElementsByTagName("iframe")[1];
  // Setting scrolling=no and then back to scrolling=unknown
  // should result in a final value of auto.
  iframe.setAttribute("scrolling", "no");
  iframe.setAttribute("scrolling", "unknown");
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
  "source_name": "html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute.html"
}
```
