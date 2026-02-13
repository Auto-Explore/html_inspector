# html/semantics/permission-element/geolocation-element/iframe-use-after-free-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/iframe-use-after-free-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class=test-wait>
  <head>
    <meta charset="utf-8" />
    <title>Geolocation creation should not crash after document is shutdown</title>
    <link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md" />
  </head>
  <body>
    <iframe id="target-frame" src="about:blank"></iframe>
    <script>
      const iframe = document.getElementById("target-frame");

      // Save a reference to its contentDocument
      const savedDoc = iframe.contentDocument;

      iframe.onload = () => {
        // Call createElement through that reference
        let geolocation = savedDoc.createElement("geolocation");
        geolocation.setAttribute("watch", "");
        geolocation.setAttribute("autolocate", "");
        geolocation.setAttribute("accuracymode", "precise");
        savedDoc.body.appendChild(geolocation);
        document.documentElement.classList.remove("test-wait");
      };

      // Navigate away the iframe to "shut down" said document
      // We navigate the iframe to a new URL. This detaches the original document.
      iframe.src = "data:text/html,<h1>Navigated Away</h1>";
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
  "source_name": "html/semantics/permission-element/geolocation-element/iframe-use-after-free-crash.html"
}
```
