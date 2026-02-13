# html/semantics/embedded-content/the-iframe-element/iframe-initially-empty-is-updated.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-initially-empty-is-updated.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>Iframe that doesn't load can be updated and rendered.</title>
<meta charset="utf-8">
<link rel="match" href="iframe-initially-empty-is-updated-ref.html"/>
<html>
  <body>
    <iframe src="resources/empty.html"></iframe>
    <script>
      window[0].document.body.appendChild(document.createElement('div'))
          .appendChild(document.createTextNode('Hello world!'));
      window[0].document.body.firstChild.style = 'color: green';
      window.stop();
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          document.documentElement.classList.remove("reftest-wait");
        });
      });
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-initially-empty-is-updated.html"
}
```
